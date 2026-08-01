//! Source replay for the empirical marginal net-interest rollover feature.
use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use taxlane_net_interest::{FinancingFlow, IssuanceBucket, simulate_monthly_rollover};

#[derive(Clone, Copy, Default)]
struct BucketReplay {
    rows: i64,
    amount: f64,
    amount_days: f64,
}

#[derive(Clone, Copy, Default)]
struct MonthReplay {
    rows: i64,
    total: f64,
    buckets: [f64; 8],
}

fn date_parts(value: &str) -> Result<(i64, usize, i64), String> {
    if value.len() != 10 {
        return Err(format!("invalid MSPD ISO date: {value}"));
    }
    Ok((
        value[0..4].parse::<i64>().map_err(|e| e.to_string())?,
        value[5..7].parse::<usize>().map_err(|e| e.to_string())?,
        value[8..10].parse::<i64>().map_err(|e| e.to_string())?,
    ))
}

fn leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn ordinal(year: i64, month: usize, day: i64) -> i64 {
    let mut month_days = [31_i64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if leap(year) {
        month_days[1] = 29;
    }
    (0..year)
        .map(|candidate| if leap(candidate) { 366 } else { 365 })
        .sum::<i64>()
        + month_days[..month - 1].iter().sum::<i64>()
        + day
}

fn term_bucket(days: i64) -> usize {
    match days {
        ..=92 => 0,
        93..=183 => 1,
        184..=366 => 2,
        367..=1096 => 3,
        1097..=1827 => 4,
        1828..=3653 => 5,
        3654..=7305 => 6,
        _ => 7,
    }
}

pub(crate) fn validate_net_interest_mspd_empirical_rollover_convention(
    root: &Path,
) -> Result<(), String> {
    for path in [
        NET_EMPIRICAL_ROLLOVER_JSON_PATH,
        NET_EMPIRICAL_ROLLOVER_SCHEMA_PATH,
        NET_EMPIRICAL_ROLLOVER_READER_PATH,
        NET_EMPIRICAL_ROLLOVER_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing NET empirical-rollover artifact: {path}"));
        }
    }
    validate_net_interest_mspd_public_maturity_envelope(root)?;
    let record: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join(NET_EMPIRICAL_ROLLOVER_JSON_PATH))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if string_field(&record, "record_id")? != "net-interest-mspd-empirical-rollover-convention:v1"
        || string_field(&record, "status")?
            != "empirical_marginal_rollover_ready_full_stock_forecast_blocked"
        || string_field(&record, "public_maturity_envelope_path")?
            != NET_PUBLIC_MATURITY_ENVELOPE_JSON_PATH
    {
        return Err("NET empirical-rollover identity failed".to_string());
    }

    let source = &record["source_packet"];
    let source_path = root.join(string_field(source, "raw_artifact_path")?);
    if int_field(source, "raw_byte_count")? != 55_726_310
        || fs::metadata(&source_path).map_err(|e| e.to_string())?.len() != 55_726_310
        || sha256_file(&source_path)? != string_field(source, "raw_sha256")?
    {
        return Err("NET empirical-rollover source custody failed".to_string());
    }

    let mut reader = csv::Reader::from_path(&source_path).map_err(|e| e.to_string())?;
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let column = |name: &str| {
        headers
            .iter()
            .position(|value| value == name)
            .ok_or_else(|| format!("missing MSPD column: {name}"))
    };
    let record_date_index = column("record_date")?;
    let security_type_index = column("security_type_desc")?;
    let cusip_index = column("security_class2_desc")?;
    let yield_index = column("yield_pct")?;
    let issue_index = column("issue_date")?;
    let maturity_index = column("maturity_date")?;
    let issued_index = column("issued_amt")?;
    let mut buckets = [BucketReplay::default(); 8];
    let mut months = BTreeMap::<String, MonthReplay>::new();
    let mut keys = BTreeSet::new();
    let mut duplicates = 0_i64;

    for csv_row in reader.records() {
        let csv_row = csv_row.map_err(|e| e.to_string())?;
        let record_date = csv_row
            .get(record_date_index)
            .ok_or("missing record date")?;
        if record_date < "2025-07-31"
            || record_date > "2026-06-30"
            || csv_row.get(security_type_index) != Some("Marketable")
        {
            continue;
        }
        let issue = csv_row.get(issue_index).ok_or("missing issue date")?;
        let maturity = csv_row.get(maturity_index).ok_or("missing maturity date")?;
        let amount_text = csv_row.get(issued_index).ok_or("missing issued amount")?;
        if issue.len() != 10
            || maturity.len() != 10
            || matches!(issue, "null" | "")
            || matches!(maturity, "null" | "")
            || matches!(amount_text, "null" | "*" | "")
            || issue[0..7] != record_date[0..7]
        {
            continue;
        }
        let amount = amount_text.parse::<f64>().map_err(|e| e.to_string())?;
        let (iy, im, id) = date_parts(issue)?;
        let (my, mm, md) = date_parts(maturity)?;
        let days = ordinal(my, mm, md) - ordinal(iy, im, id);
        if days <= 0 {
            return Err("NET empirical-rollover nonpositive original term".to_string());
        }
        let bucket = term_bucket(days);
        buckets[bucket].rows += 1;
        buckets[bucket].amount += amount;
        buckets[bucket].amount_days += amount * days as f64;
        let month = months.entry(record_date.to_string()).or_default();
        month.rows += 1;
        month.total += amount;
        month.buckets[bucket] += amount;
        let key = format!(
            "{}|{}|{}|{}|{}|{}",
            record_date,
            csv_row.get(cusip_index).unwrap_or(""),
            issue,
            maturity,
            amount_text,
            csv_row.get(yield_index).unwrap_or("")
        );
        if !keys.insert(key) {
            duplicates += 1;
        }
    }

    let extraction = &record["extraction_rule"];
    let total_rows = buckets.iter().map(|bucket| bucket.rows).sum::<i64>();
    let total_amount = buckets.iter().map(|bucket| bucket.amount).sum::<f64>();
    if months.len() != 12
        || total_rows != 434
        || duplicates != 0
        || int_field(extraction, "included_rows")? != total_rows
        || int_field(extraction, "exact_duplicate_rows")? != duplicates
        || (number_field(extraction, "gross_issuance_musd")? - total_amount).abs() > 0.000001
        || !string_field(extraction, "calendar_month_rule")?.contains("including the first")
        || !string_field(extraction, "prior_working_result_correction")?.contains("423-row")
    {
        return Err("NET empirical-rollover extraction replay failed".to_string());
    }

    let rows = record["central_term_mix"]
        .as_array()
        .ok_or("NET empirical term mix")?;
    if rows.len() != 8 {
        return Err("NET empirical-rollover bucket count failed".to_string());
    }
    let mut mix = Vec::new();
    let mut ppb_total = 0_i64;
    for (index, (row, replay)) in rows.iter().zip(buckets).enumerate() {
        let shares = months
            .values()
            .map(|month| 100.0 * month.buckets[index] / month.total)
            .collect::<Vec<_>>();
        let min_share = shares.iter().copied().fold(f64::INFINITY, f64::min);
        let max_share = shares.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let representative_months = (replay.amount_days / replay.amount / 30.4375).round() as i64;
        let share_ppb = int_field(row, "share_ppb")?;
        ppb_total += share_ppb;
        if int_field(row, "rows")? != replay.rows
            || (number_field(row, "gross_issuance_musd")? - replay.amount).abs() > 0.000001
            || (number_field(row, "share_percent")? - 100.0 * replay.amount / total_amount).abs()
                > 0.000001
            || (number_field(row, "weighted_average_original_term_days")?
                - replay.amount_days / replay.amount)
                .abs()
                > 0.000001
            || int_field(row, "representative_term_months")? != representative_months
            || (number_field(row, "observed_monthly_share_min_percent")? - min_share).abs()
                > 0.000001
            || (number_field(row, "observed_monthly_share_max_percent")? - max_share).abs()
                > 0.000001
        {
            return Err(format!(
                "NET empirical-rollover bucket replay failed: {index}"
            ));
        }
        mix.push(IssuanceBucket {
            id: string_field(row, "bucket_id")?,
            share_ppb: i128::from(share_ppb),
            term_months: u16::try_from(representative_months).map_err(|e| e.to_string())?,
        });
    }
    if ppb_total != 1_000_000_000 {
        return Err("NET empirical-rollover shares do not sum to scale".to_string());
    }

    for rail in record["observed_monthly_rails"]
        .as_array()
        .ok_or("observed rails")?
    {
        let record_date = string_field(rail, "record_date")?;
        let replay = months
            .get(&record_date)
            .ok_or("missing observed rail month")?;
        let shares = rail["share_percent_by_bucket"]
            .as_array()
            .ok_or("rail shares")?;
        if shares.len() != 8
            || int_field(rail, "rows")? != replay.rows
            || (number_field(rail, "gross_issuance_musd")? - replay.total).abs() > 0.000001
        {
            return Err(format!("NET empirical-rollover rail failed: {record_date}"));
        }
        for (index, share) in shares.iter().enumerate() {
            let expected = 100.0 * replay.buckets[index] / replay.total;
            if (share.as_f64().ok_or("rail share number")? - expected).abs() > 0.000001 {
                return Err(format!(
                    "NET empirical-rollover rail share failed: {record_date}"
                ));
            }
        }
    }

    let fixture = &record["mechanical_fixture"];
    let flow = &fixture["financing_flow"];
    let snapshot_months = fixture["snapshot_months"]
        .as_array()
        .ok_or("snapshot months")?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .ok_or_else(|| "snapshot month integer".to_string())
                .and_then(|value| u16::try_from(value).map_err(|e| e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let simulated = simulate_monthly_rollover(
        &mix,
        &[FinancingFlow {
            month: u16::try_from(int_field(flow, "month")?).map_err(|e| e.to_string())?,
            amount_musd_micros: i128::from(int_field(flow, "amount_musd_micros")?),
        }],
        u16::try_from(int_field(fixture, "month_count")?).map_err(|e| e.to_string())?,
        &snapshot_months,
    )?;
    let expected = fixture["annual_snapshots"]
        .as_array()
        .ok_or("annual snapshots")?;
    if simulated.len() != 10 || expected.len() != simulated.len() {
        return Err("NET empirical-rollover fixture horizon failed".to_string());
    }
    for (actual, expected) in simulated.iter().zip(expected) {
        let expected_buckets = expected["remaining_term_buckets_musd_micros"]
            .as_array()
            .ok_or("remaining buckets")?;
        if int_field(expected, "fiscal_year")? != 2026 + i64::from(actual.month / 12)
            || int_field(expected, "month")? != i64::from(actual.month)
            || i128::from(int_field(expected, "total_principal_musd_micros")?)
                != actual.total_principal_musd_micros
            || i128::from(int_field(
                expected,
                "rollover_since_prior_snapshot_musd_micros",
            )?) != actual.rollover_since_prior_snapshot_musd_micros
            || expected_buckets.len() != 8
        {
            return Err(format!(
                "NET empirical-rollover fixture row failed: {}",
                actual.month
            ));
        }
        for (index, amount) in expected_buckets.iter().enumerate() {
            if i128::from(amount.as_i64().ok_or("remaining bucket integer")?)
                != actual.remaining_term_buckets_musd_micros[index]
            {
                return Err(format!(
                    "NET empirical-rollover fixture bucket failed: {}",
                    actual.month
                ));
            }
        }
    }

    let effect = &record["readiness_effect"];
    if int_field(effect, "formula_inputs_ready_before")? != 7
        || int_field(effect, "formula_inputs_ready_after")? != 8
        || int_field(effect, "completion_steps_ready_before")? != 6
        || int_field(effect, "completion_steps_ready_after")? != 7
        || int_field(effect, "net_verdict_score_before")? != 13
        || int_field(effect, "net_verdict_score_after")? != 14
    {
        return Err("NET empirical-rollover readiness effect failed".to_string());
    }
    let claims = record["claim_booleans"]
        .as_object()
        .ok_or("rollover claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "empirical_issuance_mix_ready"
                | "marginal_rollover_convention_ready"
                | "mechanical_rollover_fixture_ready"
        );
        if value.as_bool().ok_or("rollover claim bool")? != expected {
            return Err(format!(
                "NET empirical-rollover claim boundary failed: {field}"
            ));
        }
    }
    let reader = fs::read_to_string(root.join(NET_EMPIRICAL_ROLLOVER_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        "434 distinct",
        "$31.594618 trillion",
        "first calendar day",
        "$100 billion",
        "marginal incremental-feedback",
        "not forecast Treasury's total future issuance",
    ] {
        if !reader.contains(required) {
            return Err(format!("NET empirical-rollover reader missing {required}"));
        }
    }
    Ok(())
}
