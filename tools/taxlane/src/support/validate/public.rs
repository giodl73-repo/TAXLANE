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

pub(crate) fn validate_public_rate_card_v2_contract(root: &Path) -> Result<(), String> {
    for path in [
        PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH,
        PUBLIC_RATE_CARD_V2_CONTRACT_SCHEMA_PATH,
        PUBLIC_RATE_CARD_V2_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing public-rate-card v2 artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "public-rate-card-v2-contract:v1"
        || string_field(&contract, "record_family")? != "public_rate_card_v2_contract"
        || int_field(&contract, "pulse")? != 85
        || string_field(&contract, "adaptive_rate_system_contract_path")?
            != ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH
        || string_field(&contract, "overspending_risk_taxonomy_path")?
            != OVERSPENDING_RISK_TAXONOMY_JSON_PATH
        || string_field(&contract, "technology_transition_operating_model_path")?
            != TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err("public-rate-card v2 identity or governing paths failed".to_string());
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_external_request") {
        return Err(
            "public-rate-card v2 custody status must prohibit external requests".to_string(),
        );
    }
    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "public rate-card v2 contract, not a published public rate card",
        "statutory-rate proposal",
        "effective-rate publication",
        "department-cut instruction",
        "balanced-budget claim",
        "Not calculated and blocked are first-class public outcomes",
    ] {
        if !boundary.contains(required) {
            return Err(format!("public-rate-card v2 boundary missing {required}"));
        }
    }

    let statuses = contract
        .get("card_status_values")
        .and_then(serde_json::Value::as_array)
        .ok_or("public-rate-card v2 status values")?;
    let observed_statuses = statuses
        .iter()
        .map(|row| string_field(row, "status_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "not_calculated",
        "blocked",
        "valid_internal_only",
        "publishable_after_role_review",
    ] {
        if !observed_statuses.contains(required) {
            return Err(format!("public-rate-card v2 status missing {required}"));
        }
    }
    for status in statuses {
        let status_id = string_field(status, "status_id")?;
        let display_allowed = status
            .get("public_display_allowed")
            .and_then(serde_json::Value::as_bool)
            .ok_or("public-rate-card v2 status display flag")?;
        if (status_id == "not_calculated" || status_id == "blocked") && !display_allowed {
            return Err("not_calculated and blocked must be displayable outcomes".to_string());
        }
        if (status_id == "valid_internal_only" || status_id == "publishable_after_role_review")
            && display_allowed
        {
            return Err(
                "unpublished internal/publishable statuses must not display yet".to_string(),
            );
        }
    }

    let fields = contract
        .get("required_card_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("public-rate-card v2 required fields")?;
    let observed_fields = fields
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_fields = BTreeSet::from([
        "lane_or_budget_row".to_string(),
        "current_law_cost".to_string(),
        "current_law_receipts_and_fund_treatment".to_string(),
        "target_cost_if_valid".to_string(),
        "assigned_base".to_string(),
        "effective_rate_if_valid".to_string(),
        "all_receipt_funding_share".to_string(),
        "residual_general_fund_requirement_share".to_string(),
        "burden".to_string(),
        "distribution_by_income".to_string(),
        "why_rate_changed".to_string(),
        "outcome_floors".to_string(),
        "technology_transition_status".to_string(),
        "overspending_risk_classification".to_string(),
        "evidence_grade".to_string(),
        "blockers".to_string(),
        "public_claim_status".to_string(),
    ]);
    if observed_fields != expected_fields {
        return Err("public-rate-card v2 required field set failed".to_string());
    }
    for field in fields {
        if field.get("required").and_then(serde_json::Value::as_bool) != Some(true)
            || !["not_calculated", "blocked"]
                .contains(&string_field(field, "initial_status")?.as_str())
            || !field.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("public-rate-card v2 fields must be required and null".to_string());
        }
    }

    let gates = contract
        .get("publication_gates")
        .and_then(serde_json::Value::as_object)
        .ok_or("public-rate-card v2 publication gates")?;
    if gates
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("public-rate-card v2 publication gates must all remain false".to_string());
    }
    for required in [
        "rate_calculation_gate_open",
        "rate_publication_gate_open",
        "role_review_complete",
        "public_language_review_complete",
        "statutory_rate_language_allowed",
        "effective_rate_language_allowed",
        "public_card_publishable",
    ] {
        if !gates.contains_key(required) {
            return Err(format!("public-rate-card v2 gate missing {required}"));
        }
    }

    let statutory_rule = string_field(&contract, "statutory_language_rule")?;
    for required in [
        "Avoid statutory-rate language",
        "rate-publication gate",
        "role review",
        "assigned-base model",
        "incidence model",
        "distribution model",
        "behavior model",
        "administration model",
        "interaction scoring",
    ] {
        if !statutory_rule.contains(required) {
            return Err(format!(
                "public-rate-card v2 statutory rule missing {required}"
            ));
        }
    }

    let display = contract
        .get("display_rules")
        .ok_or("public-rate-card v2 display rules")?;
    if string_field(display, "null_display_label")? != "not calculated"
        || string_field(display, "false_gate_display_label")? != "blocked"
        || !string_field(display, "missing_values_rule")?.contains("instead of filling a zero")
        || !string_field(display, "denominator_boundary")?.contains("not share of every tax dollar")
        || !string_field(display, "risk_language_boundary")?
            .contains("must not be described as waste")
        || !string_field(display, "technology_language_boundary")?
            .contains("must not be described as savings")
    {
        return Err("public-rate-card v2 display rules failed".to_string());
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("public-rate-card v2 outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!(
                "public-rate-card v2 output {field} must remain null"
            ));
        }
    }
    let claims = contract
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("public-rate-card v2 claim booleans")?;
    if claims
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("public-rate-card v2 claim booleans must all remain false".to_string());
    }

    let reader = fs::read_to_string(root.join(PUBLIC_RATE_CARD_V2_CONTRACT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH,
        "This is a public rate-card v2 contract, not a published public rate card",
        "statutory-rate proposal",
        "effective-rate publication",
        "technology-savings claim",
        "balanced-budget claim",
        "\"not calculated\" and \"blocked\" first-class public outcomes",
        "Missing values remain null and blocked gates remain false",
        "current-law cost",
        "target cost if valid",
        "assigned base",
        "effective rate if valid",
        "all-receipt funding share",
        "residual general-fund requirement share",
        "technology-transition status",
        "overspending-risk classification",
        "Avoid statutory-rate language unless",
        "All published public-rate-card",
        "Every claim boolean remains false.",
    ] {
        if !reader.contains(required) {
            return Err(format!("public-rate-card v2 reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_public_thesis_packet(root: &Path) -> Result<(), String> {
    for path in [
        PUBLIC_THESIS_PACKET_JSON_PATH,
        PUBLIC_THESIS_PACKET_SCHEMA_PATH,
        PUBLIC_THESIS_PACKET_READER_PATH,
        PUBLIC_THESIS_PACKET_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing public thesis packet artifact: {path}"));
        }
    }

    let text =
        fs::read_to_string(root.join(PUBLIC_THESIS_PACKET_JSON_PATH)).map_err(|e| e.to_string())?;
    let packet: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&packet, "record_id")? != "public-thesis-packet:v1"
        || string_field(&packet, "record_family")? != "public_thesis_packet"
        || int_field(&packet, "pulse")? != 88
        || string_field(&packet, "adaptive_rate_system_contract_path")?
            != ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH
        || string_field(&packet, "overspending_risk_taxonomy_path")?
            != OVERSPENDING_RISK_TAXONOMY_JSON_PATH
        || string_field(&packet, "technology_transition_operating_model_path")?
            != TECHNOLOGY_TRANSITION_OPERATING_MODEL_JSON_PATH
        || string_field(&packet, "public_rate_card_v2_contract_path")?
            != PUBLIC_RATE_CARD_V2_CONTRACT_JSON_PATH
        || string_field(&packet, "pilot_lane_selection_gate_path")?
            != PILOT_LANE_SELECTION_GATE_JSON_PATH
        || string_field(
            &packet,
            "deterministic_annual_update_simulator_contract_path",
        )? != DETERMINISTIC_ANNUAL_UPDATE_SIMULATOR_CONTRACT_JSON_PATH
        || string_field(&packet, "role_review_path")? != PUBLIC_THESIS_PACKET_ROLE_REVIEW_PATH
        || string_field(&packet, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err("public thesis packet identity or governing paths failed".to_string());
    }
    if !string_field(&packet, "source_custody_status")?.contains("no_new_external_request") {
        return Err(
            "public thesis packet custody status must prohibit external requests".to_string(),
        );
    }

    let thesis = string_field(&packet, "public_thesis")?;
    for required in [
        "calculates effective funding rates only after",
        "assigned bases",
        "behavior",
        "incidence",
        "distribution",
        "administration",
        "interactions",
        "outcome floors",
        "endogenous net interest",
        "what is blocked",
    ] {
        if !thesis.contains(required) {
            return Err(format!("public thesis missing {required}"));
        }
    }

    let boundary = string_field(&packet, "non_claim_boundary")?;
    for required in [
        "role-reviewed public thesis packet",
        "statutory-rate proposal",
        "effective-rate publication",
        "public rate card",
        "tax proposal",
        "savings estimate",
        "waste finding",
        "fraud finding",
        "department-cut instruction",
        "technology-savings claim",
        "solver result",
        "pilot selection",
        "balanced-budget claim",
    ] {
        if !boundary.contains(required) {
            return Err(format!("public thesis boundary missing {required}"));
        }
    }

    let rules = packet
        .get("public_language_rules")
        .and_then(serde_json::Value::as_array)
        .ok_or("public thesis language rules")?;
    let observed_rules = rules
        .iter()
        .map(|row| string_field(row, "rule_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_rules = BTreeSet::from([
        "overspending_risk_not_waste".to_string(),
        "improper_payment_not_fraud".to_string(),
        "technology_transition_timing".to_string(),
        "blocked_rates_are_valid_output".to_string(),
        "fairness_requires_distribution".to_string(),
        "balanced_budget_blocked".to_string(),
    ]);
    if observed_rules != expected_rules {
        return Err("public thesis language rule set failed".to_string());
    }
    for rule in rules {
        if string_field(rule, "required_phrase")?.is_empty()
            || string_field(rule, "prohibited_phrase")?.is_empty()
            || string_field(rule, "rule")?.is_empty()
        {
            return Err("public thesis language rule fields missing".to_string());
        }
    }

    let sections = packet
        .get("public_packet_sections")
        .and_then(serde_json::Value::as_array)
        .ok_or("public thesis sections")?;
    let observed_sections = sections
        .iter()
        .map(|row| string_field(row, "section_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "what_taxlane_can_show_now",
        "what_taxlane_cannot_claim_yet",
        "how_rates_eventually_update",
    ] {
        if !observed_sections.contains(required) {
            return Err(format!("public thesis section missing {required}"));
        }
    }
    for section in sections {
        if section
            .get("numeric_outputs_allowed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err("public thesis sections must prohibit numeric outputs".to_string());
        }
    }

    let review = packet
        .get("role_review_summary")
        .and_then(serde_json::Value::as_array)
        .ok_or("public thesis role review")?;
    if review.len() != 8 {
        return Err("public thesis role review must cover eight roles".to_string());
    }
    let observed_roles = review
        .iter()
        .map(|row| string_field(row, "role_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "T-1 Taxpayer Advocate",
        "T-2 Budget Accountant",
        "T-3 Source Custodian",
        "T-4 Public Goods Steward",
        "T-5 Program Beneficiary",
        "T-6 Compliance Burden",
        "T-7 Fiscal Sustainability",
        "T-8 Reform Skeptic",
    ] {
        if !observed_roles.contains(required) {
            return Err(format!("public thesis role missing {required}"));
        }
    }
    for row in review {
        let result = string_field(row, "result")?;
        if !result.starts_with("pass_with_") || string_field(row, "finding")?.is_empty() {
            return Err("public thesis role review must pass with guardrails/blockers".to_string());
        }
    }

    let blockers = packet
        .get("blocking_conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or("public thesis blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "assigned_base_models_missing",
        "behavior_incidence_distribution_administration_missing",
        "outcome_floors_missing_or_false",
        "pilot_lane_not_selected",
        "simulator_not_run",
        "public_rate_cards_not_role_reviewed",
        "fund_reserve_emergency_interest_reconciliation_missing",
        "unrounded_deficit_gap_not_zero",
    ] {
        if !blockers.contains(required) {
            return Err(format!("public thesis blocker missing {required}"));
        }
    }

    let outputs = packet
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("public thesis outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!("public thesis output {field} must remain null"));
        }
    }
    let claims = packet
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("public thesis claim booleans")?;
    if claims
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("public thesis claim booleans must all remain false".to_string());
    }

    let reader = fs::read_to_string(root.join(PUBLIC_THESIS_PACKET_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        PUBLIC_THESIS_PACKET_JSON_PATH,
        "calculates effective funding rates only after",
        "This is a role-reviewed public thesis packet, not a statutory-rate proposal",
        "overspending risk",
        "improper-payment or methodology gap is not fraud",
        "Technology is a transition path, not an automatic cut.",
        "Blocked rates and not-calculated values are first-class public outcomes.",
        "Fairness requires burden and distribution analysis",
        "balanced-budget claim remains blocked",
        "eight-role review passes this packet only as explanatory design",
        "Every output placeholder remains null. Every claim boolean remains false.",
    ] {
        if !reader.contains(required) {
            return Err(format!("public thesis reader missing {required}"));
        }
    }

    let role_review = fs::read_to_string(root.join(PUBLIC_THESIS_PACKET_ROLE_REVIEW_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        "Approved as explanatory public-language design.",
        "Not approved for statutory rates",
        "T-1 Taxpayer Advocate",
        "T-2 Budget Accountant",
        "T-3 Source Custodian",
        "T-4 Public Goods Steward",
        "T-5 Program Beneficiary",
        "T-6 Compliance Burden",
        "T-7 Fiscal Sustainability",
        "T-8 Reform Skeptic",
        "Use \"overspending risk\" rather than unsupported \"waste.\"",
        "Do not infer fraud",
        "technology is a transition path",
        "balanced-budget claim remains blocked",
    ] {
        if !role_review.contains(required) {
            return Err(format!("public thesis role review missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_public_explainer_wave_c_promotion(root: &Path) -> Result<(), String> {
    for path in [
        PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH,
        PUBLIC_EXPLAINER_WAVE_C_PROMOTION_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing public explainer Wave C artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH)).map_err(
        |err| format!("failed to read {PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH}: {err}"),
    )?;
    let promotion: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH}: {err}")
    })?;

    if string_field(&promotion, "record_id")? != "public-explainer-wave-c-promotion:v1"
        || string_field(&promotion, "record_family")? != "public_explainer_wave_c_promotion"
        || string_field(&promotion, "status")?
            != "wave_c_public_explainer_gate_complete_downstream_gates_blocked"
        || string_field(&promotion, "lane_full_coverage_matrix_path")?
            != LANE_FULL_COVERAGE_MATRIX_JSON_PATH
        || string_field(&promotion, "wave_lane_depth_scaffold_rollup_path")?
            != WAVE_LANE_DEPTH_SCAFFOLD_ROLLUP_JSON_PATH
        || string_field(&promotion, "lane_depth_explainability_tracker_path")?
            != LANE_DEPTH_EXPLAINABILITY_TRACKER_JSON_PATH
    {
        return Err("public explainer Wave C identity failed".to_string());
    }

    let completion = promotion
        .get("completion_definition")
        .and_then(serde_json::Value::as_object)
        .ok_or("public explainer Wave C completion definition")?;
    for required in [
        "uses_existing_public_depth_packets",
        "promotes_missing_lanes_to_completion_template",
        "keeps_partial_depth_cards_as_template_evidence",
        "blocked_claims_enumerated_for_every_lane",
        "no_rates_or_savings_published",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
    ] {
        if completion
            .get(required)
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err(format!(
                "public explainer Wave C completion flag failed: {required}"
            ));
        }
    }

    let required_questions = promotion
        .get("required_public_questions")
        .and_then(serde_json::Value::as_array)
        .ok_or("public explainer Wave C required questions")?;
    let expected_questions = [
        "what_it_does",
        "what_taxpayers_pay_now",
        "who_is_served_or_protected",
        "outcomes",
        "overspending_underfunding_boundary",
        "technology_transition",
        "evidence_gaps",
        "blocked_claims",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_questions = required_questions
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or("public explainer question string")
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_questions != expected_questions {
        return Err("public explainer Wave C question set failed".to_string());
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

    let rows = promotion
        .get("lane_explainers")
        .and_then(serde_json::Value::as_array)
        .ok_or("public explainer Wave C rows")?;
    if rows.len() != 15 {
        return Err("public explainer Wave C must contain 15 rows".to_string());
    }
    let mut observed_lanes = BTreeSet::new();
    for row in rows {
        let lane_id = string_field(row, "lane_id")?;
        if !observed_lanes.insert(lane_id.clone()) {
            return Err(format!("duplicate public explainer Wave C row: {lane_id}"));
        }
        if string_field(row, "public_explainer_status")? != "complete" {
            return Err(format!("{lane_id}: public explainer must be complete"));
        }
        let evidence_paths = row
            .get("evidence_paths")
            .and_then(serde_json::Value::as_array)
            .ok_or("public explainer Wave C evidence paths")?;
        if evidence_paths.is_empty() {
            return Err(format!("{lane_id}: public explainer needs evidence"));
        }
        for path in evidence_paths {
            let path = path
                .as_str()
                .ok_or("public explainer Wave C evidence path string")?;
            if !root.join(path).exists() {
                return Err(format!(
                    "{lane_id}: missing public explainer evidence {path}"
                ));
            }
        }
        let answers = row
            .get("template_answers")
            .and_then(serde_json::Value::as_object)
            .ok_or("public explainer Wave C template answers")?;
        let answer_keys = answers.keys().cloned().collect::<BTreeSet<_>>();
        if answer_keys != expected_questions {
            return Err(format!("{lane_id}: public explainer template set failed"));
        }
        for (field, value) in answers {
            if value.as_str().map_or(true, str::is_empty) {
                return Err(format!("{lane_id}: empty public explainer answer {field}"));
            }
        }
        let still_blocked = row
            .get("still_blocked_gates")
            .and_then(serde_json::Value::as_array)
            .ok_or("public explainer Wave C still blocked gates")?;
        if still_blocked.len() < 6 {
            return Err(format!("{lane_id}: too few still-blocked gates"));
        }
        for field in [
            "lane_full_coverage_complete",
            "solver_ready",
            "rate_ready",
            "savings_ready",
            "balanced_budget_ready",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!("{lane_id}: {field} must remain false"));
            }
        }
    }
    if observed_lanes != expected_lanes {
        return Err("public explainer Wave C lane set failed".to_string());
    }

    let aggregate = promotion
        .get("aggregate_status")
        .ok_or("public explainer Wave C aggregate status")?;
    if int_field(aggregate, "lane_count")? != 15
        || int_field(aggregate, "public_explainers_complete")? != 15
        || int_field(aggregate, "lane_full_coverage_complete")? != 0
        || aggregate
            .get("wave_c_done")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || aggregate
            .get("solver_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("rates_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("savings_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || aggregate
            .get("balanced_budget_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("public explainer Wave C aggregate status failed".to_string());
    }

    let claims = promotion
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("public explainer Wave C claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("public explainer claim bool")?;
        if field == "public_explainer_wave_c_promotion_published"
            || field == "wave_c_done"
            || field == "all_public_explainers_complete"
        {
            if !observed {
                return Err(format!(
                    "public explainer allowed flag {field} must be true"
                ));
            }
        } else if observed {
            return Err(format!(
                "public explainer downstream claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(PUBLIC_EXPLAINER_WAVE_C_PROMOTION_READER_PATH))
        .map_err(|err| {
            format!("failed to read {PUBLIC_EXPLAINER_WAVE_C_PROMOTION_READER_PATH}: {err}")
        })?;
    let reader_words = reader.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        PUBLIC_EXPLAINER_WAVE_C_PROMOTION_JSON_PATH,
        "Wave C is done",
        "all 15 public explainers are complete",
        "what it does",
        "what taxpayers pay now",
        "who is served or protected",
        "outcomes",
        "overspending or underfunding boundary",
        "technology transition",
        "evidence gaps",
        "blocked claims",
        "not lane-depth complete",
        "not solver-ready",
        "not rate-ready",
        "not savings-ready",
        "not balanced-budget-ready",
    ] {
        if !reader_words.contains(required) {
            return Err(format!("public explainer Wave C reader missing {required}"));
        }
    }

    Ok(())
}

