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

pub(crate) fn validate_fifteen_lane_bounded_d_portfolio(root: &Path) -> Result<(), String> {
    let canonical = [
        ("TRN", "transportation-infrastructure", "E"),
        ("HLT", "health-medicare", "E"),
        ("EDU", "education-workforce", "E"),
        ("OAS", "social-security", "E"),
        ("ISF", "income-security-family", "E"),
        ("VET", "veterans", "E"),
        ("AGR", "agriculture", "E"),
        ("DEF", "national-defense", "E"),
        ("DIS", "disaster-resilience", "E"),
        ("JUS", "justice-courts-public-safety", "E"),
        ("SEE", "science-energy-environment", "E"),
        ("INT", "international-affairs", "E"),
        ("PAY", "payment-integrity", "E"),
        ("REV", "revenue-solvency", "E"),
        ("NET", "net-interest", "E"),
    ];

    let matrix = read_json_artifact(root, FIFTEEN_LANE_STAGE_MATRIX_JSON_PATH)?;
    let rows = matrix
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-lane stage matrix rows")?;
    let aggregate = matrix
        .get("aggregate_status")
        .ok_or("fifteen-lane stage matrix aggregate")?;
    if string_field(&matrix, "record_id")? != "fifteen-lane-track-stage-matrix:v1"
        || string_field(&matrix, "status")? != "all_fifteen_tracks_at_bounded_e"
        || int_field(&matrix, "pulse")? != 372
        || rows.len() != 15
        || int_field(aggregate, "lane_tracks")? != 15
        || int_field(aggregate, "at_or_beyond_e")? != 15
        || int_field(aggregate, "remaining_e_closures")? != 0
        || !bool_field(aggregate, "prefixes_unique")?
        || !bool_field(aggregate, "lane_ids_unique")?
    {
        return Err("fifteen-lane stage matrix identity failed".to_string());
    }
    let mut prefixes = BTreeSet::new();
    let mut lane_ids = BTreeSet::new();
    for (row, (prefix, lane_id, current_stage)) in rows.iter().zip(canonical) {
        let evidence = string_field(row, "current_evidence_path")?;
        if string_field(row, "prefix")? != prefix
            || string_field(row, "lane_id")? != lane_id
            || string_field(row, "current_stage")? != current_stage
            || string_field(row, "target_stage")? != "E"
            || !bool_field(row, "target_met")?
            || !root.join(&evidence).is_file()
            || !prefixes.insert(prefix.to_string())
            || !lane_ids.insert(lane_id.to_string())
        {
            return Err(format!("fifteen-lane stage row failed: {prefix}"));
        }
    }
    validate_blocked_outputs_null(&matrix, "fifteen-lane stage matrix")?;
    validate_claim_boundary(
        &matrix,
        "fifteen-lane stage matrix",
        &[
            "fifteen_lane_stage_matrix_published",
            "all_fifteen_tracks_named",
            "all_fifteen_tracks_at_or_beyond_e",
        ],
    )?;

    for (path, wave_id, pulse, done_field) in [
        (ISF_D_CLOSURE_JSON_PATH, "ISF-D", 337, "isf_d_done"),
        (VET_D_CLOSURE_JSON_PATH, "VET-D", 338, "vet_d_done"),
        (AGR_D_CLOSURE_JSON_PATH, "AGR-D", 339, "agr_d_done"),
    ] {
        let stage = read_json_artifact(root, path)?;
        let packages = stage
            .get("package_reconciliation")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("{wave_id} package reconciliation"))?;
        let decision = stage
            .get("closure_decision")
            .ok_or_else(|| format!("{wave_id} closure decision"))?;
        if int_field(&stage, "pulse")? != pulse
            || string_field(&stage, "track_wave_id")? != wave_id
            || packages.len() != 6
            || packages.iter().any(|row| {
                !matches!(
                    row.get("status").and_then(serde_json::Value::as_str),
                    Some("complete" | "not_required")
                )
            })
            || !bool_field(decision, "role_review_complete")?
            || !bool_field(decision, done_field)?
            || bool_field(decision, "reform_admitted")?
            || bool_field(decision, "numeric_completion")?
            || bool_field(decision, "output_admission")?
        {
            return Err(format!("{wave_id} bounded-D closure failed"));
        }
        validate_blocked_outputs_null(&stage, wave_id)?;
    }

    let three_lane = read_json_artifact(root, THREE_LANE_STAGE_D_BUNDLE_JSON_PATH)?;
    let three_evidence = three_lane
        .get("lane_stage_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("three-lane D evidence")?;
    let three_aggregate = three_lane
        .get("aggregate_status")
        .ok_or("three-lane D aggregate")?;
    if int_field(&three_lane, "pulse")? != 340
        || three_evidence.len() != 3
        || three_evidence.iter().any(|row| {
            !bool_field(row, "requirement_met").unwrap_or(false)
                || bool_field(row, "reform_admitted").unwrap_or(true)
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || int_field(three_aggregate, "d_stages_done")? != 3
        || int_field(three_aggregate, "portfolio_tracks_at_or_beyond_d")? != 7
        || int_field(three_aggregate, "portfolio_tracks_remaining_to_d")? != 8
    {
        return Err("three-lane bounded-D bundle failed".to_string());
    }
    validate_blocked_outputs_null(&three_lane, "three-lane bounded-D bundle")?;

    let stage_chains = [
        (DEF_D_CLOSURE_JSON_PATH, "DEF", 342, "def_d_done", None),
        (DIS_D_CLOSURE_JSON_PATH, "DIS", 343, "dis_d_done", None),
        (JUS_D_CLOSURE_JSON_PATH, "JUS", 344, "jus_d_done", None),
        (SEE_D_CLOSURE_JSON_PATH, "SEE", 345, "see_d_done", None),
        (INT_D_CLOSURE_JSON_PATH, "INT", 346, "int_d_done", None),
        (
            PAY_D_CLOSURE_JSON_PATH,
            "PAY",
            347,
            "pay_d_done",
            Some("non_additive_overlay"),
        ),
        (
            REV_D_CLOSURE_JSON_PATH,
            "REV",
            348,
            "rev_d_done",
            Some("non_additive_overlay"),
        ),
        (
            NET_D_CLOSURE_JSON_PATH,
            "NET",
            349,
            "net_d_done",
            Some("endogenous"),
        ),
    ];
    for (path, prefix, pulse, done_field, treatment) in stage_chains {
        let stage = read_json_artifact(root, path)?;
        let stages = stage
            .get("stage_reconciliation")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("{prefix} stage reconciliation"))?;
        let bridge = stage
            .get("d_bridge_components")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("{prefix} D bridge"))?;
        let decision = stage
            .get("closure_decision")
            .ok_or_else(|| format!("{prefix} closure decision"))?;
        if int_field(&stage, "pulse")? != pulse
            || string_field(&stage, "track_prefix")? != prefix
            || string_field(&stage, "track_wave_id")? != format!("{prefix}-D")
            || stages.len() != 4
            || stages.iter().enumerate().any(|(index, row)| {
                string_field(row, "stage").ok().as_deref()
                    != Some(format!("{prefix}-{}", (b'A' + index as u8) as char).as_str())
                    || row.get("status").and_then(serde_json::Value::as_str) != Some("complete")
            })
            || bridge.len() != 5
            || bridge.iter().any(|row| {
                !row.get("status")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|status| {
                        status == "complete"
                            || status.starts_with("complete_")
                            || status.starts_with("not_required_")
                    })
            })
            || !bool_field(decision, "dependency_order_verified")?
            || !bool_field(decision, "role_review_complete")?
            || !bool_field(decision, done_field)?
            || bool_field(decision, "reform_admitted")?
            || bool_field(decision, "output_admission")?
            || treatment.is_some_and(|expected| {
                string_field(&stage, "lane_treatment").ok().as_deref() != Some(expected)
            })
        {
            return Err(format!("{prefix} bounded A-through-D chain failed"));
        }
        for field in [
            "baseline_evidence_path",
            "source_evidence_path",
            "floor_evidence_path",
            "scenario_evidence_path",
            "claim_boundary_path",
        ] {
            let evidence = string_field(&stage, field)?;
            if !root.join(&evidence).is_file() {
                return Err(format!("{prefix} missing {field}: {evidence}"));
            }
        }
        validate_blocked_outputs_null(&stage, prefix)?;
    }

    let eight_lane = read_json_artifact(root, EIGHT_LANE_A_D_BUNDLE_JSON_PATH)?;
    let eight_evidence = eight_lane
        .get("lane_stage_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("eight-lane bounded bundle evidence")?;
    let eight_aggregate = eight_lane
        .get("aggregate_status")
        .ok_or("eight-lane bounded bundle aggregate")?;
    if int_field(&eight_lane, "pulse")? != 350
        || eight_evidence.len() != 8
        || eight_evidence.iter().any(|row| {
            !bool_field(row, "requirement_met").unwrap_or(false)
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || int_field(eight_aggregate, "a_stages_done")? != 8
        || int_field(eight_aggregate, "b_stages_done")? != 8
        || int_field(eight_aggregate, "c_stages_done")? != 8
        || int_field(eight_aggregate, "d_stages_done")? != 8
        || int_field(eight_aggregate, "non_additive_overlays")? != 2
        || int_field(eight_aggregate, "endogenous_tracks")? != 1
    {
        return Err("eight-lane bounded A-through-D bundle failed".to_string());
    }
    validate_blocked_outputs_null(&eight_lane, "eight-lane bounded bundle")?;

    let portfolio = read_json_artifact(root, FIFTEEN_LANE_D_PORTFOLIO_JSON_PATH)?;
    let evidence = portfolio
        .get("lane_stage_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-lane portfolio evidence")?;
    let audit = portfolio
        .get("completion_audit")
        .ok_or("fifteen-lane completion audit")?;
    let evidence_prefixes = evidence
        .iter()
        .map(|row| string_field(row, "prefix"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let evidence_lanes = evidence
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if string_field(&portfolio, "record_id")? != "fifteen-lane-stage-d-portfolio-closure:v1"
        || string_field(&portfolio, "status")?
            != "all_fifteen_tracks_at_or_beyond_bounded_d_role_reviewed"
        || int_field(&portfolio, "pulse")? != 352
        || evidence.len() != 15
        || evidence_prefixes.len() != 15
        || evidence_lanes.len() != 15
        || evidence.iter().any(|row| {
            !bool_field(row, "at_or_beyond_d").unwrap_or(false)
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || int_field(audit, "canonical_lane_rows")? != 15
        || int_field(audit, "unique_lane_ids")? != 15
        || int_field(audit, "unique_prefixes")? != 15
        || int_field(audit, "evidence_paths_present")? != 15
        || int_field(audit, "lane_decisions_role_reviewed")? != 15
        || int_field(audit, "lane_outputs_blocked")? != 15
        || int_field(audit, "at_or_beyond_d")? != 15
        || int_field(audit, "remaining_below_d")? != 0
        || int_field(audit, "non_additive_overlays_preserved")? != 2
        || int_field(audit, "endogenous_tracks_preserved")? != 1
        || !bool_field(audit, "all_fifteen_d_goal_achieved")?
    {
        return Err("fifteen-lane bounded-D portfolio audit failed".to_string());
    }
    validate_blocked_outputs_null(&portfolio, "fifteen-lane bounded-D portfolio")?;
    validate_claim_boundary(
        &portfolio,
        "fifteen-lane bounded-D portfolio",
        &[
            "fifteen_lane_stage_d_portfolio_closure_published",
            "all_fifteen_tracks_named",
            "all_fifteen_tracks_at_or_beyond_d",
            "all_fifteen_lane_decisions_role_reviewed",
            "non_additive_overlays_preserved",
            "endogenous_net_interest_preserved",
        ],
    )?;

    let bill_sources = [
        (
            GOVINFO_HR2137_REPORTED_BILL_PDF_PATH,
            293_973,
            "8a3ab9dad836076a4ef43870e3c1c1a2f8791a3f443da942ee118d90052df3ec",
        ),
        (
            GOVINFO_HR2137_REPORTED_BILL_HTML_PATH,
            39_798,
            "817c56d92f624f44b2d35dd21f6a1beeb77a615ed6ba247fdaf9055b2a9928f7",
        ),
    ];
    for (path, expected_bytes, expected_hash) in bill_sources {
        let source = root.join(path);
        if fs::metadata(&source)
            .map_err(|err| format!("failed to stat {path}: {err}"))?
            .len()
            != expected_bytes
            || sha256_file(&source)? != expected_hash
        {
            return Err(format!("H.R. 2137 source custody failed: {path}"));
        }
    }

    for (path, phrase) in [
        (
            FIFTEEN_LANE_STAGE_MATRIX_READER_PATH,
            "stable prefixes for all fifteen analytical lanes",
        ),
        (
            THREE_LANE_STAGE_D_REVIEW_PATH,
            "approve bounded structural closure of ISF-D, VET-D, and AGR-D",
        ),
        (
            EIGHT_LANE_A_D_REVIEW_PATH,
            "approve bounded A-through-D closure for DEF, DIS, JUS, SEE, INT, PAY",
        ),
        (
            FIFTEEN_LANE_D_PORTFOLIO_READER_PATH,
            "All fifteen Taxlane tracks are now at or beyond bounded stage D",
        ),
        (
            FIFTEEN_LANE_D_PORTFOLIO_REVIEW_PATH,
            "approve the portfolio claim that all fifteen named tracks are at or",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("fifteen-lane bounded-D prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_fifteen_lane_bounded_e_portfolio(root: &Path) -> Result<(), String> {
    let required_paths = [
        LANE_E_CONTRACT_JSON_PATH,
        LANE_E_CONTRACT_SCHEMA_PATH,
        LANE_E_CONTRACT_READER_PATH,
        HLT_E_CLOSURE_JSON_PATH,
        EDU_E_CLOSURE_JSON_PATH,
        OAS_E_CLOSURE_JSON_PATH,
        ISF_E_CLOSURE_JSON_PATH,
        VET_E_CLOSURE_JSON_PATH,
        AGR_E_CLOSURE_JSON_PATH,
        DEF_E_CLOSURE_JSON_PATH,
        DIS_E_CLOSURE_JSON_PATH,
        JUS_E_CLOSURE_JSON_PATH,
        SEE_E_CLOSURE_JSON_PATH,
        INT_E_CLOSURE_JSON_PATH,
        PAY_E_CLOSURE_JSON_PATH,
        REV_E_CLOSURE_JSON_PATH,
        NET_E_CLOSURE_JSON_PATH,
        FOURTEEN_LANE_E_BUNDLE_JSON_PATH,
        FOURTEEN_LANE_E_BUNDLE_SCHEMA_PATH,
        FOURTEEN_LANE_E_BUNDLE_READER_PATH,
        FOURTEEN_LANE_E_REVIEW_PATH,
        FIFTEEN_LANE_E_PORTFOLIO_JSON_PATH,
        FIFTEEN_LANE_E_PORTFOLIO_SCHEMA_PATH,
        FIFTEEN_LANE_E_PORTFOLIO_READER_PATH,
        FIFTEEN_LANE_E_PORTFOLIO_REVIEW_PATH,
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing bounded-E portfolio artifact: {path}"));
        }
    }

    let contract = read_json_artifact(root, LANE_E_CONTRACT_JSON_PATH)?;
    let rule = contract.get("stage_rule").ok_or("lane-E stage rule")?;
    let packages = contract
        .get("required_packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane-E required packages")?;
    if string_field(&contract, "record_id")? != "lane-e-bounded-selection-solver-gate-contract:v1"
        || string_field(&contract, "status")?
            != "canonical_lane_e_bounded_closure_contract_complete"
        || int_field(&contract, "pulse")? != 356
        || string_field(rule, "required_predecessor")? != "lane-D"
        || int_field(rule, "package_count")? != 5
        || !bool_field(rule, "selection_must_resolve")?
        || !bool_field(
            rule,
            "solver_run_requires_selected_candidate_and_complete_inputs",
        )?
        || !bool_field(rule, "bounded_closure_without_solver_run_allowed")?
        || !bool_field(rule, "bounded_closure_requires_role_review")?
        || bool_field(rule, "bounded_closure_may_not_open_stage_f")? != true
        || !bool_field(
            rule,
            "output_admission_requires_completed_solver_and_review",
        )?
        || packages.len() != 5
    {
        return Err("canonical lane-E contract failed".to_string());
    }
    validate_blocked_outputs_null(&contract, "lane-E contract")?;
    validate_claim_boundary(
        &contract,
        "lane-E contract",
        &[
            "lane_e_contract_published",
            "bounded_e_without_solver_run_allowed",
        ],
    )?;

    let closures = [
        (
            HLT_E_CLOSURE_JSON_PATH,
            "HLT",
            "health-medicare",
            357,
            "hlt_e_done",
            "hlt_f_may_start",
            None,
        ),
        (
            EDU_E_CLOSURE_JSON_PATH,
            "EDU",
            "education-workforce",
            358,
            "edu_e_done",
            "edu_f_may_start",
            None,
        ),
        (
            OAS_E_CLOSURE_JSON_PATH,
            "OAS",
            "social-security",
            359,
            "oas_e_done",
            "oas_f_may_start",
            None,
        ),
        (
            ISF_E_CLOSURE_JSON_PATH,
            "ISF",
            "income-security-family",
            360,
            "isf_e_done",
            "isf_f_may_start",
            None,
        ),
        (
            VET_E_CLOSURE_JSON_PATH,
            "VET",
            "veterans",
            361,
            "vet_e_done",
            "vet_f_may_start",
            None,
        ),
        (
            AGR_E_CLOSURE_JSON_PATH,
            "AGR",
            "agriculture",
            362,
            "agr_e_done",
            "agr_f_may_start",
            None,
        ),
        (
            DEF_E_CLOSURE_JSON_PATH,
            "DEF",
            "national-defense",
            363,
            "def_e_done",
            "def_f_may_start",
            None,
        ),
        (
            DIS_E_CLOSURE_JSON_PATH,
            "DIS",
            "disaster-resilience",
            364,
            "dis_e_done",
            "dis_f_may_start",
            None,
        ),
        (
            JUS_E_CLOSURE_JSON_PATH,
            "JUS",
            "justice-courts-public-safety",
            365,
            "jus_e_done",
            "jus_f_may_start",
            None,
        ),
        (
            SEE_E_CLOSURE_JSON_PATH,
            "SEE",
            "science-energy-environment",
            366,
            "see_e_done",
            "see_f_may_start",
            None,
        ),
        (
            INT_E_CLOSURE_JSON_PATH,
            "INT",
            "international-affairs",
            367,
            "int_e_done",
            "int_f_may_start",
            None,
        ),
        (
            PAY_E_CLOSURE_JSON_PATH,
            "PAY",
            "payment-integrity",
            368,
            "pay_e_done",
            "pay_f_may_start",
            Some("non_additive_overlay"),
        ),
        (
            REV_E_CLOSURE_JSON_PATH,
            "REV",
            "revenue-solvency",
            369,
            "rev_e_done",
            "rev_f_may_start",
            Some("non_additive_overlay"),
        ),
        (
            NET_E_CLOSURE_JSON_PATH,
            "NET",
            "net-interest",
            370,
            "net_e_done",
            "net_f_may_start",
            Some("endogenous"),
        ),
    ];
    for (path, prefix, lane_id, pulse, done_field, next_field, treatment) in closures {
        let closure = read_json_artifact(root, path)?;
        let packages = closure
            .get("package_reconciliation")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("{prefix}-E package reconciliation"))?;
        let selection = closure
            .get("selection_decision")
            .ok_or_else(|| format!("{prefix}-E selection decision"))?;
        let decision = closure
            .get("closure_decision")
            .ok_or_else(|| format!("{prefix}-E closure decision"))?;
        let expected_statuses = [
            "complete",
            "complete",
            "not_required",
            "not_required",
            "complete",
        ];
        let d_path = string_field(&closure, "d_closure_path")?;
        if int_field(&closure, "pulse")? != pulse
            || string_field(&closure, "track_prefix")? != prefix
            || string_field(&closure, "track_wave_id")? != format!("{prefix}-E")
            || string_field(&closure, "lane_id")? != lane_id
            || string_field(&closure, "contract_path")? != LANE_E_CONTRACT_JSON_PATH
            || !root.join(&d_path).is_file()
            || packages.len() != 5
            || packages
                .iter()
                .zip(expected_statuses)
                .any(|(row, expected)| {
                    row.get("status").and_then(serde_json::Value::as_str) != Some(expected)
                })
            || !bool_field(selection, "selection_complete")?
            || bool_field(selection, "candidate_selected")?
            || !selection
                .get("candidate_target_cost")
                .is_some_and(serde_json::Value::is_null)
            || !selection
                .get("financing_source")
                .is_some_and(serde_json::Value::is_null)
            || !bool_field(decision, "dependency_order_verified")?
            || !bool_field(decision, "role_review_complete")?
            || !bool_field(decision, done_field)?
            || bool_field(decision, next_field)?
            || bool_field(decision, "solver_run_performed")?
            || bool_field(decision, "numeric_completion")?
            || bool_field(decision, "output_admission")?
            || treatment.is_some_and(|expected| {
                string_field(&closure, "lane_treatment").ok().as_deref() != Some(expected)
            })
        {
            return Err(format!("{prefix}-E bounded closure failed"));
        }
        validate_blocked_outputs_null(&closure, &format!("{prefix}-E"))?;
    }

    let bundle = read_json_artifact(root, FOURTEEN_LANE_E_BUNDLE_JSON_PATH)?;
    let bundle_rows = bundle
        .get("lane_stage_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("fourteen-lane E evidence")?;
    let bundle_aggregate = bundle
        .get("aggregate_status")
        .ok_or("fourteen-lane E aggregate")?;
    let bundle_prefixes = bundle_rows
        .iter()
        .map(|row| string_field(row, "prefix"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if int_field(&bundle, "pulse")? != 371
        || bundle_rows.len() != 14
        || bundle_prefixes.len() != 14
        || bundle_rows.iter().any(|row| {
            !bool_field(row, "requirement_met").unwrap_or(false)
                || !string_field(row, "highest_completed_stage")
                    .is_ok_and(|stage| stage.ends_with("-E"))
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || int_field(bundle_aggregate, "e_stages_done")? != 14
        || int_field(bundle_aggregate, "candidate_selections_completed")? != 14
        || int_field(bundle_aggregate, "candidates_selected")? != 0
        || int_field(bundle_aggregate, "solver_runs")? != 0
        || int_field(bundle_aggregate, "f_starts_allowed")? != 0
        || int_field(bundle_aggregate, "non_additive_overlays")? != 2
        || int_field(bundle_aggregate, "endogenous_tracks")? != 1
    {
        return Err("fourteen-lane bounded-E bundle failed".to_string());
    }
    validate_blocked_outputs_null(&bundle, "fourteen-lane bounded-E bundle")?;
    validate_claim_boundary(
        &bundle,
        "fourteen-lane bounded-E bundle",
        &[
            "fourteen_lane_e_bundle_published",
            "fourteen_bounded_e_closures_done",
        ],
    )?;

    let portfolio = read_json_artifact(root, FIFTEEN_LANE_E_PORTFOLIO_JSON_PATH)?;
    let rows = portfolio
        .get("lane_stage_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-lane E evidence")?;
    let audit = portfolio
        .get("completion_audit")
        .ok_or("fifteen-lane E completion audit")?;
    let prefixes = rows
        .iter()
        .map(|row| string_field(row, "prefix"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let lane_ids = rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let evidence_paths = rows
        .iter()
        .map(|row| string_field(row, "evidence_path"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if string_field(&portfolio, "record_id")? != "fifteen-lane-stage-e-portfolio-closure:v1"
        || string_field(&portfolio, "status")? != "all_fifteen_tracks_at_bounded_e_role_reviewed"
        || int_field(&portfolio, "pulse")? != 373
        || rows.len() != 15
        || prefixes.len() != 15
        || lane_ids.len() != 15
        || evidence_paths.len() != 15
        || rows.iter().any(|row| {
            !bool_field(row, "at_or_beyond_e").unwrap_or(false)
                || string_field(row, "highest_completed_stage").ok().as_deref()
                    != Some(
                        format!("{}-E", string_field(row, "prefix").unwrap_or_default()).as_str(),
                    )
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || int_field(audit, "canonical_lane_rows")? != 15
        || int_field(audit, "unique_lane_ids")? != 15
        || int_field(audit, "unique_prefixes")? != 15
        || int_field(audit, "unique_evidence_paths")? != 15
        || int_field(audit, "d_dependencies_present")? != 15
        || int_field(audit, "lane_decisions_role_reviewed")? != 15
        || int_field(audit, "selection_decisions_complete")? != 15
        || int_field(audit, "candidates_selected")? != 0
        || int_field(audit, "solver_runs_performed")? != 0
        || int_field(audit, "lane_outputs_blocked")? != 15
        || int_field(audit, "at_or_beyond_e")? != 15
        || int_field(audit, "remaining_below_e")? != 0
        || int_field(audit, "f_starts_allowed")? != 0
        || int_field(audit, "non_additive_overlays_preserved")? != 2
        || int_field(audit, "endogenous_tracks_preserved")? != 1
        || !bool_field(audit, "all_fifteen_e_goal_achieved")?
    {
        return Err("fifteen-lane bounded-E portfolio audit failed".to_string());
    }
    validate_blocked_outputs_null(&portfolio, "fifteen-lane bounded-E portfolio")?;
    validate_claim_boundary(
        &portfolio,
        "fifteen-lane bounded-E portfolio",
        &[
            "fifteen_lane_stage_e_portfolio_closure_published",
            "all_fifteen_tracks_named",
            "all_fifteen_tracks_at_or_beyond_e",
            "all_fifteen_lane_decisions_role_reviewed",
            "selection_decisions_complete",
            "non_additive_overlays_preserved",
            "endogenous_net_interest_preserved",
        ],
    )?;

    for (path, phrase) in [
        (
            LANE_E_CONTRACT_READER_PATH,
            "A reviewed “no candidate” decision is a",
        ),
        (
            FOURTEEN_LANE_E_BUNDLE_READER_PATH,
            "all fifteen tracks are now at bounded E",
        ),
        (
            FOURTEEN_LANE_E_REVIEW_PATH,
            "approve bounded E closure for HLT, EDU, OAS, ISF, VET, AGR, DEF, DIS,",
        ),
        (
            FIFTEEN_LANE_E_PORTFOLIO_READER_PATH,
            "All fifteen Taxlane tracks are now at bounded stage E",
        ),
        (
            FIFTEEN_LANE_E_PORTFOLIO_REVIEW_PATH,
            "approve the portfolio claim that all fifteen named tracks are at",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("fifteen-lane bounded-E prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_fifteen_lane_f_readiness_and_queue(root: &Path) -> Result<(), String> {
    let required_paths = [
        LANE_F_CONTRACT_JSON_PATH,
        LANE_F_CONTRACT_SCHEMA_PATH,
        LANE_F_CONTRACT_READER_PATH,
        FIFTEEN_LANE_F_READINESS_JSON_PATH,
        FIFTEEN_LANE_F_READINESS_SCHEMA_PATH,
        FIFTEEN_LANE_F_READINESS_READER_PATH,
        FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_JSON_PATH,
        FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_SCHEMA_PATH,
        FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_READER_PATH,
        FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_REVIEW_PATH,
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing F-readiness artifact: {path}"));
        }
    }

    let contract = read_json_artifact(root, LANE_F_CONTRACT_JSON_PATH)?;
    let requirements = contract
        .get("start_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("lane-F start requirements")?;
    let rule = contract.get("stage_rule").ok_or("lane-F stage rule")?;
    let treatments = contract
        .get("special_treatments")
        .ok_or("lane-F special treatments")?;
    let gate_ids = requirements
        .iter()
        .map(|row| string_field(row, "gate_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if string_field(&contract, "record_id")? != "lane-f-public-release-gate-contract:v1"
        || string_field(&contract, "record_family")? != "lane_f_public_release_gate_contract"
        || string_field(&contract, "status")?
            != "canonical_lane_f_typed_release_contract_complete_starts_blocked"
        || int_field(&contract, "pulse")? != 388
        || string_field(&contract, "core_m_path")? != CORE_M_CLOSURE_JSON_PATH
        || string_field(&contract, "schema_path")? != LANE_F_CONTRACT_SCHEMA_PATH
        || string_field(&contract, "reader_path")? != LANE_F_CONTRACT_READER_PATH
        || requirements.len() != 10
        || gate_ids.len() != 10
        || requirements.iter().any(|row| {
            !bool_field(row, "required").unwrap_or(false)
                || string_field(row, "current_portfolio_status")
                    .ok()
                    .as_deref()
                    != Some("blocked")
        })
        || bool_field(rule, "bounded_e_is_sufficient_for_f_start")?
        || !bool_field(rule, "e_output_ready_required")?
        || !bool_field(rule, "candidate_selected_required")?
        || !bool_field(rule, "solver_run_required_when_profile_requires")?
        || !bool_field(rule, "all_ten_gates_must_be_dispositioned")?
        || !bool_field(rule, "all_applicable_gates_required")?
        || !bool_field(rule, "reviewed_not_applicable_allowed")?
        || !bool_field(rule, "not_applicable_cannot_supply_result")?
        || !bool_field(rule, "partial_f_start_prohibited")?
        || !bool_field(rule, "public_card_requires_role_and_language_review")?
        || string_field(treatments, "PAY")? != "non_additive_overlay_no_savings_shortcut"
        || string_field(treatments, "REV")? != "non_additive_overlay_no_rate_shortcut"
        || string_field(treatments, "NET")? != "endogenous_no_direct_cut"
    {
        return Err("canonical lane-F release contract failed".to_string());
    }
    validate_blocked_outputs_null(&contract, "lane-F release contract")?;
    validate_claim_boundary(
        &contract,
        "lane-F release contract",
        &[
            "lane_f_contract_published",
            "all_f_start_requirements_defined",
            "typed_release_profiles_integrated",
        ],
    )?;

    let readiness = read_json_artifact(root, FIFTEEN_LANE_F_READINESS_JSON_PATH)?;
    let rows = readiness
        .get("lane_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-lane F readiness rows")?;
    let aggregate = readiness
        .get("aggregate_status")
        .ok_or("fifteen-lane F readiness aggregate")?;
    let prefixes = rows
        .iter()
        .map(|row| string_field(row, "prefix"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let lane_ids = rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let evidence_paths = rows
        .iter()
        .map(|row| string_field(row, "evidence_path"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if string_field(&readiness, "record_id")? != "fifteen-lane-stage-f-start-readiness:v1"
        || string_field(&readiness, "record_family")? != "fifteen_lane_stage_f_start_readiness"
        || string_field(&readiness, "status")?
            != "trn_f_complete_cost_note_fourteen_f_starts_blocked"
        || int_field(&readiness, "pulse")? != 409
        || string_field(&readiness, "contract_path")? != LANE_F_CONTRACT_JSON_PATH
        || string_field(&readiness, "trn_level_1_dossier_path")? != TRN_LEVEL_1_DOSSIER_JSON_PATH
        || string_field(&readiness, "trn_level_2_e_rerun_path")? != TRN_LEVEL_2_E_RERUN_JSON_PATH
        || string_field(&readiness, "trn_f_cost_note_path")? != TRN_F_COST_NOTE_JSON_PATH
        || string_field(&readiness, "schema_path")? != FIFTEEN_LANE_F_READINESS_SCHEMA_PATH
        || string_field(&readiness, "reader_path")? != FIFTEEN_LANE_F_READINESS_READER_PATH
        || rows.len() != 15
        || prefixes.len() != 15
        || lane_ids.len() != 15
        || evidence_paths.len() != 15
        || rows.iter().any(|row| {
            let is_trn = string_field(row, "prefix").ok().as_deref() == Some("TRN");
            string_field(row, "e_completion_class").ok().as_deref()
                != Some(if is_trn {
                    "output_ready_typed_cost_only"
                } else {
                    "bounded_structural"
                })
                || bool_field(row, "candidate_selected").ok() != Some(is_trn)
                || bool_field(row, "solver_run_complete").unwrap_or(true)
                || bool_field(row, "e_output_ready").ok() != Some(is_trn)
                || bool_field(row, "f_may_start").ok() != Some(is_trn)
                || string_field(row, "primary_blocker").is_err()
                || !string_field(row, "evidence_path").is_ok_and(|path| root.join(path).is_file())
        })
        || rows
            .iter()
            .find(|row| string_field(row, "prefix").ok().as_deref() == Some("PAY"))
            .and_then(|row| row.get("special_treatment"))
            .and_then(serde_json::Value::as_str)
            != Some("non_additive_overlay")
        || rows
            .iter()
            .find(|row| string_field(row, "prefix").ok().as_deref() == Some("REV"))
            .and_then(|row| row.get("special_treatment"))
            .and_then(serde_json::Value::as_str)
            != Some("non_additive_overlay")
        || rows
            .iter()
            .find(|row| string_field(row, "prefix").ok().as_deref() == Some("NET"))
            .and_then(|row| row.get("special_treatment"))
            .and_then(serde_json::Value::as_str)
            != Some("endogenous")
        || int_field(aggregate, "lane_tracks")? != 15
        || int_field(aggregate, "evidence_paths_present")? != 15
        || int_field(aggregate, "bounded_e_closures")? != 14
        || int_field(aggregate, "output_ready_e_closures")? != 1
        || int_field(aggregate, "candidate_selections")? != 1
        || int_field(aggregate, "solver_runs")? != 0
        || int_field(aggregate, "f_starts_ready")? != 1
        || int_field(aggregate, "f_starts_actual")? != 1
        || int_field(aggregate, "f_completions")? != 1
        || int_field(aggregate, "f_starts_blocked")? != 14
        || int_field(aggregate, "non_additive_overlays")? != 2
        || int_field(aggregate, "endogenous_tracks")? != 1
    {
        return Err("fifteen-lane F-start readiness audit failed".to_string());
    }
    validate_blocked_outputs_null(&readiness, "fifteen-lane F readiness")?;
    validate_claim_boundary(
        &readiness,
        "fifteen-lane F readiness",
        &[
            "fifteen_lane_f_readiness_published",
            "all_fifteen_f_starts_audited",
            "any_candidate_selected",
            "trn_e_output_ready",
            "trn_f_may_start",
            "trn_f_started",
            "trn_f_done",
            "any_f_stage_started",
        ],
    )?;

    let queue = read_json_artifact(root, FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_JSON_PATH)?;
    let queue_rows = queue
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-lane two-level F queue rows")?;
    let queue_aggregate = queue
        .get("aggregate_status")
        .ok_or("fifteen-lane two-level F queue aggregate")?;
    let queue_prefixes = queue_rows
        .iter()
        .map(|row| string_field(row, "prefix"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut tier_counts = [0_i64; 3];
    for row in queue_rows {
        let tier = int_field(row, "priority_tier")?;
        if !(1..=3).contains(&tier) {
            return Err("F advancement queue has invalid priority tier".to_string());
        }
        tier_counts[(tier - 1) as usize] += 1;
        for level in ["level_1", "level_2"] {
            let item = row.get(level).ok_or("F advancement queue level")?;
            if string_field(item, "work")?.trim().is_empty()
                || string_field(item, "exit")?.trim().is_empty()
            {
                return Err("F advancement queue has empty work or exit".to_string());
            }
        }
    }
    if string_field(&queue, "record_id")? != "fifteen-lane-two-level-f-advancement-queue:v1"
        || string_field(&queue, "record_family")? != "fifteen_lane_two_level_f_advancement_queue"
        || string_field(&queue, "status")?
            != "trn_f_complete_rev_guarded_complete_five_tracks_advanced_twice"
        || int_field(&queue, "pulse")? != 424
        || string_field(&queue, "f_contract_path")? != LANE_F_CONTRACT_JSON_PATH
        || string_field(&queue, "f_readiness_path")? != FIFTEEN_LANE_F_READINESS_JSON_PATH
        || string_field(&queue, "trn_level_1_dossier_path")? != TRN_LEVEL_1_DOSSIER_JSON_PATH
        || string_field(&queue, "trn_level_2_e_rerun_path")? != TRN_LEVEL_2_E_RERUN_JSON_PATH
        || string_field(&queue, "trn_f_cost_note_path")? != TRN_F_COST_NOTE_JSON_PATH
        || string_field(&queue, "rev_level_1_start_path")? != REV_LEVEL_1_START_JSON_PATH
        || string_field(&queue, "rev_level_1_guarded_closure_path")?
            != REV_LEVEL_1_GUARDED_CLOSURE_JSON_PATH
        || string_field(&queue, "fiscally_decisive_level_1_path")?
            != FISCALLY_DECISIVE_LEVEL_1_JSON_PATH
        || string_field(&queue, "fiscally_decisive_level_2_path")?
            != FISCALLY_DECISIVE_LEVEL_2_JSON_PATH
        || string_field(&queue, "schema_path")? != FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_SCHEMA_PATH
        || string_field(&queue, "reader_path")? != FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_READER_PATH
        || string_field(&queue, "role_review_path")? != FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_REVIEW_PATH
        || queue_rows.len() != 15
        || queue_prefixes != prefixes
        || tier_counts != [2, 4, 9]
        || int_field(queue_aggregate, "tracks")? != 15
        || int_field(queue_aggregate, "level_1_work_items")? != 15
        || int_field(queue_aggregate, "level_2_work_items")? != 15
        || int_field(queue_aggregate, "level_1_completed")? != 7
        || int_field(queue_aggregate, "level_1_started")? != 7
        || int_field(queue_aggregate, "level_2_completed")? != 7
        || int_field(queue_aggregate, "candidate_selections_for_dependency_audit")? != 4
        || int_field(queue_aggregate, "reviewed_reblocks")? != 6
        || int_field(queue_aggregate, "tier_1_tracks")? != 2
        || int_field(queue_aggregate, "tier_2_tracks")? != 4
        || int_field(queue_aggregate, "tier_3_tracks")? != 9
        || int_field(queue_aggregate, "f_starts_ready")? != 1
        || int_field(queue_aggregate, "f_starts_actual")? != 1
        || int_field(queue_aggregate, "f_completions")? != 1
    {
        return Err("fifteen-lane two-level F advancement queue failed".to_string());
    }
    validate_blocked_outputs_null(&queue, "fifteen-lane two-level F queue")?;
    validate_claim_boundary(
        &queue,
        "fifteen-lane two-level F queue",
        &[
            "two_level_queue_published",
            "all_fifteen_tracks_have_two_levels",
            "priority_sequence_reviewed",
            "trn_level_1_done",
            "trn_level_2_may_start",
            "trn_level_2_done",
            "trn_f_may_start",
            "trn_f_started",
            "trn_f_done",
            "rev_level_1_started",
            "rev_level_1_guarded_proxy_done",
            "five_fiscal_tracks_advanced_two_levels",
            "any_candidate_selected",
            "any_f_stage_started",
        ],
    )?;

    for (path, phrase) in [
        (
            LANE_F_CONTRACT_READER_PATH,
            "TRN passed that gate and completed F only as a typed cost note",
        ),
        (
            FIFTEEN_LANE_F_READINESS_READER_PATH,
            "All fifteen F starts have been audited",
        ),
        (
            FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_READER_PATH,
            "Every track now has the same two-level advancement rhythm",
        ),
        (
            FIFTEEN_LANE_TWO_LEVEL_F_QUEUE_REVIEW_PATH,
            "approve the two-level work queue and sequencing rationale",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("F-readiness prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_fifteen_track_integrated_dependency_admission_rerun(root: &Path) -> Result<(), String> {
    for path in [
        FIFTEEN_TRACK_INTEGRATED_RERUN_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fifteen_track_integrated_dependency_admission_rerun.schema.md",
        "docs/reading/fifteen-track-integrated-dependency-admission-rerun.md",
        "reviews/2026-07-27-fifteen-track-integrated-dependency-admission-rerun-role-review.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!(
                "missing fifteen-track integration artifact: {path}"
            ));
        }
    }
    let record = read_json_artifact(root, FIFTEEN_TRACK_INTEGRATED_RERUN_JSON_PATH)?;
    let rows = record
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-track integration rows")?;
    let checks = record
        .get("integration_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("fifteen-track integration checks")?;
    let fiscal = record
        .get("combined_fiscal_surface")
        .ok_or("fifteen-track fiscal surface")?;
    let aggregate = record
        .get("aggregate_status")
        .ok_or("fifteen-track aggregate")?;
    let expected_tracks = BTreeSet::from([
        "TRN".to_string(),
        "HLT".to_string(),
        "EDU".to_string(),
        "OAS".to_string(),
        "ISF".to_string(),
        "VET".to_string(),
        "AGR".to_string(),
        "DEF".to_string(),
        "DIS".to_string(),
        "JUS".to_string(),
        "SEE".to_string(),
        "INT".to_string(),
        "PAY".to_string(),
        "REV".to_string(),
        "NET".to_string(),
    ]);
    let observed_tracks = rows
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let public_surfaces = rows
        .iter()
        .filter(|row| {
            row.get("public_surface")
                .is_some_and(|value| !value.is_null())
        })
        .count();
    if int_field(&record, "pulse")? != 459
        || rows.len() != 15
        || observed_tracks != expected_tracks
        || public_surfaces != 2
        || rows.iter().any(|row| {
            number_field(row, "admitted_fy2026_primary_reduction_billions")
                .is_ok_and(|value| value.abs() > 0.0001)
        })
        || checks.len() != 8
        || checks
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|value| value != "pass"))
        || (number_field(fiscal, "fy2026_primary_cash_target_billions")? - 813.727).abs() > 0.001
        || number_field(
            fiscal,
            "admitted_fy2026_primary_spending_reduction_billions",
        )?
        .abs()
            > 0.0001
        || (number_field(fiscal, "remaining_fy2026_revenue_target_billions")? - 813.727).abs()
            > 0.001
        || (number_field(fiscal, "rev_model_first_year_cash_proxy_billions")? - 819.220).abs()
            > 0.001
        || (number_field(fiscal, "rev_model_proxy_cushion_billions")? - 5.493).abs() > 0.001
        || (number_field(fiscal, "planning_uplift_points")? - 11.0).abs() > 0.0001
        || bool_field(fiscal, "official_combined_score")?
        || bool_field(fiscal, "formal_balance_certified")?
        || int_field(aggregate, "tracks")? != 15
        || int_field(aggregate, "candidate_decisions_complete")? != 15
        || int_field(aggregate, "public_typed_outputs")? != 2
        || int_field(aggregate, "admitted_spending_candidates")? != 0
        || !bool_field(aggregate, "integrated_dependency_rerun_complete")?
        || !bool_field(
            aggregate,
            "rev_level_7_internal_certification_audit_may_start",
        )?
        || bool_field(aggregate, "official_external_certification_ready")?
    {
        return Err("fifteen-track integrated dependency rerun failed".to_string());
    }
    validate_blocked_outputs_null(&record, "fifteen-track integrated dependency rerun")?;
    Ok(())
}

pub(crate) fn validate_fifteen_track_terminal_disposition(root: &Path) -> Result<(), String> {
    for path in [
        FIFTEEN_TRACK_TERMINAL_DISPOSITION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fifteen_track_terminal_disposition.schema.md",
        "docs/reading/fifteen-track-terminal-disposition.md",
        "reviews/2026-07-27-fifteen-track-terminal-disposition-role-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-479-fifteen-track-terminal-disposition.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing fifteen-track terminal artifact: {path}"));
        }
    }

    let record = read_json_artifact(root, FIFTEEN_TRACK_TERMINAL_DISPOSITION_JSON_PATH)?;
    let scope = record.get("scope").ok_or("terminal scope")?;
    let tracks = record
        .get("tracks")
        .and_then(serde_json::Value::as_array)
        .ok_or("terminal tracks")?;
    let result = record
        .get("portfolio_result")
        .ok_or("terminal portfolio result")?;
    let claims = record.get("claim_booleans").ok_or("terminal claims")?;
    let expected_tracks = BTreeSet::from([
        "TRN".to_string(),
        "HLT".to_string(),
        "EDU".to_string(),
        "OAS".to_string(),
        "ISF".to_string(),
        "VET".to_string(),
        "AGR".to_string(),
        "DEF".to_string(),
        "DIS".to_string(),
        "JUS".to_string(),
        "SEE".to_string(),
        "INT".to_string(),
        "PAY".to_string(),
        "REV".to_string(),
        "NET".to_string(),
    ]);
    let observed_tracks = tracks
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let allowed_dispositions = BTreeSet::from([
        "conditional_typed_output".to_string(),
        "reviewed_zero_admission".to_string(),
        "dedicated_solvency_overlay".to_string(),
        "non_additive_measurement_overlay".to_string(),
        "internal_analytical_recommendation".to_string(),
        "endogenous_result".to_string(),
    ]);
    let zero_admission_count = tracks
        .iter()
        .filter(|row| {
            string_field(row, "disposition").ok().as_deref() == Some("reviewed_zero_admission")
        })
        .count();
    let typed_outputs = tracks
        .iter()
        .filter(|row| {
            row.get("typed_output")
                .is_some_and(|value| !value.is_null())
        })
        .count();
    let admitted_total = tracks
        .iter()
        .map(|row| number_field(row, "admitted_fy2026_primary_reduction_billions"))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();
    let expected_schedule = [21, 23, 33, 35, 43, 46, 48];
    let schedule_matches = result
        .get("preferred_schedule_percent")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            values.len() == expected_schedule.len()
                && values
                    .iter()
                    .zip(expected_schedule)
                    .all(|(value, expected)| value.as_i64() == Some(expected))
        });

    for row in tracks {
        let evidence_path = string_field(row, "evidence_path")?;
        if !root.join(&evidence_path).is_file()
            || string_field(row, "lane")?.is_empty()
            || string_field(row, "reopen_when")?.is_empty()
            || !allowed_dispositions.contains(&string_field(row, "disposition")?)
        {
            return Err(format!(
                "fifteen-track terminal row failed: {}",
                string_field(row, "track")?
            ));
        }
    }

    if int_field(&record, "pulse")? != 479
        || string_field(&record, "status")?
            != "all_fifteen_tracks_internally_complete_with_typed_terminal_dispositions"
        || int_field(scope, "tracks")? != 15
        || bool_field(scope, "official_request_planned")?
        || bool_field(
            scope,
            "external_certification_required_for_internal_completion",
        )?
        || tracks.len() != 15
        || observed_tracks != expected_tracks
        || zero_admission_count != 10
        || typed_outputs != 2
        || admitted_total.abs() > 0.0001
        || int_field(result, "terminal_tracks")? != 15
        || int_field(result, "reviewed_zero_admission_tracks")? != 10
        || int_field(result, "typed_public_outputs")? != 2
        || int_field(result, "admitted_spending_candidates")? != 0
        || number_field(result, "admitted_fy2026_primary_reduction_billions")?.abs() > 0.0001
        || (number_field(result, "remaining_fy2026_revenue_target_billions")? - 813.727).abs()
            > 0.001
        || !schedule_matches
        || !bool_field(result, "internal_portfolio_complete")?
        || !bool_field(result, "active_track_work_queue_empty")?
        || !bool_field(claims, "all_fifteen_tracks_present_once")?
        || !bool_field(claims, "all_fifteen_tracks_internally_complete")?
        || !bool_field(claims, "reviewed_zero_admission_counts_as_completion")?
        || bool_field(claims, "official_score_claimed")?
        || bool_field(claims, "enacted_law_claimed")?
        || bool_field(claims, "formal_balance_certified")?
        || bool_field(claims, "all_policy_uncertainty_resolved")?
    {
        return Err("fifteen-track terminal disposition failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_fifteen_track_next_two_level_advancement_wave(root: &Path) -> Result<(), String> {
    for path in [
        FIFTEEN_TRACK_NEXT_TWO_LEVEL_WAVE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fifteen_track_next_two_level_advancement_wave.schema.md",
        "docs/reading/fifteen-track-next-two-level-advancement-wave.md",
        "reviews/2026-07-27-fifteen-track-next-two-level-advancement-wave-role-review.md",
        HLT_NEXT_LEVEL_A_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/hlt_next_level_a_site_neutral_evidence_closure.schema.md",
        "docs/reading/hlt-next-level-a-site-neutral-evidence-closure.md",
        "reviews/2026-07-27-hlt-next-level-a-site-neutral-evidence-closure-role-review.md",
        HLT_NEXT_LEVEL_B_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/hlt_next_level_b_admission_rerun.schema.md",
        "docs/reading/hlt-next-level-b-admission-rerun.md",
        "reviews/2026-07-27-hlt-next-level-b-admission-rerun-role-review.md",
        BATCH_1_REMAINING_FOUR_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/batch_1_remaining_four_two_level_closure.schema.md",
        "docs/reading/batch-1-remaining-four-two-level-closure.md",
        "reviews/2026-07-27-batch-1-remaining-four-two-level-closure-role-review.md",
        BATCH_2_EIGHT_TRACK_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/batch_2_eight_track_two_level_closure.schema.md",
        "docs/reading/batch-2-eight-track-two-level-closure.md",
        "reviews/2026-07-27-batch-2-eight-track-two-level-closure-role-review.md",
        BATCH_3_TRN_REV_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/batch_3_trn_rev_two_level_closure.schema.md",
        "docs/reading/batch-3-trn-rev-two-level-closure.md",
        "reviews/2026-07-27-batch-3-trn-rev-two-level-closure-role-review.md",
        PAY_NET_REV_POST_FIFTEEN_RECONCILIATION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/pay_net_rev_post_fifteen_track_reconciliation.schema.md",
        "docs/reading/pay-net-rev-post-fifteen-track-reconciliation.md",
        "reviews/2026-07-27-pay-net-rev-post-fifteen-track-reconciliation-role-review.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing all-15 next-wave artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, FIFTEEN_TRACK_NEXT_TWO_LEVEL_WAVE_JSON_PATH)?;
    let rows = record
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("all-15 next-wave rows")?;
    let aggregate = record
        .get("aggregate_status")
        .ok_or("all-15 next-wave aggregate")?;
    let expected = BTreeSet::from([
        "TRN".to_string(),
        "HLT".to_string(),
        "EDU".to_string(),
        "OAS".to_string(),
        "ISF".to_string(),
        "VET".to_string(),
        "AGR".to_string(),
        "DEF".to_string(),
        "DIS".to_string(),
        "JUS".to_string(),
        "SEE".to_string(),
        "INT".to_string(),
        "PAY".to_string(),
        "REV".to_string(),
        "NET".to_string(),
    ]);
    let observed = rows
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if int_field(&record, "pulse")? != 472
        || string_field(&record, "status")?
            != "all_fifteen_internal_two_levels_complete_external_rev_certification_pending"
        || string_field(&record, "predecessor_rerun_path")?
            != FIFTEEN_TRACK_INTEGRATED_RERUN_JSON_PATH
        || string_field(&record, "rev_level_7_handoff_path")?
            != REV_LEVEL_7_CERTIFICATION_HANDOFF_JSON_PATH
        || rows.len() != 15
        || observed != expected
        || rows.iter().any(|row| {
            let track = string_field(row, "track").ok();
            let evidence = string_field(row, "current_evidence_path").ok();
            let level_a = row.get("level_a");
            let level_b = row.get("level_b");
            evidence.is_none_or(|path| !root.join(path).is_file())
                || level_a.is_none_or(|level| {
                    string_field(level, "work").is_err()
                        || string_field(level, "exit").is_err()
                        || match track.as_deref() {
                            Some("HLT") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_precise_reblock")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(HLT_NEXT_LEVEL_A_JSON_PATH)
                            }
                            Some("DEF" | "OAS" | "PAY") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_precise_reblock")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_1_REMAINING_FOUR_JSON_PATH)
                            }
                            Some("NET") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_zero_input_contract")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_1_REMAINING_FOUR_JSON_PATH)
                            }
                            Some("VET" | "EDU" | "ISF" | "AGR" | "DIS" | "JUS" | "SEE" | "INT") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_precise_reblock")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_2_EIGHT_TRACK_JSON_PATH)
                            }
                            Some("TRN") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_precise_reblock")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_3_TRN_REV_JSON_PATH)
                            }
                            Some("REV") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_external_channel_reblock")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_3_TRN_REV_JSON_PATH)
                            }
                            _ => true,
                        }
                })
                || level_b.is_none_or(|level| {
                    string_field(level, "work").is_err()
                        || string_field(level, "exit").is_err()
                        || match track.as_deref() {
                            Some("HLT") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_zero_admission")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(HLT_NEXT_LEVEL_B_JSON_PATH)
                            }
                            Some("DEF" | "OAS" | "PAY") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_zero_admission")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_1_REMAINING_FOUR_JSON_PATH)
                            }
                            Some("NET") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_zero_endogenous_rerun")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_1_REMAINING_FOUR_JSON_PATH)
                            }
                            Some("VET" | "EDU" | "ISF" | "AGR" | "DIS" | "JUS" | "SEE" | "INT") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_zero_admission")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_2_EIGHT_TRACK_JSON_PATH)
                            }
                            Some("TRN") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_conditional_cost_note_retained")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_3_TRN_REV_JSON_PATH)
                            }
                            Some("REV") => {
                                string_field(level, "status").ok().as_deref()
                                    != Some("complete_planning_schedule_retained_external_certification_pending")
                                    || string_field(level, "evidence_path").ok().as_deref()
                                        != Some(BATCH_3_TRN_REV_JSON_PATH)
                            }
                            _ => true,
                        }
                })
        })
        || int_field(aggregate, "tracks")? != 15
        || int_field(aggregate, "unique_tracks")? != 15
        || int_field(aggregate, "level_a_work_items")? != 15
        || int_field(aggregate, "level_a_started")? != 15
        || int_field(aggregate, "level_a_completed")? != 15
        || int_field(aggregate, "level_b_work_items")? != 15
        || int_field(aggregate, "level_b_started")? != 15
        || int_field(aggregate, "level_b_completed")? != 15
        || bool_field(aggregate, "internal_wave_active")?
        || !bool_field(aggregate, "external_rev_lane_parallel")?
        || int_field(aggregate, "admitted_spending_candidates")? != 0
        || number_field(aggregate, "admitted_fy2026_primary_reduction_billions")?.abs() > 0.0001
    {
        return Err("all-15 next two-level wave identity failed".to_string());
    }
    validate_claim_boundary(
        &record,
        "all-15 next two-level wave",
        &[
            "all_fifteen_tracks_present_once",
            "all_fifteen_have_two_next_levels",
            "all_level_a_items_started",
            "any_level_a_complete",
            "any_level_b_started",
            "any_level_b_complete",
            "all_fifteen_internal_two_levels_complete",
        ],
    )?;
    validate_blocked_outputs_null(&record, "all-15 next two-level wave")?;

    let hlt_a = read_json_artifact(root, HLT_NEXT_LEVEL_A_JSON_PATH)?;
    let hlt_a_gates = hlt_a
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("HLT next Level A gates")?;
    let hlt_a_decision = hlt_a.get("decision").ok_or("HLT next Level A decision")?;
    if int_field(&hlt_a, "pulse")? != 467
        || string_field(&hlt_a, "status")?
            != "hlt_next_level_a_complete_precise_current_law_reblock"
        || hlt_a_gates.len() != 7
        || hlt_a_gates
            .iter()
            .filter(|row| {
                string_field(row, "disposition").is_ok_and(|value| value == "ready_vintage_bounded")
            })
            .count()
            != 1
        || !bool_field(hlt_a_decision, "level_a_complete")?
        || string_field(hlt_a_decision, "completion_type")? != "precise_reblock"
        || int_field(hlt_a_decision, "required_gate_count")? != 7
        || int_field(hlt_a_decision, "fully_ready_gate_count")? != 1
        || bool_field(hlt_a_decision, "candidate_admitted")?
        || number_field(hlt_a_decision, "admitted_fy2026_reduction_billions")?.abs() > 0.0001
        || !bool_field(hlt_a_decision, "level_b_may_run_zero_admission_review")?
    {
        return Err("HLT next Level A evidence closure failed".to_string());
    }
    validate_blocked_outputs_null(&hlt_a, "HLT next Level A evidence closure")?;

    let hlt_b = read_json_artifact(root, HLT_NEXT_LEVEL_B_JSON_PATH)?;
    let rerun = hlt_b
        .get("admission_rerun")
        .ok_or("HLT next Level B rerun")?;
    let accounting = hlt_b
        .get("accounting_result")
        .ok_or("HLT next Level B accounting")?;
    let hlt_b_decision = hlt_b.get("decision").ok_or("HLT next Level B decision")?;
    if int_field(&hlt_b, "pulse")? != 468
        || string_field(&hlt_b, "status")?
            != "hlt_next_level_b_complete_zero_admission_current_law_rescore_required"
        || string_field(&hlt_b, "level_a_path")? != HLT_NEXT_LEVEL_A_JSON_PATH
        || int_field(rerun, "applicable_gate_count")? != 7
        || int_field(rerun, "fully_ready_gate_count")? != 1
        || int_field(rerun, "unresolved_gate_count")? != 6
        || bool_field(rerun, "all_applicable_gates_pass")?
        || bool_field(rerun, "candidate_admitted")?
        || bool_field(rerun, "hlt_typed_savings_output_ready")?
        || number_field(rerun, "admitted_fy2026_reduction_billions")?.abs() > 0.0001
        || number_field(accounting, "primary_spending_contribution_billions")?.abs() > 0.0001
        || number_field(accounting, "pay_overlay_contribution_billions")?.abs() > 0.0001
        || number_field(accounting, "net_interest_input_billions")?.abs() > 0.0001
        || number_field(accounting, "rev_target_reduction_billions")?.abs() > 0.0001
        || !bool_field(hlt_b_decision, "level_b_complete")?
        || string_field(hlt_b_decision, "completion_type")? != "reviewed_zero_admission"
        || bool_field(hlt_b_decision, "planning_rate_change_supported")?
    {
        return Err("HLT next Level B admission rerun failed".to_string());
    }
    validate_blocked_outputs_null(&hlt_b, "HLT next Level B admission rerun")?;

    let batch = read_json_artifact(root, BATCH_1_REMAINING_FOUR_JSON_PATH)?;
    let batch_rows = batch
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Batch 1 remaining-four rows")?;
    let batch_result = batch
        .get("integrated_batch_result")
        .ok_or("Batch 1 integrated result")?;
    let batch_decision = batch.get("decision").ok_or("Batch 1 decision")?;
    let batch_tracks = batch_rows
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if int_field(&batch, "pulse")? != 469
        || string_field(&batch, "status")?
            != "def_oas_pay_net_two_levels_complete_zero_additive_spending"
        || batch_rows.len() != 4
        || batch_tracks
            != BTreeSet::from([
                "DEF".to_string(),
                "OAS".to_string(),
                "PAY".to_string(),
                "NET".to_string(),
            ])
        || batch_rows.iter().any(|row| {
            let predecessor = string_field(row, "predecessor_path").ok();
            let level_a = row.get("level_a");
            let level_b = row.get("level_b");
            predecessor.is_none_or(|path| !root.join(path).is_file())
                || level_a.is_none_or(|level| {
                    !string_field(level, "status").is_ok_and(|status| {
                        status == "complete_precise_reblock"
                            || status == "complete_zero_input_contract"
                    }) || int_field(level, "ready_gates").is_err()
                        || int_field(level, "blocked_gates").is_err()
                })
                || level_b.is_none_or(|level| {
                    bool_field(level, "candidate_admitted").unwrap_or(true)
                        || number_field(level, "admitted_primary_spending_billions")
                            .map_or(true, |value| value.abs() > 0.0001)
                        || !string_field(level, "status").is_ok_and(|status| {
                            status == "complete_zero_admission"
                                || status == "complete_zero_endogenous_rerun"
                        })
                })
        })
        || int_field(batch_result, "tracks")? != 4
        || int_field(batch_result, "level_a_completed")? != 4
        || int_field(batch_result, "level_b_completed")? != 4
        || int_field(batch_result, "new_candidates_admitted")? != 0
        || number_field(batch_result, "admitted_primary_spending_billions")?.abs() > 0.0001
        || number_field(batch_result, "rev_target_reduction_billions")?.abs() > 0.0001
        || bool_field(batch_result, "planning_rate_change_supported")?
        || !bool_field(
            batch_decision,
            "batch_1_all_five_tracks_two_levels_complete_including_hlt",
        )?
        || !bool_field(batch_decision, "def_zero_admission")?
        || !bool_field(
            batch_decision,
            "oas_dedicated_overlay_zero_primary_admission",
        )?
        || !bool_field(batch_decision, "pay_non_additive_zero_admission")?
        || !bool_field(batch_decision, "net_zero_endogenous_result")?
        || !bool_field(batch_decision, "batch_2_may_start")?
    {
        return Err("Batch 1 remaining-four two-level closure failed".to_string());
    }
    validate_blocked_outputs_null(&batch, "Batch 1 remaining-four closure")?;

    let batch_2 = read_json_artifact(root, BATCH_2_EIGHT_TRACK_JSON_PATH)?;
    let batch_2_rows = batch_2
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Batch 2 eight-track rows")?;
    let batch_2_result = batch_2
        .get("integrated_batch_result")
        .ok_or("Batch 2 integrated result")?;
    let batch_2_decision = batch_2.get("decision").ok_or("Batch 2 decision")?;
    let batch_2_tracks = batch_2_rows
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if int_field(&batch_2, "pulse")? != 470
        || string_field(&batch_2, "status")? != "eight_tracks_two_levels_complete_zero_admission"
        || string_field(&batch_2, "all_fifteen_wave_path")?
            != FIFTEEN_TRACK_NEXT_TWO_LEVEL_WAVE_JSON_PATH
        || batch_2_rows.len() != 8
        || batch_2_tracks
            != BTreeSet::from([
                "VET".to_string(),
                "EDU".to_string(),
                "ISF".to_string(),
                "AGR".to_string(),
                "DIS".to_string(),
                "JUS".to_string(),
                "SEE".to_string(),
                "INT".to_string(),
            ])
        || batch_2_rows.iter().any(|row| {
            let level_a = row.get("level_a");
            let level_b = row.get("level_b");
            string_field(row, "candidate_id").is_err()
                || level_a.is_none_or(|level| {
                    string_field(level, "status").ok().as_deref()
                        != Some("complete_precise_reblock")
                        || int_field(level, "ready_gates").is_err()
                        || int_field(level, "blocked_gates").is_err()
                        || string_field(level, "finding").is_err()
                })
                || level_b.is_none_or(|level| {
                    string_field(level, "status").ok().as_deref() != Some("complete_zero_admission")
                        || bool_field(level, "candidate_admitted").unwrap_or(true)
                        || number_field(level, "admitted_primary_spending_billions")
                            .map_or(true, |value| value.abs() > 0.0001)
                        || bool_field(level, "typed_output_ready").unwrap_or(true)
                })
        })
        || int_field(batch_2_result, "tracks")? != 8
        || int_field(batch_2_result, "level_a_completed")? != 8
        || int_field(batch_2_result, "level_b_completed")? != 8
        || int_field(batch_2_result, "precise_reblocks")? != 8
        || int_field(batch_2_result, "new_candidates_admitted")? != 0
        || number_field(batch_2_result, "admitted_fy2026_primary_spending_billions")?.abs() > 0.0001
        || number_field(batch_2_result, "rev_target_reduction_billions")?.abs() > 0.0001
        || bool_field(batch_2_result, "planning_rate_change_supported")?
        || !bool_field(batch_2_decision, "all_eight_tracks_complete_both_levels")?
        || !bool_field(
            batch_2_decision,
            "all_headline_envelopes_excluded_from_package",
        )?
        || !bool_field(batch_2_decision, "batch_3_may_start")?
        || bool_field(batch_2_decision, "proper_rate_ready")?
    {
        return Err("Batch 2 eight-track two-level closure failed".to_string());
    }
    validate_blocked_outputs_null(&batch_2, "Batch 2 eight-track closure")?;

    let batch_3 = read_json_artifact(root, BATCH_3_TRN_REV_JSON_PATH)?;
    let batch_3_rows = batch_3
        .get("track_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Batch 3 TRN-REV rows")?;
    let batch_3_result = batch_3
        .get("integrated_batch_result")
        .ok_or("Batch 3 integrated result")?;
    let batch_3_decision = batch_3.get("decision").ok_or("Batch 3 decision")?;
    let batch_3_tracks = batch_3_rows
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if int_field(&batch_3, "pulse")? != 471
        || string_field(&batch_3, "status")?
            != "trn_rev_internal_two_levels_complete_external_rev_certification_pending"
        || batch_3_tracks != BTreeSet::from(["TRN".to_string(), "REV".to_string()])
        || batch_3_rows.len() != 2
        || int_field(batch_3_result, "tracks")? != 2
        || int_field(batch_3_result, "level_a_completed")? != 2
        || int_field(batch_3_result, "level_b_completed")? != 2
        || !bool_field(
            batch_3_result,
            "all_fifteen_internal_two_level_wave_complete",
        )?
        || int_field(batch_3_result, "new_spending_candidates_admitted")? != 0
        || number_field(batch_3_result, "admitted_fy2026_primary_reduction_billions")?.abs()
            > 0.0001
        || bool_field(batch_3_result, "planning_rate_change_supported")?
        || !bool_field(batch_3_result, "external_rev_certification_pending")?
        || !bool_field(batch_3_decision, "trn_conditional_cost_note_retained")?
        || !bool_field(batch_3_decision, "trn_savings_claim_rejected")?
        || !bool_field(batch_3_decision, "rev_internal_handoff_retained")?
        || !bool_field(batch_3_decision, "rev_planning_schedule_retained")?
        || !bool_field(batch_3_decision, "authorized_external_submission_pending")?
        || !bool_field(batch_3_decision, "integrated_pay_net_rev_rerun_may_start")?
        || bool_field(batch_3_decision, "proper_rate_ready")?
    {
        return Err("Batch 3 TRN-REV two-level closure failed".to_string());
    }
    let trn = batch_3_rows
        .iter()
        .find(|row| string_field(row, "track").is_ok_and(|track| track == "TRN"))
        .ok_or("Batch 3 TRN row")?;
    let trn_a = trn.get("level_a").ok_or("Batch 3 TRN Level A")?;
    let trn_b = trn.get("level_b").ok_or("Batch 3 TRN Level B")?;
    if string_field(trn_a, "status")? != "complete_precise_reblock"
        || string_field(trn_b, "status")? != "complete_conditional_cost_note_retained"
        || bool_field(trn_b, "candidate_admitted")?
        || number_field(trn_b, "admitted_primary_spending_reduction_billions")?.abs() > 0.0001
        || !bool_field(trn_b, "conditional_cost_note_ready")?
        || int_field(trn_b, "conditional_2026_2031_outlay_cost_millions")? != 18
        || (number_field(trn_b, "fy2026_outlay_cost_upper_bound_millions")? - 0.5).abs() > 0.0001
    {
        return Err("Batch 3 TRN result failed".to_string());
    }
    let rev = batch_3_rows
        .iter()
        .find(|row| string_field(row, "track").is_ok_and(|track| track == "REV"))
        .ok_or("Batch 3 REV row")?;
    let rev_a = rev.get("level_a").ok_or("Batch 3 REV Level A")?;
    let rev_b = rev.get("level_b").ok_or("Batch 3 REV Level B")?;
    if string_field(rev_a, "status")? != "complete_external_channel_reblock"
        || string_field(rev_b, "status")?
            != "complete_planning_schedule_retained_external_certification_pending"
        || bool_field(rev_b, "official_score_received")?
        || bool_field(rev_b, "official_rate_certified")?
        || bool_field(rev_b, "formal_balance_certified")?
        || !bool_field(rev_b, "planning_only")?
        || number_array_field(rev_b, "planning_schedule_percent")?
            != vec![21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0]
    {
        return Err("Batch 3 REV result failed".to_string());
    }
    validate_blocked_outputs_null(&batch_3, "Batch 3 TRN-REV closure")?;

    let reconciliation =
        read_json_artifact(root, PAY_NET_REV_POST_FIFTEEN_RECONCILIATION_JSON_PATH)?;
    let fiscal = reconciliation
        .get("fiscal_inputs")
        .ok_or("PAY-NET-REV fiscal inputs")?;
    let rate = reconciliation
        .get("rate_result")
        .ok_or("PAY-NET-REV rate result")?;
    let identity = reconciliation
        .get("accounting_identity")
        .ok_or("PAY-NET-REV accounting identity")?;
    let decision = reconciliation
        .get("decision")
        .ok_or("PAY-NET-REV decision")?;
    if int_field(&reconciliation, "pulse")? != 472
        || string_field(&reconciliation, "status")?
            != "integrated_rerun_complete_zero_admitted_spending_planning_rate_unchanged"
        || int_field(fiscal, "fy2026_primary_deficit_millions")? != 813727
        || int_field(
            fiscal,
            "admitted_fy2026_primary_spending_reduction_millions",
        )? != 0
        || int_field(fiscal, "pay_additive_contribution_millions")? != 0
        || int_field(fiscal, "net_interest_endogenous_contribution_millions")? != 0
        || int_field(fiscal, "remaining_fy2026_revenue_need_millions")? != 813727
        || number_array_field(rate, "prior_planning_schedule_percent")?
            != vec![21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0]
        || number_array_field(rate, "reconciled_planning_schedule_percent")?
            != vec![21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0]
        || bool_field(rate, "schedule_changed")?
        || bool_field(rate, "lower_rate_supported")?
        || bool_field(rate, "proper_rate_ready")?
        || bool_field(rate, "official_rate_certified")?
        || bool_field(rate, "formal_balance_certified")?
        || int_field(identity, "equals_remaining_revenue_need_millions")? != 813727
        || !bool_field(identity, "identity_pass")?
        || !bool_field(decision, "all_fifteen_internal_results_integrated")?
        || !bool_field(decision, "zero_admitted_spending_preserved")?
        || !bool_field(decision, "pay_double_count_prevented")?
        || !bool_field(decision, "net_interest_direct_cut_prevented")?
        || !bool_field(decision, "planning_schedule_unchanged")?
        || !bool_field(decision, "external_official_score_still_required")?
    {
        return Err("PAY-NET-REV post-fifteen reconciliation failed".to_string());
    }
    validate_blocked_outputs_null(&reconciliation, "PAY-NET-REV reconciliation")?;
    Ok(())
}

