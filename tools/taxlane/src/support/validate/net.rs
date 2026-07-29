//! Auto-split from main.rs (ROUTE-style domain layout).
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use roxmltree::Document;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use taxlane_core::{
    AccountabilityEvidenceRecord, ArtifactMetadata, BreadthBenchmarkRecord, CostDownBacklogRecord,
    CostDownEvidenceQueueRecord, CostDownFirstPassRollupRecord, CostDownScoringReadinessRecord,
    CostDownSourcePacketRecord, DebtMaturityRiskTreasuryProbeRecord,
    DebtPrimaryBalanceFiscalProbeRecord, DefenseAuditControlProbeRecord,
    DefenseProcurementControlProbeRecord, DisasterDeclarationProbeRecord,
    DisasterMitigationProjectProbeRecord, EfficiencyPressureRecord,
    ExternalAccountabilityClaimIntakeRecord, ExternalClaimAmountDerivation,
    ExternalClaimAmountSemantic, ExternalClaimCustodyStatus, ExternalClaimEvidenceRelation,
    ExternalClaimLegalOrAdministrativeStatus, ExternalClaimPublicationKind,
    ExternalClaimResponseRequestStatus, ExternalClaimReviewStatus, ExternalClaimStatus,
    ExternalClaimType, HeadlineBasisRecord, HealthAdminSimplificationProbeRecord,
    HealthPriceDisciplineProbeRecord, PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE,
    PUBLIC_CLAIM_ALLOWED_LABEL, PUBLIC_CLAIM_BLOCKED_LABEL,
    PaymentIntegrityClaimsTimelinessProbeRecord, PaymentIntegrityMethodologyClosureCoverageRecord,
    PaymentIntegrityMethodologyClosureDecisionRecord,
    PaymentIntegrityMethodologyClosureReadinessRecord,
    PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord,
    PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord,
    PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord,
    PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord,
    PaymentIntegrityMethodologyComponentGateProgressRecord,
    PaymentIntegrityMethodologyComponentGateProgressRequirementRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord,
    PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord,
    PaymentIntegrityMethodologyComponentGateRequirementRecord,
    PaymentIntegrityMethodologyComponentGateSourceCaptureRecord,
    PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord,
    PaymentIntegrityMethodologyComponentGateSourceQueryRecord,
    PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord,
    PaymentIntegrityMethodologyComponentGateSourceTargetRecord,
    PaymentIntegrityMethodologyFieldRecord, PaymentIntegrityMethodologyFieldReviewRecord,
    PaymentIntegrityMethodologyFieldUpdateRecord,
    PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord,
    PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord,
    PaymentIntegrityMethodologyFollowupSourceCaptureRecord,
    PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord,
    PaymentIntegrityMethodologyFollowupSourceQueryRecord,
    PaymentIntegrityMethodologyFollowupSourceQueryRunRecord,
    PaymentIntegrityMethodologyGapFollowupRecord,
    PaymentIntegrityMethodologyGapSourceCaptureRecord,
    PaymentIntegrityMethodologyNarrowClosureCandidateRecord,
    PaymentIntegrityMethodologyNarrowClosureDecisionRecord,
    PaymentIntegrityMethodologyOpenProgramComponentProgressRecord,
    PaymentIntegrityMethodologyOpenProgramStatusRecord, PaymentIntegrityMethodologyPlanRecord,
    PaymentIntegrityMethodologyPriorityReviewerActionRecord,
    PaymentIntegrityMethodologyPrioritySourceWorkRecord,
    PaymentIntegrityMethodologyProgramRollupRecord, PaymentIntegrityMethodologyQueryRecord,
    PaymentIntegrityMethodologyQueryRunRecord,
    PaymentIntegrityMethodologyResidualGapPriorityRecord,
    PaymentIntegrityMethodologyResidualSourceGapRecord, PaymentIntegrityMethodologyResultRecord,
    PaymentIntegrityMethodologyResultReviewReadinessRecord,
    PaymentIntegrityMethodologyScoringGateRecord,
    PaymentIntegrityMethodologySourceCaptureRollupRecord,
    PaymentIntegrityMethodologySourceTargetRecord, PaymentIntegrityNextProgramSelectionRecord,
    PaymentIntegrityPortalProbeRecord, PaymentIntegrityProgramReviewGateRecord,
    PaymentIntegrityProgramReviewStatusRecord, PaymentIntegrityProgramReviewTaskRecord,
    PaymentIntegrityScorecardProbeRecord, PerUnitDisplayReadinessRecord, PerUnitReceiptCardRecord,
    PerformanceDemandChecklistRecord, PerformanceDemandResponseBundleArtifact,
    PerformanceDemandResponseBundleManifest, PerformanceDemandResponseClass,
    PerformanceDemandResponseDeltaRow, PerformanceDemandResponseIntakeRecord,
    PerformanceDemandResponseLogClass, PerformanceDemandResponseLogRecord,
    PerformanceDemandResponseStatus, SpendCategoryMapRecord,
};

use zip::ZipArchive;

