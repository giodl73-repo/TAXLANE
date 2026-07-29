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

pub(crate) fn validate_health_outcome_floor_definition_packet(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")? != "health_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 161
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(&record, "health_target_cost_scenario_path")?
            != HEALTH_TARGET_COST_SCENARIO_JSON_PATH
        || string_field(&record, "health_target_admissibility_path")?
            != HEALTH_TARGET_ADMISSIBILITY_JSON_PATH
        || string_field(&record, "health_national_phi_sensitivity_path")?
            != HEALTH_NATIONAL_PHI_JSON_PATH
    {
        return Err("health floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("health floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health floor status {field} must be true"));
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
            return Err(format!("health floor status {field} must be false"));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("health floor definition policy")?;
    for field in [
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "phi_sensitivities_cannot_populate_federal_target_cost_fields",
        "international_differences_not_savings",
        "no_fraud_inference",
        "federal_translation_required_before_solver_use",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health floor policy {field} must be true"));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("health required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("health required floor class count failed".to_string());
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
        return Err("health required floor class set failed".to_string());
    }
    for row in classes {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!("health floor class {field} must be null"));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("health floor class must remain unpassed".to_string());
        }
    }

    let health_floors = record
        .get("health_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("health-specific floor definitions")?;
    let expected_health_floors = [
        "coverage",
        "access",
        "quality",
        "risk_adjusted_outcomes",
        "rural_and_safety_net_capacity",
    ];
    if health_floors.len() != expected_health_floors.len() {
        return Err("health-specific floor count failed".to_string());
    }
    let observed_health_floors = health_floors
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_health_floor_set = expected_health_floors
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_health_floors != expected_health_floor_set {
        return Err("health-specific floor set failed".to_string());
    }
    for row in health_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("health-specific floors must remain null and unpassed".to_string());
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
        .ok_or("health floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("health_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
    {
        return Err("health floor summary counts failed".to_string());
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
            return Err(format!("health floor summary {field} must be false"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("health floor definition packet publication flag failed".to_string());
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
        "solver_input_ready",
        "solver_run_published",
        "rate_published",
        "department_cut_instruction_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("health floor claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This health floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "Health PHI sensitivities remain mechanical private-insurance payer-payment sensitivities and cannot populate federal target-cost or solver fields.",
        "No lower-cost health scenario is admissible until access, quality, equity, adequacy/resilience, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "No target cost, federal effect, gross savings, net savings, solver input, rate, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a federal score",
        "not a target-cost selection",
        "not solver input",
        "not a solver run",
        "not a rate calculation",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!("health floor reader missing phrase: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_medicare_provider_adequacy_margin_floor_value_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH,
        HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health/Medicare provider adequacy margin floor-value artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "health-medicare-provider-adequacy-margin-floor-value-packet:v1"
        || string_field(&record, "record_family")?
            != "health_medicare_provider_adequacy_margin_floor_value_packet"
        || int_field(&record, "pulse")? != 217
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "floor_id")? != "provider_adequacy_margin"
        || string_field(&record, "floor_class")? != "adequacy_resilience"
        || string_field(&record, "floor_definition_packet_path")?
            != HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "health_target_admissibility_path")?
            != HEALTH_TARGET_ADMISSIBILITY_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err(
            "health/Medicare provider adequacy margin floor-value identity failed".to_string(),
        );
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("health/Medicare provider adequacy margin threshold")?;
    if string_field(threshold, "rationale_id")?
        != "no-regression-from-fy2024-efficient-hospital-median-ffs-medicare-margin"
        || string_field(threshold, "selected_measure")?
            != "FY2024 median FFS Medicare margin for relatively efficient hospitals"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_floor"
        || number_field(threshold, "threshold_value")? != -1.0
        || string_field(threshold, "threshold_unit")? != "percent_margin"
        || !string_field(threshold, "source_table")?.contains("MedPAC March 2026")
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("health/Medicare provider adequacy margin threshold failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("health/Medicare provider adequacy margin baseline")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("health/Medicare provider adequacy margin primary baseline")?;
    if string_field(baseline, "reporting_period")? != "FY2024 actual"
        || string_field(primary, "measure")?
            != "relatively efficient hospital median FFS Medicare margin"
        || number_field(primary, "value")? != -1.0
        || string_field(primary, "unit")? != "percent_margin"
        || string_field(primary, "source_path")? != HEALTH_TARGET_ADMISSIBILITY_JSON_PATH
    {
        return Err("health/Medicare provider adequacy margin primary baseline failed".to_string());
    }
    let source_ids = primary
        .get("source_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("health/Medicare provider adequacy source ids")?
        .iter()
        .map(|value| value.as_str().ok_or("health/Medicare source id string"))
        .collect::<Result<Vec<_>, _>>()?;
    if source_ids != ["SRC-MEDPAC-MARCH-2026"] {
        return Err("health/Medicare provider adequacy source id failed".to_string());
    }
    let context = baseline
        .get("supporting_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("health/Medicare provider adequacy supporting context")?;
    if context.len() != 6 {
        return Err("health/Medicare provider adequacy context count failed".to_string());
    }
    let mut context_values = BTreeMap::new();
    for row in context {
        context_values.insert(string_field(row, "measure")?, row);
    }
    for (measure, expected) in [
        ("aggregate FFS Medicare hospital margin", -12.1),
        (
            "efficient hospital projected 2026 median FFS Medicare margin",
            1.0,
        ),
        ("all-payer operating margin", 6.5),
        ("commercial hospital price reference", 253.0),
    ] {
        let row = context_values
            .get(measure)
            .ok_or("health/Medicare numeric context missing")?;
        if (number_field(row, "value")? - expected).abs() > 0.000001 {
            return Err(format!(
                "health/Medicare provider adequacy context value failed: {measure}"
            ));
        }
    }
    for (measure, expected) in [
        ("hospital access status", "good_overall"),
        ("hospital quality status", "mixed"),
    ] {
        let row = context_values
            .get(measure)
            .ok_or("health/Medicare status context missing")?;
        if string_field(row, "value")? != expected {
            return Err(format!(
                "health/Medicare provider adequacy context status failed: {measure}"
            ));
        }
    }
    let boundary = string_field(baseline, "boundary")?;
    for required in [
        "not a universal Medicare-relative target",
        "not net savings",
        "not solver input",
    ] {
        if !boundary.contains(required) {
            return Err(format!(
                "health/Medicare provider adequacy boundary missing {required}"
            ));
        }
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!("health/Medicare {field} must remain null"));
        }
    }
    let readiness = record
        .get("readiness_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("health/Medicare provider adequacy readiness")?;
    for field in [
        "threshold_rationale_ready",
        "threshold_value_populated",
        "baseline_value_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health/Medicare readiness {field} must be true"));
        }
    }
    for (field, value) in readiness {
        let observed = value.as_bool().ok_or("health/Medicare readiness bool")?;
        if !matches!(
            field.as_str(),
            "threshold_rationale_ready" | "threshold_value_populated" | "baseline_value_ready"
        ) && observed
        {
            return Err(format!(
                "health/Medicare readiness {field} must remain false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("health/Medicare provider adequacy blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "health/Medicare blocked output {field} must remain null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health/Medicare provider adequacy claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("health/Medicare claim bool")?;
        if matches!(
            field.as_str(),
            "provider_adequacy_margin_floor_value_packet_published"
                | "threshold_rationale_ready"
                | "threshold_value_populated"
                | "baseline_value_ready"
        ) {
            if !observed {
                return Err(format!("health/Medicare claim {field} must be true"));
            }
        } else if observed {
            return Err(format!("health/Medicare claim {field} must remain false"));
        }
    }

    let public_warning = string_field(&record, "public_warning")?;
    for required in [
        "draft no-regression health/Medicare provider-adequacy margin floor threshold",
        "not a universal Medicare-relative target",
        "not access-floor passage",
        "not quality-floor passage",
        "not rural or safety-net capacity evidence",
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
                "health/Medicare provider adequacy warning missing {required}"
            ));
        }
    }

    let schema = fs::read_to_string(
        root.join(HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_SCHEMA_PATH}: {err}"
        )
    })?;
    if !schema.contains("health_medicare_provider_adequacy_margin_floor_value_packet") {
        return Err("health/Medicare provider adequacy schema missing record family".to_string());
    }

    let reader = fs::read_to_string(
        root.join(HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_READER_PATH}: {err}"
        )
    })?;
    for required in [
        HEALTH_MEDICARE_PROVIDER_ADEQUACY_MARGIN_FLOOR_VALUE_PACKET_JSON_PATH,
        HEALTH_TARGET_ADMISSIBILITY_JSON_PATH,
        "-1.0 percent",
        "-12.1 percent",
        "253 percent of Medicare",
        "draft no-regression health/Medicare provider-adequacy margin floor threshold",
        "not a universal Medicare-relative target",
        "policy and stress values remain null",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health/Medicare provider adequacy reader missing {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_floor_source_capture_status(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH,
        HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_SCHEMA_PATH,
        HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health floor source capture artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-floor-source-capture-status:v1"
        || string_field(&record, "record_family")? != "health_floor_source_capture_status"
        || int_field(&record, "pulse")? != 178
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "lane_floor_readiness_rollup_path")?
            != LANE_FLOOR_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "health_outcome_floor_definition_packet_path")?
            != HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "current_law_fy2025_17_row_ledger_custody_path")?
            != CURRENT_LAW_FY2025_17_ROW_LEDGER_CUSTODY_JSON_PATH
        || string_field(&record, "current_law_fy2025_dedicated_receipt_anchors_path")?
            != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&record, "health_national_phi_sensitivity_path")?
            != HEALTH_NATIONAL_PHI_JSON_PATH
    {
        return Err("health floor source capture identity failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("health floor source custody")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "new_external_download_performed",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "partial_fiscal_source_custody_ready",
        "partial_floor_indicator_context_custody_ready",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health floor source custody must be true: {field}"));
        }
    }
    for field in [
        "floor_indicator_source_custody_ready",
        "threshold_source_custody_ready",
        "baseline_floor_value_source_custody_ready",
        "policy_floor_value_source_custody_ready",
        "stress_floor_value_source_custody_ready",
        "source_capture_complete",
        "solver_input_ready",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "health floor source custody must be false: {field}"
            ));
        }
    }

    let fiscal_sources = record
        .get("captured_fiscal_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health captured fiscal sources")?;
    let expected_sources = [
        (
            "SRC-OMB-HIST-3-2-FY2027",
            (
                60343,
                "78100f3efb1a6b08d675b24af173a57359e47dce103a2f1499d905a4bbba06ce",
            ),
        ),
        (
            "SRC-OMB-HIST-2-4-FY2027",
            (
                26752,
                "21d071576d5627a18c3f62de86bfc7faeced1a68265f2db87b4f737b2773c5bd",
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let observed_sources = fiscal_sources
        .iter()
        .map(|source| string_field(source, "source_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_ids = expected_sources
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_sources != expected_ids || fiscal_sources.len() != 2 {
        return Err("health floor fiscal source set failed".to_string());
    }
    for source in fiscal_sources {
        let source_id = string_field(source, "source_id")?;
        let (expected_bytes, expected_sha) = expected_sources
            .get(source_id.as_str())
            .ok_or("health expected source")?;
        if int_field(source, "raw_byte_count")? != *expected_bytes
            || string_field(source, "raw_sha256")? != *expected_sha
            || source
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || source
                .get("may_populate_floor_threshold_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("health fiscal source custody failed: {source_id}"));
        }
        if !root
            .join(string_field(source, "raw_artifact_path")?)
            .exists()
            || !root.join(string_field(source, "metadata_path")?).exists()
        {
            return Err(format!("health fiscal source file missing: {source_id}"));
        }
    }

    let context = record
        .get("current_law_context_values")
        .ok_or("health current-law context")?;
    if int_field(context, "medicare_current_law_outlays_musd")? != 996718
        || int_field(context, "non_medicare_health_current_law_outlays_musd")? != 978511
        || int_field(
            context,
            "combined_health_and_medicare_current_law_outlays_musd",
        )? != 1975229
        || int_field(context, "medicare_hi_payroll_receipt_anchor_musd")? != 395350
        || context
            .get("may_populate_floor_threshold_or_pass_fail")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || context
            .get("may_populate_solver_input")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health current-law context values failed".to_string());
    }

    let captured_floor_context = record
        .get("captured_floor_context_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health captured floor context sources")?;
    if captured_floor_context.len() != 7 {
        return Err("health captured floor context source count failed".to_string());
    }
    let nhe_context = &captured_floor_context[0];
    if string_field(nhe_context, "source_id")? != "SRC-CMS-NHE-TABLES-2024"
        || string_field(nhe_context, "source_family")? != "CMS national health expenditure accounts"
        || int_field(nhe_context, "raw_byte_count")? != 520391
        || string_field(nhe_context, "raw_sha256")?
            != "a09ef6d3e84e25d745047a47b6b08a0d96b303085b4c725b67ce67a0eb0c4420"
        || nhe_context
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || nhe_context
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || nhe_context
            .get("may_populate_floor_threshold_or_pass_fail")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health captured NHE floor context failed".to_string());
    }
    if !root
        .join(string_field(nhe_context, "raw_artifact_path")?)
        .exists()
        || !root
            .join(string_field(nhe_context, "metadata_path")?)
            .exists()
    {
        return Err("health captured NHE floor context file missing".to_string());
    }
    let expected_pdc_sources = [
        (
            "SRC-CMS-PDC-HOSPITAL-GENERAL-INFORMATION-2026-05-13",
            (
                1453884,
                "83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40",
            ),
        ),
        (
            "SRC-CMS-PDC-COMPLICATIONS-DEATHS-HOSPITAL-2026-05-13",
            (
                22982118,
                "4edd2c4c9aa3596b97a374a91250f0bcc225c576a609907166cb2c64fc003720",
            ),
        ),
        (
            "SRC-CMS-PDC-HAI-HOSPITAL-2026-05-13",
            (
                38658196,
                "90be9098189e6922e7a0387f085cc34121eca476f0b50361423261cb7d12948f",
            ),
        ),
        (
            "SRC-CMS-PDC-UNPLANNED-HOSPITAL-VISITS-HOSPITAL-2026-05-13",
            (
                19035194,
                "6f8f59fed5a56e78868d8a4d73f1a78341168cc07f3536b72be952f35c76751d",
            ),
        ),
        (
            "SRC-CMS-PDC-TIMELY-EFFECTIVE-CARE-HOSPITAL-2026-05-13",
            (
                34178467,
                "5d39e1fd8b7b272fe83f7b53e2f69288c997dfb4d28b68dd74454e80e7d860e9",
            ),
        ),
        (
            "SRC-CMS-PDC-RURAL-EMERGENCY-TIMELY-EFFECTIVE-CARE-HOSPITAL-2026-05-13",
            (
                59753,
                "c83bdee86d813a9a23b642cc3ed159825cef355e9f025c274f50f64cd12568e0",
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    for context_source in captured_floor_context.iter().skip(1) {
        let source_id = string_field(context_source, "source_id")?;
        let (expected_bytes, expected_sha) = expected_pdc_sources
            .get(source_id.as_str())
            .ok_or("health expected PDC floor context")?;
        if string_field(context_source, "source_family")?
            != "CMS Provider Data Catalog hospital quality/access context"
            || int_field(context_source, "raw_byte_count")? != *expected_bytes
            || string_field(context_source, "raw_sha256")? != *expected_sha
            || context_source
                .get("needed_for")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
            || context_source
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || context_source
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || context_source
                .get("may_populate_floor_threshold_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!(
                "health captured PDC floor context failed: {source_id}"
            ));
        }
        if !root
            .join(string_field(context_source, "raw_artifact_path")?)
            .exists()
            || !root
                .join(string_field(context_source, "metadata_path")?)
                .exists()
        {
            return Err(format!(
                "health captured PDC floor context file missing: {source_id}"
            ));
        }
    }

    let needed = record
        .get("source_candidates_still_needed")
        .and_then(serde_json::Value::as_array)
        .ok_or("health source candidates still needed")?;
    if needed.len() != 3 {
        return Err("health source candidate count failed".to_string());
    }
    for candidate in needed {
        if string_field(candidate, "source_family")?.is_empty()
            || candidate
                .get("needed_for")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
        {
            return Err("health source candidate fields missing".to_string());
        }
        for field in ["raw_artifact_path", "raw_byte_count", "raw_sha256"] {
            if !candidate.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "health source candidate field must be null: {field}"
                ));
            }
        }
        if candidate
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err("health source candidate custody must be false".to_string());
        }
    }

    let floors = record
        .get("floor_value_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("health floor value status")?;
    let observed_floors = floors
        .iter()
        .map(|floor| string_field(floor, "floor_class"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_floors = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_floors != expected_floors || floors.len() != 5 {
        return Err("health floor set failed".to_string());
    }
    for floor in floors {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if !floor.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!("health floor value must be null: {field}"));
            }
        }
        if floor.get("passed").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err("health floor passed flag must be false".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("health blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("health blocked output must be null: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health floor source claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("health claim bool")?;
        if matches!(
            field.as_str(),
            "health_floor_source_capture_status_published"
                | "partial_fiscal_source_custody_ready"
                | "partial_floor_indicator_context_custody_ready"
        ) {
            if !observed {
                return Err(format!("health source claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("health source claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "partial FY2025 fiscal source custody, CMS NHE raw context custody, and partial CMS Provider Data Catalog hospital quality/access context custody only",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
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
            return Err(format!("health source warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH,
        "Existing OMB FY2027 Historical Table 3.2 custody supports FY2025 Medicare",
        "Existing OMB FY2027 Historical Table 2.4 custody supports the FY2025 Medicare HI dedicated receipt anchor.",
        "CMS national health expenditure source bytes are captured for context-only",
        "CMS Provider Data Catalog hospital quality/access source bytes are partially",
        "CBO federal health baseline source bytes",
        "remaining CMS quality/access measure lineage",
        "threshold selection",
        "observed baseline, policy, or stress floor values",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("health source reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_medicare_trustees_source_capture_status(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH,
        HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_SCHEMA_PATH,
        HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health Medicare Trustees source capture artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-medicare-trustees-source-capture-status:v1"
        || string_field(&record, "record_family")?
            != "health_medicare_trustees_source_capture_status"
        || int_field(&record, "pulse")? != 179
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "health_floor_source_capture_status_path")?
            != HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "receipt_base_official_source_capture_path")?
            != RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH
        || string_field(&record, "medicare_part_financing_path")?
            != MEDICARE_PART_FINANCING_CY2025_CMS_TRUSTEES_JSONL_PATH
        || string_field(&record, "medicare_denominator_values_path")?
            != MEDICARE_DENOMINATOR_VALUES_CY2025_CMS_TRUSTEES_JSONL_PATH
    {
        return Err("health Medicare Trustees source capture identity failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("health Medicare Trustees custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_medicare_trustees_source_custody_ready",
        "may_populate_medicare_financing_and_enrollment_context",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "health Medicare Trustees custody must be true: {field}"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "may_populate_health_floor_thresholds",
        "may_populate_observed_floor_values",
        "may_populate_policy_or_stress_floor_values",
        "may_populate_pass_fail_findings",
        "may_populate_solver_inputs",
        "health_floor_source_capture_complete",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "health Medicare Trustees custody must be false: {field}"
            ));
        }
    }

    let source = record
        .get("captured_source")
        .ok_or("health Medicare Trustees captured source")?;
    if string_field(source, "source_id")? != "SRC-CMS-MEDICARE-TRUSTEES-2026"
        || int_field(source, "raw_byte_count")? != 2844621
        || string_field(source, "raw_sha256")?
            != "ffa56b9137006872300b0346149eae1613d09a172b6ba118aad48e66dfc48fa8"
        || source
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || source
            .get("floor_threshold_or_pass_fail_use_allowed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health Medicare Trustees captured source failed".to_string());
    }
    if !root
        .join(string_field(source, "raw_artifact_path")?)
        .exists()
        || !root.join(string_field(source, "metadata_path")?).exists()
    {
        return Err("health Medicare Trustees source files missing".to_string());
    }

    let financing_rows =
        read_jsonl(root.join(MEDICARE_PART_FINANCING_CY2025_CMS_TRUSTEES_JSONL_PATH))?;
    let financing_parts = financing_rows
        .iter()
        .map(|row| string_field(row, "program_part"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_financing = ["HI", "Part B", "Part D"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if financing_rows.len() != 3 || financing_parts != expected_financing {
        return Err("health Medicare Trustees financing row set failed".to_string());
    }
    for row in &financing_rows {
        if string_field(row, "year_basis")? != "calendar_year"
            || int_field(row, "calendar_year")? != 2025
            || !row
                .get("source_ids")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|ids| {
                    ids.iter()
                        .any(|id| id.as_str() == Some("SRC-CMS-MEDICARE-TRUSTEES-2026"))
                })
            || {
                let rule = string_field(row, "public_use_rule")?;
                !rule.contains("Do not") && !rule.contains("do not")
            }
        {
            return Err("health Medicare Trustees financing row boundary failed".to_string());
        }
    }

    let denominator_rows =
        read_jsonl(root.join(MEDICARE_DENOMINATOR_VALUES_CY2025_CMS_TRUSTEES_JSONL_PATH))?;
    let denominator_ids = denominator_rows
        .iter()
        .map(|row| string_field(row, "denominator_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_denominators = [
        "medicare_part_a_enrollment",
        "medicare_part_b_enrollment",
        "medicare_part_d_enrollment",
        "medicare_total_beneficiaries",
        "medicare_private_health_plan_enrollment",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if denominator_rows.len() != 5 || denominator_ids != expected_denominators {
        return Err("health Medicare Trustees denominator row set failed".to_string());
    }
    for row in &denominator_rows {
        if string_field(row, "year_basis")? != "calendar_year"
            || string_field(row, "year")? != "CY2025"
            || string_field(row, "unit")? != "people"
            || !row
                .get("source_ids")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|ids| {
                    ids.iter()
                        .any(|id| id.as_str() == Some("SRC-CMS-MEDICARE-TRUSTEES-2026"))
                })
            || !string_field(row, "public_use_rule")?.contains("Do not")
        {
            return Err("health Medicare Trustees denominator row boundary failed".to_string());
        }
    }

    let groups = record
        .get("captured_context_groups")
        .and_then(serde_json::Value::as_array)
        .ok_or("health Medicare Trustees context groups")?;
    if groups.len() != 2 {
        return Err("health Medicare Trustees context group count failed".to_string());
    }
    for group in groups {
        let group_id = string_field(group, "group_id")?;
        let expected_count = match group_id.as_str() {
            "medicare_part_financing_cy2025" => 3,
            "medicare_enrollment_denominators_cy2025" => 5,
            _ => {
                return Err(format!(
                    "unexpected health Medicare Trustees group: {group_id}"
                ));
            }
        };
        if int_field(group, "record_count")? != expected_count
            || group
                .get("may_populate_floor_threshold_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || group
                .get("may_populate_solver_input")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!(
                "health Medicare Trustees context group failed: {group_id}"
            ));
        }
    }

    let gaps = record
        .get("remaining_health_floor_source_gaps")
        .and_then(serde_json::Value::as_array)
        .ok_or("health Medicare Trustees remaining gaps")?;
    if gaps.len() != 4 {
        return Err("health Medicare Trustees remaining gap count failed".to_string());
    }
    for gap in gaps {
        if string_field(gap, "source_family")?.is_empty()
            || string_field(gap, "gap")?.is_empty()
            || gap
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("health Medicare Trustees remaining gap failed".to_string());
        }
    }

    let floors = record
        .get("floor_value_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("health Medicare Trustees floor values")?;
    if floors.len() != 5 {
        return Err("health Medicare Trustees floor count failed".to_string());
    }
    for floor in floors {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if !floor.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "health Medicare Trustees floor value must be null: {field}"
                ));
            }
        }
        if floor.get("passed").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err("health Medicare Trustees floor passed flag must be false".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("health Medicare Trustees blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "health Medicare Trustees blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health Medicare Trustees claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("health Medicare Trustees claim bool")?;
        if matches!(
            field.as_str(),
            "health_medicare_trustees_source_capture_status_published"
                | "cms_medicare_trustees_source_custody_ready"
                | "medicare_financing_and_enrollment_context_ready"
        ) {
            if !observed {
                return Err(format!(
                    "health Medicare Trustees claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "health Medicare Trustees claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "CY2025 Medicare financing and enrollment context only",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
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
                "health Medicare Trustees warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH,
        "CMS Medicare Trustees 2026 raw PDF custody",
        "CY2025 Medicare HI, Part B, and Part D financing context",
        "CY2025 Medicare Part A, Part B, Part D, total beneficiary, and private-plan enrollment denominator context.",
        "CMS NHE source bytes",
        "CBO federal health baseline source bytes",
        "threshold rationale and stronger-model review",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
        "not target-cost selection",
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
                "health Medicare Trustees reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_nhe_source_custody_gap(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH,
        HEALTH_NHE_SOURCE_CUSTODY_GAP_SCHEMA_PATH,
        HEALTH_NHE_SOURCE_CUSTODY_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health NHE source custody gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-nhe-source-custody-gap:v1"
        || string_field(&record, "record_family")? != "health_nhe_source_custody_gap"
        || int_field(&record, "pulse")? != 180
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "health_floor_source_capture_status_path")?
            != HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(
            &record,
            "health_medicare_trustees_source_capture_status_path",
        )? != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "health_national_phi_sensitivity_path")?
            != HEALTH_NATIONAL_PHI_JSON_PATH
        || string_field(&record, "health_service_price_volume_bridge_path")?
            != HEALTH_SERVICE_BRIDGE_JSON_PATH
    {
        return Err("health NHE source custody gap identity failed".to_string());
    }

    let referenced = record
        .get("source_ids_referenced_with_raw_custody_ready")
        .and_then(serde_json::Value::as_array)
        .ok_or("health NHE referenced source ids")?;
    let observed = referenced
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect::<BTreeSet<_>>();
    let expected = ["SRC-CMS-NHE-TABLES-2024", "SRC-CMS-NHE-2024"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed != expected || referenced.len() != 2 {
        return Err("health NHE referenced source set failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("health NHE custody status")?;
    for field in [
        "official_sources_only",
        "new_external_download_performed",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "nhe_source_referenced_in_derived_health_artifacts",
        "nhe_raw_artifact_path_present",
        "nhe_metadata_path_present",
        "nhe_raw_sha256_present",
        "nhe_source_custody_ready",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health NHE custody must be true: {field}"));
        }
    }
    for field in [
        "used_existing_captured_sources_only",
        "nhe_values_may_populate_floor_thresholds",
        "nhe_values_may_populate_observed_floor_values",
        "nhe_values_may_populate_solver_inputs",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("health NHE custody must be false: {field}"));
        }
    }

    let captured = record
        .get("captured_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health NHE captured sources")?;
    let expected_captured = [
        (
            "SRC-CMS-NHE-TABLES-2024",
            (
                520391,
                "a09ef6d3e84e25d745047a47b6b08a0d96b303085b4c725b67ce67a0eb0c4420",
            ),
        ),
        (
            "SRC-CMS-NHE-2024",
            (
                214153,
                "b03aeda7424c6edc53b7b947341d89862603f813bb1e0924faad82de5fcdf8e1",
            ),
        ),
        (
            "SRC-CMS-NHE-SUMMARY-2024",
            (
                19575,
                "ef92c5602a96ffebd5ec0d313f0e7dc55bca979178ca53d8e519cfd51bd99256",
            ),
        ),
        (
            "SRC-CMS-NHE-SERVICE-SOURCE-2024",
            (
                123721,
                "b6a9e26774ca36931d42add46204947d6335fc4898011780563196898f964746",
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if captured.len() != expected_captured.len() {
        return Err("health NHE captured source count failed".to_string());
    }
    for source in captured {
        let source_id = string_field(source, "source_id")?;
        let (expected_bytes, expected_sha) = expected_captured
            .get(source_id.as_str())
            .ok_or("health NHE unexpected captured source")?;
        if int_field(source, "raw_byte_count")? != *expected_bytes
            || string_field(source, "raw_sha256")? != *expected_sha
            || source
                .get("custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || source
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || source
                .get("may_populate_floor_threshold_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || source
                .get("may_populate_solver_input")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("health NHE captured source failed: {source_id}"));
        }
        if !root
            .join(string_field(source, "raw_artifact_path")?)
            .exists()
            || !root.join(string_field(source, "metadata_path")?).exists()
        {
            return Err(format!(
                "health NHE captured source file missing: {source_id}"
            ));
        }
    }

    let refs = record
        .get("referencing_artifacts")
        .and_then(serde_json::Value::as_array)
        .ok_or("health NHE referencing artifacts")?;
    if refs.len() != 2 {
        return Err("health NHE referencing artifact count failed".to_string());
    }
    for item in refs {
        let path = string_field(item, "artifact_path")?;
        if path != HEALTH_NATIONAL_PHI_JSON_PATH && path != HEALTH_SERVICE_BRIDGE_JSON_PATH {
            return Err(format!(
                "unexpected health NHE referencing artifact: {path}"
            ));
        }
        if !root.join(&path).exists()
            || item
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || item
                .get("may_populate_floor_threshold_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("health NHE referencing artifact failed: {path}"));
        }
    }

    let requirements = record
        .get("nhe_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("health NHE capture requirements")?;
    if requirements.len() != 6 {
        return Err("health NHE capture requirement count failed".to_string());
    }
    for req in requirements {
        if string_field(req, "requirement")?.is_empty()
            || req.get("value").is_some_and(serde_json::Value::is_null)
            || req.get("ready").and_then(serde_json::Value::as_bool) != Some(true)
        {
            return Err("health NHE capture requirement must be populated/true".to_string());
        }
    }

    let blocked = record
        .get("blocked_health_floor_uses")
        .and_then(serde_json::Value::as_object)
        .ok_or("health NHE blocked floor uses")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("health NHE blocked use must be null: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health NHE claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("health NHE claim bool")?;
        if matches!(
            field.as_str(),
            "health_nhe_source_custody_gap_published"
                | "nhe_source_referenced_in_derived_health_artifacts"
                | "nhe_source_custody_ready"
                | "nhe_raw_artifact_captured"
                | "nhe_metadata_captured"
        ) {
            if !observed {
                return Err(format!("health NHE claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("health NHE claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "CMS NHE raw custody is ready",
        "NHE source capture for raw custody only",
        "not health floor threshold selection",
        "not observed floor values",
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
            return Err(format!("health NHE warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(HEALTH_NHE_SOURCE_CUSTODY_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH,
        "Referenced with raw custody ready",
        "SRC-CMS-NHE-TABLES-2024",
        "SRC-CMS-NHE-2024",
        "nhe-tables.zip",
        "highlights.pdf",
        "NHE source capture for raw custody only",
        "not health floor threshold selection",
        "not observed floor values",
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
        if !reader.contains(required) {
            return Err(format!("health NHE reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_cbo_source_custody_gap(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH,
        HEALTH_CBO_SOURCE_CUSTODY_GAP_SCHEMA_PATH,
        HEALTH_CBO_SOURCE_CUSTODY_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health CBO source custody gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-cbo-source-custody-gap:v1"
        || string_field(&record, "record_family")? != "health_cbo_source_custody_gap"
        || int_field(&record, "pulse")? != 181
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "health_floor_source_capture_status_path")?
            != HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(
            &record,
            "health_medicare_trustees_source_capture_status_path",
        )? != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "health_nhe_source_custody_gap_path")?
            != HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH
        || string_field(&record, "cbo_health_insurance_table2_browser_rowmap_path")?
            != CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
    {
        return Err("health CBO source custody gap identity failed".to_string());
    }

    let referenced = record
        .get("source_ids_referenced_but_not_raw_custody_ready")
        .and_then(serde_json::Value::as_array)
        .ok_or("health CBO referenced source ids")?;
    let observed = referenced
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_string())
        .collect::<BTreeSet<_>>();
    let expected = ["SRC-CBO-LTBO", "SRC-CBO-COMMERCIAL-PROVIDER-PRICES"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed != expected || referenced.len() != 2 {
        return Err("health CBO referenced source set failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("health CBO custody status")?;
    for field in [
        "official_sources_only",
        "new_external_download_performed",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cbo_source_referenced_in_derived_health_artifacts",
        "cbo_official_browser_access_ready",
        "cbo_command_line_download_blocked",
        "cbo_february_2026_health_baseline_raw_custody_ready",
        "cbo_table2_browser_rowmap_context_ready",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health CBO custody must be true: {field}"));
        }
    }
    for field in [
        "used_existing_captured_sources_only",
        "cbo_raw_artifact_path_present",
        "cbo_metadata_path_present",
        "cbo_raw_sha256_present",
        "cbo_source_custody_ready",
        "cbo_values_may_populate_federal_policy_translation",
        "cbo_values_may_populate_behavior_or_incidence",
        "cbo_values_may_populate_solver_inputs",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("health CBO custody must be false: {field}"));
        }
    }

    let access = record
        .get("official_access_boundary")
        .and_then(serde_json::Value::as_array)
        .ok_or("health CBO official access boundary")?;
    if access.len() != 3 {
        return Err("health CBO official access boundary count failed".to_string());
    }
    for item in access {
        let source_id = string_field(item, "source_id")?;
        if !matches!(
            source_id.as_str(),
            "SRC-CBO-LTBO"
                | "SRC-CBO-COMMERCIAL-PROVIDER-PRICES"
                | "no_source_id_assigned_cbo_62380_browser_context_only"
        ) || string_field(item, "publisher")? != "Congressional Budget Office"
            || string_field(item, "browser_review_date")? != "2026-07-24"
            || !string_field(item, "browser_review_evidence")?.contains("HTTP 403")
            || !item
                .get("local_raw_artifact_path")
                .is_some_and(serde_json::Value::is_null)
            || !item
                .get("raw_byte_count")
                .is_some_and(serde_json::Value::is_null)
            || !item
                .get("raw_sha256")
                .is_some_and(serde_json::Value::is_null)
            || item
                .get("access_boundary_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || item
                .get("local_raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || item
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || item
                .get("may_populate_policy_translation_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("health CBO access boundary failed: {source_id}"));
        }
    }

    let refs = record
        .get("referencing_artifacts")
        .and_then(serde_json::Value::as_array)
        .ok_or("health CBO referencing artifacts")?;
    let expected_paths = [
        "data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl",
        "docs/reading/health-price-discipline-source-packet.md",
        "docs/reading/health-administrative-simplification-source-packet.md",
        HEALTH_CATEGORY_BENCHMARK_JSON_PATH,
        CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH,
        CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH,
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_paths = refs
        .iter()
        .map(|item| string_field(item, "artifact_path"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_paths != expected_paths || refs.len() != 6 {
        return Err("health CBO referencing artifact set failed".to_string());
    }
    for item in refs {
        let path = string_field(item, "artifact_path")?;
        let expected_raw_ready =
            path == CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH;
        if expected_raw_ready {
            let source_ids = item
                .get("referenced_source_ids")
                .and_then(serde_json::Value::as_array)
                .ok_or("health CBO browser context referenced source ids")?;
            if !source_ids.iter().any(|source| {
                source.as_str() == Some("no_source_id_assigned_cbo_62380_browser_context_only")
            }) || !string_field(item, "allowed_use")?
                .contains("May 2026 browser-visible presentation context")
            {
                return Err(
                    "health CBO browser context reference missing May presentation boundary"
                        .to_string(),
                );
            }
        }
        if !root.join(&path).exists()
            || item
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(expected_raw_ready)
            || item
                .get("may_populate_policy_translation_or_pass_fail")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err(format!("health CBO referencing artifact failed: {path}"));
        }
    }

    let requirements = record
        .get("cbo_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("health CBO capture requirements")?;
    if requirements.len() != 11 {
        return Err("health CBO capture requirement count failed".to_string());
    }
    for req in requirements {
        let requirement = string_field(req, "requirement")?;
        if matches!(
            requirement.as_str(),
            "official_access_boundary"
                | "february_2026_health_baseline_raw_custody"
                | "may_2026_health_subsidy_presentation_browser_context"
                | "browser_verified_table2_rowmap"
        ) {
            if req.get("value").is_some_and(serde_json::Value::is_null)
                || req.get("ready").and_then(serde_json::Value::as_bool) != Some(true)
            {
                return Err(format!(
                    "health CBO ready requirement failed: {requirement}"
                ));
            }
        } else if requirement.is_empty()
            || !req.get("value").is_some_and(serde_json::Value::is_null)
            || req.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("health CBO capture requirement must remain null/false".to_string());
        }
    }

    let blocked = record
        .get("blocked_health_policy_uses")
        .and_then(serde_json::Value::as_object)
        .ok_or("health CBO blocked policy uses")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("health CBO blocked use must be null: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health CBO claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("health CBO claim bool")?;
        if matches!(
            field.as_str(),
            "health_cbo_source_custody_gap_published"
                | "cbo_source_referenced_in_derived_health_artifacts"
                | "cbo_official_browser_access_ready"
                | "cbo_command_line_download_blocked"
                | "cbo_february_2026_health_baseline_raw_custody_ready"
                | "cbo_table2_browser_rowmap_context_ready"
        ) {
            if !observed {
                return Err(format!("health CBO claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("health CBO claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "official CBO browser access is documented",
        "February 2026 CBO health-insurance PDF/spreadsheet raw files are captured",
        "May 11, 2026 CBO health-subsidy presentation page/PDF/data workbook are browser-visible context only",
        "February 2026 health-insurance Table 2 rowmap is assigned as context only",
        "May and July 2026 raw files remain blocked",
        "CBO source capture is still incomplete",
        "not complete CBO source capture",
        "not federal health policy translation",
        "not behavior modeling",
        "not incidence modeling",
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
            return Err(format!("health CBO warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(HEALTH_CBO_SOURCE_CUSTODY_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH,
        "Referenced but not raw-custody-ready",
        "SRC-CBO-LTBO",
        "SRC-CBO-COMMERCIAL-PROVIDER-PRICES",
        "official access boundary",
        "February 2026 health baseline raw custody",
        "browser-verified Table 2 rowmap",
        "SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02",
        "no_source_id_assigned_cbo_62380_browser_context_only",
        "62380-Data.xlsx",
        "cbo-health-insurance-table2-browser-rowmap-fy2026-2036.md",
        "local spreadsheet custody",
        "HTTP 403",
        "raw artifact path",
        "raw byte count",
        "raw SHA-256",
        "metadata path",
        "retrieval date",
        "health baseline table lineage",
        "behavior and incidence table lineage",
        "not complete CBO source capture",
        "not federal health policy translation",
        "not behavior modeling",
        "not incidence modeling",
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
        if !reader.contains(required) {
            return Err(format!("health CBO reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_quality_access_indicator_source_gap(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH,
        HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_SCHEMA_PATH,
        HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health quality/access indicator source gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-quality-access-indicator-source-gap:v1"
        || string_field(&record, "record_family")? != "health_quality_access_indicator_source_gap"
        || int_field(&record, "pulse")? != 182
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "health_floor_source_capture_status_path")?
            != HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(
            &record,
            "health_medicare_trustees_source_capture_status_path",
        )? != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "health_nhe_source_custody_gap_path")?
            != HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH
        || string_field(&record, "health_cbo_source_custody_gap_path")?
            != HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH
        || string_field(
            &record,
            "cms_hospital_quality_methodology_surface_context_path",
        )? != CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH
        || string_field(
            &record,
            "cms_hospital_measure_methodology_report_custody_path",
        )? != CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH
        || string_field(&record, "cms_hospital_quality_dataset_field_crosswalk_path")?
            != CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH
        || string_field(&record, "cms_hrsa_rural_safety_net_capacity_context_path")?
            != CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH
        || string_field(&record, "health_outcome_floor_definition_packet_path")?
            != HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
    {
        return Err("health quality/access indicator source gap identity failed".to_string());
    }

    let families = record
        .get("source_families_needed_but_not_custody_ready")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access source families")?;
    let observed = families
        .iter()
        .map(|value| value.as_str().unwrap_or_default().to_string())
        .collect::<BTreeSet<_>>();
    let expected = [
        "complete CMS quality/access denominator-to-dataset field crosswalk",
        "complete all-measure risk-adjusted outcome case-mix methodology crosswalk across selected floor indicators",
        "rural and safety-net capacity series beyond rural emergency hospital timely/effective-care context",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || families.len() != 3 {
        return Err("health quality/access source family set failed".to_string());
    }

    let custody = record
        .get("source_custody_status")
        .ok_or("health quality/access custody status")?;
    for field in [
        "official_sources_only",
        "new_external_download_performed",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "quality_access_source_family_declared",
        "quality_access_raw_artifact_path_present",
        "quality_access_metadata_path_present",
        "quality_access_raw_sha256_present",
        "quality_access_partial_source_custody_ready",
        "quality_access_lineage_context_captured",
        "qualitynet_methodology_surface_context_captured",
        "selected_mortality_methodology_report_content_captured",
        "methodology_report_content_custody_ready",
        "partial_denominator_field_crosswalk_captured",
        "partial_rural_safety_net_capacity_context_captured",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "health quality/access custody must be true: {field}"
            ));
        }
    }
    for field in [
        "used_existing_captured_sources_only",
        "quality_access_source_custody_ready",
        "quality_access_values_may_populate_thresholds",
        "quality_access_values_may_populate_observed_values",
        "quality_access_values_may_populate_pass_fail",
        "quality_access_values_may_populate_solver_inputs",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "health quality/access custody must be false: {field}"
            ));
        }
    }

    let captured = record
        .get("captured_indicator_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured indicator sources")?;
    let expected_captured = [
        (
            "SRC-CMS-PDC-HOSPITAL-GENERAL-INFORMATION-2026-05-13",
            (
                "xubh-q36u",
                5432,
                1453884,
                "83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40",
                1215,
                "a421368204acb1b91b4074ef797145aac3a11be132ae285730577b151e370cc4",
            ),
        ),
        (
            "SRC-CMS-PDC-COMPLICATIONS-DEATHS-HOSPITAL-2026-05-13",
            (
                "ynj2-r877",
                95840,
                22982118,
                "4edd2c4c9aa3596b97a374a91250f0bcc225c576a609907166cb2c64fc003720",
                1253,
                "e2f8cbc077d40dc54c832ed78bb929d6f8d8a8577482d56c1a5cd7f5ba1b4ad6",
            ),
        ),
        (
            "SRC-CMS-PDC-HAI-HOSPITAL-2026-05-13",
            (
                "77hc-ibv8",
                172512,
                38658196,
                "90be9098189e6922e7a0387f085cc34121eca476f0b50361423261cb7d12948f",
                1810,
                "861c224648f3693d6b57eddcf0ede22b76b7844e97631a62cfc6688a3dc42915",
            ),
        ),
        (
            "SRC-CMS-PDC-UNPLANNED-HOSPITAL-VISITS-HOSPITAL-2026-05-13",
            (
                "632h-zaca",
                67088,
                19035194,
                "6f8f59fed5a56e78868d8a4d73f1a78341168cc07f3536b72be952f35c76751d",
                1574,
                "8e54a39daeda7d9ca3c856bc955be131b95feabf98be2f224026bc63889a4b03",
            ),
        ),
        (
            "SRC-CMS-PDC-TIMELY-EFFECTIVE-CARE-HOSPITAL-2026-05-13",
            (
                "yv7e-xc69",
                138173,
                34178467,
                "5d39e1fd8b7b272fe83f7b53e2f69288c997dfb4d28b68dd74454e80e7d860e9",
                1306,
                "220d1e02b9c49c60fd9c3282a88493e9f01fc79325f20246eaf4d47389e270d8",
            ),
        ),
        (
            "SRC-CMS-PDC-RURAL-EMERGENCY-TIMELY-EFFECTIVE-CARE-HOSPITAL-2026-05-13",
            (
                "97xg-v3wv",
                164,
                59753,
                "c83bdee86d813a9a23b642cc3ed159825cef355e9f025c274f50f64cd12568e0",
                1314,
                "2d6fbd5a0ee028efd23b51f3e26f8c25461f34842e23eb1c6a1f15e18f913044",
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let observed_captured = captured
        .iter()
        .map(|source| string_field(source, "source_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_captured_ids = expected_captured
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_captured != expected_captured_ids || captured.len() != 6 {
        return Err("health quality/access captured source set failed".to_string());
    }
    for source in captured {
        let source_id = string_field(source, "source_id")?;
        let (dataset_id, rows, raw_bytes, raw_sha, metadata_bytes, metadata_sha) =
            expected_captured
                .get(source_id.as_str())
                .ok_or("health quality/access expected captured source")?;
        if string_field(source, "dataset_id")? != *dataset_id
            || int_field(source, "row_count")? != *rows
            || int_field(source, "raw_byte_count")? != *raw_bytes
            || string_field(source, "raw_sha256")? != *raw_sha
            || int_field(source, "metadata_byte_count")? != *metadata_bytes
            || string_field(source, "metadata_sha256")? != *metadata_sha
            || string_field(source, "official_host_or_publisher")?
                != "Centers for Medicare & Medicaid Services (CMS)"
            || string_field(source, "retrieval_date")? != "2026-07-24"
            || string_field(source, "released")? != "2026-05-13"
            || !string_field(source, "landing_page_url")?.starts_with("https://data.cms.gov/")
            || !string_field(source, "download_url")?.starts_with("https://data.cms.gov/")
            || string_field(source, "metadata_path")?
                != "data/metadata/SRC-CMS-PDC-HOSPITAL-QUALITY-ACCESS-2026-05-13.2026-07-24.metadata.md"
            || source
                .get("indicator_context")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
            || source
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "health quality/access captured source custody failed: {source_id}"
            ));
        }
        for field in [
            "may_populate_thresholds",
            "may_populate_observed_values",
            "may_populate_pass_fail",
            "may_populate_solver_inputs",
        ] {
            if source.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "health quality/access captured source must block {field}: {source_id}"
                ));
            }
        }
        for path_field in [
            "raw_artifact_path",
            "metadata_artifact_path",
            "metadata_path",
        ] {
            if !root.join(string_field(source, path_field)?).exists() {
                return Err(format!(
                    "health quality/access captured source file missing: {source_id}"
                ));
            }
        }
    }

    let lineage_sources = record
        .get("captured_lineage_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured lineage sources")?;
    if lineage_sources.len() != 1 {
        return Err("health quality/access captured lineage source count failed".to_string());
    }
    let lineage = &lineage_sources[0];
    if string_field(lineage, "source_id")? != "SRC-CMS-PDC-HOSPITAL-DATA-DICTIONARY-2026-04"
        || string_field(lineage, "title")? != "Hospital DDB Data Dictionary April 2026"
        || string_field(lineage, "official_host_or_publisher")?
            != "Centers for Medicare & Medicaid Services (CMS)"
        || string_field(lineage, "source_url")?
            != "https://data.cms.gov/provider-data/sites/default/files/data_dictionaries/hospital/HOSPITAL_Data_Dictionary.pdf"
        || string_field(lineage, "retrieval_date")? != "2026-07-24"
        || string_field(lineage, "raw_artifact_path")?
            != "data/raw/cms/SRC-CMS-PDC-HOSPITAL-DATA-DICTIONARY-2026-04/2026-07-24/HOSPITAL_Data_Dictionary.pdf"
        || int_field(lineage, "raw_byte_count")? != 1291356
        || string_field(lineage, "raw_sha256")?
            != "cd5016abee26e914b273a8fea8ab698710ff60f1c53a1b66e43bbd7168f6cb81"
        || string_field(lineage, "metadata_path")?
            != "data/metadata/SRC-CMS-PDC-HOSPITAL-DATA-DICTIONARY-2026-04.2026-07-24.metadata.md"
        || lineage
            .get("lineage_context")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.len() < 5)
        || lineage
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("health quality/access lineage source custody failed".to_string());
    }
    for field in [
        "may_populate_thresholds",
        "may_populate_observed_values",
        "may_populate_pass_fail",
        "may_populate_solver_inputs",
    ] {
        if lineage.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "health quality/access lineage source must block {field}"
            ));
        }
    }
    for path_field in ["raw_artifact_path", "metadata_path"] {
        if !root.join(string_field(lineage, path_field)?).exists() {
            return Err(format!(
                "health quality/access lineage source file missing: {path_field}"
            ));
        }
    }

    let methodology_surfaces = record
        .get("captured_methodology_surface_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured methodology surface sources")?;
    if methodology_surfaces.len() != 1 {
        return Err(
            "health quality/access captured methodology surface source count failed".to_string(),
        );
    }
    let methodology_surface = &methodology_surfaces[0];
    if string_field(methodology_surface, "source_id")?
        != "SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24"
        || string_field(methodology_surface, "context_path")?
            != CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH
        || string_field(methodology_surface, "reader_path")?
            != CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_READER_PATH
        || string_field(methodology_surface, "metadata_path")?
            != "data/metadata/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24.2026-07-24.metadata.md"
        || int_field(methodology_surface, "raw_file_count")? != 8
        || int_field(methodology_surface, "raw_total_byte_count")? != 14_167_176
        || methodology_surface
            .get("surface_context")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.len() != 3)
        || methodology_surface
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || methodology_surface
            .get("methodology_report_content_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health quality/access methodology surface custody failed".to_string());
    }
    for path_field in ["context_path", "reader_path", "metadata_path"] {
        if !root
            .join(string_field(methodology_surface, path_field)?)
            .exists()
        {
            return Err(format!(
                "health quality/access methodology surface file missing: {path_field}"
            ));
        }
    }
    for field in [
        "may_populate_thresholds",
        "may_populate_observed_values",
        "may_populate_pass_fail",
        "may_populate_solver_inputs",
    ] {
        if methodology_surface
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "health quality/access methodology surface must block {field}"
            ));
        }
    }

    let methodology_reports = record
        .get("captured_methodology_report_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured methodology report sources")?;
    if methodology_reports.len() != 1 {
        return Err(
            "health quality/access captured methodology report source count failed".to_string(),
        );
    }
    let methodology_report = &methodology_reports[0];
    if string_field(methodology_report, "source_id")?
        != "SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02"
        || string_field(methodology_report, "context_path")?
            != CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH
        || string_field(methodology_report, "reader_path")?
            != CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_READER_PATH
        || string_field(methodology_report, "metadata_path")?
            != "data/metadata/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02.2026-07-24.metadata.md"
        || int_field(methodology_report, "raw_file_count")? != 3
        || int_field(methodology_report, "raw_total_byte_count")? != 6_518_093
        || methodology_report
            .get("methodology_context")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.len() != 3)
        || methodology_report
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || methodology_report
            .get("complete_quality_access_methodology_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || methodology_report
            .get("denominator_to_dataset_field_crosswalk_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health quality/access methodology report custody failed".to_string());
    }
    for path_field in ["context_path", "reader_path", "metadata_path"] {
        if !root
            .join(string_field(methodology_report, path_field)?)
            .exists()
        {
            return Err(format!(
                "health quality/access methodology report file missing: {path_field}"
            ));
        }
    }
    for field in [
        "may_populate_thresholds",
        "may_populate_observed_values",
        "may_populate_pass_fail",
        "may_populate_solver_inputs",
    ] {
        if methodology_report
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "health quality/access methodology report must block {field}"
            ));
        }
    }

    let field_crosswalks = record
        .get("captured_dataset_field_crosswalk_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured dataset field crosswalk sources")?;
    if field_crosswalks.len() != 1 {
        return Err(
            "health quality/access captured dataset field crosswalk count failed".to_string(),
        );
    }
    let field_crosswalk = &field_crosswalks[0];
    if string_field(field_crosswalk, "context_path")?
        != CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH
        || string_field(field_crosswalk, "reader_path")?
            != CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_READER_PATH
        || int_field(field_crosswalk, "captured_dataset_count")? != 6
        || int_field(field_crosswalk, "captured_total_rows")? != 479_209
        || field_crosswalk
            .get("partial_denominator_or_measure_count_field_presence_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || field_crosswalk
            .get("complete_denominator_to_dataset_field_crosswalk_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health quality/access field crosswalk custody failed".to_string());
    }
    for path_field in ["context_path", "reader_path", "metadata_path"] {
        if !root
            .join(string_field(field_crosswalk, path_field)?)
            .exists()
        {
            return Err(format!(
                "health quality/access field crosswalk file missing: {path_field}"
            ));
        }
    }

    let rural_capacity_sources = record
        .get("captured_rural_safety_net_capacity_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access captured rural safety-net capacity sources")?;
    if rural_capacity_sources.len() != 1 {
        return Err(
            "health quality/access captured rural safety-net capacity count failed".to_string(),
        );
    }
    let rural_capacity = &rural_capacity_sources[0];
    if string_field(rural_capacity, "source_id")?
        != "SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24"
        || string_field(rural_capacity, "context_path")?
            != CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH
        || string_field(rural_capacity, "reader_path")?
            != CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_READER_PATH
        || string_field(rural_capacity, "metadata_path")?
            != "data/metadata/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24.2026-07-24.metadata.md"
        || int_field(rural_capacity, "cms_local_raw_file_count")? != 3
        || int_field(rural_capacity, "cms_local_raw_total_byte_count")? != 10_650_669
        || rural_capacity
            .get("cms_psf_capacity_fields_identified")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || rural_capacity
            .get("hrsa_browser_context_identified")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || rural_capacity
            .get("hrsa_local_raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || rural_capacity
            .get("facility_to_county_rural_join_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || rural_capacity
            .get("complete_rural_safety_net_capacity_series_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("health quality/access rural safety-net capacity custody failed".to_string());
    }
    for path_field in ["context_path", "reader_path", "metadata_path"] {
        if !root
            .join(string_field(rural_capacity, path_field)?)
            .exists()
        {
            return Err(format!(
                "health quality/access rural safety-net capacity file missing: {path_field}"
            ));
        }
    }
    for field in [
        "may_populate_thresholds",
        "may_populate_observed_values",
        "may_populate_pass_fail",
        "may_populate_solver_inputs",
    ] {
        if rural_capacity
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!(
                "health quality/access rural safety-net capacity must block {field}"
            ));
        }
    }

    let floors = record
        .get("floor_indicator_families_blocked")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access floor indicator families")?;
    let expected_floors = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_floors = floors
        .iter()
        .map(|item| string_field(item, "floor_class"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_floors != expected_floors || floors.len() != 4 {
        return Err("health quality/access floor set failed".to_string());
    }
    for floor in floors {
        if floor
            .get("candidate_indicators")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|indicators| indicators.is_empty())
            || floor
                .get("partial_context_custody_ready")
                .and_then(serde_json::Value::as_bool)
                .is_none()
            || floor
                .get("raw_custody_ready")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
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
            return Err("health quality/access floor values must remain blocked".to_string());
        }
    }

    let requirements = record
        .get("quality_access_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("health quality/access capture requirements")?;
    if requirements.len() != 12 {
        return Err("health quality/access capture requirement count failed".to_string());
    }
    for (index, req) in requirements.iter().enumerate() {
        let expected_ready = index < 9;
        if string_field(req, "requirement")?.is_empty()
            || req.get("ready").and_then(serde_json::Value::as_bool) != Some(expected_ready)
            || (expected_ready && req.get("value").is_some_and(serde_json::Value::is_null))
        {
            return Err("health quality/access capture requirement readiness failed".to_string());
        }
        if !expected_ready {
            let requirement = string_field(req, "requirement")?;
            let value = req
                .get("value")
                .ok_or("health quality/access requirement value")?;
            if requirement == "indicator_definition_and_denominator_lineage" {
                if !value.as_str().is_some_and(|text| {
                    text.contains("partial CMS Hospital Data Dictionary context captured")
                        && text.contains(
                            "dataset-specific denominator-to-field crosswalk remains incomplete",
                        )
                }) {
                    return Err(
                        "health quality/access partial lineage context requirement failed"
                            .to_string(),
                    );
                }
            } else if !value.is_null() {
                return Err("health quality/access blocked requirement value failed".to_string());
            }
        }
    }

    let blocked = record
        .get("blocked_health_floor_uses")
        .and_then(serde_json::Value::as_object)
        .ok_or("health quality/access blocked floor uses")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "health quality/access blocked use must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health quality/access claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("health quality/access claim bool")?;
        if matches!(
            field.as_str(),
            "health_quality_access_indicator_source_gap_published"
                | "quality_access_source_family_declared"
                | "quality_access_partial_source_custody_ready"
                | "quality_access_lineage_context_captured"
                | "qualitynet_methodology_surface_context_captured"
                | "selected_mortality_methodology_report_content_captured"
                | "partial_denominator_field_crosswalk_captured"
                | "partial_rural_safety_net_capacity_context_captured"
                | "quality_access_raw_artifact_captured"
                | "quality_access_metadata_captured"
        ) {
            if !observed {
                return Err(format!(
                    "health quality/access claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "health quality/access claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "CMS Provider Data Catalog hospital quality/access raw context custody is partially ready",
        "CMS Hospital Data Dictionary lineage context is locally captured",
        "CMS/QualityNet methodology surface HTML/JavaScript custody is locally captured",
        "selected CMS mortality methodology report content is locally captured",
        "partial CMS dataset denominator-field crosswalk context is locally captured",
        "partial CMS/HRSA rural safety-net capacity context is locally captured",
        "complete denominator-to-field crosswalk, complete all-measure case-mix lineage, rural capacity series, safety-net capacity series, threshold selection, observed values, and pass/fail lineage remain blocked",
        "not complete CMS quality/access source capture",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
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
            return Err(format!("health quality/access warning missing: {required}"));
        }
    }

    let reader =
        fs::read_to_string(root.join(HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH,
        "Source families needed but not custody-ready",
        "Hospital General Information",
        "Complications and Deaths - Hospital",
        "Healthcare Associated Infections - Hospital",
        "Unplanned Hospital Visits - Hospital",
        "Timely and Effective Care - Hospital",
        "Rural Emergency Hospital Timely and Effective Care - Hospital",
        "Hospital DDB Data Dictionary April 2026",
        "CMS Provider Data Catalog Overall Hospital Quality Star Rating topic",
        "QualityNet inpatient mortality methodology route",
        "QualityNet overall ratings resources route",
        "CMS Measure Methodology page",
        "Hybrid Hospital-Wide Risk-Standardized Mortality Methodology Report Version",
        "2022 Condition-Specific Mortality Measures Updates and Specifications Report",
        "Denominator, Sample, measure-count fields, and HAI measure-ID pattern context",
        "CMS TEAM safety-net and rural hospital fact sheet",
        "CMS Inpatient PSF October 2025 ZIP",
        "HRSA FORHP rural data files are browser-visible official context",
        "complete CMS quality/access denominator-to-dataset field crosswalk",
        "complete all-measure risk-adjusted outcome case-mix methodology crosswalk",
        "rural and safety-net capacity series beyond rural emergency hospital",
        "raw artifact path: ready for six CMS CSVs",
        "raw byte count: ready for six CMS CSVs",
        "raw SHA-256: ready for six CMS CSVs",
        "metadata path: ready for the custody packet",
        "retrieval date: 2026-07-24",
        "official methodology surface HTML/JavaScript custody",
        "selected mortality methodology report content",
        "partial denominator or measure-count field presence",
        "partial rural/safety-net capacity context",
        "indicator definition and denominator lineage: partial data dictionary context",
        "risk adjustment and case mix lineage",
        "rural and safety-net capacity lineage",
        "not complete CMS quality/access source capture",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
        "not target-cost selection",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("health quality/access reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_source_readiness_rollup(root: &Path) -> Result<(), String> {
    for path in [
        HEALTH_SOURCE_READINESS_ROLLUP_JSON_PATH,
        HEALTH_SOURCE_READINESS_ROLLUP_SCHEMA_PATH,
        HEALTH_SOURCE_READINESS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing health source readiness rollup artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(HEALTH_SOURCE_READINESS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "health-source-readiness-rollup:v1"
        || string_field(&record, "record_family")? != "health_source_readiness_rollup"
        || int_field(&record, "pulse")? != 183
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "health_floor_source_capture_status_path")?
            != HEALTH_FLOOR_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(
            &record,
            "health_medicare_trustees_source_capture_status_path",
        )? != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "health_nhe_source_custody_gap_path")?
            != HEALTH_NHE_SOURCE_CUSTODY_GAP_JSON_PATH
        || string_field(&record, "health_cbo_source_custody_gap_path")?
            != HEALTH_CBO_SOURCE_CUSTODY_GAP_JSON_PATH
        || string_field(&record, "health_quality_access_indicator_source_gap_path")?
            != HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH
    {
        return Err("health source readiness rollup identity failed".to_string());
    }

    let source_rows = record
        .get("source_family_rollup")
        .and_then(serde_json::Value::as_array)
        .ok_or("health source rollup rows")?;
    if source_rows.len() != 5 {
        return Err("health source rollup must have five rows".to_string());
    }

    let mut ready_context_only = 0usize;
    let mut partial_context_only = 0usize;
    let mut custody_gaps = 0usize;
    let expected_families = [
        "OMB FY2027 historical fiscal tables",
        "CMS Medicare Trustees 2026",
        "CMS national health expenditure accounts",
        "CBO federal health baseline",
        "CMS quality and access indicator series",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_families = source_rows
        .iter()
        .map(|row| string_field(row, "source_family"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_families != expected_families {
        return Err("health source rollup family set failed".to_string());
    }

    for row in source_rows {
        let family = string_field(row, "source_family")?;
        let custody_status = string_field(row, "custody_status")?;
        let raw_ready = row
            .get("raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            .ok_or("health source raw custody bool")?;
        if string_field(row, "source_role")?.is_empty() {
            return Err(format!("health source role missing: {family}"));
        }
        if custody_status == "custody_ready_context_only" {
            ready_context_only += 1;
            if !matches!(
                family.as_str(),
                "OMB FY2027 historical fiscal tables"
                    | "CMS Medicare Trustees 2026"
                    | "CMS national health expenditure accounts"
            ) || !raw_ready
                || row
                    .get("may_populate_current_law_context")
                    .and_then(serde_json::Value::as_bool)
                    != Some(true)
            {
                return Err(format!("unexpected context-ready health source: {family}"));
            }
        } else if custody_status == "custody_gap" {
            custody_gaps += 1;
            if matches!(
                family.as_str(),
                "OMB FY2027 historical fiscal tables"
                    | "CMS Medicare Trustees 2026"
                    | "CMS national health expenditure accounts"
                    | "CMS quality and access indicator series"
            ) || raw_ready
            {
                return Err(format!("unexpected health source custody gap: {family}"));
            }
        } else if custody_status == "partial_custody_context_only" {
            partial_context_only += 1;
            if family != "CMS quality and access indicator series"
                || raw_ready
                || row
                    .get("partial_raw_custody_ready")
                    .and_then(serde_json::Value::as_bool)
                    != Some(true)
            {
                return Err(format!("unexpected partial health source: {family}"));
            }
        } else {
            return Err(format!("unexpected health source custody status: {family}"));
        }
        for field in [
            "may_populate_floor_threshold_or_pass_fail",
            "may_populate_federal_policy_translation",
            "may_populate_solver_input",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!("health source row must block {field}: {family}"));
            }
        }
    }
    if ready_context_only != 3 || partial_context_only != 1 || custody_gaps != 1 {
        return Err("health source rollup ready/gap counts failed".to_string());
    }

    let counts = record
        .get("readiness_counts")
        .ok_or("health source readiness counts")?;
    let expected_counts = [
        ("source_family_count", 5),
        ("custody_ready_context_only_count", 3),
        ("partial_custody_context_only_count", 1),
        ("custody_gap_count", 1),
        ("floor_passage_ready_count", 0),
        ("federal_policy_translation_ready_count", 0),
        ("solver_input_ready_count", 0),
        ("public_rate_ready_count", 0),
    ];
    for (field, expected) in expected_counts {
        if int_field(counts, field)? != expected {
            return Err(format!("health source readiness count failed: {field}"));
        }
    }

    let readiness = record
        .get("health_lane_readiness")
        .ok_or("health lane readiness")?;
    for field in [
        "partial_fiscal_context_ready",
        "medicare_financing_and_enrollment_context_ready",
        "nhe_raw_custody_ready",
        "quality_access_partial_raw_custody_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("health lane readiness should be true: {field}"));
        }
    }
    for field in [
        "cbo_raw_custody_ready",
        "quality_access_raw_custody_ready",
        "threshold_values_selected",
        "observed_floor_values_populated",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "federal_policy_translation_ready",
        "behavior_or_incidence_model_ready",
        "solver_input_ready",
        "public_rate_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("health lane readiness must be false: {field}"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("health source blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "health source blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("health source readiness claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("health source readiness claim bool")?;
        if matches!(
            field.as_str(),
            "health_source_readiness_rollup_published"
                | "partial_fiscal_context_ready"
                | "medicare_financing_and_enrollment_context_ready"
                | "nhe_source_custody_ready"
                | "quality_access_partial_source_custody_ready"
        ) {
            if !observed {
                return Err(format!(
                    "health source readiness claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "health source readiness claim must be false: {field}"
            ));
        }
    }

    let gates = record
        .get("next_source_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("health next source gates")?;
    if gates.len() != 5 {
        return Err("health next source gate count failed".to_string());
    }
    for gate in gates {
        if string_field(gate, "gate")?.is_empty()
            || gate.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !gate.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("health next source gates must remain null/false".to_string());
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "partial context custody only, including CMS NHE and partial CMS Provider Data Catalog hospital quality/access context",
        "not complete health source capture",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
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
                "health source readiness warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(HEALTH_SOURCE_READINESS_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SOURCE_READINESS_ROLLUP_JSON_PATH,
        "Ready, context-only",
        "OMB FY2027 historical fiscal tables",
        "CMS Medicare Trustees 2026",
        "CMS national health expenditure accounts",
        "Partial, context-only",
        "CMS quality and access indicator series",
        "Still custody gaps",
        "CBO federal health baseline",
        "remaining CMS quality/access lineage needed for complete floor passage",
        "not complete health source capture",
        "not health floor threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not lower-cost scenario admissibility",
        "not a federal policy score",
        "not target-cost selection",
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
                "health source readiness reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_health_national_phi_sensitivity(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(HEALTH_NATIONAL_PHI_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health national PHI categories")?;
    if categories.len() != 2 {
        return Err("health national PHI sensitivity needs two categories".to_string());
    }
    let mut covered = 0.0;
    let mut scenario_totals = BTreeMap::<String, f64>::new();
    for category in categories {
        let base = number_field(category, "phi_payments_usd_billions")?;
        let reference = number_field(category, "reference_percent_medicare")?;
        covered += base;
        for scenario in category
            .get("scenarios")
            .and_then(|v| v.as_array())
            .ok_or("PHI scenarios")?
        {
            let target = number_field(scenario, "target_percent_medicare")?;
            let change = number_field(scenario, "mechanical_phi_payment_change_usd_billions")?;
            let expected = base * (target / reference - 1.0);
            if (change - expected).abs() > 0.001 {
                return Err(
                    "health national PHI category sensitivity does not reconcile".to_string(),
                );
            }
            *scenario_totals
                .entry(string_field(scenario, "scenario")?)
                .or_default() += change;
        }
    }
    if (covered - number_field(&card, "covered_phi_payments_usd_billions")?).abs() > 0.001 {
        return Err("health national PHI covered payments do not reconcile".to_string());
    }
    for row in card
        .get("combined_scenarios")
        .and_then(|v| v.as_array())
        .ok_or("combined PHI scenarios")?
    {
        let name = string_field(row, "scenario")?;
        if (number_field(row, "mechanical_phi_payment_change_usd_billions")?
            - scenario_totals[&name])
            .abs()
            > 0.001
        {
            return Err("health national PHI combined scenario does not reconcile".to_string());
        }
    }
    if !string_field(&card, "comparison_grade")?.starts_with("C_")
        || !string_field(&card, "net_savings_status")?.contains("blocked")
        || !string_field(&card, "federal_budget_effect_status")?.contains("blocked")
    {
        return Err(
            "health national PHI sensitivity must remain Grade C with savings blocked".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(HEALTH_NATIONAL_PHI_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_NATIONAL_PHI_JSON_PATH,
        "national payer sensitivity != gross savings != net savings != federal savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health national PHI reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_sample_sensitivity(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_SAMPLE_SENSITIVITY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health sample sensitivity categories")?;
    if categories.len() != 2 {
        return Err("health sample sensitivity needs two categories".to_string());
    }
    for category in categories {
        let commercial = number_field(category, "commercial_allowed_usd_billions")?;
        let medicare = number_field(category, "simulated_medicare_allowed_usd_billions")?;
        let scenarios = category
            .get("scenarios")
            .and_then(|v| v.as_array())
            .ok_or("health sample sensitivity scenarios")?;
        if scenarios.len() != 3 {
            return Err("health sample sensitivity needs three scenarios per category".to_string());
        }
        for scenario in scenarios {
            let target = number_field(scenario, "target_percent_medicare")? / 100.0;
            let change = number_field(scenario, "mechanical_sample_payment_change_usd_billions")?;
            let expected = medicare * target - commercial;
            if (change - expected).abs() > 0.001 {
                return Err("health sample dollar sensitivity does not reconcile".to_string());
            }
        }
    }
    let prohibited = card
        .get("prohibited_uses")
        .and_then(|v| v.as_array())
        .ok_or("health sample prohibited uses")?;
    if prohibited.len() < 5 || !string_field(&card, "net_savings_status")?.contains("blocked") {
        return Err("health sample sensitivity must block national and net claims".to_string());
    }
    let reader = fs::read_to_string(root.join(HEALTH_SAMPLE_SENSITIVITY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SAMPLE_SENSITIVITY_JSON_PATH,
        "sample sensitivity != national gross savings != net savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health sample sensitivity reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_scenarios(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(HEALTH_SCENARIOS_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health scenario categories")?;
    if categories.len() != 2 {
        return Err("health scenarios need hospital and professional categories".to_string());
    }
    for category in categories {
        let current = number_field(category, "current_reference_percent_medicare")?;
        let scenarios = category
            .get("scenarios")
            .and_then(|v| v.as_array())
            .ok_or("health category scenarios")?;
        if scenarios.len() != 3
            || !string_field(category, "dollar_effect_status")?.contains("blocked")
        {
            return Err(
                "health category must have three scenarios with dollars blocked".to_string(),
            );
        }
        for scenario in scenarios {
            let target = number_field(scenario, "target_percent_medicare")?;
            let change = number_field(scenario, "gross_rate_change_percent")?;
            let expected = (target / current - 1.0) * 100.0;
            if (change - expected).abs() > 0.001 {
                return Err("health scenario rate change does not reconcile".to_string());
            }
        }
    }
    let gates = card
        .get("shared_gates")
        .and_then(|v| v.as_array())
        .ok_or("health scenario shared gates")?;
    if gates.len() < 6
        || !string_field(&card, "gross_dollar_effect_status")?.contains("blocked")
        || !string_field(&card, "net_savings_status")?.contains("blocked")
    {
        return Err("health scenarios must preserve model gates and savings blocks".to_string());
    }
    let reader =
        fs::read_to_string(root.join(HEALTH_SCENARIOS_READER_PATH)).map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SCENARIOS_JSON_PATH,
        "illustrative rate path != spending reduction != federal savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health scenario reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_target_admissibility(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_TARGET_ADMISSIBILITY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let anchors = card
        .get("anchors")
        .and_then(|v| v.as_array())
        .ok_or("health target admissibility anchors")?;
    if anchors.len() != 2 {
        return Err(
            "health target admissibility needs hospital and professional anchors".to_string(),
        );
    }
    for anchor in anchors {
        if string_field(anchor, "anchor_status")? != "conditional_scenario_anchor"
            || !string_field(anchor, "target_status")?.contains("blocked")
        {
            return Err(
                "health target anchor must be conditional with universal target blocked"
                    .to_string(),
            );
        }
    }
    let hospital = anchors
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("hospital care"))
        .ok_or("hospital admissibility anchor")?;
    if number_field(hospital, "aggregate_ffs_medicare_margin_percent")? != -12.1
        || number_field(hospital, "efficient_hospital_median_ffs_margin_percent")? != -1.0
        || number_field(hospital, "efficient_hospital_projected_2026_margin_percent")? != 1.0
    {
        return Err(
            "hospital adequacy margins must preserve aggregate and efficient-provider distinction"
                .to_string(),
        );
    }
    let rules = card
        .get("admissibility_rules")
        .and_then(|v| v.as_array())
        .ok_or("health target admissibility rules")?;
    if rules.len() < 6 || !string_field(&card, "savings_status")?.contains("blocked") {
        return Err(
            "health target admissibility must preserve floors and savings block".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(HEALTH_TARGET_ADMISSIBILITY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_TARGET_ADMISSIBILITY_JSON_PATH,
        "credible anchor != universal target != gross reduction != net savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health target admissibility reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_category_benchmark_ladder(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_CATEGORY_BENCHMARK_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let rows = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health category benchmark rows")?;
    if rows.len() != 3 {
        return Err("health category benchmark ladder must contain three rows".to_string());
    }
    let hospital = rows
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("hospital care"))
        .ok_or("hospital benchmark row")?;
    if number_field(hospital, "current_value")? != 253.0
        || !string_field(hospital, "scoring_status")?.contains("blocked")
    {
        return Err("hospital reference must be 253 percent with target blocked".to_string());
    }
    let professional = rows
        .iter()
        .find(|v| {
            string_field(v, "category").ok().as_deref() == Some("physician and clinical services")
        })
        .ok_or("professional benchmark row")?;
    if number_field(professional, "current_value")? != 139.0
        || number_field(professional, "state_average_low")? != 117.0
        || number_field(professional, "state_average_high")? != 243.0
        || string_field(professional, "comparison_grade")? != "B"
        || !string_field(professional, "scoring_status")?.contains("blocked")
    {
        return Err(
            "professional reference must preserve estimate, range, grade, and target block"
                .to_string(),
        );
    }
    let drugs = rows
        .iter()
        .find(|v| string_field(v, "category").ok().as_deref() == Some("retail prescription drugs"))
        .ok_or("drug benchmark row")?;
    let ratio = number_field(drugs, "current_value")? / number_field(drugs, "benchmark_value")?;
    if (ratio - number_field(drugs, "current_to_benchmark_ratio")?).abs() > 0.001
        || !string_field(drugs, "scoring_status")?.contains("blocked")
    {
        return Err(
            "drug spending comparison must reconcile with price target blocked".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(HEALTH_CATEGORY_BENCHMARK_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_CATEGORY_BENCHMARK_JSON_PATH,
        "reference price != expected price != addressable excess != savings",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "health category benchmark reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_service_bridge(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_SERVICE_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let categories = card
        .get("categories")
        .and_then(|v| v.as_array())
        .ok_or("health service bridge categories")?;
    if categories.len() != 3 {
        return Err("health service bridge must contain three categories".to_string());
    }
    let mut spending = 0.0;
    let mut share = 0.0;
    for category in categories {
        spending += number_field(category, "spending_usd_billions")?;
        share += number_field(category, "share_total_nhe_percent")?;
        let total = number_field(category, "expenditure_growth_percent")? / 100.0;
        let price = number_field(category, "price_growth_percent")? / 100.0;
        let residual = number_field(category, "implied_non_price_growth_percent")?;
        let expected = ((1.0 + total) / (1.0 + price) - 1.0) * 100.0;
        if (expected - residual).abs() > 0.001 {
            return Err("health service bridge residual does not reconcile".to_string());
        }
        if !string_field(category, "peer_price_benchmark_status")?.contains("blocked") {
            return Err("health service category peer benchmark must remain blocked".to_string());
        }
    }
    if (spending - number_field(&card, "covered_spending_usd_billions")?).abs() > 0.001
        || (share - number_field(&card, "covered_share_total_nhe_percent")?).abs() > 0.001
    {
        return Err("health service bridge totals do not reconcile".to_string());
    }
    let reader = fs::read_to_string(root.join(HEALTH_SERVICE_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_SERVICE_BRIDGE_JSON_PATH,
        "growth decomposition != peer efficiency finding != fraud != savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health service bridge reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_health_cost_decomposition(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(HEALTH_COST_DECOMPOSITION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let headline = card
        .get("headline")
        .ok_or("health decomposition headline")?;
    let us = number_field(headline, "us_total_health_spending_percent_gdp")?;
    let peer = number_field(headline, "oecd_average_percent_gdp")?;
    let gap = number_field(headline, "observed_gap_percentage_points")?;
    if us != 17.2 || peer != 9.3 || (us - peer - gap).abs() > 0.0001 {
        return Err("health decomposition headline does not reconcile".to_string());
    }
    let signals = card
        .get("diagnostic_signals")
        .and_then(|v| v.as_array())
        .ok_or("health decomposition diagnostic signals")?;
    if signals.len() != 5 {
        return Err("health decomposition must contain five diagnostic signals".to_string());
    }
    for field in ["decomposition_status", "fraud_status", "savings_status"] {
        let value = string_field(&card, field)?;
        if !value.contains("not_") && !value.contains("blocked") && !value.contains("diagnostic") {
            return Err(format!(
                "health decomposition must preserve {field} boundary"
            ));
        }
    }
    let reader = fs::read_to_string(root.join(HEALTH_COST_DECOMPOSITION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HEALTH_COST_DECOMPOSITION_JSON_PATH,
        "observed spending gap != inefficiency != fraud != recoverable savings",
    ] {
        if !reader.contains(required) {
            return Err(format!("health decomposition reader missing {required}"));
        }
    }
    Ok(())
}

