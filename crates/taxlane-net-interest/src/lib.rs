use std::collections::BTreeMap;

pub const SHARE_SCALE: i128 = 1_000_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuanceBucket {
    pub id: String,
    pub share_ppb: i128,
    pub term_months: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancingFlow {
    pub month: u16,
    pub amount_musd_micros: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RolloverSnapshot {
    pub month: u16,
    pub total_principal_musd_micros: i128,
    pub rollover_since_prior_snapshot_musd_micros: i128,
    pub remaining_term_buckets_musd_micros: [i128; 8],
}

pub fn validate_mix(mix: &[IssuanceBucket]) -> Result<(), String> {
    if mix.is_empty() {
        return Err("issuance mix must not be empty".to_string());
    }
    if mix
        .iter()
        .any(|row| row.id.trim().is_empty() || row.share_ppb < 0 || row.term_months == 0)
    {
        return Err(
            "issuance mix rows require id, nonnegative share, and positive term".to_string(),
        );
    }
    if mix.iter().map(|row| row.share_ppb).sum::<i128>() != SHARE_SCALE {
        return Err("issuance shares must sum to one billion ppb".to_string());
    }
    let unique_ids = mix
        .iter()
        .map(|row| row.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if unique_ids.len() != mix.len() {
        return Err("issuance mix bucket ids must be unique".to_string());
    }
    Ok(())
}

fn allocate(
    amount: i128,
    month: u16,
    mix: &[IssuanceBucket],
    cohorts: &mut BTreeMap<u16, Vec<i128>>,
) -> Result<(), String> {
    let mut allocated = 0_i128;
    for (index, bucket) in mix.iter().enumerate() {
        let part = if index + 1 == mix.len() {
            amount - allocated
        } else {
            amount
                .checked_mul(bucket.share_ppb)
                .ok_or("issuance allocation overflow")?
                / SHARE_SCALE
        };
        allocated = allocated
            .checked_add(part)
            .ok_or("issuance allocation total overflow")?;
        let due = month
            .checked_add(bucket.term_months)
            .ok_or("issuance maturity month overflow")?;
        cohorts.entry(due).or_default().push(part);
    }
    if allocated != amount {
        return Err("issuance allocation does not preserve principal".to_string());
    }
    Ok(())
}

fn remaining_bucket(remaining_months: u16) -> usize {
    match remaining_months {
        0..=3 => 0,
        4..=6 => 1,
        7..=12 => 2,
        13..=36 => 3,
        37..=60 => 4,
        61..=120 => 5,
        121..=240 => 6,
        _ => 7,
    }
}

pub fn simulate_monthly_rollover(
    mix: &[IssuanceBucket],
    flows: &[FinancingFlow],
    month_count: u16,
    snapshot_months: &[u16],
) -> Result<Vec<RolloverSnapshot>, String> {
    validate_mix(mix)?;
    if snapshot_months.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("snapshot months must be strictly increasing".to_string());
    }
    if snapshot_months.iter().any(|month| *month >= month_count) {
        return Err("snapshot month falls outside simulation horizon".to_string());
    }
    let mut flow_by_month = BTreeMap::<u16, i128>::new();
    for flow in flows {
        if flow.month >= month_count {
            return Err("financing flow falls outside simulation horizon".to_string());
        }
        let entry = flow_by_month.entry(flow.month).or_default();
        *entry = entry
            .checked_add(flow.amount_musd_micros)
            .ok_or("financing flow total overflow")?;
    }
    let mut cohorts = BTreeMap::<u16, Vec<i128>>::new();
    let mut rollover_since_prior_snapshot = 0_i128;
    let mut snapshots = Vec::new();
    for month in 0..month_count {
        let matured = cohorts
            .remove(&month)
            .unwrap_or_default()
            .into_iter()
            .sum::<i128>();
        if matured != 0 {
            rollover_since_prior_snapshot = rollover_since_prior_snapshot
                .checked_add(matured)
                .ok_or("rollover total overflow")?;
            allocate(matured, month, mix, &mut cohorts)?;
        }
        if let Some(flow) = flow_by_month.get(&month).copied() {
            allocate(flow, month, mix, &mut cohorts)?;
        }
        if snapshot_months.binary_search(&month).is_ok() {
            let mut remaining = [0_i128; 8];
            for (due, amounts) in &cohorts {
                let bucket = remaining_bucket(due - month);
                remaining[bucket] += amounts.iter().sum::<i128>();
            }
            snapshots.push(RolloverSnapshot {
                month,
                total_principal_musd_micros: remaining.iter().sum(),
                rollover_since_prior_snapshot_musd_micros: rollover_since_prior_snapshot,
                remaining_term_buckets_musd_micros: remaining,
            });
            rollover_since_prior_snapshot = 0;
        }
    }
    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mix() -> Vec<IssuanceBucket> {
        vec![
            IssuanceBucket {
                id: "short".into(),
                share_ppb: 600_000_000,
                term_months: 2,
            },
            IssuanceBucket {
                id: "long".into(),
                share_ppb: 400_000_000,
                term_months: 24,
            },
        ]
    }

    #[test]
    fn rejects_incomplete_mix() {
        let mut invalid = mix();
        invalid[0].share_ppb -= 1;
        assert!(validate_mix(&invalid).is_err());
    }

    #[test]
    fn rejects_duplicate_bucket_ids_and_unordered_snapshots() {
        let mut duplicate = mix();
        duplicate[1].id = duplicate[0].id.clone();
        assert!(validate_mix(&duplicate).is_err());
        assert!(simulate_monthly_rollover(&mix(), &[], 24, &[23, 11]).is_err());
        assert!(simulate_monthly_rollover(&mix(), &[], 24, &[11, 11]).is_err());
        assert!(simulate_monthly_rollover(&mix(), &[], 24, &[24]).is_err());
    }

    #[test]
    fn zero_flow_stays_zero() {
        let rows = simulate_monthly_rollover(&mix(), &[], 24, &[11, 23]).unwrap();
        assert!(rows.iter().all(|row| row.total_principal_musd_micros == 0));
    }

    #[test]
    fn rollover_preserves_signed_principal() {
        let amount = -100_000_000_000_i128;
        let rows = simulate_monthly_rollover(
            &mix(),
            &[FinancingFlow {
                month: 5,
                amount_musd_micros: amount,
            }],
            36,
            &[11, 23, 35],
        )
        .unwrap();
        assert_eq!(rows.len(), 3);
        assert!(
            rows.iter()
                .all(|row| row.total_principal_musd_micros == amount)
        );
        assert!(
            rows.iter()
                .any(|row| row.rollover_since_prior_snapshot_musd_micros != 0)
        );
    }
}