pub(crate) fn validate_net_interest_formula_contract(root: &Path) -> Result<(), String> {
    for path in [
        NET_INTEREST_FORMULA_CONTRACT_JSON_PATH,
        NET_INTEREST_FORMULA_CONTRACT_SCHEMA_PATH,
        NET_INTEREST_FORMULA_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing net-interest formula artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(NET_INTEREST_FORMULA_CONTRACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "net-interest-formula-contract:v1"
        || string_field(&contract, "record_family")? != "net_interest_formula_contract"
        || int_field(&contract, "pulse")? != 104
        || string_field(&contract, "solver_input_inventory_path")?
            != SOLVER_INPUT_INVENTORY_JSON_PATH
        || string_field(&contract, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&contract, "solver_accounting_readiness_gate_path")?
            != SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH
        || string_field(&contract, "balance_guardrail_path")?
            != "docs/research/2026-06-23-balance-rule-guardrail-spec.md"
        || string_field(&contract, "rate_adjustment_operating_model_path")?
            != "docs/research/2026-06-24-rate-adjustment-operating-model.md"
    {
        return Err("net-interest formula identity failed".to_string());
    }

    let formula = contract
        .get("formula_identity")
        .ok_or("net-interest formula identity")?;
    for required in [
        (
            "primary_balance",
            "total_federal_receipts - primary_outlays",
        ),
        (
            "deficit",
            "primary_outlays + net_interest - total_federal_receipts",
        ),
        (
            "debt_t",
            "debt_t_minus_1 + deficit_t + explicit_other_financing_t",
        ),
    ] {
        if string_field(formula, required.0)? != required.1 {
            return Err(format!("net-interest formula {} failed", required.0));
        }
    }
    for required in [
        "sum_over_maturity_buckets",
        "debt_stock_bucket_t_minus_1",
        "effective_rate_bucket_t",
        "interest_receipts_t",
        "After any primary-balance change",
        "recompute deficit, debt, maturity-bucket debt stock, and subsequent net interest",
    ] {
        let formula_text = format!(
            "{} {}",
            string_field(formula, "net_interest_t")?,
            string_field(formula, "iteration_rule")?
        );
        if !formula_text.contains(required) {
            return Err(format!("net-interest formula text missing {required}"));
        }
    }

    let inputs = contract
        .get("required_inputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest required inputs")?;
    let observed = inputs
        .iter()
        .map(|row| string_field(row, "input_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "baseline_debt_stock",
        "baseline_net_interest",
        "maturity_bucket_schedule",
        "effective_rate_path_by_bucket",
        "new_borrowing_timing_rule",
        "interest_receipts_treatment",
        "explicit_other_financing_series",
        "primary_balance_feedback_test_fixture",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || inputs.len() != expected.len() {
        return Err("net-interest input set failed".to_string());
    }
    for row in inputs {
        if row.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("net-interest inputs must be required/false/null".to_string());
        }
        let blockers = row
            .get("blockers")
            .and_then(serde_json::Value::as_array)
            .ok_or("net-interest blockers")?;
        if blockers.is_empty() {
            return Err("net-interest inputs must name blockers".to_string());
        }
    }

    let rules = contract
        .get("contract_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest rules")?;
    for required in [
        "net_interest_is_endogenous",
        "net_interest_cannot_be_cut_directly",
        "primary_change_must_change_subsequent_debt",
        "primary_change_must_change_subsequent_interest",
        "maturity_and_rate_paths_must_be_explicit",
        "interest_receipts_must_be_explicit",
        "explicit_other_financing_must_be_explicit",
        "missing_values_remain_null",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("net-interest rule failed {required}"));
        }
    }
    if rules
        .get("solver_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("net-interest solver_ready must be false".to_string());
    }

    let regression = contract
        .get("regression_test_contract")
        .ok_or("net-interest regression contract")?;
    if regression
        .get("required")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || regression.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || string_field(regression, "test_name")?
            != "primary_balance_change_recomputes_debt_and_interest"
        || !regression
            .get("test_fixture_path")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("net-interest regression contract failed".to_string());
    }
    let blocked = regression
        .get("blocked_until")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest regression blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "baseline_debt_stock",
        "maturity_bucket_schedule",
        "effective_rate_path_by_bucket",
        "primary_balance_feedback_test_fixture",
    ] {
        if !blocked.contains(required) {
            return Err(format!(
                "net-interest regression blocker missing {required}"
            ));
        }
    }

    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("net-interest claim bool")?;
        if field == "net_interest_formula_contract_published" {
            if !observed {
                return Err("net-interest formula publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("net-interest public claim {field} must be false"));
        }
    }

    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "net-interest formula contract",
        "not a net-interest path",
        "not a solver run",
        "not target-cost selection",
        "not rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!("net-interest boundary missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(NET_INTEREST_FORMULA_CONTRACT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        NET_INTEREST_FORMULA_CONTRACT_JSON_PATH,
        "does not publish a debt path",
        "baseline debt stock",
        "baseline net interest",
        "maturity bucket schedule",
        "effective rate path by bucket",
        "new borrowing timing rule",
        "interest receipts treatment",
        "explicit other financing series",
        "primary-balance feedback test fixture",
        "Net interest is endogenous",
        "Net interest cannot be cut directly",
        "After any primary-balance change",
        "recompute deficit, debt, maturity-bucket debt stock, and subsequent net interest",
        "primary_balance_change_recomputes_debt_and_interest",
        "fixture path is still null",
        "not a net-interest path",
        "not a solver run",
        "not target-cost selection",
        "not rate calculation",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("net-interest reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_interest_pbd_fy2025_2031_current_law_context_path(
    root: &Path,
) -> Result<(), String> {
    for path in [
        NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest PBD current-law context artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "net-interest-pbd-fy2025-2031-current-law-context-path:v1"
        || string_field(&record, "record_family")? != "net_interest_current_law_context_path"
        || string_field(&record, "status")?
            != "draft_pbd_net_interest_context_debt_feedback_blocked"
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "source_id")? != "SRC-OMB-PBD-OUTLAYS-FY2027"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-PBD-OUTLAYS-FY2027/2026-07-13/outlays_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 2_144_756
        || string_field(&record, "raw_sha256")?
            != "d892f2247e6c1aed68414d3e4168f8b4ab97bcfc7acf82a6a449a3fcb1addb07"
    {
        return Err("net-interest PBD context identity failed".to_string());
    }

    let aggregation = record
        .get("aggregation_rule")
        .ok_or("net-interest PBD aggregation rule")?;
    if string_field(aggregation, "workbook_unit")? != "thousands_usd"
        || string_field(aggregation, "output_unit")? != "millions_usd"
        || int_field(aggregation, "row_count")? != 355
        || aggregation
            .get("no_interpolation_used")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || aggregation
            .get("negative_interest_receipts_preserved_in_net_sum")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("net-interest PBD aggregation rule failed".to_string());
    }

    let status = record
        .get("path_status")
        .ok_or("net-interest PBD path status")?;
    for (field, expected) in [
        ("official_fy2025_fy2031_rows_present", true),
        ("source_custody_ready", true),
        ("complete_fy2025_fy2035_path_ready", false),
        ("debt_stock_path_ready", false),
        ("maturity_schedule_ready", false),
        ("rate_path_ready", false),
        ("primary_balance_feedback_ready", false),
        ("solver_ready", false),
        ("rate_ready", false),
        ("savings_ready", false),
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(expected) {
            return Err(format!("net-interest PBD path status {field} failed"));
        }
    }
    if int_field(status, "row_count")? != 7
        || int_field(status, "actual_rows")? != 1
        || int_field(status, "projection_rows")? != 6
    {
        return Err("net-interest PBD path counts failed".to_string());
    }

    let rows = record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest PBD annual rows")?;
    let expected_values = [
        (2025, 970_065),
        (2026, 1_016_650),
        (2027, 1_065_368),
        (2028, 1_166_961),
        (2029, 1_234_644),
        (2030, 1_304_508),
        (2031, 1_363_769),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        observed_years.insert(year);
        let expected = expected_values
            .get(&(year as i32))
            .ok_or("unexpected net-interest PBD year")?;
        if int_field(row, "net_interest_millions")? != i64::from(*expected)
            || int_field(row, "source_row_count")? != 355
        {
            return Err(format!("net-interest PBD row {year} value failed"));
        }
    }
    let expected_years = (2025..=2031).map(i64::from).collect::<BTreeSet<_>>();
    if observed_years != expected_years {
        return Err("net-interest PBD year coverage failed".to_string());
    }

    let missing = record
        .get("missing_year_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest PBD missing rows")?;
    let missing_years = missing
        .iter()
        .map(|row| int_field(row, "fiscal_year"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if missing_years != (2032..=2035).map(i64::from).collect::<BTreeSet<_>>() {
        return Err("net-interest PBD missing-year coverage failed".to_string());
    }
    for row in missing {
        if !row
            .get("net_interest_millions")
            .is_some_and(serde_json::Value::is_null)
        {
            return Err("net-interest PBD missing-year values must stay null".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest PBD blocked outputs")?;
    for field in [
        "complete_fy2025_fy2035_net_interest_path",
        "debt_stock_path",
        "maturity_schedule",
        "rate_path",
        "primary_balance_feedback_fixture",
        "direct_net_interest_cut",
        "solver_input",
        "solver_run",
        "public_rate_card",
        "net_savings",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "net-interest PBD blocked output {field} must stay null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest PBD claims")?;
    for field in [
        "net_interest_pbd_fy2025_2031_current_law_context_path_published",
        "official_fy2025_fy2031_rows_present",
        "source_custody_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("net-interest PBD claim {field} must be true"));
        }
    }
    for field in [
        "complete_fy2025_fy2035_net_interest_path_ready",
        "debt_stock_path_ready",
        "maturity_schedule_ready",
        "rate_path_ready",
        "primary_balance_feedback_ready",
        "direct_net_interest_cut_published",
        "solver_input_ready",
        "solver_run_published",
        "target_cost_published",
        "gross_savings_published",
        "net_savings_published",
        "rate_calculation_published",
        "public_rate_card_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("net-interest PBD claim {field} must be false"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "net-interest context only",
        "Net interest is endogenous",
        "not a direct cut",
        "not solver input",
        "not a savings estimate",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!("net-interest PBD warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_READER_PATH}: {err}"
        )
    })?;
    for required in [
        NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        "FY2025-FY2031",
        "355 workbook rows",
        "explicit nulls for FY2032-FY2035",
        "Net interest is endogenous",
        "not a policy lever or solver-ready feedback model",
    ] {
        if !reader.contains(required) {
            return Err(format!("net-interest PBD reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_interest_outcome_floor_definition_packet(root: &Path) -> Result<(), String> {
    for path in [
        NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "net-interest-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")? != "net_interest_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 166
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(
            &record,
            "revenue_solvency_outcome_floor_definition_packet_path",
        )? != REVENUE_SOLVENCY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "net_interest_formula_contract_path")?
            != NET_INTEREST_FORMULA_CONTRACT_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("net-interest floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("net-interest floor status {field} must be true"));
        }
    }
    for field in [
        "new_external_download_performed",
        "debt_path_ready",
        "maturity_path_ready",
        "rate_path_ready",
        "primary_balance_feedback_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("net-interest floor status {field} must be false"));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest floor definition policy")?;
    for field in [
        "net_interest_is_endogenous",
        "net_interest_cannot_be_cut_directly",
        "primary_balance_changes_must_recompute_debt_and_interest",
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "international_differences_not_savings",
        "no_fraud_inference",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("net-interest floor policy {field} must be true"));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("net-interest required floor class count failed".to_string());
    }
    let observed_classes = classes
        .iter()
        .map(|row| string_field(row, "floor_class"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_class_set = expected_classes
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_classes != expected_class_set {
        return Err("net-interest required floor class set failed".to_string());
    }
    for row in classes {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!("net-interest floor class {field} must be null"));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("net-interest floor class must remain unpassed".to_string());
        }
    }

    let lane_floors = record
        .get("net_interest_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest-specific floor definitions")?;
    let expected_lane_floors = [
        "full_and_timely_debt_service",
        "endogenous_interest_formula",
        "debt_maturity_rate_path",
        "primary_balance_feedback",
        "stress_resilience",
    ];
    if lane_floors.len() != expected_lane_floors.len() {
        return Err("net-interest-specific floor count failed".to_string());
    }
    let observed_lane_floors = lane_floors
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_lane_floor_set = expected_lane_floors
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_lane_floors != expected_lane_floor_set {
        return Err("net-interest-specific floor set failed".to_string());
    }
    for row in lane_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("net-interest-specific floors must remain null and unpassed".to_string());
        }
    }

    for object_name in ["blocked_inputs", "blocked_outputs"] {
        let object = record
            .get(object_name)
            .and_then(serde_json::Value::as_object)
            .ok_or(object_name)?;
        if object
            .values()
            .any(|value| value != &serde_json::Value::Null)
        {
            return Err(format!("{object_name} must remain null"));
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("net_interest_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
    {
        return Err("net-interest floor summary counts failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "all_floors_passed",
        "direct_cut_allowed",
        "solver_input_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("net-interest floor summary {field} must be false"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("net-interest floor packet publication flag failed".to_string());
    }
    for field in [
        "debt_path_ready",
        "maturity_path_ready",
        "rate_path_ready",
        "primary_balance_feedback_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
        "direct_cut_published",
        "target_cost_published",
        "federal_effect_published",
        "gross_savings_published",
        "net_savings_published",
        "solver_input_ready",
        "solver_run_published",
        "public_rate_card_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("net-interest floor claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This net-interest floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "Net interest is endogenous and cannot be cut directly.",
        "Any primary-balance change must recompute subsequent debt and interest before solver use.",
        "No direct cut amount, target cost, federal effect, gross savings, net savings, solver input, solver run, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a debt path",
        "not a maturity path",
        "not a rate path",
        "not a direct cut",
        "not a federal score",
        "not a target-cost selection",
        "not solver input",
        "not a solver run",
        "not a rate calculation",
        "not a savings estimate",
        "not a fraud finding",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "net-interest floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_interest_average_rate_floor_value_packet(root: &Path) -> Result<(), String> {
    for path in [
        NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH,
        NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest average-rate floor-value artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH}: {err}"
                )
            })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "net-interest-average-rate-floor-value-packet:v1"
        || string_field(&record, "record_family")? != "net_interest_average_rate_floor_value_packet"
        || int_field(&record, "pulse")? != 218
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "floor_id")? != "debt_service_rate_path"
        || string_field(&record, "floor_class")? != "adequacy_resilience"
        || string_field(&record, "floor_definition_packet_path")?
            != NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "treasury_average_interest_rate_context_path")?
            != NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH
        || string_field(&record, "net_interest_current_law_context_path")?
            != NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err("net-interest average-rate floor-value identity failed".to_string());
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("net-interest average-rate threshold")?;
    if string_field(threshold, "rationale_id")?
        != "no-regression-from-2026-06-30-total-interest-bearing-debt-average-rate"
        || string_field(threshold, "selected_measure")?
            != "Treasury Total Interest-bearing Debt average interest rate"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_ceiling"
        || (number_field(threshold, "threshold_value")? - 3.409).abs() > 0.000001
        || string_field(threshold, "threshold_unit")? != "percent"
        || !string_field(threshold, "source_table")?.contains("2026-06-30")
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("net-interest average-rate threshold failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("net-interest average-rate baseline")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("net-interest average-rate primary baseline")?;
    if string_field(baseline, "reporting_period")? != "2026-06-30 latest-month context"
        || string_field(primary, "measure")? != "Total Interest-bearing Debt average interest rate"
        || (number_field(primary, "value")? - 3.409).abs() > 0.000001
        || string_field(primary, "unit")? != "percent"
        || string_field(primary, "source_id")?
            != "SRC-TREASURY-FISCALDATA-AVG-INTEREST-RATES-2026-07-24"
        || string_field(primary, "source_path")?
            != NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH
    {
        return Err("net-interest average-rate primary baseline failed".to_string());
    }

    let context = baseline
        .get("supporting_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest average-rate supporting context")?;
    if context.len() != 4 {
        return Err("net-interest average-rate supporting context count failed".to_string());
    }
    let context_values = context
        .iter()
        .map(|row| Ok((string_field(row, "measure")?, row)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for (measure, expected) in [
        ("Total Marketable average interest rate", 3.411),
        ("Total Non-marketable average interest rate", 3.399),
        ("FY2025 OMB PBD net-interest outlay context", 970_065.0),
        (
            "FY2031 OMB PBD projected net-interest outlay context",
            1_363_769.0,
        ),
    ] {
        let row = context_values
            .get(measure)
            .ok_or("net-interest average-rate context missing")?;
        if (number_field(row, "value")? - expected).abs() > 0.000001 {
            return Err(format!(
                "net-interest average-rate context value failed: {measure}"
            ));
        }
    }

    let custody = baseline
        .get("source_custody")
        .ok_or("net-interest average-rate custody")?;
    if string_field(custody, "publisher")?
        != "Bureau of the Fiscal Service, U.S. Department of the Treasury"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/treasury/SRC-TREASURY-FISCALDATA-AVG-INTEREST-RATES-2026-07-24/2026-07-24/avg_interest_rates.csv"
        || int_field(custody, "raw_byte_count")? != 501_180
        || string_field(custody, "raw_sha256")?
            != "48c1c47f506a7cae791fa0b2ff2259094486062dfca3b1c42433eaf6508c252b"
        || string_field(custody, "latest_record_date")? != "2026-06-30"
        || string_field(custody, "review_status")? != "source_metadata_present_and_hash_matched"
    {
        return Err("net-interest average-rate custody failed".to_string());
    }

    let boundary = string_field(baseline, "boundary")?;
    for required in [
        "not a complete fiscal-year rate path",
        "not a debt-stock projection",
        "not a maturity schedule",
        "not primary-balance feedback",
        "not a direct net-interest cut",
        "not solver input",
        "not net savings",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "net-interest average-rate boundary missing {required}"
            ));
        }
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!("net-interest {field} must remain null"));
        }
    }

    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest average-rate readiness")?;
    for field in [
        "threshold_rationale_ready",
        "threshold_value_populated",
        "baseline_value_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("net-interest readiness {field} must be true"));
        }
    }
    for (field, value) in readiness {
        let observed = value.as_bool().ok_or("net-interest readiness bool")?;
        if !matches!(
            field.as_str(),
            "threshold_rationale_ready" | "threshold_value_populated" | "baseline_value_ready"
        ) && observed
        {
            return Err(format!("net-interest readiness {field} must remain false"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest average-rate blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "net-interest blocked output {field} must remain null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest average-rate claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("net-interest claim bool")?;
        if matches!(
            field.as_str(),
            "average_rate_floor_value_packet_published"
                | "threshold_rationale_ready"
                | "threshold_value_populated"
                | "baseline_value_ready"
        ) {
            if !observed {
                return Err(format!("net-interest claim {field} must be true"));
            }
        } else if observed {
            return Err(format!("net-interest claim {field} must remain false"));
        }
    }

    let public_warning = string_field(&record, "public_warning")?;
    for required in [
        "draft no-regression net-interest average-rate floor threshold",
        "not a complete FY2025-FY2035 net-interest path",
        "not a debt-stock projection",
        "not a maturity schedule",
        "not a fiscal-year rate path",
        "not primary-balance feedback",
        "not a direct net-interest cut",
        "not policy values",
        "not stress values",
        "not pass/fail evidence",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !public_warning.contains(required) {
            return Err(format!(
                "net-interest average-rate warning missing {required}"
            ));
        }
    }

    let schema = fs::read_to_string(
        root.join(NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!("failed to read {NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH}: {err}")
    })?;
    if !schema.contains("net_interest_average_rate_floor_value_packet") {
        return Err("net-interest average-rate schema missing record family".to_string());
    }

    let reader = fs::read_to_string(
        root.join(NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_READER_PATH}: {err}")
    })?;
    for required in [
        NET_INTEREST_AVERAGE_RATE_FLOOR_VALUE_PACKET_JSON_PATH,
        NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH,
        NET_INTEREST_PBD_FY2025_2031_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        "3.409 percent",
        "$970,065 million",
        "$1,363,769 million",
        "draft no-regression net-interest average-rate floor threshold",
        "Net interest remains endogenous",
        "policy and stress values remain null",
        "not a direct net-interest cut",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "net-interest average-rate reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_fiscal_package_conversion(root: &Path) -> Result<(), String> {
    let required_paths = [
        NET_LEVEL_3_RECOMPUTATION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/net_level_3_zero_input_endogenous_recomputation.schema.md",
        "docs/reading/net-level-3-zero-input-endogenous-recomputation.md",
        "reviews/2026-07-27-net-level-3-zero-input-endogenous-recomputation-role-review.md",
        NET_LEVEL_4_AUDIT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/net_level_4_endogenous_dependency_audit.schema.md",
        "docs/reading/net-level-4-endogenous-dependency-audit.md",
        "reviews/2026-07-27-net-level-4-endogenous-dependency-audit-role-review.md",
        FISCAL_PACKAGE_NET_BRIDGE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fiscal_package_net_contribution_bridge.schema.md",
        "docs/reading/fiscal-package-net-contribution-bridge.md",
        "reviews/2026-07-27-fiscal-package-net-contribution-bridge-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing NET fiscal conversion artifact: {path}"));
        }
    }

    let recomputation = read_json_artifact(root, NET_LEVEL_3_RECOMPUTATION_JSON_PATH)?;
    let baseline = recomputation
        .get("current_law_baseline")
        .ok_or("NET current-law baseline")?;
    let treasury = recomputation
        .get("latest_treasury_context")
        .ok_or("NET Treasury context")?;
    let result = recomputation
        .get("recomputation")
        .ok_or("NET recomputation")?;
    let sensitivity = recomputation
        .get("mechanical_sensitivity")
        .ok_or("NET mechanical sensitivity")?;
    let decision = recomputation
        .get("decision")
        .ok_or("NET Level-3 decision")?;
    if int_field(&recomputation, "pulse")? != 437
        || (number_field(baseline, "fy2026_debt_held_by_public_billions")? - 32095.165).abs()
            > 0.001
        || (number_field(baseline, "fy2026_net_interest_billions")? - 1038.976).abs() > 0.001
        || (number_field(treasury, "total_interest_bearing_debt_average_rate_percent")? - 3.409)
            .abs()
            > 0.0001
        || int_field(result, "admitted_primary_adjustment_billions")? != 0
        || int_field(result, "fy2026_debt_delta_billions")? != 0
        || int_field(result, "fy2026_net_interest_delta_billions")? != 0
        || (number_field(sensitivity, "one_year_carry_billions")? - 17.045).abs() > 0.001
        || bool_field(sensitivity, "models_within_year_timing")?
        || bool_field(sensitivity, "models_maturity_rollover")?
        || bool_field(sensitivity, "models_interest_receipts")?
        || bool_field(sensitivity, "is_savings_estimate")?
        || !bool_field(decision, "zero_input_recomputation_ready")?
        || bool_field(decision, "nonzero_endogenous_effect_ready")?
        || bool_field(decision, "direct_cut_allowed")?
    {
        return Err("NET Level-3 zero-input recomputation failed".to_string());
    }
    validate_blocked_outputs_null(&recomputation, "NET Level-3 recomputation")?;

    let audit = read_json_artifact(root, NET_LEVEL_4_AUDIT_JSON_PATH)?;
    let gates = audit
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("NET audit gates")?;
    let candidate = audit
        .get("candidate_decision")
        .ok_or("NET candidate decision")?;
    let pass_count = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|value| value == "evidence_pass"))
        .count();
    let blocked_count = gates
        .iter()
        .filter(|row| {
            string_field(row, "disposition").is_ok_and(|value| value == "required_blocked")
        })
        .count();
    if int_field(&audit, "pulse")? != 438
        || gates.len() != 8
        || pass_count != 3
        || blocked_count != 5
        || !bool_field(candidate, "zero_input_fixture_valid")?
        || bool_field(candidate, "nonzero_endogenous_result_valid")?
        || bool_field(candidate, "direct_net_interest_cut_valid")?
        || bool_field(candidate, "candidate_admitted_to_spending_package")?
    {
        return Err("NET Level-4 endogenous dependency audit failed".to_string());
    }
    validate_blocked_outputs_null(&audit, "NET Level-4 audit")?;

    let package = read_json_artifact(root, FISCAL_PACKAGE_NET_BRIDGE_JSON_PATH)?;
    let package_state = package.get("package_state").ok_or("NET package state")?;
    let net_state = package.get("net_state").ok_or("NET state")?;
    let package_decision = package.get("decision").ok_or("NET package decision")?;
    if int_field(&package, "pulse")? != 439
        || int_field(
            package_state,
            "admitted_primary_spending_correction_billions",
        )? != 0
        || (number_field(net_state, "fy2026_current_law_net_interest_billions")? - 1038.976).abs()
            > 0.001
        || int_field(
            net_state,
            "fy2026_endogenous_effect_from_admitted_paths_billions",
        )? != 0
        || (number_field(net_state, "mechanical_full_year_500b_carry_billions")? - 17.045).abs()
            > 0.001
        || bool_field(net_state, "mechanical_carry_admitted")?
        || bool_field(package_decision, "nonzero_net_effect_admitted")?
        || bool_field(package_decision, "package_spending_correction_ready")?
        || !bool_field(package_decision, "all_five_fiscal_tracks_converted")?
        || string_field(package_decision, "next_decisive_track")? != "REV-Level-2"
    {
        return Err("fiscal-package NET contribution bridge failed".to_string());
    }
    validate_blocked_outputs_null(&package, "fiscal-package NET bridge")?;
    Ok(())
}

