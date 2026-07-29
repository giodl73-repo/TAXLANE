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

pub(crate) fn validate_international_comparator_target_rubric(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH}: {err}")
        })?;
    let rubric: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH}: {err}")
    })?;

    if string_field(&rubric, "record_id")?
        != "comparison-method:international-comparator-target-rubric:v1"
        || string_field(&rubric, "record_family")? != "international_comparator_target_rubric"
        || string_field(&rubric, "as_of_date")? != "2026-07-17"
        || string_field(&rubric, "reference_country")? != "USA"
        || string_field(&rubric, "public_rule")?
            != "Taxlane treats the peer median as the typical benchmark, the favorable quartile as an attainable performance reference only when comparability and outcomes support it, and sustained high performers as examples to study. It does not assume the best country, the 85th percentile, or the OECD average is the right United States target."
    {
        return Err(
            "international comparator rubric identity, date, or public rule failed".to_string(),
        );
    }

    let universes = rubric
        .get("comparator_universes")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs comparator_universes")?;
    let expected_universes: BTreeSet<String> = [
        "broad_reference_panel",
        "core_display_panel",
        "outcome_qualified_panel",
        "policy_commitment_group",
        "structural_transfer_panel",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let mut observed_universes = BTreeSet::new();
    for universe in universes {
        let universe_id = string_field(universe, "universe_id")?;
        if !observed_universes.insert(universe_id.clone()) {
            return Err(format!("duplicate comparator universe {universe_id}"));
        }
        if string_field(universe, "rule")?.is_empty() {
            return Err(format!("{universe_id}: universe rule is required"));
        }
    }
    if universes.len() != 5 || observed_universes != expected_universes {
        return Err("international comparator rubric exact universe set failed".to_string());
    }
    let broad = universes
        .iter()
        .find(|row| string_field(row, "universe_id").as_deref() == Ok("broad_reference_panel"))
        .ok_or("international comparator rubric needs broad_reference_panel")?;
    if !string_field(broad, "rule")?.contains("exclude it from the peer statistic")
        || string_field(broad, "weighting")? != "equal_country_default"
    {
        return Err(
            "broad reference panel must exclude the United States and use equal-country weighting"
                .to_string(),
        );
    }

    let sample = rubric
        .get("sample_rules")
        .ok_or("international comparator rubric needs sample_rules")?;
    for (field, expected) in [
        ("display_only_minimum_n", 3.0),
        ("median_claim_minimum_n", 8.0),
        ("quartile_claim_minimum_n", 10.0),
        ("p85_or_frontier_minimum_n", 20.0),
        ("minimum_eligible_universe_coverage_rate", 0.70),
        ("p85_minimum_eligible_universe_coverage_rate", 0.80),
    ] {
        if (number_field(sample, field)? - expected).abs() > f64::EPSILON {
            return Err(format!(
                "international comparator sample rule {field} failed"
            ));
        }
    }
    for field in [
        "no_imputation_for_rank_or_percentile",
        "reference_country_excluded_from_peer_statistic",
    ] {
        if sample.get(field).and_then(|value| value.as_bool()) != Some(true) {
            return Err(format!(
                "international comparator sample rule {field} must be true"
            ));
        }
    }

    let expected_roles = BTreeMap::from([
        ("best_observed_country", "target_prohibited"),
        ("context_median_iqr", "default_descriptive_reference"),
        ("favorable_quartile", "conditional_scenario_anchor"),
        ("matched_peer_range", "separate_context_reference"),
        ("p85_stretch", "normally_blocked"),
        ("policy_band", "policy_context_only"),
        ("sustained_exemplar", "conditional_case_anchor"),
    ]);
    let roles = rubric
        .get("statistic_roles")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs statistic_roles")?;
    let observed_roles: BTreeMap<&str, &str> = roles
        .iter()
        .map(|row| {
            Ok((
                row.get("role")
                    .and_then(|value| value.as_str())
                    .ok_or("statistic role needs role")?,
                row.get("status")
                    .and_then(|value| value.as_str())
                    .ok_or("statistic role needs status")?,
            ))
        })
        .collect::<Result<_, &str>>()?;
    if roles.len() != 7 || observed_roles != expected_roles {
        return Err(
            "international comparator exact statistic roles or status boundaries failed"
                .to_string(),
        );
    }

    let quantiles = rubric
        .get("quantile_rules")
        .ok_or("international comparator rubric needs quantile_rules")?;
    if string_field(quantiles, "default_method")? != "linear_interpolation_type_7"
        || string_field(quantiles, "required_sensitivity_method")? != "nearest_rank"
        || !string_field(quantiles, "small_sample_rule")?.contains("leave-one-country-out")
        || !string_field(quantiles, "small_sample_rule")?.contains("indeterminate")
        || !string_field(quantiles, "ties_rule")?.contains("rank bands")
    {
        return Err(
            "international comparator quantile methods or sensitivities failed".to_string(),
        );
    }

    let directionality = rubric
        .get("directionality_classes")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs directionality_classes")?;
    let expected_directions = BTreeSet::from([
        "definition_fragmented",
        "higher_beneficial",
        "input_no_inherent_direction",
        "lower_beneficial_with_floors",
        "target_band_or_non_monotonic",
    ]);
    let observed_directions: BTreeSet<&str> = directionality
        .iter()
        .map(|row| {
            row.get("class")
                .and_then(|value| value.as_str())
                .ok_or("directionality class needs class")
        })
        .collect::<Result<_, _>>()?;
    if directionality.len() != 5 || observed_directions != expected_directions {
        return Err("international comparator exact directionality classes failed".to_string());
    }

    let gates = rubric
        .get("admissibility_gates")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs admissibility_gates")?;
    let expected_gates: BTreeSet<String> = (1..=7).map(|n| format!("A{n}")).collect();
    let mut observed_gates = BTreeSet::new();
    for gate in gates {
        let gate_id = string_field(gate, "gate")?;
        let prefix = gate_id.split('_').next().unwrap_or_default().to_string();
        if !observed_gates.insert(prefix) {
            return Err(format!("duplicate admissibility gate {gate_id}"));
        }
        let requires = gate
            .get("requires")
            .and_then(|value| value.as_array())
            .ok_or_else(|| format!("{gate_id}: requires must be an array"))?;
        if requires.is_empty() || requires.iter().any(|value| value.as_str().is_none()) {
            return Err(format!("{gate_id}: requirements must be nonempty strings"));
        }
    }
    if gates.len() != 7 || observed_gates != expected_gates {
        return Err("international comparator exact A1-A7 gate sequence failed".to_string());
    }

    let claims = rubric
        .get("claim_ladder")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs claim_ladder")?;
    let expected_claims: BTreeSet<String> = (0..=6).map(|n| format!("G{n}")).collect();
    let observed_claims: BTreeSet<String> = claims
        .iter()
        .map(|row| string_field(row, "claim_gate"))
        .collect::<Result<_, _>>()?;
    if claims.len() != 7 || observed_claims != expected_claims {
        return Err("international comparator exact G0-G6 claim ladder failed".to_string());
    }
    let g6 = claims
        .iter()
        .find(|row| string_field(row, "claim_gate").as_deref() == Ok("G6"))
        .ok_or("international comparator rubric needs G6")?;
    if string_field(g6, "label")? != "efficiency_or_savings"
        || !string_field(g6, "maximum_claim")?
            .contains("country comparisons alone can never open this gate")
    {
        return Err("international comparator G6 closing boundary failed".to_string());
    }

    let expected_lanes = BTreeSet::from([
        "agriculture",
        "disaster-resilience",
        "education-workforce",
        "health-medicare",
        "income-security-family",
        "international-affairs",
        "justice-courts-public-safety",
        "national-defense",
        "net-interest",
        "payment-integrity",
        "revenue-solvency",
        "science-energy-environment",
        "social-security",
        "transportation-infrastructure",
        "veterans",
    ]);
    let lanes = rubric
        .get("lane_rules")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs lane_rules")?;
    let observed_lanes: BTreeSet<&str> = lanes
        .iter()
        .map(|row| {
            row.get("lane_id")
                .and_then(|value| value.as_str())
                .ok_or("lane rule needs lane_id")
        })
        .collect::<Result<_, _>>()?;
    if lanes.len() != 15 || observed_lanes != expected_lanes {
        return Err("international comparator exact 15 lane IDs failed".to_string());
    }
    let lane = |id: &str| {
        lanes
            .iter()
            .find(|row| row.get("lane_id").and_then(|value| value.as_str()) == Some(id))
            .ok_or_else(|| format!("missing international comparator lane {id}"))
    };
    for id in ["payment-integrity", "veterans"] {
        if string_field(lane(id)?, "default_role")? != "structured_case_only"
            || string_field(lane(id)?, "directionality")? != "definition_fragmented"
        {
            return Err(format!(
                "{id}: must remain structured-case-only and definition-fragmented"
            ));
        }
    }
    if string_field(lane("national-defense")?, "default_role")? != "policy_band" {
        return Err("national-defense must use policy-band context".to_string());
    }
    for id in [
        "agriculture",
        "international-affairs",
        "science-energy-environment",
    ] {
        if string_field(lane(id)?, "default_role")? != "component_scorecards" {
            return Err(format!("{id}: must preserve component scorecards"));
        }
    }
    if !string_field(lane("health-medicare")?, "rule")?
        .contains("No spending divided by one outcome as efficiency")
        || !string_field(lane("education-workforce")?, "rule")?
            .contains("No PISA-only champion or spending/test-score efficiency ratio")
    {
        return Err(
            "international comparator spending-efficiency lane boundaries failed".to_string(),
        );
    }

    let expected_prohibited = BTreeSet::from([
        "calling_the_oecd_mean_optimal",
        "changing_metric_direction_after_viewing_results",
        "collapsing_unlike_components_into_an_opaque_composite",
        "converting_cross_country_gaps_into_waste_fraud_or_recoverable_savings",
        "copying_the_best_country_as_a_target",
        "dividing_spending_by_one_outcome_and_calling_it_efficiency",
        "hiding_mixed_years_or_provisional_values",
        "mixing_us_federal_scope_with_peer_general_government_scope",
        "selecting_good_countries_after_viewing_the_focal_result",
        "selecting_high_performers_on_the_same_single_outcome_used_to_validate_them",
        "treating_missing_as_zero_or_imputing_a_rank",
        "using_p85_on_ten_or_eleven_peers",
    ]);
    let prohibited: BTreeSet<&str> = rubric
        .get("prohibited_patterns")
        .and_then(|value| value.as_array())
        .ok_or("international comparator rubric needs prohibited_patterns")?
        .iter()
        .map(|value| value.as_str().ok_or("prohibited pattern must be a string"))
        .collect::<Result<_, _>>()?;
    if prohibited != expected_prohibited {
        return Err("international comparator exact prohibited patterns failed".to_string());
    }

    for (field, expected) in [
        (
            "comparability_status",
            "method_defined_no_existing_claim_gate_opened",
        ),
        (
            "ranking_status",
            "blocked_until_metric_specific_application",
        ),
        ("efficiency_status", "not_causal_from_country_comparisons"),
        ("fraud_status", "not_measured_not_inferred"),
        ("savings_status", "blocked_not_scored"),
        ("status", "draft-method-reviewed"),
    ] {
        if string_field(&rubric, field)? != expected {
            return Err(format!(
                "international comparator closing gate {field} failed"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_international_affairs_outcome_floor_definition_packet(
    root: &Path,
) -> Result<(), String> {
    for path in [
        INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_SCHEMA_PATH,
        INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing international-affairs outcome floor definition packet artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "international-affairs-outcome-floor-definition-packet:v1"
        || string_field(&record, "record_family")?
            != "international_affairs_outcome_floor_definition_packet"
        || int_field(&record, "pulse")? != 175
        || string_field(&record, "lane_id")? != "international-affairs"
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "outcome_floor_thresholds_gap_path")?
            != OUTCOME_FLOOR_THRESHOLDS_GAP_JSON_PATH
        || string_field(&record, "agriculture_outcome_floor_definition_packet_path")?
            != AGRICULTURE_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH
        || string_field(&record, "international_affairs_depth_card_path")?
            != INTERNATIONAL_DEPTH_CARD_JSON_PATH
        || string_field(
            &record,
            "international_financial_programs_account_bridge_path",
        )? != INTERNATIONAL_FINANCIAL_BRIDGE_JSON_PATH
        || string_field(&record, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("international-affairs floor definition packet identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("international-affairs floor source custody status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "definition_packet_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "international-affairs floor status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "component_specific_policy_paths_ready",
        "commitment_outlay_bridge_ready",
        "recipient_instrument_purpose_decomposition_ready",
        "matched_peer_scope_ready",
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
                "international-affairs floor status {field} must be false"
            ));
        }
    }

    let policy = record
        .get("definition_policy")
        .and_then(serde_json::Value::as_object)
        .ok_or("international-affairs floor definition policy")?;
    for field in [
        "diplomacy_oda_humanitarian_security_and_financial_instruments_must_remain_separate",
        "negative_financial_program_entry_is_accounting_not_savings",
        "foreign_military_sales_customer_deposits_are_not_negative_security_assistance",
        "single_gdp_division_is_not_component_performance_score",
        "all_lower_cost_scenarios_must_pass_floors",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "named_floor_concepts_are_not_threshold_values",
        "international_differences_not_savings",
        "no_fraud_inference",
    ] {
        if policy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "international-affairs floor policy {field} must be true"
            ));
        }
    }

    let classes = record
        .get("required_floor_classes")
        .and_then(serde_json::Value::as_array)
        .ok_or("international-affairs required floor classes")?;
    let expected_classes = [
        "access_coverage",
        "quality_safety",
        "equity_distribution",
        "adequacy_resilience",
        "fiscal_delivery_feasibility",
    ];
    if classes.len() != expected_classes.len() {
        return Err("international-affairs required floor class count failed".to_string());
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
        return Err("international-affairs required floor class set failed".to_string());
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
                    "international-affairs floor class {field} must be null"
                ));
            }
        }
        if row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
            || string_field(row, "review_status")? != "definition_only_not_thresholded"
        {
            return Err("international-affairs floor class must remain unpassed".to_string());
        }
    }

    let lane_floors = record
        .get("international_affairs_specific_floor_definitions")
        .and_then(serde_json::Value::as_array)
        .ok_or("international-affairs-specific floor definitions")?;
    let expected_lane_floors = [
        "diplomacy_consular_presence",
        "oda_development_effectiveness",
        "humanitarian_response",
        "security_assistance_controls",
        "financial_instrument_risk_accounting",
        "component_commitment_outlay_delivery_feasibility",
    ];
    if lane_floors.len() != expected_lane_floors.len() {
        return Err("international-affairs-specific floor count failed".to_string());
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
        return Err("international-affairs-specific floor set failed".to_string());
    }
    for row in lane_floors {
        if row.get("threshold_value") != Some(&serde_json::Value::Null)
            || row.get("observed_value") != Some(&serde_json::Value::Null)
            || row.get("passed").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err(
                "international-affairs-specific floors must remain null and unpassed".to_string(),
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
        .ok_or("international-affairs floor summary")?;
    if summary
        .get("floor_classes")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("international_affairs_specific_floors")
            .and_then(serde_json::Value::as_i64)
            != Some(6)
        || summary
            .get("component_paths_required")
            .and_then(serde_json::Value::as_i64)
            != Some(6)
    {
        return Err("international-affairs floor summary counts failed".to_string());
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
                "international-affairs floor summary {field} must be false"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("international-affairs floor claims")?;
    if claims
        .get("definition_packet_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("international-affairs floor packet publication flag failed".to_string());
    }
    for field in [
        "component_specific_policy_paths_ready",
        "diplomacy_component_path_ready",
        "oda_development_component_path_ready",
        "humanitarian_component_path_ready",
        "security_assistance_component_path_ready",
        "foreign_information_exchange_component_path_ready",
        "financial_instrument_component_path_ready",
        "commitment_to_outlay_bridge_ready",
        "recipient_instrument_purpose_decomposition_ready",
        "matched_peer_scope_ready",
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
                "international-affairs floor claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        INTERNATIONAL_AFFAIRS_OUTCOME_FLOOR_DEFINITION_PACKET_JSON_PATH,
        "This international-affairs floor packet defines required floor concepts, but it does not set threshold values or pass/fail findings.",
        "The negative international financial-program entry is trust-fund, collection, and credit accounting; it is not negative diplomacy, negative foreign aid, fraud recovery, or automatic savings.",
        "Diplomacy, ODA/development, humanitarian response, security assistance, information/exchange, and financial instruments must remain separate.",
        "No lower-cost international-affairs scenario is admissible until diplomacy/consular presence, ODA/development effectiveness, humanitarian response, security-assistance controls, financial-instrument risk/accounting, component-commitment-outlay, equity, adequacy/resilience, and delivery-feasibility floors are thresholded, sourced, reviewed, and passed.",
        "No target cost, federal effect, gross savings, net savings, solver input, department-cut instruction, technology-savings claim, or balanced-budget claim is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not outcome-floor passage",
        "not a diplomacy path",
        "not an ODA path",
        "not a humanitarian path",
        "not a security-assistance path",
        "not a financial-instrument path",
        "not a commitment-to-outlay bridge",
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
                "international-affairs floor reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_international_depth_card(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(INTERNATIONAL_DEPTH_CARD_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let parts = card
        .get("components")
        .and_then(|v| v.as_array())
        .ok_or("international components")?;
    let sum: f64 = parts
        .iter()
        .map(|v| number_field(v, "outlays_millions"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let financial = parts
        .iter()
        .find(|v| v.get("subfunction_code").and_then(|x| x.as_str()) == Some("155"))
        .ok_or("financial component")?;
    if parts.len() != 5
        || sum != 45_171.0
        || number_field(financial, "outlays_millions")? != -14_936.0
        || financial.get("accounting_caveat").is_none()
    {
        return Err("international depth boundary failed".to_string());
    }
    let reader = fs::read_to_string(root.join(INTERNATIONAL_DEPTH_CARD_READER_PATH))
        .map_err(|e| e.to_string())?;
    if !reader.contains(INTERNATIONAL_DEPTH_CARD_JSON_PATH)
        || !reader.contains("negative diplomacy")
    {
        return Err("international reader boundary failed".to_string());
    }

    let bridge_text = fs::read_to_string(root.join(INTERNATIONAL_FINANCIAL_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let bridge: serde_json::Value =
        serde_json::from_str(&bridge_text).map_err(|e| e.to_string())?;
    let account_rows = bridge
        .get("account_rows")
        .and_then(|v| v.as_array())
        .ok_or("international financial bridge account rows")?;
    let account_sum: f64 = account_rows
        .iter()
        .map(|v| number_field(v, "amount"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    let groups = bridge
        .get("explanatory_groups")
        .and_then(|v| v.as_array())
        .ok_or("international financial bridge groups")?;
    let group_sum: f64 = groups
        .iter()
        .map(|v| number_field(v, "amount"))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    if account_rows.len() != 10
        || account_sum != -14_936.0
        || group_sum != -14_936.0
        || number_field(&bridge, "historical_table_total")? != -14_936.0
        || number_field(&bridge, "public_budget_database_total")? != -14_936.0
        || number_field(&bridge, "reconciliation_difference")? != 0.0
        || string_field(&bridge, "bridge_status")? != "exact_account_reconciliation_complete"
        || string_field(&bridge, "savings_status")? != "blocked_not_scored"
    {
        return Err("international financial account bridge failed".to_string());
    }
    let bridge_reader = fs::read_to_string(root.join(INTERNATIONAL_FINANCIAL_BRIDGE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        INTERNATIONAL_FINANCIAL_BRIDGE_JSON_PATH,
        "zero reconciliation",
        "not an efficiency score",
    ] {
        if !bridge_reader.contains(required) {
            return Err(format!(
                "international financial bridge reader missing {required}"
            ));
        }
    }
    Ok(())
}

