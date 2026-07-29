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

pub(crate) fn validate_social_security_outcome_floor_definition_packet(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "social-security-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")?
            != "social_security_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 162
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(&record, "health_outcome_floor_definition_packet_path")?
            != HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("Social Security floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Social Security floor status {field} must be true"));
        }
    }
    for field in [
        "new_external_download_performed",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "lower_cost_scenario_admissibility_ready",
        "target_cost_ready",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security floor status {field} must be false"
            ));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security floor definition policy")?;
    for field in [
        "trust_funds_remain_separate",
        "transfers_must_be_explicit",
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "statutory_rates_cannot_be_published_before_base_behavior_incidence_distribution_and_administration",
        "international_differences_not_savings",
        "no_fraud_inference",
        "federal_translation_required_before_solver_use",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Social Security floor policy {field} must be true"));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("Social Security required floor class count failed".to_string());
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
        return Err("Social Security required floor class set failed".to_string());
    }
    for row in classes {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!("Social Security floor class {field} must be null"));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("Social Security floor class must remain unpassed".to_string());
        }
    }

    let ss_floors = record
        .get("social_security_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security-specific floor definitions")?;
    let expected_ss_floors = [
        "replacement_adequacy",
        "old_age_poverty",
        "disability_and_survivor_protection",
        "trust_fund_continuity",
        "administration_and_transition_feasibility",
    ];
    if ss_floors.len() != expected_ss_floors.len() {
        return Err("Social Security-specific floor count failed".to_string());
    }
    let observed_ss_floors = ss_floors
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_ss_floor_set = expected_ss_floors
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_ss_floors != expected_ss_floor_set {
        return Err("Social Security-specific floor set failed".to_string());
    }
    for row in ss_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "Social Security-specific floors must remain null and unpassed".to_string(),
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
        .ok_or("Social Security floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("social_security_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
    {
        return Err("Social Security floor summary counts failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "all_floors_passed",
        "lower_cost_scenario_admissibility_ready",
        "target_cost_ready",
        "solver_input_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security floor summary {field} must be false"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Social Security floor definition packet publication flag failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
        "lower_cost_scenario_admissibility_ready",
        "target_cost_published",
        "federal_effect_published",
        "gross_savings_published",
        "net_savings_published",
        "assigned_base_rate_published",
        "solver_input_ready",
        "solver_run_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Social Security floor claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This Social Security floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "OASDI trust-fund accounting remains separate; transfers or reallocations must be explicit and cannot be inferred.",
        "No lower-cost Social Security scenario is admissible until replacement adequacy, old-age poverty, disability/survivor protection, access, equity, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "No target cost, federal effect, gross savings, net savings, assigned-base rate, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a demographic score",
        "not a trust-fund solvency score",
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
                "Social Security floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_source_readiness_gap(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH,
        SOCIAL_SECURITY_SOURCE_READINESS_GAP_SCHEMA_PATH,
        SOCIAL_SECURITY_SOURCE_READINESS_GAP_READER_PATH,
        DENOMINATOR_VALUES_CY2025_SSA_TRUSTEES_JSONL_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security source readiness artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "social-security-source-readiness-gap:v1"
        || string_field(&record, "record_family")? != "social_security_source_readiness_gap"
        || int_field(&record, "pulse")? != 184
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(
            &record,
            "social_security_outcome_floor_definition_packet_path",
        )? != SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&record, "receipt_base_official_source_capture_path")?
            != RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH
        || string_field(&record, "denominator_values_path")?
            != DENOMINATOR_VALUES_CY2025_SSA_TRUSTEES_JSONL_PATH
    {
        return Err("Social Security source readiness identity failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("Social Security source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "derived_denominator_context_present",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Social Security source custody should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "ssa_trustees_raw_artifact_path_present",
        "ssa_trustees_metadata_path_present",
        "ssa_trustees_raw_sha256_present",
        "ssa_trustees_source_custody_ready",
        "oasdi_annual_fund_path_ready",
        "oasdi_taxable_payroll_base_ready",
        "seventy_five_year_solvency_path_ready",
        "adequacy_or_poverty_floor_values_ready",
        "solver_input_ready",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security source custody must be false: {field}"
            ));
        }
    }

    let denom_text =
        fs::read_to_string(root.join(DENOMINATOR_VALUES_CY2025_SSA_TRUSTEES_JSONL_PATH))
            .map_err(|e| e.to_string())?;
    let denom_rows = denom_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<serde_json::Value>(line).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    if denom_rows.len() != 4 {
        return Err("Social Security denominator JSONL row count failed".to_string());
    }

    let context_rows = record
        .get("existing_derived_context_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security derived context rows")?;
    if context_rows.len() != 4 {
        return Err("Social Security derived context row count failed".to_string());
    }

    let expected_values = [
        (
            "denominator-value:oasdi-covered-workers:cy2025:ssa-trustees-2026",
            "oasdi_covered_workers",
            185000000,
        ),
        (
            "denominator-value:oasdi-beneficiaries:cy2025:ssa-trustees-2026",
            "oasdi_beneficiaries",
            70500000,
        ),
        (
            "denominator-value:oasi-beneficiaries:cy2025:ssa-trustees-2026",
            "oasi_beneficiaries",
            62300000,
        ),
        (
            "denominator-value:di-beneficiaries:cy2025:ssa-trustees-2026",
            "di_beneficiaries",
            8200000,
        ),
    ];
    for (record_id, denominator_id, expected_value) in expected_values {
        let row = context_rows
            .iter()
            .find(|item| string_field(item, "record_id").as_deref() == Ok(record_id))
            .ok_or_else(|| format!("missing Social Security context row: {record_id}"))?;
        let denom = denom_rows
            .iter()
            .find(|item| string_field(item, "record_id").as_deref() == Ok(record_id))
            .ok_or_else(|| format!("missing Social Security denominator row: {record_id}"))?;
        if string_field(row, "denominator_id")? != denominator_id
            || string_field(denom, "denominator_id")? != denominator_id
            || int_field(row, "value")? != expected_value
            || int_field(denom, "value")? != expected_value
            || string_field(row, "year")? != "CY2025"
            || string_field(row, "year_basis")? != "calendar_year"
            || string_field(row, "unit")? != "people"
            || row
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("may_populate_taxable_payroll_base")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("may_populate_solver_input")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("Social Security context row failed: {record_id}"));
        }
    }

    let blocked_packet = record
        .get("blocked_source_packet")
        .ok_or("Social Security blocked source packet")?;
    if string_field(blocked_packet, "work_item_id")? != "capture-ssa-oasdi-taxable-payroll-base"
        || string_field(blocked_packet, "source_id")? != "SRC-SSA-TRUSTEES-2026"
        || string_field(blocked_packet, "prior_block_status")?
            != "official_site_returned_http_403_to_direct_raw_download"
        || !blocked_packet
            .get("raw_artifact_path")
            .is_some_and(serde_json::Value::is_null)
        || !blocked_packet
            .get("raw_byte_count")
            .is_some_and(serde_json::Value::is_null)
        || !blocked_packet
            .get("raw_sha256")
            .is_some_and(serde_json::Value::is_null)
        || !blocked_packet
            .get("metadata_path")
            .is_some_and(serde_json::Value::is_null)
        || blocked_packet
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || !blocked_packet
            .get("value_populated")
            .is_some_and(serde_json::Value::is_null)
        || blocked_packet
            .get("ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Social Security blocked source packet failed".to_string());
    }

    let floors = record
        .get("floor_value_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security floor value status")?;
    if floors.len() != 5 {
        return Err("Social Security floor value count failed".to_string());
    }
    for floor in floors {
        if string_field(floor, "floor_class")?.is_empty()
            || !floor
                .get("threshold_value")
                .is_some_and(serde_json::Value::is_null)
            || !floor
                .get("baseline_value")
                .is_some_and(serde_json::Value::is_null)
            || !floor
                .get("policy_value")
                .is_some_and(serde_json::Value::is_null)
            || !floor
                .get("stress_value")
                .is_some_and(serde_json::Value::is_null)
            || floor.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("Social Security floor values must remain blocked".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security source readiness claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("Social Security claim bool")?;
        if matches!(
            field.as_str(),
            "social_security_source_readiness_gap_published"
                | "derived_denominator_context_present"
        ) {
            if !observed {
                return Err(format!("Social Security claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("Social Security claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "derived CY2025 OASDI denominator context only",
        "not SSA raw source custody",
        "not an OASDI annual fund path",
        "not a 75-year solvency path",
        "not a taxable payroll base",
        "not benefit adequacy or old-age poverty floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security source readiness warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(SOCIAL_SECURITY_SOURCE_READINESS_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH,
        "OASDI covered workers: 185,000,000 people",
        "OASDI beneficiaries: 70,500,000 people",
        "OASI beneficiaries: 62,300,000 people",
        "DI beneficiaries: 8,200,000 people",
        "SSA raw source custody",
        "annual OASDI fund path",
        "75-year solvency path",
        "taxable payroll base",
        "not SSA raw source custody",
        "not an OASDI annual fund path",
        "not a 75-year solvency path",
        "not a taxable payroll base",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security source readiness reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_source_capture_queue(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH,
        SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security source capture queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "social-security-source-capture-queue:v1"
        || string_field(&record, "record_family")? != "social_security_source_capture_queue"
        || int_field(&record, "pulse")? != 185
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "social_security_source_readiness_gap_path")?
            != SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(
            &record,
            "social_security_outcome_floor_definition_packet_path",
        )? != SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
    {
        return Err("Social Security source capture queue identity failed".to_string());
    }

    let rules = record
        .get("source_rules")
        .ok_or("Social Security source capture rules")?;
    for field in [
        "official_sources_only",
        "use_existing_captured_sources_when_available",
        "new_external_downloads_not_performed_in_this_pulse",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "threshold_selection_requires_stronger_model_review",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Social Security source rule must be true: {field}"));
        }
    }

    let items = record
        .get("capture_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security capture items")?;
    if items.len() != 6 {
        return Err("Social Security capture item count failed".to_string());
    }
    let expected = [
        ("capture-oasdi-annual-fund-path", 1),
        ("capture-oasdi-75-year-solvency-path", 2),
        ("capture-oasdi-taxable-payroll-base", 3),
        ("capture-social-security-benefit-adequacy-floors", 4),
        ("capture-old-age-poverty-floor-values", 5),
        ("capture-ssa-administration-transition-capacity", 6),
    ];
    for (work_item_id, priority) in expected {
        let item = items
            .iter()
            .find(|item| string_field(item, "work_item_id").as_deref() == Ok(work_item_id))
            .ok_or_else(|| format!("missing Social Security capture item: {work_item_id}"))?;
        if int_field(item, "priority")? != priority
            || string_field(item, "official_source_family")?.is_empty()
            || item
                .get("needed_for")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.is_empty())
            || item
                .get("required_fields")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.is_empty())
        {
            return Err(format!(
                "Social Security capture item shape failed: {work_item_id}"
            ));
        }
        for field in [
            "raw_artifact_path",
            "raw_byte_count",
            "raw_sha256",
            "metadata_path",
            "value",
        ] {
            if !item.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "Social Security capture item field must be null: {work_item_id}.{field}"
                ));
            }
        }
        if item.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security capture item must not be ready: {work_item_id}"
            ));
        }
    }

    let counts = record
        .get("aggregate_status")
        .ok_or("Social Security capture aggregate status")?;
    for (field, expected) in [
        ("capture_item_count", 6),
        ("values_populated_count", 0),
        ("items_ready_count", 0),
        ("threshold_values_selected", 0),
        ("baseline_values_populated", 0),
        ("policy_values_populated", 0),
        ("stress_values_populated", 0),
        ("solver_ready_items", 0),
        ("public_rate_ready_items", 0),
    ] {
        if int_field(counts, field)? != expected {
            return Err(format!(
                "Social Security capture aggregate count failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security capture blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security capture blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security capture claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security capture claim bool")?;
        if field == "social_security_source_capture_queue_published" {
            if !observed {
                return Err("Social Security capture queue published flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "Social Security capture claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "names official-source work items only",
        "not SSA raw source custody",
        "not an OASDI annual fund path",
        "not a 75-year solvency path",
        "not a taxable payroll base",
        "not benefit adequacy or old-age poverty floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security capture warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        "OASDI annual fund path",
        "OASDI 75-year solvency path",
        "OASDI taxable payroll base",
        "Social Security benefit adequacy floors",
        "Old-age poverty floor values",
        "SSA administration and transition capacity",
        "not SSA raw source custody",
        "not an OASDI annual fund path",
        "not a 75-year solvency path",
        "not a taxable payroll base",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security capture reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_trustees_source_capture_status(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH,
        SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH,
        "data/metadata/SRC-SSA-TRUSTEES-2026.2026-07-23.metadata.md",
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security Trustees source capture artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH),
    )
    .map_err(|err| {
        format!("failed to read {SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH}: {err}")
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "social-security-trustees-source-capture-status:v1"
        || string_field(&record, "record_family")?
            != "social_security_trustees_source_capture_status"
        || int_field(&record, "pulse")? != 202
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "social_security_source_capture_queue_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "social_security_source_readiness_gap_path")?
            != SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(&record, "denominator_values_path")?
            != DENOMINATOR_VALUES_CY2025_SSA_TRUSTEES_JSONL_PATH
    {
        return Err("Social Security Trustees source capture status identity failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("Social Security Trustees custody status")?;
    if !bool_field(custody, "official_sources_only")?
        || !bool_field(custody, "new_external_download_attempted")?
        || bool_field(custody, "local_raw_byte_capture_ready")?
        || !bool_field(custody, "access_boundary_recorded")?
        || !bool_field(custody, "browser_verified_official_source")?
        || bool_field(custody, "sha256_available")?
        || !bool_field(custody, "may_populate_oasdi_context_values")?
        || bool_field(custody, "may_populate_complete_annual_fund_path")?
        || bool_field(custody, "may_populate_taxable_payroll_base")?
        || bool_field(custody, "may_populate_floor_thresholds")?
        || bool_field(custody, "may_populate_pass_fail_findings")?
        || bool_field(custody, "may_populate_solver_inputs")?
        || bool_field(custody, "source_capture_complete")?
    {
        return Err(
            "Social Security Trustees custody status must preserve local-byte and solver blockers"
                .to_string(),
        );
    }

    let surfaces = record
        .get("official_source_surfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security Trustees source surfaces")?;
    if surfaces.len() != 3 {
        return Err(
            "Social Security Trustees capture must record 3 official source surfaces".to_string(),
        );
    }
    for surface in surfaces {
        if string_field(surface, "publisher")?
            != "Social Security Administration, Office of the Chief Actuary"
            || surface.get("raw_artifact_path") != Some(&serde_json::Value::Null)
            || surface.get("raw_byte_count") != Some(&serde_json::Value::Null)
            || surface.get("raw_sha256") != Some(&serde_json::Value::Null)
            || bool_field(surface, "custody_ready")?
            || !bool_field(surface, "link_review_ready")?
        {
            return Err(
                "Social Security Trustees source surfaces must be link-reviewed with local bytes blocked"
                    .to_string(),
            );
        }
        if !string_field(surface, "source_url")?.starts_with("https://www.ssa.gov/oact/TR/2026/")
            || !string_field(surface, "access_boundary")?.contains("HTTP 403")
        {
            return Err(
                "Social Security Trustees source surfaces must cite official SSA URLs and access boundary"
                    .to_string(),
            );
        }
    }

    let contexts = record
        .get("verified_context_values")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security Trustees context values")?;
    if contexts.len() != 3 {
        return Err(
            "Social Security Trustees capture must carry 3 context value groups".to_string(),
        );
    }
    let mut context_ids = BTreeSet::new();
    for context in contexts {
        let context_id = string_field(context, "context_id")?;
        if !context_ids.insert(context_id.to_string()) {
            return Err(format!(
                "duplicate Social Security Trustees context {context_id}"
            ));
        }
        if !string_field(context, "allowed_use")?.contains("not") || context.get("values").is_none()
        {
            return Err(
                "Social Security Trustees context values must preserve bounded allowed use"
                    .to_string(),
            );
        }
    }
    for required in [
        "oasdi-key-results-intermediate-2026",
        "oasdi-2025-review-highlight",
        "oasdi-single-year-table-inventory",
    ] {
        if !context_ids.contains(required) {
            return Err(format!(
                "missing Social Security Trustees context {required}"
            ));
        }
    }

    let progress = record
        .get("capture_item_progress")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security Trustees capture item progress")?;
    if progress.len() != 6 {
        return Err(
            "Social Security Trustees capture progress must cover 6 queue items".to_string(),
        );
    }
    let mut ready_count = 0;
    for item in progress {
        if bool_field(item, "ready")? {
            ready_count += 1;
        }
    }
    if ready_count != 0 {
        return Err("Social Security Trustees capture items must remain not ready".to_string());
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("Social Security Trustees aggregate status")?;
    if int_field(aggregate, "capture_item_count")? != 6
        || int_field(aggregate, "items_with_new_official_source_progress_count")? != 3
        || int_field(aggregate, "items_ready_count")? != 0
        || int_field(aggregate, "browser_verified_source_surface_count")? != 3
        || int_field(aggregate, "local_raw_custody_ready_count")? != 0
        || int_field(aggregate, "context_value_groups_populated")? != 3
        || int_field(aggregate, "solver_ready_items")? != 0
        || int_field(aggregate, "public_rate_ready_items")? != 0
    {
        return Err("Social Security Trustees aggregate status failed".to_string());
    }

    let blocked_outputs = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security Trustees blocked outputs")?;
    for key in [
        "oasdi_complete_annual_fund_path",
        "taxable_payroll_base",
        "pass_fail_findings",
        "gross_savings",
        "net_savings",
        "solver_input",
        "rate_calculation",
        "public_rate_card",
    ] {
        if blocked_outputs.get(key) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security Trustees blocked output must stay null: {key}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .ok_or("Social Security Trustees claim booleans")?;
    for blocked in [
        "local_raw_byte_custody_ready",
        "source_capture_complete",
        "complete_annual_fund_path_ready",
        "taxable_payroll_base_ready",
        "threshold_values_selected",
        "baseline_floor_values_populated",
        "policy_floor_values_populated",
        "stress_floor_values_populated",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "target_cost_published",
        "federal_effect_published",
        "gross_savings_published",
        "net_savings_published",
        "solver_input_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if bool_field(claims, blocked)? {
            return Err(format!(
                "Social Security Trustees claim must remain false: {blocked}"
            ));
        }
    }
    if !bool_field(claims, "official_source_surfaces_browser_verified")?
        || !bool_field(claims, "oasdi_context_values_populated")?
    {
        return Err(
            "Social Security Trustees verified source/context claims must be true".to_string(),
        );
    }

    let reader =
        fs::read_to_string(root.join(SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH))
            .map_err(|err| {
            format!(
                "failed to read {SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH}: {err}"
            )
        })?;
    for required in [
        SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH,
        "HTTP 403",
        "OASDI",
        "What remains blocked",
        "not a rate or savings result",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security Trustees reader must cite required text: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_oasdi_fy2025_2035_current_law_path(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH,
        SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security OASDI current-law path artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "social-security-oasdi-fy2025-2035-current-law-path:v1"
        || string_field(&record, "record_family")? != "social_security_oasdi_current_law_path"
        || int_field(&record, "pulse")? != 203
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_path")?
            != SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "source_id")? != "SRC-SSA-TRUSTEES-2026"
        || string_field(&record, "source_url")?
            != "https://www.ssa.gov/OACT/TR/2026/VI_C_SRfyproj.html"
        || string_field(&record, "year_basis")? != "fiscal_year"
        || string_field(&record, "trust_fund_scope")? != "combined_oasi_di"
        || string_field(&record, "assumption_set")? != "intermediate"
    {
        return Err("Social Security OASDI current-law path identity failed".to_string());
    }
    if !string_field(&record, "source_table_ref")?.contains("Table VI.C6")
        || !string_field(&record, "access_boundary")?.contains("HTTP 403")
    {
        return Err(
            "Social Security OASDI path must cite SSA Table VI.C6 and access boundary".to_string(),
        );
    }
    for raw_field in ["raw_artifact_path", "raw_byte_count", "raw_sha256"] {
        if record.get(raw_field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security OASDI path raw field must remain null: {raw_field}"
            ));
        }
    }

    let status = record
        .get("path_status")
        .ok_or("Social Security OASDI path status")?;
    if !bool_field(status, "official_fy2025_fy2035_rows_present")?
        || int_field(status, "row_count")? != 11
        || int_field(status, "actual_rows")? != 1
        || int_field(status, "intermediate_projection_rows")? != 10
        || int_field(status, "complete_total_income_cost_reserve_rows")? != 9
        || int_field(status, "post_depletion_partial_rows")? != 2
        || bool_field(status, "local_raw_custody_ready")?
        || bool_field(status, "taxable_payroll_base_ready")?
        || bool_field(status, "fiscal_solver_ready")?
        || bool_field(status, "rate_ready")?
        || bool_field(status, "savings_ready")?
    {
        return Err("Social Security OASDI path status failed".to_string());
    }

    let rows = record
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security OASDI rows")?;
    if rows.len() != 11 {
        return Err("Social Security OASDI current-law path must have 11 rows".to_string());
    }

    let mut years = BTreeSet::new();
    let mut complete_rows = 0;
    let mut partial_rows = 0;
    for row in rows {
        let fiscal_year = int_field(row, "fiscal_year")?;
        years.insert(fiscal_year);
        let completeness = string_field(row, "row_completeness")?;
        if completeness == "complete_displayed_row" {
            complete_rows += 1;
            for field in [
                "total_income",
                "net_interest",
                "net_change_during_year",
                "reserves_end_of_year",
            ] {
                number_field(row, field)?;
            }
        } else if completeness == "partial_after_reserve_depletion" {
            partial_rows += 1;
            for field in [
                "total_income",
                "net_interest",
                "net_change_during_year",
                "reserves_end_of_year",
            ] {
                if row.get(field) != Some(&serde_json::Value::Null) {
                    return Err(format!(
                        "post-depletion Social Security OASDI field must be null: FY{fiscal_year} {field}"
                    ));
                }
            }
        } else {
            return Err(format!(
                "unexpected Social Security OASDI row completeness: {completeness}"
            ));
        }
    }
    let expected_years: BTreeSet<i64> = (2025..=2035).collect();
    if years != expected_years || complete_rows != 9 || partial_rows != 2 {
        return Err(
            "Social Security OASDI current-law rows must cover FY2025-FY2035 with 9 complete and 2 partial rows"
                .to_string(),
        );
    }

    let row_by_year: BTreeMap<i64, &serde_json::Value> = rows
        .iter()
        .map(|row| Ok((int_field(row, "fiscal_year")?, row)))
        .collect::<Result<_, String>>()?;
    let fy2025 = row_by_year.get(&2025).ok_or("missing FY2025 OASDI row")?;
    if (number_field(fy2025, "total_income")? - 1438.2).abs() > 0.001
        || (number_field(fy2025, "total_cost")? - 1581.8).abs() > 0.001
        || (number_field(fy2025, "reserves_end_of_year")? - 2616.6).abs() > 0.001
        || int_field(fy2025, "trust_fund_ratio_start_of_year")? != 175
    {
        return Err("Social Security OASDI FY2025 anchor values failed".to_string());
    }
    let fy2033 = row_by_year.get(&2033).ok_or("missing FY2033 OASDI row")?;
    if (number_field(fy2033, "reserves_end_of_year")? - 322.3).abs() > 0.001
        || int_field(fy2033, "trust_fund_ratio_start_of_year")? != 29
    {
        return Err("Social Security OASDI FY2033 reserve anchor failed".to_string());
    }
    let fy2034 = row_by_year.get(&2034).ok_or("missing FY2034 OASDI row")?;
    if row_by_year
        .get(&2035)
        .ok_or("missing FY2035 OASDI row")?
        .get("trust_fund_ratio_start_of_year")
        != Some(&serde_json::Value::Null)
        || int_field(fy2034, "trust_fund_ratio_start_of_year")? != 13
        || (number_field(fy2034, "total_cost")? - 2467.3).abs() > 0.001
    {
        return Err("Social Security OASDI post-depletion anchors failed".to_string());
    }

    let blocked_outputs = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security OASDI blocked outputs")?;
    for key in [
        "complete_oasi_di_split_path",
        "taxable_payroll_base",
        "pass_fail_findings",
        "gross_savings",
        "net_savings",
        "solver_input",
        "rate_calculation",
        "public_rate_card",
    ] {
        if blocked_outputs.get(key) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security OASDI blocked output must remain null: {key}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .ok_or("Social Security OASDI claim booleans")?;
    for expected_true in [
        "official_fy2025_fy2035_combined_oasdi_rows_present",
        "complete_displayed_rows_ready",
        "post_depletion_rows_partial",
    ] {
        if !bool_field(claims, expected_true)? {
            return Err(format!(
                "Social Security OASDI claim must be true: {expected_true}"
            ));
        }
    }
    for blocked in [
        "local_raw_byte_custody_ready",
        "complete_oasi_di_split_path_ready",
        "taxable_payroll_base_ready",
        "floor_values_ready",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "solver_input_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if bool_field(claims, blocked)? {
            return Err(format!(
                "Social Security OASDI claim must remain false: {blocked}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_READER_PATH}: {err}"
        )
    })?;
    for required in [
        SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH,
        "Table VI.C6",
        "explicit nulls",
        "What remains blocked",
        "not a solver-ready Social Security lane",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security OASDI reader must cite required text: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_taxable_payroll_base_bridge(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH,
        SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security taxable payroll base bridge artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH}: {err}")
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")?
        != "social-security-taxable-payroll-base-bridge:cy2025-2035:v1"
        || string_field(&record, "record_family")? != "social_security_taxable_payroll_base_bridge"
        || int_field(&record, "pulse")? != 204
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_path")?
            != SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "current_law_path_path")?
            != SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH
        || string_field(&record, "year_basis")? != "calendar_year"
        || string_field(&record, "assumption_set")? != "intermediate"
    {
        return Err("Social Security taxable payroll base bridge identity failed".to_string());
    }
    if !string_field(&record, "access_boundary")?.contains("HTTP 403") {
        return Err(
            "Social Security taxable payroll base bridge must preserve access boundary".to_string(),
        );
    }
    for raw_field in ["raw_artifact_path", "raw_byte_count", "raw_sha256"] {
        if record.get(raw_field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security taxable payroll raw field must remain null: {raw_field}"
            ));
        }
    }

    let table_refs = record
        .get("source_table_refs")
        .ok_or("Social Security taxable payroll table refs")?;
    for (field, required) in [
        ("taxable_payroll", "Table VI.G1"),
        ("contribution_and_benefit_base", "Table V.C1"),
        ("statutory_rate_context", "Table V.C6"),
    ] {
        if !string_field(table_refs, field)?.contains(required) {
            return Err(format!(
                "Social Security taxable payroll bridge must cite {required}"
            ));
        }
    }

    let base_scope = record
        .get("base_scope")
        .ok_or("Social Security taxable payroll base scope")?;
    if string_field(base_scope, "legal_base")? != "OASDI taxable payroll"
        || !string_field(base_scope, "economic_base")?.contains("taxable self-employment")
        || bool_field(base_scope, "calendar_to_fiscal_bridge_ready")?
        || bool_field(base_scope, "omb_receipt_reconciliation_ready")?
        || bool_field(base_scope, "reform_yield_ready")?
        || bool_field(base_scope, "rate_publication_ready")?
    {
        return Err(
            "Social Security taxable payroll base scope must keep bridge blockers".to_string(),
        );
    }

    let rows = record
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security taxable payroll rows")?;
    if rows.len() != 11 {
        return Err("Social Security taxable payroll bridge must have 11 rows".to_string());
    }
    let mut years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "calendar_year")?;
        years.insert(year);
        number_field(row, "taxable_payroll_billions")?;
        number_field(row, "gross_domestic_product_billions")?;
        number_field(row, "taxable_payroll_to_gdp_ratio")?;
        number_field(row, "average_wage_index")?;
        int_field(row, "contribution_and_benefit_base_dollars")?;
        if (number_field(row, "combined_oasdi_payroll_tax_rate_percent")? - 12.4).abs() > 0.001 {
            return Err(format!(
                "Social Security taxable payroll row must preserve 12.4 percent current-law rate for CY{year}"
            ));
        }
    }
    let expected_years: BTreeSet<i64> = (2025..=2035).collect();
    if years != expected_years {
        return Err("Social Security taxable payroll bridge must cover CY2025-CY2035".to_string());
    }
    let row_by_year: BTreeMap<i64, &serde_json::Value> = rows
        .iter()
        .map(|row| Ok((int_field(row, "calendar_year")?, row)))
        .collect::<Result<_, String>>()?;
    for (year, payroll, base, awi) in [
        (2025, 10562.0, 176100, 72025.07),
        (2030, 13258.0, 215400, 88895.99),
        (2035, 16486.0, 267000, 109064.86),
    ] {
        let row = row_by_year
            .get(&year)
            .ok_or_else(|| format!("missing taxable payroll anchor CY{year}"))?;
        if (number_field(row, "taxable_payroll_billions")? - payroll).abs() > 0.001
            || int_field(row, "contribution_and_benefit_base_dollars")? != base
            || (number_field(row, "average_wage_index")? - awi).abs() > 0.001
        {
            return Err(format!(
                "Social Security taxable payroll anchor failed for CY{year}"
            ));
        }
    }

    let bridge_status = record
        .get("bridge_status")
        .ok_or("Social Security taxable payroll bridge status")?;
    for ready in [
        "calendar_year_taxable_payroll_base_ready",
        "contribution_and_benefit_base_ready",
        "current_law_combined_rate_context_ready",
    ] {
        if !bool_field(bridge_status, ready)? {
            return Err(format!(
                "Social Security taxable payroll bridge status must be true: {ready}"
            ));
        }
    }
    for blocked in [
        "fiscal_year_bridge_ready",
        "omb_receipt_yield_reconciliation_ready",
        "distribution_incidence_ready",
        "behavior_reform_yield_ready",
        "administration_burden_ready",
        "solver_receipt_row_ready",
        "public_rate_ready",
    ] {
        if bool_field(bridge_status, blocked)? {
            return Err(format!(
                "Social Security taxable payroll bridge status must remain false: {blocked}"
            ));
        }
    }

    let blocked_outputs = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security taxable payroll blocked outputs")?;
    for key in [
        "fiscal_year_taxable_payroll_base",
        "omb_social_insurance_receipt_reconciliation",
        "current_law_yield_for_solver",
        "reform_yield",
        "solver_receipt_row",
        "rate_calculation",
        "public_rate_card",
        "gross_savings",
        "net_savings",
        "balanced_budget_claim",
    ] {
        if blocked_outputs.get(key) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security taxable payroll blocked output must stay null: {key}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .ok_or("Social Security taxable payroll claim booleans")?;
    for ready in [
        "calendar_year_taxable_payroll_base_ready",
        "contribution_and_benefit_base_ready",
        "current_law_combined_rate_context_ready",
    ] {
        if !bool_field(claims, ready)? {
            return Err(format!(
                "Social Security taxable payroll claim must be true: {ready}"
            ));
        }
    }
    for blocked in [
        "fiscal_year_bridge_ready",
        "omb_receipt_yield_reconciliation_ready",
        "distribution_incidence_ready",
        "behavior_reform_yield_ready",
        "administration_burden_ready",
        "solver_receipt_row_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if bool_field(claims, blocked)? {
            return Err(format!(
                "Social Security taxable payroll claim must remain false: {blocked}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_READER_PATH}: {err}")
    })?;
    for required in [
        SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH,
        "CY2025-CY2035",
        "Table VI.G1",
        "What remains blocked",
        "does not make the Social Security lane solver-ready or rate-ready",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security taxable payroll reader must cite required text: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_oasdi_receipt_yield_boundary(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH,
        SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security OASDI receipt-yield boundary artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH}: {err}"
                )
            })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")?
        != "social-security-oasdi-receipt-yield-boundary:fy2025-cy2025:v1"
        || string_field(&record, "record_family")? != "social_security_oasdi_receipt_yield_boundary"
        || int_field(&record, "pulse")? != 205
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "dedicated_receipt_anchors_path")?
            != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&record, "taxable_payroll_base_bridge_path")?
            != SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH
        || string_field(&record, "current_law_path_path")?
            != SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH
    {
        return Err("Social Security OASDI receipt-yield boundary identity failed".to_string());
    }

    let year_boundary = record
        .get("year_basis_boundary")
        .ok_or("Social Security OASDI year basis boundary")?;
    if string_field(year_boundary, "omb_receipt_anchor_year_basis")? != "fiscal_year_2025"
        || string_field(year_boundary, "ssa_taxable_payroll_year_basis")? != "calendar_year_2025"
        || !bool_field(year_boundary, "may_compare_as_context")?
        || bool_field(year_boundary, "may_treat_as_reconciled_solver_yield")?
        || !string_field(year_boundary, "required_next_bridge")?
            .contains("calendar-year to fiscal-year")
    {
        return Err("Social Security OASDI year boundary failed".to_string());
    }

    let omb = record
        .get("omb_fy2025_receipt_anchor")
        .ok_or("Social Security OASDI OMB anchor")?;
    let oasi = int_field(omb, "oasi_anchor_musd")?;
    let di = int_field(omb, "di_anchor_musd")?;
    let oasdi = int_field(omb, "oasdi_anchor_sum_musd")?;
    if oasi != 1_097_382
        || di != 186_354
        || oasi + di != oasdi
        || oasdi != 1_283_736
        || bool_field(omb, "may_populate_solver_yield")?
    {
        return Err("Social Security OASDI OMB receipt anchor failed".to_string());
    }

    let ssa = record
        .get("ssa_cy2025_taxable_payroll_yield_context")
        .ok_or("Social Security OASDI SSA yield context")?;
    let taxable_payroll = int_field(ssa, "taxable_payroll_musd")?;
    let rate = number_field(ssa, "combined_oasdi_payroll_tax_rate_percent")?;
    let computed_yield = int_field(ssa, "computed_payroll_tax_yield_musd")?;
    if taxable_payroll != 10_562_000
        || (rate - 12.4).abs() > 0.001
        || ((taxable_payroll as f64) * rate / 100.0).round() as i64 != computed_yield
        || computed_yield != 1_309_688
        || bool_field(ssa, "may_populate_solver_yield")?
    {
        return Err("Social Security OASDI SSA payroll yield context failed".to_string());
    }

    let comparison = record
        .get("context_comparison")
        .ok_or("Social Security OASDI context comparison")?;
    let diff = int_field(comparison, "omb_anchor_minus_ssa_cy_payroll_yield_musd")?;
    let abs_diff = int_field(comparison, "absolute_difference_musd")?;
    if diff != oasdi - computed_yield
        || abs_diff != diff.abs()
        || abs_diff != 25_952
        || (number_field(comparison, "absolute_difference_percent_of_omb_anchor")?
            - ((abs_diff as f64) / (oasdi as f64) * 100.0))
            .abs()
            > 0.0000001
        || (number_field(comparison, "omb_anchor_as_percent_of_ssa_cy_payroll_yield")?
            - ((oasdi as f64) / (computed_yield as f64) * 100.0))
            .abs()
            > 0.0000001
        || bool_field(comparison, "ready_for_solver")?
    {
        return Err("Social Security OASDI context comparison failed".to_string());
    }

    let requirements = record
        .get("remaining_bridge_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security OASDI remaining bridge requirements")?;
    if requirements.len() != 4 {
        return Err(
            "Social Security OASDI boundary must list 4 remaining requirements".to_string(),
        );
    }
    for requirement in requirements {
        if bool_field(requirement, "ready")? {
            return Err("Social Security OASDI remaining requirements must be unready".to_string());
        }
    }

    let status = record
        .get("bridge_status")
        .ok_or("Social Security OASDI bridge status")?;
    for ready in [
        "omb_fy2025_receipt_anchor_ready",
        "ssa_cy2025_taxable_payroll_context_ready",
        "context_comparison_ready",
    ] {
        if !bool_field(status, ready)? {
            return Err(format!(
                "Social Security OASDI boundary status must be true: {ready}"
            ));
        }
    }
    for blocked in [
        "calendar_to_fiscal_bridge_ready",
        "omb_row_perimeter_reconciliation_ready",
        "current_law_yield_for_solver_ready",
        "reform_yield_ready",
        "rate_ready",
        "public_card_ready",
    ] {
        if bool_field(status, blocked)? {
            return Err(format!(
                "Social Security OASDI boundary status must remain false: {blocked}"
            ));
        }
    }

    let blocked_outputs = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security OASDI blocked outputs")?;
    for key in [
        "fiscal_year_taxable_payroll_yield",
        "omb_row_perimeter_reconciliation",
        "current_law_yield_for_solver",
        "reform_yield",
        "solver_receipt_row",
        "rate_calculation",
        "public_rate_card",
        "gross_savings",
        "net_savings",
        "balanced_budget_claim",
    ] {
        if blocked_outputs.get(key) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security OASDI boundary blocked output must stay null: {key}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .ok_or("Social Security OASDI boundary claim booleans")?;
    for ready in [
        "omb_fy2025_receipt_anchor_ready",
        "ssa_cy2025_taxable_payroll_context_ready",
        "context_comparison_ready",
    ] {
        if !bool_field(claims, ready)? {
            return Err(format!(
                "Social Security OASDI boundary claim must be true: {ready}"
            ));
        }
    }
    for blocked in [
        "calendar_to_fiscal_bridge_ready",
        "omb_row_perimeter_reconciliation_ready",
        "current_law_yield_for_solver_ready",
        "reform_yield_ready",
        "solver_receipt_row_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if bool_field(claims, blocked)? {
            return Err(format!(
                "Social Security OASDI boundary claim must remain false: {blocked}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_READER_PATH}: {err}")
    })?;
    for required in [
        SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH,
        "OMB FY2025",
        "SSA CY2025",
        "not interchangeable",
        "not a rate calculation or solver input",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security OASDI receipt-yield reader must cite required text: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_benefit_adequacy_context_bridge(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_JSON_PATH,
        SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_SCHEMA_PATH,
        SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security benefit adequacy bridge artifact: {path}"
            ));
        }
    }

    for (path, byte_count, checksum) in [
        (
            PENSION_REPLACEMENT_GROSS_RAW_PATH,
            1140,
            PENSION_REPLACEMENT_GROSS_RAW_SHA256,
        ),
        (
            PENSION_REPLACEMENT_NET_RAW_PATH,
            1134,
            PENSION_REPLACEMENT_NET_RAW_SHA256,
        ),
    ] {
        let raw = root.join(path);
        if !raw.exists()
            || raw.metadata().map_err(|e| e.to_string())?.len() != byte_count
            || sha256_file(&raw)? != checksum
        {
            return Err(format!(
                "Social Security benefit adequacy raw custody failed: {path}"
            ));
        }
    }

    let panel_text = fs::read_to_string(root.join(PENSION_REPLACEMENT_PANEL_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let panel: serde_json::Value = serde_json::from_str(&panel_text).map_err(|e| e.to_string())?;
    let usa = panel
        .get("country_records")
        .and_then(serde_json::Value::as_array)
        .and_then(|records| {
            records
                .iter()
                .find(|record| string_field(record, "country_code").as_deref() == Ok("USA"))
        })
        .ok_or("Social Security benefit adequacy USA panel row")?;
    if (number_field(usa, "gross_replacement_rate_percent")? - 39.7).abs() > 0.000001
        || (number_field(usa, "net_replacement_rate_percent")? - 51.3).abs() > 0.000001
        || string_field(usa, "observation_status")? != "modeled"
    {
        return Err("Social Security benefit adequacy source value failed".to_string());
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "social-security-benefit-adequacy-context-bridge:v1"
        || string_field(&record, "record_family")?
            != "social_security_benefit_adequacy_context_bridge"
        || int_field(&record, "pulse")? != 208
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_rollup_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "existing_context_artifact_path")?
            != PENSION_REPLACEMENT_PANEL_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-social-security-benefit-adequacy-floors"
    {
        return Err("Social Security benefit adequacy bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("Social Security benefit adequacy bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_captured_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Social Security benefit adequacy scope should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_downloads_performed",
        "closure_gate_ready",
        "threshold_values_may_be_populated",
        "floor_values_may_be_populated",
        "pass_fail_findings_may_be_populated",
        "solver_inputs_may_be_populated",
        "rates_may_be_calculated",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security benefit adequacy scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "international mandatory-scheme replacement-rate context"
        || !string_field(scope, "unclosed_component")?
            .contains("domestic Social Security benefit adequacy")
    {
        return Err("Social Security benefit adequacy scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("Social Security benefit adequacy source custody")?;
    if string_field(custody, "source_id")? != "SRC-OECD-PAG-PENSION-REPLACEMENT-PANEL-2024"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OECD-PAG-PENSION-REPLACEMENT-PANEL-2024.2026-07-15.metadata.md"
        || custody
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Social Security benefit adequacy custody header failed".to_string());
    }
    let raw_files = custody
        .get("raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security benefit adequacy raw files")?;
    if raw_files.len() != 2 {
        return Err("Social Security benefit adequacy raw file count failed".to_string());
    }
    for (measure, path, byte_count, checksum) in [
        (
            "gross_replacement_rate",
            PENSION_REPLACEMENT_GROSS_RAW_PATH,
            1140,
            PENSION_REPLACEMENT_GROSS_RAW_SHA256,
        ),
        (
            "net_replacement_rate",
            PENSION_REPLACEMENT_NET_RAW_PATH,
            1134,
            PENSION_REPLACEMENT_NET_RAW_SHA256,
        ),
    ] {
        let file = raw_files
            .iter()
            .find(|file| string_field(file, "measure").as_deref() == Ok(measure))
            .ok_or_else(|| {
                format!("Social Security benefit adequacy raw file missing {measure}")
            })?;
        if string_field(file, "raw_artifact_path")? != path
            || int_field(file, "raw_byte_count")? != byte_count
            || string_field(file, "raw_sha256")? != checksum
        {
            return Err(format!(
                "Social Security benefit adequacy raw file failed: {measure}"
            ));
        }
    }

    let values = record
        .get("context_values")
        .ok_or("Social Security benefit adequacy context values")?;
    let us = values
        .get("primary_us_context")
        .ok_or("Social Security benefit adequacy US context")?;
    if int_field(values, "model_entry_year")? != 2024
        || string_field(values, "unit")? != "percent"
        || int_field(values, "country_count")? != 12
        || int_field(values, "observed_country_count")? != 11
        || string_field(us, "country_code")? != "USA"
        || (number_field(us, "gross_replacement_rate_percent")? - 39.7).abs() > 0.000001
        || (number_field(us, "net_replacement_rate_percent")? - 51.3).abs() > 0.000001
        || string_field(us, "observation_status")? != "modeled"
        || !string_field(values, "floor_use_boundary")?
            .contains("not a Social Security benefit-adequacy threshold")
    {
        return Err("Social Security benefit adequacy context values failed".to_string());
    }

    let requirements = record
        .get("remaining_floor_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security benefit adequacy remaining requirements")?;
    if requirements.len() != 6 {
        return Err("Social Security benefit adequacy requirements count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security benefit adequacy blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security benefit adequacy blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security benefit adequacy claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security benefit adequacy claim bool")?;
        match field.as_str() {
            "benefit_adequacy_context_bridge_published"
            | "international_pension_replacement_context_ready" => {
                if !observed {
                    return Err(format!(
                        "Social Security benefit adequacy claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "Social Security benefit adequacy downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only existing OECD modeled pension replacement-rate context",
        "not domestic Social Security benefit adequacy custody",
        "not benefit-adequacy floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security benefit adequacy warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_JSON_PATH,
        "1,140",
        PENSION_REPLACEMENT_GROSS_RAW_SHA256,
        "1,134",
        PENSION_REPLACEMENT_NET_RAW_SHA256,
        "39.7 percent gross",
        "51.3 percent net",
        "international modeled pension replacement-rate context only",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security benefit adequacy reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_old_age_poverty_context_bridge(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH,
        SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security old-age poverty bridge artifact: {path}"
            ));
        }
    }

    let raw = root.join(IDD_OLD_AGE_POVERTY_RAW_PATH);
    if !raw.exists()
        || raw.metadata().map_err(|e| e.to_string())?.len() != 5061
        || sha256_file(&raw)? != IDD_OLD_AGE_POVERTY_RAW_SHA256
    {
        return Err("Social Security old-age poverty raw custody failed".to_string());
    }

    let panel_text = fs::read_to_string(root.join(AGE_RELATIVE_POVERTY_PANEL_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let panel: serde_json::Value = serde_json::from_str(&panel_text).map_err(|e| e.to_string())?;
    let usa_older = panel
        .get("country_records")
        .and_then(serde_json::Value::as_array)
        .and_then(|records| {
            records
                .iter()
                .find(|record| string_field(record, "country_code").as_deref() == Ok("USA"))
        })
        .ok_or("Social Security old-age poverty USA panel row")?;
    if int_field(usa_older, "older_people_reference_year")? != 2023
        || (number_field(usa_older, "older_people_poverty_percent")? - 22.874).abs() > 0.000001
        || string_field(usa_older, "older_people_observation_status")? != "actual"
    {
        return Err("Social Security old-age poverty source value failed".to_string());
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "social-security-old-age-poverty-context-bridge:v1"
        || string_field(&record, "record_family")?
            != "social_security_old_age_poverty_context_bridge"
        || int_field(&record, "pulse")? != 207
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_rollup_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "existing_context_artifact_path")?
            != AGE_RELATIVE_POVERTY_PANEL_JSON_PATH
        || string_field(&record, "target_work_item_id")? != "capture-old-age-poverty-floor-values"
    {
        return Err("Social Security old-age poverty bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("Social Security old-age poverty bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_captured_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Social Security old-age poverty scope should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_downloads_performed",
        "closure_gate_ready",
        "threshold_values_may_be_populated",
        "floor_values_may_be_populated",
        "pass_fail_findings_may_be_populated",
        "solver_inputs_may_be_populated",
        "rates_may_be_calculated",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security old-age poverty scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "international old-age relative-income-poverty context"
        || !string_field(scope, "unclosed_component")?.contains("domestic old-age poverty")
    {
        return Err("Social Security old-age poverty scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("Social Security old-age poverty source custody")?;
    if string_field(custody, "source_id")? != "SRC-OECD-IDD-AGE-POVERTY-PANELS"
        || string_field(custody, "raw_artifact_path")? != IDD_OLD_AGE_POVERTY_RAW_PATH
        || int_field(custody, "raw_byte_count")? != 5061
        || string_field(custody, "raw_sha256")? != IDD_OLD_AGE_POVERTY_RAW_SHA256
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OECD-IDD-AGE-POVERTY-PANELS.2026-07-15.metadata.md"
        || custody
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Social Security old-age poverty custody fields failed".to_string());
    }

    let values = record
        .get("context_values")
        .ok_or("Social Security old-age poverty context values")?;
    let us = values
        .get("primary_us_context")
        .ok_or("Social Security old-age poverty US context")?;
    if string_field(values, "unit")? != "percent"
        || int_field(values, "country_count")? != 12
        || int_field(values, "observed_country_count")? != 11
        || string_field(us, "country_code")? != "USA"
        || int_field(us, "reference_year")? != 2023
        || (number_field(us, "old_age_relative_poverty_percent")? - 22.874).abs() > 0.000001
        || string_field(us, "observation_status")? != "actual"
    {
        return Err("Social Security old-age poverty context values failed".to_string());
    }

    let requirements = record
        .get("remaining_floor_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security old-age poverty remaining requirements")?;
    if requirements.len() != 6 {
        return Err("Social Security old-age poverty requirements count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security old-age poverty blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security old-age poverty blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security old-age poverty claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security old-age poverty claim bool")?;
        match field.as_str() {
            "old_age_poverty_context_bridge_published"
            | "international_old_age_poverty_context_ready" => {
                if !observed {
                    return Err(format!(
                        "Social Security old-age poverty claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "Social Security old-age poverty downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only existing OECD international old-age relative-poverty context",
        "not domestic old-age poverty custody",
        "not old-age poverty floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security old-age poverty warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        "5,061 bytes",
        IDD_OLD_AGE_POVERTY_RAW_SHA256,
        "22.874 percent",
        "international old-age relative-poverty context only",
        "not domestic old-age poverty custody",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security old-age poverty reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_domestic_old_age_poverty_context_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH,
        SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security domestic old-age poverty bridge artifact: {path}"
            ));
        }
    }

    for (path, byte_count, checksum) in [
        (
            CENSUS_P60_287_TABLE_A3_RAW_PATH,
            57388,
            CENSUS_P60_287_TABLE_A3_RAW_SHA256,
        ),
        (
            CENSUS_P60_287_TABLE_B2_RAW_PATH,
            43484,
            CENSUS_P60_287_TABLE_B2_RAW_SHA256,
        ),
        (
            CENSUS_P60_287_TABLE_B7_RAW_PATH,
            14272,
            CENSUS_P60_287_TABLE_B7_RAW_SHA256,
        ),
        (
            CENSUS_P60_287_INCOME_TO_POVERTY_RAW_PATH,
            14948,
            CENSUS_P60_287_INCOME_TO_POVERTY_RAW_SHA256,
        ),
    ] {
        let raw = root.join(path);
        if !raw.exists()
            || raw.metadata().map_err(|e| e.to_string())?.len() != byte_count
            || sha256_file(&raw)? != checksum
        {
            return Err(format!(
                "Social Security domestic old-age poverty raw custody failed: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "social-security-domestic-old-age-poverty-context-bridge:v1"
        || string_field(&record, "record_family")?
            != "social_security_domestic_old_age_poverty_context_bridge"
        || int_field(&record, "pulse")? != 209
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_rollup_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "census_child_poverty_income_capture_gap_path")?
            != INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH
        || string_field(&record, "target_work_item_id")? != "capture-old-age-poverty-floor-values"
    {
        return Err("Social Security domestic old-age poverty bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("Social Security domestic old-age poverty bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_captured_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Social Security domestic old-age poverty scope should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_downloads_performed",
        "closure_gate_ready",
        "threshold_values_may_be_populated",
        "floor_values_may_be_populated",
        "pass_fail_findings_may_be_populated",
        "solver_inputs_may_be_populated",
        "rates_may_be_calculated",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security domestic old-age poverty scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "domestic 65-plus poverty and near-poverty context"
        || !string_field(scope, "unclosed_component")?.contains("floor threshold rationale")
    {
        return Err("Social Security domestic old-age poverty scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("Social Security domestic old-age poverty custody")?;
    if string_field(custody, "source_id")? != "SRC-CENSUS-P60-287-POVERTY-2024"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-CENSUS-P60-287-POVERTY-2024.2026-07-24.metadata.md"
        || custody
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Social Security domestic old-age poverty custody header failed".to_string());
    }
    let raw_files = custody
        .get("raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security domestic old-age poverty raw files")?;
    if raw_files.len() != 4 {
        return Err("Social Security domestic old-age poverty raw file count failed".to_string());
    }
    for (table_id, path, byte_count, checksum) in [
        (
            "Table A-3",
            CENSUS_P60_287_TABLE_A3_RAW_PATH,
            57388,
            CENSUS_P60_287_TABLE_A3_RAW_SHA256,
        ),
        (
            "Table B-2",
            CENSUS_P60_287_TABLE_B2_RAW_PATH,
            43484,
            CENSUS_P60_287_TABLE_B2_RAW_SHA256,
        ),
        (
            "Table B-7",
            CENSUS_P60_287_TABLE_B7_RAW_PATH,
            14272,
            CENSUS_P60_287_TABLE_B7_RAW_SHA256,
        ),
        (
            "Income-to-Poverty Ratios",
            CENSUS_P60_287_INCOME_TO_POVERTY_RAW_PATH,
            14948,
            CENSUS_P60_287_INCOME_TO_POVERTY_RAW_SHA256,
        ),
    ] {
        let file = raw_files
            .iter()
            .find(|file| string_field(file, "table_id").as_deref() == Ok(table_id))
            .ok_or_else(|| {
                format!("Social Security domestic old-age poverty raw file missing {table_id}")
            })?;
        if string_field(file, "raw_artifact_path")? != path
            || int_field(file, "raw_byte_count")? != byte_count
            || string_field(file, "raw_sha256")? != checksum
        {
            return Err(format!(
                "Social Security domestic old-age poverty raw file failed: {table_id}"
            ));
        }
    }

    let values = record
        .get("context_values")
        .ok_or("Social Security domestic old-age poverty context values")?;
    let official = values
        .get("official_poverty_measure")
        .ok_or("Social Security domestic old-age poverty official values")?;
    let spm = values
        .get("supplemental_poverty_measure")
        .ok_or("Social Security domestic old-age poverty SPM values")?;
    let effect = values
        .get("spm_social_security_element_effect")
        .ok_or("Social Security domestic old-age poverty SPM effect")?;
    let ratios = values
        .get("official_income_to_poverty_ratio_context")
        .ok_or("Social Security domestic old-age poverty ratio context")?;
    if int_field(values, "reference_year")? != 2024
        || string_field(values, "age_group")? != "65 years and over"
        || int_field(official, "population_thousands")? != 61490
        || int_field(official, "below_poverty_thousands")? != 6108
        || (number_field(official, "below_poverty_percent")? - 9.9).abs() > 0.000001
        || int_field(spm, "population_thousands")? != 61490
        || int_field(spm, "below_poverty_thousands")? != 9223
        || (number_field(spm, "below_poverty_percent")? - 15.0).abs() > 0.000001
        || int_field(effect, "effect_on_65_plus_spm_poverty_thousands")? != -20100
        || int_field(effect, "margin_of_error_thousands")? != 422
        || int_field(ratios, "under_125_percent_threshold_thousands")? != 8703
        || (number_field(ratios, "under_125_percent_threshold_percent")? - 14.2).abs() > 0.000001
        || int_field(ratios, "under_150_percent_threshold_thousands")? != 11640
        || (number_field(ratios, "under_150_percent_threshold_percent")? - 18.9).abs() > 0.000001
        || int_field(ratios, "under_200_percent_threshold_thousands")? != 17290
        || (number_field(ratios, "under_200_percent_threshold_percent")? - 28.1).abs() > 0.000001
        || !string_field(values, "floor_use_boundary")?
            .contains("not an old-age poverty floor threshold")
    {
        return Err("Social Security domestic old-age poverty context values failed".to_string());
    }

    let requirements = record
        .get("remaining_floor_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security domestic old-age poverty remaining requirements")?;
    if requirements.len() != 5 {
        return Err(
            "Social Security domestic old-age poverty requirements count failed".to_string(),
        );
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security domestic old-age poverty blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security domestic old-age poverty blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security domestic old-age poverty claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security domestic old-age poverty claim bool")?;
        match field.as_str() {
            "domestic_old_age_poverty_context_bridge_published"
            | "census_old_age_poverty_context_ready" => {
                if !observed {
                    return Err(format!(
                        "Social Security domestic old-age poverty claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "Social Security domestic old-age poverty downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only existing Census domestic 65-plus old-age poverty",
        "not old-age poverty measure selection",
        "not old-age poverty floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security domestic old-age poverty warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        "57,388",
        CENSUS_P60_287_TABLE_A3_RAW_SHA256,
        "43,484",
        CENSUS_P60_287_TABLE_B2_RAW_SHA256,
        "14,272",
        CENSUS_P60_287_TABLE_B7_RAW_SHA256,
        "14,948",
        CENSUS_P60_287_INCOME_TO_POVERTY_RAW_SHA256,
        "6.108 million, 9.9 percent",
        "9.223 million, 15.0 percent",
        "-20.100 million",
        "Census domestic 65-plus old-age poverty context only",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security domestic old-age poverty reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_administration_service_context_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_JSON_PATH,
        SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_SCHEMA_PATH,
        SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security administration service bridge artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "social-security-administration-service-context-bridge:v1"
        || string_field(&record, "record_family")?
            != "social_security_administration_service_context_bridge"
        || int_field(&record, "pulse")? != 210
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "source_capture_status_rollup_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-ssa-administration-transition-capacity"
    {
        return Err("Social Security administration service bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("Social Security administration service bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_browser_visible_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Social Security administration service scope should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_downloads_performed",
        "closure_gate_ready",
        "floor_values_may_be_populated",
        "pass_fail_findings_may_be_populated",
        "solver_inputs_may_be_populated",
        "rates_may_be_calculated",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Social Security administration service scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "browser-visible May 2026 service-channel and processing-time context"
        || !string_field(scope, "unclosed_component")?.contains("payment accuracy context")
    {
        return Err("Social Security administration service scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("Social Security administration service custody")?;
    if string_field(custody, "source_id")? != "SRC-SSA-PERFORMANCE-DASHBOARD-2026-06-09"
        || string_field(custody, "publisher")? != "Social Security Administration"
        || string_field(custody, "source_url")? != "https://www.ssa.gov/ssa-performance"
        || string_field(custody, "retrieval_date")? != "2026-07-25"
        || string_field(custody, "last_updated_displayed")? != "2026-06-09"
        || bool_field(custody, "local_raw_byte_custody_ready")?
        || !bool_field(custody, "browser_visible_context_ready")?
    {
        return Err("Social Security administration service custody fields failed".to_string());
    }
    let access = custody
        .get("access_boundary")
        .ok_or("Social Security administration service access boundary")?;
    if string_field(access, "command_line_fetch_status")? != "blocked_access_denied"
        || string_field(access, "attempted_url")? != "https://www.ssa.gov/ssa-performance"
        || !bool_field(access, "boundary_recorded")?
    {
        return Err("Social Security administration service access boundary failed".to_string());
    }

    let values = record
        .get("context_values")
        .ok_or("Social Security administration service context values")?;
    let channels = values
        .get("service_channels")
        .ok_or("Social Security administration service channels")?;
    let phone = values
        .get("national_800_number")
        .ok_or("Social Security administration service phone values")?;
    let offices = values
        .get("field_offices")
        .ok_or("Social Security administration service field offices")?;
    let processing = values
        .get("claims_and_appeals_processing_context")
        .ok_or("Social Security administration service processing")?;
    if string_field(values, "display_period")?
        != "May 2026 and fiscal year to date October through May where labeled"
        || (number_field(
            channels,
            "year_to_date_customer_contacts_online_and_phone_percent",
        )? - 96.3)
            .abs()
            > 0.000001
        || (number_field(
            channels,
            "estimated_public_wait_time_reduction_hours_millions",
        )? - 14.2)
            .abs()
            > 0.000001
        || int_field(phone, "may_2025_average_speed_of_answer_minutes")? != 11
        || int_field(phone, "may_2026_average_speed_of_answer_minutes")? != 5
        || int_field(phone, "may_2026_answer_rate_percent")? != 89
        || int_field(offices, "fy2026_average_combined_wait_time_minutes")? != 21
        || int_field(
            processing,
            "retirement_and_survivors_most_claims_processed_within_days",
        )? != 14
        || int_field(
            processing,
            "initial_disability_decisions_faster_than_may_2025_days",
        )? != 42
        || int_field(processing, "hearings_average_wait_months")? != 9
        || int_field(processing, "hearings_processing_goal_days")? != 270
        || !string_field(values, "floor_use_boundary")?
            .contains("not transition-capacity floor values")
    {
        return Err("Social Security administration service context values failed".to_string());
    }

    let requirements = record
        .get("remaining_floor_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security administration service remaining requirements")?;
    if requirements.len() != 9 {
        return Err("Social Security administration service requirements count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security administration service blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security administration service blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security administration service claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security administration service claim bool")?;
        match field.as_str() {
            "ssa_administration_service_context_bridge_published"
            | "ssa_browser_visible_service_context_ready" => {
                if !observed {
                    return Err(format!(
                        "Social Security administration service claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "Social Security administration service downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only browser-visible SSA May 2026 service-channel",
        "not local SSA raw-byte custody",
        "not complete claims-processing values",
        "not payment accuracy values",
        "not transition capacity floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security administration service warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_JSON_PATH,
        "June 9, 2026",
        "96.3 percent",
        "14.2 million hours",
        "11 minutes",
        "5 minutes",
        "89 percent",
        "42 days faster",
        "browser-visible SSA service context only",
        "command-line access boundary recorded",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security administration service reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_old_age_poverty_floor_value_packet(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH,
        SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security old-age poverty floor value packet artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "social-security-old-age-poverty-floor-value-packet:v1"
        || string_field(&record, "record_family")?
            != "social_security_old_age_poverty_floor_value_packet"
        || int_field(&record, "pulse")? != 211
        || string_field(&record, "lane_id")? != "social-security"
        || string_field(&record, "floor_id")? != "old_age_poverty"
        || string_field(&record, "floor_definition_packet_path")?
            != SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "domestic_old_age_poverty_context_bridge_path")?
            != SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&record, "international_old_age_poverty_context_bridge_path")?
            != SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err(
            "Social Security old-age poverty floor value packet identity failed".to_string(),
        );
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("Social Security old-age poverty threshold rationale")?;
    if string_field(threshold, "rationale_id")? != "no-regression-from-domestic-65-plus-spm-poverty"
        || string_field(threshold, "selected_measure")?
            != "Census Supplemental Poverty Measure poverty rate for people 65 years and over"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_ceiling"
        || (number_field(threshold, "threshold_value")? - 15.0).abs() > 0.000001
        || string_field(threshold, "threshold_unit")? != "percent"
        || string_field(threshold, "source_table")? != "Census P60-287 Table B-2"
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("Social Security old-age poverty threshold rationale failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("Social Security old-age poverty baseline values")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("Social Security old-age poverty primary baseline")?;
    if int_field(baseline, "reference_year")? != 2024
        || string_field(primary, "measure")? != "65-plus SPM poverty rate"
        || (number_field(primary, "value")? - 15.0).abs() > 0.000001
        || int_field(primary, "population_thousands")? != 61490
        || int_field(primary, "below_poverty_thousands")? != 9223
        || (number_field(primary, "margin_of_error_percentage_points")? - 0.5).abs() > 0.000001
        || string_field(primary, "source_path")?
            != SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
    {
        return Err("Social Security old-age poverty primary baseline failed".to_string());
    }
    let supporting = baseline
        .get("supporting_domestic_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security old-age poverty supporting context")?;
    if supporting.len() != 5 {
        return Err("Social Security old-age poverty supporting context count failed".to_string());
    }
    let international = baseline
        .get("international_context_not_threshold")
        .ok_or("Social Security old-age poverty international context")?;
    if (number_field(international, "value")? - 22.874).abs() > 0.000001
        || string_field(international, "boundary")?
            != "International comparator context only; not the selected Taxlane floor threshold."
    {
        return Err("Social Security old-age poverty international context failed".to_string());
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "Social Security old-age poverty floor value packet field must stay null: {field}"
            ));
        }
    }

    let readiness = record
        .get("readiness_status")
        .ok_or("Social Security old-age poverty readiness")?;
    for field in [
        "threshold_rationale_ready",
        "threshold_value_populated",
        "baseline_value_ready",
    ] {
        if !bool_field(readiness, field)? {
            return Err(format!(
                "Social Security old-age poverty readiness should be true: {field}"
            ));
        }
    }
    for field in [
        "policy_value_ready",
        "stress_value_ready",
        "pass_fail_ready",
        "lower_cost_scenario_admissible",
        "solver_ready",
        "rate_ready",
    ] {
        if bool_field(readiness, field)? {
            return Err(format!(
                "Social Security old-age poverty readiness must remain false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security old-age poverty floor blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Social Security old-age poverty floor blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security old-age poverty floor claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Social Security old-age poverty floor claim bool")?;
        match field.as_str() {
            "old_age_poverty_floor_value_packet_published"
            | "threshold_rationale_ready"
            | "threshold_value_populated"
            | "baseline_value_ready" => {
                if !observed {
                    return Err(format!(
                        "Social Security old-age poverty floor claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "Social Security old-age poverty floor downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "draft no-regression old-age poverty floor threshold",
        "not policy values",
        "not stress values",
        "not pass/fail evidence",
        "not lower-cost scenario admissibility",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "Social Security old-age poverty floor warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        SOCIAL_SECURITY_OLD_AGE_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH,
        "15.0 percent",
        "9.223 million",
        "61.490 million",
        "-20.100 million",
        "draft no-regression old-age poverty floor threshold",
        "policy and stress values remain null",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security old-age poverty floor reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_social_security_source_capture_status_rollup(root: &Path) -> Result<(), String> {
    for path in [
        SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Social Security source capture status rollup artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH}: {err}"
                )
            })?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH}: {err}")
    })?;

    if string_field(&rollup, "record_id")? != "social-security-source-capture-status-rollup:v1"
        || string_field(&rollup, "record_family")? != "social_security_source_capture_status_rollup"
        || int_field(&rollup, "pulse")? != 206
        || string_field(&rollup, "lane_id")? != "social-security"
        || string_field(&rollup, "source_capture_queue_path")?
            != SOCIAL_SECURITY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&rollup, "source_readiness_gap_path")?
            != SOCIAL_SECURITY_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(&rollup, "trustees_source_capture_status_path")?
            != SOCIAL_SECURITY_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&rollup, "current_law_path_path")?
            != SOCIAL_SECURITY_OASDI_FY2025_2035_CURRENT_LAW_PATH_JSON_PATH
        || string_field(&rollup, "taxable_payroll_base_bridge_path")?
            != SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH
        || string_field(&rollup, "receipt_yield_boundary_path")?
            != SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH
        || string_field(&rollup, "benefit_adequacy_context_bridge_path")?
            != SOCIAL_SECURITY_BENEFIT_ADEQUACY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&rollup, "old_age_poverty_context_bridge_path")?
            != SOCIAL_SECURITY_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&rollup, "domestic_old_age_poverty_context_bridge_path")?
            != SOCIAL_SECURITY_DOMESTIC_OLD_AGE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&rollup, "administration_service_context_bridge_path")?
            != SOCIAL_SECURITY_ADMINISTRATION_SERVICE_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&rollup, "coverage_matrix_path")? != LANE_FULL_COVERAGE_MATRIX_JSON_PATH
    {
        return Err("Social Security source capture status rollup identity failed".to_string());
    }

    let rules = rollup
        .get("source_capture_rules")
        .ok_or("Social Security source capture rules")?;
    for required_true in [
        "official_sources_only",
        "access_boundary_recorded",
        "browser_verified_official_sources_allowed_for_context",
        "calendar_year_context_cannot_substitute_for_fiscal_year_solver_yield",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if !bool_field(rules, required_true)? {
            return Err(format!(
                "Social Security source capture rule must be true: {required_true}"
            ));
        }
    }
    if bool_field(rules, "local_raw_byte_custody_ready")? {
        return Err("Social Security local raw-byte custody must remain false".to_string());
    }

    let items = rollup
        .get("capture_item_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("Social Security source capture item status")?;
    if items.len() != 6 {
        return Err("Social Security source capture rollup must contain 6 items".to_string());
    }
    let mut observed_items = BTreeSet::new();
    let mut partial_count = 0;
    let mut not_started_count = 0;
    let mut ready_count = 0;
    for item in items {
        let item_id = string_field(item, "work_item_id")?;
        observed_items.insert(item_id);
        let status = string_field(item, "status")?;
        if status.starts_with("partial_") {
            partial_count += 1;
            let evidence_paths = item
                .get("evidence_paths")
                .and_then(serde_json::Value::as_array)
                .ok_or("Social Security partial evidence paths")?;
            if evidence_paths.is_empty() {
                return Err("Social Security partial capture item needs evidence".to_string());
            }
            for path in evidence_paths {
                let path = path
                    .as_str()
                    .ok_or("Social Security evidence path string")?;
                if !root.join(path).exists() {
                    return Err(format!(
                        "Social Security source capture rollup referenced path missing: {path}"
                    ));
                }
            }
        } else if status == "not_started" {
            not_started_count += 1;
        } else {
            return Err(format!(
                "Social Security source capture rollup unexpected status: {status}"
            ));
        }
        if bool_field(item, "ready")? {
            ready_count += 1;
        }
    }
    let expected_items = [
        "capture-oasdi-annual-fund-path",
        "capture-oasdi-75-year-solvency-path",
        "capture-oasdi-taxable-payroll-base",
        "capture-social-security-benefit-adequacy-floors",
        "capture-old-age-poverty-floor-values",
        "capture-ssa-administration-transition-capacity",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_items != expected_items
        || partial_count != 6
        || not_started_count != 0
        || ready_count != 0
    {
        return Err("Social Security source capture item status counts failed".to_string());
    }

    let gate_updates = rollup
        .get("coverage_gate_updates")
        .ok_or("Social Security coverage gate updates")?;
    for (gate, updated_status) in [
        ("current_law_baseline", "partial"),
        ("source_custody", "partial"),
        ("receipt_rate_bridge", "partial"),
    ] {
        let update = gate_updates
            .get(gate)
            .ok_or_else(|| format!("missing Social Security gate update {gate}"))?;
        if string_field(update, "updated_status")? != updated_status {
            return Err(format!(
                "Social Security gate update {gate} must be {updated_status}"
            ));
        }
    }

    let aggregate = rollup
        .get("aggregate_status")
        .ok_or("Social Security source capture aggregate")?;
    if int_field(aggregate, "capture_item_count")? != 6
        || int_field(aggregate, "items_with_partial_progress")? != 6
        || int_field(aggregate, "items_not_started")? != 0
        || int_field(aggregate, "items_ready_count")? != 0
        || int_field(aggregate, "floor_context_artifacts_ready")? != 3
        || int_field(aggregate, "administration_context_artifacts_ready")? != 1
        || int_field(aggregate, "floor_value_artifacts_ready")? != 0
        || int_field(aggregate, "solver_ready_items")? != 0
        || int_field(aggregate, "rate_ready_items")? != 0
        || int_field(aggregate, "savings_ready_items")? != 0
    {
        return Err("Social Security source capture aggregate failed".to_string());
    }

    let blocked_outputs = rollup
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Social Security source capture blocked outputs")?;
    for key in [
        "complete_oasi_di_split_path",
        "complete_75_year_solvency_path",
        "fiscal_year_taxable_payroll_yield",
        "omb_row_perimeter_reconciliation",
        "benefit_adequacy_floor_values",
        "old_age_poverty_floor_values",
        "solver_input",
        "rate_calculation",
        "public_rate_card",
        "gross_savings",
        "net_savings",
        "balanced_budget_claim",
    ] {
        if blocked_outputs.get(key) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Social Security source capture blocked output must stay null: {key}"
            ));
        }
    }

    let claims = rollup
        .get("claim_booleans")
        .ok_or("Social Security source capture claim booleans")?;
    for ready in [
        "social_security_source_capture_status_rollup_published",
        "current_law_baseline_gate_partial",
        "receipt_rate_bridge_gate_partial",
        "benefit_adequacy_context_bridge_published",
        "international_pension_replacement_context_ready",
        "old_age_poverty_context_bridge_published",
        "international_old_age_poverty_context_ready",
        "domestic_old_age_poverty_context_bridge_published",
        "census_old_age_poverty_context_ready",
        "ssa_administration_service_context_bridge_published",
        "ssa_browser_visible_service_context_ready",
    ] {
        if !bool_field(claims, ready)? {
            return Err(format!(
                "Social Security source capture claim must be true: {ready}"
            ));
        }
    }
    for blocked in [
        "all_source_capture_items_ready",
        "local_raw_byte_custody_ready",
        "complete_oasi_di_split_path_ready",
        "complete_75_year_solvency_path_ready",
        "floor_values_ready",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "solver_input_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if bool_field(claims, blocked)? {
            return Err(format!(
                "Social Security source capture claim must remain false: {blocked}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH}: {err}")
    })?;
    for required in [
        SOCIAL_SECURITY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        "partial current-law baseline context",
        "partial taxable-payroll base context",
        "OECD modeled pension replacement-rate context",
        "international modeled pension replacement-rate context only",
        "OECD old-age relative-poverty context",
        "international relative-poverty context only",
        "Census domestic 65-plus old-age poverty context only",
        "browser-visible SSA service context only",
        "What remains blocked",
        "partial, not complete",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Social Security source capture rollup reader missing required text: {required}"
            ));
        }
    }

    Ok(())
}

