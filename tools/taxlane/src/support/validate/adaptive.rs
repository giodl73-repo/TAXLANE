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

pub(crate) fn validate_adaptive_rate_system_contract(root: &Path) -> Result<(), String> {
    for path in [
        ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH,
        ADAPTIVE_RATE_SYSTEM_CONTRACT_SCHEMA_PATH,
        ADAPTIVE_RATE_SYSTEM_CONTRACT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing adaptive-rate system artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let contract: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&contract, "record_id")? != "adaptive-rate-system-contract:v1"
        || string_field(&contract, "record_family")? != "adaptive_rate_system_contract"
        || int_field(&contract, "pulse")? != 82
        || string_field(&contract, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&contract, "rubric_path")?
            != INTERNATIONAL_COMPARATOR_TARGET_RUBRIC_JSON_PATH
        || string_field(&contract, "coverage_contract_path")? != GLOBAL_COUNTRY_COMPARISON_JSON_PATH
        || string_field(&contract, "balanced_rate_readiness_gate_path")?
            != BALANCED_RATE_READINESS_GATE_JSON_PATH
        || string_field(&contract, "final_closure_readiness_gate_path")?
            != FINAL_CLOSURE_READINESS_GATE_JSON_PATH
        || string_field(&contract, "phase_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
    {
        return Err("adaptive-rate system identity or governing paths failed".to_string());
    }
    for path in [
        BALANCED_RATE_READINESS_GATE_JSON_PATH,
        FINAL_CLOSURE_READINESS_GATE_JSON_PATH,
        "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md",
    ] {
        if !root.join(path).exists() {
            return Err(format!("adaptive-rate linked artifact missing {path}"));
        }
    }
    if !string_field(&contract, "source_custody_status")?.contains("no_new_external_request") {
        return Err("adaptive-rate custody status must prohibit external requests".to_string());
    }
    let boundary = string_field(&contract, "non_claim_boundary")?;
    for required in [
        "adaptive rate system contract, not a rate card",
        "Rate calculation and rate publication are separate gates",
        "All rate outputs remain null",
    ] {
        if !boundary.contains(required) {
            return Err(format!("adaptive-rate boundary missing {required}"));
        }
    }

    let lifecycle = contract
        .get("annual_update_lifecycle")
        .and_then(serde_json::Value::as_array)
        .ok_or("adaptive-rate lifecycle")?;
    if lifecycle.len() != 14 {
        return Err("adaptive-rate lifecycle must contain 14 steps".to_string());
    }
    for (index, step) in lifecycle.iter().enumerate() {
        if int_field(step, "step")? != (index + 1) as i64
            || string_field(step, "step_id")?.is_empty()
        {
            return Err("adaptive-rate lifecycle order failed".to_string());
        }
    }

    let gates = contract
        .get("rate_gate_sequence")
        .ok_or("adaptive-rate gate sequence")?;
    for gate_name in [
        "rate_calculation_gate",
        "rate_publication_gate",
        "balanced_budget_claim_gate",
    ] {
        let gate = gates
            .get(gate_name)
            .ok_or_else(|| format!("adaptive-rate missing {gate_name}"))?;
        if gate
            .get("currently_open")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
            || string_field(gate, "description")?.is_empty()
        {
            return Err(format!("adaptive-rate gate {gate_name} must remain closed"));
        }
    }

    let denominators = contract
        .get("denominator_definitions")
        .ok_or("adaptive-rate denominator definitions")?;
    let all_receipt = denominators
        .get("all_receipt_funding_share")
        .ok_or("adaptive-rate all-receipt definition")?;
    let residual = denominators
        .get("residual_general_fund_requirement_share")
        .ok_or("adaptive-rate residual definition")?;
    if string_field(all_receipt, "formula")? != "gross program cost / total funded federal cost"
        || string_field(residual, "formula")?
            != "residual general-fund need / total residual general-fund need"
        || !all_receipt
            .get("value")
            .is_some_and(serde_json::Value::is_null)
        || !residual
            .get("value")
            .is_some_and(serde_json::Value::is_null)
        || !string_field(denominators, "denominator_boundary")?
            .contains("not share of every tax dollar")
    {
        return Err("adaptive-rate denominator boundary failed".to_string());
    }

    let assigned = contract
        .get("assigned_base_required_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("adaptive-rate assigned-base fields")?;
    let observed = assigned
        .iter()
        .map(|row| string_field(row, "field_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = BTreeSet::from([
        "matched_year".to_string(),
        "legal_perimeter".to_string(),
        "economic_perimeter".to_string(),
        "baseline_amount".to_string(),
        "elasticity".to_string(),
        "avoidance_and_compliance".to_string(),
        "employer_taxpayer_agency_burden".to_string(),
        "distribution_by_income".to_string(),
        "interaction_with_other_taxes".to_string(),
        "current_law_yield".to_string(),
        "reform_yield".to_string(),
    ]);
    if observed != expected {
        return Err("adaptive-rate assigned-base field set failed".to_string());
    }
    for row in assigned {
        if string_field(row, "status")? != "missing"
            || !row.get("value").is_some_and(serde_json::Value::is_null)
        {
            return Err("adaptive-rate assigned-base fields must remain missing/null".to_string());
        }
    }

    let outputs = contract
        .get("output_placeholders")
        .and_then(serde_json::Value::as_object)
        .ok_or("adaptive-rate outputs")?;
    for (field, value) in outputs {
        if !value.is_null() {
            return Err(format!("adaptive-rate output {field} must remain null"));
        }
    }
    let guardrails = contract
        .get("guardrail_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("adaptive-rate guardrail booleans")?;
    if guardrails
        .iter()
        .any(|(_, value)| value.as_bool() != Some(false))
    {
        return Err("adaptive-rate guardrail booleans must all remain false".to_string());
    }

    let blockers = contract
        .get("explicit_blockers")
        .and_then(serde_json::Value::as_array)
        .ok_or("adaptive-rate blockers")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "assigned_base_fields_missing",
        "distributional_analysis_missing",
        "behavioral_sensitivity_missing",
        "macro_feedback_missing",
        "role_review_missing_for_public_rate_cards",
        "zero_unrounded_deficit_gap_not_demonstrated",
    ] {
        if !blockers.contains(required) {
            return Err(format!("adaptive-rate blocker missing {required}"));
        }
    }

    let reader = fs::read_to_string(root.join(ADAPTIVE_RATE_SYSTEM_CONTRACT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        ADAPTIVE_RATE_SYSTEM_CONTRACT_JSON_PATH,
        "This is an adaptive rate system contract, not a rate card, statutory-rate proposal, tax proposal, spending-cut order, waste finding, fraud finding, savings estimate, or balanced-budget claim.",
        "The contract separates rate calculation from rate publication.",
        "all-receipt funding share = gross program cost / total funded federal cost",
        "residual general-fund requirement share = residual general-fund need / total residual general-fund need",
        "A value calculated after subtracting dedicated receipts is not share of every tax dollar.",
        "All rate outputs remain null.",
        "No statutory rate, effective rate, public rate card, savings claim, waste finding, fraud finding, technology-savings claim, or balanced-budget claim",
    ] {
        if !reader.contains(required) {
            return Err(format!("adaptive-rate reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_adaptive_rate_corpus_track_plan(root: &Path) -> Result<(), String> {
    for path in [
        CORPUS_TRACK_PLAN_JSON_PATH,
        CORPUS_TRACK_PLAN_SCHEMA_PATH,
        CORPUS_TRACK_PLAN_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing corpus track plan artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(CORPUS_TRACK_PLAN_JSON_PATH))
        .map_err(|err| format!("failed to read corpus track plan: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse corpus track plan: {err}"))?;
    if string_field(&record, "record_id")? != "adaptive-rate-corpus-track-plan:v1"
        || string_field(&record, "record_family")? != "adaptive_rate_corpus_track_plan"
        || string_field(&record, "status")? != "targeted_hlt_def_spending_rate_wave_complete"
        || int_field(&record, "pulse")? != 478
        || string_field(&record, "schema_path")? != CORPUS_TRACK_PLAN_SCHEMA_PATH
        || string_field(&record, "reader_path")? != CORPUS_TRACK_PLAN_READER_PATH
        || string_field(&record, "predecessor_roadmap_path")? != POST_F_WAVE_ROADMAP_JSON_PATH
        || string_field(&record, "core_g_contract_path")? != WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH
        || string_field(&record, "trn_a_path")? != TRN_A_BASELINE_SPINE_JSON_PATH
        || string_field(&record, "core_h_path")? != CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH
        || string_field(&record, "trn_b_start_path")? != TRN_B_START_GATE_JSON_PATH
        || string_field(&record, "trn_b_closure_path")? != TRN_B_CLOSURE_JSON_PATH
        || string_field(&record, "trn_c_start_path")? != TRN_C_START_GATE_JSON_PATH
        || string_field(&record, "trn_c_closure_path")? != TRN_C_CLOSURE_JSON_PATH
        || string_field(&record, "core_i_path")? != CORE_I_REFORM_ADMISSION_JSON_PATH
        || string_field(&record, "trn_d_start_path")? != TRN_D_START_GATE_JSON_PATH
        || string_field(&record, "hlt_a_start_path")? != HLT_A_START_GATE_JSON_PATH
        || string_field(&record, "edu_a_start_path")? != EDU_A_START_GATE_JSON_PATH
        || string_field(&record, "multi_track_frontier_path")? != MULTI_TRACK_FRONTIER_JSON_PATH
        || string_field(&record, "trn_d_01_path")? != TRN_D_LEGAL_PERIMETER_JSON_PATH
        || string_field(&record, "hlt_a_01_path")? != HLT_A_PERIMETER_INVENTORY_JSON_PATH
        || string_field(&record, "edu_a_01_path")? != EDU_A_PERIMETER_INVENTORY_JSON_PATH
        || string_field(&record, "trn_d_03_path")? != TRN_D_ADMIN_BEHAVIOR_JSON_PATH
        || string_field(&record, "hlt_a_02_path")? != HLT_A_BASELINE_JSON_PATH
        || string_field(&record, "edu_a_02_path")? != EDU_A_BASELINE_JSON_PATH
        || string_field(&record, "trn_d_04_path")? != TRN_D_INCIDENCE_FAIRNESS_JSON_PATH
        || string_field(&record, "hlt_a_03_path")? != HLT_A_FINANCING_LINEAGE_JSON_PATH
        || string_field(&record, "edu_a_03_path")? != EDU_A_FINANCING_LINEAGE_JSON_PATH
        || string_field(&record, "trn_d_05_path")? != TRN_D_INTERACTIONS_BRIDGE_JSON_PATH
        || string_field(&record, "trn_d_closure_path")? != TRN_D_CLOSURE_JSON_PATH
        || string_field(&record, "hlt_a_04_path")? != HLT_A_SERVICE_FLOOR_SPINE_JSON_PATH
        || string_field(&record, "hlt_a_closure_path")? != HLT_A_CLOSURE_JSON_PATH
        || string_field(&record, "edu_a_04_path")? != EDU_A_SERVICE_FLOOR_SPINE_JSON_PATH
        || string_field(&record, "edu_a_closure_path")? != EDU_A_CLOSURE_JSON_PATH
        || string_field(&record, "trn_e_start_path")? != TRN_E_START_GATE_JSON_PATH
        || string_field(&record, "hlt_b_start_path")? != HLT_B_START_GATE_JSON_PATH
        || string_field(&record, "edu_b_start_path")? != EDU_B_START_GATE_JSON_PATH
        || string_field(&record, "trn_e_01_path")? != TRN_E_INPUT_READINESS_JSON_PATH
        || string_field(&record, "hlt_b_01_path")? != HLT_B_COMPONENT_MAPPING_JSON_PATH
        || string_field(&record, "edu_b_01_path")? != EDU_B_COMPONENT_MAPPING_JSON_PATH
        || string_field(&record, "core_j_contract_path")? != CORE_J_CONTRACT_JSON_PATH
        || string_field(&record, "core_j_closure_path")? != CORE_J_CLOSURE_JSON_PATH
        || string_field(&record, "core_k_contract_path")? != CORE_K_CONTRACT_JSON_PATH
        || string_field(&record, "core_k_closure_path")? != CORE_K_CLOSURE_JSON_PATH
        || string_field(&record, "stage_c_catchup_bundle_path")? != STAGE_C_CATCHUP_BUNDLE_JSON_PATH
        || string_field(&record, "hlt_b_closure_path")? != HLT_B_CLOSURE_JSON_PATH
        || string_field(&record, "hlt_c_closure_path")? != HLT_C_CLOSURE_JSON_PATH
        || string_field(&record, "edu_b_closure_path")? != EDU_B_CLOSURE_JSON_PATH
        || string_field(&record, "edu_c_closure_path")? != EDU_C_CLOSURE_JSON_PATH
        || string_field(&record, "oas_a_path")? != OAS_A_SPINE_JSON_PATH
        || string_field(&record, "oas_b_path")? != OAS_B_CLOSURE_JSON_PATH
        || string_field(&record, "oas_c_path")? != OAS_C_CLOSURE_JSON_PATH
        || string_field(&record, "core_l_contract_path")? != CORE_L_CONTRACT_JSON_PATH
        || string_field(&record, "core_l_closure_path")? != CORE_L_CLOSURE_JSON_PATH
        || string_field(&record, "seven_lane_catchup_bundle_path")? != SEVEN_LANE_CATCHUP_JSON_PATH
        || string_field(&record, "trn_e_closure_path")? != TRN_E_CLOSURE_JSON_PATH
        || string_field(&record, "hlt_d_closure_path")? != HLT_D_CLOSURE_JSON_PATH
        || string_field(&record, "edu_d_closure_path")? != EDU_D_CLOSURE_JSON_PATH
        || string_field(&record, "oas_d_closure_path")? != OAS_D_CLOSURE_JSON_PATH
        || string_field(&record, "isf_a_path")? != ISF_A_SPINE_JSON_PATH
        || string_field(&record, "isf_b_path")? != ISF_B_CLOSURE_JSON_PATH
        || string_field(&record, "vet_a_path")? != VET_A_SPINE_JSON_PATH
        || string_field(&record, "vet_b_path")? != VET_B_CLOSURE_JSON_PATH
        || string_field(&record, "agr_a_path")? != AGR_A_SPINE_JSON_PATH
        || string_field(&record, "agr_b_path")? != AGR_B_CLOSURE_JSON_PATH
        || string_field(&record, "isf_c_path")? != ISF_C_CLOSURE_JSON_PATH
        || string_field(&record, "vet_c_path")? != VET_C_CLOSURE_JSON_PATH
        || string_field(&record, "agr_c_path")? != AGR_C_CLOSURE_JSON_PATH
        || string_field(&record, "three_lane_stage_c_bundle_path")?
            != THREE_LANE_STAGE_C_BUNDLE_JSON_PATH
        || string_field(&record, "fifteen_lane_stage_matrix_path")?
            != FIFTEEN_LANE_STAGE_MATRIX_JSON_PATH
        || string_field(&record, "isf_d_path")? != ISF_D_CLOSURE_JSON_PATH
        || string_field(&record, "vet_d_path")? != VET_D_CLOSURE_JSON_PATH
        || string_field(&record, "agr_d_path")? != AGR_D_CLOSURE_JSON_PATH
        || string_field(&record, "three_lane_stage_d_bundle_path")?
            != THREE_LANE_STAGE_D_BUNDLE_JSON_PATH
        || string_field(&record, "def_d_path")? != DEF_D_CLOSURE_JSON_PATH
        || string_field(&record, "dis_d_path")? != DIS_D_CLOSURE_JSON_PATH
        || string_field(&record, "jus_d_path")? != JUS_D_CLOSURE_JSON_PATH
        || string_field(&record, "see_d_path")? != SEE_D_CLOSURE_JSON_PATH
        || string_field(&record, "int_d_path")? != INT_D_CLOSURE_JSON_PATH
        || string_field(&record, "pay_d_path")? != PAY_D_CLOSURE_JSON_PATH
        || string_field(&record, "rev_d_path")? != REV_D_CLOSURE_JSON_PATH
        || string_field(&record, "net_d_path")? != NET_D_CLOSURE_JSON_PATH
        || string_field(&record, "eight_lane_a_d_bundle_path")? != EIGHT_LANE_A_D_BUNDLE_JSON_PATH
        || string_field(&record, "fifteen_lane_stage_d_portfolio_closure_path")?
            != FIFTEEN_LANE_D_PORTFOLIO_JSON_PATH
        || string_field(&record, "lane_e_contract_path")? != LANE_E_CONTRACT_JSON_PATH
        || string_field(&record, "hlt_e_path")? != HLT_E_CLOSURE_JSON_PATH
        || string_field(&record, "edu_e_path")? != EDU_E_CLOSURE_JSON_PATH
        || string_field(&record, "oas_e_path")? != OAS_E_CLOSURE_JSON_PATH
        || string_field(&record, "isf_e_path")? != ISF_E_CLOSURE_JSON_PATH
        || string_field(&record, "vet_e_path")? != VET_E_CLOSURE_JSON_PATH
        || string_field(&record, "agr_e_path")? != AGR_E_CLOSURE_JSON_PATH
        || string_field(&record, "def_e_path")? != DEF_E_CLOSURE_JSON_PATH
        || string_field(&record, "dis_e_path")? != DIS_E_CLOSURE_JSON_PATH
        || string_field(&record, "jus_e_path")? != JUS_E_CLOSURE_JSON_PATH
        || string_field(&record, "see_e_path")? != SEE_E_CLOSURE_JSON_PATH
        || string_field(&record, "int_e_path")? != INT_E_CLOSURE_JSON_PATH
        || string_field(&record, "pay_e_path")? != PAY_E_CLOSURE_JSON_PATH
        || string_field(&record, "rev_e_path")? != REV_E_CLOSURE_JSON_PATH
        || string_field(&record, "net_e_path")? != NET_E_CLOSURE_JSON_PATH
        || string_field(&record, "fourteen_lane_stage_e_bundle_path")?
            != FOURTEEN_LANE_E_BUNDLE_JSON_PATH
        || string_field(&record, "fifteen_lane_stage_e_portfolio_closure_path")?
            != FIFTEEN_LANE_E_PORTFOLIO_JSON_PATH
        || string_field(&record, "lane_f_contract_path")? != LANE_F_CONTRACT_JSON_PATH
        || string_field(&record, "fifteen_lane_f_readiness_path")?
            != FIFTEEN_LANE_F_READINESS_JSON_PATH
        || string_field(&record, "fifteen_lane_two_level_f_queue_path")?
            != FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_JSON_PATH
        || string_field(&record, "trn_level_1_core_lessons_path")?
            != TRN_LEVEL_1_CORE_LESSONS_JSON_PATH
        || string_field(&record, "core_m_contract_path")? != CORE_M_CONTRACT_JSON_PATH
        || string_field(&record, "core_m_closure_path")? != CORE_M_CLOSURE_JSON_PATH
        || string_field(&record, "trn_level_1_dossier_path")? != TRN_LEVEL_1_DOSSIER_JSON_PATH
        || string_field(&record, "trn_level_2_e_rerun_path")? != TRN_LEVEL_2_E_RERUN_JSON_PATH
        || string_field(&record, "core_n_contract_path")? != CORE_N_CONTRACT_JSON_PATH
        || string_field(&record, "core_n_closure_path")? != CORE_N_CLOSURE_JSON_PATH
        || string_field(&record, "trn_f_cost_note_path")? != TRN_F_COST_NOTE_JSON_PATH
        || string_field(&record, "rev_level_1_start_path")? != REV_LEVEL_1_START_JSON_PATH
        || string_field(&record, "rev_level_1_guarded_closure_path")?
            != REV_LEVEL_1_GUARDED_CLOSURE_JSON_PATH
        || string_field(&record, "fiscally_decisive_level_1_path")?
            != FISCALLY_DECISIVE_LEVEL_1_JSON_PATH
        || string_field(&record, "fiscally_decisive_level_2_path")?
            != FISCALLY_DECISIVE_LEVEL_2_JSON_PATH
        || string_field(&record, "hlt_level_3_proxy_path")? != HLT_LEVEL_3_MA_PROXY_JSON_PATH
        || string_field(&record, "hlt_level_4_floor_audit_path")?
            != HLT_LEVEL_4_FLOOR_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_hlt_bridge_path")?
            != FISCAL_PACKAGE_HLT_BRIDGE_JSON_PATH
        || string_field(&record, "def_level_3_scale_bridge_path")?
            != DEF_LEVEL_3_SCALE_BRIDGE_JSON_PATH
        || string_field(&record, "def_level_4_allocation_audit_path")?
            != DEF_LEVEL_4_ALLOCATION_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_def_bridge_path")?
            != FISCAL_PACKAGE_DEF_BRIDGE_JSON_PATH
        || string_field(&record, "pay_level_3_control_path")? != PAY_LEVEL_3_CONTROL_JSON_PATH
        || string_field(&record, "pay_level_4_audit_path")? != PAY_LEVEL_4_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_pay_bridge_path")?
            != FISCAL_PACKAGE_PAY_BRIDGE_JSON_PATH
        || string_field(&record, "oas_level_3_bridge_path")? != OAS_LEVEL_3_BRIDGE_JSON_PATH
        || string_field(&record, "oas_level_4_audit_path")? != OAS_LEVEL_4_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_oas_bridge_path")?
            != FISCAL_PACKAGE_OAS_BRIDGE_JSON_PATH
        || string_field(&record, "net_level_3_recomputation_path")?
            != NET_LEVEL_3_RECOMPUTATION_JSON_PATH
        || string_field(&record, "net_level_4_audit_path")? != NET_LEVEL_4_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_net_bridge_path")?
            != FISCAL_PACKAGE_NET_BRIDGE_JSON_PATH
        || string_field(&record, "rev_level_2_reconciliation_path")?
            != REV_LEVEL_2_RECONCILIATION_JSON_PATH
        || string_field(&record, "rev_level_2_audit_path")? != REV_LEVEL_2_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_rate_readiness_path")?
            != FISCAL_PACKAGE_RATE_READINESS_JSON_PATH
        || string_field(&record, "rev_level_3_microsimulation_path")?
            != REV_LEVEL_3_MICROSIMULATION_JSON_PATH
        || string_field(&record, "rev_level_3_audit_path")? != REV_LEVEL_3_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_provisional_rate_path")?
            != FISCAL_PACKAGE_PROVISIONAL_RATE_JSON_PATH
        || string_field(&record, "rev_level_4_timing_path")? != REV_LEVEL_4_TIMING_JSON_PATH
        || string_field(&record, "rev_level_4_audit_path")? != REV_LEVEL_4_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_fiscal_timing_rate_path")?
            != FISCAL_PACKAGE_FISCAL_TIMING_RATE_JSON_PATH
        || string_field(&record, "rev_level_5_administration_ceiling_path")?
            != REV_LEVEL_5_ADMINISTRATION_CEILING_JSON_PATH
        || string_field(&record, "rev_level_5_macro_audit_path")?
            != REV_LEVEL_5_MACRO_AUDIT_JSON_PATH
        || string_field(&record, "fiscal_package_administration_bounded_rate_path")?
            != FISCAL_PACKAGE_ADMINISTRATION_BOUNDED_RATE_JSON_PATH
        || string_field(&record, "rev_level_6_policy_decision_path")?
            != REV_LEVEL_6_POLICY_DECISION_JSON_PATH
        || string_field(&record, "rev_level_6_dossier_path")? != REV_LEVEL_6_DOSSIER_JSON_PATH
        || string_field(&record, "rev_f_planning_rate_card_path")?
            != REV_F_PLANNING_RATE_CARD_JSON_PATH
        || string_field(&record, "rate_down_bundle_path")? != RATE_DOWN_BUNDLE_RERUN_JSON_PATH
        || string_field(&record, "eight_track_catchup_path")?
            != EIGHT_TRACK_TWO_LEVEL_CATCHUP_JSON_PATH
        || string_field(&record, "fifteen_track_integrated_rerun_path")?
            != FIFTEEN_TRACK_INTEGRATED_RERUN_JSON_PATH
        || string_field(&record, "rev_level_7_certification_handoff_path")?
            != REV_LEVEL_7_CERTIFICATION_HANDOFF_JSON_PATH
        || string_field(&record, "rev_level_7_policy_specification_path")?
            != REV_LEVEL_7_POLICY_SPEC_JSON_PATH
        || string_field(&record, "rev_level_7_score_workbook_path")?
            != REV_LEVEL_7_SCORE_WORKBOOK_JSON_PATH
        || string_field(&record, "rev_level_7_discussion_draft_path")?
            != REV_LEVEL_7_DISCUSSION_DRAFT_JSON_PATH
        || string_field(&record, "rev_level_7_external_submission_control_path")?
            != REV_LEVEL_7_EXTERNAL_SUBMISSION_CONTROL_JSON_PATH
        || string_field(&record, "rev_level_7_external_response_intake_path")?
            != REV_LEVEL_7_EXTERNAL_RESPONSE_INTAKE_JSON_PATH
        || string_field(&record, "fifteen_track_next_two_level_wave_path")?
            != FIFTEEN_TRACK_NEXT_TWO_LEVEL_WAVE_JSON_PATH
        || string_field(&record, "batch_2_eight_track_closure_path")?
            != BATCH_2_EIGHT_TRACK_JSON_PATH
        || string_field(&record, "batch_3_trn_rev_closure_path")? != BATCH_3_TRN_REV_JSON_PATH
        || string_field(&record, "pay_net_rev_reconciliation_path")?
            != PAY_NET_REV_POST_FIFTEEN_RECONCILIATION_JSON_PATH
        || string_field(&record, "rev_internal_next_ten_steps_path")?
            != REV_INTERNAL_NEXT_TEN_STEPS_JSON_PATH
        || string_field(&record, "rev_internal_baseline_freeze_path")?
            != REV_INTERNAL_BASELINE_FREEZE_JSON_PATH
        || string_field(&record, "rev_internal_completion_path")?
            != REV_INTERNAL_COMPLETION_JSON_PATH
        || string_field(&record, "targeted_spending_rate_decision_path")?
            != TARGETED_SPENDING_RATE_DECISION_JSON_PATH
    {
        return Err("corpus track plan identity failed".to_string());
    }

    let naming = record
        .get("naming_contract")
        .ok_or("corpus naming contract")?;
    if string_field(naming, "shared_namespace")? != "CORE"
        || string_field(naming, "transportation_prefix")? != "TRN"
        || !bool_field(naming, "track_prefix_precedes_stage")?
        || !bool_field(naming, "bare_t_prefix_prohibited_as_ambiguous")?
        || !bool_field(naming, "core_can_extend_without_renaming_tracks")?
        || !bool_field(naming, "track_can_pin_compatible_core_wave_or_version")?
    {
        return Err("corpus naming contract failed".to_string());
    }
    let expected_trn = ["TRN-A", "TRN-B", "TRN-C", "TRN-D", "TRN-E", "TRN-F"];
    let named_stages = naming
        .get("transportation_stages")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus transportation stages")?;
    if named_stages.len() != expected_trn.len()
        || named_stages
            .iter()
            .zip(expected_trn)
            .any(|(value, expected)| value.as_str() != Some(expected))
    {
        return Err("corpus transportation stage names failed".to_string());
    }
    let expected_prefixes = [
        "TRN", "HLT", "EDU", "OAS", "ISF", "VET", "AGR", "DEF", "DIS", "JUS", "SEE", "INT", "PAY",
        "REV", "NET",
    ];
    let named_prefixes = naming
        .get("canonical_track_prefixes")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus canonical track prefixes")?;
    if named_prefixes.len() != expected_prefixes.len()
        || named_prefixes
            .iter()
            .zip(expected_prefixes)
            .any(|(value, expected)| value.as_str() != Some(expected))
    {
        return Err("corpus canonical track prefix contract failed".to_string());
    }

    let core = record
        .get("core_roadmap")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus core roadmap")?;
    if core.len() != 8
        || string_field(&core[0], "wave_id")? != "CORE-G"
        || string_field(&core[0], "status")? != "complete"
        || string_field(&core[1], "wave_id")? != "CORE-H"
        || string_field(&core[1], "status")? != "complete"
        || string_field(&core[2], "wave_id")? != "CORE-I"
        || string_field(&core[2], "status")? != "complete"
        || string_field(&core[3], "wave_id")? != "CORE-J"
        || string_field(&core[3], "status")? != "complete"
        || string_field(&core[4], "wave_id")? != "CORE-K"
        || string_field(&core[4], "status")? != "complete"
        || string_field(&core[5], "wave_id")? != "CORE-L"
        || string_field(&core[5], "status")? != "complete"
        || string_field(&core[6], "wave_id")? != "CORE-M"
        || string_field(&core[6], "status")? != "complete"
        || string_field(&core[7], "wave_id")? != "CORE-N"
        || string_field(&core[7], "status")? != "complete"
    {
        return Err("corpus CORE roadmap failed".to_string());
    }

    let recipe = record
        .get("transportation_recipe")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus transportation recipe")?;
    if recipe.len() != expected_trn.len() {
        return Err("corpus plan must define exactly six transportation stages".to_string());
    }
    for (index, stage) in recipe.iter().enumerate() {
        if string_field(stage, "wave_id")? != expected_trn[index]
            || string_field(stage, "title")?.is_empty()
            || string_field(stage, "completion")?.is_empty()
            || string_field(stage, "status")?.is_empty()
            || stage
                .get("depends_on")
                .and_then(serde_json::Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "corpus transportation stage failed: {}",
                expected_trn[index]
            ));
        }
    }
    let trn_a_dependencies = recipe[0]
        .get("depends_on")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A dependencies")?;
    let trn_b_dependencies = recipe[1]
        .get("depends_on")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B dependencies")?;
    if trn_a_dependencies.len() != 1
        || trn_a_dependencies[0].as_str() != Some("CORE-G")
        || trn_b_dependencies.len() != 2
        || trn_b_dependencies[0].as_str() != Some("TRN-A")
        || trn_b_dependencies[1].as_str() != Some("CORE-H")
    {
        return Err("corpus CORE-to-transportation dependency boundary failed".to_string());
    }
    if string_field(&recipe[0], "status")? != "complete"
        || string_field(&recipe[1], "status")? != "complete"
        || string_field(&recipe[2], "status")? != "complete_cost_only_reform"
        || string_field(&recipe[3], "status")? != "complete_bounded_bridge"
        || string_field(&recipe[4], "status")? != "complete_output_ready_typed_cost_only"
        || string_field(&recipe[5], "status")? != "complete_typed_cost_note"
    {
        return Err("corpus transportation execution status failed".to_string());
    }
    for index in 2..recipe.len() - 1 {
        let dependencies = recipe[index]
            .get("depends_on")
            .and_then(serde_json::Value::as_array)
            .ok_or("transportation dependencies")?;
        if dependencies.len() != 1 || dependencies[0].as_str() != Some(expected_trn[index - 1]) {
            return Err(format!(
                "corpus dependency order failed: {}",
                expected_trn[index]
            ));
        }
    }
    let trn_f_dependencies = recipe[5]
        .get("depends_on")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-F dependencies")?;
    if trn_f_dependencies.len() != 2
        || trn_f_dependencies[0].as_str() != Some("TRN-E")
        || trn_f_dependencies[1].as_str() != Some("CORE-N")
    {
        return Err("corpus TRN-F dependency boundary failed".to_string());
    }

    let start = record
        .get("start_contract")
        .ok_or("corpus start contract")?;
    if string_field(start, "track_wave")? != "TRN-A"
        || !bool_field(start, "core_g_done")?
        || !bool_field(start, "trn_a_may_start")?
        || !bool_field(start, "core_h_not_required_to_start_trn_a")?
        || !bool_field(start, "core_h_required_before_trn_b")?
    {
        return Err("corpus TRN-A start contract failed".to_string());
    }
    let execution = record
        .get("execution_state")
        .ok_or("corpus execution state")?;
    for field in [
        "trn_a_done",
        "core_h_findings_derived_from_trn_a",
        "core_h_done",
        "trn_b_dependencies_complete",
        "trn_b_started",
        "trn_b_done",
        "trn_c_may_start",
        "trn_c_started",
        "trn_c_done",
        "core_i_done",
        "trn_d_started",
        "trn_d_01_done",
        "trn_d_03_done",
        "trn_d_04_done",
        "trn_d_05_done",
        "trn_d_06_done",
        "trn_d_done",
        "trn_e_started",
        "hlt_a_started",
        "hlt_a_01_done",
        "hlt_a_02_done",
        "hlt_a_03_done",
        "hlt_a_04_done",
        "hlt_a_05_done",
        "hlt_a_done",
        "hlt_b_may_start",
        "hlt_b_started",
        "edu_a_started",
        "edu_a_01_done",
        "edu_a_02_done",
        "edu_a_03_done",
        "edu_a_04_done",
        "edu_a_05_done",
        "edu_a_done",
        "edu_b_may_start",
        "edu_b_started",
        "core_j_done",
        "trn_e_01_done",
        "trn_e_02_started",
        "hlt_b_01_done",
        "hlt_b_02_started",
        "edu_b_01_done",
        "edu_b_02_started",
        "core_k_done",
        "hlt_b_done",
        "hlt_c_done",
        "edu_b_done",
        "edu_c_done",
        "oas_a_done",
        "oas_b_done",
        "oas_c_done",
        "all_four_tracks_at_or_beyond_c",
        "core_l_done",
        "trn_e_done",
        "hlt_d_done",
        "edu_d_done",
        "oas_d_done",
        "isf_a_done",
        "isf_b_done",
        "vet_a_done",
        "vet_b_done",
        "agr_a_done",
        "agr_b_done",
        "isf_c_done",
        "vet_c_done",
        "agr_c_done",
        "isf_d_done",
        "vet_d_done",
        "agr_d_done",
        "def_d_done",
        "dis_d_done",
        "jus_d_done",
        "see_d_done",
        "int_d_done",
        "pay_d_done",
        "rev_d_done",
        "net_d_done",
        "all_fifteen_tracks_at_or_beyond_d",
        "hlt_e_done",
        "edu_e_done",
        "oas_e_done",
        "isf_e_done",
        "vet_e_done",
        "agr_e_done",
        "def_e_done",
        "dis_e_done",
        "jus_e_done",
        "see_e_done",
        "int_e_done",
        "pay_e_done",
        "rev_e_done",
        "net_e_done",
        "all_fifteen_tracks_at_or_beyond_e",
        "all_fifteen_f_starts_audited",
        "two_f_advancement_levels_defined",
        "trn_level_1_core_lessons_audited",
        "core_m_discovery_recommended",
        "core_m_started",
        "core_m_done",
        "trn_level_1_may_resume",
        "trn_level_1_done",
        "trn_level_2_may_start",
        "trn_level_2_done",
        "trn_e_output_ready",
        "trn_f_may_start",
        "core_n_done",
        "trn_f_started",
        "trn_f_done",
        "rev_level_1_started",
        "candidate_selected",
        "all_requested_stage_minimums_met",
        "nonofficial_discussion_draft_done",
        "all_fifteen_next_two_level_wave_started",
        "all_fifteen_next_level_a_items_started",
        "hlt_next_two_levels_complete_zero_admission",
        "batch_1_five_tracks_two_levels_complete_zero_additive_savings",
        "batch_2_eight_tracks_two_levels_complete_zero_admission",
        "all_fifteen_internal_two_levels_complete",
        "post_fifteen_pay_net_rev_reconciliation_complete",
        "sealed_external_submission_preflight_complete",
        "deterministic_submission_bundle_built",
        "external_response_intake_ready",
        "independent_analysis_only",
        "internal_rate_analysis_complete",
        "taxlane_analytical_rate_finalized",
        "targeted_spending_rate_wave_complete",
    ] {
        if !bool_field(execution, field)? {
            return Err(format!("corpus execution state failed: {field}"));
        }
    }
    let other_tracks = record
        .get("additional_track_starts")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus additional track starts")?;
    if other_tracks.len() != 14
        || other_tracks.iter().any(|track| {
            !matches!(
                track.get("status").and_then(serde_json::Value::as_str),
                Some(
                    "complete_bounded_no_candidate_solver_run"
                        | "complete_bounded_non_additive_no_candidate_solver_run"
                        | "complete_bounded_endogenous_no_candidate_solver_run"
                        | "advanced_two_levels_candidate_selected_for_audit_dependency_reblock"
                        | "advanced_four_levels_official_score_current_trustees_ready_six_gates_blocked"
                        | "advanced_four_levels_scale_proxy_complete_uniform_candidate_floor_rejected"
                        | "advanced_four_levels_timing_envelope_complete_unallocated_candidate_rejected"
                        | "advanced_two_levels_measurement_envelope_dependency_reblock_non_additive"
                        | "advanced_four_levels_specific_control_causal_pass_accounting_blocked_non_additive"
                        | "rev_internal_rate_analysis_complete_taxlane_recommendation_published"
                        | "advanced_four_levels_zero_fixture_valid_nonzero_feedback_blocked"
                )
            ) || !track
                .get("wave_id")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|wave| wave.ends_with("-E"))
        })
    {
        return Err("corpus additional track start boundary failed".to_string());
    }
    let frontier = record
        .get("parallel_frontier")
        .ok_or("corpus parallel frontier")?;
    let active = frontier
        .get("active_waves")
        .and_then(serde_json::Value::as_array)
        .ok_or("corpus active waves")?;
    if !active.is_empty()
        || !bool_field(frontier, "parallel_discovery_may_not_bypass_dependencies")?
    {
        return Err("corpus parallel frontier failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("corpus claim booleans")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "corpus_track_plan_published"
                | "core_g_done"
                | "core_h_done"
                | "trn_a_started"
                | "trn_a_done"
                | "trn_b_started"
                | "trn_b_done"
                | "trn_c_started"
                | "trn_c_done"
                | "core_i_done"
                | "trn_d_started"
                | "hlt_a_started"
                | "edu_a_started"
                | "trn_d_01_done"
                | "trn_d_03_done"
                | "trn_d_04_done"
                | "trn_d_05_done"
                | "trn_d_06_done"
                | "trn_d_done"
                | "trn_e_started"
                | "hlt_a_01_done"
                | "hlt_a_02_done"
                | "hlt_a_03_done"
                | "hlt_a_04_done"
                | "hlt_a_05_done"
                | "hlt_a_done"
                | "hlt_b_started"
                | "edu_a_01_done"
                | "edu_a_02_done"
                | "edu_a_03_done"
                | "edu_a_04_done"
                | "edu_a_05_done"
                | "edu_a_done"
                | "edu_b_started"
                | "core_j_done"
                | "trn_e_01_done"
                | "trn_e_02_started"
                | "hlt_b_01_done"
                | "hlt_b_02_started"
                | "edu_b_01_done"
                | "edu_b_02_started"
                | "core_k_done"
                | "hlt_b_done"
                | "hlt_c_done"
                | "edu_b_done"
                | "edu_c_done"
                | "oas_a_done"
                | "oas_b_done"
                | "oas_c_done"
                | "all_four_tracks_at_or_beyond_c"
                | "core_l_done"
                | "trn_e_done"
                | "hlt_d_done"
                | "edu_d_done"
                | "oas_d_done"
                | "isf_a_done"
                | "isf_b_done"
                | "vet_a_done"
                | "vet_b_done"
                | "agr_a_done"
                | "agr_b_done"
                | "isf_c_done"
                | "vet_c_done"
                | "agr_c_done"
                | "isf_d_done"
                | "vet_d_done"
                | "agr_d_done"
                | "def_d_done"
                | "dis_d_done"
                | "jus_d_done"
                | "see_d_done"
                | "int_d_done"
                | "pay_d_done"
                | "rev_d_done"
                | "net_d_done"
                | "all_fifteen_tracks_at_or_beyond_d"
                | "hlt_e_done"
                | "edu_e_done"
                | "oas_e_done"
                | "isf_e_done"
                | "vet_e_done"
                | "agr_e_done"
                | "def_e_done"
                | "dis_e_done"
                | "jus_e_done"
                | "see_e_done"
                | "int_e_done"
                | "pay_e_done"
                | "rev_e_done"
                | "net_e_done"
                | "all_fifteen_tracks_at_or_beyond_e"
                | "all_fifteen_f_starts_audited"
                | "two_f_advancement_levels_defined"
                | "trn_level_1_core_lessons_audited"
                | "core_m_discovery_recommended"
                | "core_m_started"
                | "core_m_done"
                | "trn_level_1_may_resume"
                | "trn_level_1_done"
                | "trn_level_2_may_start"
                | "trn_level_2_done"
                | "trn_e_output_ready"
                | "trn_f_may_start"
                | "core_n_done"
                | "trn_f_started"
                | "trn_f_done"
                | "rev_level_1_started"
                | "rev_level_5_administration_ceiling_done"
                | "rev_level_5_macro_methodology_audit_done"
                | "strongest_bounded_planning_rate_ready"
                | "rev_level_6_policy_rate_decision_done"
                | "rev_level_6_revenue_dossier_done"
                | "planning_rate_selected"
                | "planning_rate_assigned"
                | "rev_f_started"
                | "rev_f_done"
                | "planning_rate_published"
                | "rate_down_bundle_done"
                | "eight_track_two_level_catchup_done"
                | "fifteen_track_integrated_rerun_done"
                | "rev_level_7_internal_certification_done"
                | "official_score_handoff_ready"
                | "scorer_ready_policy_specification_done"
                | "ten_year_score_request_workbook_done"
                | "nonofficial_discussion_draft_done"
                | "all_fifteen_next_two_level_wave_started"
                | "all_fifteen_next_level_a_items_started"
                | "hlt_next_two_levels_complete_zero_admission"
                | "batch_1_five_tracks_two_levels_complete_zero_additive_savings"
                | "batch_2_eight_tracks_two_levels_complete_zero_admission"
                | "all_fifteen_internal_two_levels_complete"
                | "post_fifteen_pay_net_rev_reconciliation_complete"
                | "sealed_external_submission_preflight_complete"
                | "deterministic_submission_bundle_built"
                | "external_response_intake_ready"
                | "independent_analysis_only"
                | "internal_rate_analysis_complete"
                | "taxlane_analytical_rate_finalized"
                | "targeted_spending_rate_wave_complete"
                | "candidate_selected"
                | "any_f_stage_started"
                | "all_requested_stage_minimums_met"
                | "rate_published"
        );
        if value.as_bool().ok_or("corpus claim must be bool")? != expected {
            return Err(format!("corpus claim boundary failed: {field}"));
        }
    }

    for (path, phrase) in [
        (
            CORPUS_TRACK_PLAN_SCHEMA_PATH,
            "CORE-J supplies shared bounded-closure",
        ),
        (
            CORPUS_TRACK_PLAN_READER_PATH,
            "TRN-A -> TRN-B -> TRN-C -> TRN-D -> TRN-E -> TRN-F",
        ),
        (
            "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md",
            "## CORE and lane-track roadmap",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("corpus plan prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_adaptive_rate_post_f_wave_roadmap(root: &Path) -> Result<(), String> {
    for path in [
        POST_F_WAVE_ROADMAP_JSON_PATH,
        POST_F_WAVE_ROADMAP_SCHEMA_PATH,
        POST_F_WAVE_ROADMAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing post-F roadmap artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(POST_F_WAVE_ROADMAP_JSON_PATH))
        .map_err(|err| format!("failed to read post-F roadmap: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse post-F roadmap: {err}"))?;
    if string_field(&record, "record_id")? != "adaptive-rate-post-f-wave-roadmap:v1"
        || string_field(&record, "record_family")? != "adaptive_rate_post_f_wave_roadmap"
        || string_field(&record, "status")?
            != "legacy_g_i_complete_j_started_mapped_to_core_and_trn"
        || int_field(&record, "pulse")? != 233
        || string_field(&record, "schema_path")? != POST_F_WAVE_ROADMAP_SCHEMA_PATH
        || string_field(&record, "reader_path")? != POST_F_WAVE_ROADMAP_READER_PATH
        || string_field(&record, "wave_plan_path")?
            != "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md"
        || string_field(&record, "wave_g_contract_path")? != WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH
        || string_field(&record, "solver_input_readiness_rollup_path")?
            != SOLVER_INPUT_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "final_closure_readiness_gate_path")?
            != FINAL_CLOSURE_READINESS_GATE_JSON_PATH
    {
        return Err("post-F roadmap identity failed".to_string());
    }

    let rules = record.get("roadmap_rules").ok_or("post-F roadmap rules")?;
    for field in [
        "dependency_order_is_mandatory",
        "a_wave_may_start_discovery_before_prior_wave_closes",
        "a_wave_may_not_admit_outputs_that_depend_on_an_open_prior_gate",
        "missing_values_remain_null",
        "official_source_custody_required",
        "unrounded_arithmetic_controls",
        "net_interest_remains_endogenous",
        "trust_funds_remain_separate",
        "revenue_solvency_and_payment_integrity_remain_non_additive_overlays",
        "first_public_release_is_one_pilot_not_all_lanes",
    ] {
        if !bool_field(rules, field)? {
            return Err(format!("post-F roadmap rule failed: {field}"));
        }
    }

    let waves = record
        .get("waves")
        .and_then(serde_json::Value::as_array)
        .ok_or("post-F roadmap waves")?;
    let expected_ids = ["G", "H", "I", "J", "K", "L"];
    if waves.len() != expected_ids.len() {
        return Err("post-F roadmap must define exactly six waves".to_string());
    }
    for (index, wave) in waves.iter().enumerate() {
        let wave_id = string_field(wave, "wave_id")?;
        if wave_id != expected_ids[index]
            || string_field(wave, "title")?.is_empty()
            || string_field(wave, "objective")?.is_empty()
        {
            return Err(format!("post-F roadmap wave shape failed: {wave_id}"));
        }
        for field in [
            "depends_on",
            "acceptance_gates",
            "newly_allowed_outputs",
            "still_prohibited",
        ] {
            if wave
                .get(field)
                .and_then(serde_json::Value::as_array)
                .is_none_or(Vec::is_empty)
            {
                return Err(format!("post-F roadmap {wave_id} missing {field}"));
            }
        }
        let expected_dependency = if index == 0 {
            "F"
        } else {
            expected_ids[index - 1]
        };
        let dependencies = wave
            .get("depends_on")
            .and_then(serde_json::Value::as_array)
            .ok_or("post-F wave dependencies")?;
        let expected_state = match index {
            0 => "complete_as_core_g",
            1 => "complete_as_core_h",
            2 => "complete_as_trn_c_cost_only_reform",
            3 => "in_progress_as_trn_d",
            4 => "defined_waiting_on_j",
            _ => "defined_waiting_on_k",
        };
        if dependencies.len() != 1
            || dependencies[0].as_str() != Some(expected_dependency)
            || string_field(wave, "completion_state")? != expected_state
        {
            return Err(format!("post-F roadmap dependency failed: {wave_id}"));
        }
    }
    if string_field(&record, "dependency_chain")? != "G -> H -> I -> J -> K -> L" {
        return Err("post-F roadmap dependency chain failed".to_string());
    }

    let allocations = record
        .get("blocker_allocation")
        .and_then(serde_json::Value::as_array)
        .ok_or("post-F blocker allocation")?;
    let expected_blockers = BTreeSet::from([
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
    let observed_blockers = allocations
        .iter()
        .map(|row| string_field(row, "blocker_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if allocations.len() != 10 || observed_blockers != expected_blockers {
        return Err("post-F roadmap must allocate all ten blockers once".to_string());
    }
    for allocation in allocations {
        let primary = string_field(allocation, "primary_wave")?;
        if !["G", "H", "I", "J"].contains(&primary.as_str()) {
            return Err(format!(
                "post-F blocker has invalid primary wave: {primary}"
            ));
        }
    }

    let aggregate = record
        .get("aggregate_status")
        .ok_or("post-F roadmap aggregate")?;
    if int_field(aggregate, "wave_count")? != 6
        || int_field(aggregate, "defined_waves")? != 6
        || int_field(aggregate, "completed_waves")? != 3
        || int_field(aggregate, "in_progress_waves")? != 1
        || int_field(aggregate, "mapped_blockers")? != 10
        || string_field(aggregate, "first_public_output_wave")? != "L"
        || bool_field(aggregate, "all_lane_publication_planned")?
        || bool_field(aggregate, "balanced_budget_publication_planned")?
    {
        return Err("post-F roadmap aggregate failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("post-F roadmap claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("post-F roadmap claim bool")?;
        let expected = matches!(
            field.as_str(),
            "post_f_roadmap_published"
                | "wave_g_done"
                | "wave_h_done"
                | "wave_i_done"
                | "wave_j_started"
        );
        if observed != expected {
            return Err(format!("post-F roadmap claim boundary failed: {field}"));
        }
    }

    for (path, phrase) in [
        (
            POST_F_WAVE_ROADMAP_SCHEMA_PATH,
            "G -> H -> I -> J -> K -> L",
        ),
        (
            POST_F_WAVE_ROADMAP_READER_PATH,
            "This Pulse 233 predecessor mapped the next six waves as one dependency chain",
        ),
        (
            "context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md",
            "## CORE and lane-track roadmap",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("post-F roadmap prose missing: {phrase}"));
        }
    }

    Ok(())
}

