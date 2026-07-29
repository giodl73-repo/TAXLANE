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

pub(crate) fn validate_income_security_family_outcome_floor_definition_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 164
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(&record, "defense_outcome_floor_definition_packet_path")?
            != DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("income-security/family floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family floor status {field} must be true"
            ));
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
                "income-security/family floor status {field} must be false"
            ));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family floor definition policy")?;
    for field in [
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "benefit_package_and_take_up_model_required_before_target_cost",
        "work_transition_and_childcare_access_required_before_solver_use",
        "international_differences_not_savings",
        "no_fraud_inference",
        "federal_state_local_translation_required_before_solver_use",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family floor policy {field} must be true"
            ));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("income-security/family required floor class count failed".to_string());
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
        return Err("income-security/family required floor class set failed".to_string());
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
                    "income-security/family floor class {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("income-security/family floor class must remain unpassed".to_string());
        }
    }

    let lane_floors = record
        .get("income_security_family_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family-specific floor definitions")?;
    let expected_lane_floors = [
        "child_poverty",
        "material_hardship",
        "formal_childcare_access",
        "work_and_care_transition",
        "benefit_package_take_up_delivery_feasibility",
    ];
    if lane_floors.len() != expected_lane_floors.len() {
        return Err("income-security/family-specific floor count failed".to_string());
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
        return Err("income-security/family-specific floor set failed".to_string());
    }
    for row in lane_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "income-security/family-specific floors must remain null and unpassed".to_string(),
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
        .ok_or("income-security/family floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("income_security_family_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
    {
        return Err("income-security/family floor summary counts failed".to_string());
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
                "income-security/family floor summary {field} must be false"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err(
            "income-security/family floor definition packet publication flag failed".to_string(),
        );
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
        "lower_cost_scenario_admissibility_ready",
        "benefit_package_model_published",
        "take_up_model_published",
        "target_cost_published",
        "federal_effect_published",
        "gross_savings_published",
        "net_savings_published",
        "solver_input_ready",
        "solver_run_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family floor claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This income-security/family floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "No lower-cost income-security/family scenario is admissible until child poverty, material hardship, childcare access, work/care transition, equity, adequacy/resilience, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "A benefit concept is not a benefit package model, take-up model, federal/state/local translation, federal score, target cost, or solver input.",
        "No target cost, federal effect, gross savings, net savings, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a benefit package model",
        "not a take-up model",
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
                "income-security/family floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_source_readiness_gap(root: &Path) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family source readiness gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "income-security-family-source-readiness-gap:v1"
        || string_field(&record, "record_family")? != "income_security_family_source_readiness_gap"
        || int_field(&record, "pulse")? != 190
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(
            &record,
            "income_security_family_outcome_floor_definition_packet_path",
        )? != INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("income-security/family source readiness identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "income_security_family_floor_definition_present",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family source status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "benefit_package_raw_custody_ready",
        "take_up_raw_custody_ready",
        "child_poverty_raw_custody_ready",
        "material_hardship_raw_custody_ready",
        "childcare_access_raw_custody_ready",
        "work_transition_raw_custody_ready",
        "federal_state_local_translation_ready",
        "source_capture_complete",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family source status {field} must be false"
            ));
        }
    }

    let families = record
        .get("required_source_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family required source families")?;
    if families.len() != 6 {
        return Err("income-security/family source family count failed".to_string());
    }
    let expected_families = [
        "OMB budget tables and program account materials",
        "CBO income security and family baseline materials",
        "Census poverty and income tables",
        "HHS childcare, TANF, and family-service administrative series",
        "USDA food security and nutrition assistance administrative series",
        "OECD family, SOCX, Eurostat ESSPROS, and ILO social protection comparator context",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_families = families
        .iter()
        .map(|row| string_field(row, "source_family"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_families != expected_families {
        return Err("income-security/family source family set failed".to_string());
    }
    for row in families {
        if row
            .get("needed_for")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|values| values.is_empty())
            || row
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row
                .get("may_populate_solver_input")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("income-security/family source family must stay blocked".to_string());
        }
    }

    let requirements = record
        .get("source_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source capture requirements")?;
    if requirements.len() != 11 {
        return Err("income-security/family source requirement count failed".to_string());
    }
    for row in requirements {
        if row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "income-security/family source requirements must remain null/false".to_string(),
            );
        }
    }

    let floors = record
        .get("floor_value_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source floor status")?;
    if floors.len() != 5 {
        return Err("income-security/family source floor count failed".to_string());
    }
    for row in floors {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!(
                    "income-security/family source floor {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err("income-security/family source floors must remain unpassed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family source blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family source claim bool")?;
        if matches!(
            field.as_str(),
            "income_security_family_source_readiness_gap_published"
                | "income_security_family_floor_definition_present"
        ) {
            if !observed {
                return Err(format!(
                    "income-security/family source claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "income-security/family source claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "records required source families only",
        "not raw source custody",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family source warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH,
        "OMB budget tables and program account materials",
        "CBO income security and family baseline materials",
        "Census poverty and income tables",
        "HHS childcare, TANF, and family-service administrative series",
        "USDA food security and nutrition assistance administrative series",
        "OECD family, SOCX, Eurostat ESSPROS, and ILO social protection comparator context",
        "not raw source custody",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family source reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_source_capture_queue(root: &Path) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family source capture queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "income-security-family-source-capture-queue:v1"
        || string_field(&record, "record_family")? != "income_security_family_source_capture_queue"
        || int_field(&record, "pulse")? != 191
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "income_security_family_source_readiness_gap_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_outcome_floor_definition_packet_path",
        )? != INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
    {
        return Err("income-security/family source capture queue identity failed".to_string());
    }

    let rules = record
        .get("source_rules")
        .ok_or("income-security/family source capture rules")?;
    for field in [
        "official_sources_only",
        "use_existing_captured_sources_when_available",
        "new_external_downloads_not_performed_in_this_pulse",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "threshold_selection_requires_stronger_model_review",
        "benefit_package_design_requires_stronger_model_review",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "international_spending_differences_are_not_savings",
        "no_fraud_inference_from_comparison_or_administrative_context",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family source capture rule must be true: {field}"
            ));
        }
    }

    let items = record
        .get("capture_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source capture items")?;
    if items.len() != 6 {
        return Err("income-security/family source capture item count failed".to_string());
    }
    let expected = [
        (
            "capture-income-security-federal-program-outlay-perimeter",
            1,
        ),
        ("capture-income-security-cbo-baseline-and-takeup-context", 2),
        ("capture-income-security-child-poverty-income-context", 3),
        (
            "capture-income-security-childcare-family-service-context",
            4,
        ),
        ("capture-income-security-food-hardship-nutrition-context", 5),
        (
            "capture-income-security-international-comparator-context",
            6,
        ),
    ];
    for (work_item_id, priority) in expected {
        let item = items
            .iter()
            .find(|item| string_field(item, "work_item_id").as_deref() == Ok(work_item_id))
            .ok_or_else(|| {
                format!("missing income-security/family capture item: {work_item_id}")
            })?;
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
                "income-security/family source capture item shape failed: {work_item_id}"
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
                    "income-security/family source capture item field must be null: {work_item_id}.{field}"
                ));
            }
        }
        if item.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family source capture item must not be ready: {work_item_id}"
            ));
        }
    }

    let counts = record
        .get("aggregate_status")
        .ok_or("income-security/family source capture aggregate status")?;
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
                "income-security/family source capture aggregate count failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family source capture blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family source capture claim bool")?;
        if field == "income_security_family_source_capture_queue_published" {
            if !observed {
                return Err(
                    "income-security/family source capture publication flag must be true"
                        .to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family source capture claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "names official-source work items only",
        "not raw source custody",
        "not a program outlay perimeter",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family source capture warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        "Federal program outlay perimeter",
        "CBO baseline and take-up context",
        "Child poverty and income context",
        "Childcare and family-service context",
        "Food hardship and nutrition context",
        "International comparator context",
        "not raw source custody",
        "not a program outlay perimeter",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family source capture reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_source_capture_status_rollup(root: &Path) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family source capture status rollup artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-source-capture-status-rollup:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_source_capture_status_rollup"
        || int_field(&record, "pulse")? != 201
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "income_security_family_source_readiness_gap_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(&record, "income_security_family_source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(
            &record,
            "income_security_family_federal_program_perimeter_bridge_path",
        )? != INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH
        || string_field(
            &record,
            "income_security_family_cbo_baseline_takeup_capture_gap_path",
        )? != INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_child_relative_poverty_context_bridge_path",
        )? != INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(
            &record,
            "income_security_family_socx_family_benefit_comparator_bridge_path",
        )? != INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH
        || string_field(
            &record,
            "income_security_family_childcare_family_service_capture_gap_path",
        )? != INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_food_hardship_nutrition_capture_gap_path",
        )? != INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_census_child_poverty_income_capture_gap_path",
        )? != INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_outcome_floor_definition_packet_path",
        )? != INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
    {
        return Err(
            "income-security/family source capture status rollup identity failed".to_string(),
        );
    }

    let summary = record
        .get("post_pulse_200_summary")
        .ok_or("income-security/family source capture post-Pulse-200 summary")?;
    for (field, expected) in [
        ("narrow_source_custody_or_context_complete_count", 5),
        ("capture_gap_documented_count", 4),
        ("remaining_closure_gate_count", 6),
    ] {
        if int_field(summary, field)? != expected {
            return Err(format!(
                "income-security/family source capture post-Pulse-200 summary count failed: {field}"
            ));
        }
    }
    for field in ["solver_ready", "rate_ready"] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family source capture post-Pulse-200 summary must block {field}"
            ));
        }
    }
    if string_field(summary, "showcase_status")?
        != "demo_ready_as_readiness_and_guardrail_system_not_rate_or_savings_model"
    {
        return Err("income-security/family source capture showcase status failed".to_string());
    }

    let rows = record
        .get("source_family_rollup")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source capture status rows")?;
    if rows.len() != 6 {
        return Err(
            "income-security/family source capture status rollup must have six rows".to_string(),
        );
    }
    let expected_families = [
        "Federal program outlay perimeter",
        "CBO baseline and take-up context",
        "Child poverty and income context",
        "Childcare and family-service context",
        "Food hardship and nutrition context",
        "International comparator context",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let expected_work_items = [
        "capture-income-security-federal-program-outlay-perimeter",
        "capture-income-security-cbo-baseline-and-takeup-context",
        "capture-income-security-child-poverty-income-context",
        "capture-income-security-childcare-family-service-context",
        "capture-income-security-food-hardship-nutrition-context",
        "capture-income-security-international-comparator-context",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_families = rows
        .iter()
        .map(|row| string_field(row, "source_family"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let observed_work_items = rows
        .iter()
        .map(|row| string_field(row, "queue_work_item_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_families != expected_families || observed_work_items != expected_work_items {
        return Err(
            "income-security/family source capture status family/work item set failed".to_string(),
        );
    }

    for row in rows {
        let family = string_field(row, "source_family")?;
        let custody_status = string_field(row, "custody_status")?;
        if string_field(row, "source_role")?.is_empty()
            || string_field(row, "supporting_artifact_path")?.is_empty()
        {
            return Err(format!(
                "income-security/family source capture status row shape failed: {family}"
            ));
        }
        let (expected_status, raw_ready, context_ready) = match family.as_str() {
            "Federal program outlay perimeter" => (
                "narrow_fy2025_federal_account_perimeter_source_custody_ready",
                true,
                true,
            ),
            "CBO baseline and take-up context" => {
                ("browser_context_ready_raw_custody_blocked", false, true)
            }
            "Child poverty and income context" => (
                "census_poverty_income_raw_custody_context_ready_values_blocked",
                true,
                true,
            ),
            "Childcare and family-service context" => (
                "capture_gap_documented_hhs_acf_source_custody_open",
                false,
                false,
            ),
            "Food hardship and nutrition context" => (
                "partial_ers_and_fns_raw_custody_ready_boundary_open",
                false,
                true,
            ),
            "International comparator context" => (
                "partial_oecd_child_poverty_and_socx_context_ready_broader_comparator_gate_open",
                true,
                true,
            ),
            _ => {
                return Err(format!(
                    "income-security/family source capture status unexpected family: {family}"
                ));
            }
        };
        if custody_status != expected_status
            || row
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(raw_ready)
            || row
                .get("may_populate_context")
                .and_then(serde_json::Value::as_bool)
                != Some(context_ready)
        {
            return Err(format!(
                "income-security/family source capture status row readiness failed: {family}"
            ));
        }
        for field in [
            "may_populate_program_outlay_perimeter",
            "may_populate_benefit_package_or_takeup_model",
            "may_populate_floor_values_or_pass_fail",
            "may_populate_solver_input",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "income-security/family source capture status row must block {field}: {family}"
                ));
            }
        }
    }

    let counts = record
        .get("readiness_counts")
        .ok_or("income-security/family source capture status counts")?;
    for (field, expected) in [
        ("source_family_count", 6),
        ("capture_item_open_count", 4),
        ("capture_gap_documented_count", 4),
        ("narrow_source_custody_or_context_ready_count", 5),
        ("raw_custody_ready_count", 3),
        ("context_ready_count", 5),
        ("program_outlay_perimeter_ready_count", 0),
        ("benefit_package_or_takeup_model_ready_count", 0),
        ("floor_value_ready_count", 0),
        ("solver_input_ready_count", 0),
        ("public_rate_ready_count", 0),
    ] {
        if int_field(counts, field)? != expected {
            return Err(format!(
                "income-security/family source capture status count failed: {field}"
            ));
        }
    }

    let readiness = record
        .get("income_security_family_lane_readiness")
        .ok_or("income-security/family lane source capture readiness")?;
    for field in [
        "source_readiness_gap_published",
        "source_capture_queue_published",
        "post_pulse_200_status_rollup_published",
        "fy2025_federal_account_perimeter_source_custody_ready",
        "cbo_snap_browser_context_ready",
        "census_child_poverty_income_context_ready",
        "ers_food_security_context_ready",
        "fns_snap_context_ready",
        "socx_family_benefit_context_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family lane source capture readiness should be true: {field}"
            ));
        }
    }
    for field in [
        "source_capture_complete",
        "program_outlay_perimeter_ready",
        "benefit_package_raw_custody_ready",
        "take_up_raw_custody_ready",
        "material_hardship_raw_custody_ready",
        "childcare_access_raw_custody_ready",
        "work_transition_raw_custody_ready",
        "federal_state_local_translation_ready",
        "threshold_values_selected",
        "observed_floor_values_populated",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "solver_input_ready",
        "public_rate_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family lane source capture readiness must be false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture status blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family source capture status blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture status claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family source capture status claim bool")?;
        if matches!(
            field.as_str(),
            "income_security_family_source_capture_status_rollup_published"
                | "source_readiness_gap_published"
                | "source_capture_queue_published"
                | "post_pulse_200_status_rollup_published"
                | "fy2025_federal_account_perimeter_source_custody_ready"
                | "cbo_snap_browser_context_ready"
                | "census_child_poverty_income_context_ready"
                | "child_poverty_raw_custody_ready"
                | "ers_food_security_context_ready"
                | "fns_snap_context_ready"
                | "socx_family_benefit_context_ready"
        ) {
            if !observed {
                return Err(format!(
                    "income-security/family source capture status claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "income-security/family source capture status claim must be false: {field}"
            ));
        }
    }

    let gates = record
        .get("next_source_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source capture status next gates")?;
    if gates.len() != 6 {
        return Err("income-security/family source capture status gate count failed".to_string());
    }
    for gate in gates {
        if string_field(gate, "gate")?.is_empty()
            || gate.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !gate.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err(
                "income-security/family source capture status gates must remain null/false"
                    .to_string(),
            );
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "SNAP browser-visible CBO baseline context",
        "Census child-poverty/income raw custody",
        "partial ERS food-security and FNS SNAP raw custody",
        "not complete source capture",
        "not full raw source custody",
        "not CBO raw PDF/spreadsheet custody",
        "not complete USDA raw source custody",
        "not a program outlay perimeter model",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family source capture status warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        "Pulse 201 summarizes",
        "FY2025 federal account-perimeter source custody is narrowly ready",
        "OECD SOCX family-benefit comparator context is displayable",
        "CBO SNAP browser-visible baseline context is now documented",
        "CBO raw",
        "Census child poverty and income raw custody/context is now ready",
        "HHS/ACF childcare and family-service capture remains a documented gap",
        "USDA ERS food-security and FNS SNAP raw custody/context are now partially ready",
        "not complete USDA raw source custody",
        "not complete source capture",
        "not full raw source custody",
        "not CBO raw PDF/spreadsheet custody",
        "not a program outlay perimeter model",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family source capture status reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_source_capture_closure_work_queue(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family source capture closure work queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-source-capture-closure-work-queue:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_source_capture_closure_work_queue"
        || int_field(&record, "pulse")? != 193
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "income_security_family_source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(
            &record,
            "income_security_family_source_capture_status_rollup_path",
        )? != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(
            &record,
            "income_security_family_outcome_floor_definition_packet_path",
        )? != INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
    {
        return Err(
            "income-security/family source capture closure work queue identity failed".to_string(),
        );
    }

    let rules = record
        .get("closure_rules")
        .ok_or("income-security/family source capture closure rules")?;
    for field in [
        "official_sources_only",
        "new_external_downloads_not_performed_in_this_pulse",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "threshold_selection_requires_stronger_model_review",
        "benefit_package_design_requires_stronger_model_review",
        "federal_state_local_translation_requires_explicit_lineage",
        "international_spending_differences_are_not_savings",
        "no_fraud_inference_from_comparison_or_administrative_context",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family source capture closure rule must be true: {field}"
            ));
        }
    }

    let items = record
        .get("closure_work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family source capture closure items")?;
    if items.len() != 6 {
        return Err("income-security/family source capture closure item count failed".to_string());
    }
    let expected = [
        (
            "close-income-security-federal-program-perimeter-lineage",
            "capture-income-security-federal-program-outlay-perimeter",
            "program_outlay_perimeter_ready",
            1,
        ),
        (
            "close-income-security-cbo-baseline-takeup-lineage",
            "capture-income-security-cbo-baseline-and-takeup-context",
            "cbo_baseline_takeup_context_ready",
            2,
        ),
        (
            "close-income-security-child-poverty-income-lineage",
            "capture-income-security-child-poverty-income-context",
            "child_poverty_income_context_ready",
            3,
        ),
        (
            "close-income-security-childcare-family-service-lineage",
            "capture-income-security-childcare-family-service-context",
            "childcare_family_service_context_ready",
            4,
        ),
        (
            "close-income-security-food-hardship-nutrition-lineage",
            "capture-income-security-food-hardship-nutrition-context",
            "food_hardship_nutrition_context_ready",
            5,
        ),
        (
            "close-income-security-international-comparator-lineage",
            "capture-income-security-international-comparator-context",
            "international_comparator_context_ready",
            6,
        ),
    ];
    for (closure_item_id, queue_item_id, closure_gate, priority) in expected {
        let item = items
            .iter()
            .find(|item| string_field(item, "closure_item_id").as_deref() == Ok(closure_item_id))
            .ok_or_else(|| {
                format!("missing income-security/family closure item: {closure_item_id}")
            })?;
        if int_field(item, "priority")? != priority
            || string_field(item, "depends_on_queue_work_item_id")? != queue_item_id
            || string_field(item, "closure_gate")? != closure_gate
            || item
                .get("required_closure_evidence")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.len() < 6)
            || item
                .get("unblocks_when_complete")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.is_empty())
        {
            return Err(format!(
                "income-security/family source capture closure item shape failed: {closure_item_id}"
            ));
        }
        for field in ["raw_artifact_path", "metadata_path", "closure_value"] {
            if !item.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "income-security/family source capture closure field must be null: {closure_item_id}.{field}"
                ));
            }
        }
        if item.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family source capture closure item must not be ready: {closure_item_id}"
            ));
        }
    }

    let counts = record
        .get("aggregate_status")
        .ok_or("income-security/family source capture closure aggregate status")?;
    for (field, expected) in [
        ("closure_work_item_count", 6),
        ("items_ready_count", 0),
        ("raw_custody_ready_count", 0),
        ("lineage_review_ready_count", 0),
        ("threshold_values_selected", 0),
        ("baseline_values_populated", 0),
        ("policy_values_populated", 0),
        ("stress_values_populated", 0),
        ("solver_ready_items", 0),
        ("public_rate_ready_items", 0),
    ] {
        if int_field(counts, field)? != expected {
            return Err(format!(
                "income-security/family source capture closure aggregate count failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture closure blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family source capture closure blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family source capture closure claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family source capture closure claim bool")?;
        if field == "income_security_family_source_capture_closure_work_queue_published" {
            if !observed {
                return Err(
                    "income-security/family source capture closure publication flag must be true"
                        .to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family source capture closure claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "names closure gates only",
        "not complete source capture",
        "not raw source custody",
        "not lineage review completion",
        "not a program outlay perimeter",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family source capture closure warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH,
        "Federal program perimeter lineage",
        "CBO baseline and take-up lineage",
        "Child poverty and income lineage",
        "Childcare and family-service lineage",
        "Food hardship and nutrition lineage",
        "International comparator lineage",
        "not complete source capture",
        "not raw source custody",
        "not lineage review completion",
        "not a program outlay perimeter",
        "not a benefit package model",
        "not a take-up model",
        "not child-poverty floor values",
        "not material-hardship floor values",
        "not childcare-access floor values",
        "not a work-transition model",
        "not federal/state/local translation",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family source capture closure reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_federal_program_perimeter_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH,
        INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family federal program perimeter bridge artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-federal-program-perimeter-bridge:fy2025"
        || string_field(&record, "record_family")?
            != "income_security_family_federal_program_perimeter_bridge"
        || int_field(&record, "pulse")? != 194
        || string_field(&record, "lane_id")? != "income-security-family"
        || int_field(&record, "fiscal_year")? != 2025
        || string_field(&record, "function_code")? != "600"
    {
        return Err(
            "income-security/family federal program perimeter bridge identity failed".to_string(),
        );
    }

    let custody = record
        .get("source_custody")
        .ok_or("income-security/family federal perimeter source custody")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family federal perimeter custody flag must be true: {field}"
            ));
        }
    }
    if custody
        .get("new_external_download_performed")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-PBD-OUTLAYS-FY2027/2026-07-13/outlays_fy2027.xlsx"
        || int_field(custody, "raw_byte_count")? != 2_144_756
        || string_field(custody, "raw_sha256")?
            != "D892F2247E6C1AED68414D3E4168F8B4AB97BCFC7ACF82A6A449A3FCB1ADDB07"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OMB-PBD-OUTLAYS-FY2027.2026-07-13.metadata.md"
        || string_field(custody, "retrieval_date")? != "2026-07-13"
        || string_field(custody, "source_unit")? != "thousands_usd"
        || string_field(custody, "record_unit")? != "millions_usd"
    {
        return Err(
            "income-security/family federal perimeter source custody values failed".to_string(),
        );
    }

    let perimeter = record
        .get("perimeter_definition")
        .ok_or("income-security/family federal perimeter definition")?;
    let included = perimeter
        .get("included_subfunction_codes")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family included subfunctions")?;
    let observed: BTreeSet<_> = included
        .iter()
        .map(|value| value.as_str().unwrap_or_default().to_string())
        .collect();
    let expected: BTreeSet<_> = ["601", "602", "603", "604", "605", "609"]
        .iter()
        .map(|value| value.to_string())
        .collect();
    if observed != expected
        || !string_field(perimeter, "federal_versus_state_local_statement")?
            .contains("federal account outlays")
    {
        return Err("income-security/family federal perimeter definition failed".to_string());
    }

    let reconciliation = record
        .get("reconciliation")
        .ok_or("income-security/family federal perimeter reconciliation")?;
    for (field, expected) in [
        ("historical_table_3_2_total_musd", 701_609),
        ("public_budget_database_total_musd", 701_609),
        ("reconciliation_difference_musd", 0),
        ("nonzero_account_rows", 160),
        ("positive_account_entries_total_musd", 728_963),
        ("negative_account_entries_total_musd", -27_354),
    ] {
        if int_field(reconciliation, field)? != expected {
            return Err(format!(
                "income-security/family federal perimeter reconciliation failed: {field}"
            ));
        }
    }

    let subfunctions = record
        .get("subfunction_totals")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family federal perimeter subfunctions")?;
    if subfunctions.len() != 6 {
        return Err(
            "income-security/family federal perimeter subfunction count failed".to_string(),
        );
    }
    let expected_totals = [
        ("601", 6_663),
        ("602", 190_234),
        ("603", 41_771),
        ("604", 77_989),
        ("605", 149_631),
        ("609", 235_321),
    ];
    let mut total = 0;
    for (code, expected_total) in expected_totals {
        let row = subfunctions
            .iter()
            .find(|row| string_field(row, "subfunction_code").as_deref() == Ok(code))
            .ok_or_else(|| {
                format!("missing income-security/family federal subfunction total: {code}")
            })?;
        if int_field(row, "total_musd")? != expected_total || int_field(row, "account_rows")? <= 0 {
            return Err(format!(
                "income-security/family federal subfunction total failed: {code}"
            ));
        }
        total += expected_total;
    }
    if total != 701_609 {
        return Err("income-security/family federal subfunction sum failed".to_string());
    }

    let closure = record
        .get("closure_relationship")
        .ok_or("income-security/family federal perimeter closure relationship")?;
    if string_field(closure, "source_capture_queue_path")?
        != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(closure, "source_capture_status_rollup_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(closure, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(closure, "satisfies_work_item_id")?
            != "capture-income-security-federal-program-outlay-perimeter"
        || string_field(closure, "narrow_gate_status")?
            != "fy2025_federal_account_perimeter_source_custody_ready"
    {
        return Err(
            "income-security/family federal perimeter closure relationship failed".to_string(),
        );
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family federal perimeter blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family federal perimeter blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family federal perimeter claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family federal perimeter claim bool")?;
        if field == "fy2025_federal_account_perimeter_source_custody_ready" {
            if !observed {
                return Err(
                    "income-security/family federal perimeter custody flag must be true"
                        .to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family federal perimeter downstream claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "closes only FY2025 federal account-perimeter source custody",
        "not complete source capture",
        "not a benefit package model",
        "not a take-up model",
        "not federal/state/local translation",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family federal perimeter warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH,
        "$701.609B",
        "zero reconciliation difference",
        "federal account source custody only",
        "not federal/state/local translation",
        "not target-cost selection",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family federal perimeter reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_cbo_baseline_takeup_capture_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH,
        INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family CBO baseline take-up capture gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-cbo-baseline-takeup-capture-gap:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_cbo_baseline_takeup_capture_gap"
        || int_field(&record, "pulse")? != 195
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_bridge_path")?
            != INCOME_SECURITY_FAMILY_FEDERAL_PROGRAM_PERIMETER_BRIDGE_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-cbo-baseline-and-takeup-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-cbo-baseline-takeup-lineage"
    {
        return Err(
            "income-security/family CBO baseline take-up capture gap identity failed".to_string(),
        );
    }

    let discovery = record
        .get("source_discovery")
        .ok_or("income-security/family CBO source discovery")?;
    if discovery
        .get("official_sources_only")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || string_field(discovery, "selected_programs_page_url")?
            != "https://www.cbo.gov/data/baseline-projections-selected-programs"
        || string_field(discovery, "candidate_snap_pdf_url")?
            != "https://www.cbo.gov/system/files/2026-01/51312-2026-02-snap.pdf"
        || string_field(discovery, "candidate_snap_spreadsheet_url")?
            != "https://www.cbo.gov/system/files/2026-01/51312-2026-02-snap.xlsx"
        || string_field(discovery, "official_cbo_open_data_repo_url")?
            != "https://github.com/US-CBO/cbo-data"
        || string_field(discovery, "official_cbo_open_data_repo_head")?
            != "284a95665f9f2f74ed1f482feb629b43fce323da"
        || discovery
            .get("open_data_catalog_checked")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || discovery
            .get("selected_program_snap_csv_found_in_open_data_catalog")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err(
            "income-security/family CBO baseline take-up source discovery failed".to_string(),
        );
    }

    let browser_context = record
        .get("browser_visible_context")
        .ok_or("income-security/family CBO browser-visible context")?;
    if browser_context
        .get("selected_programs_page_browser_visible")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || browser_context
            .get("snap_pdf_browser_visible")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || int_field(browser_context, "snap_pdf_page_count")? != 3
        || string_field(browser_context, "snap_pdf_title")? != "SNAP Baseline--02-2026-rev"
        || string_field(browser_context, "snap_program_name")?
            != "Supplemental Nutrition Assistance Program"
        || string_field(browser_context, "snap_publication_month")? != "February 2026"
        || !string_field(browser_context, "context_boundary")?.contains("Browser text verifies")
        || !string_field(browser_context, "context_boundary")?.contains("raw PDF custody")
        || !string_field(browser_context, "context_boundary")?.contains("spreadsheet custody")
    {
        return Err("income-security/family CBO browser-visible context failed".to_string());
    }
    let browser_fields = browser_context
        .get("browser_visible_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family CBO browser fields")?;
    for field in [
        "estimated outlays",
        "average monthly participation",
        "average monthly benefit per participant",
        "employment and training budget authority",
    ] {
        if !browser_fields
            .iter()
            .any(|value| value.as_str() == Some(field))
        {
            return Err(format!(
                "income-security/family CBO browser field missing: {field}"
            ));
        }
    }

    let attempts = record
        .get("capture_attempts")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family CBO capture attempts")?;
    if attempts.len() != 5 {
        return Err("income-security/family CBO capture attempt count failed".to_string());
    }
    for attempt_id in [
        "cbo-snap-pdf-powershell-invoke-webrequest",
        "cbo-snap-pdf-curl-user-agent",
        "cbo-open-data-catalog-search",
        "cbo-snap-pdf-powershell-direct-url-2026-07-24",
        "cbo-snap-spreadsheet-powershell-direct-url-2026-07-24",
    ] {
        let attempt = attempts
            .iter()
            .find(|attempt| string_field(attempt, "attempt_id").as_deref() == Ok(attempt_id))
            .ok_or_else(|| format!("missing income-security/family CBO attempt: {attempt_id}"))?;
        if attempt
            .get("captured_as_source")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
            || !attempt
                .get("retained_raw_artifact_path")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err(format!(
                "income-security/family CBO attempt must not be source custody: {attempt_id}"
            ));
        }
    }
    let curl_attempt = attempts
        .iter()
        .find(|attempt| {
            string_field(attempt, "attempt_id").as_deref() == Ok("cbo-snap-pdf-curl-user-agent")
        })
        .ok_or("income-security/family CBO curl attempt")?;
    if int_field(curl_attempt, "response_byte_count")? != 770
        || string_field(curl_attempt, "result")? != "blocked_by_js_challenge"
    {
        return Err("income-security/family CBO curl attempt shape failed".to_string());
    }
    for attempt_id in [
        "cbo-snap-pdf-powershell-direct-url-2026-07-24",
        "cbo-snap-spreadsheet-powershell-direct-url-2026-07-24",
    ] {
        let attempt = attempts
            .iter()
            .find(|attempt| string_field(attempt, "attempt_id").as_deref() == Ok(attempt_id))
            .ok_or_else(|| {
                format!("missing income-security/family CBO 403 attempt: {attempt_id}")
            })?;
        if string_field(attempt, "result")? != "blocked_by_403_from_command_line"
            || string_field(attempt, "attempted_at_local_date")? != "2026-07-24"
            || attempt
                .get("captured_as_source")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!(
                "income-security/family CBO 403 attempt failed: {attempt_id}"
            ));
        }
    }

    let requirements = record
        .get("next_manual_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family CBO manual requirements")?;
    if requirements.len() < 4 {
        return Err("income-security/family CBO manual requirements too short".to_string());
    }

    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family CBO readiness")?;
    for (field, value) in readiness {
        let expected = field == "cbo_snap_browser_context_ready";
        if value.as_bool() != Some(expected) {
            return Err(format!(
                "income-security/family CBO readiness boundary failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family CBO blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family CBO blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family CBO claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family CBO claim bool")?;
        if field == "cbo_capture_gap_published" {
            if !observed {
                return Err(
                    "income-security/family CBO gap publication flag must be true".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family CBO downstream claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "blocked source acquisition only",
        "browser-visible SNAP context",
        "raw PDF/spreadsheet custody remains blocked",
        "not raw CBO source custody",
        "not CBO baseline values",
        "not take-up context",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family CBO warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH,
        "JavaScript challenge HTML",
        "770-byte",
        "SNAP Baseline--02-2026-rev",
        "HTTP 403",
        "no local raw PDF/spreadsheet custody",
        "284a95665f9f2f74ed1f482feb629b43fce323da",
        "not raw CBO source custody",
        "not CBO baseline values",
        "not take-up context",
        "not solver input",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family CBO reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_child_relative_poverty_context_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family child relative poverty bridge artifact: {path}"
            ));
        }
    }

    let raw = root.join(IDD_CHILD_POVERTY_RAW_PATH);
    if !raw.exists()
        || raw.metadata().map_err(|e| e.to_string())?.len() != 2546
        || sha256_file(&raw)? != IDD_CHILD_POVERTY_RAW_SHA256
    {
        return Err("income-security/family child relative poverty raw custody failed".to_string());
    }

    let panel_text = fs::read_to_string(root.join(AGE_RELATIVE_POVERTY_PANEL_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let panel: serde_json::Value = serde_json::from_str(&panel_text).map_err(|e| e.to_string())?;
    let usa_child = panel
        .get("country_records")
        .and_then(serde_json::Value::as_array)
        .and_then(|records| {
            records
                .iter()
                .find(|record| string_field(record, "country_code").as_deref() == Ok("USA"))
        })
        .ok_or("income-security/family child poverty USA panel row")?;
    if int_field(usa_child, "child_reference_year")? != 2021
        || (number_field(usa_child, "child_poverty_percent")? - 13.99).abs() > 0.000001
        || string_field(usa_child, "child_observation_status")? != "actual"
    {
        return Err("income-security/family child poverty source value failed".to_string());
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-child-relative-poverty-context-bridge:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_child_relative_poverty_context_bridge"
        || int_field(&record, "pulse")? != 196
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_capture_gap_path")?
            != INCOME_SECURITY_FAMILY_CBO_BASELINE_TAKEUP_CAPTURE_GAP_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "existing_context_artifact_path")?
            != AGE_RELATIVE_POVERTY_PANEL_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-child-poverty-income-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-child-poverty-income-lineage"
    {
        return Err("income-security/family child poverty bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("income-security/family child poverty bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_captured_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family child poverty scope should be true: {field}"
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
                "income-security/family child poverty scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "international child relative-income-poverty context"
        || string_field(scope, "unclosed_component")?
            != "Census domestic child poverty and income-unit context"
    {
        return Err("income-security/family child poverty scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("income-security/family child poverty source custody")?;
    if string_field(custody, "source_id")? != "SRC-OECD-IDD-AGE-POVERTY-PANELS"
        || string_field(custody, "raw_artifact_path")? != IDD_CHILD_POVERTY_RAW_PATH
        || int_field(custody, "raw_byte_count")? != 2546
        || string_field(custody, "raw_sha256")? != IDD_CHILD_POVERTY_RAW_SHA256
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OECD-IDD-AGE-POVERTY-PANELS.2026-07-15.metadata.md"
        || custody
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("income-security/family child poverty custody fields failed".to_string());
    }

    let values = record
        .get("context_values")
        .ok_or("income-security/family child poverty context values")?;
    let us = values
        .get("primary_us_context")
        .ok_or("income-security/family child poverty US context")?;
    if string_field(values, "unit")? != "percent"
        || int_field(values, "country_count")? != 12
        || int_field(values, "observed_country_count")? != 11
        || string_field(us, "country_code")? != "USA"
        || int_field(us, "reference_year")? != 2021
        || (number_field(us, "child_relative_poverty_percent")? - 13.99).abs() > 0.000001
        || string_field(us, "observation_status")? != "actual"
    {
        return Err("income-security/family child poverty context values failed".to_string());
    }

    let requirements = record
        .get("remaining_domestic_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family child poverty domestic requirements")?;
    if requirements.len() != 5 {
        return Err(
            "income-security/family child poverty domestic requirements count failed".to_string(),
        );
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family child poverty blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family child poverty blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family child poverty claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family child poverty claim bool")?;
        match field.as_str() {
            "child_relative_poverty_context_bridge_published"
            | "international_child_poverty_context_ready" => {
                if !observed {
                    return Err(format!(
                        "income-security/family child poverty claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "income-security/family child poverty downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only existing OECD international child relative-poverty context",
        "not Census domestic child poverty custody",
        "not child-poverty floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family child poverty warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH,
        "2,546 bytes",
        IDD_CHILD_POVERTY_RAW_SHA256,
        "13.99 percent",
        "international child relative-poverty context only",
        "not Census domestic child poverty custody",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family child poverty reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_socx_family_benefit_comparator_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH,
        INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family SOCX comparator bridge artifact: {path}"
            ));
        }
    }

    let raw = root.join(SOCX_OLDAGE_FAMILY_RAW_PATH);
    if !raw.exists()
        || raw.metadata().map_err(|e| e.to_string())?.len() != 4334
        || sha256_file(&raw)? != SOCX_OLDAGE_FAMILY_RAW_SHA256
    {
        return Err("income-security/family SOCX raw custody failed".to_string());
    }

    let panel_text = fs::read_to_string(root.join(SOCX_OLDAGE_FAMILY_PANEL_JSON_PATH))
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
        .ok_or("income-security/family SOCX USA panel row")?;
    if (number_field(usa, "family_total_percent_gdp")? - 0.658).abs() > 0.000001
        || (number_field(usa, "family_cash_percent_gdp")? - 0.051).abs() > 0.000001
        || (number_field(usa, "family_services_percent_gdp")? - 0.607).abs() > 0.000001
        || string_field(usa, "observation_status")? != "complete_rounded_components"
    {
        return Err("income-security/family SOCX source value failed".to_string());
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-socx-family-benefit-comparator-bridge:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_socx_family_benefit_comparator_bridge"
        || int_field(&record, "pulse")? != 197
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_child_relative_poverty_bridge_path")?
            != INCOME_SECURITY_FAMILY_CHILD_RELATIVE_POVERTY_CONTEXT_BRIDGE_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "existing_context_artifact_path")?
            != SOCX_OLDAGE_FAMILY_PANEL_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-international-comparator-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-international-comparator-lineage"
    {
        return Err("income-security/family SOCX bridge identity failed".to_string());
    }

    let scope = record
        .get("closure_scope")
        .ok_or("income-security/family SOCX bridge scope")?;
    for field in [
        "official_sources_only",
        "uses_existing_captured_source",
        "context_may_be_displayed",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "income-security/family SOCX scope should be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_downloads_performed",
        "closure_gate_ready",
        "target_cost_may_be_selected",
        "solver_inputs_may_be_populated",
        "rates_may_be_calculated",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family SOCX scope should be false: {field}"
            ));
        }
    }
    if string_field(scope, "closed_component")?
        != "OECD SOCX 2022 public family-benefit total, cash, and in-kind services context"
        || !string_field(scope, "unclosed_component")?.contains("tax breaks")
    {
        return Err("income-security/family SOCX scope boundary failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("income-security/family SOCX source custody")?;
    if string_field(custody, "source_id")? != "SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022"
        || string_field(custody, "raw_artifact_path")? != SOCX_OLDAGE_FAMILY_RAW_PATH
        || int_field(custody, "raw_byte_count")? != 4334
        || string_field(custody, "raw_sha256")? != SOCX_OLDAGE_FAMILY_RAW_SHA256
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022.2026-07-15.metadata.md"
        || custody
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("income-security/family SOCX custody fields failed".to_string());
    }

    let values = record
        .get("context_values")
        .ok_or("income-security/family SOCX context values")?;
    let us = values
        .get("primary_us_context")
        .ok_or("income-security/family SOCX US context")?;
    if string_field(values, "unit")? != "percent_gdp"
        || int_field(values, "source_year")? != 2022
        || int_field(values, "country_count")? != 12
        || int_field(values, "observed_family_country_count")? != 7
        || string_field(us, "country_code")? != "USA"
        || (number_field(us, "family_total_percent_gdp")? - 0.658).abs() > 0.000001
        || (number_field(us, "family_cash_percent_gdp")? - 0.051).abs() > 0.000001
        || (number_field(us, "family_services_percent_gdp")? - 0.607).abs() > 0.000001
        || string_field(us, "observation_status")? != "complete_rounded_components"
    {
        return Err("income-security/family SOCX context values failed".to_string());
    }
    let missing = values
        .get("missing_country_codes")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family SOCX missing countries")?;
    if missing.len() != 5 {
        return Err("income-security/family SOCX missing-country count failed".to_string());
    }

    let requirements = record
        .get("remaining_comparator_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family SOCX remaining comparator requirements")?;
    if requirements.len() != 6 {
        return Err(
            "income-security/family SOCX remaining comparator requirements count failed"
                .to_string(),
        );
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family SOCX blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family SOCX blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family SOCX claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family SOCX claim bool")?;
        match field.as_str() {
            "socx_family_benefit_comparator_bridge_published"
            | "socx_family_benefit_context_ready" => {
                if !observed {
                    return Err(format!(
                        "income-security/family SOCX claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "income-security/family SOCX downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "only existing OECD SOCX 2022 public family-benefit",
        "not complete international comparator lineage",
        "not tax-credit composition",
        "not childcare participation context",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family SOCX warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH,
        "4,334 bytes",
        SOCX_OLDAGE_FAMILY_RAW_SHA256,
        "0.658 percent of GDP",
        "0.051 percent cash",
        "0.607",
        "SOCX family-benefit comparator context only",
        "not tax-credit composition",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family SOCX reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_childcare_family_service_capture_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH,
        INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family childcare capture gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-childcare-family-service-capture-gap:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_childcare_family_service_capture_gap"
        || int_field(&record, "pulse")? != 198
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_socx_bridge_path")?
            != INCOME_SECURITY_FAMILY_SOCX_FAMILY_BENEFIT_COMPARATOR_BRIDGE_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-childcare-family-service-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-childcare-family-service-lineage"
    {
        return Err("income-security/family childcare gap identity failed".to_string());
    }

    let discovery = record
        .get("source_discovery")
        .ok_or("income-security/family childcare source discovery")?;
    if discovery
        .get("official_sources_only")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || string_field(discovery, "candidate_ccdf_statistics_url")?
            != "https://www.acf.hhs.gov/occ/data/child-care-and-development-fund-statistics"
        || string_field(discovery, "candidate_tanf_program_url")?
            != "https://www.acf.hhs.gov/ofa/programs/temporary-assistance-needy-families-tanf"
        || string_field(discovery, "candidate_tanf_application_data_url")?
            != "https://www.acf.hhs.gov/ofa/data/tanf-application-data-2020-2029"
    {
        return Err("income-security/family childcare source discovery failed".to_string());
    }
    let source_families = discovery
        .get("required_source_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family childcare source families")?;
    if source_families.len() != 3 {
        return Err("income-security/family childcare source family count failed".to_string());
    }

    let custody = record
        .get("local_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family childcare local custody status")?;
    for field in [
        "local_raw_ccdf_artifact_path",
        "local_raw_tanf_artifact_path",
        "local_raw_family_service_artifact_path",
        "metadata_path",
        "raw_byte_count",
        "raw_sha256",
        "retrieval_date",
    ] {
        if !custody.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "income-security/family childcare custody field must be null: {field}"
            ));
        }
    }
    for field in ["custody_ready", "values_may_be_populated"] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family childcare custody bool must be false: {field}"
            ));
        }
    }

    let boundary_attempts = record
        .get("access_boundary_attempts")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family childcare access boundary attempts")?;
    if boundary_attempts.len() != 5 {
        return Err(
            "income-security/family childcare access boundary attempt count failed".to_string(),
        );
    }
    let mut saw_ccdf_statistics_url = false;
    let mut saw_tanf_application_url = false;
    for attempt in boundary_attempts {
        if string_field(attempt, "retrieval_date")? != "2026-07-24"
            || string_field(attempt, "method")? != "PowerShell Invoke-WebRequest"
            || string_field(attempt, "result")? != "http_202_empty_body"
            || attempt
                .get("retained_as_custody")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(
                "income-security/family childcare access boundary attempt fields failed"
                    .to_string(),
            );
        }
        let url = string_field(attempt, "url")?;
        if url == "https://www.acf.hhs.gov/occ/data/child-care-and-development-fund-statistics" {
            saw_ccdf_statistics_url = true;
        }
        if url == "https://www.acf.hhs.gov/ofa/data/tanf-application-data-2020-2029" {
            saw_tanf_application_url = true;
        }
        if !string_field(attempt, "notes")?.contains("HTTP 202") {
            return Err(
                "income-security/family childcare access boundary notes missing HTTP 202"
                    .to_string(),
            );
        }
    }
    if !saw_ccdf_statistics_url || !saw_tanf_application_url {
        return Err(
            "income-security/family childcare access boundary source coverage failed".to_string(),
        );
    }

    let requirements = record
        .get("next_manual_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family childcare manual requirements")?;
    if requirements.len() != 5 {
        return Err(
            "income-security/family childcare manual requirements count failed".to_string(),
        );
    }

    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family childcare readiness")?;
    for (field, value) in readiness {
        let observed = value
            .as_bool()
            .ok_or("income-security/family childcare readiness bool")?;
        if field == "childcare_family_service_capture_gap_published" {
            if !observed {
                return Err(
                    "income-security/family childcare gap publication flag failed".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family childcare readiness must be false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family childcare blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family childcare blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family childcare claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family childcare claim bool")?;
        if field == "childcare_family_service_capture_gap_published" {
            if !observed {
                return Err(
                    "income-security/family childcare claim publication flag failed".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family childcare downstream claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "childcare/family-service source-capture gap only",
        "HTTP 202 empty-body access boundaries",
        "not HHS/ACF raw source custody",
        "not CCDF context",
        "not TANF context",
        "not childcare-access floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family childcare warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH,
        "ACF Office of Child Care CCDF statistics",
        "ACF TANF program data",
        "HTTP 202 empty-body access boundaries",
        "no local raw CCDF, TANF, or family-service",
        "childcare/family-service source-capture gap only",
        "not HHS/ACF raw source custody",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family childcare reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_food_hardship_nutrition_capture_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH,
        INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family food nutrition capture gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-food-hardship-nutrition-capture-gap:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_food_hardship_nutrition_capture_gap"
        || int_field(&record, "pulse")? != 199
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_childcare_gap_path")?
            != INCOME_SECURITY_FAMILY_CHILDCARE_FAMILY_SERVICE_CAPTURE_GAP_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-food-hardship-nutrition-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-food-hardship-nutrition-lineage"
    {
        return Err("income-security/family food nutrition gap identity failed".to_string());
    }

    let discovery = record
        .get("source_discovery")
        .ok_or("income-security/family food nutrition source discovery")?;
    if discovery
        .get("official_sources_only")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || string_field(discovery, "candidate_ers_food_security_report_url")?
            != "https://www.ers.usda.gov/publications"
        || string_field(discovery, "candidate_ers_food_security_topic_url")?
            != "https://www.ers.usda.gov/topics/food-nutrition-assistance/food-security-in-the-u-s"
        || string_field(discovery, "candidate_fns_snap_program_data_url")?
            != "https://www.fns.usda.gov/pd/supplemental-nutrition-assistance-program-snap"
    {
        return Err("income-security/family food nutrition source discovery failed".to_string());
    }
    let source_families = discovery
        .get("required_source_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family food nutrition source families")?;
    if source_families.len() != 3 {
        return Err("income-security/family food nutrition source family count failed".to_string());
    }

    let custody = record
        .get("local_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family food nutrition local custody status")?;
    for field in ["local_raw_nutrition_assistance_artifact_path"] {
        if !custody.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "income-security/family food nutrition custody field must be null: {field}"
            ));
        }
    }
    for (field, expected) in [
        (
            "local_raw_ers_food_security_artifact_path",
            "data/raw/usda/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024/2026-07-24/err-358-household-food-security-2024.pdf",
        ),
        (
            "local_raw_fns_snap_artifact_path",
            "data/raw/usda/SRC-USDA-FNS-SNAP-PARTICIPATION-COST-DATA/2026-07-24/snap-annualsummary-7.xlsx",
        ),
        (
            "metadata_path",
            "data/metadata/SRC-USDA-FNS-SNAP-PARTICIPATION-COST-DATA.2026-07-24.metadata.md",
        ),
        (
            "raw_sha256",
            "53c101e4f23c12d04c65ed304919b5f5ed18c560f9ea81acb9191cf8a54254e3",
        ),
        ("retrieval_date", "2026-07-24"),
        (
            "ers_metadata_path",
            "data/metadata/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024.2026-07-24.metadata.md",
        ),
        (
            "ers_raw_sha256",
            "dfe19c73cd5fbaa08a2dec52768690c968892150153806fec83038d3dac0adf7",
        ),
        ("ers_retrieval_date", "2026-07-24"),
    ] {
        if string_field(&serde_json::Value::Object(custody.clone()), field)? != expected {
            return Err(format!(
                "income-security/family food nutrition custody field failed: {field}"
            ));
        }
    }
    if custody
        .get("raw_byte_count")
        .and_then(serde_json::Value::as_i64)
        != Some(24215)
    {
        return Err("income-security/family food nutrition raw byte count failed".to_string());
    }
    if custody
        .get("ers_raw_byte_count")
        .and_then(serde_json::Value::as_i64)
        != Some(1_017_042)
    {
        return Err("income-security/family food nutrition ERS raw byte count failed".to_string());
    }
    let ers_supporting = custody
        .get("supporting_ers_food_security_raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family food nutrition supporting ERS raw files")?;
    if ers_supporting.len() != 7
        || !ers_supporting.iter().any(|item| {
            item.get("path").and_then(serde_json::Value::as_str)
                == Some("data/raw/usda/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024/2026-07-24/ap-126-statistical-supplement-2024.pdf")
        })
        || !ers_supporting.iter().any(|item| {
            item.get("path").and_then(serde_json::Value::as_str)
                == Some("data/raw/usda/SRC-USDA-ERS-HOUSEHOLD-FOOD-SECURITY-2024/2026-07-24/december-2024-cps-food-security-supplement-technical-documentation.pdf")
        })
    {
        return Err(
            "income-security/family food nutrition supporting ERS raw files failed".to_string(),
        );
    }
    let supporting = custody
        .get("supporting_fns_snap_raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family food nutrition supporting FNS raw files")?;
    if supporting.len() != 6
        || !supporting.iter().any(|item| {
            item.get("path").and_then(serde_json::Value::as_str)
                == Some("data/raw/usda/SRC-USDA-FNS-SNAP-PARTICIPATION-COST-DATA/2026-07-24/snap-zip-fy69tocurrent-7.zip")
        })
    {
        return Err(
            "income-security/family food nutrition supporting FNS raw files failed".to_string(),
        );
    }
    let structure = custody
        .get("observed_file_structure")
        .ok_or("income-security/family food nutrition observed file structure")?;
    if string_field(structure, "snap_annualsummary_xlsx_dimension")? != "A1:F200"
        || string_field(structure, "snap_4fymonthly_xlsx_dimension")? != "A1:GD59"
        || int_field(structure, "snap_zip_fy69tocurrent_entry_count")? != 38
    {
        return Err("income-security/family food nutrition file structure failed".to_string());
    }
    for field in ["custody_ready", "values_may_be_populated"] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family food nutrition custody bool must be false: {field}"
            ));
        }
    }
    if custody
        .get("ers_food_security_raw_custody_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || custody
            .get("fns_snap_raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("income-security/family food nutrition FNS custody flag failed".to_string());
    }

    let requirements = record
        .get("next_manual_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family food nutrition manual requirements")?;
    if requirements.len() != 5 {
        return Err(
            "income-security/family food nutrition manual requirements count failed".to_string(),
        );
    }

    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family food nutrition readiness")?;
    for (field, value) in readiness {
        let observed = value
            .as_bool()
            .ok_or("income-security/family food nutrition readiness bool")?;
        if field == "food_hardship_nutrition_capture_gap_published"
            || field == "ers_food_security_context_ready"
            || field == "fns_snap_context_ready"
        {
            if !observed {
                return Err(
                    "income-security/family food nutrition gap publication flag failed".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family food nutrition readiness must be false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family food nutrition blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family food nutrition blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family food nutrition claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family food nutrition claim bool")?;
        if field == "food_hardship_nutrition_capture_gap_published"
            || field == "ers_food_security_context_ready"
            || field == "fns_snap_context_ready"
        {
            if !observed {
                return Err(
                    "income-security/family food nutrition claim publication flag failed"
                        .to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family food nutrition downstream claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "food-hardship/nutrition source-capture gap with partial ERS food-security and FNS SNAP raw custody",
        "partial ERS food-security and FNS SNAP raw custody",
        "not complete USDA raw source custody",
        "not a complete nutrition-program boundary",
        "not material-hardship floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family food nutrition warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH,
        "USDA ERS food security publications",
        "USDA FNS SNAP participation, cost, and benefit data tables",
        "partial ERS food-security and FNS SNAP raw custody",
        "err-358-household-food-security-2024.pdf",
        "snap-annualsummary-7.xlsx",
        "not complete USDA raw source custody",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family food nutrition reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_census_child_poverty_income_capture_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH,
        INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family Census poverty capture gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-census-child-poverty-income-capture-gap:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_census_child_poverty_income_capture_gap"
        || int_field(&record, "pulse")? != 200
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "depends_on_food_nutrition_gap_path")?
            != INCOME_SECURITY_FAMILY_FOOD_HARDSHIP_NUTRITION_CAPTURE_GAP_JSON_PATH
        || string_field(&record, "source_capture_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "source_capture_closure_work_queue_path")?
            != INCOME_SECURITY_FAMILY_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "target_work_item_id")?
            != "capture-income-security-child-poverty-income-context"
        || string_field(&record, "target_closure_item_id")?
            != "close-income-security-child-poverty-income-lineage"
    {
        return Err("income-security/family Census poverty gap identity failed".to_string());
    }

    let discovery = record
        .get("source_discovery")
        .ok_or("income-security/family Census poverty source discovery")?;
    if discovery
        .get("official_sources_only")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || string_field(discovery, "candidate_poverty_2024_report_url")?
            != "https://www.census.gov/library/publications/2025/demo/p60-287.html"
        || string_field(discovery, "candidate_poverty_2024_pdf_url")?
            != "https://www2.census.gov/library/publications/2025/demo/p60-287.pdf"
        || string_field(discovery, "candidate_spm_topic_url")?
            != "https://www.census.gov/topics/income-poverty/supplemental-poverty-measure.html"
        || string_field(discovery, "candidate_poverty_2023_report_url")?
            != "https://www.census.gov/library/publications/2024/demo/p60-283.html"
    {
        return Err("income-security/family Census poverty source discovery failed".to_string());
    }
    let source_families = discovery
        .get("required_source_families")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family Census poverty source families")?;
    if source_families.len() != 3 {
        return Err("income-security/family Census poverty source family count failed".to_string());
    }

    let custody = record
        .get("local_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family Census poverty local custody status")?;
    for (field, expected) in [
        (
            "local_raw_census_poverty_report_path",
            "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/p60-287-poverty-2024.pdf",
        ),
        (
            "local_raw_census_poverty_table_path",
            "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/tableA3_hist_pov_by_all_and_age.xlsx",
        ),
        (
            "local_raw_census_spm_table_path",
            "data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/spm_opm_state_by_age.xlsx",
        ),
        (
            "metadata_path",
            "data/metadata/SRC-CENSUS-P60-287-POVERTY-2024.2026-07-24.metadata.md",
        ),
        (
            "raw_sha256",
            "b99624bfd024a4f9396594d23a845d5bd581f8094401a3b469aa0a3fb6c799d1",
        ),
        ("retrieval_date", "2026-07-24"),
    ] {
        if string_field(&serde_json::Value::Object(custody.clone()), field)? != expected {
            return Err(format!(
                "income-security/family Census poverty custody field failed: {field}"
            ));
        }
    }
    if custody
        .get("raw_byte_count")
        .and_then(serde_json::Value::as_i64)
        != Some(2_000_905)
    {
        return Err("income-security/family Census poverty raw byte count failed".to_string());
    }
    let supporting = custody
        .get("supporting_census_raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family Census poverty supporting raw files")?;
    if supporting.len() != 10
        || !supporting.iter().any(|item| {
            item.get("path").and_then(serde_json::Value::as_str)
                == Some("data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/cpsmar25.pdf")
        })
        || !supporting.iter().any(|item| {
            item.get("path").and_then(serde_json::Value::as_str)
                == Some("data/raw/census/SRC-CENSUS-P60-287-POVERTY-2024/2026-07-24/Income-to-Poverty-Ratios.xlsx")
        })
    {
        return Err("income-security/family Census poverty supporting raw files failed".to_string());
    }
    let structure = custody
        .get("observed_file_structure")
        .ok_or("income-security/family Census poverty observed file structure")?;
    if string_field(structure, "tableA3_hist_pov_by_all_and_age_dimension")? != "A1:M530"
        || string_field(structure, "spm_opm_state_by_age_dimension")? != "A1:Y61"
        || string_field(structure, "income_to_poverty_ratios_dimension")? != "A1:R39"
    {
        return Err("income-security/family Census poverty file structure failed".to_string());
    }
    for field in ["custody_ready", "values_may_be_populated"] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "income-security/family Census poverty custody bool must be false: {field}"
            ));
        }
    }
    if custody
        .get("raw_census_context_custody_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("income-security/family Census poverty custody flag failed".to_string());
    }

    let existing_context = record
        .get("existing_context_not_closure")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family Census poverty existing context")?;
    if existing_context.len() != 2 {
        return Err(
            "income-security/family Census poverty existing context count failed".to_string(),
        );
    }

    let requirements = record
        .get("next_manual_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family Census poverty manual requirements")?;
    if requirements.len() != 5 {
        return Err(
            "income-security/family Census poverty manual requirements count failed".to_string(),
        );
    }

    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family Census poverty readiness")?;
    for (field, value) in readiness {
        let observed = value
            .as_bool()
            .ok_or("income-security/family Census poverty readiness bool")?;
        if matches!(
            field.as_str(),
            "census_child_poverty_income_capture_gap_published"
                | "raw_census_custody_ready"
                | "official_child_poverty_context_ready"
                | "spm_child_poverty_context_ready"
                | "deep_poverty_or_near_poverty_context_ready"
                | "income_definition_ready"
                | "income_unit_perimeter_ready"
        ) {
            if !observed {
                return Err(
                    "income-security/family Census poverty gap publication flag failed".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family Census poverty readiness must be false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family Census poverty blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family Census poverty blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family Census poverty claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family Census poverty claim bool")?;
        if matches!(
            field.as_str(),
            "census_child_poverty_income_capture_gap_published"
                | "raw_census_custody_ready"
                | "official_child_poverty_context_ready"
                | "spm_child_poverty_context_ready"
                | "deep_poverty_or_near_poverty_context_ready"
                | "income_definition_ready"
                | "income_unit_perimeter_ready"
        ) {
            if !observed {
                return Err(
                    "income-security/family Census poverty claim publication flag failed"
                        .to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "income-security/family Census poverty downstream claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "Census child poverty and income source-capture gap with raw Census context custody",
        "raw Census context custody",
        "not child-poverty floor values",
        "not solver input",
        "not rate calculation",
        "not gross savings",
        "not net savings",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!(
                "income-security/family Census poverty warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH,
        "Census Current Population Reports poverty releases",
        "Census Supplemental Poverty Measure material",
        "CPS ASEC poverty/income documentation",
        "raw Census context custody",
        "p60-287-poverty-2024.pdf",
        "tableA3_hist_pov_by_all_and_age.xlsx",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family Census poverty reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_income_security_family_child_poverty_floor_value_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH,
        INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing income-security/family child-poverty floor value packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "income-security-family-child-poverty-floor-value-packet:v1"
        || string_field(&record, "record_family")?
            != "income_security_family_child_poverty_floor_value_packet"
        || int_field(&record, "pulse")? != 213
        || string_field(&record, "lane_id")? != "income-security-family"
        || string_field(&record, "floor_id")? != "child_poverty"
        || string_field(&record, "floor_definition_packet_path")?
            != INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "census_child_poverty_income_capture_gap_path")?
            != INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err(
            "income-security/family child-poverty floor value packet identity failed".to_string(),
        );
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("income-security/family child-poverty threshold rationale")?;
    if string_field(threshold, "rationale_id")?
        != "no-regression-from-2024-official-under-18-poverty"
        || string_field(threshold, "selected_measure")?
            != "Official poverty rate for people under 18 years"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_ceiling"
        || (number_field(threshold, "threshold_value")? - 14.3).abs() > 0.000001
        || string_field(threshold, "threshold_unit")? != "percent"
        || string_field(threshold, "source_table")? != "Census P60-287 Table A-1 and Table A-3"
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("income-security/family child-poverty threshold rationale failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("income-security/family child-poverty baseline values")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("income-security/family child-poverty primary baseline")?;
    if int_field(baseline, "reference_year")? != 2024
        || string_field(primary, "measure")? != "Under-18 official poverty rate"
        || (number_field(primary, "value")? - 14.3).abs() > 0.000001
        || int_field(primary, "population_thousands")? != 72550
        || int_field(primary, "below_poverty_thousands")? != 10350
        || (number_field(primary, "margin_of_error_percentage_points")? - 0.6).abs() > 0.000001
        || int_field(primary, "margin_of_error_below_poverty_thousands")? != 416
        || string_field(primary, "source_path")?
            != INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH
    {
        return Err("income-security/family child-poverty primary baseline failed".to_string());
    }
    let supporting = baseline
        .get("supporting_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("income-security/family child-poverty supporting context")?;
    if supporting.len() != 3
        || !string_field(baseline, "boundary")?.contains("not pass/fail evidence")
    {
        return Err("income-security/family child-poverty supporting context failed".to_string());
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "income-security/family child-poverty field must stay null: {field}"
            ));
        }
    }

    let readiness = record
        .get("readiness_status")
        .ok_or("income-security/family child-poverty readiness")?;
    for field in [
        "threshold_rationale_ready",
        "threshold_value_populated",
        "baseline_value_ready",
    ] {
        if !bool_field(readiness, field)? {
            return Err(format!(
                "income-security/family child-poverty readiness should be true: {field}"
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
                "income-security/family child-poverty readiness must remain false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family child-poverty blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "income-security/family child-poverty blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("income-security/family child-poverty claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("income-security/family child-poverty claim bool")?;
        match field.as_str() {
            "child_poverty_floor_value_packet_published"
            | "threshold_rationale_ready"
            | "threshold_value_populated"
            | "baseline_value_ready" => {
                if !observed {
                    return Err(format!(
                        "income-security/family child-poverty claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "income-security/family child-poverty downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "draft no-regression income-security/family child-poverty floor threshold",
        "not a benefit-package model",
        "not take-up",
        "not childcare access",
        "not nutrition handoff",
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
                "income-security/family child-poverty warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        INCOME_SECURITY_FAMILY_CHILD_POVERTY_FLOOR_VALUE_PACKET_JSON_PATH,
        INCOME_SECURITY_FAMILY_CENSUS_CHILD_POVERTY_INCOME_CAPTURE_GAP_JSON_PATH,
        "14.3 percent",
        "72.550",
        "10.350",
        "13.4 percent",
        "draft no-regression income-security/family child-poverty",
        "Policy and stress values",
        "not a benefit-package model",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "income-security/family child-poverty reader missing: {required}"
            ));
        }
    }

    Ok(())
}

