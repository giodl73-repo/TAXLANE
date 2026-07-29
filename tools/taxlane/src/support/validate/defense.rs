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

pub(crate) fn validate_defense_outcome_floor_definition_packet(root: &Path) -> Result<(), String> {
    for path in [
        DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing defense outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "defense-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")? != "defense_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 163
        || string_field(&record, "lane_id")? != "national-defense"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(
            &record,
            "social_security_outcome_floor_definition_packet_path",
        )? != SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("defense floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("defense floor status {field} must be true"));
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
            return Err(format!("defense floor status {field} must be false"));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense floor definition policy")?;
    for field in [
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "international_differences_not_savings",
        "no_fraud_inference",
        "force_structure_required_before_target_cost",
        "readiness_and_procurement_schedule_required_before_solver_use",
        "federal_translation_required_before_solver_use",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("defense floor policy {field} must be true"));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("defense required floor class count failed".to_string());
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
        return Err("defense required floor class set failed".to_string());
    }
    for row in classes {
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if row.get(field) != Some(&serde_json::Value::Null) {
                return Err(format!("defense floor class {field} must be null"));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("defense floor class must remain unpassed".to_string());
        }
    }

    let defense_floors = record
        .get("defense_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense-specific floor definitions")?;
    let expected_defense_floors = [
        "treaty_commitments",
        "readiness",
        "personnel_safety",
        "strategic_reserve",
        "force_structure_procurement_feasibility",
    ];
    if defense_floors.len() != expected_defense_floors.len() {
        return Err("defense-specific floor count failed".to_string());
    }
    let observed_defense_floors = defense_floors
        .iter()
        .map(|row| string_field(row, "floor_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_defense_floor_set = expected_defense_floors
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_defense_floors != expected_defense_floor_set {
        return Err("defense-specific floor set failed".to_string());
    }
    for row in defense_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("defense-specific floors must remain null and unpassed".to_string());
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
        .ok_or("defense floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("defense_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
    {
        return Err("defense floor summary counts failed".to_string());
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
            return Err(format!("defense floor summary {field} must be false"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("defense floor definition packet publication flag failed".to_string());
    }
    for field in [
        "threshold_values_selected",
        "baseline_values_populated",
        "policy_values_populated",
        "stress_values_populated",
        "pass_fail_review_complete",
        "all_floors_passed",
        "lower_cost_scenario_admissibility_ready",
        "force_structure_plan_published",
        "procurement_schedule_published",
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
            return Err(format!("defense floor claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This defense floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "No defense lower-cost scenario is admissible until treaty commitments, readiness, personnel safety, strategic reserve, force structure, procurement, equity, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "A GDP policy band is not a force-structure plan, procurement schedule, federal score, target cost, or solver input.",
        "No target cost, federal effect, gross savings, net savings, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a force-structure plan",
        "not a procurement schedule",
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
            return Err(format!("defense floor reader missing phrase: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_defense_source_readiness_gap(root: &Path) -> Result<(), String> {
    for path in [
        DEFENSE_SOURCE_READINESS_GAP_JSON_PATH,
        DEFENSE_SOURCE_READINESS_GAP_SCHEMA_PATH,
        DEFENSE_SOURCE_READINESS_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing defense source readiness gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(DEFENSE_SOURCE_READINESS_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "defense-source-readiness-gap:v1"
        || string_field(&record, "record_family")? != "defense_source_readiness_gap"
        || int_field(&record, "pulse")? != 186
        || string_field(&record, "lane_id")? != "national-defense"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "defense_outcome_floor_definition_packet_path")?
            != DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("defense source readiness identity failed".to_string());
    }

    let expected_sources = [
        "SRC-GAO-WEAPON-SYSTEMS-2025",
        "SRC-CBO-FYDP-2025",
        "SRC-DODIG-FY2025-AUDIT",
        "SRC-NATO-DEFEXP-2025",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_sources = record
        .get("source_ids_referenced_but_not_custody_ready")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source readiness referenced source ids")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| "defense source id must be string".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_sources != expected_sources {
        return Err("defense source readiness referenced source set failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source readiness custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "defense_source_references_present",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "defense source readiness status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "force_structure_raw_custody_ready",
        "readiness_raw_custody_ready",
        "procurement_schedule_raw_custody_ready",
        "fydp_raw_custody_ready",
        "nato_or_sipri_raw_custody_ready",
        "audit_control_raw_custody_ready",
        "source_capture_complete",
        "solver_input_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "defense source readiness status {field} must be false"
            ));
        }
    }

    let references = record
        .get("referencing_artifacts")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source readiness referencing artifacts")?;
    if references.len() != 3 {
        return Err("defense source readiness reference count failed".to_string());
    }
    let expected_reference_paths = [
        "data/derived/efficiency_pressure/cost_down_source_packets.fy2025.v1.draft.jsonl",
        "docs/reading/defense-procurement-control-source-packet.md",
        DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_reference_paths = references
        .iter()
        .map(|row| string_field(row, "artifact_path"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_reference_paths != expected_reference_paths {
        return Err("defense source readiness reference path set failed".to_string());
    }
    for row in references {
        if row
            .get("raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
            || row
                .get("may_populate_force_structure_or_solver")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("defense source readiness references must stay blocked".to_string());
        }
    }

    let requirements = record
        .get("source_capture_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source capture requirements")?;
    if requirements.len() != 10 {
        return Err("defense source readiness requirement count failed".to_string());
    }
    for row in requirements {
        if row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("defense source readiness requirements must be null/false".to_string());
        }
    }

    let floors = record
        .get("floor_value_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source readiness floor value status")?;
    if floors.len() != 5 {
        return Err("defense source readiness floor count failed".to_string());
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
                    "defense source readiness floor {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err("defense source readiness floors must remain unpassed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source readiness blocked outputs")?;
    if blocked
        .values()
        .any(|value| value != &serde_json::Value::Null)
    {
        return Err("defense source readiness blocked outputs must remain null".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source readiness claim booleans")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "defense_source_readiness_gap_published" | "defense_source_references_present"
        );
        if value.as_bool() != Some(expected) {
            return Err(format!(
                "defense source readiness claim {field} must be {expected}"
            ));
        }
    }

    let public_warning = string_field(&record, "public_warning")?;
    let reader = fs::read_to_string(root.join(DEFENSE_SOURCE_READINESS_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        DEFENSE_SOURCE_READINESS_GAP_JSON_PATH,
        "SRC-GAO-WEAPON-SYSTEMS-2025",
        "SRC-CBO-FYDP-2025",
        "SRC-DODIG-FY2025-AUDIT",
        "SRC-NATO-DEFEXP-2025",
        "raw artifact path",
        "raw byte count",
        "raw SHA-256",
        "force-structure lineage",
        "readiness indicator lineage",
        "procurement schedule lineage",
        "strategy and commitment lineage",
        "audit-control lineage",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "defense source readiness reader missing phrase: {phrase}"
            ));
        }
    }
    for phrase in [
        "context references only",
        "not defense raw source custody",
        "not a force-structure plan",
        "not a readiness floor value packet",
        "not a procurement schedule",
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
        if !public_warning.contains(phrase) {
            return Err(format!(
                "defense source readiness warning missing phrase: {phrase}"
            ));
        }
        if !reader.contains(phrase) {
            return Err(format!(
                "defense source readiness reader missing warning phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_defense_source_capture_queue(root: &Path) -> Result<(), String> {
    for path in [
        DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        DEFENSE_SOURCE_CAPTURE_QUEUE_SCHEMA_PATH,
        DEFENSE_SOURCE_CAPTURE_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing defense source capture queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "defense-source-capture-queue:v1"
        || string_field(&record, "record_family")? != "defense_source_capture_queue"
        || int_field(&record, "pulse")? != 187
        || string_field(&record, "lane_id")? != "national-defense"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "defense_source_readiness_gap_path")?
            != DEFENSE_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(&record, "defense_outcome_floor_definition_packet_path")?
            != DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "lane_floor_source_work_queue_path")?
            != LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH
    {
        return Err("defense source capture queue identity failed".to_string());
    }

    let rules = record
        .get("source_rules")
        .ok_or("defense source capture rules")?;
    for field in [
        "official_sources_only",
        "use_existing_captured_sources_when_available",
        "new_external_downloads_not_performed_in_this_pulse",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "threshold_selection_requires_stronger_model_review",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "international_spending_differences_are_not_savings",
        "no_fraud_inference_from_audit_or_comparison_context",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("defense source rule must be true: {field}"));
        }
    }

    let items = record
        .get("capture_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source capture items")?;
    if items.len() != 6 {
        return Err("defense source capture item count failed".to_string());
    }
    let expected = [
        ("capture-defense-force-structure-baseline", 1),
        ("capture-defense-readiness-indicators", 2),
        ("capture-defense-procurement-schedule", 3),
        ("capture-defense-policy-commitment-comparator-context", 4),
        ("capture-defense-audit-control-context", 5),
        ("capture-defense-transition-and-industrial-base-capacity", 6),
    ];
    for (work_item_id, priority) in expected {
        let item = items
            .iter()
            .find(|item| string_field(item, "work_item_id").as_deref() == Ok(work_item_id))
            .ok_or_else(|| format!("missing defense source capture item: {work_item_id}"))?;
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
            || item
                .get("referenced_context_source_ids")
                .and_then(serde_json::Value::as_array)
                .is_none()
        {
            return Err(format!(
                "defense source capture item shape failed: {work_item_id}"
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
                    "defense source capture item field must be null: {work_item_id}.{field}"
                ));
            }
        }
        if item.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "defense source capture item must not be ready: {work_item_id}"
            ));
        }
    }

    let counts = record
        .get("aggregate_status")
        .ok_or("defense source capture aggregate status")?;
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
                "defense source capture aggregate count failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "defense source capture blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("defense source capture claim bool")?;
        if field == "defense_source_capture_queue_published" {
            if !observed {
                return Err("defense source capture queue published flag must be true".to_string());
            }
        } else if observed {
            return Err(format!(
                "defense source capture claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "names official-source work items only",
        "not defense raw source custody",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
        "not a policy commitment band",
        "not transition or industrial-base costing",
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
                "defense source capture warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_QUEUE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH,
        "Defense force-structure baseline",
        "Defense readiness indicators",
        "Defense procurement schedule",
        "Defense policy-commitment and comparator context",
        "Defense audit-control context",
        "Defense transition and industrial-base capacity",
        "not defense raw source custody",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
        "not a policy commitment band",
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
            return Err(format!("defense source capture reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_defense_source_capture_status_rollup(root: &Path) -> Result<(), String> {
    for path in [
        DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_SCHEMA_PATH,
        DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing defense source capture status rollup artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "defense-source-capture-status-rollup:v1"
        || string_field(&record, "record_family")? != "defense_source_capture_status_rollup"
        || int_field(&record, "pulse")? != 188
        || string_field(&record, "lane_id")? != "national-defense"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "defense_source_readiness_gap_path")?
            != DEFENSE_SOURCE_READINESS_GAP_JSON_PATH
        || string_field(&record, "defense_source_capture_queue_path")?
            != DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "defense_outcome_floor_definition_packet_path")?
            != DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
    {
        return Err("defense source capture status rollup identity failed".to_string());
    }

    let rows = record
        .get("source_family_rollup")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source capture status rows")?;
    if rows.len() != 6 {
        return Err("defense source capture status rollup must have six rows".to_string());
    }
    let expected_families = [
        "DoD force-structure and posture baseline",
        "Defense readiness indicators",
        "Defense procurement schedule",
        "Defense policy commitment and comparator context",
        "Defense audit-control context",
        "Defense transition and industrial-base capacity",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let expected_work_items = [
        "capture-defense-force-structure-baseline",
        "capture-defense-readiness-indicators",
        "capture-defense-procurement-schedule",
        "capture-defense-policy-commitment-comparator-context",
        "capture-defense-audit-control-context",
        "capture-defense-transition-and-industrial-base-capacity",
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
        return Err("defense source capture status family/work item set failed".to_string());
    }

    for row in rows {
        let family = string_field(row, "source_family")?;
        if string_field(row, "source_role")?.is_empty()
            || string_field(row, "custody_status")? != "capture_item_open"
        {
            return Err(format!(
                "defense source capture status row shape failed: {family}"
            ));
        }
        for field in [
            "raw_custody_ready",
            "may_populate_context",
            "may_populate_force_structure_plan",
            "may_populate_floor_threshold_or_pass_fail",
            "may_populate_procurement_schedule",
            "may_populate_solver_input",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "defense source capture status row must block {field}: {family}"
                ));
            }
        }
    }

    let counts = record
        .get("readiness_counts")
        .ok_or("defense source capture status counts")?;
    for (field, expected) in [
        ("source_family_count", 6),
        ("capture_item_open_count", 6),
        ("raw_custody_ready_count", 0),
        ("context_ready_count", 0),
        ("force_structure_plan_ready_count", 0),
        ("readiness_floor_ready_count", 0),
        ("procurement_schedule_ready_count", 0),
        ("solver_input_ready_count", 0),
        ("public_rate_ready_count", 0),
    ] {
        if int_field(counts, field)? != expected {
            return Err(format!(
                "defense source capture status count failed: {field}"
            ));
        }
    }

    let readiness = record
        .get("defense_lane_readiness")
        .ok_or("defense lane source capture readiness")?;
    for field in [
        "defense_source_readiness_gap_published",
        "defense_source_capture_queue_published",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "defense lane source capture readiness should be true: {field}"
            ));
        }
    }
    for field in [
        "source_capture_complete",
        "force_structure_raw_custody_ready",
        "readiness_raw_custody_ready",
        "procurement_schedule_raw_custody_ready",
        "policy_commitment_context_ready",
        "audit_control_raw_custody_ready",
        "transition_industrial_base_context_ready",
        "threshold_values_selected",
        "observed_floor_values_populated",
        "pass_fail_findings_populated",
        "lower_cost_scenario_admissibility_ready",
        "federal_policy_translation_ready",
        "solver_input_ready",
        "public_rate_ready",
    ] {
        if readiness.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "defense lane source capture readiness must be false: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture status blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "defense source capture status blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture status claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("defense source capture status claim bool")?;
        if matches!(
            field.as_str(),
            "defense_source_capture_status_rollup_published"
                | "defense_source_readiness_gap_published"
                | "defense_source_capture_queue_published"
        ) {
            if !observed {
                return Err(format!(
                    "defense source capture status claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "defense source capture status claim must be false: {field}"
            ));
        }
    }

    let gates = record
        .get("next_source_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source capture status next gates")?;
    if gates.len() != 6 {
        return Err("defense source capture status gate count failed".to_string());
    }
    for gate in gates {
        if string_field(gate, "gate")?.is_empty()
            || gate.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || !gate.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("defense source capture status gates must remain null/false".to_string());
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "summarizes open work items only",
        "not complete defense source capture",
        "not defense raw source custody",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
        "not a policy commitment band",
        "not audit-control findings",
        "not transition or industrial-base costing",
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
                "defense source capture status warning missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH,
        "All six capture families remain open",
        "DoD force-structure and posture baseline",
        "defense readiness indicators",
        "defense procurement schedule",
        "defense policy commitment and comparator context",
        "defense audit-control context",
        "defense transition and industrial-base capacity",
        "not complete defense source capture",
        "not defense raw source custody",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
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
                "defense source capture status reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_defense_source_capture_closure_work_queue(root: &Path) -> Result<(), String> {
    for path in [
        DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH,
        DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_SCHEMA_PATH,
        DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing defense source capture closure work queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "defense-source-capture-closure-work-queue:v1"
        || string_field(&record, "record_family")? != "defense_source_capture_closure_work_queue"
        || int_field(&record, "pulse")? != 189
        || string_field(&record, "lane_id")? != "national-defense"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "defense_source_capture_queue_path")?
            != DEFENSE_SOURCE_CAPTURE_QUEUE_JSON_PATH
        || string_field(&record, "defense_source_capture_status_rollup_path")?
            != DEFENSE_SOURCE_CAPTURE_STATUS_ROLLUP_JSON_PATH
        || string_field(&record, "defense_outcome_floor_definition_packet_path")?
            != DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
    {
        return Err("defense source capture closure work queue identity failed".to_string());
    }

    let rules = record
        .get("closure_rules")
        .ok_or("defense source capture closure rules")?;
    for field in [
        "official_sources_only",
        "new_external_downloads_not_performed_in_this_pulse",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "threshold_selection_requires_stronger_model_review",
        "policy_mechanism_design_requires_stronger_model_review",
        "international_spending_differences_are_not_savings",
        "no_fraud_inference_from_audit_or_comparison_context",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "defense source capture closure rule must be true: {field}"
            ));
        }
    }

    let items = record
        .get("closure_work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("defense source capture closure items")?;
    if items.len() != 6 {
        return Err("defense source capture closure item count failed".to_string());
    }
    let expected = [
        (
            "close-defense-force-structure-custody-lineage",
            "capture-defense-force-structure-baseline",
            "force_structure_raw_custody_ready",
            1,
        ),
        (
            "close-defense-readiness-custody-lineage",
            "capture-defense-readiness-indicators",
            "readiness_raw_custody_ready",
            2,
        ),
        (
            "close-defense-procurement-schedule-custody-lineage",
            "capture-defense-procurement-schedule",
            "procurement_schedule_raw_custody_ready",
            3,
        ),
        (
            "close-defense-policy-commitment-comparator-lineage",
            "capture-defense-policy-commitment-comparator-context",
            "policy_commitment_context_ready",
            4,
        ),
        (
            "close-defense-audit-control-custody-lineage",
            "capture-defense-audit-control-context",
            "audit_control_raw_custody_ready",
            5,
        ),
        (
            "close-defense-transition-industrial-base-lineage",
            "capture-defense-transition-and-industrial-base-capacity",
            "transition_industrial_base_context_ready",
            6,
        ),
    ];
    for (closure_item_id, queue_item_id, closure_gate, priority) in expected {
        let item = items
            .iter()
            .find(|item| string_field(item, "closure_item_id").as_deref() == Ok(closure_item_id))
            .ok_or_else(|| format!("missing defense closure item: {closure_item_id}"))?;
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
                "defense source capture closure item shape failed: {closure_item_id}"
            ));
        }
        for field in ["raw_artifact_path", "metadata_path", "closure_value"] {
            if !item.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "defense source capture closure field must be null: {closure_item_id}.{field}"
                ));
            }
        }
        if item.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "defense source capture closure item must not be ready: {closure_item_id}"
            ));
        }
    }

    let counts = record
        .get("aggregate_status")
        .ok_or("defense source capture closure aggregate status")?;
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
                "defense source capture closure aggregate count failed: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture closure blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "defense source capture closure blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("defense source capture closure claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("defense source capture closure claim bool")?;
        if field == "defense_source_capture_closure_work_queue_published" {
            if !observed {
                return Err(
                    "defense source capture closure publication flag must be true".to_string(),
                );
            }
        } else if observed {
            return Err(format!(
                "defense source capture closure claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for required in [
        "names closure gates only",
        "not complete defense source capture",
        "not defense raw source custody",
        "not lineage review completion",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
        "not a policy commitment band",
        "not transition or industrial-base costing",
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
                "defense source capture closure warning missing: {required}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for required in [
        DEFENSE_SOURCE_CAPTURE_CLOSURE_WORK_QUEUE_JSON_PATH,
        "Force-structure custody lineage",
        "Readiness custody lineage",
        "Procurement schedule custody lineage",
        "Policy-commitment/comparator lineage",
        "Audit-control custody lineage",
        "Transition and industrial-base lineage",
        "not complete defense source capture",
        "not defense raw source custody",
        "not lineage review completion",
        "not a force-structure plan",
        "not readiness floor values",
        "not a procurement schedule",
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
                "defense source capture closure reader missing: {required}"
            ));
        }
    }

    Ok(())
}

