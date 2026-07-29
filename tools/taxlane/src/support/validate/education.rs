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

pub(crate) fn validate_education_workforce_outcome_floor_definition_packet(root: &Path) -> Result<(), String> {
    for path in [
        EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing education/workforce outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "education-workforce-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")?
            != "education_workforce_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 170
        || string_field(&record, "lane_id")? != "education-workforce"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(
            &record,
            "transportation_infrastructure_outcome_floor_definition_packet_path",
        )? != TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "education_depth_card_path")? != EDUCATION_DEPTH_CARD_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("education/workforce floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("education/workforce floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "education/workforce floor status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "federal_state_local_translation_ready",
        "program_to_outlay_allocation_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "target_cost_ready",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "education/workforce floor status {field} must be false"
            ));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("education/workforce floor definition policy")?;
    for field in [
        "federal_state_local_translation_required",
        "attainment_completion_access_employment_and_equity_floors_required",
        "negative_higher_education_entry_is_accounting_not_savings",
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "international_differences_not_savings",
        "no_fraud_inference",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "education/workforce floor policy {field} must be true"
            ));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("education/workforce required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("education/workforce required floor class count failed".to_string());
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
        return Err("education/workforce required floor class set failed".to_string());
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
                    "education/workforce floor class {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("education/workforce floor class must remain unpassed".to_string());
        }
    }

    let lane_floors = record
        .get("education_workforce_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("education/workforce-specific floor definitions")?;
    let expected_lane_floors = [
        "attainment",
        "completion_persistence",
        "access_affordability",
        "employment_earnings_transition",
        "equity_distribution",
        "federal_state_local_translation_delivery_feasibility",
    ];
    if lane_floors.len() != expected_lane_floors.len() {
        return Err("education/workforce-specific floor count failed".to_string());
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
        return Err("education/workforce-specific floor set failed".to_string());
    }
    for row in lane_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "education/workforce-specific floors must remain null and unpassed".to_string(),
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
        .ok_or("education/workforce floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("education_workforce_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(6)
    {
        return Err("education/workforce floor summary counts failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "all_floors_passed",
        "target_cost_ready",
        "solver_input_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "education/workforce floor summary {field} must be false"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("education/workforce floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("education/workforce floor packet publication flag failed".to_string());
    }
    for field in [
        "federal_state_local_translation_ready",
        "program_to_outlay_allocation_ready",
        "cohort_timing_model_ready",
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
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
                "education/workforce floor claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This education/workforce floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "The negative higher-education FY2025 entry is an account-reconciliation fact, not negative education, recovered waste, or savings.",
        "No lower-cost education/workforce scenario is admissible until attainment, completion, access, employment, equity, adequacy/resilience, federal/state/local translation, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "No target cost, federal effect, gross savings, net savings, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a federal/state/local translation",
        "not a program-to-outlay allocation",
        "not a cohort timing model",
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
                "education/workforce floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_education_workforce_graduation_floor_value_packet(root: &Path) -> Result<(), String> {
    for path in [
        EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_JSON_PATH,
        EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_SCHEMA_PATH,
        EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing education/workforce graduation floor value packet artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "education-workforce-graduation-floor-value-packet:v1"
        || string_field(&record, "record_family")?
            != "education_workforce_graduation_floor_value_packet"
        || int_field(&record, "pulse")? != 212
        || string_field(&record, "lane_id")? != "education-workforce"
        || string_field(&record, "floor_id")? != "completion_persistence"
        || string_field(&record, "floor_definition_packet_path")?
            != EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "k12_outcome_baseline_path")? != K12_OUTCOME_BASELINE_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
    {
        return Err(
            "education/workforce graduation floor value packet identity failed".to_string(),
        );
    }

    let threshold = record
        .get("threshold_rationale")
        .ok_or("education/workforce graduation threshold rationale")?;
    if string_field(threshold, "rationale_id")? != "no-regression-from-public-high-school-acgr"
        || string_field(threshold, "selected_measure")?
            != "Four-year adjusted cohort graduation rate for public high school students"
        || string_field(threshold, "threshold_type")? != "baseline_no_regression_floor"
        || (number_field(threshold, "threshold_value")? - 87.0).abs() > 0.000001
        || string_field(threshold, "threshold_unit")? != "percent"
        || !string_field(threshold, "source_table")?.contains("NCES ACGR 2021-22")
        || !string_field(threshold, "review_status")?.contains("needs_role_review_before_pass_fail")
    {
        return Err("education/workforce graduation threshold rationale failed".to_string());
    }

    let baseline = record
        .get("baseline_values")
        .ok_or("education/workforce graduation baseline values")?;
    let primary = baseline
        .get("primary_baseline")
        .ok_or("education/workforce graduation primary baseline")?;
    if string_field(baseline, "reference_school_year")? != "2021-22"
        || string_field(primary, "measure")?
            != "public high-school four-year adjusted cohort graduation rate"
        || (number_field(primary, "value")? - 87.0).abs() > 0.000001
        || string_field(primary, "unit")? != "percent"
        || string_field(primary, "source_path")? != K12_OUTCOME_BASELINE_JSON_PATH
    {
        return Err("education/workforce graduation primary baseline failed".to_string());
    }
    let equity = baseline
        .get("equity_context")
        .and_then(serde_json::Value::as_array)
        .ok_or("education/workforce graduation equity context")?;
    if equity.len() != 9 || !string_field(baseline, "boundary")?.contains("not pass/fail evidence")
    {
        return Err("education/workforce graduation baseline context failed".to_string());
    }

    for field in ["policy_values", "stress_values", "pass_fail_evidence"] {
        if !record.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "education/workforce graduation floor value packet field must stay null: {field}"
            ));
        }
    }

    let readiness = record
        .get("readiness_status")
        .ok_or("education/workforce graduation readiness")?;
    for field in [
        "threshold_rationale_ready",
        "threshold_value_populated",
        "baseline_value_ready",
    ] {
        if !bool_field(readiness, field)? {
            return Err(format!(
                "education/workforce graduation readiness should be true: {field}"
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
                "education/workforce graduation readiness must remain false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("education/workforce graduation blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "education/workforce graduation blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("education/workforce graduation claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("education/workforce graduation claim bool")?;
        match field.as_str() {
            "graduation_floor_value_packet_published"
            | "threshold_rationale_ready"
            | "threshold_value_populated"
            | "baseline_value_ready" => {
                if !observed {
                    return Err(format!(
                        "education/workforce graduation claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "education/workforce graduation downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "draft no-regression education/workforce graduation floor threshold",
        "not federal/state/local translation",
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
                "education/workforce graduation warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for required in [
        EDUCATION_WORKFORCE_GRADUATION_FLOOR_VALUE_PACKET_JSON_PATH,
        K12_OUTCOME_BASELINE_JSON_PATH,
        "87.0 percent",
        "74.0 percent",
        "68.0 percent",
        "draft no-regression education/workforce graduation floor",
        "Policy and stress values remain null",
        "not federal/state/local translation",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "education/workforce graduation reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_education_depth_card(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(EDUCATION_DEPTH_CARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let total = number_field(&card, "total_outlays_millions")?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("education components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let higher = parts
        .iter()
        .find(|v| v.get("subfunction_code").and_then(|x| x.as_str()) == Some("502"))
        .ok_or("higher education component")?;
    if parts.len() != 6
        || total != 72_042.0
        || (sum - total).abs() > 0.001
        || number_field(higher, "outlays_millions")? != -35_005.0
        || higher.get("accounting_caveat").is_none()
    {
        return Err("education depth reconciliation failed".to_string());
    }
    let reader = fs::read_to_string(root.join(EDUCATION_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(EDUCATION_DEPTH_CARD_JSON_PATH)
        || !reader.contains("does not mean government")
        || !reader.contains("negative education")
    {
        return Err("education depth caveat missing".to_string());
    }

    let bridge_text = fs::read_to_string(root.join(HIGHER_EDUCATION_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let bridge: serde_json::Value =
        serde_json::from_str(&bridge_text).map_err(|e| e.to_string())?;
    let account_rows = bridge
        .get("account_rows")
        .and_then(|v| v.as_array())
        .ok_or("higher-education bridge account rows")?;
    let mut account_sum = 0.0;
    let mut positive_sum = 0.0;
    let mut negative_sum = 0.0;
    for row in account_rows {
        let amount = row
            .get(7)
            .and_then(|v| v.as_f64())
            .ok_or("higher-education account row amount")?;
        account_sum += amount;
        if amount > 0.0 {
            positive_sum += amount;
        } else {
            negative_sum += amount;
        }
    }
    let groups = bridge
        .get("explanatory_groups")
        .and_then(|v| v.as_array())
        .ok_or("higher-education bridge groups")?;
    let group_sum: f64 = groups
        .iter()
        .map(|v| number_field(v, "amount"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if account_rows.len() != 38
        || account_sum != -35_005.0
        || positive_sum != 118_203.0
        || negative_sum != -153_208.0
        || group_sum != -35_005.0
        || number_field(&bridge, "historical_table_total")? != -35_005.0
        || number_field(&bridge, "public_budget_database_total")? != -35_005.0
        || number_field(&bridge, "reconciliation_difference")? != 0.0
        || string_field(&bridge, "bridge_status")? != "exact_account_reconciliation_complete"
        || string_field(&bridge, "cash_collection_equivalence")? != "not_established"
        || string_field(&bridge, "savings_status")? != "blocked_not_scored"
    {
        return Err("higher-education account bridge failed".to_string());
    }
    let bridge_reader = fs::read_to_string(root.join(HIGHER_EDUCATION_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        HIGHER_EDUCATION_BRIDGE_JSON_PATH,
        "zero reconciliation",
        "cash-collection equivalence is not",
    ] {
        if !bridge_reader.contains(required) {
            return Err(format!("higher-education bridge reader missing {required}"));
        }
    }
    let federalism_text = fs::read_to_string(root.join(K12_FEDERALISM_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let federalism: serde_json::Value =
        serde_json::from_str(&federalism_text).map_err(|e| e.to_string())?;
    let census = federalism
        .get("census_school_system_finance")
        .ok_or("K-12 Census finance frame")?;
    let revenue_sum = number_field(census, "federal_revenue")?
        + number_field(census, "state_revenue")?
        + number_field(census, "local_revenue")?;
    let expenditure_sum = number_field(census, "current_spending")?
        + number_field(census, "capital_outlay")?
        + number_field(census, "other_expenditure")?;
    let share_sum = number_field(census, "federal_share_percent")?
        + number_field(census, "state_share_percent")?
        + number_field(census, "local_share_percent")?;
    let crosswalk = federalism
        .get("omb_to_census_crosswalk")
        .ok_or("K-12 OMB-to-Census crosswalk")?;
    if (revenue_sum - number_field(census, "total_revenue")?).abs() > 0.001
        || (expenditure_sum - number_field(census, "total_expenditure")?).abs() > 0.001
        || (share_sum - 100.0).abs() > 0.000001
        || number_field(census, "published_current_spending_per_pupil_usd")? < 17_619.0
        || string_field(crosswalk, "comparison_status")? != "not_reconcilable_not_additive"
        || !string_field(crosswalk, "double_count_rule")?.contains("Do not add")
        || string_field(&federalism, "savings_status")? != "blocked_not_scored"
    {
        return Err("K-12 federalism finance bridge failed".to_string());
    }
    let federalism_reader = fs::read_to_string(root.join(K12_FEDERALISM_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        K12_FEDERALISM_BRIDGE_JSON_PATH,
        "double count federal support",
        "does not supply an expected spending level",
    ] {
        if !federalism_reader.contains(required) {
            return Err(format!("K-12 federalism reader missing {required}"));
        }
    }

    if string_field(&federalism, "outcome_baseline_path")? != K12_OUTCOME_BASELINE_JSON_PATH {
        return Err("K-12 federalism outcome link failed".to_string());
    }
    let outcome_text =
        fs::read_to_string(root.join(K12_OUTCOME_BASELINE_JSON_PATH)).map_err(|e| e.to_string())?;
    let outcome: serde_json::Value =
        serde_json::from_str(&outcome_text).map_err(|e| e.to_string())?;
    let achievement = outcome
        .get("achievement")
        .ok_or("K-12 achievement baseline")?;
    let achievement_rows = achievement
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or("K-12 achievement rows")?;
    let expected = [
        ("mathematics", 4.0, 237.0, 24.0, 36.0, 31.0, 9.0, 39.0),
        ("mathematics", 8.0, 272.0, 41.0, 32.0, 19.0, 8.0, 27.0),
        ("reading", 4.0, 214.0, 41.0, 29.0, 22.0, 8.0, 30.0),
        ("reading", 8.0, 257.0, 34.0, 37.0, 25.0, 4.0, 29.0),
    ];
    if achievement_rows.len() != expected.len() {
        return Err("K-12 achievement row count failed".to_string());
    }
    for (row, expected_row) in achievement_rows.iter().zip(expected) {
        let displayed_sum = number_field(row, "below_basic_percent")?
            + number_field(row, "basic_percent")?
            + number_field(row, "proficient_percent")?
            + number_field(row, "advanced_percent")?;
        if string_field(row, "subject")? != expected_row.0
            || number_field(row, "grade")? != expected_row.1
            || number_field(row, "average_score")? != expected_row.2
            || number_field(row, "below_basic_percent")? != expected_row.3
            || number_field(row, "basic_percent")? != expected_row.4
            || number_field(row, "proficient_percent")? != expected_row.5
            || number_field(row, "advanced_percent")? != expected_row.6
            || number_field(row, "at_or_above_proficient_percent")? != expected_row.7
            || displayed_sum != 100.0
            || number_field(row, "displayed_distribution_sum_percent")? != 100.0
        {
            return Err("K-12 achievement distribution failed".to_string());
        }
    }
    let completion = outcome
        .get("completion")
        .ok_or("K-12 completion baseline")?;
    let race = completion
        .get("race_ethnicity_percent")
        .ok_or("K-12 ACGR race distribution")?;
    let characteristics = completion
        .get("selected_characteristics_percent")
        .ok_or("K-12 ACGR selected characteristics")?;
    if string_field(completion, "school_year")? != "2021-22"
        || number_field(completion, "us_average_percent")? != 87.0
        || number_field(race, "american_indian_alaska_native")? != 74.0
        || number_field(race, "asian_pacific_islander")? != 94.0
        || number_field(race, "black")? != 81.0
        || number_field(race, "hispanic")? != 83.0
        || number_field(race, "white")? != 90.0
        || number_field(characteristics, "students_with_disabilities")? != 71.0
        || number_field(characteristics, "english_learners")? != 72.0
        || number_field(characteristics, "economically_disadvantaged")? != 81.0
        || number_field(characteristics, "enrolled_homeless")? != 68.0
        || string_field(&outcome, "efficiency_status")?
            != "descriptive_baseline_not_causal_efficiency"
        || string_field(&outcome, "benchmark_status")?
            != "definition_matched_pisa_outcome_comparison_attached_not_spending_target"
        || string_field(&outcome, "savings_status")? != "blocked_not_scored"
    {
        return Err("K-12 completion or evidence boundary failed".to_string());
    }
    let attendance = outcome
        .get("attendance_access")
        .ok_or("K-12 attendance/access baseline")?;
    let attendance_race = attendance
        .get("race_ethnicity_percent")
        .ok_or("K-12 attendance race distribution")?;
    let attendance_characteristics = attendance
        .get("selected_characteristics_percent")
        .ok_or("K-12 attendance selected characteristics")?;
    if string_field(attendance, "school_year")? != "2022-23"
        || number_field(attendance, "reported_membership")? != 48_268_200.0
        || number_field(attendance, "reported_chronically_absent")? != 13_406_900.0
        || number_field(attendance, "reported_chronically_absent_percent")? != 27.8
        || number_field(attendance_race, "american_indian")? != 46.0
        || number_field(attendance_race, "asian")? != 15.0
        || number_field(attendance_race, "black")? != 37.0
        || number_field(attendance_race, "hispanic")? != 33.0
        || number_field(attendance_race, "pacific_islander")? != 45.0
        || number_field(attendance_race, "two_or_more_races")? != 30.0
        || number_field(attendance_race, "white")? != 22.0
        || number_field(attendance_characteristics, "children_with_disabilities")? != 36.0
        || number_field(attendance_characteristics, "children_without_disabilities")? != 26.0
        || number_field(attendance_characteristics, "english_learners")? != 33.0
        || number_field(attendance_characteristics, "non_english_learners")? != 27.0
        || !string_field(attendance, "aggregation_caveat")?.contains("duplicate")
        || !string_field(attendance, "comparison_caveat")?.contains("vary")
        || string_field(&outcome, "access_status")?
            != "national_chronic_absenteeism_and_cps_age_enrollment_transition_context_attached_other_access_measures_open"
    {
        return Err("K-12 attendance/access boundary failed".to_string());
    }
    let outcome_reader = fs::read_to_string(root.join(K12_OUTCOME_BASELINE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        K12_OUTCOME_BASELINE_JSON_PATH,
        "not grade-level proficiency",
        "neither the same cohort",
        "Do not divide spending",
        "count a mobile student more than once",
    ] {
        if !outcome_reader.contains(required) {
            return Err(format!("K-12 outcome reader missing {required}"));
        }
    }

    if string_field(&outcome, "peer_comparison_path")? != K12_PISA_PEER_JSON_PATH {
        return Err("K-12 outcome peer link failed".to_string());
    }
    let pisa_text =
        fs::read_to_string(root.join(K12_PISA_PEER_JSON_PATH)).map_err(|e| e.to_string())?;
    let pisa: serde_json::Value = serde_json::from_str(&pisa_text).map_err(|e| e.to_string())?;
    let score_rows = pisa
        .get("score_rows")
        .and_then(|v| v.as_array())
        .ok_or("K-12 PISA score rows")?;
    let expected_scores = [
        ("mathematics", 465.0, 472.0, -7.0, "close_to_oecd_average"),
        ("reading", 504.0, 476.0, 28.0, "higher_than_oecd_average"),
        ("science", 499.0, 485.0, 14.0, "higher_than_oecd_average"),
    ];
    if score_rows.len() != expected_scores.len() {
        return Err("K-12 PISA score row count failed".to_string());
    }
    for (row, expected_row) in score_rows.iter().zip(expected_scores) {
        if string_field(row, "subject")? != expected_row.0
            || number_field(row, "united_states")? != expected_row.1
            || number_field(row, "oecd_average")? != expected_row.2
            || number_field(row, "difference_points")? != expected_row.3
            || string_field(row, "reported_comparison")? != expected_row.4
        {
            return Err("K-12 PISA score comparison failed".to_string());
        }
    }
    let proficiency_rows = pisa
        .get("proficiency_rows")
        .and_then(|v| v.as_array())
        .ok_or("K-12 PISA proficiency rows")?;
    let expected_proficiency = [
        ("mathematics", "at_least_level_2", 66.0, 69.0, -3.0),
        ("mathematics", "level_5_or_6", 7.0, 9.0, -2.0),
        ("reading", "at_least_level_2", 80.0, 74.0, 6.0),
        ("reading", "level_5_or_6", 14.0, 7.0, 7.0),
        ("science", "at_least_level_2", 78.0, 76.0, 2.0),
        ("science", "level_5_or_6", 11.0, 7.0, 4.0),
    ];
    if proficiency_rows.len() != expected_proficiency.len() {
        return Err("K-12 PISA proficiency row count failed".to_string());
    }
    for (row, expected_row) in proficiency_rows.iter().zip(expected_proficiency) {
        if string_field(row, "subject")? != expected_row.0
            || string_field(row, "level")? != expected_row.1
            || number_field(row, "united_states_percent")? != expected_row.2
            || number_field(row, "oecd_average_percent")? != expected_row.3
            || number_field(row, "difference_percentage_points")? != expected_row.4
        {
            return Err("K-12 PISA proficiency comparison failed".to_string());
        }
    }
    let sampling = pisa
        .get("sampling_caveat")
        .ok_or("K-12 PISA sampling caveat")?;
    if number_field(
        sampling,
        "us_school_participation_before_replacement_percent",
    )? != 51.0
        || number_field(
            sampling,
            "us_school_participation_after_replacement_percent",
        )? != 63.0
        || number_field(sampling, "us_student_exclusion_percent")? != 6.0
        || number_field(sampling, "us_student_response_percent")? != 80.0
        || string_field(&pisa, "benchmark_status")?
            != "definition_matched_outcome_peer_comparison_not_spending_target"
        || string_field(&pisa, "finance_match_status")?
            != "not_matched_to_fy2024_school_system_spending"
        || !string_field(&pisa, "cross_assessment_rule")?.contains("Do not equate")
        || string_field(&pisa, "savings_status")? != "blocked_not_scored"
    {
        return Err("K-12 PISA sampling or evidence boundary failed".to_string());
    }
    let pisa_reader =
        fs::read_to_string(root.join(K12_PISA_PEER_READER_PATH)).map_err(|e| e.to_string())?;
    for required in [
        K12_PISA_PEER_JSON_PATH,
        "PISA Level 2 is not NAEP Proficient",
        "sampling standards were not reached",
        "not match education-system spending",
    ] {
        if !pisa_reader.contains(required) {
            return Err(format!("K-12 PISA reader missing {required}"));
        }
    }

    if string_field(&pisa, "resource_peer_comparison_path")? != K12_OECD_RESOURCE_JSON_PATH {
        return Err("K-12 PISA resource link failed".to_string());
    }
    let resource_text =
        fs::read_to_string(root.join(K12_OECD_RESOURCE_JSON_PATH)).map_err(|e| e.to_string())?;
    let resource: serde_json::Value =
        serde_json::from_str(&resource_text).map_err(|e| e.to_string())?;
    let resource_rows = resource
        .get("matched_resource_rows")
        .and_then(|v| v.as_array())
        .ok_or("K-12 OECD matched resource rows")?;
    if resource_rows.len() != 2 {
        return Err("K-12 OECD resource row count failed".to_string());
    }
    let per_student = &resource_rows[0];
    let public_share = &resource_rows[1];
    let broader = resource
        .get("broader_context_not_k12_only")
        .and_then(|v| v.as_array())
        .ok_or("K-12 OECD broader resource context")?;
    if number_field(&resource, "data_year")? != 2022.0
        || string_field(per_student, "unit")? != "equivalent_usd_converted_using_ppp"
        || number_field(per_student, "united_states")? != 14_603.0
        || number_field(per_student, "oecd_average")? != 12_438.0
        || number_field(per_student, "difference")? != 2_165.0
        || number_field(public_share, "united_states")? != 92.4
        || number_field(public_share, "oecd_average")? != 90.4
        || number_field(public_share, "difference_percentage_points")? != 2.0
        || broader.len() != 2
        || number_field(&broader[0], "united_states")? != 20_387.0
        || number_field(&broader[0], "oecd_average")? != 15_022.0
        || number_field(&broader[1], "united_states")? != 5.8
        || number_field(&broader[1], "oecd_average")? != 4.7
        || string_field(&resource, "comparison_status")?
            != "definition_matched_resource_peer_not_causal_efficiency"
        || string_field(&resource, "target_status")?
            != "oecd_average_descriptive_not_automatic_target"
        || string_field(&resource, "omb_crosswalk_status")?
            != "not_comparable_to_federal_function_500_share"
        || string_field(&resource, "savings_status")? != "blocked_not_scored"
    {
        return Err("K-12 OECD resource comparison failed".to_string());
    }
    let resource_reader =
        fs::read_to_string(root.join(K12_OECD_RESOURCE_READER_PATH)).map_err(|e| e.to_string())?;
    for required in [
        K12_OECD_RESOURCE_JSON_PATH,
        "$14,603",
        "$12,438",
        "descriptive, not an automatic target",
        "not assigned to their schools",
    ] {
        if !resource_reader.contains(required) {
            return Err(format!("K-12 OECD resource reader missing {required}"));
        }
    }

    if string_field(&card, "worker_outcome_baseline_path")? != WIOA_OUTCOME_BASELINE_JSON_PATH
        || string_field(&card, "worker_outcome_baseline_status")?
            != "wioa_title_i_and_iii_descriptive_performance_attached"
    {
        return Err("education worker outcome link failed".to_string());
    }
    let wioa_text = fs::read_to_string(root.join(WIOA_OUTCOME_BASELINE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let wioa: serde_json::Value = serde_json::from_str(&wioa_text).map_err(|e| e.to_string())?;
    let reporting = wioa
        .get("reporting_period")
        .ok_or("WIOA reporting period")?;
    let periods = wioa
        .get("indicator_periods")
        .ok_or("WIOA indicator periods")?;
    let q2_period = periods
        .get("employment_second_quarter_and_median_earnings_exit_cohort")
        .ok_or("WIOA Q2 outcome period")?;
    let q4_period = periods
        .get("employment_fourth_quarter_and_credential_exit_cohort")
        .ok_or("WIOA Q4 outcome period")?;
    let rows = wioa
        .get("program_rows")
        .and_then(|v| v.as_array())
        .ok_or("WIOA program rows")?;
    let expected = [
        (
            "adult",
            250_160.0,
            Some(109_506.0),
            140_065.0,
            193_904.0,
            72.2,
            142_563.0,
            197_284.0,
            72.3,
            8_754.0,
            Some((50_840.0, 69_037.0, 73.6)),
            Some((70_005.0, 94_546.0, 74.0)),
        ),
        (
            "dislocated_worker",
            187_108.0,
            Some(37_119.0),
            97_770.0,
            141_790.0,
            69.0,
            107_944.0,
            153_147.0,
            70.5,
            9_897.0,
            Some((16_007.0, 21_318.0, 75.1)),
            Some((21_657.0, 29_910.0, 72.4)),
        ),
        (
            "youth",
            121_531.0,
            Some(34_956.0),
            49_419.0,
            74_950.0,
            65.9,
            50_555.0,
            74_963.0,
            67.4,
            5_038.0,
            Some((26_553.0, 42_838.0, 62.0)),
            Some((43_534.0, 64_561.0, 67.4)),
        ),
        (
            "wagner_peyser_employment_service",
            2_561_317.0,
            None,
            1_473_064.0,
            2_205_552.0,
            66.8,
            1_468_954.0,
            2_177_640.0,
            67.5,
            8_558.0,
            None,
            None,
        ),
    ];
    if rows.len() != expected.len()
        || string_field(reporting, "start")? != "2024-07-01"
        || string_field(reporting, "end")? != "2025-06-30"
        || string_field(q2_period, "start")? != "2023-07-01"
        || string_field(q2_period, "end")? != "2024-06-30"
        || string_field(q4_period, "start")? != "2023-01-01"
        || string_field(q4_period, "end")? != "2023-12-31"
    {
        return Err("WIOA row count or period alignment failed".to_string());
    }
    for (row, expected_row) in rows.iter().zip(expected) {
        let q2 = row
            .get("employment_second_quarter_after_exit")
            .ok_or("WIOA Q2 outcome")?;
        let q4 = row
            .get("employment_fourth_quarter_after_exit")
            .ok_or("WIOA Q4 outcome")?;
        let training = row
            .get("participants_received_training")
            .and_then(|v| v.as_f64());
        if string_field(row, "program")? != expected_row.0
            || number_field(row, "participants_served")? != expected_row.1
            || training != expected_row.2
            || number_field(q2, "numerator")? != expected_row.3
            || number_field(q2, "denominator")? != expected_row.4
            || number_field(q2, "percent")? != expected_row.5
            || number_field(q4, "numerator")? != expected_row.6
            || number_field(q4, "denominator")? != expected_row.7
            || number_field(q4, "percent")? != expected_row.8
            || number_field(row, "median_earnings_second_quarter_after_exit_usd")? != expected_row.9
        {
            return Err("WIOA published program outcome failed".to_string());
        }
        for (field, expected_metric) in [
            ("credential_attainment", expected_row.10),
            ("measurable_skill_gains", expected_row.11),
        ] {
            match expected_metric {
                Some((numerator, denominator, percent)) => {
                    let metric = row.get(field).ok_or("WIOA applicable metric")?;
                    if number_field(metric, "numerator")? != numerator
                        || number_field(metric, "denominator")? != denominator
                        || number_field(metric, "percent")? != percent
                    {
                        return Err("WIOA credential or skill-gain metric failed".to_string());
                    }
                }
                None if !row.get(field).is_some_and(|v| v.is_null()) => {
                    return Err("WIOA nonapplicable metric failed".to_string());
                }
                None => {}
            }
        }
    }
    let discrepancy = wioa
        .get("source_reconciliation")
        .ok_or("WIOA source discrepancy")?;
    let link = wioa
        .get("subfunction_link")
        .ok_or("WIOA subfunction link")?;
    let employer = wioa
        .get("effectiveness_in_serving_employers")
        .ok_or("WIOA employer effectiveness")?;
    if number_field(discrepancy, "pdf_participants_served_youth")? != 121_531.0
        || number_field(discrepancy, "results_webpage_participants_served_youth")? != 125_531.0
        || number_field(link, "outlays_millions")? != 5_434.0
        || string_field(link, "scope_status")?
            != "program_budget_activity_crosswalk_complete_actual_outlay_cohort_allocation_blocked"
        || string_field(link, "cost_per_outcome_status")? != "blocked_not_calculated"
        || number_field(employer, "numerator")? != 1_275_198.0
        || number_field(employer, "denominator")? != 1_971_425.0
        || number_field(employer, "percent")? != 64.7
        || string_field(&wioa, "comparison_status")?
            != "descriptive_participant_outcomes_not_causal"
        || string_field(&wioa, "fraud_status")? != "not_measured_not_inferred"
        || string_field(&wioa, "savings_status")? != "blocked_not_scored"
    {
        return Err("WIOA evidence boundary or source reconciliation failed".to_string());
    }
    let wioa_reader = fs::read_to_string(root.join(WIOA_OUTCOME_BASELINE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        WIOA_OUTCOME_BASELINE_JSON_PATH,
        "121,531",
        "125,531",
        "different time windows",
        "There is no untreated comparison group",
        "participant or outcome is therefore blocked",
    ] {
        if !wioa_reader.contains(required) {
            return Err(format!("WIOA outcome reader missing {required}"));
        }
    }

    if string_field(&card, "worker_population_baseline_path")? != BLS_CPS_WORKER_BASELINE_JSON_PATH
        || string_field(&card, "worker_population_baseline_status")?
            != "cps_population_employment_and_earnings_context_attached"
        || string_field(&wioa, "population_context_path")? != BLS_CPS_WORKER_BASELINE_JSON_PATH
        || string_field(&wioa, "population_context_status")?
            != "adjacent_population_baseline_not_counterfactual"
    {
        return Err("BLS CPS worker baseline link failed".to_string());
    }
    let bls_text = fs::read_to_string(root.join(BLS_CPS_WORKER_BASELINE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let bls: serde_json::Value = serde_json::from_str(&bls_text).map_err(|e| e.to_string())?;
    let education_pays = bls
        .get("education_pays_table_5_1")
        .ok_or("BLS education-pays table")?;
    let bls_definitions = bls
        .get("measure_definitions")
        .ok_or("BLS CPS measure definitions")?;
    let earnings_rows = education_pays
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or("BLS education-pays rows")?;
    let expected_earnings = [
        ("doctoral_degree", 2_278.0, 1.2),
        ("professional_degree", 2_363.0, 1.3),
        ("masters_degree", 1_840.0, 2.2),
        ("bachelors_degree", 1_543.0, 2.5),
        ("associates_degree", 1_099.0, 2.8),
        ("some_college_no_degree", 1_020.0, 3.8),
        ("high_school_diploma", 930.0, 4.2),
        ("less_than_high_school_diploma", 738.0, 6.2),
        ("total", 1_221.0, 3.3),
    ];
    if number_field(&bls, "calendar_year")? != 2024.0
        || earnings_rows.len() != expected_earnings.len()
        || string_field(education_pays, "earnings_universe")?
            != "Full-time wage and salary workers age 25 and older"
        || string_field(education_pays, "unemployment_universe")?
            != "Civilian labor force age 25 and older"
        || !string_field(bls_definitions, "median_usual_weekly_earnings")?
            .contains("excludes self-employed")
        || !string_field(bls_definitions, "full_time")?.contains("35 hours")
    {
        return Err("BLS education-pays scope failed".to_string());
    }
    for (row, expected_row) in earnings_rows.iter().zip(expected_earnings) {
        if string_field(row, "educational_attainment")? != expected_row.0
            || number_field(row, "median_usual_weekly_earnings_usd")? != expected_row.1
            || number_field(row, "unemployment_rate_percent")? != expected_row.2
        {
            return Err("BLS education-pays value failed".to_string());
        }
    }
    let annual = bls
        .get("cps_annual_average_table_7")
        .ok_or("BLS CPS annual-average table")?;
    let population_rows = annual
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or("BLS CPS population rows")?;
    let expected_population = [
        (
            "less_than_high_school_diploma",
            19_295.0,
            9_153.0,
            47.4,
            8_589.0,
            44.5,
            563.0,
            6.2,
        ),
        (
            "high_school_graduate_no_college",
            63_705.0,
            36_257.0,
            56.9,
            34_725.0,
            54.5,
            1_532.0,
            4.2,
        ),
        (
            "some_college_or_associates_degree_total",
            56_987.0,
            35_839.0,
            62.9,
            34_627.0,
            60.8,
            1_212.0,
            3.4,
        ),
        (
            "some_college_no_degree",
            32_179.0,
            19_489.0,
            60.6,
            18_740.0,
            58.2,
            749.0,
            3.8,
        ),
        (
            "associates_degree",
            24_808.0,
            16_350.0,
            65.9,
            15_887.0,
            64.0,
            463.0,
            2.8,
        ),
        (
            "bachelors_degree_and_higher_total",
            89_612.0,
            65_080.0,
            72.6,
            63_571.0,
            70.9,
            1_509.0,
            2.3,
        ),
        (
            "bachelors_degree_only",
            55_131.0,
            39_897.0,
            72.4,
            38_885.0,
            70.5,
            1_012.0,
            2.5,
        ),
        (
            "advanced_degree",
            34_481.0,
            25_183.0,
            73.0,
            24_686.0,
            71.6,
            497.0,
            2.0,
        ),
    ];
    if population_rows.len() != expected_population.len() {
        return Err("BLS CPS population row count failed".to_string());
    }
    for (row, expected_row) in population_rows.iter().zip(expected_population) {
        let population = number_field(row, "civilian_noninstitutional_population_thousands")?;
        let labor_force = number_field(row, "civilian_labor_force_thousands")?;
        let employed = number_field(row, "employed_thousands")?;
        let unemployed = number_field(row, "unemployed_thousands")?;
        let participation = number_field(row, "labor_force_participation_rate_percent")?;
        let employment_ratio = number_field(row, "employment_population_ratio_percent")?;
        let unemployment_rate = number_field(row, "unemployment_rate_percent")?;
        if string_field(row, "educational_attainment")? != expected_row.0
            || population != expected_row.1
            || labor_force != expected_row.2
            || participation != expected_row.3
            || employed != expected_row.4
            || employment_ratio != expected_row.5
            || unemployed != expected_row.6
            || unemployment_rate != expected_row.7
            || (labor_force - (employed + unemployed)).abs() > 1.0
            || ((100.0 * labor_force / population) - participation).abs() > 0.1
            || ((100.0 * employed / population) - employment_ratio).abs() > 0.1
            || ((100.0 * unemployed / labor_force) - unemployment_rate).abs() > 0.1
        {
            return Err("BLS CPS population employment value failed".to_string());
        }
    }
    let universe = bls
        .get("universe_reconciliation")
        .ok_or("BLS CPS universe reconciliation")?;
    let bls_wioa = bls.get("wioa_link").ok_or("BLS CPS WIOA link")?;
    let bls_subfunction = bls
        .get("subfunction_link")
        .ok_or("BLS CPS subfunction link")?;
    if string_field(universe, "status")? != "distinct_universes_preserved_not_joined_person_level"
        || string_field(bls_wioa, "status")? != "adjacent_population_context_not_counterfactual"
        || string_field(bls_subfunction, "scope_status")?
            != "population_context_not_account_crosswalk"
        || string_field(bls_subfunction, "cost_per_outcome_status")? != "blocked_not_calculated"
        || string_field(&bls, "comparison_status")?
            != "descriptive_cross_sectional_association_not_causal"
        || string_field(&bls, "fraud_status")? != "not_measured_not_inferred"
        || string_field(&bls, "savings_status")? != "blocked_not_scored"
    {
        return Err("BLS CPS worker evidence boundary failed".to_string());
    }
    let bls_reader = fs::read_to_string(root.join(BLS_CPS_WORKER_BASELINE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        BLS_CPS_WORKER_BASELINE_JSON_PATH,
        "$2,363",
        "$738",
        "do not have the same universe",
        "They are not annual earnings",
        "not a counterfactual cohort for WIOA",
        "cost per worker or outcome is blocked",
    ] {
        if !bls_reader.contains(required) {
            return Err(format!("BLS CPS worker reader missing {required}"));
        }
    }

    let training_component = parts
        .iter()
        .find(|v| v.get("subfunction_code").and_then(|x| x.as_str()) == Some("504"))
        .ok_or("training and employment component")?;
    if string_field(training_component, "account_bridge_path")?
        != TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH
        || string_field(training_component, "account_bridge_status")?
            != "exact_account_reconciliation_program_activity_mapping_attached_cohort_cost_blocked"
        || string_field(&card, "training_employment_account_bridge_path")?
            != TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH
        || string_field(&wioa, "account_bridge_path")? != TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH
        || string_field(&wioa, "account_bridge_status")?
            != "program_to_budget_activity_complete_actual_outlay_and_cohort_cost_blocked"
    {
        return Err("training and employment account bridge link failed".to_string());
    }
    let training_text = fs::read_to_string(root.join(TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let training: serde_json::Value =
        serde_json::from_str(&training_text).map_err(|e| e.to_string())?;
    let training_rows = training
        .get("account_rows")
        .and_then(|v| v.as_array())
        .ok_or("training and employment account rows")?;
    let expected_training_rows = [
        ("0172", 130.0),
        ("0172", 1.0),
        ("0174", 15.0),
        ("0174", 1_700.0),
        ("0174", 85.0),
        ("0175", 337.0),
        ("0179", 41.0),
        ("0179", 72.0),
        ("0179", 14.0),
        ("0181", 1_851.0),
        ("0326", 34.0),
        ("8042", 172.0),
        ("8042", 982.0),
    ];
    let training_sum: f64 = training_rows
        .iter()
        .map(|row| {
            row.get(7)
                .and_then(|v| v.as_f64())
                .ok_or("training account row amount")
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if training_rows.len() != expected_training_rows.len()
        || number_field(&training, "classified_row_count")? != 27.0
        || number_field(&training, "nonzero_row_count")? != 13.0
        || training_sum != 5_434.0
        || number_field(&training, "historical_table_total")? != 5_434.0
        || number_field(&training, "public_budget_database_total")? != 5_434.0
        || number_field(&training, "reconciliation_difference")? != 0.0
    {
        return Err("training and employment account reconciliation failed".to_string());
    }
    for (row, expected_row) in training_rows.iter().zip(expected_training_rows) {
        if row.get(2).and_then(|v| v.as_str()) != Some(expected_row.0)
            || row.get(7).and_then(|v| v.as_f64()) != Some(expected_row.1)
            || row.get(0).and_then(|v| v.as_str()) != Some("012")
            || row.get(1).and_then(|v| v.as_str()) != Some("05")
            || row.get(6).and_then(|v| v.as_str()) != Some("On-budget")
        {
            return Err("training and employment account row failed".to_string());
        }
    }
    let account_totals = training
        .get("account_totals")
        .and_then(|v| v.as_array())
        .ok_or("training account totals")?;
    let expected_account_totals = [
        ("0172", 131.0),
        ("0174", 1_800.0),
        ("0175", 337.0),
        ("0179", 127.0),
        ("0181", 1_851.0),
        ("0326", 34.0),
        ("8042", 1_154.0),
    ];
    if account_totals.len() != expected_account_totals.len() {
        return Err("training account total row count failed".to_string());
    }
    for (row, expected_row) in account_totals.iter().zip(expected_account_totals) {
        if string_field(row, "account_code")? != expected_row.0
            || number_field(row, "amount")? != expected_row.1
        {
            return Err("training account total failed".to_string());
        }
    }
    let ba = training
        .get("program_budget_authority_comparison")
        .ok_or("training program budget authority comparison")?;
    let ba_rows = ba
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or("training program budget authority rows")?;
    let expected_ba = [885.649, 948.130, 1_095.553, 21.413, 653.639];
    if ba_rows.len() != expected_ba.len()
        || number_field(ba, "wioa_title_i_subtotal")? != 2_929.332
        || number_field(ba, "wagner_peyser_grants_to_states_subtotal")? != 675.052
        || number_field(ba, "matched_program_total")? != 3_604.384
        || number_field(
            ba,
            "training_and_employment_services_total_budget_authority",
        )? != 3_898.587
        || number_field(
            ba,
            "training_and_employment_services_account_actual_outlays",
        )? != 1_800.0
    {
        return Err("training program budget authority comparison failed".to_string());
    }
    for (row, amount) in ba_rows.iter().zip(expected_ba) {
        if number_field(row, "amount")? != amount
            || string_field(row, "mapping_status")?
                != "program_to_budget_activity_exact_account_outlay_not_separable"
        {
            return Err("training program budget activity row failed".to_string());
        }
    }
    if string_field(&training, "bridge_status")? != "exact_account_reconciliation_complete"
        || string_field(&training, "outcome_cohort_allocation_status")?
            != "blocked_actual_outlays_not_separable"
        || string_field(&training, "cost_per_outcome_status")? != "blocked_not_calculated"
        || string_field(&training, "causal_status")? != "not_established"
        || string_field(&training, "fraud_status")? != "not_measured_not_inferred"
        || string_field(&training, "savings_status")? != "blocked_not_scored"
    {
        return Err("training account bridge evidence boundary failed".to_string());
    }
    let training_reader = fs::read_to_string(root.join(TRAINING_EMPLOYMENT_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        TRAINING_EMPLOYMENT_BRIDGE_JSON_PATH,
        "$5.434B",
        "$3,604.384M",
        "Budget authority is not outlays",
        "outlays cannot be allocated to those cohorts",
        "participant or outcome is therefore blocked",
    ] {
        if !training_reader.contains(required) {
            return Err(format!("training account bridge reader missing {required}"));
        }
    }

    if string_field(&card, "worker_causal_evidence_path")? != WIA_GOLD_STANDARD_JSON_PATH
        || string_field(&card, "worker_causal_evidence_status")?
            != "historical_wia_randomized_impact_evidence_attached_current_wioa_transportability_blocked"
        || string_field(&wioa, "historical_causal_evidence_path")? != WIA_GOLD_STANDARD_JSON_PATH
        || string_field(&wioa, "historical_causal_evidence_status")?
            != "historical_wia_randomized_contrasts_not_py2024_wioa_effect"
        || string_field(&training, "historical_causal_evidence_path")?
            != WIA_GOLD_STANDARD_JSON_PATH
    {
        return Err("WIA Gold Standard evidence link failed".to_string());
    }
    let wia_text =
        fs::read_to_string(root.join(WIA_GOLD_STANDARD_JSON_PATH)).map_err(|e| e.to_string())?;
    let wia: serde_json::Value = serde_json::from_str(&wia_text).map_err(|e| e.to_string())?;
    let design = wia.get("study_design").ok_or("WIA Gold Standard design")?;
    let enrollment = design
        .get("enrollment_period")
        .ok_or("WIA Gold Standard enrollment period")?;
    let follow_up = design
        .get("follow_up")
        .ok_or("WIA Gold Standard follow-up")?;
    if string_field(design, "design")? != "randomized_control_trial"
        || number_field(design, "randomized_applicants")? != 35_665.0
        || number_field(design, "impact_study_sample")? != 34_429.0
        || number_field(design, "ndnh_analysis_sample")? != 33_773.0
        || number_field(design, "local_workforce_investment_areas")? != 28.0
        || string_field(enrollment, "start")? != "2011-11"
        || string_field(enrollment, "end")? != "2013-04"
        || number_field(follow_up, "survey_months")? != 30.0
        || number_field(follow_up, "ndnh_administrative_earnings_months")? != 36.0
        || number_field(design, "restriction_period_months")? != 15.0
    {
        return Err("WIA Gold Standard study design failed".to_string());
    }
    let earnings = wia
        .get("cumulative_earnings_2012_usd")
        .ok_or("WIA Gold Standard earnings")?;
    let survey = earnings
        .get("survey_30_month")
        .ok_or("WIA Gold Standard survey earnings")?;
    let ndnh = earnings
        .get("ndnh_36_month")
        .ok_or("WIA Gold Standard NDNH earnings")?;
    let expected_earnings = [
        (survey, 39_528.0, 43_211.0, 36_079.0, 7_133.0, -3_684.0),
        (ndnh, 46_509.0, 47_960.0, 44_664.0, 3_296.0, -1_451.0),
    ];
    for (source, full, intensive, core, intensive_impact, training_impact) in expected_earnings {
        let means = source
            .get("group_means")
            .ok_or("WIA Gold Standard earnings means")?;
        let contrasts = source
            .get("contrasts")
            .and_then(|v| v.as_array())
            .ok_or("WIA Gold Standard earnings contrasts")?;
        if contrasts.len() != 2
            || number_field(means, "full_wia")? != full
            || number_field(means, "core_and_intensive")? != intensive
            || number_field(means, "core_only")? != core
            || number_field(&contrasts[0], "impact")? != intensive_impact
            || string_field(&contrasts[0], "service_contrast")? != "intensive_services"
            || number_field(&contrasts[1], "impact")? != training_impact
            || string_field(&contrasts[1], "service_contrast")? != "training_services"
            || string_field(&contrasts[1], "significance_status")? != "not_significant"
        {
            return Err("WIA Gold Standard earnings contrast failed".to_string());
        }
    }
    let receipt = wia
        .get("service_receipt_percent")
        .ok_or("WIA Gold Standard service receipt")?;
    let training_receipt = receipt
        .get("training")
        .ok_or("WIA Gold Standard training receipt")?;
    let credentials = wia
        .get("credential_and_completion_percent")
        .ok_or("WIA Gold Standard credentials")?;
    let any_credential = credentials
        .get("any_credential")
        .ok_or("WIA Gold Standard any credential")?;
    if number_field(training_receipt, "full_wia")? != 50.0
        || number_field(training_receipt, "core_and_intensive")? != 41.0
        || number_field(training_receipt, "core_only")? != 34.0
        || number_field(
            receipt,
            "training_take_up_contrast_full_minus_core_and_intensive_percentage_points",
        )? != 9.0
        || number_field(any_credential, "full_wia")? != 29.0
        || number_field(any_credential, "core_and_intensive")? != 24.0
        || number_field(any_credential, "core_only")? != 15.0
    {
        return Err("WIA Gold Standard service or credential value failed".to_string());
    }
    let benefit_cost = wia
        .get("benefit_cost_analysis_per_customer_2012_usd")
        .ok_or("WIA Gold Standard benefit-cost")?;
    let expected_benefit_cost = [
        ("training_services", -5_046.0, -3_274.0, -1_773.0),
        ("intensive_services", 8_573.0, 6_630.0, 1_943.0),
        ("combined_intensive_and_training", 3_526.0, 3_356.0, 170.0),
    ];
    for (field, society, customers, taxpayers) in expected_benefit_cost {
        let row = benefit_cost.get(field).ok_or("WIA benefit-cost row")?;
        if number_field(row, "society")? != society
            || number_field(row, "customers")? != customers
            || number_field(row, "taxpayers")? != taxpayers
        {
            return Err("WIA Gold Standard benefit-cost value failed".to_string());
        }
    }
    let assessment = wia
        .get("evidence_assessment")
        .ok_or("WIA Gold Standard evidence assessment")?;
    if string_field(assessment, "causal_status")?
        != "causal_for_randomized_historical_wia_service_access_contrasts"
        || string_field(assessment, "training_services_finding")?
            != "inconclusive_due_small_take_up_contrast_and_nonsignificant_impacts"
        || string_field(assessment, "wioa_transportability")?
            != "blocked_predecessor_program_and_changed_operating_context"
        || string_field(assessment, "current_fiscal_link")?
            != "blocked_no_cohort_compatible_current_outlay_crosswalk"
        || string_field(assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(assessment, "savings_status")? != "not_current_savings_not_scored"
    {
        return Err("WIA Gold Standard evidence boundary failed".to_string());
    }
    let wia_reader =
        fs::read_to_string(root.join(WIA_GOLD_STANDARD_READER_PATH)).map_err(|e| e.to_string())?;
    for required in [
        WIA_GOLD_STANDARD_JSON_PATH,
        "+$7,133",
        "nine percentage points",
        "not proof of harm",
        "service-access contrasts",
        "does not eliminate transportability limits",
        "Current cost per outcome, recoverable savings",
    ] {
        if !wia_reader.contains(required) {
            return Err(format!("WIA Gold Standard reader missing {required}"));
        }
    }

    if string_field(&card, "education_access_transition_path")?
        != CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH
        || string_field(&card, "education_access_transition_status")?
            != "cps_oct2024_age_enrollment_and_recent_graduate_transition_attached"
        || string_field(&outcome, "education_access_transition_path")?
            != CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH
        || string_field(&outcome, "access_status")?
            != "national_chronic_absenteeism_and_cps_age_enrollment_transition_context_attached_other_access_measures_open"
        || string_field(&bls, "education_transition_context_path")?
            != CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH
        || string_field(&bls, "education_transition_context_status")?
            != "cps_oct2024_younger_population_transition_context_not_age25plus_worker_cohort"
        || string_field(&wioa, "education_transition_context_path")?
            != CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH
        || string_field(&wioa, "education_transition_context_status")?
            != "cps_recent_graduate_snapshot_not_wioa_youth_comparison_group"
    {
        return Err("CPS education access and transition evidence link failed".to_string());
    }
    let access_text = fs::read_to_string(root.join(CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let access: serde_json::Value =
        serde_json::from_str(&access_text).map_err(|e| e.to_string())?;
    let source_ids = access
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("CPS education access source IDs")?;
    if string_field(&access, "record_family")? != "education_access_transition_baseline"
        || string_field(&access, "survey_period")? != "October 2024"
        || !source_ids
            .iter()
            .any(|value| value.as_str() == Some("SRC-CENSUS-CPS-SCHOOL-ENROLLMENT-2024"))
    {
        return Err("CPS education access scope failed".to_string());
    }
    let table_1 = access
        .get("table_1_enrollment_status")
        .ok_or("CPS education access Table 1")?;
    let headline = table_1
        .get("headline")
        .ok_or("CPS education access headline")?;
    let levels = table_1
        .get("enrollment_level_counts_thousands")
        .ok_or("CPS education access enrollment levels")?;
    if number_field(headline, "population_thousands")? != 321_500.0
        || number_field(headline, "enrolled_in_school_thousands")? != 75_110.0
        || number_field(headline, "enrolled_in_school_percent")? != 23.4
        || number_field(levels, "nursery_or_kindergarten")? != 8_510.0
        || number_field(levels, "elementary")? != 32_090.0
        || number_field(levels, "high_school")? != 17_120.0
        || number_field(levels, "college_undergraduate_or_graduate")? != 17_400.0
    {
        return Err("CPS education access headline failed".to_string());
    }
    let age_rows = table_1
        .get("age_rows")
        .and_then(|value| value.as_array())
        .ok_or("CPS education access age rows")?;
    let expected_age_rows = [
        ("3_and_4", 7_432.0, 4_368.0, 58.8),
        ("5_and_6", 7_657.0, 7_073.0, 92.4),
        ("7_to_9", 12_210.0, 11_820.0, 96.8),
        ("10_to_13", 16_470.0, 16_080.0, 97.7),
        ("14_and_15", 8_477.0, 8_245.0, 97.3),
        ("16_and_17", 8_973.0, 8_201.0, 91.4),
        ("18_and_19", 8_660.0, 5_724.0, 66.1),
        ("20_and_21", 8_507.0, 4_351.0, 51.1),
        ("22_to_24", 12_930.0, 3_419.0, 26.4),
    ];
    if age_rows.len() != expected_age_rows.len() {
        return Err("CPS education access age-row count failed".to_string());
    }
    for (row, expected) in age_rows.iter().zip(expected_age_rows) {
        if string_field(row, "age")? != expected.0
            || number_field(row, "population_thousands")? != expected.1
            || number_field(row, "enrolled_in_school_thousands")? != expected.2
            || number_field(row, "enrolled_in_school_percent")? != expected.3
        {
            return Err(format!(
                "CPS education access age row failed: {}",
                expected.0
            ));
        }
    }
    let table_7 = access
        .get("table_7_recent_high_school_graduate_transition")
        .ok_or("CPS education transition Table 7")?;
    let counts = table_7
        .get("both_sexes_counts_thousands")
        .ok_or("CPS education transition counts")?;
    let count_fields = [
        ("total", 3_250.0),
        ("two_year_college_full_time", 537.0),
        ("two_year_college_part_time", 130.0),
        ("four_year_college_full_time", 1_303.0),
        ("four_year_college_part_time", 60.0),
        ("graduate_school", 6.0),
        ("vocational_school", 52.0),
        ("not_enrolled_employed", 613.0),
        ("not_enrolled_not_employed", 549.0),
    ];
    for (field, expected) in count_fields {
        if number_field(counts, field)? != expected {
            return Err(format!("CPS education transition count failed: {field}"));
        }
    }
    let rates = table_7
        .get("derived_rates")
        .ok_or("CPS education transition rates")?;
    if number_field(rates, "both_sexes_any_listed_school_numerator_thousands")? != 2_088.0
        || (number_field(rates, "both_sexes_any_listed_school_percent")? - 64.246154).abs()
            > 0.000001
        || number_field(rates, "both_sexes_college_only_numerator_thousands")? != 2_036.0
        || (number_field(rates, "both_sexes_not_enrolled_not_employed_percent")? - 16.892308).abs()
            > 0.000001
        || !string_field(rates, "derivation_basis")?.contains("rounded to thousands")
    {
        return Err("CPS education transition derivation failed".to_string());
    }
    let worker_link = access
        .get("worker_evidence_link")
        .ok_or("CPS education transition worker link")?;
    let spending_link = access
        .get("spending_link")
        .ok_or("CPS education transition spending link")?;
    if string_field(&access, "comparison_status")?
        != "descriptive_cross_sectional_access_and_transition_baseline_not_causal"
        || string_field(&access, "program_attribution_status")?
            != "blocked_no_program_participation_or_counterfactual"
        || string_field(worker_link, "status")?
            != "descriptive_transition_context_not_program_evaluation"
        || string_field(spending_link, "status")? != "no_compatible_spending_or_program_cohort"
        || string_field(&access, "fraud_status")? != "not_measured_not_inferred"
        || string_field(&access, "savings_status")? != "blocked_not_scored"
    {
        return Err("CPS education access evidence boundary failed".to_string());
    }
    let access_reader = fs::read_to_string(root.join(CPS_EDUCATION_ACCESS_TRANSITION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH,
        "64.246154%",
        "not Census-published rates",
        "not person-level linked records",
        "no treatment or counterfactual",
        "cost per student",
        "cannot support program attribution",
    ] {
        if !access_reader.contains(required) {
            return Err(format!("CPS education access reader missing {required}"));
        }
    }

    if string_field(&card, "student_program_impact_evidence_path")?
        != PELL_SHORT_TRAINING_IMPACT_JSON_PATH
        || string_field(&card, "student_program_impact_evidence_status")?
            != "pell_short_training_randomized_itt_evidence_attached_fiscal_and_transportability_boundaries_open"
        || string_field(&bridge, "student_program_impact_evidence_path")?
            != PELL_SHORT_TRAINING_IMPACT_JSON_PATH
        || string_field(&bridge, "student_program_impact_evidence_status")?
            != "experimental_pell_offer_evidence_not_fy2025_account_outlay_or_cost_crosswalk"
        || string_field(&access, "program_impact_evidence_path")?
            != PELL_SHORT_TRAINING_IMPACT_JSON_PATH
        || string_field(&access, "program_impact_evidence_status")?
            != "randomized_pell_applicant_evidence_not_cps_population_or_recent_graduate_cohort"
    {
        return Err("Pell short-training evidence link failed".to_string());
    }
    let pell_text = fs::read_to_string(root.join(PELL_SHORT_TRAINING_IMPACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let pell: serde_json::Value = serde_json::from_str(&pell_text).map_err(|e| e.to_string())?;
    let design = pell
        .get("study_design")
        .ok_or("Pell short-training study design")?;
    if string_field(&pell, "record_family")? != "causal_impact_evidence"
        || string_field(design, "design")? != "randomized_control_trial"
        || string_field(design, "estimand")?
            != "intent_to_treat_effect_of_being_offered_experimental_pell_grant_funds"
        || number_field(design, "randomized_sample_total")? != 2_914.0
        || number_field(design, "primary_education_analytic_sample")? != 2_684.0
        || number_field(design, "participating_postsecondary_schools")? != 46.0
        || number_field(design, "assignment_probability_offer_percent")? != 60.0
        || !string_field(design, "statistical_test")?.contains("p < 0.001")
    {
        return Err("Pell short-training design failed".to_string());
    }
    let exp_1 = pell
        .get("experiment_1")
        .ok_or("Pell short-training experiment 1")?;
    let exp_2 = pell
        .get("experiment_2")
        .ok_or("Pell short-training experiment 2")?;
    let expected_experiments = [
        (
            exp_1,
            414.0,
            [
                (77.9, 51.9, 26.0, 4.0, 0.0),
                (52.4, 35.6, 16.7, 4.3, 0.0),
                (80.6, 83.0, -2.4, 3.8, 0.527),
                (8_956.0, 10_097.0, -1_141.0, 977.0, 0.244),
            ],
        ),
        (
            exp_2,
            2_270.0,
            [
                (66.4, 51.8, 14.6, 1.8, 0.0),
                (47.0, 37.7, 9.3, 1.9, 0.0),
                (77.6, 79.0, -1.4, 1.8, 0.424),
                (5_993.0, 6_276.0, -283.0, 251.0, 0.260),
            ],
        ),
    ];
    for (experiment, expected_sample, expected_rows) in expected_experiments {
        let profile = experiment
            .get("participant_profile")
            .ok_or("Pell short-training participant profile")?;
        let outcomes = experiment
            .get("primary_outcomes")
            .and_then(|value| value.as_array())
            .ok_or("Pell short-training primary outcomes")?;
        if number_field(profile, "analytic_sample")? != expected_sample
            || outcomes.len() != expected_rows.len()
        {
            return Err("Pell short-training experiment scope failed".to_string());
        }
        for (row, expected) in outcomes.iter().zip(expected_rows) {
            if number_field(row, "offered_mean")? != expected.0
                || number_field(row, "not_offered_mean")? != expected.1
                || number_field(row, "impact")? != expected.2
                || number_field(row, "standard_error")? != expected.3
                || number_field(row, "p_value")? != expected.4
            {
                return Err("Pell short-training outcome estimate failed".to_string());
            }
        }
    }
    let grant = pell
        .get("grant_take_up_and_amount_context")
        .ok_or("Pell short-training grant context")?;
    let grant_1 = grant
        .get("experiment_1")
        .ok_or("Pell short-training grant experiment 1")?;
    let grant_2 = grant
        .get("experiment_2")
        .ok_or("Pell short-training grant experiment 2")?;
    let combined = grant
        .get("combined")
        .ok_or("Pell short-training combined grant context")?;
    if number_field(grant_1, "grant_users_n")? != 170.0
        || number_field(grant_1, "average_disbursed_among_users_usd")? != 3_577.0
        || number_field(grant_2, "grant_users_n")? != 705.0
        || number_field(grant_2, "average_disbursed_among_users_usd")? != 1_312.0
        || number_field(combined, "grant_users_n")? != 875.0
        || number_field(combined, "total_disbursed_usd")? != 1_532_657.0
        || number_field(grant, "combined_average_disbursed_among_users_usd")? != 1_752.0
    {
        return Err("Pell short-training grant context failed".to_string());
    }
    let assessment = pell
        .get("evidence_assessment")
        .ok_or("Pell short-training evidence assessment")?;
    if string_field(assessment, "causal_status")?
        != "causal_for_offer_of_experimental_pell_eligibility_under_study_conditions"
        || string_field(assessment, "employment_finding")?
            != "no_statistically_detectable_medium_to_long_term_employment_effect_in_either_experiment"
        || string_field(assessment, "earnings_finding")?
            != "no_statistically_detectable_medium_to_long_term_quarterly_earnings_effect_in_either_experiment"
        || !string_field(assessment, "null_interpretation")?
            .contains("does not establish equivalence")
        || string_field(assessment, "cost_effectiveness_status")?
            != "blocked_no_full_incremental_cost_and_benefit_analysis"
        || string_field(assessment, "current_fiscal_link")?
            != "blocked_no_compatible_outlay_or_budget_cohort"
        || string_field(assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(assessment, "savings_status")? != "blocked_not_scored"
    {
        return Err("Pell short-training evidence boundary failed".to_string());
    }
    let pell_reader = fs::read_to_string(root.join(PELL_SHORT_TRAINING_IMPACT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        PELL_SHORT_TRAINING_IMPACT_JSON_PATH,
        "intent-to-treat",
        "not the effect of using a grant",
        "93 months after random assignment",
        "$1,752",
        "not proof of harm",
        "not prove equivalence",
        "cohort-compatible federal outlay crosswalk",
    ] {
        if !pell_reader.contains(required) {
            return Err(format!("Pell short-training reader missing {required}"));
        }
    }

    if string_field(&card, "student_aid_program_access_path")?
        != FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH
        || string_field(&card, "student_aid_program_access_status")?
            != "fy2024_title_iv_and_pell_administrative_scale_attached_mixed_period_outlay_and_outcome_links_blocked"
        || string_field(&bridge, "student_aid_program_access_path")?
            != FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH
        || string_field(&bridge, "student_aid_program_access_status")?
            != "fy2024_title_iv_program_scale_not_fy2025_account_outlay_or_reconciliation_crosswalk"
        || string_field(&access, "student_aid_program_access_path")?
            != FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH
        || string_field(&access, "student_aid_program_access_status")?
            != "fsa_administrative_aid_universe_not_cps_population_or_transition_denominator"
        || string_field(&pell, "current_program_access_path")?
            != FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH
        || string_field(&pell, "current_program_access_status")?
            != "fy2024_fsa_administrative_scale_not_experimental_offer_population_or_counterfactual"
    {
        return Err("FSA Title IV student access evidence link failed".to_string());
    }
    let fsa_text = fs::read_to_string(root.join(FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let fsa: serde_json::Value = serde_json::from_str(&fsa_text).map_err(|e| e.to_string())?;
    let fsa_sources = fsa
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("FSA Title IV source IDs")?;
    let headline = fsa
        .get("student_access_headline")
        .ok_or("FSA Title IV student access headline")?;
    if string_field(&fsa, "record_family")? != "student_program_access_baseline"
        || number_field(&fsa, "fiscal_year")? != 2024.0
        || !fsa_sources
            .iter()
            .any(|value| value.as_str() == Some("SRC-ED-FSA-ANNUAL-REPORT-FY2024"))
        || number_field(headline, "fafsa_forms_processed_more_than_millions")? != 17.6
        || number_field(
            headline,
            "title_iv_aid_delivered_approximately_millions_usd",
        )? != 120_800.0
        || number_field(
            headline,
            "postsecondary_students_and_families_more_than_millions",
        )? != 9.9
        || number_field(headline, "active_participating_postsecondary_institutions")? != 5_378.0
        || !string_field(headline, "qualifier_note")?.contains("not converted to exact")
    {
        return Err("FSA Title IV student access scope failed".to_string());
    }
    let table_4 = fsa.get("table_4").ok_or("FSA Title IV Table 4")?;
    let fsa_rows = table_4
        .get("rows")
        .and_then(|value| value.as_array())
        .ok_or("FSA Title IV Table 4 rows")?;
    let expected_fsa_rows = [
        (85_802.4, 83_295.3, 2_507.1),
        (32_995.7, 28_689.2, 4_306.5),
        (871.5, 893.8, -22.3),
        (42.3, 82.1, -39.8),
        (0.7, 0.6, 0.1),
        (1_103.5, 1_150.2, -46.7),
    ];
    if fsa_rows.len() != expected_fsa_rows.len() {
        return Err("FSA Title IV Table 4 row count failed".to_string());
    }
    for (row, expected) in fsa_rows.iter().zip(expected_fsa_rows) {
        if number_field(row, "fy2024_aid_disbursed")? != expected.0
            || number_field(row, "fy2023_aid_disbursed")? != expected.1
            || number_field(row, "difference")? != expected.2
        {
            return Err("FSA Title IV Table 4 amount failed".to_string());
        }
    }
    let grant_subtotal = table_4
        .get("published_grant_subtotal")
        .ok_or("FSA Title IV grant subtotal")?;
    let grand_total = table_4
        .get("published_grand_total")
        .ok_or("FSA Title IV grand total")?;
    let iraq_row = fsa_rows
        .iter()
        .find(|row| {
            row.get("program").and_then(|value| value.as_str())
                == Some("Iraq and Afghanistan Service Grant Program")
        })
        .ok_or("FSA Iraq and Afghanistan grant row")?;
    if number_field(grant_subtotal, "fy2024_aid_disbursed")? != 33_910.2
        || number_field(grand_total, "fy2024_aid_disbursed")? != 120_816.1
        || number_field(grand_total, "fy2023_aid_disbursed")? != 114_111.2
        || iraq_row
            .get("published_sign_matches_difference")
            .and_then(|value| value.as_bool())
            != Some(false)
        || grant_subtotal
            .get("published_sign_matches_difference")
            .and_then(|value| value.as_bool())
            != Some(false)
    {
        return Err("FSA Title IV Table 4 reconciliation failed".to_string());
    }
    let access_details = fsa
        .get("program_access_details")
        .ok_or("FSA Title IV program access details")?;
    let pell_access = access_details
        .get("pell_grants")
        .ok_or("FSA Pell access details")?;
    if number_field(
        pell_access,
        "fy2024_disbursements_approximately_millions_usd",
    )? != 33_000.0
        || number_field(pell_access, "average_grant_usd")? != 5_218.0
        || number_field(pell_access, "students_more_than_millions")? != 6.3
        || number_field(pell_access, "maximum_award_usd")? != 7_395.0
        || !string_field(pell_access, "period_note")?.contains("do not create a common")
    {
        return Err("FSA Pell access boundary failed".to_string());
    }
    if string_field(&fsa, "education_access_transition_path")?
        != CPS_EDUCATION_ACCESS_TRANSITION_JSON_PATH
        || string_field(&fsa, "student_program_impact_evidence_path")?
            != PELL_SHORT_TRAINING_IMPACT_JSON_PATH
        || string_field(&fsa, "higher_education_account_bridge_path")?
            != HIGHER_EDUCATION_BRIDGE_JSON_PATH
        || string_field(&fsa, "comparison_status")?
            != "descriptive_administrative_access_and_disbursement_baseline_not_outcome_or_causal_evidence"
        || string_field(&fsa, "period_alignment_status")?
            != "fiscal_year_disbursements_and_award_year_recipient_counts_not_cohort_aligned"
        || string_field(&fsa, "account_crosswalk_status")?
            != "blocked_no_omb_function_subfunction_or_account_row_crosswalk"
        || string_field(&fsa, "cost_per_student_or_outcome_status")? != "blocked_not_calculated"
        || string_field(&fsa, "causal_status")? != "not_applicable_no_counterfactual"
        || string_field(&fsa, "fraud_status")? != "not_measured_not_inferred"
        || string_field(&fsa, "savings_status")? != "blocked_not_scored"
    {
        return Err("FSA Title IV evidence boundary failed".to_string());
    }
    let fsa_reader = fs::read_to_string(root.join(FSA_TITLE_IV_STUDENT_ACCESS_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        FSA_TITLE_IV_STUDENT_ACCESS_JSON_PATH,
        "more than 9.9 million",
        "more than 6.3 million",
        "$120,816.1",
        "$32,995.7",
        "based on award year",
        "does not reverse-engineer exact recipients",
        "support cost per student or outcome",
    ] {
        if !fsa_reader.contains(required) {
            return Err(format!("FSA Title IV reader missing {required}"));
        }
    }

    let contains_source_id = |record: &serde_json::Value, expected: &str| {
        record
            .get("source_ids")
            .and_then(|value| value.as_array())
            .map(|source_ids| {
                source_ids
                    .iter()
                    .any(|source_id| source_id.as_str() == Some(expected))
            })
            .unwrap_or(false)
    };
    if !contains_source_id(&card, PELL_BACHELOR_OUTCOME_SOURCE_ID)
        || !contains_source_id(&bridge, PELL_BACHELOR_OUTCOME_SOURCE_ID)
        || string_field(&card, "bachelor_completer_pell_outcome_path")?
            != PELL_BACHELOR_OUTCOME_JSON_PATH
        || string_field(&card, "bachelor_completer_pell_outcome_status")?
            != "bb1620_descriptive_ever_pell_bachelor_completer_outcomes_attached_completion_cost_and_fiscal_inference_blocked"
        || string_field(&bridge, "bachelor_completer_pell_outcome_path")?
            != PELL_BACHELOR_OUTCOME_JSON_PATH
        || string_field(&bridge, "bachelor_completer_pell_outcome_status")?
            != "historical_completion_conditioned_survey_outcomes_not_fy2025_account_outlay_or_cost_crosswalk"
        || string_field(&fsa, "bachelor_completer_pell_outcome_path")?
            != PELL_BACHELOR_OUTCOME_JSON_PATH
        || string_field(&fsa, "bachelor_completer_pell_outcome_status")?
            != "historical_ever_pell_bachelor_completer_outcomes_not_fy2024_title_iv_recipient_or_disbursement_cohort"
        || string_field(&pell, "bachelor_completer_pell_outcome_path")?
            != PELL_BACHELOR_OUTCOME_JSON_PATH
        || string_field(&pell, "bachelor_completer_pell_outcome_status")?
            != "descriptive_bachelor_completer_ever_pell_contrasts_not_experimental_offer_population_or_counterfactual"
    {
        return Err("Pell bachelor-recipient outcome evidence link failed".to_string());
    }
    let bachelor_text = fs::read_to_string(root.join(PELL_BACHELOR_OUTCOME_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let bachelor: serde_json::Value =
        serde_json::from_str(&bachelor_text).map_err(|e| e.to_string())?;
    let bachelor_source_ids = bachelor
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("Pell bachelor-recipient source IDs")?;
    if bachelor_source_ids.len() != 1
        || !contains_source_id(&bachelor, PELL_BACHELOR_OUTCOME_SOURCE_ID)
    {
        return Err("Pell bachelor-recipient source membership failed".to_string());
    }
    let metadata_text = fs::read_to_string(root.join(PELL_BACHELOR_OUTCOME_METADATA_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        format!("| `source_id` | `{PELL_BACHELOR_OUTCOME_SOURCE_ID}` |"),
        format!("| `raw_path` | `{PELL_BACHELOR_OUTCOME_RAW_PATH}` |"),
        format!("| `report_bytes` | `{PELL_BACHELOR_OUTCOME_RAW_BYTES}` |"),
        format!(
            "| `report_checksum_sha256` | `{}` |",
            PELL_BACHELOR_OUTCOME_RAW_SHA256.to_ascii_uppercase()
        ),
        "| `status` | `captured-checksum-verified` |".to_string(),
    ] {
        if !metadata_text.contains(&required) {
            return Err(format!(
                "Pell bachelor-recipient metadata missing {required}"
            ));
        }
    }
    let raw_path = root.join(PELL_BACHELOR_OUTCOME_RAW_PATH);
    let raw_bytes = fs::metadata(&raw_path).map_err(|e| e.to_string())?.len();
    let raw_sha256 = sha256_file(&raw_path)?;
    if raw_bytes != PELL_BACHELOR_OUTCOME_RAW_BYTES
        || raw_sha256 != PELL_BACHELOR_OUTCOME_RAW_SHA256
    {
        return Err("Pell bachelor-recipient raw custody failed".to_string());
    }
    let cohort = bachelor
        .get("cohort")
        .ok_or("Pell bachelor-recipient cohort")?;
    let pell_group = bachelor
        .get("pell_group_definition")
        .ok_or("Pell bachelor-recipient group definition")?;
    if string_field(&bachelor, "record_family")? != "student_aid_longitudinal_outcome_baseline"
        || string_field(&bachelor, "study")?
            != "2016/20 Baccalaureate and Beyond Longitudinal Study"
        || string_field(&bachelor, "report_title")?
            != "Baccalaureate and Beyond (B&B:16/20): A First Look at the 2020 Employment and Education Experiences of 2015-16 College Graduates"
        || string_field(&bachelor, "report_number")? != "NCES 2022-241"
        || string_field(&bachelor, "publication_date")? != "2022-09"
        || string_field(cohort, "target_population")?
            != "People who completed requirements for a bachelor's degree between July 1, 2015, and June 30, 2016, and were awarded that degree by a Title IV eligible institution in the 50 states, District of Columbia, or Puerto Rico no later than June 30, 2017."
        || number_field(cohort, "represented_population_approximate")? != 2_000_000.0
        || number_field(cohort, "sampled_students_rounded")? != 26_510.0
        || number_field(cohort, "eligible_students_rounded")? != 26_250.0
        || number_field(cohort, "survey_respondents_rounded")? != 17_160.0
        || number_field(cohort, "unweighted_response_rate_percent")? != 65.4
        || number_field(cohort, "weighted_response_rate_percent")? != 62.5
        || string_field(cohort, "outcome_timing")?
            != "Four years after 2015-16 bachelor's degree completion, with data collected in 2020."
        || !string_field(cohort, "completion_condition")?.contains("cannot estimate")
        || string_field(pell_group, "variable")? != "PELLCUM"
        || string_field(pell_group, "label")? != "Ever received a Pell Grant"
        || !string_field(pell_group, "definition")?.contains("1993-94 and 2015-16")
        || number_field(pell_group, "pell_yes_percent")? != 50.9
        || number_field(pell_group, "pell_yes_standard_error")? != 0.02
        || number_field(pell_group, "pell_no_percent")? != 49.1
        || number_field(pell_group, "pell_no_standard_error")? != 0.02
        || !string_field(pell_group, "interpretation")?
            .contains("observational lifetime-receipt categories")
    {
        return Err("Pell bachelor-recipient cohort scope failed".to_string());
    }
    let check_number_array =
        |object: &serde_json::Value, field: &str, expected: &[f64]| -> Result<(), String> {
            let values = object
                .get(field)
                .and_then(|value| value.as_array())
                .ok_or_else(|| format!("Pell bachelor-recipient {field}"))?;
            if values.len() != expected.len()
                || values
                    .iter()
                    .zip(expected)
                    .any(|(value, expected)| value.as_f64() != Some(*expected))
            {
                return Err(format!("Pell bachelor-recipient {field} failed"));
            }
            Ok(())
        };
    let employment = bachelor
        .get("table_a2_postbaccalaureate_employment_and_enrollment")
        .ok_or("Pell bachelor-recipient employment table")?;
    if string_field(employment, "universe")?
        != "All 2015-16 bachelor's degree earners in the B&B:16/20 target population."
    {
        return Err("Pell bachelor-recipient employment universe failed".to_string());
    }
    let employment_yes = employment
        .get("pell_yes")
        .ok_or("Pell bachelor-recipient employment Pell yes")?;
    let employment_no = employment
        .get("pell_no")
        .ok_or("Pell bachelor-recipient employment Pell no")?;
    check_number_array(employment_yes, "estimates", &[69.0, 11.8, 4.2, 4.2, 10.8])?;
    check_number_array(
        employment_yes,
        "standard_errors",
        &[0.78, 0.50, 0.33, 0.34, 0.49],
    )?;
    check_number_array(employment_no, "estimates", &[71.7, 11.0, 7.4, 2.7, 7.2])?;
    check_number_array(
        employment_no,
        "standard_errors",
        &[0.67, 0.52, 0.47, 0.33, 0.42],
    )?;
    let debt = bachelor
        .get("table_a3_federal_student_loan_debt_and_repayment")
        .ok_or("Pell bachelor-recipient debt table")?;
    let debt_rows = debt
        .get("amount_and_ratio_rows")
        .and_then(|value| value.as_array())
        .ok_or("Pell bachelor-recipient debt rows")?;
    let debt_yes = debt_rows
        .first()
        .ok_or("Pell bachelor-recipient debt Pell yes")?;
    let debt_no = debt_rows
        .get(1)
        .ok_or("Pell bachelor-recipient debt Pell no")?;
    let payment_rows = debt
        .get("monthly_payment_among_borrowers_in_repayment")
        .and_then(|value| value.as_array())
        .ok_or("Pell bachelor-recipient monthly payment rows")?;
    let payment_yes = payment_rows
        .first()
        .ok_or("Pell bachelor-recipient payment Pell yes")?;
    let payment_no = payment_rows
        .get(1)
        .ok_or("Pell bachelor-recipient payment Pell no")?;
    if debt_rows.len() != 2
        || payment_rows.len() != 2
        || string_field(debt, "universe")?
            != "Federal student loan borrowers among the 2015-16 bachelor's degree earners."
        || string_field(debt_yes, "group")? != "pell_yes"
        || number_field(debt_yes, "average_amount_borrowed_usd")? != 42_900.0
        || number_field(debt_yes, "average_amount_borrowed_standard_error")? != 540.0
        || number_field(debt_yes, "median_amount_borrowed_usd")? != 33_700.0
        || number_field(debt_yes, "median_amount_borrowed_standard_error")? != 400.0
        || number_field(debt_yes, "average_owed_to_borrowed_ratio_percent")? != 85.3
        || number_field(debt_yes, "average_ratio_standard_error")? != 0.62
        || number_field(debt_yes, "median_owed_to_borrowed_ratio_percent")? != 100.0
        || number_field(debt_yes, "median_ratio_standard_error")? != 0.55
        || number_field(debt_yes, "in_repayment_percent")? != 33.1
        || number_field(debt_yes, "in_repayment_standard_error")? != 0.67
        || string_field(debt_no, "group")? != "pell_no"
        || number_field(debt_no, "average_amount_borrowed_usd")? != 38_500.0
        || number_field(debt_no, "average_amount_borrowed_standard_error")? != 900.0
        || number_field(debt_no, "median_amount_borrowed_usd")? != 27_000.0
        || string_field(debt_no, "median_amount_borrowed_standard_error_status")?
            != "rounds_to_zero"
        || number_field(debt_no, "average_owed_to_borrowed_ratio_percent")? != 66.3
        || number_field(debt_no, "average_ratio_standard_error")? != 0.90
        || number_field(debt_no, "median_owed_to_borrowed_ratio_percent")? != 76.0
        || number_field(debt_no, "median_ratio_standard_error")? != 1.22
        || number_field(debt_no, "in_repayment_percent")? != 18.7
        || number_field(debt_no, "in_repayment_standard_error")? != 0.77
        || string_field(payment_yes, "group")? != "pell_yes"
        || number_field(payment_yes, "average_usd")? != 240.0
        || number_field(payment_yes, "average_standard_error")? != 6.0
        || number_field(payment_yes, "median_usd")? != 200.0
        || number_field(payment_yes, "median_standard_error")? != 8.0
        || string_field(payment_no, "group")? != "pell_no"
        || number_field(payment_no, "average_usd")? != 230.0
        || number_field(payment_no, "average_standard_error")? != 8.0
        || number_field(payment_no, "median_usd")? != 200.0
        || number_field(payment_no, "median_standard_error")? != 9.0
        || !string_field(debt, "loan_scope")?.contains("Parent PLUS Loans are excluded")
        || !string_field(debt, "repayment_note")?.contains("administrative forbearance")
        || !string_field(debt, "cost_boundary")?.contains("not Pell Grant amounts")
    {
        return Err("Pell bachelor-recipient debt boundary failed".to_string());
    }
    let job = bachelor
        .get("table_a4_most_recent_job_hours_and_pay")
        .ok_or("Pell bachelor-recipient job table")?;
    let job_yes = job
        .get("pell_yes")
        .ok_or("Pell bachelor-recipient job Pell yes")?;
    let job_no = job
        .get("pell_no")
        .ok_or("Pell bachelor-recipient job Pell no")?;
    if string_field(job, "universe")?
        != "2015-16 bachelor's degree earners who were working for pay four years after degree completion."
        || !string_field(job, "most_recent_job_definition")?
            .contains("most recent job held for at least four months")
        || number_field(job_yes, "working_full_time_percent")? != 87.3
        || number_field(job_yes, "working_full_time_standard_error")? != 0.58
        || number_field(job_yes, "average_full_time_hours_per_week")? != 41.2
        || number_field(job_yes, "full_time_hours_standard_error")? != 0.12
        || number_field(job_yes, "average_part_time_hours_per_week")? != 16.5
        || number_field(job_yes, "part_time_hours_standard_error")? != 0.32
        || number_field(job_yes, "full_time_annualized_earned_income_average_usd")? != 55_500.0
        || number_field(job_yes, "full_time_income_average_standard_error")? != 530.0
        || number_field(job_yes, "full_time_annualized_earned_income_median_usd")? != 50_000.0
        || number_field(job_yes, "full_time_income_median_standard_error")? != 410.0
        || number_field(job_yes, "part_time_annualized_earned_income_average_usd")? != 18_700.0
        || number_field(job_yes, "part_time_income_average_standard_error")? != 770.0
        || number_field(job_yes, "part_time_annualized_earned_income_median_usd")? != 15_600.0
        || number_field(job_yes, "part_time_income_median_standard_error")? != 410.0
        || number_field(job_no, "working_full_time_percent")? != 88.1
        || number_field(job_no, "working_full_time_standard_error")? != 0.56
        || number_field(job_no, "average_full_time_hours_per_week")? != 42.0
        || number_field(job_no, "full_time_hours_standard_error")? != 0.13
        || number_field(job_no, "average_part_time_hours_per_week")? != 15.1
        || number_field(job_no, "part_time_hours_standard_error")? != 0.37
        || number_field(job_no, "full_time_annualized_earned_income_average_usd")? != 64_200.0
        || number_field(job_no, "full_time_income_average_standard_error")? != 730.0
        || number_field(job_no, "full_time_annualized_earned_income_median_usd")? != 57_900.0
        || number_field(job_no, "full_time_income_median_standard_error")? != 800.0
        || number_field(job_no, "part_time_annualized_earned_income_average_usd")? != 19_900.0
        || number_field(job_no, "part_time_income_average_standard_error")? != 890.0
        || number_field(job_no, "part_time_annualized_earned_income_median_usd")? != 15_600.0
        || number_field(job_no, "part_time_income_median_standard_error")? != 670.0
        || !string_field(job, "earnings_boundary")?.contains("not earnings for the full")
    {
        return Err("Pell bachelor-recipient job boundary failed".to_string());
    }
    let well_being = bachelor
        .get("table_a6_financial_well_being")
        .ok_or("Pell bachelor-recipient financial well-being")?;
    let well_yes = well_being
        .get("pell_yes")
        .ok_or("Pell bachelor-recipient well-being Pell yes")?;
    let well_no = well_being
        .get("pell_no")
        .ok_or("Pell bachelor-recipient well-being Pell no")?;
    check_number_array(well_yes, "estimates", &[33.7, 69.0, 42.9, 14.8])?;
    check_number_array(well_yes, "standard_errors", &[0.79, 0.78, 0.80, 0.58])?;
    check_number_array(well_no, "estimates", &[27.8, 79.2, 24.0, 6.2])?;
    check_number_array(well_no, "standard_errors", &[0.66, 0.63, 0.64, 0.44])?;
    let bachelor_assessment = bachelor
        .get("evidence_assessment")
        .ok_or("Pell bachelor-recipient evidence assessment")?;
    if string_field(bachelor_assessment, "comparison_status")?
        != "descriptive_observational_pell_group_comparison"
        || string_field(bachelor_assessment, "causal_status")?
            != "not_causal_no_randomized_or_adjusted_counterfactual"
        || string_field(bachelor_assessment, "completion_effect_status")?
            != "blocked_cohort_conditioned_on_bachelor_completion"
        || !string_field(bachelor_assessment, "covid_context")?.contains("CARES Act")
        || string_field(bachelor_assessment, "cost_effectiveness_status")?
            != "blocked_no_pell_amount_or_full_incremental_cost_and_benefit_analysis"
        || string_field(bachelor_assessment, "current_fiscal_link")?
            != "blocked_no_compatible_account_outlay_or_budget_cohort"
        || string_field(bachelor_assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(bachelor_assessment, "savings_status")? != "blocked_not_scored"
    {
        return Err("Pell bachelor-recipient evidence boundary failed".to_string());
    }
    let bachelor_reader = fs::read_to_string(root.join(PELL_BACHELOR_OUTCOME_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        PELL_BACHELOR_OUTCOME_JSON_PATH,
        "ever received Pell",
        "had already completed",
        "not randomized",
        "CARES Act administrative forbearance",
        "Grant amounts, program costs",
        "cannot estimate Pell effects on persistence",
        "recoverable savings estimate",
    ] {
        if !bachelor_reader.contains(required) {
            return Err(format!("Pell bachelor-recipient reader missing {required}"));
        }
    }

    if !contains_source_id(&card, BPS_FIRST_LOOK_SOURCE_ID)
        || !contains_source_id(&card, BPS_DFD_SOURCE_ID)
        || !contains_source_id(&card, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || !contains_source_id(&card, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || !contains_source_id(&bridge, BPS_FIRST_LOOK_SOURCE_ID)
        || !contains_source_id(&bridge, BPS_DFD_SOURCE_ID)
        || !contains_source_id(&bridge, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || !contains_source_id(&bridge, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || contains_source_id(&fsa, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || contains_source_id(&fsa, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || contains_source_id(&pell, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || contains_source_id(&pell, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || contains_source_id(&bachelor, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || contains_source_id(&bachelor, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || string_field(&card, "first_time_student_longitudinal_bridge_path")?
            != BPS_FIRST_TIME_STUDENT_JSON_PATH
        || string_field(&card, "first_time_student_longitudinal_bridge_status")?
            != "early_descriptive_first_time_entrant_bridge_and_datalab_receipt_group_cross_tab_attached_noncausal_mature_cost_and_fiscal_gates_blocked"
        || string_field(&bridge, "first_time_student_longitudinal_bridge_path")?
            != BPS_FIRST_TIME_STUDENT_JSON_PATH
        || string_field(&bridge, "first_time_student_longitudinal_bridge_status")?
            != "early_descriptive_entrant_bridge_and_datalab_receipt_group_cross_tab_attached_noncausal_mature_cost_and_fiscal_gates_blocked_not_fy2025_account_outlay_or_cost_crosswalk"
        || string_field(&fsa, "first_time_student_longitudinal_bridge_path")?
            != BPS_FIRST_TIME_STUDENT_JSON_PATH
        || string_field(&fsa, "first_time_student_longitudinal_bridge_status")?
            != "early_descriptive_ay2019_20_entrant_bridge_and_datalab_receipt_group_cross_tab_attached_noncausal_mature_cost_and_fiscal_gates_blocked_not_fy2024_title_iv_recipient_or_disbursement_cohort"
        || string_field(&pell, "first_time_student_longitudinal_bridge_path")?
            != BPS_FIRST_TIME_STUDENT_JSON_PATH
        || string_field(&pell, "first_time_student_longitudinal_bridge_status")?
            != "early_descriptive_entrant_bridge_and_datalab_receipt_group_cross_tab_attached_noncausal_mature_cost_and_fiscal_gates_blocked_not_experimental_offer_population_or_counterfactual"
        || string_field(&bachelor, "first_time_student_longitudinal_bridge_path")?
            != BPS_FIRST_TIME_STUDENT_JSON_PATH
        || string_field(&bachelor, "first_time_student_longitudinal_bridge_status")?
            != "early_descriptive_first_time_entrant_bridge_and_datalab_receipt_group_cross_tab_attached_noncausal_mature_cost_and_fiscal_gates_blocked_not_bachelor_completer_postcompletion_outcomes"
        || string_field(&card, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&card, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked"
        || string_field(&bridge, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&bridge, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked_not_fy2025_account_outlay_or_cost_crosswalk"
        || string_field(&fsa, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&fsa, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked_not_fy2024_title_iv_recipient_or_disbursement_cohort"
        || string_field(&pell, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&pell, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked_not_experimental_offer_population_or_counterfactual"
        || string_field(&bachelor, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&bachelor, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked_not_bachelor_completer_postcompletion_outcomes"
        || string_field(&card, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&card, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal"
        || string_field(&bridge, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&bridge, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal_not_fy2025_account_outlay_or_cost_crosswalk"
        || string_field(&fsa, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&fsa, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal_not_fy2024_title_iv_recipient_or_disbursement_cohort"
        || string_field(&pell, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&pell, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal_not_experimental_offer_population_or_counterfactual"
        || string_field(&bachelor, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&bachelor, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal_not_bachelor_completer_postcompletion_outcomes"
    {
        return Err("BPS first-time-student longitudinal bridge link failed".to_string());
    }
    let bps_text = fs::read_to_string(root.join(BPS_FIRST_TIME_STUDENT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let bps: serde_json::Value = serde_json::from_str(&bps_text).map_err(|e| e.to_string())?;
    let bps_source_ids = bps
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("BPS first-time-student source IDs")?;
    if bps_source_ids.len() != 4
        || !contains_source_id(&bps, BPS_FIRST_LOOK_SOURCE_ID)
        || !contains_source_id(&bps, BPS_DFD_SOURCE_ID)
        || !contains_source_id(&bps, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || !contains_source_id(&bps, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || string_field(&bps, "current_entrant_pell_persistence_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(&bps, "current_entrant_pell_persistence_status")?
            != "official_datalab_entry_year_pell_receipt_by_three_year_persistence_attached_descriptive_noncausal_mature_cost_and_fiscal_gates_blocked"
        || string_field(&bps, "current_entrant_pell_significance_screen_path")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(&bps, "current_entrant_pell_significance_screen_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal"
    {
        return Err("BPS first-time-student source membership failed".to_string());
    }
    let validate_bps_custody = |source_id: &str,
                                metadata_path: &str,
                                raw_path: &str,
                                expected_bytes: u64,
                                expected_sha256: &str|
     -> Result<(), String> {
        let metadata_text =
            fs::read_to_string(root.join(metadata_path)).map_err(|e| e.to_string())?;
        for required in [
            format!("| `source_id` | `{source_id}` |"),
            format!("| `raw_path` | `{raw_path}` |"),
            format!("| `report_bytes` | `{expected_bytes}` |"),
            format!(
                "| `report_checksum_sha256` | `{}` |",
                expected_sha256.to_ascii_uppercase()
            ),
            "| `status` | `captured-checksum-verified` |".to_string(),
        ] {
            if !metadata_text.contains(&required) {
                return Err(format!("BPS source metadata missing {required}"));
            }
        }
        let raw_path = root.join(raw_path);
        if fs::metadata(&raw_path).map_err(|e| e.to_string())?.len() != expected_bytes
            || sha256_file(&raw_path)? != expected_sha256
        {
            return Err(format!("BPS source raw custody failed for {source_id}"));
        }
        Ok(())
    };
    validate_bps_custody(
        BPS_FIRST_LOOK_SOURCE_ID,
        BPS_FIRST_LOOK_METADATA_PATH,
        BPS_FIRST_LOOK_RAW_PATH,
        BPS_FIRST_LOOK_RAW_BYTES,
        BPS_FIRST_LOOK_RAW_SHA256,
    )?;
    validate_bps_custody(
        BPS_DFD_SOURCE_ID,
        BPS_DFD_METADATA_PATH,
        BPS_DFD_RAW_PATH,
        BPS_DFD_RAW_BYTES,
        BPS_DFD_RAW_SHA256,
    )?;
    let bps_cohort = bps.get("cohort").ok_or("BPS first-time-student cohort")?;
    if string_field(&bps, "record_family")? != "student_aid_longitudinal_bridge"
        || string_field(&bps, "report_number")? != "NCES 2024-401"
        || string_field(&bps, "documentation_number")? != "NCES 2026-013"
        || !string_field(bps_cohort, "target_population")?
            .contains("Title IV eligible postsecondary institution")
        || number_field(bps_cohort, "represented_population_approximate")? != 3_300_000.0
        || number_field(bps_cohort, "sample_members_rounded")? != 34_240.0
        || number_field(bps_cohort, "respondents_rounded")? != 22_320.0
        || number_field(bps_cohort, "bps_followup_response_rate_unweighted_percent")? != 65.2
        || number_field(bps_cohort, "bps_followup_response_rate_weighted_percent")? != 59.6
        || number_field(bps_cohort, "overall_response_rate_unweighted_percent")? != 52.1
        || number_field(bps_cohort, "overall_response_rate_weighted_percent")? != 49.8
        || !string_field(bps_cohort, "observation_window")?.contains("June 2022")
        || !string_field(bps_cohort, "response_rate_definition")?
            .contains("Counts are rounded to the nearest 10")
        || !string_field(bps_cohort, "pandemic_context")?.contains("COVID-19")
    {
        return Err("BPS first-time-student cohort scope failed".to_string());
    }
    let bps_weighting = bps
        .get("weighting_and_variance")
        .ok_or("BPS first-time-student weighting")?;
    let bps_weight_adjustments = bps_weighting
        .get("weight_adjustments")
        .and_then(|value| value.as_array())
        .ok_or("BPS first-time-student weight adjustments")?;
    let expected_bps_weight_adjustments = [
        "NPSAS:20 sample design and multiplicity",
        "BPS subsampling",
        "unknown eligibility",
        "nonresponse",
        "population coverage and poststratification",
    ];
    if bps_weight_adjustments.len() != expected_bps_weight_adjustments.len()
        || bps_weight_adjustments
            .iter()
            .zip(expected_bps_weight_adjustments)
            .any(|(actual, expected)| actual.as_str() != Some(expected))
        || string_field(bps_weighting, "analysis_weight")? != "WTA000"
        || string_field(bps_weighting, "replicate_weights")? != "WTA001-WTA200"
        || string_field(bps_weighting, "variance_method")?
            != "balanced_repeated_replication_with_bootstrap_replicate_weights"
        || !string_field(bps_weighting, "imputation_status")?.contains("were imputed")
        || !string_field(bps_weighting, "disclosure_status")?.contains("targeted swapping")
    {
        return Err("BPS first-time-student weighting boundary failed".to_string());
    }
    let bps_table = bps
        .get("table_a1_total_attainment_and_persistence")
        .ok_or("BPS first-time-student Table A-1")?;
    let bps_categories = bps_table
        .get("categories")
        .and_then(|value| value.as_array())
        .ok_or("BPS first-time-student Table A-1 categories")?;
    let expected_bps_categories = [
        "attained_certificate",
        "attained_associates_degree",
        "attained_bachelors_degree",
        "no_credential_enrolled_at_4_year_institution",
        "no_credential_enrolled_at_less_than_4_year_institution",
        "no_credential_not_enrolled",
    ];
    if bps_categories.len() != expected_bps_categories.len()
        || bps_categories
            .iter()
            .zip(expected_bps_categories)
            .any(|(actual, expected)| actual.as_str() != Some(expected))
        || string_field(bps_table, "unit")? != "percent"
        || string_field(bps_table, "standard_error_unit")? != "percentage_points"
        || number_field(bps_table, "sum_percent")? != 100.0
        || !string_field(bps_table, "category_boundary")?.contains("mutually exclusive")
    {
        return Err("BPS first-time-student Table A-1 category contract failed".to_string());
    }
    check_number_array(bps_table, "estimates", &[5.1, 6.8, 0.7, 47.9, 16.9, 22.6])?;
    check_number_array(
        bps_table,
        "standard_errors",
        &[0.23, 0.24, 0.07, 0.51, 0.42, 0.48],
    )?;
    let bps_estimate_sum: f64 = bps_table
        .get("estimates")
        .and_then(|value| value.as_array())
        .ok_or("BPS first-time-student Table A-1 estimates")?
        .iter()
        .map(|value| value.as_f64().ok_or("BPS Table A-1 numeric estimate"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if (bps_estimate_sum - 100.0).abs() > 0.001 {
        return Err("BPS first-time-student Table A-1 reconciliation failed".to_string());
    }
    let bps_variables = bps
        .get("documented_variable_map")
        .ok_or("BPS first-time-student variable map")?;
    for (field, expected) in [
        ("entry_pell", "PELL20"),
        ("three_year_outcome", "PROUT3_NEW"),
        ("direct_loan_borrowing", "STFCUM22"),
        ("title_iv_amount_owed_including_parent_plus", "T4TDUE22"),
        ("title_iv_amount_owed_excluding_parent_plus", "T4XDUE22"),
        ("employment", "JOBST22"),
        ("annual_salary", "SALARY22"),
    ] {
        let variable = bps_variables
            .get(field)
            .ok_or_else(|| format!("BPS first-time-student variable {field}"))?;
        if string_field(variable, "variable")? != expected {
            return Err(format!("BPS first-time-student variable {field} failed"));
        }
    }
    let bps_gate = bps
        .get("pell_outcome_gate")
        .ok_or("BPS first-time-student Pell gate")?;
    if string_field(bps_gate, "status")?
        != "captured_as_separate_official_datalab_descriptive_baseline_not_causal"
        || string_field(bps_gate, "baseline_path")? != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(bps_gate, "source_id")? != PELL_CURRENT_ENTRANT_SOURCE_ID
        || string_field(bps_gate, "retrieval_code")? != PELL_CURRENT_ENTRANT_RETRIEVAL_CODE
        || string_field(bps_gate, "contrast_status")?
            != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal"
        || !string_field(bps_gate, "published_table_status")?
            .contains("does not publish Pell-status outcome rows")
        || bps_gate.get("required_future_extract").is_some()
    {
        return Err("BPS first-time-student Pell gate failed".to_string());
    }
    let bps_assessment = bps
        .get("evidence_assessment")
        .ok_or("BPS first-time-student evidence assessment")?;
    if string_field(bps_assessment, "comparison_status")?
        != "national_descriptive_three_year_cohort_baseline"
        || string_field(bps_assessment, "causal_status")?
            != "not_causal_no_randomized_or_adjusted_counterfactual"
        || string_field(bps_assessment, "pell_effect_status")?
            != "blocked_descriptive_receipt_group_cross_tab_and_independent_estimates_ttest_screen_no_randomized_or_adjusted_causal_design"
        || string_field(bps_assessment, "group_difference_significance_status")?
            != "independent_estimates_screen_only_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked"
        || string_field(bps_assessment, "mature_completion_status")?
            != "blocked_three_year_window_many_students_still_enrolled"
        || string_field(bps_assessment, "repayment_status")?
            != "blocked_documented_restricted_nslds_histories_not_a_public_outcome_estimate"
        || string_field(bps_assessment, "earnings_status")?
            != "survey_year_three_job_salary_not_mature_post_completion_earnings"
        || string_field(bps_assessment, "cost_effectiveness_status")?
            != "blocked_no_full_incremental_program_cost_and_benefit_analysis"
        || string_field(bps_assessment, "current_fiscal_link")?
            != "blocked_no_compatible_account_outlay_or_budget_cohort"
        || string_field(bps_assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(bps_assessment, "savings_status")? != "blocked_not_scored"
        || !string_field(bps_assessment, "future_gate")?.contains("BPS:20/25")
    {
        return Err("BPS first-time-student evidence boundary failed".to_string());
    }
    let bps_reader = fs::read_to_string(root.join(BPS_FIRST_TIME_STUDENT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        BPS_FIRST_TIME_STUDENT_JSON_PATH,
        "34,240",
        "22,320",
        "47.9%",
        "0.51 percentage points",
        "PELL20",
        "PROUT3_NEW",
        "retrieval code `zclxfu`",
        "five-category",
        "not establish ineligibility",
        "covariance-aware confirmation",
        "adjusted causal design",
        "pandemic-era",
        "Restricted NSLDS repayment histories",
        "not full incremental Pell costs",
        "fraud finding",
        "recoverable savings estimate",
        "BPS:20/25",
    ] {
        if !bps_reader.contains(required) {
            return Err(format!("BPS first-time-student reader missing {required}"));
        }
    }

    let pell_current_text =
        fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_JSON_PATH)).map_err(|e| e.to_string())?;
    let pell_current: serde_json::Value =
        serde_json::from_str(&pell_current_text).map_err(|e| e.to_string())?;
    let pell_current_source_ids = pell_current
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant source IDs")?;
    if pell_current_source_ids.len() != 2
        || !contains_source_id(&pell_current, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || !contains_source_id(&pell_current, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || string_field(&pell_current, "record_family")?
            != "pell_current_entrant_persistence_baseline"
        || string_field(
            &pell_current,
            "current_entrant_pell_significance_screen_path",
        )? != PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH
        || string_field(
            &pell_current,
            "current_entrant_pell_significance_screen_status",
        )? != "official_independent_estimates_ttest_screen_attached_bonferroni_three_of_five_pass_covariance_aware_confirmation_blocked_noncausal"
    {
        return Err("Pell current-entrant source membership failed".to_string());
    }
    let pell_current_metadata = fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_METADATA_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        format!("| `source_id` | `{PELL_CURRENT_ENTRANT_SOURCE_ID}` |"),
        format!("| `query_id` | `{PELL_CURRENT_ENTRANT_QUERY_ID}` |"),
        format!("| `retrieval_code` | `{PELL_CURRENT_ENTRANT_RETRIEVAL_CODE}` |"),
        format!("| `raw_path` | `{PELL_CURRENT_ENTRANT_RAW_PATH}` |"),
        format!("| `report_bytes` | `{PELL_CURRENT_ENTRANT_RAW_BYTES}` |"),
        format!(
            "| `report_checksum_sha256` | `{}` |",
            PELL_CURRENT_ENTRANT_RAW_SHA256.to_ascii_uppercase()
        ),
        "| `source_url` | <https://nces.ed.gov/datalab/powerstats/table/zclxfu> |".to_string(),
        "| `api_url` | <https://nces.ed.gov/datalab/api/v1/workspace/retrieve/zclxfu> |"
            .to_string(),
        "| `status` | `captured-checksum-verified` |".to_string(),
    ] {
        if !pell_current_metadata.contains(&required) {
            return Err(format!("Pell current-entrant metadata missing {required}"));
        }
    }
    let pell_current_raw_path = root.join(PELL_CURRENT_ENTRANT_RAW_PATH);
    if fs::metadata(&pell_current_raw_path)
        .map_err(|e| e.to_string())?
        .len()
        != PELL_CURRENT_ENTRANT_RAW_BYTES
        || sha256_file(&pell_current_raw_path)? != PELL_CURRENT_ENTRANT_RAW_SHA256
    {
        return Err("Pell current-entrant raw custody failed".to_string());
    }
    let pell_current_dataset = pell_current
        .get("dataset")
        .ok_or("Pell current-entrant dataset")?;
    let pell_current_retrieval = pell_current
        .get("retrieval")
        .ok_or("Pell current-entrant retrieval")?;
    if number_field(pell_current_dataset, "dataset_id")? != 168.0
        || string_field(pell_current_dataset, "dataset_abbreviation")? != "BPS2022"
        || number_field(pell_current_dataset, "collection_period_id")? != 44.0
        || string_field(pell_current_dataset, "collection_period")? != "2020/2022"
        || string_field(pell_current_dataset, "universe")? != "All respondents."
        || !string_field(pell_current_dataset, "observation_window")?.contains("June 2022")
        || !string_field(pell_current_dataset, "pandemic_context")?.contains("COVID-19")
        || int_field(pell_current_retrieval, "query_id")? != PELL_CURRENT_ENTRANT_QUERY_ID
        || string_field(pell_current_retrieval, "retrieval_code")?
            != PELL_CURRENT_ENTRANT_RETRIEVAL_CODE
        || string_field(pell_current_retrieval, "raw_path")? != PELL_CURRENT_ENTRANT_RAW_PATH
        || number_field(pell_current_retrieval, "raw_bytes")?
            != PELL_CURRENT_ENTRANT_RAW_BYTES as f64
        || string_field(pell_current_retrieval, "raw_sha256")?.to_ascii_lowercase()
            != PELL_CURRENT_ENTRANT_RAW_SHA256
    {
        return Err("Pell current-entrant dataset or retrieval identity failed".to_string());
    }
    let pell_current_spec = pell_current
        .get("analysis_specification")
        .ok_or("Pell current-entrant analysis specification")?;
    let pell_current_row = pell_current_spec
        .get("row")
        .ok_or("Pell current-entrant row specification")?;
    let pell_current_column = pell_current_spec
        .get("column")
        .ok_or("Pell current-entrant column specification")?;
    let pell_current_row_groups = pell_current_row
        .get("groups")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant row groups")?;
    let pell_current_column_values = pell_current_column
        .get("values")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant column values")?;
    if number_field(pell_current_spec, "major_version")? != 1.0
        || number_field(pell_current_spec, "minor_version")? != 0.0
        || string_field(pell_current_spec, "application_type")? != "PowerStats"
        || string_field(pell_current_spec, "analysis_type")? != "PercentageDistribution"
        || number_field(pell_current_spec, "dataset_id")? != 168.0
        || string_field(pell_current_spec, "weight")? != "WTA000"
        || string_field(pell_current_spec, "variance_method")? != "BRR"
        || number_field(pell_current_spec, "processing_type")? != 3.0
        || pell_current_spec
            .get("filters")
            .and_then(|value| value.as_array())
            .is_none_or(|values| !values.is_empty())
        || !pell_current_spec
            .get("subtable")
            .is_some_and(serde_json::Value::is_null)
        || string_field(pell_current_row, "variable")? != "PELL20"
        || string_field(pell_current_row, "type")? != "Continuous"
        || string_field(pell_current_row, "processing_type")? != "Cut"
        || pell_current_row_groups.len() != 2
        || number_field(&pell_current_row_groups[0], "start_value")? != 0.0
        || number_field(&pell_current_row_groups[0], "end_value")? != 0.0
        || number_field(&pell_current_row_groups[1], "start_value")? != 1.0
        || number_field(&pell_current_row_groups[1], "end_value")? != 9_293.0
        || !string_field(pell_current_row, "receipt_boundary")?
            .contains("does not establish ineligibility")
        || string_field(pell_current_column, "variable")? != "PROUT3_NEW"
        || string_field(pell_current_column, "type")? != "Categorical"
        || string_field(pell_current_column, "processing_type")? != "Category"
        || pell_current_column_values.len() != 5
        || pell_current_column_values
            .iter()
            .zip([1, 2, 3, 4, 5])
            .any(|(actual, expected)| actual.as_i64() != Some(expected))
        || pell_current_spec
            .get("suppression_symbols")
            .and_then(|value| value.as_array())
            .is_none_or(|values| !values.is_empty())
        || string_field(pell_current_spec, "suppression_status")? != "no_cells_suppressed"
    {
        return Err("Pell current-entrant exact analysis specification failed".to_string());
    }
    let pell_current_categories = pell_current
        .get("outcome_categories")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant categories")?;
    let expected_pell_current_categories = [
        ("attained_bachelors_degree", 1, "Attained bachelor's degree"),
        (
            "attained_associates_degree",
            2,
            "Attained associate's degree",
        ),
        ("attained_certificate", 3, "Attained certificate"),
        (
            "no_degree_enrolled_ay2021_22",
            4,
            "No degree, enrolled in 2021-22 AY",
        ),
        (
            "no_degree_not_enrolled_ay2021_22",
            5,
            "No degree, not enrolled in 2021-22 AY",
        ),
    ];
    if pell_current_categories.len() != expected_pell_current_categories.len()
        || pell_current_categories
            .iter()
            .zip(expected_pell_current_categories)
            .any(|(actual, expected)| {
                actual.get("id").and_then(|value| value.as_str()) != Some(expected.0)
                    || actual.get("value").and_then(|value| value.as_i64()) != Some(expected.1)
                    || actual.get("label").and_then(|value| value.as_str()) != Some(expected.2)
            })
    {
        return Err("Pell current-entrant exact category contract failed".to_string());
    }
    let pell_current_category_boundary = pell_current
        .get("category_boundary")
        .ok_or("Pell current-entrant category boundary")?;
    if string_field(pell_current_category_boundary, "status")?
        != "five_category_prout3_new_not_six_category_table_a1"
        || !string_field(pell_current_category_boundary, "description")?.contains("does not divide")
        || !string_field(pell_current_category_boundary, "not_permanent_dropout")?
            .contains("not a permanent-dropout")
    {
        return Err("Pell current-entrant five-category boundary failed".to_string());
    }
    let pell_current_results = pell_current
        .get("results")
        .ok_or("Pell current-entrant results")?;
    let pell_current_result_identity = pell_current_results
        .get("result_identity")
        .ok_or("Pell current-entrant result identity")?;
    if string_field(pell_current_results, "unit")? != "percent"
        || string_field(pell_current_results, "standard_error_unit")? != "percentage_points"
        || string_field(pell_current_results, "count_unit")? != "weighted_students"
        || string_field(pell_current_result_identity, "row_id")? != "PELL20"
        || string_field(pell_current_result_identity, "row_identifier")? != "pell20_receipt_binary"
        || string_field(pell_current_result_identity, "column_id")? != "PROUT3_NEW"
        || string_field(pell_current_result_identity, "column_identifier")? != "prout3_new_outcome"
        || string_field(pell_current_result_identity, "year")? != "2020/2022"
        || pell_current_result_identity
            .get("datasets")
            .and_then(|value| value.as_array())
            .is_none_or(|values| values.len() != 1 || values[0].as_i64() != Some(168))
    {
        return Err("Pell current-entrant result units failed".to_string());
    }
    let check_pell_current_array =
        |object: &serde_json::Value, field: &str, expected: &[f64]| -> Result<(), String> {
            let values = object
                .get(field)
                .and_then(|value| value.as_array())
                .ok_or_else(|| format!("Pell current-entrant {field}"))?;
            if values.len() != expected.len()
                || values
                    .iter()
                    .zip(expected)
                    .any(|(value, expected)| value.as_f64() != Some(*expected))
            {
                return Err(format!("Pell current-entrant {field} failed"));
            }
            Ok(())
        };
    let pell_zero = pell_current_results
        .get("pell_amount_zero")
        .ok_or("Pell current-entrant zero group")?;
    let pell_positive = pell_current_results
        .get("pell_amount_positive")
        .ok_or("Pell current-entrant positive group")?;
    let pell_total = pell_current_results
        .get("total")
        .ok_or("Pell current-entrant total group")?;
    for (group, arrays, weighted_total) in [
        (
            pell_zero,
            [
                &[0.8150894, 7.0807806, 3.4187823, 70.4424349, 18.2429128][..],
                &[0.1008173, 0.3414563, 0.2497779, 0.6186631, 0.5892492][..],
                &[12.3688601, 4.8222975, 7.3060493, 0.8782535, 3.2300171][..],
                &[14_532.0, 126_241.0, 60_952.0, 1_255_894.0, 325_247.0][..],
                &[0.6174875, 6.4115262, 2.9292176, 69.2298552, 17.0879844][..],
                &[1.0126912, 7.7500349, 3.908347, 71.6550147, 19.3978412][..],
            ],
            1_782_866.0,
        ),
        (
            pell_positive,
            [
                &[0.4971528, 6.4224743, 7.1063823, 58.0382947, 27.935696][..],
                &[0.0834908, 0.3544924, 0.3445115, 0.7774679, 0.7039664][..],
                &[16.7937819, 5.5195613, 4.8479174, 1.3395775, 2.519953][..],
                &[7_301.0, 94_312.0, 104_355.0, 852_278.0, 410_229.0][..],
                &[0.3335109, 5.7276691, 6.4311396, 56.5144576, 26.5559218][..],
                &[0.6607947, 7.1172794, 7.7816249, 59.5621318, 29.3154701][..],
            ],
            1_468_475.0,
        ),
        (
            pell_total,
            [
                &[0.6714926, 6.7834551, 5.0842948, 64.8400782, 22.6206793][..],
                &[0.069599, 0.2427381, 0.2258418, 0.547403, 0.4797255][..],
                &[10.3648189, 3.5783843, 4.4419492, 0.8442355, 2.1207387][..],
                &[21_833.0, 220_553.0, 165_308.0, 2_108_172.0, 735_475.0][..],
                &[0.5350786, 6.3076885, 4.6416449, 63.7671683, 21.6804173][..],
                &[0.8079067, 7.2592218, 5.5269447, 65.912988, 23.5609413][..],
            ],
            3_251_341.0,
        ),
    ] {
        for (field, expected) in [
            ("estimates", arrays[0]),
            ("standard_errors", arrays[1]),
            ("relative_standard_errors_percent", arrays[2]),
            ("weighted_counts", arrays[3]),
            ("lower_confidence_bounds", arrays[4]),
            ("upper_confidence_bounds", arrays[5]),
        ] {
            check_pell_current_array(group, field, expected)?;
        }
        let estimate_sum: f64 = group
            .get("estimates")
            .and_then(|value| value.as_array())
            .ok_or("Pell current-entrant estimate reconciliation")?
            .iter()
            .map(|value| value.as_f64().ok_or("Pell current-entrant estimate"))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let count_sum: f64 = group
            .get("weighted_counts")
            .and_then(|value| value.as_array())
            .ok_or("Pell current-entrant count reconciliation")?
            .iter()
            .map(|value| value.as_f64().ok_or("Pell current-entrant count"))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        if (estimate_sum - 100.0).abs() > 0.000_001
            || count_sum != weighted_total
            || number_field(group, "weighted_total")? != weighted_total
        {
            return Err("Pell current-entrant result reconciliation failed".to_string());
        }
    }
    let pell_current_assessment = pell_current
        .get("evidence_assessment")
        .ok_or("Pell current-entrant evidence assessment")?;
    if string_field(pell_current_assessment, "comparison_status")?
        != "official_national_descriptive_weighted_receipt_group_cross_tab"
        || string_field(pell_current_assessment, "causal_status")?
            != "not_causal_observational_unadjusted_receipt_group_comparison"
        || !string_field(pell_current_assessment, "pell_effect_status")?.contains("not_identified")
        || !string_field(pell_current_assessment, "eligibility_status")?.contains("not_measured")
        || !string_field(pell_current_assessment, "mature_completion_status")?
            .contains("early_three_year")
        || !string_field(pell_current_assessment, "pandemic_status")?.contains("pandemic")
        || !string_field(pell_current_assessment, "cost_effectiveness_status")?.contains("blocked")
        || !string_field(pell_current_assessment, "current_fiscal_link")?.contains("blocked")
        || string_field(pell_current_assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(pell_current_assessment, "savings_status")? != "blocked_not_scored"
        || !string_field(pell_current_assessment, "future_gate")?.contains("BPS:20/25")
    {
        return Err("Pell current-entrant evidence boundary failed".to_string());
    }

    let pell_current_raw_text =
        fs::read_to_string(&pell_current_raw_path).map_err(|e| e.to_string())?;
    let pell_current_raw: serde_json::Value =
        serde_json::from_str(&pell_current_raw_text).map_err(|e| e.to_string())?;
    if pell_current_raw
        .get("success")
        .and_then(|value| value.as_bool())
        != Some(true)
        || int_field(&pell_current_raw, "status")? != 200
    {
        return Err("Pell current-entrant raw response envelope failed".to_string());
    }
    let pell_current_raw_outer = pell_current_raw
        .get("result")
        .ok_or("Pell current-entrant raw result")?;
    if int_field(pell_current_raw_outer, "queryId")? != PELL_CURRENT_ENTRANT_QUERY_ID
        || pell_current_raw_outer
            .get("isSaved")
            .and_then(|value| value.as_bool())
            != Some(false)
    {
        return Err("Pell current-entrant raw query identity failed".to_string());
    }
    let raw_specification_text = string_field(pell_current_raw_outer, "specification")?;
    let raw_specification: serde_json::Value =
        serde_json::from_str(&raw_specification_text).map_err(|e| e.to_string())?;
    let raw_rows = raw_specification
        .get("_rows")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant raw rows")?;
    let raw_columns = raw_specification
        .get("_columns")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant raw columns")?;
    let raw_row_elements = raw_rows
        .first()
        .and_then(|value| value.get("elements"))
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant raw row elements")?;
    let raw_column_elements = raw_columns
        .first()
        .and_then(|value| value.get("elements"))
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant raw column elements")?;
    if string_field(&raw_specification, "_applicationType")? != "PowerStats"
        || string_field(&raw_specification, "_analysisType")? != "PercentageDistribution"
        || number_field(&raw_specification, "_dataset")? != 168.0
        || string_field(&raw_specification, "_weight")? != "WTA000"
        || raw_rows.len() != 1
        || raw_columns.len() != 1
        || string_field(&raw_rows[0], "id")? != "PELL20"
        || string_field(&raw_rows[0], "processingType")? != "Cut"
        || raw_row_elements.len() != 2
        || number_field(&raw_row_elements[0], "startValue")? != 0.0
        || number_field(&raw_row_elements[0], "endValue")? != 0.0
        || number_field(&raw_row_elements[1], "startValue")? != 1.0
        || number_field(&raw_row_elements[1], "endValue")? != 9_293.0
        || string_field(&raw_columns[0], "id")? != "PROUT3_NEW"
        || string_field(&raw_columns[0], "processingType")? != "Category"
        || raw_column_elements.len() != 5
        || raw_column_elements
            .iter()
            .zip([1, 2, 3, 4, 5])
            .any(|(element, expected)| {
                element
                    .get("values")
                    .and_then(|value| value.as_array())
                    .is_none_or(|values| values.len() != 1 || values[0].as_i64() != Some(expected))
            })
        || raw_specification
            .get("_filters")
            .and_then(|value| value.as_array())
            .is_none_or(|values| !values.is_empty())
        || !raw_specification
            .get("_subtable")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("Pell current-entrant raw specification failed".to_string());
    }
    let raw_result_set_text = string_field(pell_current_raw_outer, "resultSet")?;
    let raw_result_set: serde_json::Value =
        serde_json::from_str(&raw_result_set_text).map_err(|e| e.to_string())?;
    let raw_result = raw_result_set
        .as_array()
        .and_then(|values| values.first())
        .and_then(|value| value.as_array())
        .and_then(|values| values.first())
        .ok_or("Pell current-entrant raw result set")?;
    let raw_year = raw_result
        .get("years")
        .and_then(|value| value.as_array())
        .and_then(|values| values.first())
        .ok_or("Pell current-entrant raw year")?;
    let raw_data = raw_year
        .get("data")
        .ok_or("Pell current-entrant raw data")?;
    let raw_datasets = raw_result
        .get("datasets")
        .and_then(|value| value.as_array())
        .ok_or("Pell current-entrant raw datasets")?;
    if string_field(raw_result, "rowId")? != "PELL20"
        || string_field(raw_result, "rowIdentifier")? != "pell20_receipt_binary"
        || string_field(raw_result, "columnId")? != "PROUT3_NEW"
        || string_field(raw_result, "columnIdentifier")? != "prout3_new_outcome"
        || raw_datasets.len() != 1
        || raw_datasets[0].as_i64() != Some(168)
        || string_field(raw_year, "year")? != "2020/2022"
        || string_field(raw_data, "standardErrorType")? != "BRR"
        || string_field(raw_data, "suppressionSymbols")? != "[]"
    {
        return Err("Pell current-entrant raw result identity failed".to_string());
    }
    let raw_category_keys = [
        "PROUT3_NEW_Attained bachelor's degree",
        "PROUT3_NEW_Attained associate's degree",
        "PROUT3_NEW_Attained certificate",
        "PROUT3_NEW_No degree, enrolled in 2021–22 AY",
        "PROUT3_NEW_No degree, not enrolled in 2021–22 AY",
    ];
    for (derived_group, raw_group) in [
        (pell_zero, "PELL20_No Pell received (amount = $0)"),
        (pell_positive, "PELL20_Pell received (amount > $0)"),
        (pell_total, "PROUT3_NEW_Total"),
    ] {
        for (raw_field, derived_field) in [
            ("estimate", "estimates"),
            ("standardError", "standard_errors"),
            ("relativeStandardError", "relative_standard_errors_percent"),
            ("lowerConfidenceInterval", "lower_confidence_bounds"),
            ("upperConfidenceInterval", "upper_confidence_bounds"),
        ] {
            let raw_group_values = raw_data
                .get(raw_field)
                .and_then(|value| value.get(raw_group))
                .ok_or_else(|| format!("Pell current-entrant raw {raw_field} {raw_group}"))?;
            let derived_values = derived_group
                .get(derived_field)
                .and_then(|value| value.as_array())
                .ok_or_else(|| format!("Pell current-entrant derived {derived_field}"))?;
            if raw_category_keys
                .iter()
                .zip(derived_values)
                .any(|(key, derived)| {
                    raw_group_values.get(key).and_then(|value| value.as_f64()) != derived.as_f64()
                })
            {
                return Err(format!(
                    "Pell current-entrant raw reconciliation failed for {derived_field} {raw_group}"
                ));
            }
        }
        let derived_counts = derived_group
            .get("weighted_counts")
            .and_then(|value| value.as_array())
            .ok_or("Pell current-entrant derived weighted counts")?;
        if raw_category_keys
            .iter()
            .zip(derived_counts)
            .any(|(key, derived)| {
                raw_data
                    .get("count")
                    .and_then(|value| value.get(key))
                    .and_then(|value| value.get(raw_group))
                    .and_then(|value| value.as_f64())
                    != derived.as_f64()
            })
            || raw_data
                .get("count")
                .and_then(|value| value.get("PROUT3_NEW_Total"))
                .and_then(|value| value.get(raw_group))
                .and_then(|value| value.as_f64())
                != derived_group
                    .get("weighted_total")
                    .and_then(|value| value.as_f64())
        {
            return Err(format!(
                "Pell current-entrant raw count reconciliation failed for {raw_group}"
            ));
        }
    }
    let pell_current_reader = fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        PELL_CURRENT_ENTRANT_JSON_PATH,
        PELL_CURRENT_ENTRANT_RAW_PATH,
        "query `396385`",
        "retrieval code `zclxfu`",
        "all respondents",
        "WTA000",
        "balanced repeated replication",
        "0.8150894%",
        "7.1063823%",
        "1,782,866",
        "3,251,341",
        "five categories",
        "does **not** reproduce",
        "student was ineligible",
        "adjusted causal design",
        "COVID-19",
        "not a mature completion baseline",
        "fiscal return",
        "fraud finding",
        "budget saving",
        PELL_CURRENT_ENTRANT_RAW_SHA256
            .to_ascii_uppercase()
            .as_str(),
    ] {
        if !pell_current_reader.contains(required) {
            return Err(format!("Pell current-entrant reader missing {required}"));
        }
    }

    let significance_text =
        fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let significance: serde_json::Value =
        serde_json::from_str(&significance_text).map_err(|e| e.to_string())?;
    let significance_sources = significance
        .get("source_ids")
        .and_then(|value| value.as_array())
        .ok_or("Pell significance source IDs")?;
    let significance_dataset = significance
        .get("dataset")
        .ok_or("Pell significance dataset")?;
    let significance_source_table = significance
        .get("source_table")
        .ok_or("Pell significance source table")?;
    if significance_sources.len() != 2
        || !contains_source_id(&significance, PELL_CURRENT_ENTRANT_SOURCE_ID)
        || !contains_source_id(&significance, PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID)
        || string_field(&significance, "record_family")?
            != "pell_current_entrant_persistence_significance_screen"
        || string_field(&significance, "status")?
            != "draft-official-datalab-independent-estimates-screen-checksum-verified"
        || number_field(significance_dataset, "dataset_id")? != 168.0
        || string_field(significance_dataset, "dataset_abbreviation")? != "BPS2022"
        || string_field(significance_dataset, "collection_period")? != "2020/2022"
        || string_field(significance_dataset, "universe")? != "All respondents."
        || string_field(significance_dataset, "weight")? != "WTA000"
        || string_field(significance_dataset, "source_variance_method")? != "BRR"
        || !string_field(significance_dataset, "pandemic_context")?.contains("COVID-19")
        || int_field(significance_source_table, "query_id")? != PELL_CURRENT_ENTRANT_QUERY_ID
        || string_field(significance_source_table, "retrieval_code")?
            != PELL_CURRENT_ENTRANT_RETRIEVAL_CODE
        || string_field(significance_source_table, "baseline_path")?
            != PELL_CURRENT_ENTRANT_JSON_PATH
        || string_field(significance_source_table, "raw_path")? != PELL_CURRENT_ENTRANT_RAW_PATH
        || number_field(significance_source_table, "raw_bytes")?
            != PELL_CURRENT_ENTRANT_RAW_BYTES as f64
        || string_field(significance_source_table, "raw_sha256")?.to_ascii_lowercase()
            != PELL_CURRENT_ENTRANT_RAW_SHA256
    {
        return Err("Pell significance source identity failed".to_string());
    }

    let significance_method = significance
        .get("test_method")
        .ok_or("Pell significance test method")?;
    let significance_multiplicity = significance
        .get("multiple_comparison_control")
        .ok_or("Pell significance multiplicity")?;
    if string_field(significance_method, "name")? != "NCES DataLab Independent Estimates t-Test"
        || string_field(significance_method, "difference_order")?
            != "pell_amount_zero_minus_pell_amount_positive"
        || string_field(significance_method, "difference_unit")? != "percentage_points"
        || !string_field(significance_method, "formula")?.contains("sqrt(se_no_pell^2 + se_pell^2)")
        || string_field(significance_method, "test_sidedness")? != "two_sided"
        || number_field(significance_method, "nominal_alpha")? != 0.05
        || !string_field(significance_method, "independence_boundary")?
            .contains("only for independent groups")
        || string_field(significance_method, "covariance_status")?
            != "not_covariance_aware_not_replicate_weight_difference_estimation"
        || string_field(significance_multiplicity, "method")? != "Bonferroni"
        || number_field(significance_multiplicity, "familywise_alpha")? != 0.05
        || number_field(significance_multiplicity, "comparison_count")? != 5.0
        || number_field(significance_multiplicity, "per_comparison_alpha")? != 0.01
        || !string_field(significance_multiplicity, "boundary")?
            .contains("not a value returned by DataLab")
    {
        return Err("Pell significance method boundary failed".to_string());
    }

    let significance_metadata =
        fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_SIGNIFICANCE_METADATA_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID,
        PELL_CURRENT_ENTRANT_SIGNIFICANCE_RAW_DIR,
        "`request-manifest.json` | 3468",
        "`396385`",
        "`zclxfu`",
        "`168`",
        "`WTA000`",
        "`comparison_count` | `5`",
        "`captured-checksum-verified-independent-estimates-screen`",
        "independent",
        "covariance",
        PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_SHA256
            .to_ascii_uppercase()
            .as_str(),
    ] {
        if !significance_metadata.contains(required) {
            return Err(format!("Pell significance metadata missing {required}"));
        }
    }

    let significance_raw_custody = significance
        .get("raw_custody")
        .ok_or("Pell significance raw custody")?;
    if string_field(significance_raw_custody, "directory")?
        != PELL_CURRENT_ENTRANT_SIGNIFICANCE_RAW_DIR
    {
        return Err("Pell significance raw directory failed".to_string());
    }
    let significance_manifest_record = significance_raw_custody
        .get("request_manifest")
        .ok_or("Pell significance request manifest record")?;
    if string_field(significance_manifest_record, "path")?
        != PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_PATH
        || number_field(significance_manifest_record, "bytes")?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_BYTES as f64
        || string_field(significance_manifest_record, "sha256")?.to_ascii_lowercase()
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_SHA256
    {
        return Err("Pell significance manifest record failed".to_string());
    }
    let significance_manifest_path = root.join(PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_PATH);
    if fs::metadata(&significance_manifest_path)
        .map_err(|e| e.to_string())?
        .len()
        != PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_BYTES
        || sha256_file(&significance_manifest_path)?
            != PELL_CURRENT_ENTRANT_SIGNIFICANCE_MANIFEST_SHA256
    {
        return Err("Pell significance request-manifest custody failed".to_string());
    }
    let raw_manifest_text =
        fs::read_to_string(&significance_manifest_path).map_err(|e| e.to_string())?;
    let raw_manifest: serde_json::Value =
        serde_json::from_str(&raw_manifest_text).map_err(|e| e.to_string())?;
    let raw_requests = raw_manifest
        .get("requests")
        .and_then(|value| value.as_array())
        .ok_or("Pell significance raw requests")?;
    if string_field(&raw_manifest, "source_id")? != PELL_CURRENT_ENTRANT_SIGNIFICANCE_SOURCE_ID
        || number_field(&raw_manifest, "dataset_id")? != 168.0
        || string_field(&raw_manifest, "weight")? != "WTA000"
        || int_field(&raw_manifest, "source_query_id")? != PELL_CURRENT_ENTRANT_QUERY_ID
        || string_field(&raw_manifest, "source_retrieval_code")?
            != PELL_CURRENT_ENTRANT_RETRIEVAL_CODE
        || raw_requests.len() != 5
        || !string_field(
            raw_manifest
                .get("method")
                .ok_or("Pell significance raw method")?,
            "covariance",
        )?
        .contains("No covariance term")
    {
        return Err("Pell significance raw manifest identity failed".to_string());
    }

    let significance_results = significance
        .get("results")
        .and_then(|value| value.as_array())
        .ok_or("Pell significance results")?;
    let significance_responses = significance_raw_custody
        .get("responses")
        .and_then(|value| value.as_array())
        .ok_or("Pell significance response custody")?;
    let expected_significance_results = [
        (
            "attained_bachelors_degree",
            0.8150894,
            0.4971528,
            0.1008173,
            0.0834908,
            0.3179366,
            2.428848787151347,
            0.016030784667445643,
            0.08015392333722821,
            false,
            "pvalue-attained-bachelors-degree.json",
            77_u64,
            "2879ec8b8e28caeb36feb15125f152a10020a9cf8aafd441a0a54cea071aab88",
        ),
        (
            "attained_associates_degree",
            7.0807806,
            6.4224743,
            0.3414563,
            0.3544924,
            0.6583063,
            1.337487109537279,
            0.1825827005618011,
            0.9129135028090055,
            false,
            "pvalue-attained-associates-degree.json",
            75_u64,
            "6bda131e950fe31623fa034fdd81bfd99deff76d02552cabf3e25f6c0d8f0e53",
        ),
        (
            "attained_certificate",
            3.4187823,
            7.1063823,
            0.2497779,
            0.3445115,
            -3.6876,
            -8.665865686216897,
            1.5192919907382282e-15,
            7.596459953691141e-15,
            true,
            "pvalue-attained-certificate.json",
            79_u64,
            "4bd0676e5c74ba112f7108a688b75eed4a7b5aa87416c400c677a72bbf22eba2",
        ),
        (
            "no_degree_enrolled_ay2021_22",
            70.4424349,
            58.0382947,
            0.6186631,
            0.7774679,
            12.4041402,
            12.484294709430277,
            7.994894151222828e-27,
            3.997447075611414e-26,
            true,
            "pvalue-no-degree-enrolled-ay2021-22.json",
            78_u64,
            "317b326ed9ed859b02b96d68b2d6e4a3cc4890d5c569aa6b157cd7de30769111",
        ),
        (
            "no_degree_not_enrolled_ay2021_22",
            18.2429128,
            27.935696,
            0.5892492,
            0.7039664,
            -9.6927832,
            -10.558215819775572,
            5.3799655359236484e-21,
            2.6899827679618242e-20,
            true,
            "pvalue-no-degree-not-enrolled-ay2021-22.json",
            79_u64,
            "16f76fd195f10c4a504a0cf3225a21d8d162ab013e3c3a8e59345cdb7f1c1d95",
        ),
    ];
    if significance_results.len() != expected_significance_results.len()
        || significance_responses.len() != expected_significance_results.len()
    {
        return Err("Pell significance result count failed".to_string());
    }
    for (((result, response), request), expected) in significance_results
        .iter()
        .zip(significance_responses)
        .zip(raw_requests)
        .zip(expected_significance_results)
    {
        let (
            outcome_id,
            estimate_zero,
            estimate_positive,
            se_zero,
            se_positive,
            difference,
            t_value,
            p_value,
            adjusted_p_value,
            adjusted_pass,
            response_name,
            response_bytes,
            response_sha256,
        ) = expected;
        let expected_response_path =
            format!("{PELL_CURRENT_ENTRANT_SIGNIFICANCE_RAW_DIR}/{response_name}");
        let expected_label = match outcome_id {
            "attained_bachelors_degree" => "Attained bachelor's degree",
            "attained_associates_degree" => "Attained associate's degree",
            "attained_certificate" => "Attained certificate",
            "no_degree_enrolled_ay2021_22" => "No degree, enrolled in 2021-22 AY",
            "no_degree_not_enrolled_ay2021_22" => "No degree, not enrolled in 2021-22 AY",
            _ => return Err(format!("unexpected Pell significance outcome {outcome_id}")),
        };
        let recomputed_t = (estimate_zero - estimate_positive)
            / f64::sqrt(se_zero * se_zero + se_positive * se_positive);
        let close = |actual: f64, expected_value: f64| {
            (actual - expected_value).abs()
                <= f64::EPSILON * expected_value.abs().max(f64::MIN_POSITIVE) * 4.0
        };
        for (field, actual, expected_value) in [
            (
                "estimate_no_pell_percent",
                number_field(result, "estimate_no_pell_percent")?,
                estimate_zero,
            ),
            (
                "estimate_pell_percent",
                number_field(result, "estimate_pell_percent")?,
                estimate_positive,
            ),
            (
                "standard_error_no_pell_percentage_points",
                number_field(result, "standard_error_no_pell_percentage_points")?,
                se_zero,
            ),
            (
                "standard_error_pell_percentage_points",
                number_field(result, "standard_error_pell_percentage_points")?,
                se_positive,
            ),
            (
                "difference_no_pell_minus_pell_percentage_points",
                number_field(result, "difference_no_pell_minus_pell_percentage_points")?,
                difference,
            ),
            ("t_value", number_field(result, "t_value")?, t_value),
        ] {
            if actual != expected_value {
                return Err(format!(
                    "Pell significance {outcome_id} {field} mismatch: {actual} vs {expected_value}"
                ));
            }
        }
        if (recomputed_t - t_value).abs() > 1e-14 {
            return Err(format!(
                "Pell significance {outcome_id} recomputed t mismatch: {recomputed_t} vs {t_value}"
            ));
        }
        if string_field(result, "outcome_id")? != outcome_id
            || string_field(result, "label")? != expected_label
            || string_field(response, "outcome_id")? != outcome_id
            || string_field(request, "outcome_id")? != outcome_id
            || number_field(result, "estimate_no_pell_percent")? != estimate_zero
            || number_field(result, "estimate_pell_percent")? != estimate_positive
            || number_field(result, "standard_error_no_pell_percentage_points")? != se_zero
            || number_field(result, "standard_error_pell_percentage_points")? != se_positive
            || number_field(result, "difference_no_pell_minus_pell_percentage_points")?
                != difference
            || (number_field(result, "t_value")? - t_value).abs() > 1e-14
            || (recomputed_t - t_value).abs() > 1e-14
            || !close(number_field(result, "p_value")?, p_value)
            || !close(
                number_field(result, "bonferroni_adjusted_p_value")?,
                adjusted_p_value,
            )
            || (adjusted_p_value - (p_value * 5.0_f64).min(1.0)).abs() > 1e-15
            || string_field(response, "path")? != expected_response_path
            || number_field(response, "bytes")? != response_bytes as f64
            || string_field(response, "sha256")?.to_ascii_lowercase() != response_sha256
            || string_field(request, "response_path")? != response_name
            || number_field(request, "estimate_no_pell")? != estimate_zero
            || number_field(request, "estimate_pell")? != estimate_positive
            || number_field(request, "standard_error_no_pell")? != se_zero
            || number_field(request, "standard_error_pell")? != se_positive
            || number_field(request, "difference_percentage_points")? != difference
            || (number_field(request, "t_value")? - t_value).abs() > 1e-14
            || !string_field(request, "url")?.ends_with(&format!("tValue={t_value}"))
        {
            return Err(format!(
                "Pell significance result reconciliation failed for {outcome_id}"
            ));
        }
        let expected_nominal = if p_value < 0.05 {
            "statistically_significant_under_independent_estimates_screen"
        } else {
            "not_statistically_significant_under_independent_estimates_screen"
        };
        let expected_adjusted = if adjusted_pass {
            "statistically_significant_after_five_comparison_adjustment"
        } else {
            "not_statistically_significant_after_five_comparison_adjustment"
        };
        if string_field(result, "nominal_alpha_0_05")? != expected_nominal
            || string_field(result, "bonferroni_familywise_alpha_0_05")? != expected_adjusted
        {
            return Err(format!(
                "Pell significance threshold status failed for {outcome_id}"
            ));
        }
        let metadata_custody_row = format!(
            "`{response_name}` | {response_bytes} | `{}`",
            response_sha256.to_ascii_uppercase()
        );
        if !significance_metadata.contains(&metadata_custody_row) {
            return Err(format!(
                "Pell significance metadata custody missing {response_name}"
            ));
        }
        let response_path = root.join(&expected_response_path);
        if fs::metadata(&response_path)
            .map_err(|e| e.to_string())?
            .len()
            != response_bytes
            || sha256_file(&response_path)? != response_sha256
        {
            return Err(format!(
                "Pell significance raw response custody failed for {outcome_id}"
            ));
        }
        let response_text = fs::read_to_string(response_path).map_err(|e| e.to_string())?;
        let response_json: serde_json::Value =
            serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
        let response_p_value = string_field(&response_json, "result")?
            .parse::<f64>()
            .map_err(|e| e.to_string())?;
        if response_json
            .get("success")
            .and_then(|value| value.as_bool())
            != Some(true)
            || int_field(&response_json, "status")? != 200
            || !response_json
                .get("message")
                .is_some_and(serde_json::Value::is_null)
            || !close(response_p_value, p_value)
        {
            return Err(format!(
                "Pell significance raw p-value response failed for {outcome_id}"
            ));
        }
    }

    let significance_assessment = significance
        .get("evidence_assessment")
        .ok_or("Pell significance evidence assessment")?;
    if string_field(significance_assessment, "screen_status")?
        != "official_datalab_independent_estimates_screen_captured"
        || string_field(significance_assessment, "covariance_aware_status")?
            != "blocked_no_covariance_term_or_replicate_weight_difference_estimate"
        || string_field(significance_assessment, "causal_status")?
            != "not_causal_observational_unadjusted_receipt_group_comparison"
        || !string_field(significance_assessment, "eligibility_status")?.contains("not_measured")
        || !string_field(significance_assessment, "category_status")?.contains("five_category")
        || !string_field(significance_assessment, "mature_completion_status")?.contains("blocked")
        || !string_field(significance_assessment, "permanent_dropout_status")?
            .contains("not_measured")
        || !string_field(significance_assessment, "cost_effectiveness_status")?.contains("blocked")
        || !string_field(significance_assessment, "current_fiscal_link")?.contains("blocked")
        || string_field(significance_assessment, "fraud_status")? != "not_measured_not_inferred"
        || string_field(significance_assessment, "savings_status")? != "blocked_not_scored"
        || !string_field(significance_assessment, "future_gate")?.contains("BPS:20/25")
    {
        return Err("Pell significance evidence boundary failed".to_string());
    }

    let significance_reader =
        fs::read_to_string(root.join(PELL_CURRENT_ENTRANT_SIGNIFICANCE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        PELL_CURRENT_ENTRANT_SIGNIFICANCE_JSON_PATH,
        "Independent Estimates t-Test",
        "query `396385`",
        "retrieval code `zclxfu`",
        "0.0160308",
        "1.5193e-15",
        "Bonferroni",
        "not a causal",
        "does **not** include a",
        "covariance-aware",
        "does not establish ineligibility",
        "not permanent dropout",
        "BPS:20/25",
        "fraud finding",
        "budget saving",
    ] {
        if !significance_reader.contains(required) {
            return Err(format!("Pell significance reader missing {required}"));
        }
    }
    Ok(())
}