pub(crate) fn validate_net_interest_treasury_mspd_maturity_detail_context(root: &Path) -> Result<(), String> {
    for path in [
        NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH,
        NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest Treasury MSPD maturity detail artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "net-interest-treasury-mspd-maturity-detail-context:v1"
        || string_field(&record, "record_family")?
            != "net_interest_treasury_mspd_maturity_detail_context"
        || string_field(&record, "status")?
            != "draft_treasury_mspd_maturity_detail_context_solver_blocked"
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "source_id")? != "SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24.2026-07-24.metadata.md"
    {
        return Err("net-interest Treasury MSPD maturity detail identity failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("net-interest Treasury MSPD source boundary")?;
    for field in [
        "official_public_source",
        "complete_paged_local_csv_custody",
        "context_only",
        "not_weighted_average_maturity",
        "not_remaining_maturity_schedule",
        "not_cbo_omb_projection_bridge",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "net-interest Treasury MSPD boundary flag {field} must be true"
            ));
        }
    }
    if string_field(boundary, "latest_record_date")? != "2026-06-30" {
        return Err("net-interest Treasury MSPD latest date failed".to_string());
    }

    let files = record
        .get("raw_context_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest Treasury MSPD raw context files")?;
    let expected = [
        (
            "mspd_table_3",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24/2026-07-24/mspd_table_3.full.csv",
                55_726_310,
                "347ff7878822ec7cf108b575cc440fe104dea4f0f9688cc0da89f5ce815a67c4",
                217_144,
                1_088,
                878,
                875,
                764,
                875,
                475,
            ),
        ),
        (
            "mspd_table_5",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24/2026-07-24/mspd_table_5.full.csv",
                13_156_635,
                "801fceb68ece6ae48640697b7a6ab2dec65bcdb7c899b2b3fbe1c3bd133c5867",
                86_145,
                409,
                405,
                0,
                405,
                0,
                409,
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if files.len() != expected.len() {
        return Err("net-interest Treasury MSPD file count failed".to_string());
    }
    for file in files {
        let table = string_field(file, "table")?;
        let (
            path,
            bytes,
            sha,
            total_rows,
            latest_rows,
            maturity_rows,
            issue_rows,
            rate_rows,
            yield_rows,
            outstanding_rows,
        ) = expected
            .get(table.as_str())
            .ok_or("unexpected net-interest Treasury MSPD table")?;
        if string_field(file, "raw_artifact_path")? != *path
            || int_field(file, "raw_byte_count")? != *bytes
            || string_field(file, "raw_sha256")? != *sha
            || int_field(file, "total_rows")? != *total_rows
            || string_field(file, "latest_record_date")? != "2026-06-30"
            || int_field(file, "latest_record_date_rows")? != *latest_rows
            || int_field(file, "latest_rows_with_maturity_date")? != *maturity_rows
            || int_field(file, "latest_rows_with_issue_date")? != *issue_rows
            || int_field(file, "latest_rows_with_interest_rate")? != *rate_rows
            || int_field(file, "latest_rows_with_yield")? != *yield_rows
            || int_field(file, "latest_rows_with_outstanding_amount")? != *outstanding_rows
            || !root.join(path).exists()
        {
            return Err(format!(
                "net-interest Treasury MSPD file coverage failed: {table}"
            ));
        }
    }

    for array_name in ["field_coverage_use", "blocked_model_steps"] {
        if record
            .get(array_name)
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "net-interest Treasury MSPD {array_name} must be nonempty"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury MSPD blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "net-interest Treasury MSPD blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury MSPD claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("net-interest Treasury MSPD claim bool")?;
        if matches!(
            field.as_str(),
            "net_interest_treasury_mspd_maturity_detail_context_published"
                | "complete_paged_local_csv_custody"
                | "latest_month_field_coverage_recorded"
        ) {
            if !observed {
                return Err(format!(
                    "net-interest Treasury MSPD claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "net-interest Treasury MSPD claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "maturity-detail custody is locally captured",
        "not a weighted average maturity",
        "not a remaining-maturity schedule",
        "not a CBO/OMB fiscal-year projection bridge",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "net-interest Treasury MSPD warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH,
        "MSPD Table 3: 217,144 total rows",
        "MSPD Table 5: 86,145 total rows",
        "2026-06-30",
        "not a weighted average maturity",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "net-interest Treasury MSPD reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic(
    root: &Path,
) -> Result<(), String> {
    for path in [
        NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH,
        NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest Treasury MSPD remaining maturity diagnostic artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(
        NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "net-interest-treasury-mspd-remaining-maturity-bucket-diagnostic:v1"
        || string_field(&record, "record_family")?
            != "net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic"
        || string_field(&record, "status")?
            != "draft_partial_mspd_bucket_diagnostic_reconciliation_blocked"
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "source_id")? != "SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-TREASURY-FISCALDATA-MSPD-DETAIL-2026-07-24.2026-07-24.metadata.md"
        || string_field(&record, "maturity_detail_context_path")?
            != NET_INTEREST_TREASURY_MSPD_MATURITY_DETAIL_CONTEXT_JSON_PATH
    {
        return Err(
            "net-interest Treasury MSPD remaining maturity diagnostic identity failed".to_string(),
        );
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("net-interest Treasury MSPD remaining maturity boundary")?;
    for field in [
        "official_public_source",
        "diagnostic_bucket_coverage_only",
        "table_3_and_table_5_not_combined",
        "amount_unit_and_perimeter_reconciliation_required",
        "not_weighted_average_maturity",
        "not_remaining_maturity_schedule",
        "not_debt_stock_projection",
        "not_cbo_omb_projection_bridge",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity boundary flag {field} must be true"
            ));
        }
    }
    if string_field(boundary, "latest_record_date")? != "2026-06-30"
        || string_field(boundary, "retrieval_date")? != "2026-07-24"
    {
        return Err(
            "net-interest Treasury MSPD remaining maturity boundary date failed".to_string(),
        );
    }

    let method = record
        .get("bucket_method")
        .ok_or("net-interest Treasury MSPD bucket method")?;
    if string_field(method, "record_date")? != "2026-06-30"
        || string_field(method, "amount_field")? != "outstanding_amt"
        || !string_field(method, "remaining_days_basis")?
            .contains("maturity_date minus record_date")
    {
        return Err("net-interest Treasury MSPD bucket method failed".to_string());
    }
    let edges = method
        .get("bucket_edges")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest Treasury MSPD bucket edges")?;
    if edges.len() != 7 {
        return Err("net-interest Treasury MSPD bucket edge count failed".to_string());
    }

    let diagnostics = record
        .get("table_diagnostics")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest Treasury MSPD table diagnostics")?;
    let expected_tables = BTreeMap::from([
        (
            "mspd_table_3",
            (
                1_088,
                462,
                "31082178.32343828",
                [
                    ("lte_1y", 112, "10146468.31559119"),
                    ("gt_1y_lte_3y", 110, "6636027.78188565"),
                    ("gt_3y_lte_5y", 68, "4271384.57657272"),
                    ("gt_5y_lte_10y", 56, "4310030.81901257"),
                    ("gt_10y_lte_20y", 66, "2863280.78527407"),
                    ("gt_20y_lte_30y", 50, "2854986.04510208"),
                    ("gt_30y", 0, "0"),
                ],
            ),
        ),
        (
            "mspd_table_5",
            (
                409,
                405,
                "23685187972.23828",
                [
                    ("lte_1y", 59, "3103105125.49119"),
                    ("gt_1y_lte_3y", 106, "6282400620.78565"),
                    ("gt_3y_lte_5y", 68, "4271384576.57272"),
                    ("gt_5y_lte_10y", 56, "4310030819.01257"),
                    ("gt_10y_lte_20y", 66, "2863280785.27407"),
                    ("gt_20y_lte_30y", 50, "2854986045.10208"),
                    ("gt_30y", 0, "0"),
                ],
            ),
        ),
    ]);
    if diagnostics.len() != expected_tables.len() {
        return Err("net-interest Treasury MSPD remaining maturity table count failed".to_string());
    }
    for diagnostic in diagnostics {
        let table = string_field(diagnostic, "table")?;
        let (latest_rows, usable_rows, total, expected_buckets) = expected_tables
            .get(table.as_str())
            .ok_or("unexpected net-interest Treasury MSPD diagnostic table")?;
        if int_field(diagnostic, "latest_record_date_rows")? != *latest_rows
            || int_field(diagnostic, "usable_rows")? != *usable_rows
            || string_field(diagnostic, "total_outstanding_amt_raw_units")? != *total
        {
            return Err(format!(
                "net-interest Treasury MSPD diagnostic totals failed: {table}"
            ));
        }
        let buckets = diagnostic
            .get("buckets")
            .and_then(serde_json::Value::as_array)
            .ok_or("net-interest Treasury MSPD diagnostic buckets")?;
        if buckets.len() != expected_buckets.len() {
            return Err(format!(
                "net-interest Treasury MSPD diagnostic bucket count failed: {table}"
            ));
        }
        let observed_buckets = buckets
            .iter()
            .map(|bucket| {
                Ok((
                    string_field(bucket, "bucket")?,
                    int_field(bucket, "row_count")?,
                    string_field(bucket, "outstanding_amt_raw_units")?,
                ))
            })
            .collect::<Result<BTreeSet<_>, String>>()?;
        let expected_buckets = expected_buckets
            .iter()
            .map(|(bucket, rows, amount)| (bucket.to_string(), *rows, amount.to_string()))
            .collect::<BTreeSet<_>>();
        if observed_buckets != expected_buckets {
            return Err(format!(
                "net-interest Treasury MSPD diagnostic buckets failed: {table}"
            ));
        }
    }

    for array_name in ["reconciliation_findings", "blocked_model_steps"] {
        if record
            .get(array_name)
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity {array_name} must be nonempty"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury MSPD remaining maturity blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury MSPD remaining maturity claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("net-interest Treasury MSPD remaining maturity claim bool")?;
        if matches!(
            field.as_str(),
            "net_interest_treasury_mspd_remaining_maturity_bucket_diagnostic_published"
                | "latest_month_bucket_diagnostic_recorded"
        ) {
            if !observed {
                return Err(format!(
                    "net-interest Treasury MSPD remaining maturity claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "remaining-maturity bucket diagnostics are recorded",
        "Table 3 and Table 5 are not combined",
        "not a weighted average maturity",
        "not a remaining-maturity schedule",
        "not a CBO/OMB fiscal-year projection bridge",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(
        NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        NET_INTEREST_TREASURY_MSPD_REMAINING_MATURITY_BUCKET_DIAGNOSTIC_JSON_PATH,
        "MSPD Table 3: 1,088 latest-month rows, 462 usable rows",
        "MSPD Table 5: 409 latest-month rows, 405 usable rows",
        "Table 3 and Table 5 are not combined",
        "not a remaining-maturity schedule",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "net-interest Treasury MSPD remaining maturity reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_net_interest_treasury_average_interest_rate_context(root: &Path) -> Result<(), String> {
    for path in [
        NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH,
        NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing net-interest Treasury average-rate artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "net-interest-treasury-average-interest-rate-context:v1"
        || string_field(&record, "record_family")?
            != "net_interest_treasury_average_interest_rate_context"
        || string_field(&record, "status")?
            != "draft_treasury_average_rate_context_rate_path_blocked"
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "net-interest"
        || string_field(&record, "source_id")?
            != "SRC-TREASURY-FISCALDATA-AVG-INTEREST-RATES-2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-TREASURY-FISCALDATA-AVG-INTEREST-RATES-2026-07-24.2026-07-24.metadata.md"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/treasury/SRC-TREASURY-FISCALDATA-AVG-INTEREST-RATES-2026-07-24/2026-07-24/avg_interest_rates.csv"
        || int_field(&record, "raw_byte_count")? != 501_180
        || string_field(&record, "raw_sha256")?
            != "48c1c47f506a7cae791fa0b2ff2259094486062dfca3b1c42433eaf6508c252b"
        || int_field(&record, "raw_row_count")? != 4_977
        || string_field(&record, "latest_record_date")? != "2026-06-30"
        || int_field(&record, "latest_row_count")? != 16
        || string_field(&record, "unit")? != "percent"
    {
        return Err("net-interest Treasury average-rate identity failed".to_string());
    }
    let raw_path = root.join(string_field(&record, "raw_artifact_path")?);
    if !raw_path.exists()
        || fs::metadata(&raw_path)
            .map_err(|err| err.to_string())?
            .len()
            != 501_180
        || sha256_file(&raw_path)?
            != "48c1c47f506a7cae791fa0b2ff2259094486062dfca3b1c42433eaf6508c252b"
        || !root.join(string_field(&record, "metadata_path")?).exists()
    {
        return Err("net-interest Treasury average-rate custody failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("net-interest Treasury average-rate source boundary")?;
    for field in [
        "official_public_source",
        "local_raw_custody_ready",
        "latest_month_context_only",
        "not_fiscal_year_rate_path",
        "not_cbo_omb_projection_bridge",
        "not_debt_stock_projection",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "net-interest Treasury average-rate boundary {field} failed"
            ));
        }
    }

    let check_number =
        |object: &serde_json::Value, field: &str, expected: f64| -> Result<(), String> {
            let observed = object
                .get(field)
                .and_then(serde_json::Value::as_f64)
                .ok_or_else(|| format!("net-interest Treasury average-rate missing {field}"))?;
            if (observed - expected).abs() > 0.000_001 {
                return Err(format!("net-interest Treasury average-rate failed {field}"));
            }
            Ok(())
        };

    let rows = record
        .get("latest_average_interest_rate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("net-interest Treasury average-rate latest rows")?;
    let expected_rows = [
        (1, "Marketable", "Treasury Bills", 3.706),
        (2, "Marketable", "Treasury Notes", 3.283),
        (3, "Marketable", "Treasury Bonds", 3.430),
        (
            4,
            "Marketable",
            "Treasury Inflation-Protected Securities (TIPS)",
            1.090,
        ),
        (5, "Marketable", "Treasury Floating Rate Notes (FRN)", 3.512),
        (6, "Marketable", "Federal Financing Bank", 2.383),
        (7, "Marketable", "Total Marketable", 3.411),
        (8, "Non-marketable", "Domestic Series", 7.577),
        (9, "Non-marketable", "Special Purpose Vehicle", 2.898),
        (
            10,
            "Non-marketable",
            "State and Local Government Series",
            3.336,
        ),
        (
            11,
            "Non-marketable",
            "United States Savings Securities",
            3.160,
        ),
        (
            12,
            "Non-marketable",
            "United States Savings Inflation Securities",
            4.418,
        ),
        (13, "Non-marketable", "Government Account Series", 3.395),
        (
            14,
            "Non-marketable",
            "Government Account Series Inflation Securities",
            1.391,
        ),
        (15, "Non-marketable", "Total Non-marketable", 3.399),
        (
            16,
            "Interest-bearing Debt",
            "Total Interest-bearing Debt",
            3.409,
        ),
    ]
    .into_iter()
    .map(|(line, security_type, security, rate)| {
        (line, security_type.to_string(), security.to_string(), rate)
    })
    .collect::<Vec<_>>();
    if rows.len() != expected_rows.len() {
        return Err("net-interest Treasury average-rate row count failed".to_string());
    }
    for (row, (line, security_type, security, rate)) in rows.iter().zip(expected_rows) {
        if int_field(row, "source_line_number")? != line
            || string_field(row, "security_type")? != security_type
            || string_field(row, "security")? != security
        {
            return Err(format!(
                "net-interest Treasury average-rate row identity failed: {line}"
            ));
        }
        check_number(row, "average_interest_rate_percent", rate)?;
    }

    let totals = record
        .get("latest_totals")
        .ok_or("net-interest Treasury average-rate totals")?;
    for (field, expected) in [
        ("total_marketable_average_rate_percent", 3.411),
        ("total_non_marketable_average_rate_percent", 3.399),
        ("total_interest_bearing_debt_average_rate_percent", 3.409),
    ] {
        check_number(totals, field, expected)?;
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury average-rate blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "net-interest Treasury average-rate blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("net-interest Treasury average-rate claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("net-interest Treasury average-rate claim bool")?;
        if matches!(
            field.as_str(),
            "treasury_average_interest_rate_context_published"
                | "local_raw_custody_ready"
                | "latest_month_rate_context_ready"
        ) {
            if !observed {
                return Err(format!(
                    "net-interest Treasury average-rate claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "net-interest Treasury average-rate claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "latest-month rate context",
        "Total Interest-bearing Debt 3.409 percent",
        "not a fiscal-year rate path",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "net-interest Treasury average-rate warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        NET_INTEREST_TREASURY_AVERAGE_INTEREST_RATE_CONTEXT_JSON_PATH,
        "2026-06-30",
        "Total Marketable: `3.411` percent",
        "Total Interest-bearing Debt: `3.409` percent",
        "4977",
        "not a fiscal-year rate path",
        "not solver input",
        "balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "net-interest Treasury average-rate reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

