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

pub(crate) fn validate_lane_floor_readiness_rollup(root: &Path) -> Result<(), String> {
    for path in [
        LANE_FLOOR_READINESS_ROLLUP_JSON_PATH,
        LANE_FLOOR_READINESS_ROLLUP_SCHEMA_PATH,
        LANE_FLOOR_READINESS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing lane floor readiness artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(LANE_FLOOR_READINESS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&rollup, "record_id")? != "lane-floor-readiness-rollup:v1"
        || string_field(&rollup, "record_family")? != "lane_floor_readiness_rollup"
        || int_field(&rollup, "pulse")? != 176
        || string_field(&rollup, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&rollup, "international_comparator_target_rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(&rollup, "global_country_comparison_coverage_path")?
            != GLOBAL_COUNTRY_COMPARISON_JSON_PATH
        || string_field(&rollup, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&rollup, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
    {
        return Err("lane floor readiness identity failed".to_string());
    }

    let coverage = rollup
        .get("coverage_rule")
        .ok_or("lane floor coverage rule")?;
    if int_field(coverage, "analytical_lane_count")? != 15
        || int_field(coverage, "budget_row_count")? != 17
    {
        return Err("lane floor coverage counts failed".to_string());
    }
    for field in [
        "fifteen_lanes_are_not_seventeen_budget_rows",
        "floor_definition_packet_coverage_is_not_floor_passage",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "international_differences_are_not_savings",
        "revenue_solvency_and_payment_integrity_are_non_additive_overlays",
        "net_interest_is_endogenous_not_directly_cuttable",
    ] {
        if coverage.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("lane floor coverage rule must be true: {field}"));
        }
    }

    let aggregate = rollup
        .get("aggregate_status")
        .ok_or("lane floor aggregate status")?;
    for (field, expected) in [
        ("lanes_total", 15),
        ("lanes_with_floor_definition_packet", 15),
        ("lanes_with_threshold_values", 15),
        ("lanes_with_sourced_baseline_floor_values", 15),
        ("lanes_with_wave_e_reference_policy_values", 15),
        ("lanes_with_wave_e_synthetic_stress_values", 15),
        ("lanes_with_wave_e_comparator_results", 15),
        ("lanes_with_real_reform_policy_floor_values", 0),
        ("lanes_with_real_reform_stress_floor_values", 0),
        ("lanes_with_all_floors_passed", 0),
        ("lanes_with_component_reference_paths", 15),
        ("lanes_with_component_reform_policy_paths", 0),
        ("lanes_with_behavior_incidence_transition_models", 0),
        ("lanes_solver_ready", 0),
    ] {
        if int_field(aggregate, field)? != expected {
            return Err(format!("lane floor aggregate count failed: {field}"));
        }
    }
    for field in [
        "all_floor_thresholds_selected",
        "all_floor_values_sourced",
        "all_floors_passed",
        "all_lanes_defensible_for_target_costs",
        "all_lanes_defensible_for_public_rates",
        "solver_ready",
        "balanced_budget_ready",
    ] {
        if aggregate.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("lane floor aggregate must remain false: {field}"));
        }
    }
    if aggregate
        .get("floor_definition_packet_coverage_complete")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("lane floor coverage-complete flag must be true".to_string());
    }

    let rows = rollup
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane floor rows")?;
    let expected_paths = [
        (
            "health-medicare",
            HEALTH_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "social-security",
            SOCIAL_SECURITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "national-defense",
            DEFENSE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "income-security-family",
            INCOME_SECURITY_FAMILY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "revenue-solvency",
            REVENUE_SOLVENCY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "net-interest",
            NET_INTEREST_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "payment-integrity",
            PAYMENT_INTEGRITY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "veterans",
            VETERANS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "transportation-infrastructure",
            TRANSPORTATION_INFRASTRUCTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "education-workforce",
            EDUCATION_WORKFORCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "disaster-resilience",
            DISASTER_RESILIENCE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "justice-courts-public-safety",
            JUSTICE_COURTS_PUBLIC_SAFETY_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "science-energy-environment",
            SCIENCE_ENERGY_ENVIRONMENT_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "agriculture",
            AGRICULTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
        (
            "international-affairs",
            INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let observed = rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = expected_paths.keys().copied().map(str::to_string).collect();
    if observed != expected || rows.len() != expected_paths.len() {
        return Err("lane floor row set failed".to_string());
    }

    for row in rows {
        let lane_id = string_field(row, "lane_id")?;
        if string_field(row, "public_label")?.is_empty()
            || string_field(row, "next_blocker")?.is_empty()
        {
            return Err(format!("lane floor row missing label/blocker: {lane_id}"));
        }
        let expected_path = expected_paths
            .get(lane_id.as_str())
            .ok_or("lane floor expected path")?;
        if string_field(row, "floor_definition_packet_path")? != *expected_path {
            return Err(format!("lane floor packet path failed: {lane_id}"));
        }
        if !root.join(expected_path).exists() {
            return Err(format!("lane floor packet path missing on disk: {lane_id}"));
        }
        if row
            .get("floor_definition_packet_exists")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!("lane floor packet existence failed: {lane_id}"));
        }
        for field in ["threshold_values_selected", "baseline_values_sourced"] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
                return Err(format!(
                    "lane floor row threshold/baseline readiness failed: {lane_id} {field}"
                ));
            }
        }
        for field in [
            "policy_values_sourced",
            "stress_values_sourced",
            "all_floors_passed",
            "component_policy_path_ready",
            "behavior_incidence_transition_model_ready",
            "solver_ready",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "lane floor row must remain false: {lane_id} {field}"
                ));
            }
        }
    }

    let blocked = rollup
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane floor blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("lane floor blocked output must be null: {field}"));
        }
    }

    let warnings = rollup
        .get("public_warning_phrases")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane floor public warnings")?;
    for required in [
        "All fifteen analytical lanes now have outcome-floor definition packets and Wave E comparator results, but no lane has passed a complete floor set under a real reform scenario.",
        "All fifteen lanes now have at least one draft anchor threshold and sourced baseline floor-value packet, but those packets are not complete lane floors, actual reform pass/fail findings, target costs, savings, solver input, or rates.",
        "Fifteen analytical lanes are not the same as the 17 budget rows.",
        "Revenue-solvency and payment-integrity remain non-additive overlays.",
        "Net interest is endogenous and cannot be cut directly.",
        "International spending differences are not savings, and no fraud inference is allowed from comparison or improper-payment estimates.",
        "Missing values remain null and blocked gates remain false.",
    ] {
        if !warnings.iter().any(|v| v.as_str() == Some(required)) {
            return Err(format!("lane floor warning missing: {required}"));
        }
    }

    let claims = rollup
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane floor claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("lane floor claim bool")?;
        if matches!(
            field.as_str(),
            "lane_floor_readiness_rollup_published"
                | "floor_definition_packet_coverage_complete"
                | "wave_e_reference_calibrations_ready"
                | "component_reference_paths_ready"
                | "reference_policy_values_ready"
                | "synthetic_stress_values_ready"
                | "comparator_results_ready"
        ) {
            if !observed {
                return Err(format!("lane floor claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("lane floor claim must remain false: {field}"));
        }
    }

    let status = string_field(&rollup, "plain_english_status")?;
    for required in [
        "completed floor-definition packet coverage",
        "fifteen draft threshold/baseline anchor floor-value packets",
        "neither wave makes the lanes defensible",
        "target costs",
        "rates",
        "savings",
        "solver outputs",
        "technology savings",
        "department cuts",
        "waste findings",
        "fraud findings",
        "balanced-budget claim",
    ] {
        if !status.contains(required) {
            return Err(format!(
                "lane floor plain-English status missing: {required}"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_FLOOR_READINESS_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        LANE_FLOOR_READINESS_ROLLUP_JSON_PATH,
        "outcome-floor definition packets",
        "draft threshold and sourced",
        "no lane has passed a complete floor set under a real reform scenario",
        "not actual reform pass/fail findings",
        "Fifteen analytical lanes are not the same as the 17 budget rows.",
        "Revenue-solvency and payment-integrity remain non-additive overlays.",
        "Net interest is endogenous and cannot be cut directly.",
        "International spending differences are not savings",
        "Wave E reference-calibration layer now supplies",
        "not target-cost selection",
        "not a federal score",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader_words.contains(required) {
            return Err(format!("lane floor reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_lane_floor_source_work_queue(root: &Path) -> Result<(), String> {
    for path in [
        LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH,
        LANE_FLOOR_SOURCE_WORK_QUEUE_SCHEMA_PATH,
        LANE_FLOOR_SOURCE_WORK_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing lane floor source queue artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let queue: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&queue, "record_id")? != "lane-floor-source-work-queue:v1"
        || string_field(&queue, "record_family")? != "lane_floor_source_work_queue"
        || int_field(&queue, "pulse")? != 177
        || string_field(&queue, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&queue, "international_comparator_target_rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(&queue, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&queue, "lane_floor_readiness_rollup_path")?
            != LANE_FLOOR_READINESS_ROLLUP_JSON_PATH
    {
        return Err("lane floor source queue identity failed".to_string());
    }

    let rules = queue.get("source_rules").ok_or("lane floor source rules")?;
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
            return Err(format!("lane floor source rule must be true: {field}"));
        }
    }

    let aggregate = queue
        .get("aggregate_status")
        .ok_or("lane floor source aggregate")?;
    for (field, expected) in [
        ("work_item_count", 15),
        ("work_items_ready_for_source_capture", 15),
        ("threshold_values_selected", 0),
        ("baseline_values_populated", 0),
        ("policy_values_populated", 0),
        ("stress_values_populated", 0),
        ("pass_fail_findings_populated", 0),
        ("solver_ready_items", 0),
        ("public_rate_ready_items", 0),
    ] {
        if int_field(aggregate, field)? != expected {
            return Err(format!("lane floor source aggregate failed: {field}"));
        }
    }

    let items = queue
        .get("work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane floor source work items")?;
    let expected_lanes = [
        "health-medicare",
        "social-security",
        "national-defense",
        "income-security-family",
        "revenue-solvency",
        "net-interest",
        "payment-integrity",
        "veterans",
        "transportation-infrastructure",
        "education-workforce",
        "disaster-resilience",
        "justice-courts-public-safety",
        "science-energy-environment",
        "agriculture",
        "international-affairs",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_lanes = items
        .iter()
        .map(|item| string_field(item, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_lanes != expected_lanes || items.len() != 15 {
        return Err("lane floor source lane set failed".to_string());
    }

    let mut priorities = BTreeSet::new();
    for item in items {
        let lane_id = string_field(item, "lane_id")?;
        let priority = int_field(item, "priority")?;
        if !(1..=15).contains(&priority) || !priorities.insert(priority) {
            return Err(format!("lane floor source priority failed: {lane_id}"));
        }
        for field in ["floor_dimensions", "official_source_families"] {
            let values = item
                .get(field)
                .and_then(serde_json::Value::as_array)
                .ok_or("lane floor source list")?;
            if values.len() < 4 {
                return Err(format!(
                    "lane floor source list too short: {lane_id} {field}"
                ));
            }
        }
        if string_field(item, "next_capture")?.is_empty() {
            return Err(format!("lane floor source next_capture empty: {lane_id}"));
        }
        for field in [
            "threshold_value",
            "baseline_value",
            "policy_value",
            "stress_value",
        ] {
            if !item.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "lane floor source value must be null: {lane_id} {field}"
                ));
            }
        }
        for field in ["pass_fail", "solver_ready"] {
            if item.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "lane floor source gate must be false: {lane_id} {field}"
                ));
            }
        }
    }

    let blocked = queue
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane floor source blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "lane floor source blocked output must be null: {field}"
            ));
        }
    }

    let claims = queue
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane floor source claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("lane floor source claim bool")?;
        if field == "lane_floor_source_work_queue_published" {
            if !observed {
                return Err("lane floor source publication flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("lane floor source claim must be false: {field}"));
        }
    }

    let warning = string_field(&queue, "public_warning")?;
    for required in [
        "official-source work queue",
        "not threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not target-cost selection",
        "not a federal score",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(required) {
            return Err(format!("lane floor source warning missing: {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_FLOOR_SOURCE_WORK_QUEUE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        LANE_FLOOR_SOURCE_WORK_QUEUE_JSON_PATH,
        "This queue covers all fifteen analytical lanes.",
        "It does not choose thresholds",
        "official sources only",
        "no FOIA request, records request, form, email, phone call, or agency/person contact",
        "threshold selection requires stronger-model review",
        "missing values remain null",
        "blocked gates remain false",
        "not threshold selection",
        "not observed floor values",
        "not pass/fail findings",
        "not target-cost selection",
        "not a federal score",
        "not gross savings",
        "not net savings",
        "not solver input",
        "not rate calculation",
        "not a public rate card",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("lane floor source reader missing: {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_lane_scenario_pack_wave_e_readiness(root: &Path) -> Result<(), String> {
    for path in [
        LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH,
        LANE_SCENARIO_PACK_WAVE_E_READINESS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing lane scenario Wave E artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH}: {err}")
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "lane-scenario-pack-wave-e-readiness:v1"
        || string_field(&record, "record_family")? != "lane_scenario_pack_wave_e_readiness"
        || string_field(&record, "status")?
            != "wave_e_complete_fifteen_lane_reference_calibrations_reform_and_fiscal_effects_blocked"
        || string_field(&record, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "lane_full_coverage_matrix_path")?
            != LANE_FULL_COVERAGE_MATRIX_JSON_PATH
        || string_field(&record, "wave_e_reference_scenario_packs_path")?
            != WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH
        || string_field(&record, "role_review_path")?
            != WAVE_E_REFERENCE_SCENARIO_PACKS_ROLE_REVIEW_PATH
    {
        return Err("lane scenario Wave E identity failed".to_string());
    }

    let required_fields = record
        .get("required_scenario_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane scenario Wave E required fields")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("scenario field string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_fields = [
        "policy_instrument",
        "phase_in",
        "behavior",
        "transition_admin_cost",
        "incidence",
        "score_provenance",
        "floor_results",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if required_fields != expected_fields {
        return Err("lane scenario Wave E required field set failed".to_string());
    }

    let rules = record
        .get("scenario_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane scenario Wave E rules")?;
    for required in [
        "policy_values_require_wave_d_floor_values",
        "wave_d_lane_anchor_contract_satisfied",
        "wave_e_owns_policy_values_stress_values_and_floor_results",
        "lower_cost_scenarios_require_passed_floors",
        "science_energy_environment_components_separate",
        "international_affairs_components_separate",
        "agriculture_components_separate",
        "revenue_solvency_non_additive_overlay",
        "payment_integrity_non_additive_overlay",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "reference_calibration_may_use_current_policy_continuation",
        "synthetic_stress_must_be_one_reported_increment_adverse",
        "reference_calibration_does_not_publish_federal_effect",
    ] {
        if rules.get(required).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("lane scenario Wave E rule failed: {required}"));
        }
    }

    let rows = record
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane scenario Wave E rows")?;
    if rows.len() != 15 {
        return Err("lane scenario Wave E must contain 15 rows".to_string());
    }
    let expected_lanes = [
        "health-medicare",
        "social-security",
        "national-defense",
        "income-security-family",
        "education-workforce",
        "veterans",
        "disaster-resilience",
        "justice-courts-public-safety",
        "science-energy-environment",
        "agriculture",
        "international-affairs",
        "transportation-infrastructure",
        "revenue-solvency",
        "payment-integrity",
        "net-interest",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_lanes = rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_lanes != expected_lanes {
        return Err("lane scenario Wave E lane set failed".to_string());
    }

    for row in rows {
        let lane_id = string_field(row, "lane_id")?;
        let components = row
            .get("required_components")
            .and_then(serde_json::Value::as_array)
            .ok_or("lane scenario components")?;
        if components.is_empty() {
            return Err(format!("{lane_id}: components must not be empty"));
        }
        if lane_id == "science-energy-environment" && components.len() != 3 {
            return Err(
                "science-energy-environment must have three separate components".to_string(),
            );
        }
        if lane_id == "international-affairs" && components.len() != 6 {
            return Err("international-affairs must have six separate components".to_string());
        }
        if lane_id == "agriculture" && components.len() != 5 {
            return Err("agriculture must have five separate components".to_string());
        }
        if string_field(row, "scenario_pack_path")? != WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH
            || string_field(row, "scenario_id")? != format!("{lane_id}:wave-e-reference:v1")
        {
            return Err(format!("{lane_id}: scenario pack lineage failed"));
        }
        for field in [
            "policy_instrument",
            "phase_in",
            "behavior",
            "transition_admin_cost",
            "incidence",
            "score_provenance",
        ] {
            if row
                .get(field)
                .and_then(serde_json::Value::as_str)
                .is_none_or(str::is_empty)
            {
                return Err(format!("{lane_id}: {field} must be populated"));
            }
        }
        let floor_results = row
            .get("floor_results")
            .ok_or("lane scenario Wave E floor results")?;
        if !bool_field(floor_results, "central_reference_passed")?
            || bool_field(floor_results, "adverse_stress_passed")?
            || !bool_field(floor_results, "comparator_verified")?
        {
            return Err(format!("{lane_id}: floor result calibration failed"));
        }
        for field in [
            "scenario_pack_ready",
            "policy_values_ready",
            "stress_values_ready",
            "floor_results_ready",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
                return Err(format!("{lane_id}: {field} must be true"));
            }
        }
        for field in [
            "reform_scenario_ready",
            "federal_effect_ready",
            "lower_cost_scenario_admissible",
            "solver_ready",
            "rate_ready",
            "savings_ready",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!("{lane_id}: {field} must remain false"));
            }
        }
        if matches!(lane_id.as_str(), "revenue-solvency" | "payment-integrity")
            && string_field(row, "additivity")? != "non_additive_overlay"
        {
            return Err(format!("{lane_id}: overlay additivity failed"));
        }
        if lane_id == "net-interest" && string_field(row, "additivity")? != "endogenous" {
            return Err("net-interest must remain endogenous".to_string());
        }
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("lane scenario Wave E aggregate")?;
    for (field, expected) in [
        ("lane_count", 15),
        ("lanes_with_component_requirements", 15),
        ("scenario_packs_ready", 15),
        ("lanes_with_policy_values", 15),
        ("lanes_with_stress_values", 15),
        ("lanes_with_floor_results", 15),
        ("central_reference_passes", 15),
        ("adverse_stress_failures", 15),
        ("reform_scenario_packs_ready", 0),
        ("federal_effects_ready", 0),
        ("lower_cost_scenarios_admissible", 0),
    ] {
        if int_field(aggregate, field)? != expected {
            return Err(format!("lane scenario Wave E aggregate failed: {field}"));
        }
    }
    if aggregate
        .get("wave_e_done")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("lane scenario Wave E completion flag must be true".to_string());
    }
    for field in [
        "solver_ready",
        "rates_ready",
        "savings_ready",
        "balanced_budget_ready",
    ] {
        if aggregate.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "lane scenario Wave E aggregate flag failed: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane scenario Wave E claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("lane scenario Wave E claim bool")?;
        if [
            "lane_scenario_pack_wave_e_readiness_published",
            "wave_e_done",
            "scenario_packs_ready",
            "policy_values_published",
            "stress_values_published",
            "floor_results_published",
        ]
        .contains(&field.as_str())
        {
            if !observed {
                return Err("lane scenario Wave E published flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("lane scenario Wave E claim must be false: {field}"));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_SCENARIO_PACK_WAVE_E_READINESS_READER_PATH))
        .map_err(|err| {
            format!("failed to read {LANE_SCENARIO_PACK_WAVE_E_READINESS_READER_PATH}: {err}")
        })?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH,
        "Wave D's one-source-custodied-anchor-per-lane prerequisite is satisfied",
        "Wave E is complete under the reference-calibration contract",
        "All 15 lanes have current-policy continuation component paths",
        "All 15 central values pass",
        "all 15 adverse stress values fail",
        "Science, energy, and environment stay separate",
        "International affairs stays split",
        "Agriculture stays split",
        "Revenue solvency and payment integrity remain non-additive overlays",
        "not a reform scenario",
        "not a federal effect",
        "not solver-ready",
        "not rate-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader_words.contains(required) {
            return Err(format!("lane scenario Wave E reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_lane_depth_explainability_tracker(root: &Path) -> Result<(), String> {
    for path in [
        LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH,
        LANE_DEPTH_EXPLAINABILITY_TRACKER_SCHEMA_PATH,
        LANE_DEPTH_EXPLAINABILITY_TRACKER_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing lane depth explainability tracker artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let tracker: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&tracker, "record_id")? != "lane-depth-explainability-tracker:v1"
        || string_field(&tracker, "record_family")? != "lane_depth_explainability_tracker"
        || int_field(&tracker, "pulse")? != 110
        || string_field(&tracker, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&tracker, "international_comparator_target_rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(&tracker, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&tracker, "current_law_source_custody_preflight_path")?
            != CURRENT_LAW_SOURCE_CUSTODY_PREFLIGHT_JSON_PATH
    {
        return Err("lane depth explainability tracker identity failed".to_string());
    }

    let completion = tracker
        .get("completion_definition")
        .ok_or("lane depth completion definition")?;
    for field in [
        "lane_depth_complete_requires",
        "public_explainability_complete_requires",
    ] {
        let values = completion
            .get(field)
            .and_then(serde_json::Value::as_array)
            .ok_or("lane depth completion list")?;
        if values.len() < 8 {
            return Err(format!("lane depth completion list too short: {field}"));
        }
    }
    if completion
        .get("missing_values_remain_null")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || completion
            .get("blocked_statuses_remain_false")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("lane depth completion null/false rules failed".to_string());
    }

    let aggregate = tracker
        .get("aggregate_status")
        .ok_or("lane depth aggregate status")?;
    if int_field(aggregate, "lane_count")? != 15
        || int_field(aggregate, "lanes_depth_complete")? != 0
        || int_field(aggregate, "lanes_public_explainability_complete")? != 0
        || aggregate
            .get("every_lane_defensible_for_public_rates")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("solver_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("balanced_rate_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("lane depth aggregate status must remain incomplete".to_string());
    }

    let rows = tracker
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane depth rows")?;
    let observed = rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = [
        "health-medicare",
        "social-security",
        "national-defense",
        "income-security-family",
        "revenue-solvency",
        "net-interest",
        "payment-integrity",
        "veterans",
        "transportation-infrastructure",
        "education-workforce",
        "disaster-resilience",
        "justice-courts-public-safety",
        "science-energy-environment",
        "agriculture",
        "international-affairs",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed != expected || rows.len() != expected.len() {
        return Err("lane depth row set failed".to_string());
    }

    for row in rows {
        for field in [
            "public_label",
            "current_law_baseline_status",
            "source_custody_status",
            "policy_scenario_status",
            "outcome_floor_status",
            "modernization_transition_status",
            "public_explainer_status",
            "solver_mapping_status",
            "next_work",
        ] {
            if string_field(row, field)?.is_empty() {
                return Err(format!("lane depth field empty: {field}"));
            }
        }
        if row
            .get("depth_artifact_paths")
            .and_then(serde_json::Value::as_array)
            .is_none()
        {
            return Err("lane depth artifact paths must be an array".to_string());
        }
        if row
            .get("lane_depth_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
            || row
                .get("public_explainability_complete")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("lane depth completion booleans must remain false".to_string());
        }
    }

    let questions = tracker
        .get("public_questions_required_for_each_lane")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane depth public questions")?;
    if questions.len() != 8 {
        return Err("lane depth public question count failed".to_string());
    }

    let claims = tracker
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane depth claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("lane depth claim bool")?;
        if field == "lane_depth_explainability_tracker_published" {
            if !observed {
                return Err("lane depth tracker publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("lane depth public claim {field} must be false"));
        }
    }

    let status = string_field(&tracker, "plain_english_status")?;
    for required in [
        "We are not done",
        "Transportation is the deepest pilot",
        "every lane remains blocked",
        "public rates",
        "target costs",
        "solver output",
        "savings claims",
        "waste claims",
        "fraud claims",
        "department-cut instructions",
        "technology-savings claims",
        "balanced-budget claims",
    ] {
        if !status.contains(required) {
            return Err(format!("lane depth status missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_DEPTH_EXPLAINABILITY_TRACKER_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH,
        "we are not done filling out the depth of each lane or the public explainability layer",
        "Transportation is the deepest pilot",
        "Every lane still has missing baseline, source-custody, scenario, outcome-floor",
        "What does this lane do?",
        "What are taxpayers paying now?",
        "Who is served or protected?",
        "What outcomes matter before any lower-cost scenario is admissible?",
        "What would count as overspending, underfunding, or only a review-needed signal?",
        "What can technology change, and what transition risks must be paid for?",
        "What evidence is still missing?",
        "What claims are blocked?",
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
            return Err(format!("lane depth reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_lane_agent_work_order_plan(root: &Path) -> Result<(), String> {
    for path in [
        LANE_AGENT_WORK_ORDER_PLAN_JSON_PATH,
        LANE_AGENT_WORK_ORDER_PLAN_SCHEMA_PATH,
        LANE_AGENT_WORK_ORDER_PLAN_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing lane agent work-order artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(LANE_AGENT_WORK_ORDER_PLAN_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let plan: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&plan, "record_id")? != "lane-agent-work-order-plan:v1"
        || string_field(&plan, "record_family")? != "lane_agent_work_order_plan"
        || int_field(&plan, "pulse")? != 111
        || string_field(&plan, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&plan, "program_lane_target_cost_contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&plan, "international_comparator_target_rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
    {
        return Err("lane agent work-order identity failed".to_string());
    }

    let rules = plan
        .get("parallelization_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane agent parallelization rules")?;
    for required_true in [
        "one_lane_per_agent",
        "one_clean_worktree_per_lane",
        "agents_may_not_share_edit_targets",
        "integration_agent_required_after_each_wave",
        "parallel_source_value_scouts_allowed",
        "serial_validator_integration_required",
        "workstream_agents_may_not_flip_gates",
        "normative_target_choices_require_review",
        "policy_mechanism_design_requires_review",
        "outcome_floor_threshold_choices_require_review",
    ] {
        if rules
            .get(required_true)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!("lane agent rule must be true: {required_true}"));
        }
    }
    for required_false in ["external_requests_allowed", "public_claims_allowed"] {
        if rules
            .get(required_false)
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(format!("lane agent rule must be false: {required_false}"));
        }
    }

    let work_order = plan
        .get("standard_agent_work_order")
        .ok_or("lane agent standard work order")?;
    for field in [
        "required_inputs_to_read",
        "required_outputs_per_lane",
        "required_public_questions",
        "required_claim_firewall",
    ] {
        let values = work_order
            .get(field)
            .and_then(serde_json::Value::as_array)
            .ok_or("lane agent work order list")?;
        if values.len() < 6 {
            return Err(format!("lane agent work-order list too short: {field}"));
        }
    }

    let blocker_workstreams = plan
        .get("parallel_blocker_workstreams")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane agent parallel blocker workstreams")?;
    if blocker_workstreams.len() != 5 {
        return Err("lane agent blocker workstream count failed".to_string());
    }
    let expected_workstreams = [
        "current_law_path_extraction",
        "health_medicare_hi_bridge",
        "receipt_base_rate_bridge",
        "outcome_floor_values",
        "scenario_pack_prep",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_workstreams = BTreeSet::new();
    for workstream in blocker_workstreams {
        let workstream_id = string_field(workstream, "workstream_id")?;
        observed_workstreams.insert(workstream_id.clone());
        if int_field(workstream, "max_parallel_agents")? > 3
            || workstream
                .get("integration_required_before_gate_change")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "lane agent blocker workstream rules failed: {workstream_id}"
            ));
        }
        for field in [
            "primary_blockers",
            "agent_allowed_outputs",
            "blocked_outputs",
        ] {
            let values = workstream
                .get(field)
                .and_then(serde_json::Value::as_array)
                .ok_or("lane agent blocker workstream list")?;
            if values.len() < 3 {
                return Err(format!(
                    "lane agent blocker workstream list too short: {workstream_id}.{field}"
                ));
            }
        }
    }
    if observed_workstreams != expected_workstreams {
        return Err("lane agent blocker workstream set failed".to_string());
    }

    let central_protocol = plan
        .get("central_integration_protocol")
        .ok_or("lane agent central integration protocol")?;
    for field in [
        "parallel_agents_recommend_only",
        "main_integration_updates_machine_artifacts",
        "run_required_validation_after_each_integrated_slice",
        "public_outputs_remain_blocked_until_upstream_gates_pass",
    ] {
        if central_protocol
            .get(field)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "lane agent central integration protocol flag failed: {field}"
            ));
        }
    }
    if int_field(central_protocol, "max_active_agent_workstreams")? != 3 {
        return Err("lane agent active workstream cap failed".to_string());
    }

    let waves = plan
        .get("assignment_waves")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane agent assignment waves")?;
    if waves.len() != 5 {
        return Err("lane agent wave count failed".to_string());
    }
    let mut lane_ids = BTreeSet::new();
    for wave in waves {
        if int_field(wave, "max_parallel_agents")? > 3
            || wave
                .get("integration_required")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
            || string_field(wave, "wave_id")?.is_empty()
            || string_field(wave, "rationale")?.is_empty()
        {
            return Err("lane agent wave rules failed".to_string());
        }
        let lanes = wave
            .get("lane_ids")
            .and_then(serde_json::Value::as_array)
            .ok_or("lane agent wave lanes")?;
        if lanes.len() != 3 {
            return Err("lane agent waves must contain three lanes each".to_string());
        }
        for lane in lanes {
            let lane = lane.as_str().ok_or("lane agent lane id string")?;
            if !lane_ids.insert(lane.to_string()) {
                return Err(format!("lane agent duplicate lane: {lane}"));
            }
        }
    }
    let expected = [
        "health-medicare",
        "social-security",
        "national-defense",
        "income-security-family",
        "revenue-solvency",
        "net-interest",
        "payment-integrity",
        "veterans",
        "transportation-infrastructure",
        "education-workforce",
        "disaster-resilience",
        "justice-courts-public-safety",
        "science-energy-environment",
        "agriculture",
        "international-affairs",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if lane_ids != expected {
        return Err("lane agent assignment must cover all 15 analytical lanes".to_string());
    }

    let integration = plan
        .get("integration_review_checklist")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane agent integration checklist")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("lane agent integration checklist string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "all_15_analytical_lanes_present_once",
        "17_budget_rows_not_confused_with_15_analytical_lanes",
        "revenue_solvency_and_payment_integrity_remain_non_additive_overlays",
        "net_interest_remains_endogenous_and_not_directly_cuttable",
        "trust_funds_remain_separate",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "technology_changes_are_transition_paths_not_automatic_savings",
        "international_differences_are_not_savings",
        "improper_payment_estimates_do_not_imply_fraud",
        "public_reader_is_plain_language",
        "validator_and_focused_test_added",
    ] {
        if !integration.contains(required) {
            return Err(format!(
                "lane agent integration checklist missing {required}"
            ));
        }
    }

    let claims = plan
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane agent claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("lane agent claim bool")?;
        if field == "lane_agent_work_order_plan_published"
            || field == "parallel_blocker_workstreams_published"
        {
            if !observed {
                return Err("lane agent work-order publish flag must be true".to_string());
            }
        } else if observed {
            return Err(format!("lane agent public claim {field} must be false"));
        }
    }

    let status = string_field(&plan, "plain_english_status")?;
    for required in [
        "Agents can scale",
        "one lane each",
        "clean worktrees",
        "integration review",
        "five parallel blocker workstreams",
        "Parallel agents recommend only",
        "main integration pass updates machine artifacts",
        "runs validation before any gate changes",
        "does not execute lane agents",
        "execute workstream agents",
        "complete lane depth",
        "calculate rates",
        "publish target costs",
        "claim savings",
        "identify waste or fraud",
        "direct department cuts",
        "claim technology savings",
        "claim a balanced budget",
    ] {
        if !status.contains(required) {
            return Err(format!("lane agent status missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_AGENT_WORK_ORDER_PLAN_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        LANE_AGENT_WORK_ORDER_PLAN_JSON_PATH,
        "one lane per agent",
        "one clean worktree per lane",
        "one integration review after each wave",
        "five parallel blocker workstreams",
        "Current-law path extraction",
        "Health/Medicare HI bridge",
        "Receipt-base/rate bridge",
        "Outcome floor values",
        "Scenario pack prep",
        "Parallel agents recommend only",
        "main integration pass updates machine artifacts",
        "may not flip readiness gates",
        "Health, Social Security, and Defense",
        "Revenue-solvency, Payment-integrity, and Net-interest overlays",
        "all 15 analytical lanes appear once",
        "the 15 analytical lanes are not confused with the 17 budget rows",
        "revenue-solvency and payment-integrity remain non-additive overlays",
        "net interest remains endogenous",
        "trust funds remain separate",
        "missing values remain null",
        "blocked gates remain false",
        "technology changes are transition paths not automatic savings",
        "international differences are not savings",
        "improper-payment estimates do not imply fraud",
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
            return Err(format!("lane agent reader missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_lane_full_coverage_matrix(root: &Path) -> Result<(), String> {
    for path in [
        LANE_FULL_COVERAGE_MATRIX_JSON_PATH,
        LANE_FULL_COVERAGE_MATRIX_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing lane full coverage matrix artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(LANE_FULL_COVERAGE_MATRIX_JSON_PATH))
        .map_err(|err| format!("failed to read {LANE_FULL_COVERAGE_MATRIX_JSON_PATH}: {err}"))?;
    let matrix: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse {LANE_FULL_COVERAGE_MATRIX_JSON_PATH}: {err}"))?;

    if string_field(&matrix, "record_id")? != "lane-full-coverage-matrix:v1"
        || string_field(&matrix, "record_family")? != "lane_full_coverage_matrix"
        || string_field(&matrix, "status")? != "draft_full_coverage_visibility_no_lane_complete"
        || string_field(&matrix, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&matrix, "public_explainer_wave_c_promotion_path")?
            != PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH
        || string_field(&matrix, "lane_floor_readiness_rollup_path")?
            != LANE_FLOOR_READINESS_ROLLUP_JSON_PATH
        || string_field(&matrix, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&matrix, "lane_scenario_pack_wave_e_readiness_path")?
            != LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH
        || string_field(&matrix, "solver_rate_wave_f_readiness_path")?
            != SOLVER_RATE_WAVE_F_READINESS_JSON_PATH
        || string_field(&matrix, "wave_f_transportation_calibration_path")?
            != WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH
        || string_field(&matrix, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&matrix, "post_rollup_readiness_work_queue_path")?
            != POST_ROLLUP_READINESS_WORK_QUEUE_JSON_PATH
    {
        return Err("lane full coverage matrix identity failed".to_string());
    }

    let required_gates = matrix
        .get("required_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane full coverage matrix required_gates")?;
    let expected_gates = [
        "current_law_baseline",
        "source_custody",
        "public_explainer",
        "outcome_floors",
        "policy_scenarios",
        "transition_model",
        "solver_mapping",
        "receipt_rate_bridge",
        "claim_boundary",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_gates = required_gates
        .iter()
        .map(|gate| {
            gate.as_str()
                .map(str::to_string)
                .ok_or_else(|| "lane full coverage gate must be a string".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_gates != expected_gates {
        return Err("lane full coverage matrix required gate set failed".to_string());
    }

    let rows = matrix
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane full coverage matrix lane_rows")?;
    if rows.len() != 15 {
        return Err(format!(
            "lane full coverage matrix must contain exactly 15 lane rows, got {}",
            rows.len()
        ));
    }

    let expected_lanes = [
        "health-medicare",
        "social-security",
        "national-defense",
        "income-security-family",
        "revenue-solvency",
        "net-interest",
        "payment-integrity",
        "veterans",
        "transportation-infrastructure",
        "education-workforce",
        "disaster-resilience",
        "justice-courts-public-safety",
        "science-energy-environment",
        "agriculture",
        "international-affairs",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_lanes = BTreeSet::new();
    let mut complete_lane_count = 0usize;
    let mut partial_current_law_count = 0usize;
    let mut partial_source_custody_count = 0usize;
    let mut complete_public_explainer_count = 0usize;
    let mut partial_outcome_floor_context_count = 0usize;
    let mut partial_policy_scenario_requirement_count = 0usize;
    let mut partial_transition_model_count = 0usize;
    let mut missing_transition_model_count = 0usize;
    let mut partial_solver_mapping_count = 0usize;
    let mut missing_solver_mapping_count = 0usize;
    let mut partial_receipt_rate_bridge_count = 0usize;
    let mut missing_receipt_rate_bridge_count = 0usize;

    for row in rows {
        let lane_id = string_field(row, "lane_id")?;
        if !observed_lanes.insert(lane_id.clone()) {
            return Err(format!("duplicate lane full coverage row {lane_id}"));
        }
        if string_field(row, "coverage_tier")? == "deepest_pilot"
            && lane_id != "transportation-infrastructure"
        {
            return Err("only transportation may be marked deepest_pilot".to_string());
        }

        let gates = row
            .get("gates")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| format!("{lane_id}: missing gates object"))?;
        let row_gate_ids = gates.keys().cloned().collect::<BTreeSet<_>>();
        if row_gate_ids != expected_gates {
            return Err(format!("{lane_id}: gate set failed"));
        }
        for (gate_id, gate) in gates {
            let status = gate
                .get("status")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("{lane_id}:{gate_id}: missing status"))?;
            if !matches!(status, "missing" | "partial" | "complete") {
                return Err(format!("{lane_id}:{gate_id}: unsupported status {status}"));
            }
            if gate
                .get("evidence_paths")
                .and_then(serde_json::Value::as_array)
                .is_none()
            {
                return Err(format!("{lane_id}:{gate_id}: missing evidence_paths"));
            }
            if gate
                .get("blocked_outputs")
                .and_then(serde_json::Value::as_array)
                .is_none()
            {
                return Err(format!("{lane_id}:{gate_id}: missing blocked_outputs"));
            }
            if status == "complete"
                && gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .is_some_and(Vec::is_empty)
            {
                return Err(format!(
                    "{lane_id}:{gate_id}: complete gate has no evidence"
                ));
            }
            match (gate_id.as_str(), status) {
                ("current_law_baseline", "partial") => partial_current_law_count += 1,
                ("source_custody", "partial") => partial_source_custody_count += 1,
                ("transition_model", "partial") => partial_transition_model_count += 1,
                ("transition_model", "missing") => missing_transition_model_count += 1,
                ("solver_mapping", "partial") => partial_solver_mapping_count += 1,
                ("solver_mapping", "missing") => missing_solver_mapping_count += 1,
                ("receipt_rate_bridge", "partial") => partial_receipt_rate_bridge_count += 1,
                ("receipt_rate_bridge", "missing") => missing_receipt_rate_bridge_count += 1,
                _ => {}
            }
            if gate_id == "public_explainer" {
                if status != "complete" {
                    return Err(format!(
                        "{lane_id}: Wave C requires public_explainer complete"
                    ));
                }
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("public explainer evidence paths")?;
                if !evidence_paths
                    .iter()
                    .any(|path| path.as_str() == Some(PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH))
                {
                    return Err(format!(
                        "{lane_id}: public_explainer evidence must include Wave C promotion"
                    ));
                }
                complete_public_explainer_count += 1;
            }
            if gate_id == "outcome_floors" {
                if status != "partial" {
                    return Err(format!(
                        "{lane_id}: Wave D requires outcome_floors partial until values pass"
                    ));
                }
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("outcome floor evidence paths")?;
                if !evidence_paths.iter().any(|path| {
                    path.as_str() == Some(OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}: outcome_floors evidence must include Wave D readiness"
                    ));
                }
                let blocked_outputs = gate
                    .get("blocked_outputs")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("outcome floor blocked outputs")?;
                if blocked_outputs.is_empty() {
                    return Err(format!(
                        "{lane_id}: outcome_floors must keep value outputs blocked"
                    ));
                }
                partial_outcome_floor_context_count += 1;
            }
            if gate_id == "policy_scenarios" {
                if status != "partial" {
                    return Err(format!(
                        "{lane_id}: Wave E requires policy_scenarios partial until packs pass"
                    ));
                }
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("policy scenario evidence paths")?;
                if !evidence_paths.iter().any(|path| {
                    path.as_str() == Some(LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}: policy_scenarios evidence must include Wave E readiness"
                    ));
                }
                if !evidence_paths
                    .iter()
                    .any(|path| path.as_str() == Some(WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH))
                {
                    return Err(format!(
                        "{lane_id}: policy_scenarios evidence must include Wave E reference packs"
                    ));
                }
                let blocked_outputs = gate
                    .get("blocked_outputs")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("policy scenario blocked outputs")?;
                if blocked_outputs.is_empty() {
                    return Err(format!(
                        "{lane_id}: policy_scenarios must keep scenario outputs blocked"
                    ));
                }
                partial_policy_scenario_requirement_count += 1;
            }
            if gate_id == "current_law_baseline"
                && matches!(
                    lane_id.as_str(),
                    "health-medicare"
                        | "social-security"
                        | "national-defense"
                        | "income-security-family"
                        | "veterans"
                        | "transportation-infrastructure"
                        | "education-workforce"
                        | "justice-courts-public-safety"
                        | "agriculture"
                )
            {
                if status != "partial" {
                    return Err(format!(
                        "{lane_id}: CBO category context gate must stay partial"
                    ));
                }
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("current-law baseline evidence paths")?;
                if !evidence_paths.iter().any(|path| {
                    path.as_str() == Some(CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}: current_law_baseline evidence must include CBO major outlay category context"
                    ));
                }
            }
            if lane_id == "revenue-solvency"
                && matches!(
                    gate_id.as_str(),
                    "current_law_baseline" | "receipt_rate_bridge"
                )
            {
                if status != "partial" {
                    return Err(format!(
                        "{lane_id}:{gate_id}: CBO revenue detail context gate must stay partial"
                    ));
                }
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("revenue-solvency evidence paths")?;
                if !evidence_paths.iter().any(|path| {
                    path.as_str() == Some(CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include CBO revenue detail context"
                    ));
                }
                if !evidence_paths.iter().any(|path| {
                    path.as_str() == Some(OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include OMB receipt category FY2025-FY2031 context"
                    ));
                }
                if gate_id == "current_law_baseline"
                    && !evidence_paths.iter().any(|path| {
                        path.as_str()
                            == Some(OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH)
                    })
                {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include OMB Table 2.2 FY2025-FY2031 receipt share context"
                    ));
                }
                if gate_id == "current_law_baseline"
                    && !evidence_paths.iter().any(|path| {
                        path.as_str()
                        == Some(
                            OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH,
                        )
                    })
                {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include OMB FY2025-FY2031 amount/share reconciliation context"
                    ));
                }
                if gate_id == "receipt_rate_bridge"
                    && !evidence_paths.iter().any(|path| {
                        path.as_str()
                            == Some(IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_JSON_PATH)
                    })
                {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include IRS SOI individual income base context"
                    ));
                }
            }
            if matches!(
                lane_id.as_str(),
                "health-medicare"
                    | "social-security"
                    | "transportation-infrastructure"
                    | "revenue-solvency"
            ) && matches!(
                gate_id.as_str(),
                "current_law_baseline" | "receipt_rate_bridge"
            ) {
                let evidence_paths = gate
                    .get("evidence_paths")
                    .and_then(serde_json::Value::as_array)
                    .ok_or("OMB Table 2.4 matrix evidence paths")?;
                if !evidence_paths.iter().any(|path| {
                    path.as_str()
                        == Some(OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH)
                }) {
                    return Err(format!(
                        "{lane_id}:{gate_id}: evidence must include OMB Table 2.4 FY2025-FY2031 receipt detail context"
                    ));
                }
            }
        }
        if row
            .get("lane_full_coverage_complete")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        {
            complete_lane_count += 1;
        }
    }
    if observed_lanes != expected_lanes {
        return Err("lane full coverage matrix lane set failed".to_string());
    }
    if complete_lane_count != 0 {
        return Err("lane full coverage matrix must not mark any lane complete yet".to_string());
    }
    if complete_public_explainer_count != 15 {
        return Err(
            "lane full coverage matrix must mark 15 public explainers complete".to_string(),
        );
    }
    if partial_outcome_floor_context_count != 15 {
        return Err(
            "lane full coverage matrix must mark 15 outcome floor gates partial".to_string(),
        );
    }
    if partial_policy_scenario_requirement_count != 15 {
        return Err(
            "lane full coverage matrix must mark 15 policy scenario gates partial".to_string(),
        );
    }

    let aggregate = matrix
        .get("aggregate_status")
        .ok_or("lane full coverage aggregate_status")?;
    if int_field(aggregate, "lanes_with_partial_current_law_baseline")?
        != partial_current_law_count as i64
        || int_field(aggregate, "lanes_with_partial_source_custody")?
            != partial_source_custody_count as i64
    {
        return Err("lane full coverage current-law/source aggregate failed".to_string());
    }
    if int_field(
        aggregate,
        "lanes_with_partial_outcome_floor_baseline_context",
    )? != 15
        || int_field(aggregate, "lanes_with_source_custodied_anchor_thresholds")? != 15
        || int_field(aggregate, "lanes_with_source_custodied_anchor_baselines")? != 15
        || int_field(aggregate, "lanes_with_complete_outcome_floor_values")? != 0
    {
        return Err("lane full coverage outcome floor aggregate failed".to_string());
    }
    if int_field(
        aggregate,
        "lanes_with_partial_policy_scenario_component_requirements",
    )? != 15
        || int_field(aggregate, "lanes_with_ready_policy_scenario_packs")? != 15
        || int_field(aggregate, "lanes_with_wave_e_reference_policy_values")? != 15
        || int_field(aggregate, "lanes_with_wave_e_reference_stress_values")? != 15
        || int_field(aggregate, "lanes_with_wave_e_comparator_results")? != 15
        || int_field(aggregate, "lanes_with_ready_reform_scenario_packs")? != 0
    {
        return Err("lane full coverage policy scenario aggregate failed".to_string());
    }
    if int_field(aggregate, "lanes_with_partial_transition_model")?
        != partial_transition_model_count as i64
        || int_field(aggregate, "lanes_missing_transition_model")?
            != missing_transition_model_count as i64
        || int_field(aggregate, "lanes_with_partial_solver_mapping")?
            != partial_solver_mapping_count as i64
        || int_field(aggregate, "lanes_missing_solver_mapping")?
            != missing_solver_mapping_count as i64
        || int_field(aggregate, "lanes_with_partial_receipt_rate_bridge")?
            != partial_receipt_rate_bridge_count as i64
        || int_field(aggregate, "lanes_missing_receipt_rate_bridge")?
            != missing_receipt_rate_bridge_count as i64
    {
        return Err("lane full coverage downstream gate aggregate failed".to_string());
    }
    if int_field(aggregate, "wave_f_prerequisites_ready")? != 0
        || int_field(aggregate, "wave_f_calibration_interfaces_ready")? != 10
        || aggregate
            .get("deterministic_solver_dry_run_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || int_field(aggregate, "lanes_solver_ready")? != 0
        || int_field(aggregate, "public_rates_ready")? != 0
        || int_field(aggregate, "public_rate_cards_ready")? != 0
    {
        return Err("lane full coverage Wave F aggregate failed".to_string());
    }

    let special_handling = matrix
        .get("special_lane_handling")
        .ok_or("lane full coverage special_lane_handling")?;
    if special_handling
        .get("revenue-solvency")
        .and_then(serde_json::Value::as_str)
        != Some("non_additive_overlay")
        || special_handling
            .get("payment-integrity")
            .and_then(serde_json::Value::as_str)
            != Some("non_additive_overlay")
        || special_handling
            .get("net-interest")
            .and_then(serde_json::Value::as_str)
            != Some("endogenous")
    {
        return Err("lane full coverage special lane handling failed".to_string());
    }

    let claims = matrix
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("lane full coverage claim_booleans")?;
    if claims
        .get("lane_full_coverage_matrix_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("lane full coverage matrix published flag must be true".to_string());
    }
    for field in [
        "all_lanes_full_coverage_complete",
        "solver_ready",
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
            return Err(format!("lane full coverage claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(LANE_FULL_COVERAGE_MATRIX_READER_PATH))
        .map_err(|err| format!("failed to read {LANE_FULL_COVERAGE_MATRIX_READER_PATH}: {err}"))?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for phrase in [
        LANE_FULL_COVERAGE_MATRIX_JSON_PATH,
        "exactly 15 analytical lanes",
        "nine gates",
        "Current aggregate gate counts",
        "15 lanes have partial current-law baseline coverage",
        "15 lanes have partial source-custody coverage",
        "Transition models remain partial for 2 lanes and missing for 13 lanes",
        "Solver mapping remains partial for 7 lanes and missing for 8 lanes",
        "Receipt/rate bridge coverage remains partial for 4 lanes and missing for 11 lanes",
        "Transportation is the deepest pilot",
        "Revenue solvency and payment integrity are non-additive overlays.",
        "Net interest is endogenous.",
        "Wave D now completes its lane-anchor contract",
        "all 15 lanes have one source-custodied anchor threshold and baseline",
        "not full component-floor completion",
        "Wave E now completes one current-policy continuation reference calibration",
        "all 15 Wave E reference scenario packs are ready",
        "No lane has a ready reform scenario or federal effect",
        "Wave F now records its completed deterministic calibration",
        "not substantive solver/rate readiness",
        "zero ready substantive Wave F prerequisites",
        "one deterministic transportation dry run",
        "does not make missing solver mapping or receipt/rate bridge gates partial",
        "not solver-ready",
        "not rate-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader_words.contains(phrase) {
            return Err(format!(
                "lane full coverage matrix reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

