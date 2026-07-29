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

pub(crate) fn validate_wave_e_reference_scenario_packs(root: &Path) -> Result<(), String> {
    for path in [
        WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH,
        WAVE_E_REFERENCE_SCENARIO_PACKS_SCHEMA_PATH,
        WAVE_E_REFERENCE_SCENARIO_PACKS_READER_PATH,
        WAVE_E_REFERENCE_SCENARIO_PACKS_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Wave E reference scenario artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH)).map_err(
        |err| format!("failed to read {WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH}: {err}"),
    )?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH}: {err}")
    })?;
    if string_field(&record, "record_id")? != "wave-e-reference-scenario-packs:v1"
        || string_field(&record, "record_family")? != "wave_e_reference_scenario_packs"
        || string_field(&record, "status")?
            != "wave_e_complete_fifteen_lane_reference_scenario_calibrations_reform_and_fiscal_effects_blocked"
        || string_field(&record, "schema_path")? != WAVE_E_REFERENCE_SCENARIO_PACKS_SCHEMA_PATH
        || string_field(&record, "outcome_floor_wave_d_value_readiness_path")?
            != OUTCOME_FLOOR_WAVE_D_VALUE_READINESS_JSON_PATH
        || string_field(&record, "lane_scenario_pack_wave_e_readiness_path")?
            != LANE_SCENARIO_PACK_WAVE_E_READINESS_JSON_PATH
        || string_field(&record, "role_review_path")?
            != WAVE_E_REFERENCE_SCENARIO_PACKS_ROLE_REVIEW_PATH
    {
        return Err("Wave E reference scenario identity failed".to_string());
    }

    let contract = record
        .get("completion_contract")
        .and_then(serde_json::Value::as_object)
        .ok_or("Wave E reference completion contract")?;
    for field in [
        "one_reference_scenario_pack_per_lane",
        "every_required_component_has_current_policy_continuation_treatment",
        "policy_value_equals_source_custodied_wave_d_anchor",
        "adverse_stress_moves_one_reported_increment_across_threshold",
        "central_and_stress_floor_results_are_computed",
        "eight_role_review_required",
    ] {
        if contract.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Wave E reference completion rule failed: {field}"));
        }
    }

    let rows = record
        .get("lane_scenarios")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave E reference lane scenarios")?;
    if rows.len() != 15 {
        return Err("Wave E reference scenarios must contain 15 lanes".to_string());
    }
    let mut lanes = BTreeSet::new();
    for row in rows {
        let lane_id = string_field(row, "lane_id")?;
        if !lanes.insert(lane_id.clone()) {
            return Err(format!("duplicate Wave E reference lane: {lane_id}"));
        }
        if string_field(row, "scenario_id")? != format!("{lane_id}:wave-e-reference:v1")
            || string_field(row, "scenario_class")?
                != "current_policy_continuation_reference_calibration"
        {
            return Err(format!(
                "{lane_id}: Wave E reference scenario identity failed"
            ));
        }
        let components = row
            .get("required_component_paths")
            .and_then(serde_json::Value::as_array)
            .ok_or("Wave E reference components")?;
        if components.is_empty() {
            return Err(format!("{lane_id}: Wave E components must not be empty"));
        }
        for component in components {
            if string_field(component, "treatment")?
                != "maintain_current_policy_and_service_posture_for_reference_calibration"
                || !component
                    .get("fiscal_effect_millions")
                    .is_some_and(serde_json::Value::is_null)
                || bool_field(component, "scored_fiscal_effect")?
            {
                return Err(format!("{lane_id}: Wave E component boundary failed"));
            }
        }

        let anchor = row.get("anchor_floor").ok_or("Wave E reference anchor")?;
        let source_path = string_field(anchor, "source_packet_path")?;
        if !root.join(&source_path).exists()
            || string_field(anchor, "threshold_review_status")?
                != "role_reviewed_for_reference_comparator_only"
        {
            return Err(format!("{lane_id}: Wave E anchor lineage failed"));
        }
        let threshold = number_field(anchor, "threshold_value")?;
        let baseline = number_field(anchor, "baseline_value")?;
        let increment = number_field(anchor, "reporting_increment")?;
        if (threshold - baseline).abs() > 1e-9 || increment <= 0.0 {
            return Err(format!("{lane_id}: Wave E anchor values failed"));
        }

        let policy = row
            .get("policy_scenario")
            .ok_or("Wave E reference policy scenario")?;
        if string_field(policy, "policy_instrument")? != "current_policy_continuation_no_reform"
            || string_field(policy, "phase_in")? != "none_static_reference_case"
            || !string_field(policy, "behavior")?.contains("not_a_behavioral_estimate")
            || !string_field(policy, "transition_admin_cost")?.contains("not_a_zero_cost_estimate")
            || !string_field(policy, "incidence")?.contains("not_an_incidence")
            || (number_field(policy, "policy_value")? - threshold).abs() > 1e-9
        {
            return Err(format!("{lane_id}: Wave E policy reference failed"));
        }
        let score = policy
            .get("score_provenance")
            .ok_or("Wave E score provenance")?;
        if string_field(score, "source_packet_path")? != source_path
            || bool_field(score, "fiscal_score")?
            || bool_field(score, "forecast")?
        {
            return Err(format!("{lane_id}: Wave E score provenance failed"));
        }
        let policy_result = policy.get("result").ok_or("Wave E policy result")?;
        if !bool_field(policy_result, "passed")?
            || string_field(policy_result, "result_class")? != "boundary_pass"
        {
            return Err(format!("{lane_id}: Wave E policy result failed"));
        }

        let stress = row
            .get("stress_scenario")
            .ok_or("Wave E reference stress scenario")?;
        let comparator = string_field(anchor, "comparator")?;
        let expected_stress = match comparator.as_str() {
            "at_or_above" => threshold - increment,
            "at_or_below" => threshold + increment,
            _ => return Err(format!("{lane_id}: unknown comparator")),
        };
        if !bool_field(stress, "same_policy_as_reference")?
            || bool_field(stress, "forecast")?
            || bool_field(stress, "policy_recommendation")?
            || (number_field(stress, "stress_value")? - expected_stress).abs() > 1e-8
        {
            return Err(format!("{lane_id}: Wave E stress calibration failed"));
        }
        let stress_result = stress.get("result").ok_or("Wave E stress result")?;
        if bool_field(stress_result, "passed")?
            || string_field(stress_result, "result_class")? != "boundary_fail"
        {
            return Err(format!("{lane_id}: Wave E stress result failed"));
        }

        let floors = row.get("floor_results").ok_or("Wave E floor results")?;
        if !bool_field(floors, "central_reference_passed")?
            || bool_field(floors, "adverse_stress_passed")?
            || !bool_field(floors, "comparator_implementation_verified")?
        {
            return Err(format!("{lane_id}: Wave E floor result boundary failed"));
        }
        let readiness = row
            .get("readiness_status")
            .ok_or("Wave E reference readiness")?;
        for field in [
            "component_policy_path_ready",
            "policy_instrument_ready",
            "phase_in_ready",
            "behavior_boundary_ready",
            "transition_admin_cost_boundary_ready",
            "incidence_boundary_ready",
            "score_provenance_ready",
            "policy_value_ready",
            "stress_value_ready",
            "floor_results_ready",
            "reference_scenario_pack_ready",
        ] {
            if !bool_field(readiness, field)? {
                return Err(format!("{lane_id}: Wave E readiness must be true: {field}"));
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
            if bool_field(readiness, field)? {
                return Err(format!(
                    "{lane_id}: Wave E readiness must be false: {field}"
                ));
            }
        }
        let blocked = row
            .get("blocked_outputs")
            .and_then(serde_json::Value::as_object)
            .ok_or("Wave E blocked outputs")?;
        if blocked.values().any(|value| !value.is_null()) {
            return Err(format!(
                "{lane_id}: Wave E blocked outputs must remain null"
            ));
        }
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("Wave E reference aggregate")?;
    for (field, expected) in [
        ("lane_count", 15),
        ("reference_scenario_packs_ready", 15),
        ("lanes_with_complete_component_reference_paths", 15),
        ("lanes_with_policy_values", 15),
        ("lanes_with_stress_values", 15),
        ("lanes_with_floor_results", 15),
        ("central_reference_passes", 15),
        ("adverse_stress_passes", 0),
        ("adverse_stress_failures", 15),
        ("reform_scenario_packs_ready", 0),
        ("federal_effects_ready", 0),
        ("lower_cost_scenarios_admissible", 0),
    ] {
        if int_field(aggregate, field)? != expected {
            return Err(format!("Wave E reference aggregate failed: {field}"));
        }
    }
    if !bool_field(aggregate, "wave_e_done")? {
        return Err("Wave E reference completion flag must be true".to_string());
    }
    for field in [
        "solver_ready",
        "rates_ready",
        "savings_ready",
        "balanced_budget_ready",
    ] {
        if bool_field(aggregate, field)? {
            return Err(format!("Wave E downstream flag must remain false: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Wave E reference claims")?;
    for field in [
        "wave_e_reference_scenario_packs_published",
        "wave_e_done",
        "reference_policy_values_published",
        "synthetic_stress_values_published",
        "calibration_floor_results_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Wave E reference claim must be true: {field}"));
        }
    }
    for (field, value) in claims {
        if ![
            "wave_e_reference_scenario_packs_published",
            "wave_e_done",
            "reference_policy_values_published",
            "synthetic_stress_values_published",
            "calibration_floor_results_published",
        ]
        .contains(&field.as_str())
            && value.as_bool() != Some(false)
        {
            return Err(format!("Wave E public claim must remain false: {field}"));
        }
    }

    let schema = fs::read_to_string(root.join(WAVE_E_REFERENCE_SCENARIO_PACKS_SCHEMA_PATH))
        .map_err(|err| err.to_string())?;
    let reader = fs::read_to_string(root.join(WAVE_E_REFERENCE_SCENARIO_PACKS_READER_PATH))
        .map_err(|err| err.to_string())?;
    let review = fs::read_to_string(root.join(WAVE_E_REFERENCE_SCENARIO_PACKS_ROLE_REVIEW_PATH))
        .map_err(|err| err.to_string())?;
    let schema_words = schema.split_whitespace().collect::<Vec<_>>().join(" ");
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    let review_words = review.split_whitespace().collect::<Vec<_>>().join(" ");
    for phrase in [
        "current-policy continuation reference calibration",
        "synthetic adverse stress",
        "not a reform scenario or fiscal score",
    ] {
        if !schema_words.contains(phrase) {
            return Err(format!("Wave E reference schema missing: {phrase}"));
        }
    }
    for phrase in [
        "Wave E is complete under the reference-calibration contract",
        "All 15 lanes",
        "Every central reference value passes",
        "Every synthetic adverse stress value fails",
        "No lower-cost scenario is admissible",
    ] {
        if !reader_words.contains(phrase) {
            return Err(format!("Wave E reference reader missing: {phrase}"));
        }
    }
    for role in ["T-1", "T-2", "T-3", "T-4", "T-5", "T-6", "T-7", "T-8"] {
        if !review_words.contains(role) {
            return Err(format!("Wave E reference role review missing: {role}"));
        }
    }
    for phrase in [
        "Approved for current-policy continuation reference calibration",
        "does not approve complete lane floors",
        "No lower-cost scenario becomes admissible",
        "P1 blockers carried into Wave F",
    ] {
        if !review_words.contains(phrase) {
            return Err(format!("Wave E reference role review missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_wave_f_transportation_deterministic_calibration(root: &Path) -> Result<(), String> {
    for path in [
        WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH,
        WAVE_F_TRANSPORTATION_CALIBRATION_SCHEMA_PATH,
        WAVE_F_TRANSPORTATION_CALIBRATION_READER_PATH,
        WAVE_F_TRANSPORTATION_CALIBRATION_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing Wave F calibration artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH))
        .map_err(|err| format!("failed to read Wave F calibration: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse Wave F calibration: {err}"))?;
    if string_field(&record, "record_id")? != "wave-f-transportation-deterministic-calibration:v1"
        || string_field(&record, "record_family")?
            != "wave_f_transportation_deterministic_calibration"
        || string_field(&record, "status")?
            != "wave_f_complete_deterministic_transportation_calibration_only"
        || int_field(&record, "pulse")? != 230
        || string_field(&record, "schema_path")? != WAVE_F_TRANSPORTATION_CALIBRATION_SCHEMA_PATH
        || string_field(&record, "simulator_contract_path")?
            != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&record, "pilot_selection_gate_path")?
            != PILOT_LANE_SELECTION_GATE_JSON_PATH
        || string_field(&record, "wave_e_reference_path")?
            != WAVE_E_REFERENCE_SCENARIO_PACKS_JSON_PATH
        || string_field(&record, "wave_f_readiness_path")? != SOLVER_RATE_WAVE_F_READINESS_JSON_PATH
        || string_field(&record, "role_review_path")?
            != WAVE_F_TRANSPORTATION_CALIBRATION_ROLE_REVIEW_PATH
    {
        return Err("Wave F calibration identity failed".to_string());
    }

    let contract = record
        .get("completion_contract")
        .ok_or("Wave F calibration completion contract")?;
    if string_field(contract, "pilot_lane")? != "transportation-infrastructure"
        || string_field(contract, "pilot_candidate")?
            != "transportation_asset_maintenance_and_safety"
        || string_field(contract, "selection_scope")? != "deterministic_calibration_only"
        || bool_field(contract, "optimization_performed")?
    {
        return Err("Wave F calibration scope failed".to_string());
    }
    for field in [
        "baseline_modernization_and_stress_paths_present",
        "fund_accounting_exercised",
        "reserve_interface_exercised",
        "endogenous_interest_feedback_exercised",
        "floor_gate_blocks_lower_cost_recognition",
        "eight_role_review_complete",
        "wave_f_done",
    ] {
        if !bool_field(contract, field)? {
            return Err(format!(
                "Wave F calibration completion flag failed: {field}"
            ));
        }
    }

    let boundary = record
        .get("calibration_boundary")
        .ok_or("Wave F calibration boundary")?;
    if string_field(boundary, "unit")? != "synthetic_calibration_units"
        || string_field(boundary, "horizon")? != "FY2025-FY2035"
        || bool_field(boundary, "official_budget_score")?
        || bool_field(boundary, "real_reform_scenario")?
        || bool_field(boundary, "source_custodied_fiscal_path")?
        || !bool_field(boundary, "all_values_prohibited_from_policy_or_fiscal_use")?
    {
        return Err("Wave F calibration non-claim boundary failed".to_string());
    }

    let interfaces = record
        .get("interface_coverage")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave F interface coverage")?;
    let expected_interfaces = BTreeSet::from([
        "current_law_paths".to_string(),
        "source_custody".to_string(),
        "floor_values".to_string(),
        "policy_scenarios".to_string(),
        "transition_costs".to_string(),
        "receipt_bases".to_string(),
        "distribution_incidence".to_string(),
        "payment_integrity_lineage".to_string(),
        "net_interest_feedback".to_string(),
        "reserve_parameters".to_string(),
    ]);
    let observed_interfaces = interfaces
        .iter()
        .map(|row| string_field(row, "prerequisite_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_interfaces != expected_interfaces {
        return Err("Wave F calibration interface set failed".to_string());
    }
    for row in interfaces {
        if !bool_field(row, "calibration_interface_exercised")?
            || bool_field(row, "substantive_ready")?
        {
            return Err("Wave F calibration interface boundary failed".to_string());
        }
    }

    let params = record
        .get("path_parameters")
        .ok_or("Wave F calibration path parameters")?;
    let baseline = params.get("baseline").ok_or("Wave F baseline parameters")?;
    let transition = params
        .get("modernization_transition")
        .ok_or("Wave F transition parameters")?;
    let provisional = params
        .get("modernization_provisional_productivity")
        .ok_or("Wave F provisional parameters")?;
    let stress = params.get("stress").ok_or("Wave F stress parameters")?;
    let primary = |row: &serde_json::Value| -> Result<f64, String> {
        Ok(number_field(row, "gross_program_outlays")?
            + row
                .get("implementation_admin_outlays")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0)
            + row
                .get("fallback_remediation_outlays")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or(0.0))
    };
    let net_cash = |row: &serde_json::Value| -> Result<f64, String> {
        Ok(primary(row)? - number_field(row, "credited_offsetting_collections")?)
    };
    let fund_change = |row: &serde_json::Value| -> Result<f64, String> {
        Ok(number_field(row, "dedicated_receipts")?
            + number_field(row, "explicit_general_fund_transfer")?
            + number_field(row, "other_scored_fund_income")?
            - net_cash(row)?)
    };
    let rows = record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave F calibration annual rows")?;
    if rows.len() != 11 {
        return Err("Wave F calibration must cover eleven fiscal years".to_string());
    }
    for (index, row) in rows.iter().enumerate() {
        let expected_year = 2025 + index as i64;
        let modernization = if expected_year <= 2026 {
            transition
        } else {
            provisional
        };
        if int_field(row, "fiscal_year")? != expected_year
            || number_field(row, "baseline_primary_outlays")? != primary(baseline)?
            || number_field(row, "baseline_net_cash_requirement")? != net_cash(baseline)?
            || number_field(row, "baseline_fund_balance_change")? != fund_change(baseline)?
            || number_field(row, "modernization_primary_outlays")? != primary(modernization)?
            || number_field(row, "modernization_net_cash_requirement")? != net_cash(modernization)?
            || number_field(row, "modernization_fund_balance_change")?
                != fund_change(modernization)?
            || number_field(row, "stress_primary_outlays")? != primary(stress)?
            || number_field(row, "stress_net_cash_requirement")? != net_cash(stress)?
            || number_field(row, "stress_fund_balance_change")? != fund_change(stress)?
        {
            return Err(format!(
                "Wave F calibration arithmetic failed for FY{expected_year}"
            ));
        }
    }

    let floors = record.get("floor_gate").ok_or("Wave F floor gate")?;
    if bool_field(floors, "complete_floor_set_passed")?
        || bool_field(floors, "modeled_productivity_reduction_recognized")?
        || bool_field(floors, "lower_cost_scenario_admissible")?
        || !floors
            .get("target_cost")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("Wave F floor gate failed to block lower cost".to_string());
    }

    let interest = record
        .get("endogenous_interest_feedback_fixture")
        .ok_or("Wave F interest fixture")?;
    let opening_debt = number_field(interest, "opening_debt")?;
    let opening_interest = number_field(interest, "opening_interest")?;
    let rate = number_field(interest, "effective_rate")?;
    for (prefix, primary_field) in [
        ("baseline", "baseline_primary_deficit"),
        ("modernization", "modernization_primary_deficit"),
        ("stress", "stress_primary_deficit"),
    ] {
        let closing = number_field(interest, &format!("{prefix}_closing_debt"))?;
        let next_interest = number_field(interest, &format!("{prefix}_next_interest"))?;
        if (closing - (opening_debt + number_field(interest, primary_field)? + opening_interest))
            .abs()
            > 1e-9
            || (next_interest - closing * rate).abs() > 1e-9
        {
            return Err(format!(
                "Wave F endogenous interest identity failed: {prefix}"
            ));
        }
    }
    if !bool_field(interest, "net_interest_changed_endogenously")?
        || number_field(interest, "modernization_interest_delta")? <= 0.0
        || number_field(interest, "stress_interest_delta")?
            <= number_field(interest, "modernization_interest_delta")?
    {
        return Err("Wave F endogenous interest direction failed".to_string());
    }

    let assertions = record
        .get("deterministic_assertions")
        .ok_or("Wave F deterministic assertions")?;
    if int_field(assertions, "annual_row_count")? != rows.len() as i64 {
        return Err("Wave F assertion row count failed".to_string());
    }
    for field in [
        "all_primary_outlay_identities_pass",
        "all_net_cash_identities_pass",
        "all_fund_balance_identities_pass",
        "all_rounding_lines_zero",
        "interest_feedback_identity_passes",
        "lower_cost_blocking_assertion_passes",
    ] {
        if !bool_field(assertions, field)? {
            return Err(format!("Wave F deterministic assertion failed: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Wave F calibration claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("Wave F calibration claim bool")?;
        let expected = matches!(
            field.as_str(),
            "wave_f_done" | "deterministic_calibration_published"
        );
        if observed != expected {
            return Err(format!("Wave F calibration claim boundary failed: {field}"));
        }
    }

    for (path, required) in [
        (
            WAVE_F_TRANSPORTATION_CALIBRATION_SCHEMA_PATH,
            "All numeric values are synthetic calibration units",
        ),
        (
            WAVE_F_TRANSPORTATION_CALIBRATION_READER_PATH,
            "Wave F is complete as a deterministic transportation simulator calibration",
        ),
        (
            WAVE_F_TRANSPORTATION_CALIBRATION_ROLE_REVIEW_PATH,
            "Approved for deterministic simulator calibration only",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(required) {
            return Err(format!("Wave F calibration prose missing {required}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_wave_g_official_current_law_solver_spine_contract(root: &Path) -> Result<(), String> {
    for path in [
        WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH,
        WAVE_G_SOLVER_SPINE_CONTRACT_SCHEMA_PATH,
        WAVE_G_SOLVER_SPINE_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing Wave G contract artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH))
        .map_err(|err| format!("failed to read Wave G contract: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse Wave G contract: {err}"))?;
    if string_field(&record, "record_id")? != "wave-g-official-current-law-solver-spine-contract:v1"
        || string_field(&record, "record_family")?
            != "wave_g_official_current_law_solver_spine_contract"
        || string_field(&record, "status")? != "core_g_complete_official_current_law_spine_admitted"
        || string_field(&record, "core_wave_id")? != "CORE-G"
        || int_field(&record, "pulse")? != 232
        || string_field(&record, "schema_path")? != WAVE_G_SOLVER_SPINE_CONTRACT_SCHEMA_PATH
        || string_field(&record, "reader_path")? != WAVE_G_SOLVER_SPINE_CONTRACT_READER_PATH
        || string_field(&record, "completed_spine_path")? != CORE_G_SOLVER_SPINE_JSON_PATH
        || string_field(&record, "wave_f_calibration_path")?
            != WAVE_F_TRANSPORTATION_CALIBRATION_JSON_PATH
        || string_field(&record, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "current_law_path_inventory_path")?
            != CURRENT_LAW_PATH_INVENTORY_JSON_PATH
    {
        return Err("Wave G contract identity failed".to_string());
    }

    let scope = record.get("scope").ok_or("Wave G scope")?;
    if int_field(scope, "baseline_year")? != 2025
        || string_field(scope, "unit")? != "millions_usd_unrounded_source_precision"
        || string_field(scope, "grain")? != "federal_topline_current_law"
        || !bool_field(scope, "baseline_actual_bridge_must_be_explicit")?
        || !bool_field(scope, "single_projection_vintage_required")?
        || !bool_field(scope, "cross_source_stitching_without_bridge_prohibited")?
        || !bool_field(scope, "interpolation_prohibited")?
        || bool_field(scope, "optimization_allowed")?
    {
        return Err("Wave G scope boundary failed".to_string());
    }
    let required_years = scope
        .get("required_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave G required years")?;
    let observed_years = required_years
        .iter()
        .map(|value| value.as_i64().ok_or("Wave G year must be integer"))
        .collect::<Result<Vec<_>, _>>()?;
    if observed_years != (2025_i64..=2035).collect::<Vec<_>>() {
        return Err("Wave G horizon must be FY2025-FY2035".to_string());
    }

    let fields = record
        .get("required_annual_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave G annual fields")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "fiscal_year",
        "source_id",
        "source_vintage",
        "total_receipts_musd",
        "primary_outlays_musd",
        "net_interest_musd",
        "total_outlays_musd",
        "primary_deficit_musd",
        "total_deficit_musd",
        "other_financing_and_timing_musd",
        "debt_held_by_public_end_musd",
        "average_interest_rate_percent",
    ] {
        if !fields.contains(required) {
            return Err(format!("Wave G annual field missing: {required}"));
        }
    }

    let identities = record
        .get("required_identities")
        .ok_or("Wave G required identities")?;
    for field in [
        "total_outlays",
        "primary_deficit",
        "total_deficit",
        "deficit_components",
        "debt_rollforward",
        "sign_convention",
    ] {
        if string_field(identities, field)?.is_empty() {
            return Err(format!("Wave G identity missing: {field}"));
        }
    }
    if !bool_field(identities, "unrounded_arithmetic_controls")?
        || !bool_field(
            identities,
            "explicit_rounding_line_required_if_public_values_are_rounded",
        )?
    {
        return Err("Wave G rounding contract failed".to_string());
    }

    let gates = record
        .get("completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("Wave G completion gates")?;
    let expected_gates = BTreeSet::from([
        "source_custody",
        "complete_horizon",
        "vintage_control",
        "topline_reconciliation",
        "debt_rollforward",
        "interest_lineage",
        "boundary_review",
        "deterministic_validation",
    ]);
    let observed_gates = gates
        .iter()
        .map(|gate| string_field(gate, "gate_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_gates = expected_gates
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_gates != expected_gates || gates.len() != 8 {
        return Err("Wave G completion gate set failed".to_string());
    }
    for gate in gates {
        if string_field(gate, "requirement")?.is_empty() || !bool_field(gate, "ready")? {
            return Err("CORE-G contract completion gate must be ready".to_string());
        }
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("Wave G aggregate status")?;
    if int_field(aggregate, "completion_gate_count")? != 8
        || int_field(aggregate, "ready_completion_gates")? != 8
        || !bool_field(aggregate, "wave_g_defined")?
        || !bool_field(aggregate, "wave_g_done")?
        || !bool_field(aggregate, "core_g_done")?
        || !bool_field(aggregate, "official_current_law_solver_spine_ready")?
        || !bool_field(aggregate, "trn_a_may_start")?
        || bool_field(aggregate, "real_reform_admission_ready")?
        || bool_field(aggregate, "public_solver_ready")?
    {
        return Err("Wave G aggregate definition state failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Wave G claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("Wave G claim bool")?;
        let expected = matches!(
            field.as_str(),
            "wave_g_contract_published"
                | "wave_g_done"
                | "core_g_done"
                | "official_current_law_solver_spine_published"
                | "trn_a_may_start"
        );
        if observed != expected {
            return Err(format!("Wave G claim boundary failed: {field}"));
        }
    }

    for (path, phrase) in [
        (
            WAVE_G_SOLVER_SPINE_CONTRACT_SCHEMA_PATH,
            "Wave G constructs and admits one official FY2025-FY2035 current-law federal",
        ),
        (
            WAVE_G_SOLVER_SPINE_CONTRACT_READER_PATH,
            "CORE-G is complete under this contract",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("Wave G prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_wave_lane_depth_scaffold_rollup(root: &Path) -> Result<(), String> {
    for path in [
        WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_JSON_PATH,
        WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_SCHEMA_PATH,
        WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing wave scaffold rollup artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&rollup, "record_id")? != "wave-lane-depth-scaffold-rollup:v1"
        || string_field(&rollup, "record_family")? != "wave_lane_depth_scaffold_rollup"
        || int_field(&rollup, "pulse")? != 117
        || string_field(&rollup, "lane_agent_work_order_plan_path")?
            != LANE_AGENT_WORK_ORDER_PLAN_JSON_PATH
        || string_field(&rollup, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
        || string_field(&rollup, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&rollup, "balanced_rate_readiness_gate_path")?
            != BALANCED_RATE_READINESS_GATE_JSON_PATH
        || string_field(&rollup, "public_rate_card_v2_contract_path")?
            != PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH
    {
        return Err("wave scaffold rollup identity failed".to_string());
    }

    let expected_wave_paths = [
        WAVE1_PUBLIC_TOPLINE_LANE_DEPTH_PACKETS_JSON_PATH,
        WAVE2_HUMAN_SERVICES_LANE_DEPTH_PACKETS_JSON_PATH,
        WAVE3_PUBLIC_GOODS_LANE_DEPTH_PACKETS_JSON_PATH,
        WAVE4_COMPONENT_AND_PILOT_LANE_DEPTH_PACKETS_JSON_PATH,
        WAVE5_FISCAL_CONTROL_OVERLAY_DEPTH_PACKETS_JSON_PATH,
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let wave_paths = rollup
        .get("wave_packet_paths")
        .and_then(serde_json::Value::as_array)
        .ok_or("wave scaffold rollup packet paths")?
        .iter()
        .map(|value| {
            let path = value.as_str().ok_or("wave packet path string")?;
            if !root.join(path).exists() {
                return Err("referenced wave packet missing");
            }
            Ok(path.to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if wave_paths != expected_wave_paths {
        return Err("wave scaffold rollup packet path set failed".to_string());
    }

    let summary = rollup
        .get("coverage_summary")
        .ok_or("wave scaffold coverage summary")?;
    if int_field(summary, "analytical_lane_count")? != 15
        || int_field(summary, "budget_row_count")? != 17
        || int_field(summary, "waves_audited")? != 5
        || int_field(summary, "lane_or_overlay_packets_published")? != 15
        || int_field(summary, "lane_depth_complete_count")? != 0
        || int_field(summary, "public_explainability_complete_count")? != 0
        || int_field(summary, "solver_ready_count")? != 0
        || int_field(summary, "rate_ready_count")? != 0
        || summary
            .get("public_explainability_scaffold_present_for_all_15_lanes")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || summary
            .get("all_15_lanes_scaffolded")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || summary
            .get("all_15_lanes_defensible_for_public_rates")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || summary
            .get("all_15_lanes_defensible_for_solver")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || summary
            .get("balanced_budget_claim_allowed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("wave scaffold coverage summary failed".to_string());
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
    let all_lanes = rollup
        .get("all_analytical_lane_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("wave scaffold all lane ids")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("wave scaffold lane id string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if all_lanes != expected_lanes {
        return Err("wave scaffold all lane id set failed".to_string());
    }

    let covered_rows = rollup
        .get("covered_ids_by_wave")
        .and_then(serde_json::Value::as_array)
        .ok_or("wave scaffold covered ids")?;
    if covered_rows.len() != 5 {
        return Err("wave scaffold must have five covered-id rows".to_string());
    }
    let mut covered = BTreeSet::new();
    for row in covered_rows {
        let ids = row
            .get("ids")
            .and_then(serde_json::Value::as_array)
            .ok_or("wave scaffold covered row ids")?;
        if ids.len() != 3 {
            return Err("each wave scaffold row must list three ids".to_string());
        }
        for id in ids {
            covered.insert(
                id.as_str()
                    .ok_or("wave scaffold covered id string")?
                    .to_string(),
            );
        }
    }
    if covered != expected_lanes {
        return Err("wave scaffold covered lane set failed".to_string());
    }

    let gates = rollup
        .get("remaining_completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("wave scaffold remaining gates")?;
    if gates.len() < 10 {
        return Err("wave scaffold remaining gates too short".to_string());
    }
    for gate in gates {
        if string_field(gate, "status")? != "incomplete"
            || !gate.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("wave scaffold gates must remain incomplete/null".to_string());
        }
    }

    let integration = rollup
        .get("integration_review")
        .and_then(serde_json::Value::as_object)
        .ok_or("wave scaffold integration review")?;
    for required in [
        "all_five_wave_packets_exist",
        "all_15_analytical_lanes_present_once",
        "seventeen_budget_rows_not_confused_with_fifteen_analytical_lanes",
        "revenue_solvency_and_payment_integrity_remain_non_additive_overlays",
        "net_interest_remains_endogenous_and_not_directly_cuttable",
        "trust_funds_remain_separate",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "international_differences_are_not_savings",
        "improper_payment_estimates_do_not_imply_fraud",
        "technology_changes_are_transition_paths_not_automatic_savings",
    ] {
        if integration
            .get(required)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!("wave scaffold integration rule failed: {required}"));
        }
    }

    let claims = rollup
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("wave scaffold aggregate claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("wave scaffold claim bool")?;
        if field == "wave_lane_depth_scaffold_rollup_published"
            || field == "all_lane_scaffolds_present"
        {
            if !observed {
                return Err(format!("wave scaffold allowed flag {field} must be true"));
            }
        } else if observed {
            return Err(format!("wave scaffold public claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_JSON_PATH,
        "The five scaled waves now cover all 15 analytical lanes",
        "every analytical lane has a defensible public boundary",
        "It does not mean lane depth is complete.",
        "It does not mean public explainability is complete.",
        "It does not mean every lane is defensible for public rates.",
        "The 15 analytical lanes are not the same as the 17 budget rows.",
        "Revenue-solvency and payment integrity remain non-additive overlays.",
        "Net interest remains endogenous and cannot be cut directly.",
        "Trust funds remain separate.",
        "Missing values remain null and blocked gates remain false.",
        "International spending differences are not savings.",
        "Improper-payment estimates do not imply fraud.",
        "Technology changes are transition paths, not automatic savings.",
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
            return Err(format!("wave scaffold reader missing {required}"));
        }
    }

    Ok(())
}

