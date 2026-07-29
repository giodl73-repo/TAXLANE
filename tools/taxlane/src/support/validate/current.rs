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

pub(crate) fn validate_current_law_path_inventory(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_PATH_INVENTORY_JSON_PATH,
        CURRENT_LAW_PATH_INVENTORY_SCHEMA_PATH,
        CURRENT_LAW_PATH_INVENTORY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law path inventory artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_PATH_INVENTORY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let inventory: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&inventory, "record_id")? != "current-law-path-inventory:v1"
        || string_field(&inventory, "record_family")? != "current_law_path_inventory"
        || int_field(&inventory, "pulse")? != 108
        || string_field(&inventory, "solver_input_inventory_path")?
            != SOLVER_INPUT_INVENTORY_JSON_PATH
        || string_field(&inventory, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&inventory, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&inventory, "core_g_topline_spine_path")? != CORE_G_SOLVER_SPINE_JSON_PATH
        || string_field(&inventory, "core_g_topline_spine_status")?
            != "ready_federal_topline_only_not_lane_solver_input"
        || !bool_field(&inventory, "core_g_topline_spine_ready")?
    {
        return Err("current-law path inventory identity failed".to_string());
    }

    let horizon = inventory
        .get("horizon_requirement")
        .ok_or("current-law horizon requirement")?;
    if int_field(horizon, "baseline_year")? != 2025
        || int_field(horizon, "required_forward_years")? != 10
        || horizon
            .get("interpolation_allowed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || horizon
            .get("missing_values_remain_null")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("current-law horizon requirement failed".to_string());
    }
    let required_years = horizon
        .get("required_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law required years")?
        .iter()
        .map(|value| value.as_i64().ok_or("current-law required year int"))
        .collect::<Result<Vec<_>, _>>()?;
    if required_years != (2025..=2035).map(i64::from).collect::<Vec<_>>() {
        return Err("current-law required years must be FY2025-FY2035".to_string());
    }

    let rows = inventory
        .get("path_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law path rows")?;
    let observed = rows
        .iter()
        .map(|row| string_field(row, "path_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
        "health_fiscal_current_law_path",
        "net_interest_current_law_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || rows.len() != expected.len() {
        return Err("current-law path row set failed".to_string());
    }
    for row in rows {
        if row.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("current-law path rows must be required/false/null".to_string());
        }
        let missing_years = row
            .get("missing_years")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law missing years")?;
        let context_years = row
            .get("context_years_present")
            .or_else(|| row.get("context_years"))
            .and_then(serde_json::Value::as_array);
        let partial_years = row
            .get("partial_years")
            .and_then(serde_json::Value::as_array);
        if missing_years.is_empty()
            && context_years.is_none_or(Vec::is_empty)
            && partial_years.is_none_or(Vec::is_empty)
        {
            return Err(
                "current-law rows must retain missing years, explicit context years, or partial years"
                    .to_string(),
            );
        }
        if string_field(row, "official_source_family")?.is_empty() {
            return Err("current-law rows must name official source family".to_string());
        }
    }

    let rows_by_id = rows
        .iter()
        .map(|row| Ok((string_field(row, "path_id")?, row)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for (path_id, status) in [
        (
            "baseline_plus_ten_year_horizon",
            "partial_omb_fy2025_fy2031_plus_cbo_fy2032_fy2035_topline_context_not_unified_or_solver_bound",
        ),
        (
            "oasdi_fund_path",
            "partial_combined_oasdi_fy2025_fy2035_plus_cbo_balance_context_not_solver_bound",
        ),
        (
            "medicare_hi_fund_path",
            "partial_cy2025_cy2035_hi_context_plus_cbo_balance_context_fiscal_bridge_blocked",
        ),
        (
            "transportation_trust_fund_path",
            "partial_fy2025_fy2031_omb_trust_fund_context_plus_cbo_fy2032_fy2035_balance_context_reconciliation_blocked",
        ),
        (
            "health_fiscal_current_law_path",
            "partial_omb_cms_cbo_fy2026_fy2035_category_context_component_fiscal_path_blocked",
        ),
        (
            "net_interest_current_law_path",
            "partial_omb_fy2025_fy2031_plus_cbo_fy2032_fy2035_net_interest_debt_context_feedback_blocked",
        ),
    ] {
        if string_field(
            rows_by_id
                .get(path_id)
                .ok_or("current-law path row lookup")?,
            "coverage_status",
        )? != status
        {
            return Err(format!("current-law path status failed {path_id}"));
        }
    }

    let rules = inventory
        .get("inventory_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law rules")?;
    for required in [
        "official_sources_only",
        "raw_bytes_metadata_retrieval_date_byte_count_and_sha256_required",
        "no_interpolation_without_explicit_model",
        "missing_values_remain_null",
        "trust_funds_remain_separate",
        "medicare_hi_must_remain_separate",
        "current_law_zero_reform_delta_required",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("current-law rule failed {required}"));
        }
    }
    if rules
        .get("solver_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("current-law solver_ready must be false".to_string());
    }

    let claims = inventory
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("current-law claim bool")?;
        if field == "current_law_path_inventory_published" {
            if !observed {
                return Err("current-law inventory publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("current-law public claim {field} must be false"));
        }
    }

    let boundary = string_field(&inventory, "non_claim_boundary")?;
    for required in [
        "current-law path inventory",
        "not current-law path values",
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
            return Err(format!("current-law boundary missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(CURRENT_LAW_PATH_INVENTORY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        CURRENT_LAW_PATH_INVENTORY_JSON_PATH,
        "does not publish current-law path values",
        "Required years: FY2025 through FY2035",
        "OASDI annual fund path",
        "Medicare HI annual fund path",
        "transportation trust-fund annual values",
        "health fiscal current-law path",
        "net interest current-law path",
        "raw bytes, metadata, retrieval date, byte count, and SHA-256",
        "Interpolation is not allowed without an explicit model",
        "Missing values remain null",
        "Trust funds remain separate",
        "Medicare HI remains separate",
        "not current-law path values",
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
            return Err(format!("current-law reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_source_custody_preflight(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_SCHEMA_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law source-custody preflight artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let preflight: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&preflight, "record_id")? != "current-law-source-custody-preflight:v1"
        || string_field(&preflight, "record_family")? != "current_law_source_custody_preflight"
        || int_field(&preflight, "pulse")? != 109
        || string_field(&preflight, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&preflight, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&preflight, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
    {
        return Err("current-law source-custody preflight identity failed".to_string());
    }

    let requirements = preflight
        .get("custody_packet_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law source-custody requirements")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("current-law source-custody requirement string")
        })
        .collect::<Result<Vec<_>, _>>()?;
    if requirements
        != [
            "source_id",
            "official_host_or_publisher",
            "source_vintage",
            "retrieval_date",
            "raw_artifact_path",
            "raw_byte_count",
            "raw_sha256",
            "metadata_path",
            "extraction_method",
            "annual_years_covered",
            "component_mapping",
            "review_status",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    {
        return Err("current-law source-custody requirement list changed".to_string());
    }

    let rows = preflight
        .get("preflight_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law source-custody rows")?;
    let observed = rows
        .iter()
        .map(|row| string_field(row, "path_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
        "health_fiscal_current_law_path",
        "net_interest_current_law_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || rows.len() != expected.len() {
        return Err("current-law source-custody row set failed".to_string());
    }

    let nullable_fields = [
        "source_id",
        "official_host_or_publisher",
        "source_vintage",
        "retrieval_date",
        "raw_artifact_path",
        "raw_byte_count",
        "raw_sha256",
        "metadata_path",
        "extraction_method",
        "annual_years_covered",
        "component_mapping",
        "review_status",
    ];
    for row in rows {
        if row.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || row
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("values_may_be_populated")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("current-law source-custody rows must be required/false/false".to_string());
        }
        if string_field(row, "candidate_official_source_family")?.is_empty() {
            return Err("current-law source-custody rows must name source family".to_string());
        }
        for field in nullable_fields {
            if !row.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "current-law source-custody field {field} must remain null"
                ));
            }
        }
        let blockers = row
            .get("remaining_blockers")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law source-custody blockers")?;
        if blockers.is_empty() {
            return Err("current-law source-custody rows need blockers".to_string());
        }
    }

    let rules = preflight
        .get("preflight_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law source-custody rules")?;
    for required in [
        "no_external_request_submitted",
        "no_source_values_captured",
        "raw_bytes_required_before_value",
        "metadata_required_before_value",
        "retrieval_date_required_before_value",
        "byte_count_required_before_value",
        "sha256_required_before_value",
        "review_required_before_value",
        "missing_values_remain_null",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("current-law source-custody rule failed {required}"));
        }
    }
    if rules
        .get("solver_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("current-law source-custody solver_ready must be false".to_string());
    }

    let claims = preflight
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law source-custody claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("current-law source-custody claim bool")?;
        if field == "current_law_source_custody_preflight_published" {
            if !observed {
                return Err("current-law source-custody publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "current-law source-custody public claim {field} must be false"
            ));
        }
    }

    let boundary = string_field(&preflight, "non_claim_boundary")?;
    for required in [
        "current-law source-custody preflight",
        "not source custody",
        "not current-law path values",
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
                "current-law source-custody boundary missing {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH,
        "does not publish source custody",
        "does not publish current-law path values",
        "No external request was submitted",
        "No source values were captured",
        "raw bytes",
        "metadata",
        "retrieval date",
        "byte count",
        "SHA-256",
        "review",
        "full 17-row FY2025 ledger",
        "baseline plus ten-year unified horizon",
        "OASDI annual fund path",
        "Medicare HI annual fund path",
        "transportation trust-fund annual values",
        "general fund annual path",
        "health fiscal current-law path",
        "net interest current-law path",
        "not source custody",
        "not current-law path values",
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
            return Err(format!(
                "current-law source-custody reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_source_custody_batch_plan(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_SCHEMA_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law custody batch artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let plan: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&plan, "record_id")? != "current-law-source-custody-batch-plan:v1"
        || string_field(&plan, "record_family")? != "current_law_source_custody_batch_plan"
        || int_field(&plan, "pulse")? != 119
        || string_field(&plan, "post_rollup_readiness_work_queue_path")?
            != POST_ROLLUP_READINESS_WORK_QUEUE_JSON_PATH
        || string_field(&plan, "current_law_source_custody_preflight_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH
        || string_field(&plan, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
    {
        return Err("current-law custody batch plan identity failed".to_string());
    }

    let rules = plan
        .get("batch_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody batch rules")?;
    for required in [
        "no_external_request_submitted",
        "no_agency_or_person_contacted",
        "no_source_values_captured",
        "official_sources_only_for_future_capture",
        "raw_bytes_metadata_retrieval_date_byte_count_and_sha256_required_before_values",
        "review_required_before_values",
        "current_law_zero_reform_delta_required",
        "no_interpolation_without_explicit_model",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("current-law custody batch rule failed: {required}"));
        }
    }

    let expected_paths = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "general_fund_path",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "health_fiscal_current_law_path",
        "net_interest_current_law_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let covered = plan
        .get("covered_path_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law custody covered paths")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("current-law custody path string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if covered != expected_paths {
        return Err("current-law custody covered path set failed".to_string());
    }

    let batches = plan
        .get("custody_batches")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law custody batches")?;
    if batches.len() != 4 {
        return Err("current-law custody plan must contain four batches".to_string());
    }
    let mut observed_paths = BTreeSet::new();
    for (index, batch) in batches.iter().enumerate() {
        if int_field(batch, "rank")? != (index as i64) + 1
            || string_field(batch, "batch_id")?.is_empty()
            || string_field(batch, "capture_status")? != "not_started"
            || !batch
                .get("source_ids")
                .is_some_and(serde_json::Value::is_null)
            || !batch
                .get("raw_artifact_paths")
                .is_some_and(serde_json::Value::is_null)
            || !batch
                .get("metadata_paths")
                .is_some_and(serde_json::Value::is_null)
            || !batch.get("values").is_some_and(serde_json::Value::is_null)
            || batch
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || batch
                .get("values_may_be_populated")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!(
                "current-law custody batch failed at rank {}",
                index + 1
            ));
        }
        let path_ids = batch
            .get("path_ids")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law custody batch path ids")?;
        if path_ids.is_empty() {
            return Err("current-law custody batch path ids empty".to_string());
        }
        for id in path_ids {
            observed_paths.insert(
                id.as_str()
                    .ok_or("current-law custody batch path id string")?
                    .to_string(),
            );
        }
        let source_families = batch
            .get("future_official_source_families")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law custody source families")?;
        if source_families.is_empty() {
            return Err("current-law custody source families empty".to_string());
        }
    }
    if observed_paths != expected_paths {
        return Err("current-law custody batch path coverage failed".to_string());
    }

    let summary = plan
        .get("aggregate_status")
        .ok_or("current-law custody aggregate status")?;
    if int_field(summary, "batches")? != 4
        || int_field(summary, "path_ids_covered")? != 8
        || int_field(summary, "source_custody_ready_count")? != 0
        || int_field(summary, "values_may_be_populated_count")? != 0
        || summary
            .get("current_law_values_published")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || summary
            .get("solver_inputs_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || summary
            .get("rates_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law custody aggregate status failed".to_string());
    }

    let claims = plan
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("current-law custody claim bool")?;
        if field == "current_law_source_custody_batch_plan_published" {
            if !observed {
                return Err("current-law custody publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "current-law custody public claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_READER_PATH))
        .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH,
        "This is a batch plan for future source-custody work.",
        "It does not capture sources or publish current-law values.",
        "Federal baseline, unified horizon, general fund, and 17-row ledger.",
        "OASDI, Medicare HI, and transportation trust-fund paths.",
        "Health current-law component paths.",
        "Net-interest and debt path custody.",
        "No external request was submitted and no agency or person was contacted.",
        "Future capture must use official sources only",
        "raw bytes, metadata, retrieval date, byte count, SHA-256",
        "Trust funds remain separate.",
        "Medicare HI remains separate.",
        "Current-law paths must have zero reform delta.",
        "No interpolation is allowed without an explicit model.",
        "Missing values remain null and blocked gates remain false.",
        "not source custody",
        "not current-law path values",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!("current-law custody reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_source_custody_packet_template(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_SCHEMA_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law custody packet template artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let template: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&template, "record_id")? != "current-law-source-custody-packet-template:v1"
        || string_field(&template, "record_family")? != "current_law_source_custody_packet_template"
        || int_field(&template, "pulse")? != 120
        || string_field(&template, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&template, "current_law_source_custody_preflight_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH
        || string_field(&template, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
    {
        return Err("current-law custody packet template identity failed".to_string());
    }

    let rules = template
        .get("template_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody packet template rules")?;
    for required in [
        "template_only_no_capture",
        "no_external_request_submitted",
        "no_agency_or_person_contacted",
        "official_sources_only",
        "raw_bytes_required_before_value",
        "metadata_required_before_value",
        "retrieval_date_required_before_value",
        "byte_count_required_before_value",
        "sha256_required_before_value",
        "review_required_before_value",
        "component_mapping_required_before_value",
        "annual_years_covered_required_before_value",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "current-law custody packet template rule failed: {required}"
            ));
        }
    }

    let expected_fields = [
        "packet_id",
        "path_id",
        "batch_id",
        "source_id",
        "official_host_or_publisher",
        "source_vintage",
        "retrieval_date",
        "raw_artifact_path",
        "raw_byte_count",
        "raw_sha256",
        "metadata_path",
        "extraction_method",
        "annual_years_covered",
        "component_mapping",
        "review_status",
        "custody_ready",
        "values_may_be_populated",
        "claim_booleans",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_fields = template
        .get("required_packet_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law custody packet template required fields")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("current-law custody packet template field string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_fields != expected_fields {
        return Err("current-law custody packet template required field set failed".to_string());
    }

    let packet = template
        .get("packet_template")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody packet template object")?;
    for field in [
        "packet_id",
        "path_id",
        "batch_id",
        "source_id",
        "official_host_or_publisher",
        "source_vintage",
        "retrieval_date",
        "raw_artifact_path",
        "raw_byte_count",
        "raw_sha256",
        "metadata_path",
        "extraction_method",
        "annual_years_covered",
        "component_mapping",
        "review_status",
    ] {
        if !packet.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "current-law custody packet template field {field} must be null"
            ));
        }
    }
    if packet
        .get("custody_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
        || packet
            .get("values_may_be_populated")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law custody packet template gates must be false".to_string());
    }
    let packet_claims = packet
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody packet template packet claims")?;
    for (field, value) in packet_claims {
        if value.as_bool() != Some(false) {
            return Err(format!(
                "current-law custody packet template packet claim {field} must be false"
            ));
        }
    }

    let expected_checks = [
        "official_source",
        "raw_artifact_path_exists",
        "raw_byte_count_matches_file",
        "raw_sha256_matches_file",
        "metadata_path_exists",
        "retrieval_date_present",
        "source_vintage_present",
        "extraction_method_present",
        "annual_years_covered_match_required_horizon_or_declared_scope",
        "component_mapping_reviewed",
        "review_status_passed",
        "no_values_populated_before_all_checks_pass",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let checks = template
        .get("readiness_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law custody packet template readiness checks")?;
    if checks.len() != expected_checks.len() {
        return Err("current-law custody packet template readiness count failed".to_string());
    }
    let mut observed_checks = BTreeSet::new();
    for check in checks {
        observed_checks.insert(string_field(check, "check_id")?.to_string());
        if check.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || check.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !check.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("current-law custody packet template readiness gate failed".to_string());
        }
    }
    if observed_checks != expected_checks {
        return Err("current-law custody packet template readiness check set failed".to_string());
    }

    let blocked = template
        .get("blocked_output_fields")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody packet template blocked outputs")?;
    let expected_blocked = [
        "captured_packets",
        "current_law_path_values",
        "solver_inputs",
        "policy_deltas",
        "target_costs",
        "rates",
        "public_rate_cards",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if blocked.keys().cloned().collect::<BTreeSet<_>>() != expected_blocked {
        return Err("current-law custody packet template blocked output set failed".to_string());
    }
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "current-law custody packet template blocked output {field} must be null"
            ));
        }
    }

    let claims = template
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law custody packet template claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("current-law custody packet template claim bool")?;
        if field == "current_law_source_custody_packet_template_published" {
            if !observed {
                return Err(
                    "current-law custody packet template publish flag must be true".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "current-law custody packet template public claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_READER_PATH))
            .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH,
        "This is a template for future source-custody packets.",
        "It captures no source and publishes no current-law value.",
        "Future custody packets must identify the path, batch, source ID, official host or publisher, source vintage, retrieval date, raw artifact path, raw byte count, raw SHA-256, metadata path, extraction method, annual years covered, component mapping, review status, custody readiness, and whether values may be populated.",
        "Before any values can be populated",
        "raw artifact must exist",
        "byte count must match",
        "SHA-256 must match",
        "metadata must exist",
        "retrieval date and source vintage must be present",
        "extraction method must be present",
        "annual coverage and component mapping must be reviewed",
        "review status must pass",
        "No external request was submitted and no agency or person was contacted.",
        "Missing values remain null and blocked gates remain false.",
        "not source custody",
        "not current-law path values",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!(
                "current-law custody packet template reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_fy2025_17_row_ledger_custody(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH,
        CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_SCHEMA_PATH,
        CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing FY2025 ledger custody artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let custody: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&custody, "record_id")? != "current-law-fy2025-17-row-ledger-custody:v1"
        || string_field(&custody, "record_family")? != "current_law_fy2025_17_row_ledger_custody"
        || int_field(&custody, "pulse")? != 121
        || string_field(&custody, "current_law_source_custody_packet_template_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH
        || string_field(&custody, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&custody, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&custody, "path_id")? != "full_17_row_fy2025_ledger"
        || string_field(&custody, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
        || int_field(&custody, "fiscal_year")? != 2025
        || string_field(&custody, "unit")? != "millions_of_dollars"
    {
        return Err("FY2025 ledger custody identity failed".to_string());
    }

    let status = custody
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 ledger source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "values_may_be_populated_for_baseline_year_only",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("FY2025 ledger custody status {field} must be true"));
        }
    }
    for field in [
        "baseline_plus_ten_year_horizon_ready",
        "general_fund_path_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "FY2025 ledger custody status {field} must be false"
            ));
        }
    }

    let source_packets = custody
        .get("source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("FY2025 ledger source packets")?;
    if source_packets.len() != 2 {
        return Err("FY2025 ledger custody must contain two source packets".to_string());
    }
    let mut source_ids = BTreeSet::new();
    for packet in source_packets {
        if string_field(packet, "path_id")? != "full_17_row_fy2025_ledger"
            || string_field(packet, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
            || string_field(packet, "official_host_or_publisher")?
                != "Office of Management and Budget"
            || string_field(packet, "retrieval_date")? != "2026-06-21"
            || string_field(packet, "review_status")? != "source_metadata_present_and_hash_matched"
            || packet
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || packet
                .get("values_may_be_populated")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("FY2025 ledger source packet identity/gates failed".to_string());
        }
        let years = packet
            .get("annual_years_covered")
            .and_then(serde_json::Value::as_array)
            .ok_or("FY2025 ledger annual years")?;
        if years.len() != 1 || years[0].as_i64() != Some(2025) {
            return Err("FY2025 ledger source packet must cover FY2025 only".to_string());
        }
        let raw_path = string_field(packet, "raw_artifact_path")?;
        let metadata_path = string_field(packet, "metadata_path")?;
        let raw_file = root.join(&raw_path);
        let metadata_file = root.join(&metadata_path);
        if !raw_file.exists() || !metadata_file.exists() {
            return Err(format!(
                "FY2025 ledger source packet missing file for {raw_path}"
            ));
        }
        if fs::metadata(&raw_file).map_err(|e| e.to_string())?.len() as i64
            != int_field(packet, "raw_byte_count")?
        {
            return Err(format!("FY2025 ledger byte count failed for {raw_path}"));
        }
        if sha256_file(&raw_file)? != string_field(packet, "raw_sha256")? {
            return Err(format!("FY2025 ledger raw hash failed for {raw_path}"));
        }
        if sha256_file(&metadata_file)? != string_field(packet, "metadata_sha256")? {
            return Err(format!(
                "FY2025 ledger metadata hash failed for {metadata_path}"
            ));
        }
        source_ids.insert(string_field(packet, "source_id")?.to_string());

        let claims = packet
            .get("claim_booleans")
            .and_then(serde_json::Value::as_object)
            .ok_or("FY2025 ledger packet claims")?;
        for (field, value) in claims {
            let observed = value.as_bool().ok_or("FY2025 ledger packet claim bool")?;
            if matches!(
                field.as_str(),
                "source_custody_packet_published"
                    | "source_custody_ready"
                    | "current_law_path_values_published"
            ) {
                if !observed {
                    return Err(format!("FY2025 ledger packet claim {field} must be true"));
                }
            } else if observed {
                return Err(format!("FY2025 ledger packet claim {field} must be false"));
            }
        }
    }
    let expected_sources = ["SRC-OMB-HIST-1-1-FY2027", "SRC-OMB-HIST-3-2-FY2027"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if source_ids != expected_sources {
        return Err("FY2025 ledger source IDs failed".to_string());
    }

    let lineage = custody
        .get("source_value_lineage")
        .ok_or("FY2025 ledger value lineage")?;
    let value_artifact_path = string_field(lineage, "existing_local_value_artifact_path")?;
    let value_artifact = root.join(value_artifact_path);
    if !value_artifact.exists()
        || sha256_file(&value_artifact)?
            != string_field(lineage, "existing_local_value_artifact_sha256")?
        || int_field(lineage, "row_count")? != 17
    {
        return Err("FY2025 ledger value lineage failed".to_string());
    }

    let rows = custody
        .get("ledger_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("FY2025 ledger rows")?;
    if rows.len() != 17 {
        return Err("FY2025 ledger must contain 17 rows".to_string());
    }
    let mut row_ids = BTreeSet::new();
    let mut sum = 0i64;
    let mut positive = 0usize;
    let mut negative = 0usize;
    for row in rows {
        let row_id = string_field(row, "row_id")?.to_string();
        if !row_ids.insert(row_id.clone()) {
            return Err(format!("FY2025 ledger duplicate row {row_id}"));
        }
        let amount = int_field(row, "current_law_outlays_musd")?;
        sum += amount;
        if amount >= 0 {
            positive += 1;
        } else {
            negative += 1;
        }
        if int_field(row, "current_law_zero_reform_delta_musd")? != 0 {
            return Err(format!(
                "FY2025 ledger row {row_id} reform delta must be zero"
            ));
        }
    }
    for required in [
        "commerce-housing-credit",
        "undistributed-offsetting-receipts",
        "net-interest",
    ] {
        if !row_ids.contains(required) {
            return Err(format!("FY2025 ledger missing required row {required}"));
        }
    }
    if sum != 7_011_105 || positive != 15 || negative != 2 {
        return Err("FY2025 ledger row sum or sign counts failed".to_string());
    }

    let reconciliation = custody
        .get("reconciliation")
        .ok_or("FY2025 reconciliation")?;
    if int_field(reconciliation, "row_count")? != 17
        || int_field(reconciliation, "positive_row_count")? != 15
        || int_field(reconciliation, "negative_offset_row_count")? != 2
        || int_field(reconciliation, "sum_current_law_outlays_musd")? != sum
        || int_field(reconciliation, "required_total_outlays_musd")? != 7_011_105
        || int_field(reconciliation, "rounding_residual_musd")? != 0
        || int_field(reconciliation, "total_receipts_musd")? != 5_236_421
        || int_field(reconciliation, "deficit_musd")? != 1_774_684
        || int_field(reconciliation, "net_interest_musd")? != 970_065
        || reconciliation
            .get("commerce_housing_credit_kept_in_fiscal_reconciliation")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || reconciliation
            .get("undistributed_offsetting_receipts_kept_in_fiscal_reconciliation")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || reconciliation
            .get("net_interest_direct_cut_allowed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("FY2025 ledger reconciliation failed".to_string());
    }

    let blocked = custody
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 ledger blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("FY2025 ledger blocked output {field} must be null"));
        }
    }

    let claims = custody
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 ledger claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("FY2025 ledger claim bool")?;
        if matches!(
            field.as_str(),
            "source_custody_packet_published"
                | "source_custody_ready"
                | "current_law_path_values_published"
        ) {
            if !observed {
                return Err(format!("FY2025 ledger claim {field} must be true"));
            }
        } else if observed {
            return Err(format!("FY2025 ledger claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_READER_PATH))
            .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH,
        "FY2025 current-law 17-row federal ledger reconciles to `$7,011.105B`",
        "No external request was submitted and no agency or person was contacted",
        "fifteen positive budget rows plus two negative fiscal reconciliation rows",
        "Commerce/housing credit and undistributed offsetting receipts remain in fiscal reconciliation.",
        "Net interest remains visible and cannot be cut directly.",
        "not a ten-year baseline path",
        "not general-fund path values",
        "not trust-fund path values",
        "not health component path values",
        "not net-interest/debt path values",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!("FY2025 ledger reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_17_row_pbd_fy2025_2031_context_path(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH,
        CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing 17-row PBD context artifact: {path}"));
        }
    }

    let text = fs::read_to_string(
        root.join(CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH),
    )
    .map_err(|err| {
        format!("failed to read {CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH}: {err}")
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "current-law-17-row-pbd-fy2025-2031-context-path:v1"
        || string_field(&record, "record_family")? != "current_law_17_row_context_path"
        || string_field(&record, "status")? != "draft_pbd_17_row_context_fy2032_2035_blocked"
        || string_field(&record, "path_id")? != "full_17_row_fy2025_ledger"
        || string_field(&record, "source_id")? != "SRC-OMB-PBD-OUTLAYS-FY2027"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-PBD-OUTLAYS-FY2027/2026-07-13/outlays_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 2_144_756
        || string_field(&record, "raw_sha256")?
            != "d892f2247e6c1aed68414d3e4168f8b4ab97bcfc7acf82a6a449a3fcb1addb07"
        || string_field(&record, "unit")? != "millions_of_dollars"
        || string_field(&record, "source_unit")? != "thousands_of_dollars"
        || int_field(&record, "source_row_count")? != 5_760
    {
        return Err("17-row PBD context identity failed".to_string());
    }

    let covered = record
        .get("covered_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("17-row PBD covered years")?
        .iter()
        .map(|value| value.as_i64().ok_or("17-row PBD covered year int"))
        .collect::<Result<Vec<_>, _>>()?;
    if covered != (2025..=2031).map(i64::from).collect::<Vec<_>>() {
        return Err("17-row PBD covered years failed".to_string());
    }
    let missing = record
        .get("missing_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("17-row PBD missing years")?
        .iter()
        .map(|value| value.as_i64().ok_or("17-row PBD missing year int"))
        .collect::<Result<Vec<_>, _>>()?;
    if missing != (2032..=2035).map(i64::from).collect::<Vec<_>>() {
        return Err("17-row PBD missing years failed".to_string());
    }

    let rows = record
        .get("ledger_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("17-row PBD ledger rows")?;
    if rows.len() != 17 {
        return Err("17-row PBD context must contain 17 ledger rows".to_string());
    }

    let expected_ids = [
        "social-security",
        "medicare",
        "health",
        "net-interest",
        "national-defense",
        "income-security",
        "veterans",
        "transportation",
        "justice-general-government",
        "environment-energy-natural-resources",
        "community-regional-development",
        "education-training-employment-social-services",
        "agriculture",
        "international-affairs",
        "science-space-technology",
        "commerce-housing-credit",
        "undistributed-offsetting-receipts",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_ids = BTreeSet::new();
    let mut annual_sums = BTreeMap::<i64, i64>::new();
    for row in rows {
        let row_id = string_field(row, "row_id")?;
        if !observed_ids.insert(row_id.clone()) {
            return Err(format!("17-row PBD duplicate row {row_id}"));
        }
        let values = row
            .get("annual_outlays_musd")
            .and_then(serde_json::Value::as_object)
            .ok_or("17-row PBD annual values")?;
        for year in 2025..=2031 {
            let key = year.to_string();
            let amount = values
                .get(&key)
                .and_then(serde_json::Value::as_i64)
                .ok_or("17-row PBD annual amount")?;
            *annual_sums.entry(i64::from(year)).or_default() += amount;
        }
    }
    if observed_ids != expected_ids {
        return Err("17-row PBD row ID set failed".to_string());
    }

    let expected_totals = [
        (2025, 7_011_105),
        (2026, 7_540_434),
        (2027, 8_092_860),
        (2028, 8_445_361),
        (2029, 8_653_223),
        (2030, 8_996_290),
        (2031, 9_279_779),
    ]
    .into_iter()
    .map(|(year, value)| (i64::from(year), value))
    .collect::<BTreeMap<_, _>>();
    if annual_sums != expected_totals {
        return Err("17-row PBD annual sums failed".to_string());
    }

    let reconciliation = record
        .get("annual_reconciliation")
        .and_then(serde_json::Value::as_array)
        .ok_or("17-row PBD annual reconciliation")?;
    if reconciliation.len() != 7 {
        return Err("17-row PBD reconciliation row count failed".to_string());
    }
    for row in reconciliation {
        let year = int_field(row, "fiscal_year")?;
        let expected = *expected_totals
            .get(&year)
            .ok_or("17-row PBD unexpected reconciliation year")?;
        if int_field(row, "sum_17_rows_musd")? != expected
            || int_field(row, "baseline_total_outlays_musd")? != expected
            || int_field(row, "difference_musd")? != 0
        {
            return Err(format!("17-row PBD reconciliation failed for {year}"));
        }
    }

    let missing_rows = record
        .get("missing_year_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("17-row PBD missing year rows")?;
    if missing_rows.len() != 4 {
        return Err("17-row PBD missing row count failed".to_string());
    }
    for row in missing_rows {
        let year = int_field(row, "fiscal_year")?;
        if !(2032..=2035).contains(&year)
            || !row
                .get("ledger_rows")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("17-row PBD missing row failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("17-row PBD blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("17-row PBD blocked output {field} must be null"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("17-row PBD claims")?;
    for field in [
        "current_law_17_row_pbd_fy2025_2031_context_path_published",
        "official_fy2025_fy2031_rows_present",
        "source_custody_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("17-row PBD claim {field} must be true"));
        }
    }
    for field in [
        "full_fy2025_fy2035_17_row_path_ready",
        "fund_split_ready",
        "solver_input_ready",
        "solver_run_published",
        "target_cost_published",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("17-row PBD claim {field} must be false"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "FY2025-FY2031 17-row current-law outlay context only",
        "not a full FY2025-FY2035 path",
        "not a fund split",
        "not a policy scenario",
        "not solver input",
        "not a rate calculation",
        "not a savings estimate",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!("17-row PBD warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_READER_PATH}: {err}"
        )
    })?;
    for required in [
        "FY2025-FY2031",
        "17-row current-law outlay context path",
        "annual reconciliation",
        "FY2032-FY2035 rows",
        "solver input",
        "rate calculation",
        "savings estimate",
        "balanced-budget claim",
        "null, not zero",
    ] {
        if !reader.contains(required) {
            return Err(format!("17-row PBD reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_baseline_annual_path_partial(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_JSON_PATH,
        CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_SCHEMA_PATH,
        CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law baseline annual path artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let path: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&path, "record_id")? != "current-law-baseline-annual-path-partial:v1"
        || string_field(&path, "record_family")? != "current_law_baseline_annual_path_partial"
        || int_field(&path, "pulse")? != 122
        || string_field(&path, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&path, "current_law_source_custody_packet_template_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH
        || string_field(&path, "fy2025_17_row_ledger_custody_path")?
            != CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH
        || string_field(&path, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&path, "path_id")? != "baseline_plus_ten_year_horizon"
        || string_field(&path, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
        || int_field(&path, "baseline_year")? != 2025
        || string_field(&path, "unit")? != "millions_of_dollars"
        || path
            .get("interpolation_used")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law baseline annual path identity failed".to_string());
    }

    let expected_required = (2025..=2035).collect::<Vec<_>>();
    let required_years = path
        .get("required_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law baseline required years")?
        .iter()
        .map(|value| value.as_i64().ok_or("required year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if required_years != expected_required {
        return Err("current-law baseline required years failed".to_string());
    }
    let populated_years = path
        .get("populated_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law baseline populated years")?
        .iter()
        .map(|value| value.as_i64().ok_or("populated year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if populated_years != (2025..=2031).collect::<Vec<_>>() {
        return Err("current-law baseline populated years failed".to_string());
    }
    let missing_years = path
        .get("missing_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law baseline missing years")?
        .iter()
        .map(|value| value.as_i64().ok_or("missing year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if missing_years != (2032..=2035).collect::<Vec<_>>() {
        return Err("current-law baseline missing years failed".to_string());
    }

    let status = path
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law baseline source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "outlay_values_may_be_populated_for_2025_2031",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("current-law baseline status {field} must be true"));
        }
    }
    for field in [
        "receipts_and_deficit_values_may_be_populated_after_2025",
        "complete_required_horizon_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("current-law baseline status {field} must be false"));
        }
    }

    let packets = path
        .get("source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law baseline source packets")?;
    if packets.len() != 2 {
        return Err("current-law baseline needs two source packets".to_string());
    }
    let mut packet_sources = BTreeSet::new();
    for packet in packets {
        if string_field(packet, "path_id")? != "baseline_plus_ten_year_horizon"
            || string_field(packet, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
            || string_field(packet, "official_host_or_publisher")?
                != "Office of Management and Budget"
            || string_field(packet, "review_status")? != "source_metadata_present_and_hash_matched"
            || packet
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || packet
                .get("values_may_be_populated")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("current-law baseline source packet identity failed".to_string());
        }
        let raw_path = string_field(packet, "raw_artifact_path")?;
        let metadata_path = string_field(packet, "metadata_path")?;
        let raw_file = root.join(&raw_path);
        let metadata_file = root.join(&metadata_path);
        if !raw_file.exists() || !metadata_file.exists() {
            return Err(format!(
                "current-law baseline missing source file {raw_path}"
            ));
        }
        if fs::metadata(&raw_file).map_err(|e| e.to_string())?.len() as i64
            != int_field(packet, "raw_byte_count")?
        {
            return Err(format!(
                "current-law baseline raw byte count failed for {raw_path}"
            ));
        }
        if sha256_file(&raw_file)? != string_field(packet, "raw_sha256")? {
            return Err(format!(
                "current-law baseline raw hash failed for {raw_path}"
            ));
        }
        if sha256_file(&metadata_file)? != string_field(packet, "metadata_sha256")? {
            return Err(format!(
                "current-law baseline metadata hash failed for {metadata_path}"
            ));
        }
        packet_sources.insert(string_field(packet, "source_id")?.to_string());
    }
    let expected_sources = ["SRC-OMB-HIST-1-1-FY2027", "SRC-OMB-PBD-OUTLAYS-FY2027"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if packet_sources != expected_sources {
        return Err("current-law baseline source set failed".to_string());
    }

    let rows = path
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law baseline annual rows")?;
    if rows.len() != 11 {
        return Err("current-law baseline must contain 11 annual rows".to_string());
    }
    let expected_outlays = [
        (2025, 7_011_105),
        (2026, 7_540_434),
        (2027, 8_092_860),
        (2028, 8_445_361),
        (2029, 8_653_223),
        (2030, 8_996_290),
        (2031, 9_279_779),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let mut seen_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        seen_years.insert(year);
        if (2025..=2031).contains(&year) {
            if int_field(row, "total_outlays_musd")? != expected_outlays[&year] {
                return Err(format!(
                    "current-law baseline outlay value failed for FY{year}"
                ));
            }
            if year == 2025 {
                if int_field(row, "total_receipts_musd")? != 5_236_421
                    || int_field(row, "deficit_musd")? != 1_774_684
                    || string_field(row, "actual_or_projection")? != "actual"
                {
                    return Err("current-law baseline FY2025 spine failed".to_string());
                }
            } else if !row
                .get("total_receipts_musd")
                .is_some_and(serde_json::Value::is_null)
                || !row
                    .get("deficit_musd")
                    .is_some_and(serde_json::Value::is_null)
                || string_field(row, "actual_or_projection")? != "projection"
            {
                return Err(format!(
                    "current-law baseline forward receipts/deficit gates failed for FY{year}"
                ));
            }
        } else if (2032..=2035).contains(&year) {
            for field in [
                "actual_or_projection",
                "total_outlays_musd",
                "total_receipts_musd",
                "deficit_musd",
                "outlay_source_units",
            ] {
                if !row.get(field).is_some_and(serde_json::Value::is_null) {
                    return Err(format!(
                        "current-law baseline missing-year field {field} must be null for FY{year}"
                    ));
                }
            }
            let source_ids = row
                .get("source_ids")
                .and_then(serde_json::Value::as_array)
                .ok_or("current-law baseline missing-year source ids")?;
            if !source_ids.is_empty()
                || string_field(row, "missing_reason")?
                    != "no local official captured annual value; interpolation prohibited"
            {
                return Err(format!(
                    "current-law baseline missing-year gate failed for FY{year}"
                ));
            }
        } else {
            return Err(format!("current-law baseline unexpected year FY{year}"));
        }
    }
    if seen_years != (2025..=2035).collect::<BTreeSet<_>>() {
        return Err("current-law baseline annual year set failed".to_string());
    }

    let reconciliation = path
        .get("reconciliation")
        .ok_or("current-law baseline reconciliation")?;
    if int_field(reconciliation, "fy2025_pbd_outlays_musd")? != 7_011_105
        || int_field(reconciliation, "fy2025_17_row_ledger_outlays_musd")? != 7_011_105
        || int_field(reconciliation, "fy2025_difference_musd")? != 0
        || int_field(reconciliation, "populated_outlay_year_count")? != 7
        || int_field(reconciliation, "required_year_count")? != 11
        || int_field(reconciliation, "missing_year_count")? != 4
        || reconciliation
            .get("complete_horizon_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || reconciliation
            .get("solver_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law baseline reconciliation failed".to_string());
    }

    let blocked = path
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law baseline blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "current-law baseline blocked output {field} must be null"
            ));
        }
    }

    let claims = path
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law baseline claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("current-law baseline claim bool")?;
        if matches!(
            field.as_str(),
            "source_custody_packet_published"
                | "source_custody_ready"
                | "partial_current_law_outlay_path_published"
        ) {
            if !observed {
                return Err(format!("current-law baseline claim {field} must be true"));
            }
        } else if observed {
            return Err(format!("current-law baseline claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_READER_PATH))
            .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_JSON_PATH,
        "partial official current-law annual outlay path for FY2025 through FY2031",
        "FY2025 outlays reconcile to `$7,011.105B`",
        "No external request was submitted and no agency or person was contacted",
        "FY2032 through FY2035 remain null",
        "FY2026 through FY2035 receipts and deficits remain null",
        "Missing values remain null and interpolation is prohibited.",
        "not a complete ten-year baseline path",
        "not general-fund path values",
        "not trust-fund path values",
        "not health component path values",
        "not net-interest/debt path values",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!(
                "current-law baseline annual path reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_baseline_receipts_deficit_path_partial(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_JSON_PATH,
        CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_SCHEMA_PATH,
        CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law receipts/deficit path artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let path: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&path, "record_id")? != "current-law-baseline-receipts-deficit-path-partial:v1"
        || string_field(&path, "record_family")?
            != "current_law_baseline_receipts_deficit_path_partial"
        || int_field(&path, "pulse")? != 123
        || string_field(&path, "current_law_baseline_annual_path_partial_path")?
            != CURRENT_LAW_BASELINE_ANNUAL_PATH_PARTIAL_JSON_PATH
        || string_field(&path, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&path, "current_law_source_custody_packet_template_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH
        || string_field(&path, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&path, "path_id")? != "baseline_plus_ten_year_horizon"
        || string_field(&path, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
        || int_field(&path, "baseline_year")? != 2025
        || string_field(&path, "unit")? != "millions_of_dollars"
        || path
            .get("interpolation_used")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law receipts/deficit path identity failed".to_string());
    }

    let required_years = path
        .get("required_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law receipts/deficit required years")?
        .iter()
        .map(|value| value.as_i64().ok_or("required year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if required_years != (2025..=2035).collect::<Vec<_>>() {
        return Err("current-law receipts/deficit required years failed".to_string());
    }
    let populated_years = path
        .get("populated_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law receipts/deficit populated years")?
        .iter()
        .map(|value| value.as_i64().ok_or("populated year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if populated_years != (2025..=2031).collect::<Vec<_>>() {
        return Err("current-law receipts/deficit populated years failed".to_string());
    }
    let missing_years = path
        .get("missing_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law receipts/deficit missing years")?
        .iter()
        .map(|value| value.as_i64().ok_or("missing year integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if missing_years != (2032..=2035).collect::<Vec<_>>() {
        return Err("current-law receipts/deficit missing years failed".to_string());
    }

    let status = path
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law receipts/deficit source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "outlay_receipt_and_deficit_values_may_be_populated_for_2025_2031",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "current-law receipts/deficit status {field} must be true"
            ));
        }
    }
    for field in [
        "complete_required_horizon_ready",
        "fund_paths_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "current-law receipts/deficit status {field} must be false"
            ));
        }
    }

    let packets = path
        .get("source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law receipts/deficit source packets")?;
    if packets.len() != 2 {
        return Err("current-law receipts/deficit needs two source packets".to_string());
    }
    let mut packet_sources = BTreeSet::new();
    for packet in packets {
        if string_field(packet, "path_id")? != "baseline_plus_ten_year_horizon"
            || string_field(packet, "batch_id")? != "batch_1_federal_baseline_and_17_row_ledger"
            || packet
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || packet
                .get("values_may_be_populated")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("current-law receipts/deficit source packet gates failed".to_string());
        }
        let raw_path = string_field(packet, "raw_artifact_path")?;
        let raw_file = root.join(&raw_path);
        if !raw_file.exists() {
            return Err(format!(
                "current-law receipts/deficit missing file {raw_path}"
            ));
        }
        if fs::metadata(&raw_file).map_err(|e| e.to_string())?.len() as i64
            != int_field(packet, "raw_byte_count")?
        {
            return Err(format!(
                "current-law receipts/deficit byte count failed for {raw_path}"
            ));
        }
        if sha256_file(&raw_file)? != string_field(packet, "raw_sha256")? {
            return Err(format!(
                "current-law receipts/deficit raw hash failed for {raw_path}"
            ));
        }
        if let Some(metadata_path) = packet
            .get("metadata_path")
            .and_then(serde_json::Value::as_str)
        {
            let metadata_file = root.join(metadata_path);
            if !metadata_file.exists()
                || sha256_file(&metadata_file)? != string_field(packet, "metadata_sha256")?
            {
                return Err(format!(
                    "current-law receipts/deficit metadata hash failed for {metadata_path}"
                ));
            }
        } else if !packet
            .get("metadata_sha256")
            .is_some_and(serde_json::Value::is_null)
        {
            return Err("current-law receipts/deficit null metadata hash gate failed".to_string());
        }
        packet_sources.insert(string_field(packet, "source_id")?.to_string());
    }
    let expected_sources = [
        "SRC-OMB-HIST-2-1-FY2027",
        "current-law-baseline-annual-path-partial:v1",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if packet_sources != expected_sources {
        return Err("current-law receipts/deficit source set failed".to_string());
    }

    let rows = path
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law receipts/deficit annual rows")?;
    if rows.len() != 11 {
        return Err("current-law receipts/deficit must contain 11 annual rows".to_string());
    }
    let expected = [
        (2025, 7_011_105, 5_236_421, 1_774_684, "actual"),
        (2026, 7_540_434, 5_475_705, 2_064_729, "estimate"),
        (2027, 8_092_860, 5_920_951, 2_171_909, "estimate"),
        (2028, 8_445_361, 6_288_407, 2_156_954, "estimate"),
        (2029, 8_653_223, 6_660_321, 1_992_902, "estimate"),
        (2030, 8_996_290, 7_137_281, 1_859_009, "estimate"),
        (2031, 9_279_779, 7_559_389, 1_720_390, "estimate"),
    ]
    .into_iter()
    .map(|(year, outlays, receipts, deficit, status)| (year, (outlays, receipts, deficit, status)))
    .collect::<BTreeMap<_, _>>();
    let mut seen_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")?;
        seen_years.insert(year);
        if let Some((outlays, receipts, deficit, status)) = expected.get(&year) {
            if int_field(row, "total_outlays_musd")? != *outlays
                || int_field(row, "total_receipts_musd")? != *receipts
                || int_field(row, "deficit_musd")? != *deficit
                || *outlays - *receipts != *deficit
                || string_field(row, "actual_or_projection")? != *status
                || string_field(row, "deficit_formula")?
                    != "total_outlays_musd - total_receipts_musd"
                || !row
                    .get("missing_reason")
                    .is_some_and(serde_json::Value::is_null)
            {
                return Err(format!(
                    "current-law receipts/deficit annual value failed for FY{year}"
                ));
            }
        } else if (2032..=2035).contains(&year) {
            for field in [
                "actual_or_projection",
                "total_outlays_musd",
                "total_receipts_musd",
                "deficit_musd",
                "deficit_formula",
            ] {
                if !row.get(field).is_some_and(serde_json::Value::is_null) {
                    return Err(format!(
                        "current-law receipts/deficit missing-year field {field} must be null for FY{year}"
                    ));
                }
            }
            let source_ids = row
                .get("source_ids")
                .and_then(serde_json::Value::as_array)
                .ok_or("current-law receipts/deficit missing-year source ids")?;
            if !source_ids.is_empty()
                || string_field(row, "missing_reason")?
                    != "no local official captured annual value; interpolation prohibited"
            {
                return Err(format!(
                    "current-law receipts/deficit missing-year gate failed for FY{year}"
                ));
            }
        } else {
            return Err(format!(
                "current-law receipts/deficit unexpected year FY{year}"
            ));
        }
    }
    if seen_years != (2025..=2035).collect::<BTreeSet<_>>() {
        return Err("current-law receipts/deficit annual year set failed".to_string());
    }

    let reconciliation = path
        .get("reconciliation")
        .ok_or("current-law receipts/deficit reconciliation")?;
    if int_field(reconciliation, "fy2025_total_receipts_musd")? != 5_236_421
        || int_field(reconciliation, "fy2025_total_outlays_musd")? != 7_011_105
        || int_field(reconciliation, "fy2025_deficit_musd")? != 1_774_684
        || int_field(reconciliation, "populated_receipt_deficit_year_count")? != 7
        || int_field(reconciliation, "required_year_count")? != 11
        || int_field(reconciliation, "missing_year_count")? != 4
        || reconciliation
            .get("complete_horizon_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || reconciliation
            .get("fund_split_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || reconciliation
            .get("solver_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law receipts/deficit reconciliation failed".to_string());
    }

    let blocked = path
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law receipts/deficit blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "current-law receipts/deficit blocked output {field} must be null"
            ));
        }
    }

    let claims = path
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law receipts/deficit claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("current-law receipts/deficit claim bool")?;
        if matches!(
            field.as_str(),
            "source_custody_packet_published"
                | "source_custody_ready"
                | "partial_current_law_receipts_deficit_path_published"
        ) {
            if !observed {
                return Err(format!(
                    "current-law receipts/deficit claim {field} must be true"
                ));
            }
        } else if observed {
            return Err(format!(
                "current-law receipts/deficit claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_JSON_PATH,
        "partial official current-law receipts and deficit path for FY2025 through FY2031",
        "deficit is calculated as total outlays minus total receipts",
        "No external request was submitted and no agency or person was contacted",
        "FY2032 through FY2035 remain null",
        "Missing values remain null and interpolation is prohibited.",
        "not a complete ten-year baseline path",
        "not a fund split",
        "not general-fund path values",
        "not trust-fund path values",
        "not health component path values",
        "not net-interest/debt path values",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!(
                "current-law receipts/deficit reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_fy2025_fund_group_path(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH,
        CURRENT_LAW_FY2025_FUND_GROUP_PATH_SCHEMA_PATH,
        CURRENT_LAW_FY2025_FUND_GROUP_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing FY2025 fund-group artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let path: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&path, "record_id")? != "current-law-fy2025-fund-group-path:v1"
        || string_field(&path, "record_family")? != "current_law_fy2025_fund_group_path"
        || int_field(&path, "pulse")? != 124
        || string_field(
            &path,
            "current_law_baseline_receipts_deficit_path_partial_path",
        )? != CURRENT_LAW_BASELINE_RECEIPTS_DEFICIT_PATH_PARTIAL_JSON_PATH
        || string_field(&path, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&path, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&path, "current_law_source_custody_packet_template_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PACKET_TEMPLATE_JSON_PATH
        || int_field(&path, "fiscal_year")? != 2025
        || string_field(&path, "year_basis")? != "fiscal_year"
        || string_field(&path, "unit")? != "millions_of_dollars"
    {
        return Err("FY2025 fund-group identity failed".to_string());
    }

    let expected_paths = [
        "general_fund_path",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_paths = path
        .get("path_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("FY2025 fund-group path ids")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("FY2025 fund-group path id string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_paths != expected_paths {
        return Err("FY2025 fund-group path id set failed".to_string());
    }

    let status = path
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 fund-group custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "fy2025_fund_group_values_may_be_populated",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("FY2025 fund-group status {field} must be true"));
        }
    }
    for field in [
        "general_fund_specific_values_ready",
        "named_trust_fund_values_ready",
        "forward_annual_fund_values_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("FY2025 fund-group status {field} must be false"));
        }
    }

    let packet = path
        .get("source_packet")
        .ok_or("FY2025 fund-group source packet")?;
    if string_field(packet, "source_id")? != "SRC-OMB-HIST-1-4-FY2027"
        || string_field(packet, "official_host_or_publisher")? != "Office of Management and Budget"
        || string_field(packet, "retrieval_date")? != "2026-06-21"
        || string_field(packet, "review_status")? != "source_metadata_present_and_hash_matched"
        || packet
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || packet
            .get("values_may_be_populated")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("FY2025 fund-group source packet identity failed".to_string());
    }
    for (path_field, hash_field, byte_field) in [
        ("raw_artifact_path", "raw_sha256", Some("raw_byte_count")),
        ("metadata_path", "metadata_sha256", None),
        ("extracted_artifact_path", "extracted_artifact_sha256", None),
    ] {
        let artifact_path = string_field(packet, path_field)?;
        let artifact_file = root.join(&artifact_path);
        if !artifact_file.exists() {
            return Err(format!(
                "FY2025 fund-group source file missing: {artifact_path}"
            ));
        }
        if let Some(byte_field) = byte_field {
            if fs::metadata(&artifact_file)
                .map_err(|e| e.to_string())?
                .len() as i64
                != int_field(packet, byte_field)?
            {
                return Err(format!(
                    "FY2025 fund-group byte count failed for {artifact_path}"
                ));
            }
        }
        if sha256_file(&artifact_file)? != string_field(packet, hash_field)? {
            return Err(format!("FY2025 fund-group hash failed for {artifact_path}"));
        }
    }
    let years = packet
        .get("annual_years_covered")
        .and_then(serde_json::Value::as_array)
        .ok_or("FY2025 fund-group annual years")?;
    if years.len() != 1 || years[0].as_i64() != Some(2025) {
        return Err("FY2025 fund-group source packet must cover only FY2025".to_string());
    }

    let rows = path
        .get("fy2025_fund_group_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("FY2025 fund-group rows")?;
    if rows.len() != 4 {
        return Err("FY2025 fund-group must contain four rows".to_string());
    }
    let expected = [
        ("total", 5_236_421, 7_011_105, -1_774_684, Some(1_774_684)),
        (
            "federal-funds",
            3_413_497,
            5_284_502,
            -1_871_005,
            Some(1_871_005),
        ),
        ("trust-funds", 3_009_025, 2_912_704, 96_321, None),
        ("interfund-transactions", -1_186_101, -1_186_101, 0, None),
    ]
    .into_iter()
    .map(|(group, receipts, outlays, surplus, deficit)| {
        (group.to_string(), (receipts, outlays, surplus, deficit))
    })
    .collect::<BTreeMap<_, _>>();
    let mut observed_groups = BTreeSet::new();
    for row in rows {
        let group = string_field(row, "fund_group")?;
        observed_groups.insert(group.clone());
        let (receipts, outlays, surplus, deficit) = expected
            .get(&group)
            .ok_or_else(|| format!("unexpected FY2025 fund group {group}"))?;
        if int_field(row, "receipts_musd")? != *receipts
            || int_field(row, "outlays_musd")? != *outlays
        {
            return Err(format!("FY2025 fund-group value failed for {group}"));
        }
        if group == "interfund-transactions" {
            if !row
                .get("surplus_deficit_musd")
                .is_some_and(serde_json::Value::is_null)
            {
                return Err("interfund surplus/deficit must remain null".to_string());
            }
        } else if int_field(row, "surplus_deficit_musd")? != *surplus {
            return Err(format!("FY2025 fund-group surplus failed for {group}"));
        }
        match deficit {
            Some(deficit) => {
                if int_field(row, "deficit_musd")? != *deficit {
                    return Err(format!("FY2025 fund-group deficit failed for {group}"));
                }
            }
            None => {
                if !row
                    .get("deficit_musd")
                    .is_some_and(serde_json::Value::is_null)
                {
                    return Err(format!(
                        "FY2025 fund-group deficit must be null for {group}"
                    ));
                }
            }
        }
    }
    if observed_groups != expected.keys().cloned().collect::<BTreeSet<_>>() {
        return Err("FY2025 fund-group row set failed".to_string());
    }

    let reconciliation = path
        .get("reconciliation")
        .ok_or("FY2025 fund reconciliation")?;
    if int_field(reconciliation, "total_receipts_musd")? != 5_236_421
        || int_field(reconciliation, "total_outlays_musd")? != 7_011_105
        || int_field(reconciliation, "total_surplus_deficit_musd")? != -1_774_684
        || int_field(reconciliation, "total_deficit_musd")? != 1_774_684
        || int_field(
            reconciliation,
            "federal_funds_receipts_plus_trust_funds_receipts_plus_interfund_receipts_musd",
        )? != 5_236_421
        || int_field(
            reconciliation,
            "federal_funds_outlays_plus_trust_funds_outlays_plus_interfund_outlays_musd",
        )? != 7_011_105
        || int_field(
            reconciliation,
            "federal_funds_surplus_deficit_plus_trust_funds_surplus_deficit_musd",
        )? != -1_774_684
        || reconciliation
            .get("reconciles_to_pulse_123_fy2025")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || reconciliation
            .get("interfund_transactions_preserved")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || reconciliation
            .get("federal_funds_not_general_fund")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || reconciliation
            .get("trust_funds_not_split_by_named_fund")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("FY2025 fund-group reconciliation failed".to_string());
    }

    let blocked = path
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 fund-group blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "FY2025 fund-group blocked output {field} must be null"
            ));
        }
    }

    let claims = path
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("FY2025 fund-group claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("FY2025 fund-group claim bool")?;
        if matches!(
            field.as_str(),
            "source_custody_packet_published"
                | "source_custody_ready"
                | "fy2025_fund_group_values_published"
        ) {
            if !observed {
                return Err(format!("FY2025 fund-group claim {field} must be true"));
            }
        } else if observed {
            return Err(format!("FY2025 fund-group claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(CURRENT_LAW_FY2025_FUND_GROUP_PATH_READER_PATH))
        .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH,
        "FY2025 OMB fund-group actuals",
        "$5,236.421B",
        "$7,011.105B",
        "$1,774.684B",
        "No external request was submitted and no agency or person was contacted",
        "Federal funds are not the same as the general fund.",
        "Trust funds remain separate",
        "does not split OASDI, Medicare HI, or transportation trust funds",
        "Interfund transactions remain explicit",
        "not general-fund path values",
        "not named trust-fund path values",
        "not forward annual fund values",
        "not an explicit interfund transfer schedule",
        "not solver inputs",
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
        if !reader_words.contains(required) {
            return Err(format!("FY2025 fund-group reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_fy2025_dedicated_receipt_anchors(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH,
        CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_SCHEMA_PATH,
        CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing dedicated receipt anchor artifact: {path}"));
        }
    }

    let text =
        fs::read_to_string(root.join(CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let anchors: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&anchors, "record_id")? != "current-law-fy2025-dedicated-receipt-anchors:v1"
        || string_field(&anchors, "record_family")?
            != "current_law_fy2025_dedicated_receipt_anchors"
        || int_field(&anchors, "pulse")? != 125
        || int_field(&anchors, "fiscal_year")? != 2025
        || string_field(&anchors, "year_basis")? != "fiscal_year"
        || string_field(&anchors, "unit")? != "millions_of_dollars"
        || string_field(&anchors, "current_law_fy2025_fund_group_path")?
            != CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH
        || string_field(&anchors, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&anchors, "rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
    {
        return Err("dedicated receipt anchor identity failed".to_string());
    }

    let status = anchors
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("dedicated receipt source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "dedicated_receipt_anchors_may_be_populated",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("dedicated receipt status {field} must be true"));
        }
    }
    for field in [
        "named_trust_fund_outlays_ready",
        "named_trust_fund_balances_ready",
        "explicit_transfer_schedule_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("dedicated receipt status {field} must be false"));
        }
    }

    let packets = anchors
        .get("source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("dedicated receipt source packets")?;
    if packets.len() != 2 {
        return Err("dedicated receipt anchors require two source packets".to_string());
    }
    let mut observed_packet_ids = BTreeSet::new();
    for packet in packets {
        observed_packet_ids.insert(string_field(packet, "source_id")?);
        for (path_field, hash_field, byte_field) in [
            ("raw_artifact_path", "raw_sha256", "raw_byte_count"),
            ("metadata_path", "metadata_sha256", "metadata_byte_count"),
        ] {
            let artifact_path = string_field(packet, path_field)?;
            let artifact_file = root.join(&artifact_path);
            if !artifact_file.exists() {
                return Err(format!(
                    "dedicated receipt source file missing: {artifact_path}"
                ));
            }
            if fs::metadata(&artifact_file)
                .map_err(|e| e.to_string())?
                .len() as i64
                != int_field(packet, byte_field)?
            {
                return Err(format!(
                    "dedicated receipt byte count failed for {artifact_path}"
                ));
            }
            if sha256_file(&artifact_file)? != string_field(packet, hash_field)? {
                return Err(format!("dedicated receipt hash failed for {artifact_path}"));
            }
        }
        if packet.get("extracted_artifact_path").is_some() {
            let artifact_path = string_field(packet, "extracted_artifact_path")?;
            let artifact_file = root.join(&artifact_path);
            if !artifact_file.exists() {
                return Err(format!(
                    "dedicated receipt extracted file missing: {artifact_path}"
                ));
            }
            if fs::metadata(&artifact_file)
                .map_err(|e| e.to_string())?
                .len() as i64
                != int_field(packet, "extracted_artifact_byte_count")?
            {
                return Err(format!(
                    "dedicated receipt extracted byte count failed for {artifact_path}"
                ));
            }
            if sha256_file(&artifact_file)? != string_field(packet, "extracted_artifact_sha256")? {
                return Err(format!(
                    "dedicated receipt extracted hash failed for {artifact_path}"
                ));
            }
        }
        if packet
            .get("values_may_be_populated")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err("dedicated receipt source packet must allow anchor values".to_string());
        }
    }
    let expected_packet_ids = ["SRC-OMB-HIST-2-4-FY2027", "SRC-OMB-AP-13-FUNDS-FY2027"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_packet_ids != expected_packet_ids {
        return Err("dedicated receipt source packet ids failed".to_string());
    }

    let rows = anchors
        .get("receipt_anchor_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("dedicated receipt anchor rows")?;
    let expected_rows = [
        ("oasi_trust_funds_off_budget", "oasdi_fund_path", 1_097_382),
        ("di_off_budget", "oasdi_fund_path", 186_354),
        ("oasdi_receipt_anchor_sum", "oasdi_fund_path", 1_283_736),
        (
            "medicare_hi_hospital_insurance",
            "medicare_hi_fund_path",
            395_350,
        ),
        (
            "transportation_trust_fund_excise",
            "transportation_trust_fund_path",
            43_768,
        ),
        (
            "airport_and_airway_trust_fund_excise_context",
            "transportation_trust_fund_path",
            23_118,
        ),
    ]
    .into_iter()
    .map(|(id, path, amount)| (id.to_string(), (path.to_string(), amount)))
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected_rows.len() {
        return Err("dedicated receipt anchor row count failed".to_string());
    }
    let mut observed_rows = BTreeMap::new();
    for row in rows {
        let anchor_id = string_field(row, "anchor_id")?;
        let path_id = string_field(row, "path_id")?;
        let amount = int_field(row, "amount_musd")?;
        if string_field(row, "fund_group")? != "trust-funds"
            || row
                .get("may_populate_solver")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!(
                "dedicated receipt anchor {anchor_id} must stay trust-fund and solver-blocked"
            ));
        }
        observed_rows.insert(anchor_id, (path_id, amount));
    }
    if observed_rows != expected_rows {
        return Err("dedicated receipt anchor values failed".to_string());
    }

    let recon = anchors
        .get("reconciliation")
        .ok_or("dedicated receipt reconciliation")?;
    if int_field(recon, "oasi_plus_di_musd")? != 1_097_382 + 186_354
        || int_field(recon, "covered_named_anchor_sum_musd")?
            != 1_283_736 + 395_350 + 43_768 + 23_118
        || int_field(recon, "social_insurance_total_source_row_24_musd")? != 1_748_294
        || int_field(recon, "trust_fund_excise_total_source_row_53_musd")? != 73_372
        || int_field(recon, "total_excise_source_row_54_musd")? != 105_937
        || int_field(recon, "transportation_plus_airport_airway_musd")? != 43_768 + 23_118
        || string_field(recon, "category_reconciliation_status")?
            != "partial_anchor_only_not_full_fund_path"
    {
        return Err("dedicated receipt reconciliation failed".to_string());
    }

    let blocked = anchors
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("dedicated receipt blocked outputs")?;
    for field in [
        "oasdi_outlays_musd",
        "oasdi_fund_balance_musd",
        "medicare_hi_outlays_musd",
        "medicare_hi_fund_balance_musd",
        "transportation_trust_outlays_musd",
        "transportation_trust_fund_balance_musd",
        "explicit_general_fund_transfers_musd",
        "credited_offsetting_collections_musd",
        "reserve_contributions_musd",
        "solver_input_rows",
        "target_cost_fields",
        "federal_effect_fields",
        "gross_savings_fields",
        "net_savings_fields",
        "balanced_rate_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "dedicated receipt blocked field {field} must be null"
            ));
        }
    }

    let claim_booleans = anchors
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("dedicated receipt claim booleans")?;
    for field in [
        "source_custody_ready",
        "dedicated_receipt_anchors_published",
    ] {
        if claim_booleans
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "dedicated receipt claim boolean {field} must be true"
            ));
        }
    }
    for field in [
        "named_trust_fund_paths_complete",
        "solver_inputs_ready",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "balanced_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
        "balanced_budget_claim",
    ] {
        if claim_booleans
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "dedicated receipt claim boolean {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH,
        "$1,097.382B",
        "$186.354B",
        "$1,283.736B",
        "$395.350B",
        "$43.768B",
        "$23.118B",
        "These are FY2025 current-law dedicated-receipt anchors, not complete trust-fund paths.",
        "OASI and DI are summed only as an OASDI receipt anchor",
        "Hospital insurance is a Medicare HI receipt anchor, not combined Medicare financing.",
        "Transportation and airport-and-airway excise rows remain source-labeled",
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
                "dedicated receipt reader missing required phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_fy2025_named_trust_fund_outlay_anchors(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH,
        CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_SCHEMA_PATH,
        CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing named trust-fund outlay artifact: {path}"));
        }
    }

    let text =
        fs::read_to_string(root.join(CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let anchors: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&anchors, "record_id")?
        != "current-law-fy2025-named-trust-fund-outlay-anchors:v1"
        || string_field(&anchors, "record_family")?
            != "current_law_fy2025_named_trust_fund_outlay_anchors"
        || int_field(&anchors, "pulse")? != 126
        || int_field(&anchors, "fiscal_year")? != 2025
        || string_field(&anchors, "year_basis")? != "fiscal_year"
        || string_field(&anchors, "unit")? != "millions_of_dollars"
        || string_field(&anchors, "input_unit_from_source")? != "thousands_of_dollars"
        || string_field(&anchors, "conversion_formula")?
            != "amount_musd = source_amount_thousands_usd / 1000"
        || string_field(
            &anchors,
            "current_law_fy2025_dedicated_receipt_anchors_path",
        )? != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&anchors, "current_law_fy2025_fund_group_path")?
            != CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH
        || string_field(&anchors, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&anchors, "rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
    {
        return Err("named trust-fund outlay identity failed".to_string());
    }

    let status = anchors
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("named trust-fund outlay custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "source_custody_ready",
        "named_trust_fund_outlay_anchors_may_be_populated",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "named trust-fund outlay status {field} must be true"
            ));
        }
    }
    for field in [
        "named_trust_fund_balances_ready",
        "transportation_trust_fund_path_complete",
        "explicit_transfer_schedule_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "named trust-fund outlay status {field} must be false"
            ));
        }
    }

    let packet = anchors
        .get("source_packet")
        .ok_or("named trust-fund outlay source packet")?;
    if string_field(packet, "source_id")? != "SRC-OMB-PBD-OUTLAYS-FY2027"
        || string_field(packet, "official_host_or_publisher")? != "Office of Management and Budget"
        || string_field(packet, "source_table")? != "Public Budget Database outlays FY2027"
        || string_field(packet, "retrieval_date")? != "2026-07-13"
        || packet
            .get("values_may_be_populated")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("named trust-fund outlay source packet identity failed".to_string());
    }
    for (path_field, hash_field, byte_field) in [
        ("raw_artifact_path", "raw_sha256", "raw_byte_count"),
        ("metadata_path", "metadata_sha256", "metadata_byte_count"),
    ] {
        let artifact_path = string_field(packet, path_field)?;
        let artifact_file = root.join(&artifact_path);
        if !artifact_file.exists() {
            return Err(format!(
                "named trust-fund outlay source file missing: {artifact_path}"
            ));
        }
        if fs::metadata(&artifact_file)
            .map_err(|e| e.to_string())?
            .len() as i64
            != int_field(packet, byte_field)?
        {
            return Err(format!(
                "named trust-fund outlay byte count failed for {artifact_path}"
            ));
        }
        if sha256_file(&artifact_file)? != string_field(packet, hash_field)? {
            return Err(format!(
                "named trust-fund outlay hash failed for {artifact_path}"
            ));
        }
    }

    let rows = anchors
        .get("outlay_anchor_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("named trust-fund outlay anchor rows")?;
    let expected_rows = [
        ("oasi_discretionary_outlay_anchor", "oasdi_fund_path", 3_732),
        ("oasi_mandatory_outlay_anchor", "oasdi_fund_path", 1_417_859),
        ("oasi_outlay_anchor_sum", "oasdi_fund_path", 1_421_591),
        ("di_discretionary_outlay_anchor", "oasdi_fund_path", 2_648),
        ("di_mandatory_outlay_anchor", "oasdi_fund_path", 157_551),
        ("di_outlay_anchor_sum", "oasdi_fund_path", 160_199),
        ("oasdi_outlay_anchor_sum", "oasdi_fund_path", 1_581_790),
        (
            "medicare_hi_discretionary_outlay_anchor",
            "medicare_hi_fund_path",
            3_153,
        ),
        (
            "medicare_hi_mandatory_outlay_anchor",
            "medicare_hi_fund_path",
            441_679,
        ),
        (
            "medicare_hi_outlay_anchor_sum",
            "medicare_hi_fund_path",
            444_832,
        ),
    ]
    .into_iter()
    .map(|(id, path, amount)| (id.to_string(), (path.to_string(), amount)))
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected_rows.len() {
        return Err("named trust-fund outlay anchor row count failed".to_string());
    }
    let mut observed_rows = BTreeMap::new();
    for row in rows {
        let anchor_id = string_field(row, "anchor_id")?;
        let path_id = string_field(row, "path_id")?;
        let amount = int_field(row, "amount_musd")?;
        if row
            .get("may_populate_solver")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "named trust-fund outlay anchor {anchor_id} must be solver-blocked"
            ));
        }
        if let Some(source_amount) = row.get("source_amount_thousands_usd") {
            if source_amount
                .as_i64()
                .ok_or("source amount must be integer")?
                / 1000
                != amount
            {
                return Err(format!(
                    "named trust-fund outlay unit conversion failed for {anchor_id}"
                ));
            }
        }
        observed_rows.insert(anchor_id, (path_id, amount));
    }
    if observed_rows != expected_rows {
        return Err("named trust-fund outlay anchor values failed".to_string());
    }

    let recon = anchors
        .get("reconciliation")
        .ok_or("named trust-fund outlay reconciliation")?;
    if int_field(recon, "pbd_fy2025_total_outlays_source_sum_thousands_usd")? != 7_011_105_000
        || int_field(recon, "pbd_fy2025_total_outlays_source_sum_musd")? != 7_011_105
        || int_field(recon, "current_law_total_outlays_musd")? != 7_011_105
        || int_field(recon, "oasi_outlay_anchor_sum_musd")? != 3_732 + 1_417_859
        || int_field(recon, "di_outlay_anchor_sum_musd")? != 2_648 + 157_551
        || int_field(recon, "oasdi_outlay_anchor_sum_musd")? != 1_421_591 + 160_199
        || int_field(recon, "medicare_hi_outlay_anchor_sum_musd")? != 3_153 + 441_679
        || string_field(recon, "receipt_anchor_comparison_status")?
            != "receipt_and_outlay_anchors_present_but_fund_balance_and_transfer_reconciliation_blocked"
    {
        return Err("named trust-fund outlay reconciliation failed".to_string());
    }
    if !string_field(recon, "transportation_status")?
        .contains("complete transportation_trust_fund_path remains blocked")
    {
        return Err("named trust-fund outlay transportation boundary failed".to_string());
    }

    let blocked = anchors
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("named trust-fund outlay blocked outputs")?;
    for field in [
        "oasdi_fund_balance_musd",
        "oasi_fund_balance_musd",
        "di_fund_balance_musd",
        "medicare_hi_fund_balance_musd",
        "transportation_trust_outlays_musd",
        "transportation_trust_fund_balance_musd",
        "explicit_general_fund_transfers_musd",
        "credited_offsetting_collections_musd",
        "reserve_contributions_musd",
        "solver_input_rows",
        "target_cost_fields",
        "federal_effect_fields",
        "gross_savings_fields",
        "net_savings_fields",
        "balanced_rate_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "named trust-fund outlay blocked field {field} must be null"
            ));
        }
    }

    let claim_booleans = anchors
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("named trust-fund outlay claim booleans")?;
    for field in [
        "source_custody_ready",
        "named_trust_fund_outlay_anchors_published",
    ] {
        if claim_booleans
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "named trust-fund outlay claim boolean {field} must be true"
            ));
        }
    }
    for field in [
        "named_trust_fund_paths_complete",
        "transportation_trust_fund_path_complete",
        "solver_inputs_ready",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "balanced_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
        "balanced_budget_claim",
    ] {
        if claim_booleans
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "named trust-fund outlay claim boolean {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH,
        "$1,421.591B",
        "$160.199B",
        "$1,581.790B",
        "$444.832B",
        "These are FY2025 current-law named trust-fund outlay anchors, not complete trust-fund paths.",
        "OASI and DI are summed only as an OASDI outlay anchor; their fund accounting remains separate until fund-balance and transfer sources are captured.",
        "Medicare HI remains separate from SMI and other Medicare.",
        "Transportation remains blocked because the available PBD rows are fragmented across highway, mass transit, airport, interest, and offset accounts.",
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
                "named trust-fund outlay reader missing required phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_named_fund_balance_transfer_gap(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_JSON_PATH,
        CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_SCHEMA_PATH,
        CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing named fund balance-transfer gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let gap: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&gap, "record_id")? != "current-law-named-fund-balance-transfer-gap:v1"
        || string_field(&gap, "record_family")? != "current_law_named_fund_balance_transfer_gap"
        || int_field(&gap, "pulse")? != 127
        || int_field(&gap, "baseline_fiscal_year")? != 2025
        || string_field(&gap, "year_basis")? != "fiscal_year"
        || string_field(&gap, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&gap, "rubric_path")? != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(&gap, "current_law_fy2025_fund_group_path")?
            != CURRENT_LAW_FY2025_FUND_GROUP_PATH_JSON_PATH
        || string_field(&gap, "current_law_fy2025_dedicated_receipt_anchors_path")?
            != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(
            &gap,
            "current_law_fy2025_named_trust_fund_outlay_anchors_path",
        )? != CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH
    {
        return Err("named fund balance-transfer gap identity failed".to_string());
    }

    for path in [
        string_field(&gap, "current_law_fy2025_fund_group_path")?,
        string_field(&gap, "current_law_fy2025_dedicated_receipt_anchors_path")?,
        string_field(
            &gap,
            "current_law_fy2025_named_trust_fund_outlay_anchors_path",
        )?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!("named fund gap referenced path missing: {path}"));
        }
    }

    let status = gap
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("named fund gap source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "available_anchor_sources_reviewed",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("named fund gap status {field} must be true"));
        }
    }
    for field in [
        "balance_source_custody_ready",
        "transfer_source_custody_ready",
        "fund_path_reconciliation_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("named fund gap status {field} must be false"));
        }
    }

    let available = gap
        .get("available_anchor_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("named fund gap available evidence")?;
    if available.len() != 3 {
        return Err("named fund gap must cite three available anchor packets".to_string());
    }
    let missing = gap
        .get("missing_required_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("named fund gap missing source list")?;
    let expected_paths = [
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "general_fund_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_paths = missing
        .iter()
        .map(|row| string_field(row, "path_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_paths != expected_paths {
        return Err("named fund gap missing source paths failed".to_string());
    }
    for row in missing {
        let items = row
            .get("missing_items")
            .and_then(serde_json::Value::as_array)
            .ok_or("named fund gap missing items")?;
        if items.is_empty() || string_field(row, "reason_blocked")?.is_empty() {
            return Err("named fund gap missing item detail failed".to_string());
        }
    }

    let formula = gap
        .get("blocked_formula")
        .ok_or("named fund blocked formula")?;
    if string_field(formula, "status")? != "not_computable" {
        return Err("named fund gap formula must be not computable".to_string());
    }
    let blocked_terms = formula
        .get("blocked_terms")
        .and_then(serde_json::Value::as_array)
        .ok_or("named fund blocked terms")?;
    if blocked_terms.len() != 5 {
        return Err("named fund gap blocked term count failed".to_string());
    }

    let blocked = gap
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("named fund gap blocked outputs")?;
    for field in [
        "oasdi_fund_balance_path",
        "oasi_fund_balance_path",
        "di_fund_balance_path",
        "medicare_hi_fund_balance_path",
        "transportation_trust_fund_balance_path",
        "general_fund_path",
        "explicit_interfund_transfer_schedule",
        "credited_offsetting_collections_by_named_fund",
        "fund_balance_change_values",
        "solver_input_rows",
        "target_cost_fields",
        "federal_effect_fields",
        "gross_savings_fields",
        "net_savings_fields",
        "balanced_rate_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "named fund gap blocked output {field} must be null"
            ));
        }
    }

    let claims = gap
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("named fund gap claim booleans")?;
    if claims
        .get("source_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("named fund gap source_gap_published must be true".to_string());
    }
    for field in [
        "balance_source_custody_ready",
        "transfer_source_custody_ready",
        "fund_path_reconciliation_ready",
        "solver_inputs_ready",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "balanced_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
        "balanced_budget_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("named fund gap claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_JSON_PATH,
        "Receipt and outlay anchors are not complete named trust-fund paths.",
        "Fund balances and explicit transfers remain missing and must stay null.",
        "Federal funds are broader than the general fund and cannot be relabeled as a general-fund path.",
        "Transportation remains blocked until highway, mass transit, airport, interest, offset, and transfer rows reconcile.",
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
            return Err(format!("named fund gap reader missing phrase: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_source_custody_progress_rollup(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law source custody progress artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH}: {err}")
        })?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH}: {err}")
    })?;

    if string_field(&rollup, "record_id")? != "current-law-source-custody-progress-rollup:v1"
        || string_field(&rollup, "record_family")? != "current_law_source_custody_progress_rollup"
        || string_field(&rollup, "status")? != "draft_partial_custody_progress_no_complete_horizon"
        || string_field(&rollup, "current_law_source_custody_batch_plan_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_BATCH_PLAN_JSON_PATH
        || string_field(&rollup, "source_custody_current_law_paths_gap_path")?
            != SOURCE_CUSTODY_CURRENT_LAW_PATHS_GAP_JSON_PATH
        || string_field(&rollup, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
        || string_field(&rollup, "lane_full_coverage_matrix_path")?
            != LANE_FULL_COVERAGE_MATRIX_JSON_PATH
        || string_field(&rollup, "data_acquisition_eight_gap_status_path")?
            != DATA_ACQUISITION_EIGHT_GAP_STATUS_JSON_PATH
    {
        return Err("current-law source custody progress rollup identity failed".to_string());
    }

    let rows = rollup
        .get("path_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law source custody progress path_rows")?;
    if rows.len() != 8 {
        return Err(format!(
            "current-law source custody progress must contain 8 path rows, got {}",
            rows.len()
        ));
    }

    let expected_paths = [
        "full_17_row_fy2025_ledger",
        "baseline_plus_ten_year_horizon",
        "general_fund_path",
        "oasdi_fund_path",
        "medicare_hi_fund_path",
        "transportation_trust_fund_path",
        "health_fiscal_current_law_path",
        "net_interest_current_law_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_paths = BTreeSet::new();
    let mut partial_count = 0usize;
    let mut missing_count = 0usize;
    let mut health_path_checked = false;
    for row in rows {
        let path_id = string_field(row, "path_id")?;
        if !observed_paths.insert(path_id.clone()) {
            return Err(format!(
                "duplicate current-law source custody progress path {path_id}"
            ));
        }
        let status = string_field(row, "progress_status")?;
        match status.as_str() {
            "partial" => partial_count += 1,
            "missing" => missing_count += 1,
            _ => {
                return Err(format!(
                    "unsupported current-law source custody progress status {status}"
                ));
            }
        }
        if row
            .get("path_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
            || row.get("solver_ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(format!(
                "current-law source custody progress path {path_id} must remain incomplete and not solver-ready"
            ));
        }
        let evidence_paths = row
            .get("evidence_paths")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law source custody progress evidence paths")?;
        if status == "partial" && evidence_paths.is_empty() {
            return Err(format!(
                "partial current-law source custody progress path {path_id} needs evidence"
            ));
        }
        for evidence in evidence_paths {
            let evidence_path = evidence
                .as_str()
                .ok_or("current-law source custody evidence path string")?;
            if !root.join(evidence_path).exists() {
                return Err(format!(
                    "current-law source custody progress referenced path missing: {evidence_path}"
                ));
            }
        }
        if path_id == "health_fiscal_current_law_path" {
            let evidence_set = evidence_paths
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<BTreeSet<_>>();
            for expected in [
                CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH,
                CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH,
            ] {
                if !evidence_set.contains(expected) {
                    return Err(format!(
                        "current-law source custody health path missing CBO evidence: {expected}"
                    ));
                }
            }
            health_path_checked = true;
        }
        if row
            .get("blocked_outputs")
            .and_then(serde_json::Value::as_array)
            .is_none()
        {
            return Err(format!(
                "current-law source custody progress path {path_id} missing blocked outputs"
            ));
        }
    }
    if observed_paths != expected_paths {
        return Err("current-law source custody progress path set failed".to_string());
    }
    if !health_path_checked {
        return Err("current-law source custody progress health path not checked".to_string());
    }

    let aggregate = rollup
        .get("aggregate_status")
        .ok_or("current-law source custody progress aggregate")?;
    if int_field(aggregate, "required_paths")? != 8
        || int_field(aggregate, "partial_paths")? != partial_count as i64
        || int_field(aggregate, "missing_paths")? != missing_count as i64
        || int_field(aggregate, "complete_paths")? != 0
        || aggregate
            .get("complete_required_horizon_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("all_current_law_paths_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("solver_inputs_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law source custody progress aggregate failed".to_string());
    }

    let claims = rollup
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law source custody progress claims")?;
    if claims
        .get("current_law_source_custody_progress_rollup_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("current-law source custody progress published flag must be true".to_string());
    }
    for field in [
        "all_current_law_paths_complete",
        "complete_required_horizon_ready",
        "solver_inputs_ready",
        "solver_run_published",
        "target_cost_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "savings_claim_published",
        "waste_finding_published",
        "fraud_finding_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "current-law source custody progress claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_READER_PATH}: {err}"
                )
            })?;
    for phrase in [
        CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH,
        "eight required current-law paths",
        "partial custody progress",
        "February 2026 CBO health-insurance PDF/spreadsheet raw custody and Table 2 rowmap context",
        "data-acquisition eight-gap status is linked as acquisition-status evidence",
        "not as complete source custody",
        "not a complete FY2025-FY2035 baseline",
        "not solver-ready",
        "not rate-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "current-law source custody progress reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_current_law_source_custody_wave_b_closure(root: &Path) -> Result<(), String> {
    for path in [
        CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH,
        CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing current-law source custody Wave B closure artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH))
        .map_err(|err| {
        format!("failed to read {CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH}: {err}")
    })?;
    let closure: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH}: {err}")
    })?;

    if string_field(&closure, "record_id")? != "current-law-source-custody-wave-b-closure:v1"
        || string_field(&closure, "record_family")? != "current_law_source_custody_wave_b_closure"
        || string_field(&closure, "status")?
            != "wave_b_closed_existing_official_custody_exhausted_values_still_blocked"
        || string_field(&closure, "current_law_source_custody_progress_rollup_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PROGRESS_ROLLUP_JSON_PATH
        || string_field(&closure, "data_acquisition_eight_gap_status_path")?
            != DATA_ACQUISITION_EIGHT_GAP_STATUS_JSON_PATH
        || string_field(&closure, "lane_full_coverage_matrix_path")?
            != LANE_FULL_COVERAGE_MATRIX_JSON_PATH
        || string_field(&closure, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
    {
        return Err("current-law source custody Wave B closure identity failed".to_string());
    }

    let completion = closure
        .get("wave_b_completion_definition")
        .ok_or("current-law source custody Wave B completion definition")?;
    for field in [
        "existing_local_official_source_inventory_reconciled",
        "unsupported_values_remain_null",
        "no_interpolation_used",
        "no_external_request_submitted",
        "no_agency_or_person_contacted",
        "wave_b_closed_for_existing_captured_sources",
    ] {
        if completion.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "current-law source custody Wave B completion flag {field} must be true"
            ));
        }
    }
    for field in [
        "all_current_law_paths_complete",
        "all_source_custody_complete",
        "solver_inputs_ready",
        "rates_ready",
    ] {
        if completion.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "current-law source custody Wave B completion flag {field} must be false"
            ));
        }
    }

    let batches = closure
        .get("batch_closure_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("current-law source custody Wave B batch rows")?;
    if batches.len() != 4 {
        return Err(
            "current-law source custody Wave B closure must contain four batches".to_string(),
        );
    }
    let expected_batches = [
        "batch_1_federal_baseline_and_17_row_ledger",
        "batch_2_trust_fund_paths",
        "batch_3_health_current_law_components",
        "batch_4_net_interest_and_debt_path",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_batches = BTreeSet::new();
    let mut health_batch_checked = false;
    for row in batches {
        let batch_id = string_field(row, "batch_id")?;
        observed_batches.insert(batch_id.clone());
        if string_field(row, "closure_status")? != "closed_for_existing_local_official_sources"
            || row
                .get("path_complete")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row.get("solver_ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(format!(
                "current-law source custody Wave B batch {batch_id} status failed"
            ));
        }
        let evidence_paths = row
            .get("evidence_paths")
            .and_then(serde_json::Value::as_array)
            .ok_or("current-law source custody Wave B evidence paths")?;
        if evidence_paths.is_empty() {
            return Err(format!(
                "current-law source custody Wave B batch {batch_id} needs evidence"
            ));
        }
        for evidence in evidence_paths {
            let evidence_path = evidence
                .as_str()
                .ok_or("current-law source custody Wave B evidence path string")?;
            if !root.join(evidence_path).exists() {
                return Err(format!(
                    "current-law source custody Wave B referenced path missing: {evidence_path}"
                ));
            }
        }
        if batch_id == "batch_3_health_current_law_components" {
            let evidence_set = evidence_paths
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<BTreeSet<_>>();
            for expected in [
                CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH,
                CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH,
            ] {
                if !evidence_set.contains(expected) {
                    return Err(format!(
                        "current-law source custody Wave B health batch missing CBO evidence: {expected}"
                    ));
                }
            }
            if !string_field(row, "supported_values")?.contains(
                "February 2026 CBO health-insurance PDF/spreadsheet raw custody and Table 2 rowmap context",
            ) {
                return Err(
                    "current-law source custody Wave B health batch missing February CBO raw custody support"
                        .to_string(),
                );
            }
            let still_blocked = row
                .get("still_blocked")
                .and_then(serde_json::Value::as_array)
                .ok_or("current-law source custody Wave B health blockers")?;
            let still_blocked_set = still_blocked
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<BTreeSet<_>>();
            if still_blocked_set.contains("CBO health baseline local raw custody")
                || !still_blocked_set.contains("July 2026 CBO latest-publication local raw custody")
            {
                return Err(
                    "current-law source custody Wave B health CBO blocker boundary failed"
                        .to_string(),
                );
            }
            health_batch_checked = true;
        }
        if row
            .get("still_blocked")
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "current-law source custody Wave B batch {batch_id} needs blockers"
            ));
        }
    }
    if observed_batches != expected_batches {
        return Err("current-law source custody Wave B batch set failed".to_string());
    }
    if !health_batch_checked {
        return Err("current-law source custody Wave B health batch not checked".to_string());
    }

    let aggregate = closure
        .get("aggregate_status")
        .ok_or("current-law source custody Wave B aggregate")?;
    if int_field(aggregate, "batches_closed_for_existing_sources")? != 4
        || int_field(aggregate, "required_current_law_paths_reviewed")? != 8
        || int_field(aggregate, "complete_current_law_paths")? != 0
        || aggregate
            .get("wave_b_done")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || aggregate
            .get("solver_inputs_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("rates_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("current-law source custody Wave B aggregate failed".to_string());
    }

    let claims = closure
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("current-law source custody Wave B claims")?;
    if claims
        .get("current_law_source_custody_wave_b_closure_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || claims
            .get("wave_b_done")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("current-law source custody Wave B publish flags must be true".to_string());
    }
    for field in [
        "all_current_law_paths_complete",
        "all_source_custody_complete",
        "solver_inputs_ready",
        "solver_run_published",
        "target_cost_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "savings_claim_published",
        "waste_finding_published",
        "fraud_finding_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "current-law source custody Wave B claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_READER_PATH}: {err}"
                )
            })?;
    for phrase in [
        CURRENT_LAW_SOURCE_CUSTODY_WAVE_B_CLOSURE_JSON_PATH,
        "Wave B is done",
        "closed for existing captured official sources",
        "data-acquisition eight-gap status is linked as the latest acquisition-status packet",
        "not as complete source custody",
        "February 2026 CBO health-insurance PDF/spreadsheet raw custody and Table 2 rowmap context",
        "July 2026 CBO latest-publication local raw custody",
        "not all current-law paths are complete",
        "not solver-ready",
        "not rate-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "current-law source custody Wave B reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

