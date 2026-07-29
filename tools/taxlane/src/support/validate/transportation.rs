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

pub(crate) fn validate_transportation_pilot_source_plan(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH,
        TRANSPORTATION_PILOT_SOURCE_PLAN_SCHEMA_PATH,
        TRANSPORTATION_PILOT_SOURCE_PLAN_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation pilot source plan artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let plan: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&plan, "record_id")? != "transportation-pilot-source-plan:v1"
        || string_field(&plan, "record_family")? != "transportation_pilot_source_plan"
        || int_field(&plan, "pulse")? != 90
        || string_field(&plan, "selected_pilot_decision_path")?
            != PILOT_LANE_SELECTION_DECISION_JSON_PATH
        || string_field(&plan, "transportation_depth_card_path")?
            != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(&plan, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&plan, "deterministic_annual_update_simulator_contract_path")?
            != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&plan, "technology_transition_operating_model_path")?
            != TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH
        || string_field(&plan, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err("transportation source plan identity or governing paths failed".to_string());
    }
    let custody_status = string_field(&plan, "source_custody_status")?;
    if !custody_status.contains("no_new_external_request")
        || !custody_status.contains("no_source_bytes_captured")
    {
        return Err("transportation source plan custody status failed".to_string());
    }

    let selected = plan
        .get("selected_pilot")
        .ok_or("transportation source plan selected pilot")?;
    if string_field(selected, "candidate_id")? != "transportation_asset_maintenance_and_safety"
        || string_field(selected, "lane_id")? != "transportation-infrastructure"
        || string_field(selected, "source_plan_status")? != "planned_not_captured"
    {
        return Err("transportation source plan selected pilot failed".to_string());
    }

    let boundary = string_field(&plan, "non_claim_boundary")?;
    for required in [
        "transportation pilot source plan",
        "not captured source evidence",
        "source custody closure",
        "baseline path",
        "floor threshold",
        "modernization path",
        "stress path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "solver result",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation source plan boundary missing {required}"
            ));
        }
    }

    let custody = plan
        .get("custody_requirements")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation source plan custody requirements")?;
    for flag in [
        "retrieval_date_required",
        "source_url_required",
        "publisher_required",
        "vintage_required",
        "raw_bytes_required",
        "byte_count_required",
        "sha256_required",
        "local_raw_path_required",
        "metadata_record_required",
        "matched_period_unit_perimeter_required",
        "missingness_disclosure_required",
        "no_interpolation_without_model",
    ] {
        if custody.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation source plan custody flag {flag} must be true"
            ));
        }
    }
    if custody
        .get("custody_complete")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("transportation source plan custody must remain incomplete".to_string());
    }

    let families = plan
        .get("source_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation source families")?;
    let observed_families = families
        .iter()
        .map(|row| string_field(row, "family_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_families = BTreeSet::from([
        "omb_federal_outlay_baseline".to_string(),
        "treasury_trust_fund_receipts_and_balances".to_string(),
        "dot_budget_and_performance".to_string(),
        "fhwa_conditions_performance_and_bridge_pavement".to_string(),
        "nhtsa_and_bts_safety_reliability_access".to_string(),
        "census_state_local_finance".to_string(),
        "gao_oig_project_delivery_controls".to_string(),
        "international_transport_forum_oecd_context".to_string(),
    ]);
    if observed_families != expected_families {
        return Err("transportation source family set failed".to_string());
    }
    for family in families {
        if string_field(family, "publisher_family")?.is_empty()
            || string_field(family, "planned_use")?.is_empty()
            || string_field(family, "matched_period_requirement")?.is_empty()
            || string_field(family, "unit_requirement")?.is_empty()
            || string_field(family, "custody_status")? != "planned_not_captured"
            || family
                .get("value_fields_initially_null")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("transportation source family fields failed".to_string());
        }
        if family
            .get("required_scope")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|scope| scope.is_empty())
        {
            return Err("transportation source family required scope missing".to_string());
        }
    }

    let matching = plan
        .get("matching_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation source matching rules")?;
    for flag in [
        "federal_state_local_translation_required",
        "trust_funds_remain_separate",
        "explicit_general_fund_transfers_required",
        "credited_offsetting_collections_required",
        "state_local_private_user_financed_activity_kept_contextual_until_translated",
        "international_differences_not_savings",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if matching.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation source matching flag {flag} must be true"
            ));
        }
    }

    let floors = plan
        .get("floor_indicator_families_planned")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation source planned floors")?;
    let observed_floors = floors
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "access",
        "quality_safety",
        "equity",
        "adequacy_resilience",
        "delivery_feasibility",
        "asset_condition_lane_specific",
    ] {
        if !observed_floors.contains(required) {
            return Err(format!("transportation source floor missing {required}"));
        }
    }
    for floor in floors {
        if string_field(floor, "status")? != "planned_not_thresholded"
            || !floor.get("value").is_some_and(serde_json::Value::is_null)
            || floor.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("transportation source floors must remain null/false".to_string());
        }
    }

    let downstream = plan
        .get("planned_downstream_contracts")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation source downstream contracts")?;
    for contract in downstream {
        if !contract.get("path").is_some_and(serde_json::Value::is_null)
            || string_field(contract, "status")? != "not_created"
        {
            return Err("transportation source downstream paths must remain null".to_string());
        }
    }

    let blockers = plan
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation source blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "source_bytes_not_captured",
        "source_metadata_missing",
        "sha256_missing",
        "matched_period_unit_perimeter_missing",
        "federal_state_local_translation_missing",
        "trust_fund_reconciliation_missing",
        "baseline_path_missing",
        "floor_thresholds_not_set",
        "modernization_path_missing",
        "stress_path_missing",
        "simulator_not_run",
    ] {
        if !blockers.contains(required) {
            return Err(format!("transportation source blocker missing {required}"));
        }
    }

    let outputs = plan
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation source outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation source output {field} must remain null"
            ));
        }
    }

    let claims = plan
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation source claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation source claim boolean must be bool")?;
        if field == "source_plan_published" {
            if !observed {
                return Err("transportation source plan published flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation source public claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(TRANSPORTATION_PILOT_SOURCE_PLAN_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH,
        "does not capture source bytes or close source custody",
        "transportation asset maintenance and safety",
        "transportation-infrastructure",
        "not captured source evidence",
        "baseline path",
        "floor threshold",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "balanced-budget claim",
        "OMB federal transportation outlay baseline",
        "Treasury trust-fund receipts",
        "DOT budget, performance",
        "FHWA condition",
        "NHTSA and BTS safety",
        "Census state and local finance",
        "GAO and DOT Inspector General",
        "International Transport Forum and OECD",
        "retrieval date, source URL, publisher",
        "raw bytes, byte count, SHA-256",
        "Trust funds remain separate",
        "International transportation differences are not savings",
        "values remain null and their pass flags remain false",
        "Only the source plan is published",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation source reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_baseline_path_contract(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH,
        TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_SCHEMA_PATH,
        TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation baseline contract artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "transportation-pilot-baseline-path-contract:v1"
        || string_field(&contract, "record_family")?
            != "transportation_pilot_baseline_path_contract"
        || int_field(&contract, "pulse")? != 91
        || string_field(&contract, "selected_pilot_decision_path")?
            != PILOT_LANE_SELECTION_DECISION_JSON_PATH
        || string_field(&contract, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&contract, "transportation_depth_card_path")?
            != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(
            &contract,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err(
            "transportation baseline contract identity or governing paths failed".to_string(),
        );
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_source_bytes_captured") {
        return Err("transportation baseline source custody status failed".to_string());
    }

    let selected = contract
        .get("selected_pilot")
        .ok_or("transportation baseline selected pilot")?;
    if string_field(selected, "candidate_id")? != "transportation_asset_maintenance_and_safety"
        || string_field(selected, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation baseline selected pilot failed".to_string());
    }

    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "transportation pilot baseline path contract",
        "not a completed baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "solver result",
        "modernization path",
        "stress path",
        "floor threshold decision",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation baseline boundary missing {required}"
            ));
        }
    }

    let horizon = contract
        .get("baseline_horizon")
        .ok_or("transportation baseline horizon")?;
    if int_field(horizon, "start_fiscal_year")? != 2025
        || int_field(horizon, "end_fiscal_year")? != 2035
        || int_field(horizon, "annual_rows_required")? != 11
        || horizon
            .get("includes_baseline_year")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || horizon
            .get("current_law_zero_reform_delta_required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || horizon
            .get("baseline_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || horizon
            .get("simulator_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation baseline horizon must remain incomplete".to_string());
    }

    let anchor = contract
        .get("fy2025_anchor")
        .ok_or("transportation baseline fy2025 anchor")?;
    if string_field(anchor, "source_record_path")? != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(anchor, "source_id")? != "SRC-OMB-HIST-3-2-FY2027"
        || int_field(anchor, "fiscal_year")? != 2025
        || string_field(anchor, "function_code")? != "400"
        || int_field(anchor, "total_outlays_millions")? != 145320
        || int_field(anchor, "reform_delta_millions")? != 0
        || !string_field(anchor, "anchor_status")?.contains("not_multiyear_baseline")
    {
        return Err("transportation baseline FY2025 anchor identity failed".to_string());
    }
    let components = anchor
        .get("components")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation baseline anchor components")?;
    let component_sum = components
        .iter()
        .map(|row| int_field(row, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<i64>();
    if component_sum != int_field(anchor, "component_sum_millions")?
        || component_sum != int_field(anchor, "total_outlays_millions")?
    {
        return Err("transportation baseline FY2025 component sum failed".to_string());
    }

    let depth_text = fs::read_to_string(root.join(TRANSPORTATION_DEPTH_CARD_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let depth: serde_json::Value = serde_json::from_str(&depth_text).map_err(|e| e.to_string())?;
    if int_field(&depth, "total_outlays_millions")? != int_field(anchor, "total_outlays_millions")?
    {
        return Err("transportation baseline anchor must match depth card".to_string());
    }

    let fields = contract
        .get("required_annual_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation baseline required fields")?;
    let observed_fields = fields
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "fiscal_year",
        "gross_program_outlays_millions",
        "implementation_admin_outlays_millions",
        "credited_offsetting_collections_millions",
        "dedicated_receipts_millions",
        "explicit_general_fund_transfer_millions",
        "other_scored_fund_income_millions",
        "reserve_contribution_millions",
        "net_cash_requirement_millions",
        "fund_balance_change_millions",
        "federal_state_local_translation_status",
        "score_source_id",
        "source_vintage",
        "raw_source_path",
        "raw_byte_count",
        "raw_sha256",
        "unrounded_value_status",
        "current_law_reform_delta_millions",
    ] {
        if !observed_fields.contains(required) {
            return Err(format!("transportation baseline field missing {required}"));
        }
    }
    for field in fields {
        if field.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || !field
                .get("initial_value")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("transportation baseline required fields must be null".to_string());
        }
    }

    let identities = contract
        .get("accounting_identities")
        .ok_or("transportation baseline accounting identities")?;
    for required in [
        "primary_outlays",
        "net_cash_requirement",
        "fund_balance_change",
        "reform_delta_rule",
        "rounding_rule",
    ] {
        if string_field(identities, required)?.is_empty() {
            return Err(format!(
                "transportation baseline identity missing {required}"
            ));
        }
    }
    if !string_field(identities, "reform_delta_rule")?.contains("zero reform delta")
        || !string_field(identities, "rounding_rule")?.contains("explicit rounding line")
    {
        return Err("transportation baseline reform delta or rounding rule failed".to_string());
    }

    let funds = contract
        .get("fund_treatment")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation baseline fund treatment")?;
    for flag in [
        "transportation_trust_fund_required",
        "general_fund_required",
        "explicit_interfund_transfers_required",
        "credited_offsetting_collections_required",
        "state_local_private_user_financed_context_separate",
        "trust_funds_remain_separate",
    ] {
        if funds.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("transportation baseline fund flag {flag} failed"));
        }
    }

    if contract
        .get("baseline_rows")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation baseline rows must remain empty".to_string());
    }

    let gates = contract
        .get("blocked_gates")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation baseline blocked gates")?;
    if gates
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("transportation baseline blocked gates must be false".to_string());
    }

    let blockers = contract
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation baseline blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "annual_rows_missing",
        "source_bytes_not_captured",
        "source_metadata_missing",
        "sha256_missing",
        "trust_fund_reconciliation_missing",
        "explicit_general_fund_transfer_series_missing",
        "credited_offsetting_collection_series_missing",
        "federal_state_local_translation_missing",
        "unrounded_values_missing",
        "floor_indicator_contract_missing",
        "modernization_path_missing",
        "stress_path_missing",
        "simulator_not_run",
    ] {
        if !blockers.contains(required) {
            return Err(format!(
                "transportation baseline blocker missing {required}"
            ));
        }
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation baseline outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation baseline output {field} must remain null"
            ));
        }
    }

    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation baseline claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation baseline claim boolean must be bool")?;
        if field == "baseline_contract_published" {
            if !observed {
                return Err("transportation baseline contract flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation baseline public claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH,
        "not a completed baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "floor threshold decision",
        "balanced-budget claim",
        "transportation asset maintenance and safety",
        "FY2025 through FY2035",
        "zero reform delta",
        "total transportation outlays: $145.320B",
        "The component sum equals $145.320B",
        "not a multi-year baseline",
        "gross program outlays",
        "credited offsetting collections",
        "explicit general-fund transfer",
        "byte count, SHA-256",
        "Transportation trust funds remain separate",
        "Baseline rows remain empty",
        "Only the baseline contract is published",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation baseline reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_floor_indicator_contract(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH,
        TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_SCHEMA_PATH,
        TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation floor indicator contract artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "transportation-pilot-floor-indicator-contract:v1"
        || string_field(&contract, "record_family")?
            != "transportation_pilot_floor_indicator_contract"
        || int_field(&contract, "pulse")? != 92
        || string_field(&contract, "selected_pilot_decision_path")?
            != PILOT_LANE_SELECTION_DECISION_JSON_PATH
        || string_field(&contract, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&contract, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&contract, "transportation_depth_card_path")?
            != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(&contract, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&contract, "international_comparator_target_rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(
            &contract,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err("transportation floor contract identity or governing paths failed".to_string());
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_source_bytes_captured") {
        return Err("transportation floor source custody status failed".to_string());
    }

    let selected = contract
        .get("selected_pilot")
        .ok_or("transportation floor selected pilot")?;
    if string_field(selected, "candidate_id")? != "transportation_asset_maintenance_and_safety"
        || string_field(selected, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation floor selected pilot failed".to_string());
    }

    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "transportation pilot floor indicator contract",
        "not a floor threshold decision",
        "floor pass finding",
        "completed baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "solver result",
        "modernization path",
        "stress path",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!("transportation floor boundary missing {required}"));
        }
    }

    let policy = contract
        .get("floor_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation floor policy")?;
    for flag in [
        "all_lower_cost_scenarios_must_pass_floors",
        "floor_failure_blocks_target_cost",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "international_differences_not_savings",
        "no_fraud_inference",
        "federal_state_local_translation_required",
    ] {
        if policy.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("transportation floor policy flag {flag} failed"));
        }
    }
    for flag in ["thresholds_set", "floor_passes_recorded"] {
        if policy.get(flag).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation floor policy flag {flag} must remain false"
            ));
        }
    }

    let families = contract
        .get("required_floor_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation floor families")?;
    let observed_families = families
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_families = BTreeSet::from([
        "access_coverage".to_string(),
        "quality_safety".to_string(),
        "equity_distribution".to_string(),
        "adequacy_resilience".to_string(),
        "delivery_feasibility".to_string(),
        "asset_condition_lane_specific".to_string(),
    ]);
    if observed_families != expected_families {
        return Err("transportation floor family set failed".to_string());
    }
    for family in families {
        if string_field(family, "public_label")?.is_empty()
            || string_field(family, "purpose")?.is_empty()
            || string_field(family, "status")? != "planned_not_thresholded"
            || !family
                .get("threshold_value")
                .is_some_and(serde_json::Value::is_null)
            || !family
                .get("observed_value")
                .is_some_and(serde_json::Value::is_null)
            || family.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("transportation floor family values must remain blocked".to_string());
        }
        if family
            .get("planned_indicators")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.is_empty())
            || family
                .get("required_source_families")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
        {
            return Err(
                "transportation floor family indicators or source families missing".to_string(),
            );
        }
    }

    let source_plan_text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let source_plan: serde_json::Value =
        serde_json::from_str(&source_plan_text).map_err(|e| e.to_string())?;
    let source_plan_floors = source_plan
        .get("floor_indicator_families_planned")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation source plan floor families")?
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "access",
        "quality_safety",
        "equity",
        "adequacy_resilience",
        "delivery_feasibility",
        "asset_condition_lane_specific",
    ] {
        if !source_plan_floors.contains(required) {
            return Err(format!(
                "transportation source plan floor family missing {required}"
            ));
        }
    }

    let requirements = contract
        .get("indicator_record_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation floor indicator requirements")?;
    let observed_requirements = requirements
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "floor_id",
        "indicator_id",
        "source_family_id",
        "source_id",
        "retrieval_date",
        "raw_source_path",
        "raw_byte_count",
        "raw_sha256",
        "period",
        "unit",
        "perimeter",
        "observed_value",
        "threshold_value",
        "comparison_direction",
        "passed",
        "missingness_reason",
        "federal_state_local_translation_status",
    ] {
        if !observed_requirements.contains(required) {
            return Err(format!(
                "transportation floor indicator field missing {required}"
            ));
        }
    }
    for requirement in requirements {
        if requirement
            .get("required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
            || !requirement
                .get("initial_value")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err(
                "transportation floor indicator requirements must be required and null".to_string(),
            );
        }
    }

    let gates = contract
        .get("blocked_gates")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation floor blocked gates")?;
    if gates
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("transportation floor blocked gates must be false".to_string());
    }

    if contract
        .get("indicator_records")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation floor indicator records must remain empty".to_string());
    }

    let blockers = contract
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation floor blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "source_bytes_not_captured",
        "source_metadata_missing",
        "sha256_missing",
        "indicator_records_missing",
        "thresholds_not_set",
        "floor_passes_not_recorded",
        "federal_state_local_translation_missing",
        "baseline_path_incomplete",
        "modernization_path_missing",
        "stress_path_missing",
        "simulator_not_run",
    ] {
        if !blockers.contains(required) {
            return Err(format!("transportation floor blocker missing {required}"));
        }
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation floor outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation floor output {field} must remain null"
            ));
        }
    }

    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation floor claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation floor claim boolean must be bool")?;
        if field == "floor_indicator_contract_published" {
            if !observed {
                return Err("transportation floor contract flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation floor public claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH,
        "not a floor threshold decision",
        "floor pass finding",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
        "transportation asset maintenance and safety",
        "Every lower-cost scenario must pass",
        "access and coverage",
        "quality and safety",
        "equity and distribution",
        "adequacy and resilience",
        "delivery feasibility",
        "transportation asset condition",
        "No thresholds are set here",
        "No floor pass finding is made here",
        "Missing values remain null and blocked gates remain false",
        "raw byte count, raw SHA-256",
        "federal/state/local translation status",
        "International transportation differences are not savings",
        "No fraud inference is allowed",
        "Only the floor indicator contract is published",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation floor reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_modernization_path_contract(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_JSON_PATH,
        TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_SCHEMA_PATH,
        TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation modernization contract artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")?
        != "transportation-pilot-modernization-path-contract:v1"
        || string_field(&contract, "record_family")?
            != "transportation_pilot_modernization_path_contract"
        || int_field(&contract, "pulse")? != 93
        || string_field(&contract, "selected_pilot_decision_path")?
            != PILOT_LANE_SELECTION_DECISION_JSON_PATH
        || string_field(&contract, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&contract, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&contract, "floor_indicator_contract_path")?
            != TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "technology_transition_operating_model_path")?
            != TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH
        || string_field(&contract, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(
            &contract,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err(
            "transportation modernization contract identity or governing paths failed".to_string(),
        );
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_source_bytes_captured") {
        return Err("transportation modernization source custody status failed".to_string());
    }

    let selected = contract
        .get("selected_pilot")
        .ok_or("transportation modernization selected pilot")?;
    if string_field(selected, "candidate_id")? != "transportation_asset_maintenance_and_safety"
        || string_field(selected, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation modernization selected pilot failed".to_string());
    }

    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "transportation pilot modernization path contract",
        "not a modernization result",
        "technology-savings claim",
        "productivity finding",
        "completed baseline path",
        "floor pass finding",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "solver result",
        "stress path",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation modernization boundary missing {required}"
            ));
        }
    }

    let policy = contract
        .get("modernization_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation modernization policy")?;
    for flag in [
        "technology_is_transition_path_not_automatic_cut",
        "productivity_credit_requires_same_service_or_better",
        "floor_pass_required_before_lower_cost_use",
        "transition_costs_positive_outlays",
        "implementation_admin_costs_positive_outlays",
        "no_headcount_or_department_cut_instruction",
        "no_savings_without_measured_net_effect",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if policy.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation modernization policy flag {flag} failed"
            ));
        }
    }

    let technology_text =
        fs::read_to_string(root.join(TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let technology: serde_json::Value =
        serde_json::from_str(&technology_text).map_err(|e| e.to_string())?;
    let technology_boundary = string_field(&technology, "non_claim_boundary")?;
    if !technology_boundary.contains("not a technology-savings claim")
        || !technology_boundary.contains("department-cut instruction")
    {
        return Err("technology transition model boundary must remain linked".to_string());
    }

    let segments = contract
        .get("required_intervention_segments")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation modernization segments")?;
    let observed_segments = segments
        .iter()
        .map(|row| string_field(row, "segment_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_segments = BTreeSet::from([
        "asset_inventory_and_condition_data".to_string(),
        "project_delivery_and_permitting_controls".to_string(),
        "predictive_maintenance_and_operations".to_string(),
        "safety_targeting_and_network_design".to_string(),
    ]);
    if observed_segments != expected_segments {
        return Err("transportation modernization segment set failed".to_string());
    }
    for segment in segments {
        if string_field(segment, "public_label")?.is_empty()
            || string_field(segment, "purpose")?.is_empty()
            || string_field(segment, "status")? != "planned_not_scored"
            || !segment
                .get("central_effect_millions")
                .is_some_and(serde_json::Value::is_null)
            || !segment
                .get("transition_cost_millions")
                .is_some_and(serde_json::Value::is_null)
            || !segment
                .get("net_effect_millions")
                .is_some_and(serde_json::Value::is_null)
            || segment
                .get("productivity_credit_allowed")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(
                "transportation modernization segment values must remain blocked".to_string(),
            );
        }
        if segment
            .get("required_cost_fields")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.is_empty())
            || segment
                .get("required_effect_fields")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
        {
            return Err(
                "transportation modernization segment cost/effect fields missing".to_string(),
            );
        }
    }

    let requirements = contract
        .get("required_modernization_record_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation modernization record requirements")?;
    let observed_requirements = requirements
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "segment_id",
        "fiscal_year",
        "policy_instrument",
        "implementation_admin_outlays_millions",
        "transition_cost_millions",
        "monitoring_enforcement_cost_millions",
        "gross_effect_millions",
        "net_effect_millions",
        "utilization_or_volume_response",
        "vendor_or_procurement_response",
        "workforce_transition_effect",
        "service_level_effect",
        "floor_pass_link",
        "source_id",
        "raw_source_path",
        "raw_byte_count",
        "raw_sha256",
    ] {
        if !observed_requirements.contains(required) {
            return Err(format!(
                "transportation modernization field missing {required}"
            ));
        }
    }
    for requirement in requirements {
        if requirement
            .get("required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
            || !requirement
                .get("initial_value")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err(
                "transportation modernization requirements must be required and null".to_string(),
            );
        }
    }

    let scenario = contract
        .get("scenario_linkage")
        .ok_or("transportation modernization scenario linkage")?;
    for field in [
        "current_law_modernization_delta_millions",
        "central_modernization_effect_millions",
        "stress_modernization_effect_millions",
    ] {
        if !scenario.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "transportation modernization scenario field {field} must remain null"
            ));
        }
    }
    for flag in [
        "stress_path_contract_required",
        "stress_must_be_adverse_realization_of_same_policy",
        "aggressive_price_reduction_is_not_stress",
    ] {
        if scenario.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation modernization scenario flag {flag} failed"
            ));
        }
    }

    let gates = contract
        .get("blocked_gates")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation modernization blocked gates")?;
    if gates
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("transportation modernization blocked gates must be false".to_string());
    }

    if contract
        .get("modernization_records")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation modernization records must remain empty".to_string());
    }

    let blockers = contract
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation modernization blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "source_bytes_not_captured",
        "source_metadata_missing",
        "sha256_missing",
        "baseline_path_incomplete",
        "floor_thresholds_not_set",
        "floor_passes_not_recorded",
        "modernization_records_missing",
        "transition_costs_missing",
        "behavior_and_procurement_response_missing",
        "same_service_or_better_not_verified",
        "stress_path_missing",
        "simulator_not_run",
    ] {
        if !blockers.contains(required) {
            return Err(format!(
                "transportation modernization blocker missing {required}"
            ));
        }
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation modernization outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation modernization output {field} must remain null"
            ));
        }
    }

    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation modernization claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation modernization claim boolean must be bool")?;
        if field == "modernization_contract_published" {
            if !observed {
                return Err("transportation modernization contract flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation modernization public claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_JSON_PATH,
        "not a modernization result",
        "technology-savings claim",
        "productivity finding",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "stress path",
        "balanced-budget claim",
        "Technology is a transition path, not an automatic cut",
        "Productivity credit requires same service or better",
        "asset inventory and condition data",
        "project delivery and permitting controls",
        "predictive maintenance and operations",
        "safety targeting and network design",
        "Central effect, transition cost, and net effect remain null",
        "Productivity credit remains false",
        "raw byte count, and raw SHA-256",
        "Stress must later be an adverse realization of the same policy",
        "not an aggressive price reduction",
        "Only the modernization contract is published",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "transportation modernization reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_stress_path_contract(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_JSON_PATH,
        TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_SCHEMA_PATH,
        TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation stress contract artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "transportation-pilot-stress-path-contract:v1"
        || string_field(&contract, "record_family")? != "transportation_pilot_stress_path_contract"
        || int_field(&contract, "pulse")? != 94
        || string_field(&contract, "selected_pilot_decision_path")?
            != PILOT_LANE_SELECTION_DECISION_JSON_PATH
        || string_field(&contract, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&contract, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&contract, "floor_indicator_contract_path")?
            != TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "modernization_path_contract_path")?
            != TRANSPORTATION_PILOT_MODERNIZATION_PATH_CONTRACT_JSON_PATH
        || string_field(&contract, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(
            &contract,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err(
            "transportation stress contract identity or governing paths failed".to_string(),
        );
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_source_bytes_captured") {
        return Err("transportation stress source custody status failed".to_string());
    }

    let selected = contract
        .get("selected_pilot")
        .ok_or("transportation stress selected pilot")?;
    if string_field(selected, "candidate_id")? != "transportation_asset_maintenance_and_safety"
        || string_field(selected, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation stress selected pilot failed".to_string());
    }

    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "transportation pilot stress path contract",
        "not a stress result",
        "aggressive cut scenario",
        "modernization result",
        "productivity finding",
        "completed baseline path",
        "floor pass finding",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "solver result",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!("transportation stress boundary missing {required}"));
        }
    }

    let policy = contract
        .get("stress_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation stress policy")?;
    for flag in [
        "stress_is_adverse_realization_of_same_policy",
        "aggressive_cut_is_not_stress",
        "same_policy_instrument_required",
        "higher_implementation_cost_allowed_as_stress_dimension",
        "weaker_productivity_effect_allowed_as_stress_dimension",
        "access_remediation_required_when_floors_at_risk",
        "weaker_receipts_or_higher_interest_context_allowed",
        "floor_failure_blocks_target_cost",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if policy.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("transportation stress policy flag {flag} failed"));
        }
    }

    let dimensions = contract
        .get("required_stress_dimensions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation stress dimensions")?;
    let observed_dimensions = dimensions
        .iter()
        .map(|row| string_field(row, "dimension_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_dimensions = BTreeSet::from([
        "weaker_modernization_effect".to_string(),
        "higher_implementation_cost".to_string(),
        "higher_utilization_or_volume".to_string(),
        "access_quality_or_equity_remediation".to_string(),
        "weaker_receipt_or_fund_balance_context".to_string(),
        "higher_interest_context".to_string(),
    ]);
    if observed_dimensions != expected_dimensions {
        return Err("transportation stress dimension set failed".to_string());
    }
    for dimension in dimensions {
        if string_field(dimension, "public_label")?.is_empty()
            || string_field(dimension, "purpose")?.is_empty()
            || string_field(dimension, "status")? != "planned_not_scored"
            || !dimension
                .get("central_value")
                .is_some_and(serde_json::Value::is_null)
            || !dimension
                .get("stress_value")
                .is_some_and(serde_json::Value::is_null)
            || !dimension
                .get("delta_value")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("transportation stress dimensions must remain unscored".to_string());
        }
    }

    let requirements = contract
        .get("required_stress_record_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation stress record requirements")?;
    let observed_requirements = requirements
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "fiscal_year",
        "policy_instrument",
        "same_policy_as_central",
        "stress_dimension_id",
        "central_value",
        "stress_value",
        "delta_value",
        "implementation_admin_outlays_millions",
        "access_remediation_outlays_millions",
        "floor_impact_link",
        "fund_balance_context",
        "source_id",
        "raw_source_path",
        "raw_byte_count",
        "raw_sha256",
    ] {
        if !observed_requirements.contains(required) {
            return Err(format!("transportation stress field missing {required}"));
        }
    }
    for requirement in requirements {
        if requirement
            .get("required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
            || !requirement
                .get("initial_value")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("transportation stress requirements must be required and null".to_string());
        }
    }

    let scenario = contract
        .get("scenario_values")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation stress scenario values")?;
    for (field, value) in scenario {
        if !value.is_null() {
            return Err(format!(
                "transportation stress scenario value {field} must remain null"
            ));
        }
    }

    let gates = contract
        .get("blocked_gates")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation stress blocked gates")?;
    if gates
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("transportation stress blocked gates must be false".to_string());
    }

    if contract
        .get("stress_records")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation stress records must remain empty".to_string());
    }

    let blockers = contract
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation stress blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "source_bytes_not_captured",
        "source_metadata_missing",
        "sha256_missing",
        "baseline_path_incomplete",
        "floor_thresholds_not_set",
        "floor_passes_not_recorded",
        "modernization_path_incomplete",
        "central_policy_not_scored",
        "stress_records_missing",
        "same_policy_adverse_realization_missing",
        "simulator_not_run",
    ] {
        if !blockers.contains(required) {
            return Err(format!("transportation stress blocker missing {required}"));
        }
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation stress outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation stress output {field} must remain null"
            ));
        }
    }

    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation stress claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation stress claim boolean must be bool")?;
        if field == "stress_contract_published" {
            if !observed {
                return Err("transportation stress contract flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation stress public claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_STRESS_PATH_CONTRACT_JSON_PATH,
        "bad-case version of the same transportation pilot policy",
        "does not mean choosing a harsher cut",
        "not a stress result",
        "aggressive cut scenario",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "balanced-budget claim",
        "Stress is an adverse realization of the same policy",
        "An aggressive cut is not stress",
        "weaker modernization effect",
        "higher implementation cost",
        "higher utilization or volume",
        "access, quality, or equity remediation",
        "weaker receipt or fund-balance context",
        "higher interest context",
        "All central values, stress values, deltas, target costs",
        "remain null",
        "raw byte count, and raw SHA-256",
        "Stress records remain empty",
        "Only the stress contract is published",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation stress reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_fy2025_anchor_custody(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH,
        TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_SCHEMA_PATH,
        TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation FY2025 anchor custody artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let custody_record: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&custody_record, "record_id")?
        != "transportation-pilot-fy2025-anchor-custody:v1"
        || string_field(&custody_record, "record_family")?
            != "transportation_pilot_fy2025_anchor_custody"
        || int_field(&custody_record, "pulse")? != 95
        || string_field(&custody_record, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&custody_record, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&custody_record, "transportation_depth_card_path")?
            != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(&custody_record, "source_version_ledger_path")?
            != SOURCE_VERSION_LEDGER_PATH
    {
        return Err("transportation FY2025 anchor custody identity or paths failed".to_string());
    }

    let source = custody_record
        .get("source_custody")
        .ok_or("transportation FY2025 source custody")?;
    if string_field(source, "source_id")? != "SRC-OMB-HIST-3-2-FY2027"
        || string_field(source, "publisher")? != "Office of Management and Budget"
        || string_field(source, "retrieval_date")? != "2026-06-21"
        || string_field(source, "local_raw_path")?
            != "data/raw/omb/SRC-OMB-HIST-3-2-FY2027/2026-06-21/hist03z2_fy2027.xlsx"
        || int_field(source, "raw_byte_count")? != 60343
        || string_field(source, "raw_sha256")?
            != "78100f3efb1a6b08d675b24af173a57359e47dce103a2f1499d905a4bbba06ce"
        || source
            .get("raw_bytes_already_present_in_repo")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("new_external_request_submitted")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || source
            .get("custody_complete_for_fy2025_anchor")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("custody_complete_for_full_baseline_path")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation FY2025 source custody fields failed".to_string());
    }

    let raw_path = root.join(string_field(source, "local_raw_path")?);
    let bytes = fs::read(&raw_path).map_err(|e| e.to_string())?;
    if bytes.len() as i64 != int_field(source, "raw_byte_count")? {
        return Err("transportation FY2025 raw byte count does not match local file".to_string());
    }
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let observed_hash = format!("{:x}", hasher.finalize());
    if observed_hash != string_field(source, "raw_sha256")? {
        return Err("transportation FY2025 raw SHA-256 does not match local file".to_string());
    }

    let scope = custody_record
        .get("scope")
        .ok_or("transportation FY2025 custody scope")?;
    if string_field(scope, "lane_id")? != "transportation-infrastructure"
        || string_field(scope, "function_code")? != "400"
        || int_field(scope, "fiscal_year")? != 2025
        || string_field(scope, "unit")? != "millions_usd"
        || string_field(scope, "perimeter")? != "federal_budget_function_400_only"
        || !scope
            .get("federal_state_local_translation_status")
            .is_some_and(serde_json::Value::is_null)
        || !scope
            .get("trust_fund_reconciliation_status")
            .is_some_and(serde_json::Value::is_null)
        || scope
            .get("baseline_path_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation FY2025 custody scope failed".to_string());
    }

    let reconciliation = custody_record
        .get("fy2025_anchor_reconciliation")
        .ok_or("transportation FY2025 reconciliation")?;
    let rows = reconciliation
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation FY2025 rows")?;
    let component_sum = rows
        .iter()
        .map(|row| int_field(row, "amount_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<i64>();
    if component_sum != 145320
        || int_field(reconciliation, "component_sum_millions")? != 145320
        || int_field(reconciliation, "total_outlays_millions")? != 145320
        || int_field(reconciliation, "difference_millions")? != 0
        || int_field(reconciliation, "current_law_reform_delta_millions")? != 0
        || reconciliation
            .get("matches_transportation_depth_card")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || int_field(
            reconciliation
                .get("total_row")
                .ok_or("transportation FY2025 total row")?,
            "amount_millions",
        )? != 145320
    {
        return Err("transportation FY2025 reconciliation values failed".to_string());
    }
    let observed_subfunctions = rows
        .iter()
        .map(|row| string_field(row, "subfunction_code"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_subfunctions
        != BTreeSet::from([
            "401".to_string(),
            "402".to_string(),
            "403".to_string(),
            "407".to_string(),
        ])
    {
        return Err("transportation FY2025 subfunction set failed".to_string());
    }

    let depth_text = fs::read_to_string(root.join(TRANSPORTATION_DEPTH_CARD_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let depth: serde_json::Value = serde_json::from_str(&depth_text).map_err(|e| e.to_string())?;
    if int_field(&depth, "total_outlays_millions")?
        != int_field(reconciliation, "total_outlays_millions")?
    {
        return Err("transportation FY2025 custody must match depth card".to_string());
    }

    let extracted_text =
        fs::read_to_string(root.join(string_field(&custody_record, "extracted_source_path")?))
            .map_err(|e| e.to_string())?;
    for required in [
        "outlay-function:2025:400:401:outlays",
        "outlay-function:2025:400:402:outlays",
        "outlay-function:2025:400:403:outlays",
        "outlay-function:2025:400:407:outlays",
        "outlay-function:2025:400:total:outlays",
    ] {
        if !extracted_text.contains(required) {
            return Err(format!(
                "transportation FY2025 extracted source missing {required}"
            ));
        }
    }

    let missing = custody_record
        .get("still_missing_for_baseline_path")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation FY2025 missing list")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "fy2026_fy2035_annual_rows",
        "gross_program_outlays_series",
        "implementation_admin_outlays_series",
        "credited_offsetting_collections_series",
        "dedicated_receipts_series",
        "explicit_general_fund_transfer_series",
        "trust_fund_reconciliation",
        "federal_state_local_translation",
    ] {
        if !missing.contains(required) {
            return Err(format!(
                "transportation FY2025 missing baseline blocker absent {required}"
            ));
        }
    }

    let outputs = custody_record
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation FY2025 outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation FY2025 output {field} must remain null"
            ));
        }
    }

    let claims = custody_record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation FY2025 claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation FY2025 claim boolean must be bool")?;
        if field == "fy2025_anchor_custody_published" {
            if !observed {
                return Err("transportation FY2025 custody publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation FY2025 public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&custody_record, "non_claim_boundary")?;
    for required in [
        "FY2025 OMB transportation anchor only",
        "not a completed baseline path",
        "trust-fund reconciliation",
        "federal-state-local translation",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation FY2025 custody boundary missing {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH,
        "FY2025 OMB transportation anchor only",
        "no new external request was submitted",
        "byte count: 60,343",
        "78100f3efb1a6b08d675b24af173a57359e47dce103a2f1499d905a4bbba06ce",
        "component sum: $145.320B",
        "parent transportation total: $145.320B",
        "difference: $0",
        "not a completed baseline path",
        "trust-fund reconciliation",
        "federal-state-local translation",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
        "FY2026-FY2035 annual rows",
        "Only the FY2025 anchor custody record is published",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "transportation FY2025 custody reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_partial_federal_outlay_path(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH,
        TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_SCHEMA_PATH,
        TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation partial federal outlay path artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let path_record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&path_record, "record_id")?
        != "transportation-pilot-partial-federal-outlay-path:v1"
        || string_field(&path_record, "record_family")?
            != "transportation_pilot_partial_federal_outlay_path"
        || int_field(&path_record, "pulse")? != 96
        || string_field(&path_record, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&path_record, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&path_record, "fy2025_anchor_custody_path")?
            != TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH
    {
        return Err("transportation partial federal path identity failed".to_string());
    }

    let source = path_record
        .get("source_custody")
        .ok_or("transportation partial source custody")?;
    if string_field(source, "source_id")? != "SRC-OMB-PBD-OUTLAYS-FY2027"
        || string_field(source, "publisher")? != "Office of Management and Budget"
        || string_field(source, "retrieval_date")? != "2026-07-13"
        || int_field(source, "raw_byte_count")? != 2144756
        || string_field(source, "raw_sha256")?
            != "d892f2247e6c1aed68414d3e4168f8b4ab97bcfc7acf82a6a449a3fcb1addb07"
        || source
            .get("raw_bytes_already_present_in_repo")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("new_external_request_submitted")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || source
            .get("custody_complete_for_partial_federal_net_outlay_path")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("custody_complete_for_full_baseline_path")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation partial source custody fields failed".to_string());
    }

    let raw_path = root.join(string_field(source, "local_raw_path")?);
    let bytes = fs::read(&raw_path).map_err(|e| e.to_string())?;
    if bytes.len() as i64 != int_field(source, "raw_byte_count")? {
        return Err("transportation partial raw byte count mismatch".to_string());
    }
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    if format!("{:x}", hasher.finalize()) != string_field(source, "raw_sha256")? {
        return Err("transportation partial raw SHA mismatch".to_string());
    }

    let scope = path_record
        .get("scope")
        .ok_or("transportation partial scope")?;
    if string_field(scope, "lane_id")? != "transportation-infrastructure"
        || string_field(scope, "period")? != "FY2025-FY2031"
        || string_field(scope, "unit")? != "millions_usd"
        || string_field(scope, "raw_workbook_unit")? != "thousands_usd"
        || !string_field(scope, "conversion_formula")?.contains("/ 1000")
        || !scope
            .get("federal_state_local_translation_status")
            .is_some_and(serde_json::Value::is_null)
        || !scope
            .get("trust_fund_reconciliation_status")
            .is_some_and(serde_json::Value::is_null)
        || !scope
            .get("gross_to_net_reconciliation_status")
            .is_some_and(serde_json::Value::is_null)
        || scope
            .get("baseline_path_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || scope
            .get("simulator_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation partial scope failed".to_string());
    }

    let rows = path_record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation partial annual rows")?;
    if rows.len() != 7 {
        return Err("transportation partial path must contain seven annual rows".to_string());
    }
    let observed_years = rows
        .iter()
        .map(|row| int_field(row, "fiscal_year"))
        .collect::<Result<Vec<_>, _>>()?;
    if observed_years != vec![2025, 2026, 2027, 2028, 2029, 2030, 2031] {
        return Err("transportation partial annual years failed".to_string());
    }
    let expected_totals = BTreeMap::from([
        (2025, 145320),
        (2026, 150277),
        (2027, 166475),
        (2028, 175618),
        (2029, 168833),
        (2030, 157618),
        (2031, 152268),
    ]);
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        let component_sum = int_field(row, "ground_transportation_millions")?
            + int_field(row, "air_transportation_millions")?
            + int_field(row, "water_transportation_millions")?
            + int_field(row, "other_transportation_millions")?;
        if component_sum != int_field(row, "component_sum_millions")?
            || component_sum != int_field(row, "total_transportation_millions")?
            || int_field(row, "difference_millions")? != 0
            || int_field(row, "current_law_reform_delta_millions")? != 0
            || expected_totals.get(&year).copied()
                != Some(int_field(row, "total_transportation_millions")?)
        {
            return Err(format!("transportation partial row {year} failed"));
        }
        if year == 2025
            && row
                .get("matches_fy2025_anchor_custody")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("transportation partial FY2025 must match anchor custody".to_string());
        }
    }

    let anchor_text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_FY2025_ANCHOR_CUSTODY_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let anchor: serde_json::Value =
        serde_json::from_str(&anchor_text).map_err(|e| e.to_string())?;
    let anchor_total = int_field(
        anchor
            .get("fy2025_anchor_reconciliation")
            .ok_or("transportation partial anchor reconciliation")?,
        "total_outlays_millions",
    )?;
    if anchor_total != 145320 {
        return Err("transportation partial anchor total failed".to_string());
    }

    let missing = path_record
        .get("missing_year_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation partial missing rows")?;
    if missing.len() != 4 {
        return Err("transportation partial missing rows length failed".to_string());
    }
    for (row, year) in missing.iter().zip([2032, 2033, 2034, 2035]) {
        if int_field(row, "fiscal_year")? != year
            || !row
                .get("total_transportation_millions")
                .is_some_and(serde_json::Value::is_null)
            || !string_field(row, "reason")?.contains("not_available")
        {
            return Err(format!("transportation partial missing row {year} failed"));
        }
    }

    let still_missing = path_record
        .get("still_missing_for_full_baseline_path")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation partial still missing")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "fy2032_fy2035_official_annual_rows",
        "gross_program_outlays_series",
        "credited_offsetting_collections_series",
        "dedicated_receipts_series",
        "explicit_general_fund_transfer_series",
        "trust_fund_reconciliation",
        "federal_state_local_translation",
    ] {
        if !still_missing.contains(required) {
            return Err(format!(
                "transportation partial missing blocker absent {required}"
            ));
        }
    }

    let outputs = path_record
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation partial outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation partial output {field} must remain null"
            ));
        }
    }
    let claims = path_record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation partial claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation partial claim must be bool")?;
        if field == "partial_federal_net_outlay_path_published" {
            if !observed {
                return Err("transportation partial publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation partial public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&path_record, "non_claim_boundary")?;
    for required in [
        "partial FY2025-FY2031 federal net-outlay path",
        "not a completed FY2025-FY2035 baseline path",
        "trust-fund reconciliation",
        "federal-state-local translation",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation partial boundary missing {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH,
        "FY2025-FY2031",
        "No new external request was submitted",
        "FY2032-FY2035 remain missing",
        "not interpolated",
        "FY2025: $145.320B",
        "FY2028: $175.618B",
        "FY2031: $152.268B",
        "zero reform delta",
        "not a completed FY2025-FY2035 baseline path",
        "trust-fund reconciliation",
        "federal-state-local translation",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation partial reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_transportation_pilot_trust_fund_source_custody(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_JSON_PATH,
        TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_SCHEMA_PATH,
        TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation trust-fund source custody artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let custody: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&custody, "record_id")? != "transportation-pilot-trust-fund-source-custody:v1"
        || string_field(&custody, "record_family")?
            != "transportation_pilot_trust_fund_source_custody"
        || int_field(&custody, "pulse")? != 97
        || string_field(&custody, "source_plan_path")? != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&custody, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&custody, "partial_federal_outlay_path")?
            != TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH
        || string_field(&custody, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
    {
        return Err("transportation trust-fund custody identity failed".to_string());
    }

    let source = custody
        .get("source_custody")
        .ok_or("transportation trust-fund source custody")?;
    if string_field(source, "source_id")? != "SRC-OMB-AP-13-FUNDS-FY2027"
        || string_field(source, "publisher")? != "Office of Management and Budget"
        || string_field(source, "retrieval_date")? != "2026-06-21"
        || int_field(source, "raw_byte_count")? != 296958
        || string_field(source, "raw_sha256")?
            != "6a332e8291db7f8e6a4252c79e444c782f5cf2d369cae4b738fe63b7dc0d4437"
        || source
            .get("raw_bytes_already_present_in_repo")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("new_external_request_submitted")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || source
            .get("custody_complete_for_local_fund_source")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("custody_complete_for_trust_fund_reconciliation")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation trust-fund source fields failed".to_string());
    }

    let raw_path = root.join(string_field(source, "local_raw_path")?);
    let bytes = fs::read(&raw_path).map_err(|e| e.to_string())?;
    if bytes.len() as i64 != int_field(source, "raw_byte_count")? {
        return Err("transportation trust-fund raw byte count mismatch".to_string());
    }
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    if format!("{:x}", hasher.finalize()) != string_field(source, "raw_sha256")? {
        return Err("transportation trust-fund raw SHA mismatch".to_string());
    }

    let anchors = custody
        .get("text_scope_anchors")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation trust-fund text anchors")?;
    let anchor_ids = anchors
        .iter()
        .map(|row| string_field(row, "anchor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in ["highway_trust_fund", "airport_and_airway_context"] {
        if !anchor_ids.contains(required) {
            return Err(format!(
                "transportation trust-fund anchor missing {required}"
            ));
        }
    }
    for anchor in anchors {
        if anchor
            .get("observed_in_local_text")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err("transportation trust-fund anchors must be observed".to_string());
        }
    }

    let scope = custody
        .get("scope")
        .ok_or("transportation trust-fund scope")?;
    if string_field(scope, "lane_id")? != "transportation-infrastructure"
        || string_field(scope, "source_family_id")? != "treasury_trust_fund_receipts_and_balances"
        || string_field(scope, "actual_source_family")? != "omb_funds_appendix_local_custody"
        || !scope.get("period").is_some_and(serde_json::Value::is_null)
        || !scope.get("unit").is_some_and(serde_json::Value::is_null)
        || !string_field(scope, "perimeter")?.contains("source custody only")
        || scope
            .get("trust_funds_remain_separate")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || scope
            .get("explicit_general_fund_transfers_required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || scope
            .get("credited_offsetting_collections_required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || !scope
            .get("trust_fund_reconciliation_status")
            .is_some_and(serde_json::Value::is_null)
        || scope
            .get("baseline_path_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || scope
            .get("simulator_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation trust-fund scope failed".to_string());
    }

    if custody
        .get("extracted_values")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation trust-fund extracted values must remain empty".to_string());
    }

    let missing = custody
        .get("still_missing_for_trust_fund_reconciliation")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation trust-fund missing list")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "annual_highway_trust_fund_receipts",
        "annual_highway_trust_fund_outlays",
        "annual_highway_trust_fund_balances",
        "annual_airport_and_airway_trust_fund_receipts",
        "annual_airport_and_airway_trust_fund_outlays",
        "annual_airport_and_airway_trust_fund_balances",
        "explicit_general_fund_transfer_amounts",
        "credited_offsetting_collections",
        "fund_balance_change_identity",
        "mapping_to_partial_federal_outlay_path",
    ] {
        if !missing.contains(required) {
            return Err(format!(
                "transportation trust-fund missing blocker absent {required}"
            ));
        }
    }

    let outputs = custody
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation trust-fund outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation trust-fund output {field} must remain null"
            ));
        }
    }

    let claims = custody
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation trust-fund claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation trust-fund claim must be bool")?;
        if field == "local_source_custody_published" {
            if !observed {
                return Err("transportation trust-fund publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "transportation trust-fund public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&custody, "non_claim_boundary")?;
    for required in [
        "local OMB funds-appendix source custody",
        "not a trust-fund reconciliation",
        "fund-balance path",
        "dedicated-receipt path",
        "explicit transfer path",
        "completed baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "transportation trust-fund boundary missing {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_JSON_PATH,
        "No new external request was submitted",
        "Highway Trust Fund",
        "airport and airway",
        "not enough to publish annual trust-fund receipts",
        "trust-fund reconciliation",
        "fund-balance path",
        "dedicated-receipt path",
        "explicit transfer path",
        "completed baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
        "Trust funds remain separate",
        "Missing annual values remain null, not zero",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "transportation trust-fund reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_pilot_trust_fund_accounting_boundary(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_JSON_PATH,
        TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_SCHEMA_PATH,
        TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation trust-fund accounting boundary artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let boundary_record: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&boundary_record, "record_id")?
        != "transportation-pilot-trust-fund-accounting-boundary:v1"
        || string_field(&boundary_record, "record_family")?
            != "transportation_pilot_trust_fund_accounting_boundary"
        || int_field(&boundary_record, "pulse")? != 98
        || string_field(&boundary_record, "source_plan_path")?
            != TRANSPORTATION_PILOT_SOURCE_PLAN_JSON_PATH
        || string_field(&boundary_record, "baseline_path_contract_path")?
            != TRANSPORTATION_PILOT_BASELINE_PATH_CONTRACT_JSON_PATH
        || string_field(&boundary_record, "partial_federal_outlay_path")?
            != TRANSPORTATION_PILOT_PARTIAL_FEDERAL_OUTLAY_PATH_JSON_PATH
        || string_field(&boundary_record, "trust_fund_source_custody_path")?
            != TRANSPORTATION_PILOT_TRUST_FUND_SOURCE_CUSTODY_JSON_PATH
        || string_field(&boundary_record, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
    {
        return Err("transportation trust-fund accounting identity failed".to_string());
    }

    let source = boundary_record
        .get("source_custody")
        .ok_or("transportation trust-fund accounting source custody")?;
    if string_field(source, "source_id")? != "SRC-OMB-AP-13-FUNDS-FY2027"
        || string_field(source, "publisher")? != "Office of Management and Budget"
        || string_field(source, "retrieval_date")? != "2026-06-21"
        || int_field(source, "raw_byte_count")? != 296958
        || string_field(source, "raw_sha256")?
            != "6a332e8291db7f8e6a4252c79e444c782f5cf2d369cae4b738fe63b7dc0d4437"
        || source
            .get("new_external_request_submitted")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || source
            .get("custody_complete_for_accounting_boundary")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("custody_complete_for_annual_trust_fund_values")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation trust-fund accounting source fields failed".to_string());
    }

    let raw_path = root.join(string_field(source, "local_raw_path")?);
    let bytes = fs::read(&raw_path).map_err(|e| e.to_string())?;
    if bytes.len() as i64 != int_field(source, "raw_byte_count")? {
        return Err("transportation trust-fund accounting raw byte count mismatch".to_string());
    }
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    if format!("{:x}", hasher.finalize()) != string_field(source, "raw_sha256")? {
        return Err("transportation trust-fund accounting raw SHA mismatch".to_string());
    }

    let accounting = boundary_record
        .get("accounting_boundary")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation trust-fund accounting boundary")?;
    for flag in [
        "trust_funds_remain_separate",
        "highway_trust_fund_in_scope",
        "airport_and_airway_context_in_scope",
        "highway_trust_fund_financed_by_motor_fuel_taxes_and_associated_fees",
        "general_fund_transfers_must_be_explicit",
        "trust_fund_income_must_be_used_for_statutory_purposes",
        "trust_fund_balances_invested_in_treasury_securities",
        "borrowing_from_general_fund_is_financing_not_receipt",
        "repayment_of_borrowing_is_not_outlay",
        "offsetting_collections_and_offsetting_receipts_must_not_be_silent",
        "intrafund_transactions_must_not_overstate_income_or_outgo",
        "annual_table_13_4_available_online_not_captured",
    ] {
        if accounting.get(flag).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation trust-fund accounting flag {flag} must be true"
            ));
        }
    }
    if string_field(
        boundary_record
            .get("accounting_boundary")
            .ok_or("transportation trust-fund accounting boundary object")?,
        "lane_id",
    )? != "transportation-infrastructure"
    {
        return Err("transportation trust-fund accounting lane failed".to_string());
    }

    let identities = boundary_record
        .get("required_identity_for_future_rows")
        .ok_or("transportation trust-fund future identities")?;
    if string_field(identities, "primary_outlays")?
        != "gross_program_outlays + implementation_admin_outlays"
        || string_field(identities, "net_cash_requirement")?
            != "primary_outlays - credited_offsetting_collections"
        || string_field(identities, "fund_balance_change")?
            != "dedicated_receipts + explicit_general_fund_transfer + other_scored_fund_income - net_cash_requirement"
        || identities
            .get("missing_terms_remain_null")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("transportation trust-fund future identity fields failed".to_string());
    }

    if boundary_record
        .get("annual_value_rows")
        .and_then(serde_json::Value::as_array)
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err("transportation trust-fund annual value rows must remain empty".to_string());
    }

    let blockers = boundary_record
        .get("blocked_until_captured")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation trust-fund accounting blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "table_13_4_or_equivalent_official_annual_source",
        "annual_highway_trust_fund_receipts",
        "annual_highway_trust_fund_outgo",
        "annual_highway_trust_fund_balance",
        "annual_airport_and_airway_trust_fund_receipts",
        "annual_airport_and_airway_trust_fund_outgo",
        "annual_airport_and_airway_trust_fund_balance",
        "explicit_general_fund_transfer_amounts",
        "credited_offsetting_collections",
        "fund_balance_change_identity_recomputed",
        "mapping_to_transportation_function_400_net_outlays",
    ] {
        if !blockers.contains(required) {
            return Err(format!(
                "transportation trust-fund accounting blocker missing {required}"
            ));
        }
    }

    let outputs = boundary_record
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation trust-fund accounting outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "transportation trust-fund accounting output {field} must remain null"
            ));
        }
    }

    let claims = boundary_record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation trust-fund accounting claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation trust-fund accounting claim must be bool")?;
        if field == "accounting_boundary_published" {
            if !observed {
                return Err(
                    "transportation trust-fund accounting publish flag must be true".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "transportation trust-fund accounting public claim {field} must be false"
            ));
        }
    }

    let non_claim = string_field(&boundary_record, "non_claim_boundary")?;
    for required in [
        "transportation trust-fund accounting-boundary record",
        "not annual trust-fund values",
        "trust-fund reconciliation",
        "fund-balance path",
        "baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
    ] {
        if !non_claim.contains(required) {
            return Err(format!(
                "transportation trust-fund accounting boundary missing {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        TRANSPORTATION_PILOT_TRUST_FUND_ACCOUNTING_BOUNDARY_JSON_PATH,
        "No new external request was submitted",
        "trust funds remain separate",
        "general-fund transfers must be explicit",
        "borrowing from the general fund is financing, not a receipt",
        "repayment of borrowing is not an outlay",
        "offsetting collections and offsetting receipts cannot be silent",
        "fund-balance identity",
        "annual trust-fund values",
        "Table 13-4 or equivalent official annual source capture",
        "trust-fund reconciliation",
        "fund-balance path",
        "transportation baseline path",
        "simulator run",
        "target-cost selection",
        "rate calculation",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "technology-savings claim",
        "balanced-budget claim",
        "Missing annual values remain null, not zero",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "transportation trust-fund accounting reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_table_13_4_fy2025_2031_context_path(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation Table 13-4 context artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-table-13-4-fy2025-2031-context-path:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_current_law_context_path"
        || string_field(&record, "status")? != "draft_table_13_4_values_reconciliation_blocked"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || string_field(&record, "source_id")? != "SRC-OMB-AP-13-TABLES-FY2027"
        || string_field(&record, "workbook_sheet")? != "13-4"
        || string_field(&record, "unit")? != "billions_usd"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-AP-13-TABLES-FY2027/2026-07-23/ap_13_tables_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 47_862
        || string_field(&record, "raw_sha256")?
            != "86e550d366f218435f3ef9af43bafe37ff5a2e496680013a7d1dbcab7737c505"
    {
        return Err("transportation Table 13-4 context identity failed".to_string());
    }

    let status = record
        .get("path_status")
        .ok_or("transportation Table 13-4 path status")?;
    for (field, expected) in [
        ("official_fy2025_fy2031_rows_present", true),
        ("source_custody_ready", true),
        ("trust_funds_remain_separate", true),
        ("complete_fy2025_fy2035_path_ready", false),
        ("explicit_general_fund_transfer_path_ready", false),
        ("credited_offsetting_collections_ready", false),
        ("fund_balance_reconciliation_ready", false),
        ("mapping_to_function_400_ready", false),
        ("solver_ready", false),
        ("rate_ready", false),
        ("savings_ready", false),
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(expected) {
            return Err(format!("transportation Table 13-4 status {field} failed"));
        }
    }
    if int_field(status, "row_count")? != 14
        || int_field(status, "actual_rows")? != 2
        || int_field(status, "projection_rows")? != 12
    {
        return Err("transportation Table 13-4 row counts failed".to_string());
    }

    let rows = record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Table 13-4 annual rows")?;
    if rows.len() != 14 {
        return Err("transportation Table 13-4 must have 14 annual rows".to_string());
    }

    let expected = [
        ("airport_and_airway_trust_fund", 2025, 23.6, -19.9, 21.7),
        ("airport_and_airway_trust_fund", 2026, 22.0, -21.6, 22.2),
        ("airport_and_airway_trust_fund", 2027, 23.3, -21.4, 24.1),
        ("airport_and_airway_trust_fund", 2028, 24.9, -23.0, 26.0),
        ("airport_and_airway_trust_fund", 2029, 26.3, -22.5, 29.8),
        ("airport_and_airway_trust_fund", 2030, 27.9, -22.2, 35.5),
        ("airport_and_airway_trust_fund", 2031, 29.6, -22.1, 43.0),
        ("highway_trust_fund", 2025, 47.7, -74.4, 73.5),
        ("highway_trust_fund", 2026, 48.9, -75.9, 46.3),
        ("highway_trust_fund", 2027, 48.1, -77.7, 16.4),
        ("highway_trust_fund", 2028, 47.7, -79.8, -15.6),
        ("highway_trust_fund", 2029, 47.1, -81.0, -49.6),
        ("highway_trust_fund", 2030, 46.8, -82.4, -85.1),
        ("highway_trust_fund", 2031, 46.5, -83.8, -122.4),
    ];
    let expected = expected
        .into_iter()
        .map(|(fund, year, income, outgo, balance)| {
            (
                (fund.to_string(), i64::from(year)),
                (income, outgo, balance),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut observed = BTreeSet::new();
    for row in rows {
        let key = (
            string_field(row, "trust_fund_id")?,
            int_field(row, "fiscal_year")?,
        );
        observed.insert(key.clone());
        let (income, outgo, balance) = expected
            .get(&key)
            .ok_or("unexpected transportation Table 13-4 fund/year row")?;
        for (field, expected_value) in [
            ("total_income", *income),
            ("outgo", *outgo),
            ("balance_end", *balance),
        ] {
            if (number_field(row, field)? - expected_value).abs() > 0.0001 {
                return Err(format!(
                    "transportation Table 13-4 row {:?} field {field} failed",
                    key
                ));
            }
        }
    }
    if observed != expected.keys().cloned().collect::<BTreeSet<_>>() {
        return Err("transportation Table 13-4 year/fund coverage failed".to_string());
    }

    let missing = record
        .get("missing_year_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Table 13-4 missing rows")?;
    if missing.len() != 8 {
        return Err("transportation Table 13-4 must retain 8 missing-year rows".to_string());
    }
    for row in missing {
        let year = int_field(row, "fiscal_year")?;
        let fund = string_field(row, "trust_fund_id")?;
        if !(2032..=2035).contains(&year)
            || !matches!(
                fund.as_str(),
                "airport_and_airway_trust_fund" | "highway_trust_fund"
            )
        {
            return Err("transportation Table 13-4 missing row identity failed".to_string());
        }
        for field in ["total_income", "outgo", "balance_end"] {
            if !row.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "transportation Table 13-4 missing {field} must stay null"
                ));
            }
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Table 13-4 claims")?;
    for field in [
        "transportation_table_13_4_fy2025_2031_context_path_published",
        "official_fy2025_fy2031_rows_present",
        "source_custody_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation Table 13-4 claim {field} must be true"
            ));
        }
    }
    for field in [
        "complete_fy2025_fy2035_transportation_trust_fund_path_ready",
        "explicit_general_fund_transfer_path_ready",
        "credited_offsetting_collections_ready",
        "fund_balance_reconciliation_published",
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
            return Err(format!(
                "transportation Table 13-4 claim {field} must be false"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "Transportation trust-fund FY2025-FY2031 context only",
        "Highway Trust Fund and Airport and Airway Trust Fund rows remain separate",
        "not a complete FY2025-FY2035 path",
        "not solver input",
        "not a rate calculation",
        "not a savings estimate",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "transportation Table 13-4 warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_READER_PATH}: {err}"
        )
    })?;
    for required in [
        "FY2025-FY2031",
        "trust funds remain separate",
        "FY2032-FY2035 values",
        "solver input",
        "rate calculation",
        "savings estimate",
        "balanced-budget claim",
        "null, not zero",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "transportation Table 13-4 reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_receipt_base_work_item_progress(root: &Path) -> Result<(), String> {
    for path in [
        TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH,
        TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_SCHEMA_PATH,
        TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation receipt base progress artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let progress: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&progress, "record_id")? != "transportation-receipt-base-work-item-progress:v1"
        || string_field(&progress, "record_family")?
            != "transportation_receipt_base_work_item_progress"
        || int_field(&progress, "pulse")? != 136
        || string_field(&progress, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&progress, "receipt_base_source_work_queue_path")?
            != RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&progress, "receipt_base_work_item_completion_path")?
            != RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH
        || string_field(
            &progress,
            "current_law_fy2025_dedicated_receipt_anchors_path",
        )? != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&progress, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&progress, "work_item_id")? != "capture-transportation-excise-user-fee-base"
    {
        return Err("transportation receipt base progress identity failed".to_string());
    }

    for path in [
        string_field(&progress, "contract_path")?,
        string_field(&progress, "receipt_base_source_work_queue_path")?,
        string_field(&progress, "receipt_base_work_item_completion_path")?,
        string_field(
            &progress,
            "current_law_fy2025_dedicated_receipt_anchors_path",
        )?,
        string_field(&progress, "rate_publication_readiness_rollup_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "transportation receipt base progress referenced path missing: {path}"
            ));
        }
    }

    let status = progress
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation receipt base progress source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_local_custody_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "transportation_receipt_yield_context_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation receipt base progress status {field} must be true"
            ));
        }
    }
    for field in [
        "legal_receipt_base_ready",
        "economic_receipt_base_ready",
        "matched_receipt_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation receipt base progress status {field} must be false"
            ));
        }
    }

    let rows = progress
        .get("progress_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation receipt base progress rows")?;
    if rows.len() != 2 {
        return Err("transportation receipt base progress row count failed".to_string());
    }

    let mut receipts = BTreeMap::new();
    for row in rows {
        if string_field(row, "source_id")? != "SRC-OMB-HIST-2-4-FY2027"
            || int_field(row, "fiscal_year")? != 2025
            || row.get("legal_base") != Some(&serde_json::Value::Null)
            || row.get("economic_base") != Some(&serde_json::Value::Null)
            || row.get("assigned_base_rate") != Some(&serde_json::Value::Null)
            || row
                .get("ready_for_assigned_base")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("ready_for_rate_publication")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("ready_for_solver")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("transportation receipt base progress row guard failed".to_string());
        }
        receipts.insert(
            string_field(row, "anchor_id")?,
            int_field(row, "receipt_yield_musd")?,
        );
    }
    if receipts.get("transportation_trust_fund_excise") != Some(&43_768)
        || receipts.get("airport_and_airway_trust_fund_excise_context") != Some(&23_118)
    {
        return Err("transportation receipt base progress receipt values failed".to_string());
    }

    let reconciliation = progress
        .get("reconciliation")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation receipt base progress reconciliation")?;
    let transportation = reconciliation
        .get("transportation_trust_fund_excise_musd")
        .and_then(serde_json::Value::as_i64)
        .ok_or("transportation trust fund excise amount")?;
    let airport = reconciliation
        .get("airport_and_airway_excise_musd")
        .and_then(serde_json::Value::as_i64)
        .ok_or("airport and airway excise amount")?;
    let combined = reconciliation
        .get("combined_context_receipt_yield_musd")
        .and_then(serde_json::Value::as_i64)
        .ok_or("combined transportation receipt context amount")?;
    if transportation + airport != combined || combined != 66_886 {
        return Err("transportation receipt base progress reconciliation failed".to_string());
    }
    for field in [
        "matches_current_law_dedicated_receipt_anchor_formula",
        "context_only_not_assigned_base",
    ] {
        if reconciliation
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "transportation receipt base progress reconciliation {field} must be true"
            ));
        }
    }

    let anchors_text =
        fs::read_to_string(root.join(CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let anchors: serde_json::Value =
        serde_json::from_str(&anchors_text).map_err(|e| e.to_string())?;
    let anchor_rows = anchors
        .get("receipt_anchor_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current law receipt anchor rows")?;
    let mut anchor_receipts = BTreeMap::new();
    for row in anchor_rows {
        let anchor_id = string_field(row, "anchor_id")?;
        if anchor_id == "transportation_trust_fund_excise"
            || anchor_id == "airport_and_airway_trust_fund_excise_context"
        {
            anchor_receipts.insert(anchor_id, int_field(row, "amount_musd")?);
        }
    }
    if anchor_receipts != receipts {
        return Err("transportation progress does not match dedicated receipt anchors".to_string());
    }

    let summary = progress
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation receipt base progress summary")?;
    if summary
        .get("receipt_yield_context_rows")
        .and_then(serde_json::Value::as_i64)
        != Some(2)
        || summary
            .get("receipt_yield_context_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || summary
            .get("assigned_base_ready_count")
            .and_then(serde_json::Value::as_i64)
            != Some(0)
        || summary
            .get("remaining_receipt_base_work_item_count")
            .and_then(serde_json::Value::as_i64)
            != Some(4)
    {
        return Err("transportation receipt base progress summary failed".to_string());
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation receipt base progress summary {field} must be false"
            ));
        }
    }

    let blocked = progress
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation receipt base progress blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "current_law_yields_by_tax_or_fee_type",
        "reform_yields",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "transportation receipt base progress blocked output {field} must be null"
            ));
        }
    }

    let claims = progress
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation receipt base progress claims")?;
    if claims
        .get("transportation_receipt_yield_context_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("transportation receipt base progress published flag must be true".to_string());
    }
    for field in [
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation receipt base progress claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH,
        "Transportation receipt-yield context is now source-custodied, but it is not a legal or economic assigned receipt base.",
        "OMB trust-fund excise receipt rows are receipt-yield context, not statutory-rate denominators.",
        "Transportation legal bases, economic bases, elasticities, burdens, distribution, current-law yield by tax or fee type, reform yield, rates, and solver inputs remain blocked.",
        "No rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
        "No external request was submitted and no agency or person was contacted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation receipt base progress reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_infrastructure_outcome_floor_definition_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation/infrastructure outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "transportation-infrastructure-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")?
            != "transportation_infrastructure_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 169
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(&record, "veterans_outcome_floor_definition_packet_path")?
            != VETERANS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "transportation_depth_card_path")?
            != TRANSPORTATION_DEPTH_CARD_JSON_PATH
        || string_field(
            &record,
            "transportation_pilot_floor_indicator_contract_path",
        )? != TRANSPORTATION_PILOT_FLOOR_INDICATOR_CONTRACT_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err(
            "transportation/infrastructure floor definition packet identity failed".to_string(),
        );
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation/infrastructure floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation/infrastructure floor status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "asset_inventory_ready",
        "maintenance_gap_ready",
        "federal_state_local_translation_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "target_cost_ready",
        "simulator_ready",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation/infrastructure floor status {field} must be false"
            ));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation/infrastructure floor definition policy")?;
    for field in [
        "federal_state_local_translation_required",
        "asset_inventory_and_maintenance_gap_required_before_target_cost",
        "pilot_floor_indicator_contract_remains_thresholdless",
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "international_differences_not_savings",
        "no_fraud_inference",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation/infrastructure floor policy {field} must be true"
            ));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation/infrastructure required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("transportation/infrastructure required floor class count failed".to_string());
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
        return Err("transportation/infrastructure required floor class set failed".to_string());
    }
    for row in classes {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!(
                    "transportation/infrastructure floor class {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err(
                "transportation/infrastructure floor class must remain unpassed".to_string(),
            );
        }
    }

    let lane_floors = record
        .get("transportation_infrastructure_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation/infrastructure-specific floor definitions")?;
    let expected_lane_floors = [
        "asset_condition",
        "fatalities",
        "reliability",
        "access",
        "climate_resilience",
        "asset_inventory_maintenance_gap_delivery_feasibility",
    ];
    if lane_floors.len() != expected_lane_floors.len() {
        return Err("transportation/infrastructure-specific floor count failed".to_string());
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
        return Err("transportation/infrastructure-specific floor set failed".to_string());
    }
    for row in lane_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "transportation/infrastructure-specific floors must remain null and unpassed"
                    .to_string(),
            );
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
        .ok_or("transportation/infrastructure floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("transportation_infrastructure_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(6)
    {
        return Err("transportation/infrastructure floor summary counts failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "all_floors_passed",
        "target_cost_ready",
        "simulator_ready",
        "solver_input_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation/infrastructure floor summary {field} must be false"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation/infrastructure floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err(
            "transportation/infrastructure floor packet publication flag failed".to_string(),
        );
    }
    for field in [
        "asset_inventory_ready",
        "maintenance_gap_ready",
        "federal_state_local_translation_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
        "simulator_ready",
        "simulator_run_published",
        "target_cost_published",
        "federal_effect_published",
        "gross_savings_published",
        "net_savings_published",
        "solver_input_ready",
        "public_rate_card_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation/infrastructure floor claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This transportation/infrastructure floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "The transportation pilot floor indicator contract remains thresholdless and does not make the simulator ready.",
        "No lower-cost transportation scenario is admissible until asset condition, fatalities, reliability, access, climate resilience, equity, adequacy/resilience, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "No simulator run, target cost, federal effect, gross savings, net savings, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not an asset inventory",
        "not a maintenance-gap estimate",
        "not a federal/state/local translation",
        "not a simulator run",
        "not a federal score",
        "not a target-cost selection",
        "not solver input",
        "not a rate calculation",
        "not a savings estimate",
        "not a fraud finding",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation/infrastructure floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_roadway_fatality_rate_floor_value_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH,
        TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation roadway fatality-rate floor-value artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-roadway-fatality-rate-floor-value-packet:v1"
        || string_field(&record, "record_family")?
            != "transportation_roadway_fatality_rate_floor_value_packet"
        || int_field(&record, "pulse")? != 219
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || string_field(&record, "floor_id")? != "fatalities"
        || string_field(&record, "floor_class")? != "quality_safety"
        || string_field(&record, "floor_definition_packet_path")?
            != TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err("transportation roadway fatality-rate identity failed".to_string());
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("transportation roadway fatality-rate threshold")?;
    if string_field(threshold, "rationale_id")? != "no-regression-from-2024-fars-arf-fatality-rate"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_ceiling"
        || (number_field(threshold, "threshold_value")? - 1.19).abs() > 0.000001
        || string_field(threshold, "threshold_unit")?
            != "fatalities_per_100_million_vehicle_miles_traveled"
        || !string_field(threshold, "source_table")?.contains("2024 FARS ARF")
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("transportation roadway fatality-rate threshold failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("transportation roadway fatality-rate baseline")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("transportation roadway fatality-rate primary baseline")?;
    if string_field(baseline, "reporting_period")? != "calendar year 2024 FARS Annual Report File"
        || (number_field(primary, "value")? - 1.19).abs() > 0.000001
        || string_field(primary, "source_id")? != "SRC-NHTSA-CRASHSTATS-813800-2025-EARLY-2024-ARF"
    {
        return Err("transportation roadway fatality-rate baseline failed".to_string());
    }

    let context = baseline
        .get("supporting_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation roadway fatality-rate supporting context")?;
    if context.len() != 3
        || !context.iter().any(|row| {
            row.get("value").and_then(serde_json::Value::as_f64) == Some(39_254.0)
                && row
                    .get("evidence_status")
                    .and_then(serde_json::Value::as_str)
                    == Some("reported_2024_fars_arf")
        })
        || context
            .iter()
            .filter(|row| {
                row.get("evidence_status")
                    .and_then(serde_json::Value::as_str)
                    == Some("statistical_projection_not_baseline")
            })
            .count()
            != 2
    {
        return Err("transportation roadway fatality-rate context failed".to_string());
    }

    let custody = baseline
        .get("source_custody")
        .ok_or("transportation roadway fatality-rate custody")?;
    let raw_path = string_field(custody, "raw_artifact_path")?;
    if int_field(custody, "raw_byte_count")? != 710_466
        || string_field(custody, "raw_sha256")?
            != "745a943bf494b43f615d9718940981e4d4bdc1d56d86629aac00f618a61a38a8"
        || fs::metadata(root.join(&raw_path))
            .map_err(|err| format!("failed to stat {raw_path}: {err}"))?
            .len()
            != 710_466
    {
        return Err("transportation roadway fatality-rate custody failed".to_string());
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!("transportation {field} must remain null"));
        }
    }
    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation roadway fatality-rate readiness")?;
    for (field, value) in readiness {
        let observed = value.as_bool().ok_or("transportation readiness bool")?;
        let should_be_true = matches!(
            field.as_str(),
            "threshold_rationale_ready" | "threshold_value_populated" | "baseline_value_ready"
        );
        if observed != should_be_true {
            return Err(format!("transportation readiness failed: {field}"));
        }
    }
    for (field, value) in record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation blocked outputs")?
    {
        if !value.is_null() {
            return Err(format!(
                "transportation blocked output must be null: {field}"
            ));
        }
    }
    for (field, value) in record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation claims")?
    {
        let observed = value.as_bool().ok_or("transportation claim bool")?;
        let should_be_true = matches!(
            field.as_str(),
            "roadway_fatality_rate_floor_value_packet_published"
                | "threshold_rationale_ready"
                | "threshold_value_populated"
                | "baseline_value_ready"
        );
        if observed != should_be_true {
            return Err(format!("transportation claim failed: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "2025 values are statistical projections",
        "not a complete transportation floor",
        "not policy values",
        "not stress values",
        "not pass/fail evidence",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!("transportation warning missing {required}"));
        }
    }

    let schema = fs::read_to_string(
        root.join(TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_SCHEMA_PATH),
    )
    .map_err(|err| format!("failed to read transportation schema: {err}"))?;
    if !schema.contains("transportation_roadway_fatality_rate_floor_value_packet") {
        return Err("transportation schema missing record family".to_string());
    }
    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|err| format!("failed to read transportation reader: {err}"))?;
    for required in [
        TRANSPORTATION_ROADWAY_FATALITY_RATE_FLOOR_VALUE_PACKET_JSON_PATH,
        "1.19 per 100 million VMT",
        "39,254 people",
        "1.10 per 100 million VMT",
        "draft no-regression roadway fatality-rate floor threshold",
        "2025 values are statistical projections and context only",
        "policy and stress values remain null",
        "not a complete transportation floor",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("transportation reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_table_13_4_aggregate_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation Table 13-4 aggregate artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-table-13-4-aggregate-fy2025-2031-context:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_table_13_4_aggregate_context"
        || string_field(&record, "status")?
            != "draft_aggregate_context_solver_reconciliation_blocked"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || string_field(&record, "source_context_path")?
            != TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH
        || string_field(&record, "source_id")? != "SRC-OMB-AP-13-TABLES-FY2027"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-AP-13-TABLES-FY2027/2026-07-23/ap_13_tables_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 47_862
        || string_field(&record, "raw_sha256")?
            != "86e550d366f218435f3ef9af43bafe37ff5a2e496680013a7d1dbcab7737c505"
    {
        return Err("transportation Table 13-4 aggregate identity failed".to_string());
    }

    let boundary = record
        .get("aggregation_boundary")
        .ok_or("transportation Table 13-4 aggregate boundary")?;
    for field in [
        "source_rows_are_official_context",
        "funds_kept_separate_in_source_context",
        "aggregate_is_diagnostic_only",
        "not_function_400_mapping",
        "not_general_fund_transfer_schedule",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation Table 13-4 aggregate boundary {field} failed"
            ));
        }
    }

    let rows = record
        .get("annual_aggregate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Table 13-4 aggregate rows")?;
    let expected = [
        (2025, 71.3, -94.3, 95.2, false, false),
        (2026, 70.9, -97.5, 68.5, false, false),
        (2027, 71.4, -99.1, 40.5, false, false),
        (2028, 72.6, -102.8, 10.4, true, false),
        (2029, 73.4, -103.5, -19.8, true, true),
        (2030, 74.7, -104.6, -49.6, true, true),
        (2031, 76.1, -105.9, -79.4, true, true),
    ]
    .into_iter()
    .map(
        |(year, income, outgo, balance, highway_negative, combined_negative)| {
            (
                year,
                (income, outgo, balance, highway_negative, combined_negative),
            )
        },
    )
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected.len() {
        return Err("transportation Table 13-4 aggregate row count failed".to_string());
    }
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        observed_years.insert(year);
        let (income, outgo, balance, highway_negative, combined_negative) = expected
            .get(&(year as i32))
            .ok_or("unexpected transportation Table 13-4 aggregate year")?;
        for (field, expected_value) in [
            ("combined_total_income", *income),
            ("combined_outgo", *outgo),
            ("combined_balance_end", *balance),
        ] {
            if (number_field(row, field)? - expected_value).abs() > 0.0001 {
                return Err(format!(
                    "transportation Table 13-4 aggregate {year} {field} failed"
                ));
            }
        }
        if row
            .get("highway_negative_balance")
            .and_then(serde_json::Value::as_bool)
            != Some(*highway_negative)
            || row
                .get("combined_negative_balance")
                .and_then(serde_json::Value::as_bool)
                != Some(*combined_negative)
            || row
                .get("funds_included")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.len() != 2)
        {
            return Err(format!(
                "transportation Table 13-4 aggregate {year} flags failed"
            ));
        }
    }
    if observed_years != (2025..=2031).map(i64::from).collect::<BTreeSet<_>>() {
        return Err("transportation Table 13-4 aggregate year coverage failed".to_string());
    }

    let findings = record
        .get("diagnostic_findings")
        .ok_or("transportation Table 13-4 aggregate findings")?;
    if int_field(findings, "highway_first_negative_balance_year")? != 2028
        || int_field(findings, "combined_first_negative_balance_year")? != 2029
        || (number_field(findings, "fy2025_combined_balance_end")? - 95.2).abs() > 0.0001
        || (number_field(findings, "fy2031_combined_balance_end")? - -79.4).abs() > 0.0001
        || findings
            .get("airport_airway_balance_positive_through_2031")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("transportation Table 13-4 aggregate findings failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Table 13-4 aggregate blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "transportation Table 13-4 aggregate blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Table 13-4 aggregate claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation Table 13-4 aggregate claim bool")?;
        if matches!(
            field.as_str(),
            "transportation_trust_fund_table_13_4_aggregate_context_published"
                | "diagnostic_aggregate_context_ready"
        ) {
            if !observed {
                return Err(format!(
                    "transportation Table 13-4 aggregate claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "transportation Table 13-4 aggregate claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "diagnostic context only",
        "source rows remain separate",
        "does not supply FY2032-FY2035 values",
        "not solver input",
        "not a public rate card",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "transportation Table 13-4 aggregate warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH,
        "Highway Trust Fund first shows a negative ending balance in FY2028",
        "combined Highway plus Airport/Airway ending balance first turns negative",
        "not Function 400 mapping",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation Table 13-4 aggregate reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_table_13_4_identity_diagnostic(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation Table 13-4 identity diagnostic artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-table-13-4-identity-diagnostic:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_table_13_4_identity_diagnostic"
        || string_field(&record, "status")?
            != "draft_table_13_4_internal_identity_diagnostic_solver_blocked"
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || string_field(&record, "source_context_path")?
            != TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH
        || string_field(&record, "source_id")? != "SRC-OMB-AP-13-TABLES-FY2027"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-AP-13-TABLES-FY2027/2026-07-23/ap_13_tables_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 47_862
        || string_field(&record, "raw_sha256")?
            != "86e550d366f218435f3ef9af43bafe37ff5a2e496680013a7d1dbcab7737c505"
        || string_field(&record, "unit")? != "billions_usd"
    {
        return Err("transportation Table 13-4 identity diagnostic identity failed".to_string());
    }

    let method = record
        .get("identity_method")
        .ok_or("transportation Table 13-4 identity method")?;
    if int_field(method, "rows_checked")? != 14
        || number_field(method, "rounding_tolerance_billions")? != 0.1
        || method
            .get("checks")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|checks| checks.len() != 5)
    {
        return Err("transportation Table 13-4 identity method failed".to_string());
    }

    let summary = record
        .get("identity_summary")
        .ok_or("transportation Table 13-4 identity summary")?;
    if int_field(summary, "rows_checked")? != 14
        || int_field(summary, "rows_with_any_delta_over_tolerance")? != 0
        || number_field(summary, "max_abs_delta_billions")? != 0.1
        || int_field(summary, "airport_airway_rows_checked")? != 7
        || int_field(summary, "highway_rows_checked")? != 7
        || int_field(summary, "highway_first_negative_balance_year")? != 2028
        || summary
            .get("airport_airway_balance_positive_through_2031")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || summary
            .get("all_deltas_within_rounding_tolerance")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("transportation Table 13-4 identity summary failed".to_string());
    }

    let observations = record
        .get("rounding_observations")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Table 13-4 rounding observations")?;
    if observations.len() != 7 {
        return Err("transportation Table 13-4 rounding observation count failed".to_string());
    }
    let observed = observations
        .iter()
        .map(|item| {
            Ok((
                string_field(item, "trust_fund_id")?,
                int_field(item, "fiscal_year")?,
                string_field(item, "largest_delta_field")?,
            ))
        })
        .collect::<Result<BTreeSet<_>, String>>()?;
    let expected = [
        (
            "airport_and_airway_trust_fund",
            2028,
            "income_components_to_total_income",
        ),
        (
            "airport_and_airway_trust_fund",
            2030,
            "income_components_to_total_income",
        ),
        (
            "airport_and_airway_trust_fund",
            2031,
            "income_components_to_total_income",
        ),
        (
            "highway_trust_fund",
            2026,
            "income_components_to_total_income",
        ),
        (
            "highway_trust_fund",
            2027,
            "subtotal_to_total_change_and_balance_end",
        ),
        ("highway_trust_fund", 2028, "balance_end_identity"),
        ("highway_trust_fund", 2029, "noninterest_income_plus_outgo"),
    ]
    .into_iter()
    .map(|(fund, year, field)| (fund.to_string(), year, field.to_string()))
    .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err("transportation Table 13-4 rounding observation set failed".to_string());
    }

    for array_name in ["diagnostic_findings", "blocked_model_steps"] {
        if record
            .get(array_name)
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "transportation Table 13-4 identity {array_name} must be nonempty"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Table 13-4 identity blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "transportation Table 13-4 identity blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Table 13-4 identity claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation Table 13-4 identity claim bool")?;
        if matches!(
            field.as_str(),
            "transportation_trust_fund_table_13_4_identity_diagnostic_published"
                | "internal_identity_diagnostic_ready"
                | "all_deltas_within_rounding_tolerance"
        ) {
            if !observed {
                return Err(format!(
                    "transportation Table 13-4 identity claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "transportation Table 13-4 identity claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "internally reconcile within rounding tolerance",
        "not FY2032-FY2035 values",
        "not a complete transportation trust-fund path",
        "not explicit general-fund transfers",
        "not credited offsetting collections",
        "not Function 400 mapping",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "transportation Table 13-4 identity warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH,
        "14 fund-year rows checked",
        "0 rows have a delta above",
        "Highway Trust Fund balance turns negative in FY2028",
        "does not add FY2032-FY2035 values",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation Table 13-4 identity reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_cbo_balance_extension_fy2032_2035_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation CBO balance extension artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(
        TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-cbo-balance-extension-fy2032-2035-context:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_cbo_balance_extension_context"
        || string_field(&record, "status")?
            != "draft_cbo_balance_extension_context_income_outgo_reconciliation_blocked"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation CBO balance extension identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("transportation CBO balance extension custody")?;
    if string_field(custody, "source_id")? != "SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02"
        || string_field(custody, "publisher")? != "Congressional Budget Office"
        || string_field(custody, "retrieval_date")? != "2026-07-23"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/cbo/SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02/2026-07-23/annual_fy_2026-02.csv"
        || int_field(custody, "byte_count")? != 25_466
        || int_field(custody, "row_count")? != 728
        || string_field(custody, "sha256")?
            != "4fcc0f725e6ab002107bedf461e21dacbdc6ac49e85475b8ff3e8aa20c3cdaab"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("transportation CBO balance extension custody failed".to_string());
    }
    let raw_file = root.join(string_field(custody, "raw_artifact_path")?);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 25_466
        || sha256_file(&raw_file)?
            != "4fcc0f725e6ab002107bedf461e21dacbdc6ac49e85475b8ff3e8aa20c3cdaab"
    {
        return Err("transportation CBO balance extension raw custody failed".to_string());
    }

    let scope = record
        .get("source_variable_scope")
        .ok_or("transportation CBO balance extension scope")?;
    for field in [
        "no_interpolation_used",
        "cbo_balance_rows_are_unambiguous",
        "surplus_variables_have_duplicate_date_variable_rows",
        "surplus_variables_excluded_from_values",
        "not_income_outgo_reconciliation",
        "not_omb_table_13_4_extension",
        "not_function_400_mapping",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation CBO balance extension scope {field} failed"
            ));
        }
    }

    let rows = record
        .get("fy2032_2035_balance_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation CBO balance extension rows")?;
    let expected = [
        (2032, 38.411, 0.0, 38.411),
        (2033, 42.533, 0.0, 42.533),
        (2034, 47.123, 0.0, 47.123),
        (2035, 52.210, 0.0, 52.210),
    ]
    .into_iter()
    .map(|(year, airport, highway, combined)| (year, (airport, highway, combined)))
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        observed_years.insert(year);
        let (airport, highway, combined) = expected
            .get(&year)
            .ok_or("unexpected transportation CBO balance extension year")?;
        if (number_field(row, "airport_and_airway_balance_end")? - airport).abs() > 0.0001
            || (number_field(row, "highway_balance_end")? - highway).abs() > 0.0001
            || (number_field(row, "combined_balance_end")? - combined).abs() > 0.0001
            || row
                .get("highway_zero_balance")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "transportation CBO balance extension row failed for FY{year}"
            ));
        }
    }
    if observed_years != expected.keys().copied().collect::<BTreeSet<_>>() {
        return Err("transportation CBO balance extension year set failed".to_string());
    }

    let overlap = record
        .get("fy2031_overlap_diagnostic")
        .ok_or("transportation CBO balance extension overlap")?;
    if string_field(overlap, "omb_table_13_4_source_context_path")?
        != TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH
        || (number_field(overlap, "airport_and_airway_cbo_minus_omb")? + 8.301).abs() > 0.0001
        || (number_field(overlap, "highway_cbo_minus_omb")? - 122.4).abs() > 0.0001
        || !string_field(overlap, "diagnostic_note")?.contains("does not authorize stitching")
    {
        return Err("transportation CBO balance extension overlap failed".to_string());
    }

    let duplicate_boundary = record
        .get("duplicate_variable_boundary")
        .ok_or("transportation CBO balance extension duplicate boundary")?;
    if duplicate_boundary
        .get("tf_sur_airport_duplicate_date_variable_rows_observed")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || duplicate_boundary
            .get("tf_sur_highway_duplicate_date_variable_rows_observed")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || duplicate_boundary
            .get("surplus_rows_used_for_income_outgo_or_solver")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("transportation CBO balance extension duplicate boundary failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation CBO balance extension blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "transportation CBO balance extension blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation CBO balance extension claims")?;
    for field in [
        "transportation_trust_fund_cbo_balance_extension_context_published",
        "source_custody_ready",
        "fy2032_fy2035_balance_context_present",
        "cbo_omb_overlap_diagnostic_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation CBO balance extension claim {field} must be true"
            ));
        }
    }
    for field in [
        "income_outgo_reconciliation_ready",
        "complete_transportation_trust_fund_path_ready",
        "explicit_general_fund_transfer_path_ready",
        "credited_offsetting_collections_ready",
        "function_400_mapping_ready",
        "solver_input_ready",
        "solver_run_published",
        "rate_calculation_published",
        "public_rate_card_published",
        "savings_estimate_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation CBO balance extension claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(
        TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_READER_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH,
        "FY2032-FY2035",
        "tf_bal_airport",
        "tf_bal_highway",
        "OMB Table 13-4 has Airport and Airway at 43.0",
        "not OMB Table 13-4 FY2032-FY2035 income/outgo rows",
        "not Function 400 mapping",
        "not solver input",
        "duplicate date/variable rows",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation CBO balance extension reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_cross_source_reconciliation_status(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_SCHEMA_PATH,
        TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation cross-source reconciliation artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-cross-source-reconciliation-status:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_cross_source_reconciliation_status"
        || string_field(&record, "status")?
            != "draft_cross_source_reconciliation_status_complete_path_blocked"
        || string_field(&record, "as_of_date")? != "2026-07-25"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
    {
        return Err("transportation cross-source reconciliation identity failed".to_string());
    }

    let paths = record
        .get("source_context_paths")
        .ok_or("transportation cross-source context paths")?;
    let expected_paths = [
        (
            "omb_table_13_4_context_path",
            TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH,
        ),
        (
            "omb_table_13_4_aggregate_context_path",
            TRANSPORTATION_TRUST_FUND_TABLE_13_4_AGGREGATE_CONTEXT_JSON_PATH,
        ),
        (
            "omb_table_13_4_identity_diagnostic_path",
            TRANSPORTATION_TRUST_FUND_TABLE_13_4_IDENTITY_DIAGNOSTIC_JSON_PATH,
        ),
        (
            "treasury_mts_fy2025_anchor_context_path",
            TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH,
        ),
        (
            "cbo_balance_extension_context_path",
            TRANSPORTATION_TRUST_FUND_CBO_BALANCE_EXTENSION_FY2032_2035_CONTEXT_JSON_PATH,
        ),
    ];
    for (field, expected_path) in expected_paths {
        if string_field(paths, field)? != expected_path || !root.join(expected_path).exists() {
            return Err(format!(
                "transportation cross-source context path failed: {field}"
            ));
        }
    }

    let custody = record
        .get("source_custody_summary")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation cross-source custody summary")?;
    if custody.len() != 3 {
        return Err("transportation cross-source custody summary count failed".to_string());
    }
    let source_ids = custody
        .iter()
        .map(|entry| string_field(entry, "source_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_sources = [
        "SRC-OMB-AP-13-TABLES-FY2027",
        "SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02",
        "SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if source_ids != expected_sources {
        return Err("transportation cross-source custody source set failed".to_string());
    }
    for entry in custody {
        match string_field(entry, "source_id")?.as_str() {
            "SRC-OMB-AP-13-TABLES-FY2027" => {
                let path = string_field(entry, "local_artifact_path")?;
                if path
                    != "data/raw/omb/SRC-OMB-AP-13-TABLES-FY2027/2026-07-23/ap_13_tables_fy2027.xlsx"
                    || int_field(entry, "byte_count")? != 47_862
                    || string_field(entry, "sha256")?
                        != "86e550d366f218435f3ef9af43bafe37ff5a2e496680013a7d1dbcab7737c505"
                    || !root.join(&path).exists()
                {
                    return Err("transportation cross-source OMB custody failed".to_string());
                }
            }
            "SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02" => {
                let path = string_field(entry, "local_artifact_path")?;
                if path
                    != "data/raw/cbo/SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02/2026-07-23/annual_fy_2026-02.csv"
                    || int_field(entry, "byte_count")? != 25_466
                    || string_field(entry, "sha256")?
                        != "4fcc0f725e6ab002107bedf461e21dacbdc6ac49e85475b8ff3e8aa20c3cdaab"
                    || !root.join(&path).exists()
                {
                    return Err("transportation cross-source CBO custody failed".to_string());
                }
            }
            "SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025" => {
                let files = entry
                    .get("raw_context_files")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("transportation cross-source Treasury raw context files")?;
                let mut observed_tables = BTreeSet::new();
                for file in files {
                    observed_tables.insert(string_field(file, "table")?);
                    let path = string_field(file, "local_artifact_path")?;
                    let expected = match string_field(file, "table")?.as_str() {
                        "mts_table_4" => (
                            "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv",
                            15_442,
                            "f82fdcae4b28e3a9a66dfeb20726d1a81d900ca5eabc3559741882e9258fb204",
                        ),
                        "mts_table_5" => (
                            "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv",
                            203_342,
                            "fb1646d18d9cc05a217b3b6ac084fd006e0bf01fa26c8ee8815b881579cea66a",
                        ),
                        other => {
                            return Err(format!(
                                "transportation cross-source unexpected Treasury table: {other}"
                            ));
                        }
                    };
                    if path != expected.0
                        || int_field(file, "byte_count")? != expected.1
                        || string_field(file, "sha256")? != expected.2
                        || !root.join(&path).exists()
                    {
                        return Err(
                            "transportation cross-source Treasury custody failed".to_string()
                        );
                    }
                }
                if observed_tables
                    != ["mts_table_4".to_string(), "mts_table_5".to_string()]
                        .into_iter()
                        .collect::<BTreeSet<_>>()
                {
                    return Err("transportation cross-source Treasury table set failed".to_string());
                }
            }
            other => {
                return Err(format!(
                    "transportation cross-source unexpected custody source: {other}"
                ));
            }
        }
    }

    let findings = record
        .get("reconciliation_findings")
        .ok_or("transportation cross-source reconciliation findings")?;
    for field in [
        "omb_table_13_4_identity_ready",
        "omb_table_13_4_all_deltas_within_rounding_tolerance",
        "airport_airway_balance_positive_through_2031_omb",
        "cbo_fy2032_2035_balance_context_present",
        "cbo_balance_rows_are_not_omb_table_13_4_extension",
        "treasury_fy2025_anchor_present",
        "federal_funds_are_not_general_fund",
    ] {
        if findings.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation cross-source finding should be true: {field}"
            ));
        }
    }
    for field in [
        "cbo_fy2032_2035_income_outgo_context_present",
        "function_400_mapping_ready",
        "complete_fy2025_fy2035_path_ready",
    ] {
        if findings.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation cross-source finding should be false: {field}"
            ));
        }
    }
    if int_field(findings, "omb_table_13_4_rows_checked")? != 14
        || number_field(findings, "omb_rounding_tolerance_billions")? != 0.1
        || int_field(findings, "highway_first_negative_balance_year_omb")? != 2028
    {
        return Err("transportation cross-source finding values failed".to_string());
    }

    let overlap = record
        .get("fy2031_overlap_boundary")
        .ok_or("transportation cross-source FY2031 overlap")?;
    if (number_field(overlap, "omb_airport_and_airway_balance_end")? - 43.0).abs() > 0.0001
        || (number_field(overlap, "cbo_airport_and_airway_balance_end")? - 34.699).abs() > 0.0001
        || (number_field(overlap, "airport_and_airway_cbo_minus_omb")? + 8.301).abs() > 0.0001
        || (number_field(overlap, "omb_highway_balance_end")? + 122.4).abs() > 0.0001
        || number_field(overlap, "cbo_highway_balance_end")? != 0.0
        || (number_field(overlap, "highway_cbo_minus_omb")? - 122.4).abs() > 0.0001
        || overlap
            .get("stitching_authorized")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || !string_field(overlap, "boundary_note")?.contains("may not be stitched")
    {
        return Err("transportation cross-source FY2031 overlap failed".to_string());
    }

    let requirements = record
        .get("remaining_reconciliation_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation cross-source remaining requirements")?;
    if requirements.len() != 6 {
        return Err("transportation cross-source requirement count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation cross-source blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "transportation cross-source blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation cross-source claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("transportation cross-source claim bool")?;
        match field.as_str() {
            "transportation_trust_fund_cross_source_reconciliation_status_published"
            | "omb_table_13_4_identity_ready"
            | "cbo_balance_extension_context_present"
            | "fy2031_overlap_boundary_published"
            | "treasury_fy2025_anchor_context_present" => {
                if !observed {
                    return Err(format!(
                        "transportation cross-source claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "transportation cross-source downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "cross-source reconciliation status only",
        "blocks stitching the sources",
        "not an OMB/CBO stitched path",
        "not FY2032-FY2035 income/outgo rows",
        "not a complete FY2025-FY2035 transportation trust-fund path",
        "not explicit general-fund transfers",
        "not Function 400 mapping",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "transportation cross-source warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_READER_PATH}: {err}"
        )
    })?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for phrase in [
        TRANSPORTATION_TRUST_FUND_CROSS_SOURCE_RECONCILIATION_STATUS_JSON_PATH,
        "OMB Table 13-4 internally reconciles for 14 fund-year rows",
        "CBO provides FY2032-FY2035 balance-only context",
        "federal funds are not general fund",
        "FY2031 OMB/CBO overlap",
        "blocks stitching CBO FY2032-FY2035 balances onto OMB Table 13-4",
        "not FY2032-FY2035 income/outgo rows",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader_words.contains(phrase) {
            return Err(format!(
                "transportation cross-source reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_transportation_trust_fund_treasury_mts_fy2025_anchor_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH,
        TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing transportation Treasury MTS artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "transportation-trust-fund-treasury-mts-fy2025-anchor-context:v1"
        || string_field(&record, "record_family")?
            != "transportation_trust_fund_treasury_mts_anchor_context"
        || string_field(&record, "status")?
            != "draft_fy2025_mts_anchor_context_reconciliation_blocked"
        || string_field(&record, "lane_id")? != "transportation-infrastructure"
        || int_field(&record, "fiscal_year")? != 2025
        || string_field(&record, "record_date")? != "2025-09-30"
    {
        return Err("transportation Treasury MTS identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("transportation Treasury MTS source custody")?;
    if string_field(custody, "source_id")? != "SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025.2026-07-24.metadata.md"
        || string_field(custody, "review_status")? != "source_metadata_present_and_hash_matched"
    {
        return Err("transportation Treasury MTS custody identity failed".to_string());
    }
    let files = custody
        .get("raw_context_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Treasury MTS raw files")?;
    let expected_files = [
        (
            "mts_table_4",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv",
                15_442,
                "f82fdcae4b28e3a9a66dfeb20726d1a81d900ca5eabc3559741882e9258fb204",
                57,
            ),
        ),
        (
            "mts_table_5",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv",
                203_342,
                "fb1646d18d9cc05a217b3b6ac084fd006e0bf01fa26c8ee8815b881579cea66a",
                811,
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if files.len() != expected_files.len() {
        return Err("transportation Treasury MTS raw file count failed".to_string());
    }
    for file in files {
        let table = string_field(file, "table")?;
        let (path, bytes, sha, rows) = expected_files
            .get(table.as_str())
            .ok_or("unexpected transportation Treasury MTS table")?;
        let raw = root.join(path);
        if string_field(file, "raw_artifact_path")? != *path
            || int_field(file, "raw_byte_count")? != *bytes
            || string_field(file, "raw_sha256")? != *sha
            || int_field(file, "row_count")? != *rows
            || !raw.exists()
            || fs::metadata(&raw).map_err(|err| err.to_string())?.len() != *bytes as u64
            || sha256_file(&raw)? != *sha
        {
            return Err(format!(
                "transportation Treasury MTS raw custody failed: {table}"
            ));
        }
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("transportation Treasury MTS boundary")?;
    for field in [
        "official_public_source",
        "local_raw_custody_ready",
        "table_4_receipt_rows_explicit_for_airport_and_highway",
        "table_5_airport_total_outlay_row_explicit",
        "table_5_highway_total_trust_fund_outlay_row_not_observed",
        "not_transportation_income_outgo_reconciliation",
        "not_function_400_mapping",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation Treasury MTS boundary {field} must be true"
            ));
        }
    }

    let receipts = record
        .get("fy2025_transportation_receipt_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Treasury MTS receipt rows")?;
    let expected_receipts = [
        ("airport_airway_receipts_mts_table_4", 44, 22_651.06130492),
        ("highway_receipts_mts_table_4", 45, 44_294.72985973),
    ]
    .into_iter()
    .map(|(id, line, musd)| (id, (line, musd)))
    .collect::<BTreeMap<_, _>>();
    if receipts.len() != expected_receipts.len() {
        return Err("transportation Treasury MTS receipt row count failed".to_string());
    }
    for row in receipts {
        let id = string_field(row, "anchor_id")?;
        let (line, musd) = expected_receipts
            .get(id.as_str())
            .ok_or("unexpected transportation Treasury MTS receipt row")?;
        if int_field(row, "source_line_number")? != *line
            || (number_field(row, "current_fytd_net_amount_musd")? - musd).abs() > 0.00001
        {
            return Err(format!(
                "transportation Treasury MTS receipt values failed: {id}"
            ));
        }
    }

    let outlays = record
        .get("fy2025_transportation_outlay_context_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Treasury MTS outlay rows")?;
    let expected_outlays = [
        (
            "airport_airway_total_outlays_mts_table_5",
            436,
            19_679.24588718,
            "explicit_airport_airway_total_row",
        ),
        (
            "federal_highway_administration_total_outlays_mts_table_5",
            444,
            65_032.57729402,
            "agency_total_not_highway_trust_fund_total",
        ),
    ]
    .into_iter()
    .map(|(id, line, musd, status)| (id, (line, musd, status)))
    .collect::<BTreeMap<_, _>>();
    if outlays.len() != expected_outlays.len() {
        return Err("transportation Treasury MTS outlay row count failed".to_string());
    }
    for row in outlays {
        let id = string_field(row, "anchor_id")?;
        let (line, musd, status) = expected_outlays
            .get(id.as_str())
            .ok_or("unexpected transportation Treasury MTS outlay row")?;
        if int_field(row, "source_line_number")? != *line
            || (number_field(row, "current_fytd_net_amount_musd")? - musd).abs() > 0.00001
            || string_field(row, "outlay_context_status")? != *status
        {
            return Err(format!(
                "transportation Treasury MTS outlay values failed: {id}"
            ));
        }
    }

    let negative = record
        .get("negative_transportation_outlay_rows_observed_not_used_as_standalone_bridge")
        .and_then(serde_json::Value::as_array)
        .ok_or("transportation Treasury MTS negative rows")?;
    if negative.len() != 2
        || !negative.iter().all(|row| {
            string_field(row, "boundary")
                .is_ok_and(|boundary| boundary.contains("not separately netted"))
        })
    {
        return Err("transportation Treasury MTS negative row boundary failed".to_string());
    }

    let comparison = record
        .get("comparison_to_existing_context")
        .ok_or("transportation Treasury MTS comparison")?;
    if string_field(comparison, "omb_table_13_4_context_path")?
        != TRANSPORTATION_TRUST_FUND_TABLE_13_4_FY2025_2031_CONTEXT_PATH_JSON_PATH
        || !string_field(comparison, "diagnostic_note")?.contains("not reconciled")
    {
        return Err("transportation Treasury MTS comparison failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Treasury MTS blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "transportation Treasury MTS blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation Treasury MTS claims")?;
    for field in [
        "transportation_treasury_mts_fy2025_anchor_context_published",
        "local_raw_custody_ready",
        "fy2025_transportation_receipt_outlay_context_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "transportation Treasury MTS claim {field} must be true"
            ));
        }
    }
    for field in [
        "transportation_trust_fund_income_outgo_reconciliation_ready",
        "fund_balance_reconciliation_ready",
        "function_400_mapping_ready",
        "solver_input_ready",
        "solver_run_published",
        "rate_calculation_published",
        "public_rate_card_published",
        "savings_estimate_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "transportation Treasury MTS claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        TRANSPORTATION_TRUST_FUND_TREASURY_MTS_FY2025_ANCHOR_CONTEXT_JSON_PATH,
        "MTS Table 4 line 44",
        "MTS Table 4 line 45",
        "MTS Table 5 line 436",
        "MTS Table 5 line 444",
        "agency total, not a Highway Trust Fund total",
        "not transportation trust-fund income/outgo reconciliation",
        "not Function 400 mapping",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "transportation Treasury MTS reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

