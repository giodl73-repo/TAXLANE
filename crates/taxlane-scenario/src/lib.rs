//! Hypothetical scenarios: arithmetic does not admit a research savings claim.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use taxlane_core::{FederalYearInput, reconcile_federal_year};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lane {
    pub id: String,
    pub label: String,
    pub amount_musd: i64,
    pub adjustable: bool,
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Baseline {
    pub version: u32,
    pub fiscal_year: u16,
    pub receipts_musd: i64,
    pub outlays_musd: i64,
    pub lanes: Vec<Lane>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    /// Integer basis points: -10000..=10000 means -100%..=+100%.
    pub lane_change_bps: BTreeMap<String, i32>,
    pub receipts_change_bps: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResultLane {
    pub id: String,
    pub baseline_musd: i64,
    pub scenario_musd: i64,
    pub change_musd: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Comparison {
    pub fiscal_year: u16,
    pub baseline_outlays_musd: i64,
    pub outlays_musd: i64,
    pub receipts_musd: i64,
    pub primary_outlays_musd: i64,
    pub net_interest_musd: i64,
    pub deficit_musd: i64,
    pub baseline_deficit_musd: i64,
    pub deficit_change_musd: i64,
    pub lanes: Vec<ResultLane>,
}

fn scale(amount: i64, change: i32) -> Result<i64, String> {
    if !(-10000..=10000).contains(&change) {
        return Err("Changes must be between -100% and +100%".into());
    }
    if amount < 0 {
        return Err("Negative accounting offsets cannot be scaled".into());
    }
    // Published figures are millions of dollars; round once to nearest million.
    i64::try_from((i128::from(amount) * i128::from(10000 + change) + 5000) / 10000)
        .map_err(|_| "Scenario amount overflow".into())
}

pub fn compare(baseline: &Baseline, scenario: &Scenario) -> Result<Comparison, String> {
    if baseline.version != 1 || baseline.receipts_musd < 0 || baseline.lanes.is_empty() {
        return Err("Invalid baseline version, receipts, or lanes".into());
    }
    let mut ids = BTreeSet::new();
    let mut sum = 0i64;
    let mut primary = 0i64;
    let mut interest = None;
    let mut lanes = Vec::new();
    for lane in &baseline.lanes {
        if lane.id.is_empty() || !ids.insert(lane.id.clone()) {
            return Err("Baseline lane IDs must be nonempty and unique".into());
        }
        sum = sum
            .checked_add(lane.amount_musd)
            .ok_or("Baseline overflow")?;
        let change = scenario.lane_change_bps.get(&lane.id).copied().unwrap_or(0);
        if !lane.adjustable && change != 0 {
            return Err(format!("{} is a fixed accounting lane", lane.id));
        }
        let amount = if lane.adjustable {
            scale(lane.amount_musd, change)?
        } else {
            lane.amount_musd
        };
        if lane.id == "net-interest" {
            if lane.adjustable || amount < 0 {
                return Err("Net interest must be nonnegative and fixed".into());
            }
            interest = Some(amount);
        } else {
            primary = primary
                .checked_add(amount)
                .ok_or("Primary outlay overflow")?;
        }
        lanes.push(ResultLane {
            id: lane.id.clone(),
            baseline_musd: lane.amount_musd,
            scenario_musd: amount,
            change_musd: amount
                .checked_sub(lane.amount_musd)
                .ok_or("Change overflow")?,
        });
    }
    if sum != baseline.outlays_musd {
        return Err("Baseline lanes do not reconcile to total outlays".into());
    }
    if scenario.lane_change_bps.keys().any(|id| !ids.contains(id)) {
        return Err("Unknown scenario lane".into());
    }
    let net_interest = interest.ok_or("Missing net-interest lane")?;
    let receipts = scale(baseline.receipts_musd, scenario.receipts_change_bps)?;
    let result = reconcile_federal_year(FederalYearInput {
        receipts,
        primary_outlays: primary,
        net_interest,
        opening_debt_held_by_public: 0,
        other_financing_and_timing: 0,
    })?;
    // A single-year financing gap is displayed, not an invented debt-stock forecast.
    let baseline_deficit = baseline
        .outlays_musd
        .checked_sub(baseline.receipts_musd)
        .ok_or("Baseline deficit overflow")?;
    Ok(Comparison {
        fiscal_year: baseline.fiscal_year,
        baseline_outlays_musd: baseline.outlays_musd,
        outlays_musd: result.total_outlays,
        receipts_musd: receipts,
        primary_outlays_musd: primary,
        net_interest_musd: net_interest,
        deficit_musd: result.total_deficit,
        baseline_deficit_musd: baseline_deficit,
        deficit_change_musd: result
            .total_deficit
            .checked_sub(baseline_deficit)
            .ok_or("Deficit change overflow")?,
        lanes,
    })
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn compare_json(baseline: &str, scenario: &str) -> Result<String, wasm_bindgen::JsValue> {
    let evaluate = || -> Result<String, String> {
        let baseline = serde_json::from_str(baseline).map_err(|e| e.to_string())?;
        let scenario = serde_json::from_str(scenario).map_err(|e| e.to_string())?;
        serde_json::to_string(&compare(&baseline, &scenario)?).map_err(|e| e.to_string())
    };
    evaluate().map_err(|e| wasm_bindgen::JsValue::from_str(&e))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn baseline() -> Baseline {
        Baseline {
            version: 1,
            fiscal_year: 2025,
            receipts_musd: 80,
            outlays_musd: 110,
            lanes: vec![
                Lane {
                    id: "service".into(),
                    label: "Service".into(),
                    amount_musd: 100,
                    adjustable: true,
                    sources: vec![],
                },
                Lane {
                    id: "offset".into(),
                    label: "Offset".into(),
                    amount_musd: -10,
                    adjustable: false,
                    sources: vec![],
                },
                Lane {
                    id: "net-interest".into(),
                    label: "Interest".into(),
                    amount_musd: 20,
                    adjustable: false,
                    sources: vec![],
                },
            ],
        }
    }
    #[test]
    fn baseline_and_changed_scenario_reconcile() {
        let base = baseline();
        let unchanged = compare(&base, &Scenario::default()).unwrap();
        assert_eq!(unchanged.deficit_musd, 30);
        assert_eq!(unchanged.deficit_change_musd, 0);
        let scenario = Scenario {
            lane_change_bps: BTreeMap::from([("service".into(), -2000)]),
            receipts_change_bps: 2500,
        };
        let changed = compare(&base, &scenario).unwrap();
        assert_eq!(
            (
                changed.primary_outlays_musd,
                changed.net_interest_musd,
                changed.outlays_musd
            ),
            (70, 20, 90)
        );
        assert_eq!(
            (
                changed.receipts_musd,
                changed.deficit_musd,
                changed.deficit_change_musd
            ),
            (100, -10, -40)
        );
    }
    #[test]
    fn rejects_fixed_unknown_out_of_range_and_invalid_baselines() {
        for (id, change) in [
            ("offset", 100),
            ("net-interest", -100),
            ("unknown", 0),
            ("service", 10001),
        ] {
            assert!(
                compare(
                    &baseline(),
                    &Scenario {
                        lane_change_bps: BTreeMap::from([(id.into(), change)]),
                        ..Default::default()
                    }
                )
                .is_err()
            );
        }
        let mut base = baseline();
        base.outlays_musd += 1;
        assert!(compare(&base, &Scenario::default()).is_err());
        let mut base = baseline();
        base.lanes.push(base.lanes[0].clone());
        assert!(compare(&base, &Scenario::default()).is_err());
    }
    #[test]
    fn full_cut_and_rounding_are_explicit() {
        assert_eq!(scale(101, -10000).unwrap(), 0);
        assert_eq!(scale(101, 10000).unwrap(), 202);
        assert_eq!(scale(101, -5000).unwrap(), 51);
        assert!(scale(i64::MAX, 10000).is_err());
    }
}
