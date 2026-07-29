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

pub(crate) fn validate_solver_accounting_readiness_gate(root: &Path) -> Result<(), String> {
    for path in [
        SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH,
        SOLVER_ACCOUNTING_READINESS_GATE_SCHEMA_PATH,
        SOLVER_ACCOUNTING_READINESS_GATE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing solver accounting gate artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let gate: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&gate, "record_id")? != "solver-accounting-readiness-gate:v1"
        || string_field(&gate, "record_family")? != "solver_accounting_readiness_gate"
        || int_field(&gate, "pulse")? != 100
        || string_field(&gate, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&gate, "deterministic_annual_update_simulator_contract_path")?
            != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&gate, "fund_group_fy2025_reconciliation_fixture_path")?
            != FUND_GROUP_FY2025_RECONCILIATION_FIXTURE_JSON_PATH
        || string_field(&gate, "transportation_partial_federal_outlay_path")?
            != TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH
        || string_field(&gate, "transportation_trust_fund_accounting_boundary_path")?
            != TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_JSON_PATH
    {
        return Err("solver accounting gate identity failed".to_string());
    }

    let summary = gate
        .get("readiness_summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver accounting readiness summary")?;
    for true_flag in [
        "aggregate_accounting_fixture_available",
        "aggregate_fixture_may_seed_rounding_tests",
        "aggregate_fixture_may_seed_deficit_sign_tests",
        "aggregate_fixture_may_seed_fund_balance_tests",
    ] {
        if summary.get(true_flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("solver accounting true flag {true_flag} failed"));
        }
    }
    for false_flag in [
        "deterministic_solver_ready",
        "transportation_solver_ready",
        "balanced_rate_ready",
        "balanced_budget_claim_allowed",
    ] {
        if summary.get(false_flag).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("solver accounting false flag {false_flag} failed"));
        }
    }

    let inputs = gate
        .get("required_solver_inputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver accounting required inputs")?;
    let observed_inputs = inputs
        .iter()
        .map(|row| string_field(row, "input_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "full_17_row_fy2025_ledger",
        "ten_year_plus_baseline_horizon",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
        "reserves_path",
        "explicit_interfund_transfers",
        "credited_offsetting_collections",
        "net_interest_formula",
        "assigned_receipt_bases",
        "distributional_effect_placeholder",
    ] {
        if !observed_inputs.contains(required) {
            return Err(format!("solver accounting input missing {required}"));
        }
    }
    for row in inputs {
        if row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("solver accounting inputs must remain false/null".to_string());
        }
    }

    let allowed = gate
        .get("allowed_uses")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver accounting allowed uses")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "unit_test_public_rounding_residual_line",
        "unit_test_deficit_positive_financing_need_sign",
        "unit_test_trust_fund_group_balance_arithmetic",
        "document_aggregate_federal_fund_trust_fund_context",
    ] {
        if !allowed.contains(required) {
            return Err(format!("solver accounting allowed use missing {required}"));
        }
    }

    let prohibited = gate
        .get("prohibited_uses")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver accounting prohibited uses")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "populate_transportation_trust_fund_values",
        "populate_lane_target_cost",
        "populate_rate_fields",
        "populate_savings_fields",
        "run_solver",
        "claim_balanced_budget",
        "infer_waste",
        "infer_fraud",
        "infer_department_cut",
        "infer_technology_savings",
    ] {
        if !prohibited.contains(required) {
            return Err(format!(
                "solver accounting prohibited use missing {required}"
            ));
        }
    }

    let outputs = gate
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver accounting outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!("solver accounting output {field} must remain null"));
        }
    }

    let claims = gate
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver accounting claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("solver accounting claim bool")?;
        if field == "solver_accounting_readiness_gate_published" {
            if !observed {
                return Err("solver accounting gate publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "solver accounting public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&gate, "non_claim_boundary")?;
    for required in [
        "solver accounting readiness gate",
        "not a solver run",
        "not transportation trust-fund values",
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
            return Err(format!("solver accounting boundary missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(SOLVER_ACCOUNTING_READINESS_GATE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH,
        "can be used for accounting tests",
        "cannot run the solver",
        "test the public rounding residual line",
        "deficit is recorded as positive financing need",
        "aggregate trust-fund group balance arithmetic",
        "transportation trust-fund values",
        "solver run",
        "target-cost selection",
        "rate calculation",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "balanced-budget claim",
        "OASDI, Medicare HI, transportation trust, general fund",
        "endogenous net interest",
    ] {
        if !reader.contains(required) {
            return Err(format!("solver accounting reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_solver_input_inventory(root: &Path) -> Result<(), String> {
    for path in [
        SOLVER_INPUT_INVENTORY_JSON_PATH,
        SOLVER_INPUT_INVENTORY_SCHEMA_PATH,
        SOLVER_INPUT_INVENTORY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing solver input inventory artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(SOLVER_INPUT_INVENTORY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let inventory: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&inventory, "record_id")? != "solver-input-inventory:v1"
        || string_field(&inventory, "record_family")? != "solver_input_inventory"
        || int_field(&inventory, "pulse")? != 101
        || string_field(&inventory, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(
            &inventory,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&inventory, "solver_accounting_readiness_gate_path")?
            != SOLVER_ACCOUNTING_READINESS_GATE_JSON_PATH
        || string_field(&inventory, "fund_group_fy2025_reconciliation_fixture_path")?
            != FUND_GROUP_FY2025_RECONCILIATION_FIXTURE_JSON_PATH
    {
        return Err("solver input inventory identity failed".to_string());
    }

    let rows = inventory
        .get("inventory_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver input inventory rows")?;
    let observed = rows
        .iter()
        .map(|row| string_field(row, "input_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
        "reserves_path",
        "explicit_interfund_transfers",
        "credited_offsetting_collections",
        "net_interest_formula",
        "assigned_receipt_bases",
        "distributional_effect_placeholder",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || rows.len() != expected.len() {
        return Err("solver input inventory row set failed".to_string());
    }
    for row in rows {
        if row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("solver input inventory rows must remain false/null".to_string());
        }
        let missing = row
            .get("missing_for_solver")
            .and_then(serde_json::Value::as_array)
            .ok_or("solver input inventory missing list")?;
        if missing.is_empty() {
            return Err("solver input inventory rows must name missing evidence".to_string());
        }
    }

    let row_by_id = rows
        .iter()
        .map(|row| Ok((string_field(row, "input_id")?, row)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for (input_id, required_status) in [
        ("oasdi_fund_path", "missing"),
        ("medicare_hi_fund_path", "missing"),
        (
            "transportation_trust_fund_path",
            "accounting_boundary_only_no_annual_values",
        ),
        ("net_interest_formula", "missing_endogenous_formula"),
        (
            "assigned_receipt_bases",
            "missing_behavior_distribution_administration",
        ),
    ] {
        if string_field(
            row_by_id.get(input_id).ok_or("solver input row lookup")?,
            "coverage_status",
        )? != required_status
        {
            return Err(format!(
                "solver input inventory status failed for {input_id}"
            ));
        }
    }

    let actions = inventory
        .get("next_bounded_actions")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver input next actions")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "capture official annual OASDI fund path",
        "capture official annual Medicare HI fund path",
        "capture official transportation trust-fund annual values",
        "create reserve rule contract",
        "create endogenous net-interest formula contract",
        "create assigned receipt-base inventory",
    ] {
        if !actions.contains(required) {
            return Err(format!("solver input next action missing {required}"));
        }
    }

    let claims = inventory
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver input inventory claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("solver input inventory claim bool")?;
        if field == "solver_input_inventory_published" {
            if !observed {
                return Err("solver input inventory publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "solver input inventory public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&inventory, "non_claim_boundary")?;
    for required in [
        "solver input inventory",
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
            return Err(format!(
                "solver input inventory boundary missing {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(SOLVER_INPUT_INVENTORY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        SOLVER_INPUT_INVENTORY_JSON_PATH,
        "the solver is not ready",
        "OASDI annual fund path",
        "Medicare HI annual fund path",
        "transportation trust-fund annual values",
        "endogenous net-interest formula",
        "assigned receipt bases",
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
            return Err(format!("solver input inventory reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_solver_input_readiness_rollup(root: &Path) -> Result<(), String> {
    for path in [
        SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH,
        SOLVER_INPUT_READINESS_ROLLUP_SCHEMA_PATH,
        SOLVER_INPUT_READINESS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing solver input readiness artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&rollup, "record_id")? != "solver-input-readiness-rollup:v1"
        || string_field(&rollup, "record_family")? != "solver_input_readiness_rollup"
        || int_field(&rollup, "pulse")? != 107
        || string_field(&rollup, "solver_input_inventory_path")? != SOLVER_INPUT_INVENTORY_JSON_PATH
        || string_field(&rollup, "reserve_rule_contract_path")? != RESERVE_RULE_CONTRACT_JSON_PATH
        || string_field(&rollup, "reserve_parameter_readiness_gate_path")?
            != RESERVE_PARAMETER_READINESS_GATE_JSON_PATH
        || string_field(&rollup, "net_interest_formula_contract_path")?
            != NET_INTEREST_FORMULA_CONTRACT_JSON_PATH
        || string_field(&rollup, "assigned_receipt_base_inventory_path")?
            != ASSIGNED_RECEIPT_BASE_INVENTORY_JSON_PATH
        || string_field(&rollup, "distributional_effect_placeholder_path")?
            != DISTRIBUTIONAL_EFFECT_PLACEHOLDER_JSON_PATH
    {
        return Err("solver input readiness rollup identity failed".to_string());
    }

    let rows = rollup
        .get("rollup_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver input readiness rows")?;
    let observed = rows
        .iter()
        .map(|row| string_field(row, "input_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
        "reserves_path",
        "explicit_interfund_transfers",
        "credited_offsetting_collections",
        "net_interest_formula",
        "assigned_receipt_bases",
        "distributional_effect_placeholder",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || rows.len() != expected.len() {
        return Err("solver input readiness row set failed".to_string());
    }

    let rows_by_id = rows
        .iter()
        .map(|row| Ok((string_field(row, "input_id")?, row)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for row in rows {
        if row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("solver input readiness rows must remain false/null".to_string());
        }
        let blockers = row
            .get("remaining_blockers")
            .and_then(serde_json::Value::as_array)
            .ok_or("solver input readiness blockers")?;
        if blockers.is_empty() {
            return Err("solver input readiness rows must name blockers".to_string());
        }
    }
    for (input_id, path, status) in [
        (
            "reserves_path",
            RESERVE_PARAMETER_READINESS_GATE_JSON_PATH,
            "contract_and_parameter_gate_only",
        ),
        (
            "net_interest_formula",
            NET_INTEREST_FORMULA_CONTRACT_JSON_PATH,
            "formula_contract_only_inputs_missing",
        ),
        (
            "assigned_receipt_bases",
            ASSIGNED_RECEIPT_BASE_INVENTORY_JSON_PATH,
            "inventory_only_amounts_behavior_distribution_missing",
        ),
        (
            "distributional_effect_placeholder",
            DISTRIBUTIONAL_EFFECT_PLACEHOLDER_JSON_PATH,
            "placeholder_only_distribution_values_missing",
        ),
    ] {
        let row = rows_by_id
            .get(input_id)
            .ok_or("solver input readiness linked row")?;
        if string_field(row, "current_artifact_path")? != path
            || string_field(row, "rollup_status")? != status
        {
            return Err(format!(
                "solver input readiness linked row failed {input_id}"
            ));
        }
    }

    let aggregate = rollup
        .get("aggregate_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver input readiness aggregate status")?;
    for (field, value) in aggregate {
        let observed = value
            .as_bool()
            .ok_or("solver input readiness aggregate bool")?;
        if field == "current_cost_reconciled" {
            if !observed {
                return Err("current_cost_reconciled must remain true".to_string());
            }
        } else if observed {
            return Err(format!(
                "solver input readiness aggregate {field} must be false"
            ));
        }
    }

    let claims = rollup
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver input readiness claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("solver input readiness claim bool")?;
        if field == "solver_input_readiness_rollup_published" {
            if !observed {
                return Err("solver input readiness publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "solver input readiness public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&rollup, "non_claim_boundary")?;
    for required in [
        "solver-input readiness rollup",
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
            return Err(format!(
                "solver input readiness boundary missing {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(SOLVER_INPUT_READINESS_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH,
        "does not make the solver ready",
        "Every solver input remains not ready and null",
        "reserves path: contract and parameter gate only",
        "net interest formula: formula contract only",
        "assigned receipt bases: inventory only",
        "distributional effect placeholder: placeholder only",
        "OASDI annual fund path",
        "Medicare HI annual fund path",
        "transportation trust-fund annual values",
        "explicit deficit gap",
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
            return Err(format!("solver input readiness reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_solver_rate_wave_f_readiness(root: &Path) -> Result<(), String> {
    for path in [
        SOLVER_RATE_WAVE_F_READINESS_JSON_PATH,
        SOLVER_RATE_WAVE_F_READINESS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing solver/rate Wave F artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(SOLVER_RATE_WAVE_F_READINESS_JSON_PATH))
        .map_err(|err| format!("failed to read {SOLVER_RATE_WAVE_F_READINESS_JSON_PATH}: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {SOLVER_RATE_WAVE_F_READINESS_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "solver-rate-wave-f-readiness:v1"
        || string_field(&record, "record_family")? != "solver_rate_wave_f_readiness"
        || string_field(&record, "status")?
            != "wave_f_complete_deterministic_calibration_substantive_solver_and_rates_blocked"
        || string_field(&record, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "lane_scenario_pack_wave_e_readiness_path")?
            != LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH
        || string_field(&record, "wave_f_transportation_calibration_path")?
            != WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH
    {
        return Err("solver/rate Wave F identity failed".to_string());
    }

    let prerequisites = record
        .get("required_prerequisites")
        .and_then(serde_json::Value::as_array)
        .ok_or("solver/rate Wave F prerequisites")?;
    if prerequisites.len() < 9 {
        return Err("solver/rate Wave F prerequisites too short".to_string());
    }
    for row in prerequisites {
        if string_field(row, "prerequisite_id")?.is_empty()
            || string_field(row, "status")? != "blocked"
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("solver/rate Wave F prerequisite shape failed".to_string());
        }
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("solver/rate Wave F aggregate")?;
    for (field, expected) in [
        ("prerequisite_count", prerequisites.len() as i64),
        ("calibration_interfaces_ready", prerequisites.len() as i64),
        ("ready_prerequisites", 0),
        ("solver_inputs_ready", 0),
        ("public_rates_ready", 0),
    ] {
        if int_field(aggregate, field)? != expected {
            return Err(format!("solver/rate Wave F aggregate failed: {field}"));
        }
    }
    for field in [
        "solver_ready",
        "rates_ready",
        "public_rate_cards_ready",
        "savings_ready",
        "balanced_budget_ready",
    ] {
        if aggregate.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("solver/rate Wave F flag failed: {field}"));
        }
    }
    for field in ["wave_f_done", "deterministic_solver_dry_run_ready"] {
        if aggregate.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "solver/rate Wave F completion flag failed: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("solver/rate Wave F claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("solver/rate Wave F claim bool")?;
        if matches!(
            field.as_str(),
            "solver_rate_wave_f_readiness_published"
                | "wave_f_done"
                | "deterministic_solver_dry_run_ready"
        ) {
            if !observed {
                return Err("solver/rate Wave F published flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("solver/rate Wave F claim must be false: {field}"));
        }
    }

    let reader =
        fs::read_to_string(root.join(SOLVER_RATE_WAVE_F_READINESS_READER_PATH)).map_err(|err| {
            format!("failed to read {SOLVER_RATE_WAVE_F_READINESS_READER_PATH}: {err}")
        })?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        SOLVER_RATE_WAVE_F_READINESS_JSON_PATH,
        "Wave F is complete as a deterministic transportation calibration",
        "Zero substantive prerequisites are ready",
        "dry-run-ready",
        "not substantive-solver-ready",
        "not rate-ready",
        "not public-card-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader_words.contains(required) {
            return Err(format!("solver/rate Wave F reader missing {required}"));
        }
    }

    Ok(())
}

