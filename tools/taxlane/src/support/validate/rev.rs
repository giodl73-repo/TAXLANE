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

pub(crate) fn validate_rev_level_1_individual_income_rate_candidate_start(root: &Path) -> Result<(), String> {
    for path in [
        REV_LEVEL_1_START_JSON_PATH,
        REV_LEVEL_1_START_SCHEMA_PATH,
        REV_LEVEL_1_START_READER_PATH,
        REV_LEVEL_1_START_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV Level-1 start artifact: {path}"));
        }
    }
    let start = read_json_artifact(root, REV_LEVEL_1_START_JSON_PATH)?;
    for field in [
        "core_n_path",
        "prior_rev_e_path",
        "irs_base_context_path",
        "omb_receipt_context_path",
        "rate_bridge_rollup_path",
        "floor_path",
        "legal_economic_base_perimeter_path",
        "tax_fiscal_timing_bridge_path",
    ] {
        let path = string_field(&start, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("REV Level-1 dependency missing: {path}"));
        }
    }
    let candidate = start
        .get("candidate_instrument")
        .ok_or("REV Level-1 candidate instrument")?;
    let packages = start
        .get("work_packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-1 work packages")?;
    let decision = start
        .get("start_decision")
        .ok_or("REV Level-1 start decision")?;
    if string_field(&start, "record_id")? != "rev-level-1-individual-income-rate-candidate-start:v1"
        || string_field(&start, "record_family")?
            != "rev_level_1_individual_income_rate_candidate_start"
        || string_field(&start, "status")?
            != "rev_level_1_l1_02_defined_l1_03_bounded_reblock_l1_04_active"
        || int_field(&start, "pulse")? != 415
        || string_field(&start, "track_prefix")? != "REV"
        || string_field(&start, "lane_treatment")? != "non_additive_overlay"
        || string_field(&start, "schema_path")? != REV_LEVEL_1_START_SCHEMA_PATH
        || string_field(&start, "reader_path")? != REV_LEVEL_1_START_READER_PATH
        || string_field(&start, "role_review_path")? != REV_LEVEL_1_START_REVIEW_PATH
        || string_field(candidate, "candidate_id")? != "federal_individual_income_tax_current_law"
        || string_field(candidate, "candidate_class")? != "revenue_instrument"
        || string_field(candidate, "release_surface_if_completed")? != "rate_card"
        || int_field(candidate, "omb_fy2025_actual_receipts_millions")? != 2_656_044
        || (number_field(candidate, "irs_ty2023_taxable_income_context_millions")? - 11_625_278.987)
            .abs()
            > 0.0001
        || !bool_field(candidate, "legal_instrument_source_ready")?
        || !bool_field(candidate, "legal_economic_perimeter_definition_ready")?
        || bool_field(candidate, "numeric_tax_fiscal_bridge_ready")?
        || bool_field(candidate, "matched_receipt_or_fee_base_ready")?
        || packages.len() != 5
        || packages
            .iter()
            .filter(|row| string_field(row, "status").ok().as_deref() == Some("blocked"))
            .count()
            != 1
        || string_field(&packages[1], "status")? != "complete_definition_only"
        || string_field(&packages[2], "status")? != "complete_bounded_reblock"
        || string_field(&packages[3], "status")? != "active"
        || !bool_field(decision, "rev_level_1_started")?
        || !bool_field(decision, "real_instrument_context_identified")?
        || !bool_field(decision, "rev_l1_02_complete")?
        || !bool_field(decision, "rev_l1_03_bounded_reblock_complete")?
        || bool_field(decision, "candidate_selection_complete")?
        || bool_field(decision, "candidate_selected")?
        || bool_field(decision, "matched_base_ready")?
        || bool_field(decision, "assigned_rate_ready")?
        || bool_field(decision, "rev_level_2_may_start")?
        || string_field(decision, "next_active_work_package_id")? != "REV-L1-04"
    {
        return Err("REV Level-1 individual-income rate-candidate start failed".to_string());
    }
    validate_blocked_outputs_null(&start, "REV Level-1 start")?;
    validate_claim_boundary(
        &start,
        "REV Level-1 start",
        &[
            "rev_level_1_start_published",
            "core_n_rate_card_surface_available",
            "real_instrument_context_identified",
            "rev_level_1_started",
            "rev_l1_02_complete",
            "rev_l1_03_bounded_reblock_complete",
        ],
    )?;
    for (path, phrase) in [
        (
            REV_LEVEL_1_START_SCHEMA_PATH,
            "may not treat either context value as",
        ),
        (
            REV_LEVEL_1_START_READER_PATH,
            "Those values must not be divided into a rate",
        ),
        (
            REV_LEVEL_1_START_REVIEW_PATH,
            "approve the federal individual income tax",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("REV Level-1 start prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_rev_level_1_individual_income_legal_economic_base_perimeter(
    root: &Path,
) -> Result<(), String> {
    for path in [
        REV_LEVEL_1_BASE_PERIMETER_JSON_PATH,
        REV_LEVEL_1_BASE_PERIMETER_SCHEMA_PATH,
        REV_LEVEL_1_BASE_PERIMETER_READER_PATH,
        REV_LEVEL_1_BASE_PERIMETER_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV-L1-02 artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, REV_LEVEL_1_BASE_PERIMETER_JSON_PATH)?;
    for field in [
        "start_path",
        "irs_soi_context_path",
        "uscode_metadata_path",
        "form1040_metadata_path",
    ] {
        let path = string_field(&record, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("REV-L1-02 dependency missing: {path}"));
        }
    }
    for (path, checksum) in [
        (
            "data/raw/uscode/SRC-USCODE-T26-INDIVIDUAL-INCOME-BASE-PRELIM-2026/2026-07-27/26usc1.html",
            "cf888c153ba8042d6e8cead3693337790eb012c8bd2e78b9251beb87541b8cec",
        ),
        (
            "data/raw/uscode/SRC-USCODE-T26-INDIVIDUAL-INCOME-BASE-PRELIM-2026/2026-07-27/26usc61.html",
            "6e858b575c7ced821c96e0890aacd95d489ddea63c8881011061562726a9388a",
        ),
        (
            "data/raw/uscode/SRC-USCODE-T26-INDIVIDUAL-INCOME-BASE-PRELIM-2026/2026-07-27/26usc62.html",
            "b6bab132760af8a695467a6063b41fd79b2fd2a1f271d42770981cc388f78682",
        ),
        (
            "data/raw/uscode/SRC-USCODE-T26-INDIVIDUAL-INCOME-BASE-PRELIM-2026/2026-07-27/26usc63.html",
            "d453e1830fd8c9facf1a3413612b7d2be7cf1e0135343043c120f9251c526a9b",
        ),
        (
            "data/raw/irs/SRC-IRS-FORM1040-INSTRUCTIONS-TY2025/2026-07-27/i1040gi--2025.pdf",
            "482e9c487c608f1bbeaceef35bc3c0933e8b35443cfff447e4279d590468364a",
        ),
    ] {
        let raw = root.join(path);
        if !raw.is_file() || sha256_file(&raw)? != checksum {
            return Err(format!("REV-L1-02 raw custody failed: {path}"));
        }
    }
    let chain = record
        .get("legal_chain")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV-L1-02 legal chain")?;
    let perimeter = record
        .get("perimeter_decision")
        .ok_or("REV-L1-02 perimeter decision")?;
    let closure = record
        .get("closure_decision")
        .ok_or("REV-L1-02 closure decision")?;
    if string_field(&record, "record_id")?
        != "rev-level-1-individual-income-legal-economic-base-perimeter:v1"
        || string_field(&record, "status")? != "rev_l1_02_complete_definition_only"
        || int_field(&record, "pulse")? != 413
        || string_field(&record, "work_package_id")? != "REV-L1-02"
        || chain.len() != 5
        || chain
            .iter()
            .any(|row| !bool_field(row, "source_ready").unwrap_or(false))
        || !bool_field(perimeter, "soi_taxable_income_is_legal_base_context")?
        || bool_field(perimeter, "soi_taxable_income_is_complete_legal_base")?
        || bool_field(perimeter, "soi_taxable_income_is_economic_base")?
        || bool_field(perimeter, "tax_after_credits_is_cash_receipts")?
        || bool_field(perimeter, "all_returns_are_taxpayers")?
        || !bool_field(perimeter, "definition_only_complete")?
        || !bool_field(closure, "rev_l1_02_complete")?
        || !bool_field(closure, "legal_source_custody_ready")?
        || !bool_field(closure, "economic_base_definition_ready")?
        || bool_field(closure, "economic_base_quantified")?
        || bool_field(closure, "matched_base_ready")?
        || bool_field(closure, "rate_calculation_allowed")?
    {
        return Err("REV-L1-02 legal/economic base perimeter failed".to_string());
    }
    validate_blocked_outputs_null(&record, "REV-L1-02 perimeter")?;
    for (path, phrase) in [
        (
            REV_LEVEL_1_BASE_PERIMETER_SCHEMA_PATH,
            "definition-and-custody boundary",
        ),
        (
            REV_LEVEL_1_BASE_PERIMETER_READER_PATH,
            "definition-only boundary",
        ),
        (REV_LEVEL_1_BASE_PERIMETER_REVIEW_PATH, "approve REV-L1-02"),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("REV-L1-02 prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_rev_level_1_individual_income_tax_fiscal_timing_bridge(
    root: &Path,
) -> Result<(), String> {
    for path in [
        REV_LEVEL_1_TIMING_BRIDGE_JSON_PATH,
        REV_LEVEL_1_TIMING_BRIDGE_SCHEMA_PATH,
        REV_LEVEL_1_TIMING_BRIDGE_READER_PATH,
        REV_LEVEL_1_TIMING_BRIDGE_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV-L1-03 artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, REV_LEVEL_1_TIMING_BRIDGE_JSON_PATH)?;
    for field in [
        "legal_perimeter_path",
        "irs_soi_context_path",
        "omb_receipt_context_path",
        "treasury_mts_metadata_path",
        "form1040_metadata_path",
    ] {
        let path = string_field(&record, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("REV-L1-03 dependency missing: {path}"));
        }
    }
    let treasury_raw = root.join("data/raw/treasury/SRC-TREASURY-MTS-ACCOUNTING-BASIS-2026/2026-07-27/monthly-treasury-statement.html");
    if !treasury_raw.is_file()
        || sha256_file(&treasury_raw)?
            != "7e044cc77f0723f5d7ad3e2727e471e27c4742a5aa3f1c489ff05c5ccb99d817"
    {
        return Err("REV-L1-03 Treasury raw custody failed".to_string());
    }
    let components = record
        .get("required_bridge_components")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV-L1-03 bridge components")?;
    let decision = record
        .get("bridge_decision")
        .ok_or("REV-L1-03 bridge decision")?;
    if string_field(&record, "record_id")?
        != "rev-level-1-individual-income-tax-fiscal-timing-bridge:v1"
        || string_field(&record, "status")? != "rev_l1_03_complete_bounded_reblock"
        || int_field(&record, "pulse")? != 414
        || string_field(&record, "work_package_id")? != "REV-L1-03"
        || components.len() != 7
        || components.iter().any(|row| {
            !bool_field(row, "required").unwrap_or(false)
                || bool_field(row, "cohort_assignment_ready").unwrap_or(true)
        })
        || !bool_field(decision, "direct_division_rejected")?
        || bool_field(decision, "ty2023_to_fy2025_same_period")?
        || bool_field(decision, "liability_equals_cash_receipts")?
        || bool_field(decision, "complete_cohort_bridge_available")?
        || bool_field(decision, "matched_fy2025_assigned_base_ready")?
        || !bool_field(decision, "rev_l1_03_complete_as_bounded_reblock")?
        || !bool_field(decision, "rev_l1_04_may_start")?
        || bool_field(decision, "rev_l1_05_may_start")?
        || bool_field(decision, "rate_calculation_allowed")?
    {
        return Err("REV-L1-03 tax/fiscal timing bridge failed".to_string());
    }
    validate_blocked_outputs_null(&record, "REV-L1-03 timing bridge")?;
    for (path, phrase) in [
        (REV_LEVEL_1_TIMING_BRIDGE_SCHEMA_PATH, "bounded reblock"),
        (REV_LEVEL_1_TIMING_BRIDGE_READER_PATH, "must not be divided"),
        (REV_LEVEL_1_TIMING_BRIDGE_REVIEW_PATH, "approve REV-L1-03"),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("REV-L1-03 prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_rev_level_1_individual_income_rate_planning_ladder(root: &Path) -> Result<(), String> {
    for path in [
        REV_LEVEL_1_RATE_LADDER_JSON_PATH,
        REV_LEVEL_1_RATE_LADDER_SCHEMA_PATH,
        REV_LEVEL_1_RATE_LADDER_READER_PATH,
        REV_LEVEL_1_RATE_LADDER_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV rate ladder artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, REV_LEVEL_1_RATE_LADDER_JSON_PATH)?;
    for field in [
        "start_path",
        "core_g_path",
        "irs_base_context_path",
        "irs_2026_rate_metadata_path",
    ] {
        let path = string_field(&record, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("REV rate ladder dependency missing: {path}"));
        }
    }
    let raw =
        root.join("data/raw/irs/SRC-IRS-REVPROC-2025-32-2026-RATES/2026-07-27/2025-45-IRB.html");
    if !raw.is_file()
        || sha256_file(&raw)? != "796bf8979c990b99a620e1531732161f6e2539f9623ad26d9ca4a8b1c4dbbc5d"
    {
        return Err("REV rate ladder statutory-rate custody failed".to_string());
    }
    let current = record
        .get("current_law_2026")
        .ok_or("REV current rate schedule")?;
    let rates = current
        .get("ordinary_income_statutory_rates_percent")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV current rates")?;
    let anchors = record.get("planning_anchors").ok_or("REV rate anchors")?;
    let rows = record
        .get("rate_ladders")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV rate ladder rows")?;
    let decision = record
        .get("closure_decision")
        .ok_or("REV rate ladder decision")?;
    if string_field(&record, "record_id")?
        != "rev-level-1-individual-income-rate-planning-ladder:v1"
        || string_field(&record, "status")? != "planning_rates_computed_public_rate_card_blocked"
        || int_field(&record, "pulse")? != 420
        || rates
            .iter()
            .filter_map(serde_json::Value::as_i64)
            .collect::<Vec<_>>()
            != vec![10, 12, 22, 24, 32, 35, 37]
        || int_field(anchors, "cbo_fy2026_primary_deficit_millions")? != 813_727
        || int_field(anchors, "cbo_fy2026_total_deficit_millions")? != 1_852_703
        || bool_field(anchors, "cbo_option_current_law_compatible")?
        || rows.len() != 9
        || (number_field(&rows[2], "static_ty2023_base_uplift_points")? - 2.699).abs() > 0.0001
        || (number_field(&rows[2], "cbo_jct_option_uplift_points")? - 2.951).abs() > 0.0001
        || !bool_field(decision, "numeric_planning_rates_ready")?
        || bool_field(decision, "current_law_scored_rate_yield_ready")?
        || bool_field(decision, "assigned_rate_ready")?
        || bool_field(decision, "public_rate_card_ready")?
        || !bool_field(decision, "rev_l1_04_remains_active")?
    {
        return Err("REV Level-1 rate planning ladder failed".to_string());
    }
    validate_blocked_outputs_null(&record, "REV rate planning ladder")?;
    Ok(())
}

pub(crate) fn validate_rev_level_1_post_2025_rate_rescore_proxy(root: &Path) -> Result<(), String> {
    for path in [
        REV_LEVEL_1_POST_2025_PROXY_JSON_PATH,
        REV_LEVEL_1_POST_2025_PROXY_SCHEMA_PATH,
        REV_LEVEL_1_POST_2025_PROXY_READER_PATH,
        REV_LEVEL_1_POST_2025_PROXY_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV post-2025 proxy artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, REV_LEVEL_1_POST_2025_PROXY_JSON_PATH)?;
    for field in [
        "rate_ladder_path",
        "cbo_revenue_context_path",
        "omb_receipt_context_path",
        "core_g_path",
        "distribution_screen_path",
    ] {
        let path = string_field(&record, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("REV post-2025 proxy dependency missing: {path}"));
        }
    }
    let method = record.get("proxy_method").ok_or("REV proxy method")?;
    let cases = record
        .get("primary_balance_cases")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV proxy cases")?;
    let distribution = record
        .get("distribution_alternative")
        .ok_or("REV proxy distribution alternative")?;
    let decision = record.get("decision").ok_or("REV proxy decision")?;
    if string_field(&record, "record_id")? != "rev-level-1-post-2025-rate-rescore-proxy:v1"
        || string_field(&record, "status")?
            != "post_2025_current_law_proxy_ready_formal_score_blocked"
        || int_field(&record, "pulse")? != 421
        || int_field(method, "partial_fiscal_year_months")? != 9
        || (number_field(method, "all_bracket_one_point_proxy_yield_millions")? - 113_668.423).abs()
            > 0.001
        || (number_field(method, "top_four_one_point_proxy_yield_millions")? - 31_144.872).abs()
            > 0.001
        || bool_field(method, "formal_jct_score")?
        || cases.len() != 4
        || int_field(&cases[2], "annual_spending_reduction_millions")? != 500_000
        || (number_field(&cases[2], "all_bracket_uplift_points")? - 2.760).abs() > 0.0001
        || (number_field(distribution, "top_four_only_uplift_points")? - 10.073).abs() > 0.0001
        || bool_field(distribution, "distribution_score_ready")?
        || !bool_field(decision, "post_2025_proxy_ready")?
        || !bool_field(decision, "numeric_all_bracket_schedule_ready")?
        || bool_field(decision, "formal_current_law_score_ready")?
        || bool_field(decision, "assigned_rate_ready")?
        || bool_field(decision, "public_rate_card_ready")?
    {
        return Err("REV Level-1 post-2025 rescore proxy failed".to_string());
    }
    validate_blocked_outputs_null(&record, "REV post-2025 proxy")?;
    Ok(())
}

pub(crate) fn validate_rev_level_2_rate_reconciliation(root: &Path) -> Result<(), String> {
    let required_paths = [
        REV_LEVEL_2_RECONCILIATION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_2_zero_admitted_spending_reconciliation.schema.md",
        "docs/reading/rev-level-2-zero-admitted-spending-reconciliation.md",
        "reviews/2026-07-27-rev-level-2-zero-admitted-spending-reconciliation-role-review.md",
        REV_LEVEL_2_AUDIT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_2_formal_rate_gate_audit.schema.md",
        "docs/reading/rev-level-2-formal-rate-gate-audit.md",
        "reviews/2026-07-27-rev-level-2-formal-rate-gate-audit-role-review.md",
        FISCAL_PACKAGE_RATE_READINESS_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fiscal_package_rate_readiness_bridge.schema.md",
        "docs/reading/fiscal-package-rate-readiness-bridge.md",
        "reviews/2026-07-27-fiscal-package-rate-readiness-bridge-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing REV Level-2 rate artifact: {path}"));
        }
    }

    let reconciliation = read_json_artifact(root, REV_LEVEL_2_RECONCILIATION_JSON_PATH)?;
    let package = reconciliation
        .get("package_reconciliation")
        .ok_or("REV Level-2 package reconciliation")?;
    let mechanical = reconciliation
        .get("mechanical_reconciliation")
        .ok_or("REV Level-2 mechanical reconciliation")?;
    let decision = reconciliation
        .get("decision")
        .ok_or("REV Level-2 reconciliation decision")?;
    if int_field(&reconciliation, "pulse")? != 440
        || int_field(package, "fy2026_primary_deficit_millions")? != 813727
        || int_field(package, "admitted_spending_correction_millions")? != 0
        || int_field(package, "reconciled_remaining_revenue_need_millions")? != 813727
        || (number_field(package, "level_1_guarded_uplift_points")? - 2.760).abs() > 0.0001
        || bool_field(package, "level_1_schedule_valid_for_active_package")?
        || (number_field(mechanical, "uniform_uplift_points_rounded")? - 7.159).abs() > 0.0001
        || (number_field(mechanical, "static_ty2023_base_uplift_points")? - 7.000).abs() > 0.0001
        || (number_field(mechanical, "december_2024_unscaled_option_uplift_points")? - 7.655).abs()
            > 0.0001
        || bool_field(mechanical, "is_formal_current_law_score")?
        || bool_field(mechanical, "is_assigned_rate")?
        || !bool_field(
            decision,
            "level_1_guarded_schedule_superseded_for_active_package",
        )?
        || bool_field(decision, "formal_rate_ready")?
        || bool_field(decision, "assigned_rate_ready")?
    {
        return Err("REV Level-2 zero-spending reconciliation failed".to_string());
    }
    validate_blocked_outputs_null(&reconciliation, "REV Level-2 reconciliation")?;

    let audit = read_json_artifact(root, REV_LEVEL_2_AUDIT_JSON_PATH)?;
    let gates = audit
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-2 audit gates")?;
    let audit_decision = audit.get("decision").ok_or("REV Level-2 audit decision")?;
    let pass_count = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|value| value == "evidence_pass"))
        .count();
    let blocked_count = gates
        .iter()
        .filter(|row| {
            string_field(row, "disposition").is_ok_and(|value| value == "required_blocked")
        })
        .count();
    if int_field(&audit, "pulse")? != 441
        || gates.len() != 8
        || pass_count != 2
        || blocked_count != 6
        || !bool_field(audit_decision, "mechanical_rate_sensitivity_valid")?
        || bool_field(audit_decision, "formal_rate_valid")?
        || bool_field(audit_decision, "assigned_rate_valid")?
        || !bool_field(audit_decision, "rev_level_3_score_acquisition_may_start")?
    {
        return Err("REV Level-2 formal-rate gate audit failed".to_string());
    }
    validate_blocked_outputs_null(&audit, "REV Level-2 audit")?;

    let bridge = read_json_artifact(root, FISCAL_PACKAGE_RATE_READINESS_JSON_PATH)?;
    let rate_state = bridge.get("rate_state").ok_or("rate readiness state")?;
    let bridge_decision = bridge.get("decision").ok_or("rate readiness decision")?;
    if int_field(&bridge, "pulse")? != 442
        || (number_field(rate_state, "superseded_guarded_uplift_points")? - 2.760).abs() > 0.0001
        || (number_field(rate_state, "active_mechanical_uplift_points")? - 7.159).abs() > 0.0001
        || !rate_state
            .get("formal_scored_uplift_points")
            .is_some_and(serde_json::Value::is_null)
        || !rate_state
            .get("assigned_schedule_percent")
            .is_some_and(serde_json::Value::is_null)
        || bool_field(bridge_decision, "level_1_guarded_schedule_active")?
        || !bool_field(bridge_decision, "mechanical_rate_sensitivity_active")?
        || bool_field(bridge_decision, "proper_rate_ready")?
        || bool_field(bridge_decision, "rate_published")?
        || string_field(bridge_decision, "next_decisive_track")? != "REV-Level-3"
    {
        return Err("fiscal-package rate readiness bridge failed".to_string());
    }
    validate_blocked_outputs_null(&bridge, "fiscal-package rate readiness bridge")?;
    Ok(())
}

pub(crate) fn validate_rev_level_3_microsimulation_rate(root: &Path) -> Result<(), String> {
    let required_paths = [
        REV_LEVEL_3_MICROSIMULATION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_3_taxcalc_microsimulation_score.schema.md",
        "docs/reading/rev-level-3-taxcalc-microsimulation-score.md",
        "reviews/2026-07-27-rev-level-3-taxcalc-microsimulation-score-role-review.md",
        "data/metadata/SRC-TAXCALC-6.5.1-CPS-2026.2026-07-27.metadata.md",
        "experiments/rev-level-3-taxcalc/requirements.txt",
        "experiments/rev-level-3-taxcalc/run.py",
        REV_LEVEL_3_AUDIT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_3_rate_admission_audit.schema.md",
        "docs/reading/rev-level-3-rate-admission-audit.md",
        "reviews/2026-07-27-rev-level-3-rate-admission-audit-role-review.md",
        FISCAL_PACKAGE_PROVISIONAL_RATE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fiscal_package_provisional_rate_bridge.schema.md",
        "docs/reading/fiscal-package-provisional-rate-bridge.md",
        "reviews/2026-07-27-fiscal-package-provisional-rate-bridge-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!(
                "missing REV Level-3 microsimulation artifact: {path}"
            ));
        }
    }

    let score = read_json_artifact(root, REV_LEVEL_3_MICROSIMULATION_JSON_PATH)?;
    let model = score.get("model").ok_or("REV Level-3 model")?;
    let static_score = score
        .get("static_validation")
        .ok_or("REV Level-3 static validation")?;
    let central = score
        .get("central_behavioral_score")
        .ok_or("REV Level-3 central score")?;
    let range = score
        .get("elasticity_range")
        .ok_or("REV Level-3 elasticity range")?;
    let distribution = score
        .get("central_distribution_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-3 distribution")?;
    let score_decision = score.get("decision").ok_or("REV Level-3 score decision")?;
    if int_field(&score, "pulse")? != 443
        || string_field(model, "version")? != "6.5.1"
        || int_field(model, "tax_year")? != 2026
        || bool_field(model, "official_jct_or_cbo_score")?
        || (number_field(static_score, "uplift_points")? - 7.159).abs() > 0.0001
        || (number_field(static_score, "change_billions")? - 814.881).abs() > 0.001
        || (number_field(central, "uplift_points")? - 8.352523).abs() > 0.000001
        || (number_field(central, "behavioral_change_billions")? - 813.827).abs() > 0.001
        || (number_field(central, "substitution_elasticity")? - 0.25).abs() > 0.0001
        || distribution.len() != 10
        || (number_field(&distribution[9], "share_percent")? - 52.61).abs() > 0.001
        || !bool_field(score_decision, "provisional_model_rate_ready")?
        || bool_field(score_decision, "official_score_ready")?
        || bool_field(score_decision, "assigned_rate_ready")?
    {
        return Err("REV Level-3 microsimulation score failed".to_string());
    }
    let provisional_range = range
        .get("provisional_uplift_range_points")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-3 provisional range")?;
    if provisional_range.len() != 2
        || (provisional_range[0].as_f64().ok_or("REV range low")? - 7.817).abs() > 0.0001
        || (provisional_range[1].as_f64().ok_or("REV range high")? - 8.986).abs() > 0.0001
    {
        return Err("REV Level-3 elasticity range failed".to_string());
    }
    validate_blocked_outputs_null(&score, "REV Level-3 microsimulation")?;

    let audit = read_json_artifact(root, REV_LEVEL_3_AUDIT_JSON_PATH)?;
    let gates = audit
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-3 gates")?;
    let candidate = audit
        .get("candidate_decision")
        .ok_or("REV Level-3 candidate")?;
    let pass_count = gates
        .iter()
        .filter(|row| {
            string_field(row, "disposition").is_ok_and(|value| value == "model_evidence_pass")
        })
        .count();
    let blocked_count = gates
        .iter()
        .filter(|row| {
            string_field(row, "disposition").is_ok_and(|value| value == "required_blocked")
        })
        .count();
    if int_field(&audit, "pulse")? != 444
        || gates.len() != 6
        || pass_count != 3
        || blocked_count != 3
        || (number_field(candidate, "provisional_model_uplift_points")? - 8.353).abs() > 0.0001
        || !bool_field(candidate, "provisional_model_rate_ready")?
        || bool_field(candidate, "proper_fiscal_rate_ready")?
        || bool_field(candidate, "assigned_rate_ready")?
    {
        return Err("REV Level-3 rate admission audit failed".to_string());
    }
    validate_blocked_outputs_null(&audit, "REV Level-3 audit")?;

    let bridge = read_json_artifact(root, FISCAL_PACKAGE_PROVISIONAL_RATE_JSON_PATH)?;
    let progression = bridge
        .get("rate_progression")
        .ok_or("provisional rate progression")?;
    let alignment = bridge
        .get("score_alignment")
        .ok_or("provisional rate alignment")?;
    let decision = bridge.get("decision").ok_or("provisional rate decision")?;
    if int_field(&bridge, "pulse")? != 445
        || (number_field(progression, "static_tax_unit_uplift_points")? - 7.159).abs() > 0.0001
        || (number_field(progression, "provisional_behavioral_uplift_points")? - 8.353).abs()
            > 0.0001
        || !progression
            .get("assigned_uplift_points")
            .is_some_and(serde_json::Value::is_null)
        || (number_field(alignment, "central_modeled_liability_change_billions")? - 813.827).abs()
            > 0.001
        || bool_field(alignment, "period_match")?
        || bool_field(alignment, "official_score")?
        || !bool_field(decision, "provisional_model_rate_ready")?
        || bool_field(decision, "proper_fiscal_rate_ready")?
        || bool_field(decision, "rate_assigned")?
        || string_field(decision, "next_decisive_track")? != "REV-Level-4"
    {
        return Err("fiscal-package provisional rate bridge failed".to_string());
    }
    validate_blocked_outputs_null(&bridge, "fiscal-package provisional rate bridge")?;
    Ok(())
}

pub(crate) fn validate_rev_level_4_fiscal_timing_rate(root: &Path) -> Result<(), String> {
    let required_paths = [
        REV_LEVEL_4_TIMING_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_4_first_year_cash_timing_bridge.schema.md",
        "docs/reading/rev-level-4-first-year-cash-timing-bridge.md",
        "reviews/2026-07-27-rev-level-4-first-year-cash-timing-bridge-role-review.md",
        REV_LEVEL_4_AUDIT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_4_assignment_cost_macro_audit.schema.md",
        "docs/reading/rev-level-4-assignment-cost-macro-audit.md",
        "reviews/2026-07-27-rev-level-4-assignment-cost-macro-audit-role-review.md",
        FISCAL_PACKAGE_FISCAL_TIMING_RATE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fiscal_package_fiscal_timing_rate_bridge.schema.md",
        "docs/reading/fiscal-package-fiscal-timing-rate-bridge.md",
        "reviews/2026-07-27-fiscal-package-fiscal-timing-rate-bridge-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!(
                "missing REV Level-4 fiscal-timing artifact: {path}"
            ));
        }
    }

    let timing = read_json_artifact(root, REV_LEVEL_4_TIMING_JSON_PATH)?;
    let anchor = timing
        .get("official_timing_anchor")
        .ok_or("REV Level-4 timing anchor")?;
    let central = timing
        .get("central_timing_solve")
        .ok_or("REV Level-4 central timing solve")?;
    let elasticity = timing
        .get("elasticity_timing_range")
        .ok_or("REV Level-4 elasticity timing range")?;
    let decision = timing
        .get("decision")
        .ok_or("REV Level-4 timing decision")?;
    if int_field(&timing, "pulse")? != 446
        || (number_field(anchor, "first_fiscal_year_effect_billions")? - 82.3).abs() > 0.0001
        || (number_field(anchor, "next_fiscal_year_effect_billions")? - 106.3).abs() > 0.0001
        || (number_field(anchor, "first_year_realization_ratio")? - 0.774223895).abs() > 0.000000001
        || bool_field(anchor, "current_law_2026_score")?
        || (number_field(central, "fy2026_cash_target_billions")? - 813.727).abs() > 0.001
        || (number_field(central, "required_full_tax_year_liability_change_billions")?
            - 1051.022844)
            .abs()
            > 0.000001
        || (number_field(central, "substitution_elasticity")? - 0.25).abs() > 0.0001
        || (number_field(central, "uplift_points")? - 10.922).abs() > 0.0001
        || (number_field(central, "modeled_full_year_liability_change_billions")? - 1051.017).abs()
            > 0.001
        || (number_field(central, "first_year_cash_proxy_billions")? - 813.722).abs() > 0.001
        || (number_field(central, "target_difference_billions")? + 0.005).abs() > 0.001
        || !bool_field(decision, "fiscal_timing_adjusted_model_rate_ready")?
        || bool_field(decision, "formal_current_law_cash_score_ready")?
        || bool_field(decision, "administration_adjusted_rate_ready")?
        || bool_field(decision, "macro_adjusted_rate_ready")?
        || bool_field(decision, "assigned_rate_ready")?
    {
        return Err("REV Level-4 first-year cash timing bridge failed".to_string());
    }
    let timing_range = elasticity
        .get("fiscal_timing_uplift_range_points")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-4 timing uplift range")?;
    if timing_range.len() != 2
        || (timing_range[0].as_f64().ok_or("REV Level-4 range low")? - 10.159).abs() > 0.0001
        || (timing_range[1].as_f64().ok_or("REV Level-4 range high")? - 11.853).abs() > 0.0001
    {
        return Err("REV Level-4 fiscal-timing range failed".to_string());
    }
    validate_blocked_outputs_null(&timing, "REV Level-4 timing bridge")?;

    let audit = read_json_artifact(root, REV_LEVEL_4_AUDIT_JSON_PATH)?;
    let gates = audit
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-4 audit gates")?;
    let administration = audit
        .get("administration_context")
        .ok_or("REV Level-4 administration context")?;
    let candidate = audit
        .get("candidate_decision")
        .ok_or("REV Level-4 candidate decision")?;
    let pass_count = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|v| v == "proxy_evidence_pass"))
        .count();
    let blocked_count = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|v| v == "required_blocked"))
        .count();
    if int_field(&audit, "pulse")? != 447
        || gates.len() != 5
        || pass_count != 3
        || blocked_count != 2
        || (number_field(
            administration,
            "irs_fy2025_total_operating_expenditures_billions",
        )? - 19.0)
            .abs()
            > 0.0001
        || (number_field(administration, "whole_agency_scale_share_of_target_percent")? - 2.335)
            .abs()
            > 0.001
        || bool_field(administration, "is_marginal_policy_cost")?
        || !administration
            .get("withholding_table_transition_cost")
            .is_some_and(serde_json::Value::is_null)
        || (number_field(candidate, "fiscal_timing_uplift_points")? - 10.922).abs() > 0.0001
        || !bool_field(candidate, "fiscal_timing_adjusted_model_rate_ready")?
        || bool_field(candidate, "administration_adjusted_rate_ready")?
        || bool_field(candidate, "macro_adjusted_rate_ready")?
        || bool_field(candidate, "proper_assigned_rate_ready")?
        || !bool_field(candidate, "rev_level_5_may_start")?
    {
        return Err("REV Level-4 assignment cost/macro audit failed".to_string());
    }
    validate_blocked_outputs_null(&audit, "REV Level-4 assignment audit")?;

    let bridge = read_json_artifact(root, FISCAL_PACKAGE_FISCAL_TIMING_RATE_JSON_PATH)?;
    let progression = bridge
        .get("rate_progression")
        .ok_or("fiscal-timing rate progression")?;
    let alignment = bridge
        .get("score_alignment")
        .ok_or("fiscal-timing score alignment")?;
    let bridge_decision = bridge
        .get("decision")
        .ok_or("fiscal-timing rate decision")?;
    if int_field(&bridge, "pulse")? != 448
        || (number_field(progression, "superseded_guarded_uplift_points")? - 2.760).abs() > 0.0001
        || (number_field(progression, "static_full_year_uplift_points")? - 7.159).abs() > 0.0001
        || (number_field(progression, "behavioral_full_year_uplift_points")? - 8.353).abs() > 0.0001
        || (number_field(progression, "fiscal_timing_behavioral_uplift_points")? - 10.922).abs()
            > 0.0001
        || !progression
            .get("administration_macro_adjusted_uplift_points")
            .is_some_and(serde_json::Value::is_null)
        || !progression
            .get("assigned_uplift_points")
            .is_some_and(serde_json::Value::is_null)
        || (number_field(alignment, "fy2026_cash_target_billions")? - 813.727).abs() > 0.001
        || (number_field(alignment, "first_year_cash_proxy_billions")? - 813.722).abs() > 0.001
        || (number_field(alignment, "difference_billions")? + 0.005).abs() > 0.001
        || (number_field(alignment, "timing_ratio")? - 0.774223895).abs() > 0.000000001
        || bool_field(alignment, "timing_ratio_current_law_match")?
        || bool_field(alignment, "official_score")?
        || !bool_field(bridge_decision, "fiscal_timing_rate_ready")?
        || bool_field(bridge_decision, "proper_assigned_rate_ready")?
        || bool_field(bridge_decision, "rate_assigned")?
        || string_field(bridge_decision, "next_decisive_track")? != "REV-Level-5"
    {
        return Err("fiscal-package fiscal-timing rate bridge failed".to_string());
    }
    validate_blocked_outputs_null(&bridge, "fiscal-package fiscal-timing rate bridge")?;
    Ok(())
}

pub(crate) fn validate_rev_level_5_administration_macro_rate(root: &Path) -> Result<(), String> {
    let required_paths = [
        REV_LEVEL_5_ADMINISTRATION_CEILING_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_5_administration_implementation_ceiling.schema.md",
        "docs/reading/rev-level-5-administration-implementation-ceiling.md",
        "reviews/2026-07-27-rev-level-5-administration-implementation-ceiling-role-review.md",
        "data/metadata/SRC-TREASURY-FY2021-CJ-TCJA-IMPLEMENTATION.2026-07-27.metadata.md",
        "data/metadata/SRC-GAO-18-548-WITHHOLDING.2026-07-27.metadata.md",
        REV_LEVEL_5_MACRO_AUDIT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_5_macro_assignment_methodology_audit.schema.md",
        "docs/reading/rev-level-5-macro-assignment-methodology-audit.md",
        "reviews/2026-07-27-rev-level-5-macro-assignment-methodology-audit-role-review.md",
        "data/metadata/SRC-JCT-REVENUE-ESTIMATING-PROCESS-2025.2026-07-27.metadata.md",
        FISCAL_PACKAGE_ADMINISTRATION_BOUNDED_RATE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/fiscal_package_administration_bounded_rate_bridge.schema.md",
        "docs/reading/fiscal-package-administration-bounded-rate-bridge.md",
        "reviews/2026-07-27-fiscal-package-administration-bounded-rate-bridge-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing REV Level-5 artifact: {path}"));
        }
    }

    let administration = read_json_artifact(root, REV_LEVEL_5_ADMINISTRATION_CEILING_JSON_PATH)?;
    let evidence = administration
        .get("official_evidence")
        .ok_or("REV Level-5 administration evidence")?;
    let ceiling = administration
        .get("ceiling_calculation")
        .ok_or("REV Level-5 ceiling calculation")?;
    let decision = administration
        .get("decision")
        .ok_or("REV Level-5 administration decision")?;
    if int_field(&administration, "pulse")? != 449
        || (number_field(evidence, "historical_irs_implementation_funding_millions")? - 77.0).abs()
            > 0.0001
        || (number_field(evidence, "taxpayer_services_allocation_millions")? - 65.0).abs() > 0.0001
        || (number_field(evidence, "enforcement_allocation_millions")? - 12.0).abs() > 0.0001
        || bool_field(
            evidence,
            "historical_funding_is_marginal_modeled_policy_cost",
        )?
        || (number_field(ceiling, "historical_agency_ceiling_billions")? - 0.077).abs() > 0.0001
        || (number_field(ceiling, "linearized_ceiling_addon_points")? - 0.001034).abs() > 0.000001
        || (number_field(ceiling, "administration_ceiling_planning_uplift_points")? - 10.923).abs()
            > 0.0001
        || !bool_field(decision, "historical_agency_ceiling_ready")?
        || bool_field(decision, "marginal_irs_policy_cost_ready")?
        || bool_field(decision, "employer_payroll_provider_cost_ready")?
        || bool_field(decision, "taxpayer_compliance_cost_ready")?
        || !bool_field(
            decision,
            "administration_ceiling_planning_sensitivity_ready",
        )?
        || bool_field(decision, "administration_adjusted_rate_ready")?
        || bool_field(decision, "assigned_rate_ready")?
    {
        return Err("REV Level-5 administration ceiling failed".to_string());
    }
    validate_blocked_outputs_null(&administration, "REV Level-5 administration ceiling")?;

    let macro_audit = read_json_artifact(root, REV_LEVEL_5_MACRO_AUDIT_JSON_PATH)?;
    let requirements = macro_audit
        .get("methodology_requirements")
        .ok_or("REV Level-5 macro requirements")?;
    let gates = macro_audit
        .get("gate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-5 macro gates")?;
    let macro_decision = macro_audit
        .get("candidate_decision")
        .ok_or("REV Level-5 macro decision")?;
    let method_passes = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|v| v == "method_evidence_pass"))
        .count();
    let blocked = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|v| v == "required_blocked"))
        .count();
    for field in [
        "separate_present_law_model",
        "separate_proposed_law_model",
        "calibrated_macroeconomic_models",
        "conventional_revenue_estimate_subtracted",
        "model_weighting_for_point_feedback",
        "debt_service_feedback_separately_accounted",
    ] {
        if !bool_field(requirements, field)? {
            return Err(format!("REV Level-5 macro requirement failed: {field}"));
        }
    }
    if int_field(&macro_audit, "pulse")? != 450
        || bool_field(
            requirements,
            "scalar_transfer_from_unrelated_legislation_allowed",
        )?
        || gates.len() != 5
        || method_passes != 1
        || blocked != 4
        || !macro_decision
            .get("macro_feedback_value")
            .is_some_and(serde_json::Value::is_null)
        || bool_field(macro_decision, "macro_adjusted_rate_ready")?
        || bool_field(macro_decision, "proper_assigned_rate_ready")?
        || !bool_field(macro_decision, "historical_macro_ratio_transfer_prohibited")?
        || !bool_field(macro_decision, "rev_level_5_complete")?
        || !bool_field(macro_decision, "rev_level_6_may_start")?
    {
        return Err("REV Level-5 macro methodology audit failed".to_string());
    }
    validate_blocked_outputs_null(&macro_audit, "REV Level-5 macro audit")?;

    let bridge = read_json_artifact(root, FISCAL_PACKAGE_ADMINISTRATION_BOUNDED_RATE_JSON_PATH)?;
    let progression = bridge
        .get("rate_progression")
        .ok_or("administration-bounded rate progression")?;
    let boundary = bridge
        .get("scope_boundary")
        .ok_or("administration-bounded scope")?;
    let bridge_decision = bridge
        .get("decision")
        .ok_or("administration-bounded decision")?;
    if int_field(&bridge, "pulse")? != 451
        || (number_field(progression, "fiscal_timing_behavioral_uplift_points")? - 10.922).abs()
            > 0.0001
        || (number_field(progression, "historical_agency_ceiling_addon_points")? - 0.001034).abs()
            > 0.000001
        || (number_field(progression, "administration_ceiling_planning_uplift_points")? - 10.923)
            .abs()
            > 0.0001
        || !progression
            .get("macro_adjusted_uplift_points")
            .is_some_and(serde_json::Value::is_null)
        || bool_field(boundary, "historical_agency_ceiling_is_marginal_cost")?
        || bool_field(boundary, "private_compliance_cost_included")?
        || bool_field(boundary, "proposal_specific_macro_feedback_included")?
        || bool_field(boundary, "official_dynamic_score")?
        || !bool_field(bridge_decision, "strongest_bounded_planning_rate_ready")?
        || bool_field(bridge_decision, "proper_assigned_rate_ready")?
        || bool_field(bridge_decision, "rate_assigned")?
        || string_field(bridge_decision, "next_decisive_track")? != "REV-Level-6"
    {
        return Err("fiscal-package administration-bounded rate bridge failed".to_string());
    }
    validate_blocked_outputs_null(&bridge, "fiscal-package administration-bounded rate bridge")?;
    Ok(())
}

pub(crate) fn validate_rev_level_6_public_planning_rate(root: &Path) -> Result<(), String> {
    let required_paths = [
        REV_LEVEL_6_POLICY_DECISION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_6_policy_rate_decision.schema.md",
        "docs/reading/rev-level-6-policy-rate-decision.md",
        "reviews/2026-07-27-rev-level-6-policy-rate-decision-role-review.md",
        REV_LEVEL_6_DOSSIER_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_6_revenue_instrument_dossier.schema.md",
        "docs/reading/rev-level-6-revenue-instrument-dossier.md",
        "reviews/2026-07-27-rev-level-6-revenue-instrument-dossier-role-review.md",
        REV_F_PLANNING_RATE_CARD_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_f_public_planning_rate_card.schema.md",
        "docs/reading/rev-f-public-planning-rate-card.md",
        "reviews/2026-07-27-rev-f-public-planning-rate-card-role-review.md",
    ];
    for path in required_paths {
        if !root.join(path).is_file() {
            return Err(format!("missing REV Level-6/REV-F artifact: {path}"));
        }
    }

    let policy = read_json_artifact(root, REV_LEVEL_6_POLICY_DECISION_JSON_PATH)?;
    let rule = policy
        .get("decision_rule")
        .ok_or("REV Level-6 decision rule")?;
    let schedule = policy
        .get("selected_planning_schedule")
        .ok_or("REV Level-6 selected schedule")?;
    let run = policy
        .get("selected_run")
        .ok_or("REV Level-6 selected run")?;
    let decision = policy.get("decision").ok_or("REV Level-6 decision")?;
    let assigned_schedule = schedule
        .get("assigned_planning_schedule_percent")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-6 assigned schedule")?;
    let expected_schedule = [21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0];
    if int_field(&policy, "pulse")? != 452
        || (number_field(rule, "central_bounded_uplift_points")? - 10.923).abs() > 0.0001
        || string_field(rule, "rounding_direction")? != "up"
        || assigned_schedule.len() != expected_schedule.len()
        || assigned_schedule
            .iter()
            .zip(expected_schedule)
            .any(|(value, expected)| {
                value
                    .as_f64()
                    .is_none_or(|observed| (observed - expected).abs() > 0.0001)
            })
        || (number_field(schedule, "assigned_planning_uplift_points")? - 11.0).abs() > 0.0001
        || !bool_field(schedule, "bracket_thresholds_unchanged")?
        || bool_field(schedule, "statutory_or_enacted")?
        || (number_field(run, "modeled_full_year_liability_change_billions")? - 1058.117).abs()
            > 0.001
        || (number_field(run, "modeled_first_year_cash_proxy_billions")? - 819.220).abs() > 0.001
        || (number_field(run, "proxy_cushion_billions")? - 5.493).abs() > 0.001
        || (number_field(run, "top_decile_share_percent")? - 52.53).abs() > 0.001
        || (number_field(run, "all_after_tax_income_change_percent")? + 8.326).abs() > 0.001
        || !bool_field(decision, "planning_rate_selected")?
        || !bool_field(decision, "planning_rate_assigned")?
        || !bool_field(decision, "planning_rate_card_may_start")?
        || bool_field(decision, "formal_fiscal_certification_ready")?
        || bool_field(decision, "statutory_proposal_ready")?
        || bool_field(decision, "balanced_budget_claim_ready")?
    {
        return Err("REV Level-6 policy-rate decision failed".to_string());
    }
    validate_blocked_outputs_null(&policy, "REV Level-6 policy-rate decision")?;

    let dossier = read_json_artifact(root, REV_LEVEL_6_DOSSIER_JSON_PATH)?;
    let profile = dossier
        .get("candidate_profile")
        .ok_or("REV Level-6 candidate profile")?;
    let financing = dossier
        .get("financing_roles")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-6 financing roles")?;
    let gates = dossier
        .get("gate_reviews")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-6 gate reviews")?;
    let release = dossier
        .get("release_decision")
        .ok_or("REV Level-6 release decision")?;
    let ready_gates = gates
        .iter()
        .filter(|row| string_field(row, "disposition").is_ok_and(|v| v == "required_ready"))
        .count();
    let not_applicable = gates
        .iter()
        .filter(|row| {
            string_field(row, "disposition").is_ok_and(|v| v == "reviewed_not_applicable")
        })
        .count();
    if int_field(&dossier, "pulse")? != 453
        || string_field(profile, "objective_profile")? != "revenue_instrument"
        || (number_field(profile, "assigned_receipt_rate_uplift_points")? - 11.0).abs() > 0.0001
        || bool_field(profile, "statutory_or_enacted")?
        || financing.len() != 3
        || !financing.iter().any(|row| {
            string_field(row, "role").is_ok_and(|v| v == "receipt_or_fee_base")
                && bool_field(row, "source_supported").unwrap_or(false)
        })
        || gates.len() != 7
        || ready_gates != 6
        || not_applicable != 1
        || !bool_field(release, "dossier_valid")?
        || !bool_field(release, "all_required_gates_ready")?
        || !bool_field(release, "role_review_complete")?
        || !bool_field(release, "release_ready")?
        || !bool_field(release, "rev_f_planning_rate_card_may_start")?
    {
        return Err("REV Level-6 revenue-instrument dossier failed".to_string());
    }
    validate_blocked_outputs_null(&dossier, "REV Level-6 revenue dossier")?;

    let card = read_json_artifact(root, REV_F_PLANNING_RATE_CARD_JSON_PATH)?;
    let identity = card
        .get("release_identity")
        .ok_or("REV-F release identity")?;
    let public_card = card
        .get("public_rate_card")
        .ok_or("REV-F public rate card")?;
    let checks = card
        .get("release_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV-F release checks")?;
    let closure = card
        .get("closure_decision")
        .ok_or("REV-F closure decision")?;
    if int_field(&card, "pulse")? != 454
        || string_field(&card, "track_wave_id")? != "REV-F"
        || string_field(identity, "surface")? != "rate_card"
        || string_field(identity, "objective_profile")? != "revenue_instrument"
        || bool_field(identity, "statutory_or_enacted")?
        || bool_field(identity, "official_score")?
        || (number_field(public_card, "assigned_planning_uplift_points")? - 11.0).abs() > 0.0001
        || (number_field(public_card, "modeled_first_year_cash_proxy_billions")? - 819.220).abs()
            > 0.001
        || (number_field(public_card, "modeled_proxy_cushion_billions")? - 5.493).abs() > 0.001
        || checks.len() != 7
        || checks
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|v| v != "pass"))
        || !bool_field(closure, "rev_f_started")?
        || !bool_field(closure, "rev_f_done")?
        || !bool_field(closure, "public_planning_rate_card_published")?
        || !bool_field(closure, "planning_rate_published")?
        || bool_field(closure, "official_rate_published")?
        || bool_field(closure, "statutory_rate_proposed")?
        || bool_field(closure, "balanced_budget_claim_published")?
    {
        return Err("REV-F public planning rate card failed".to_string());
    }
    validate_blocked_outputs_null(&card, "REV-F planning rate card")?;
    Ok(())
}

pub(crate) fn validate_rev_internal_grid(
    root: &Path,
    path: &str,
    expected_uplifts: &[f64],
) -> Result<(), String> {
    let grid = read_json_artifact(root, path)?;
    let candidates = grid
        .get("candidates")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal grid candidates")?;
    let boundary = grid.get("boundary").ok_or("REV internal grid boundary")?;
    let baseline = [10.0, 12.0, 22.0, 24.0, 32.0, 35.0, 37.0];
    if string_field(&grid, "analysis_scope")? != "independent_taxlane_analysis_only"
        || string_field(&grid, "model")? != "Tax-Calculator 6.5.1"
        || string_field(&grid, "data")? != "bundled CPS tax-unit file"
        || int_field(&grid, "tax_year")? != 2026
        || (number_field(&grid, "first_year_ratio")? - 0.774223895).abs() > 0.000000001
        || (number_field(&grid, "fy2026_revenue_target_billions")? - 813.727).abs() > 0.001
        || candidates.len() != expected_uplifts.len()
        || bool_field(boundary, "official_request_planned")?
        || bool_field(boundary, "official_score")?
        || !bool_field(boundary, "taxlane_internal_analysis")?
        || bool_field(boundary, "administration_macro_and_debt_applied")?
    {
        return Err(format!("REV internal grid identity failed: {path}"));
    }
    for (candidate, expected_uplift) in candidates.iter().zip(expected_uplifts) {
        let uplift = number_field(candidate, "uniform_uplift_points")?;
        let schedule = candidate
            .get("schedule_percent")
            .and_then(serde_json::Value::as_array)
            .ok_or("REV internal grid schedule")?;
        let cases = candidate
            .get("elasticity_cases")
            .and_then(serde_json::Value::as_array)
            .ok_or("REV internal grid elasticity cases")?;
        if (uplift - expected_uplift).abs() > 0.0001
            || schedule.len() != baseline.len()
            || schedule.iter().zip(baseline).any(|(value, base)| {
                value
                    .as_f64()
                    .is_none_or(|rate| (rate - base - uplift).abs() > 0.0001)
            })
            || cases.len() != 3
        {
            return Err(format!("REV internal grid candidate failed: {path}"));
        }
        for (case, expected_elasticity) in cases.iter().zip([0.15, 0.25, 0.35]) {
            let full_year = number_field(case, "full_year_liability_change_billions")?;
            let cash = number_field(case, "first_year_cash_proxy_billions")?;
            let target_difference = number_field(case, "target_difference_billions")?;
            if (number_field(case, "substitution_elasticity")? - expected_elasticity).abs() > 0.0001
                || (cash - full_year * 0.774223895).abs() > 0.002
                || (target_difference - (cash - 813.727)).abs() > 0.002
            {
                return Err(format!("REV internal grid arithmetic failed: {path}"));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_rev_internal_analysis_next_ten_steps(root: &Path) -> Result<(), String> {
    for path in [
        REV_INTERNAL_NEXT_TEN_STEPS_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_internal_analysis_next_ten_steps.schema.md",
        "docs/reading/rev-internal-analysis-next-ten-steps.md",
        "reviews/2026-07-27-rev-internal-analysis-next-ten-steps-role-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-475-independent-analysis-course-correction.md",
        REV_INTERNAL_BASELINE_FREEZE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_internal_analysis_baseline_freeze.schema.md",
        "docs/reading/rev-internal-analysis-baseline-freeze.md",
        "reviews/2026-07-27-rev-internal-analysis-baseline-freeze-role-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-476-rev-internal-analysis-baseline-freeze.md",
        "experiments/rev-level-3-taxcalc/run_grid.py",
        "experiments/rev-level-3-taxcalc/analyze_grid.py",
        REV_INTERNAL_GRID_JSON_PATH,
        REV_INTERNAL_GRID_EXTENSION_JSON_PATH,
        REV_INTERNAL_CANDIDATE_ANALYSIS_JSON_PATH,
        REV_INTERNAL_COMPLETION_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_internal_rate_analysis_completion.schema.md",
        "docs/reading/rev-internal-rate-analysis-completion.md",
        "reviews/2026-07-27-rev-internal-rate-analysis-completion-role-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-477-rev-internal-rate-analysis-completion.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing REV internal-analysis artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, REV_INTERNAL_NEXT_TEN_STEPS_JSON_PATH)?;
    let scope = record
        .get("analysis_scope")
        .ok_or("REV internal-analysis scope")?;
    let steps = record
        .get("steps")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal-analysis steps")?;
    let claims = record
        .get("claim_booleans")
        .ok_or("REV internal-analysis claims")?;
    let rates = scope
        .get("current_model_schedule_percent")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal model schedule")?;
    let expected_rates = [21, 23, 33, 35, 43, 46, 48];
    if int_field(&record, "pulse")? != 477
        || string_field(&record, "status")? != "internal_analysis_ten_steps_complete"
        || !bool_field(scope, "taxlane_independent_analysis_only")?
        || bool_field(scope, "official_request_planned")?
        || bool_field(scope, "external_submission_in_scope")?
        || bool_field(scope, "official_certification_required")?
        || (number_field(scope, "current_uniform_uplift_points")? - 11.0).abs() > 0.0001
        || (number_field(scope, "fy2026_revenue_need_billions")? - 813.727).abs() > 0.001
        || number_field(scope, "admitted_fy2026_primary_spending_reduction_billions")?.abs()
            > 0.0001
        || rates.len() != expected_rates.len()
        || rates
            .iter()
            .zip(expected_rates)
            .any(|(value, expected)| value.as_i64() != Some(expected))
        || steps.len() != 10
        || steps.iter().enumerate().any(|(index, step)| {
            int_field(step, "step").ok() != Some(index as i64 + 1)
                || string_field(step, "work").is_err()
                || string_field(step, "exit").is_err()
                || string_field(step, "status").ok().as_deref() != Some("complete")
        })
        || !bool_field(claims, "independent_analysis_only")?
        || bool_field(claims, "official_request_planned")?
        || bool_field(claims, "external_submission_in_scope")?
        || bool_field(claims, "official_certification_required")?
        || !bool_field(claims, "internal_rate_analysis_started")?
        || !bool_field(claims, "taxlane_analytical_rate_finalized")?
        || bool_field(claims, "official_rate_claimed")?
        || bool_field(claims, "enacted_law_claimed")?
        || bool_field(claims, "balanced_budget_proven")?
    {
        return Err("REV internal-analysis next-ten-steps contract failed".to_string());
    }

    let baseline_freeze = read_json_artifact(root, REV_INTERNAL_BASELINE_FREEZE_JSON_PATH)?;
    let model = baseline_freeze
        .get("model_baseline")
        .ok_or("REV internal baseline model")?;
    let fiscal = baseline_freeze
        .get("fiscal_baseline")
        .ok_or("REV internal baseline fiscal")?;
    let behavior = baseline_freeze
        .get("behavior_baseline")
        .ok_or("REV internal baseline behavior")?;
    let grid_contract = baseline_freeze
        .get("grid_contract")
        .ok_or("REV internal baseline grid contract")?;
    let frozen_inputs = baseline_freeze
        .get("frozen_inputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal frozen inputs")?;
    if int_field(&baseline_freeze, "pulse")? != 476
        || string_field(&baseline_freeze, "status")? != "internal_baseline_frozen_grid_ready"
        || string_field(model, "model")? != "Tax-Calculator"
        || string_field(model, "version")? != "6.5.1"
        || string_field(model, "data")? != "bundled CPS tax-unit file"
        || int_field(model, "tax_year")? != 2026
        || (number_field(fiscal, "fy2026_revenue_need_billions")? - 813.727).abs() > 0.001
        || number_field(fiscal, "admitted_primary_spending_reduction_billions")?.abs() > 0.0001
        || number_field(fiscal, "pay_additive_contribution_billions")?.abs() > 0.0001
        || number_field(fiscal, "net_zero_input_contribution_billions")?.abs() > 0.0001
        || (number_field(fiscal, "first_year_realization_ratio")? - 0.774223895).abs() > 0.000000001
        || (number_field(fiscal, "historical_agency_ceiling_billions")? - 0.077).abs() > 0.0001
        || (number_field(behavior, "central_substitution_elasticity")? - 0.25).abs() > 0.0001
        || int_field(grid_contract, "candidate_count")? != 9
        || int_field(grid_contract, "elasticity_cases_per_candidate")? != 3
        || frozen_inputs.len() != 8
    {
        return Err("REV internal baseline freeze failed".to_string());
    }
    for input in frozen_inputs {
        let path = string_field(input, "path")?;
        if sha256_file(&root.join(&path))? != string_field(input, "sha256")? {
            return Err(format!("REV internal frozen-input hash failed: {path}"));
        }
    }

    validate_rev_internal_grid(
        root,
        REV_INTERNAL_GRID_JSON_PATH,
        &[9.5, 10.0, 10.159, 10.5, 10.922, 11.0, 11.5, 11.854, 12.0],
    )?;
    validate_rev_internal_grid(
        root,
        REV_INTERNAL_GRID_EXTENSION_JSON_PATH,
        &[12.25, 12.5, 12.6, 12.75, 13.0],
    )?;

    let analysis = read_json_artifact(root, REV_INTERNAL_CANDIDATE_ANALYSIS_JSON_PATH)?;
    let uncertainty = analysis
        .get("uncertainty_contract")
        .ok_or("REV internal uncertainty contract")?;
    let rows = analysis
        .get("candidate_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal candidate rows")?;
    let selection = analysis.get("selection").ok_or("REV internal selection")?;
    let analysis_boundary = analysis.get("boundary").ok_or("REV internal boundary")?;
    if rows.len() != 14
        || rows.iter().any(|row| {
            row.get("scenarios")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|scenarios| scenarios.len() != 9)
        })
        || (number_field(uncertainty, "administration_ceiling_billions")? - 0.077).abs() > 0.0001
        || (number_field(uncertainty, "fy2026_average_interest_rate_percent")? - 3.404).abs()
            > 0.0001
        || number_field(uncertainty, "pay_additive_contribution_billions")?.abs() > 0.0001
        || number_field(uncertainty, "net_direct_cut_billions")?.abs() > 0.0001
        || (number_field(selection, "lowest_central_case_uplift_meeting_target")? - 11.0).abs()
            > 0.0001
        || (number_field(
            selection,
            "lowest_all_behavior_case_uplift_meeting_target_without_macro_stress",
        )? - 12.0)
            .abs()
            > 0.0001
        || (number_field(selection, "lowest_all_stress_case_uplift_meeting_target")? - 12.6).abs()
            > 0.0001
        || bool_field(selection, "strict_stress_grid_extension_required")?
        || bool_field(analysis_boundary, "official_request_planned")?
        || bool_field(analysis_boundary, "official_score")?
        || !bool_field(analysis_boundary, "taxlane_internal_analysis")?
        || bool_field(analysis_boundary, "balanced_budget_proven")?
    {
        return Err("REV internal candidate analysis failed".to_string());
    }

    let completion = read_json_artifact(root, REV_INTERNAL_COMPLETION_JSON_PATH)?;
    let reproducibility = completion
        .get("reproducibility")
        .ok_or("REV internal reproducibility")?;
    let tiers = completion
        .get("recommendation_tiers")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal recommendation tiers")?;
    let track_review = completion
        .get("track_dependency_review")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV internal track review")?;
    let aggregate = completion
        .get("aggregate_review")
        .ok_or("REV internal aggregate review")?;
    let decision = completion.get("decision").ok_or("REV internal decision")?;
    let expected_hashes = [
        ("grid_runner_path", "grid_runner_sha256"),
        ("analysis_runner_path", "analysis_runner_sha256"),
        ("initial_grid_path", "initial_grid_sha256"),
        ("extension_grid_path", "extension_grid_sha256"),
        ("candidate_analysis_path", "candidate_analysis_sha256"),
    ];
    for (path_field, hash_field) in expected_hashes {
        let path = if let Ok(path) = string_field(reproducibility, path_field) {
            path
        } else {
            string_field(&completion, path_field)?
        };
        if sha256_file(&root.join(&path))? != string_field(reproducibility, hash_field)? {
            return Err(format!("REV internal reproducibility hash failed: {path}"));
        }
    }
    let observed_tracks = track_review
        .iter()
        .map(|row| string_field(row, "track"))
        .collect::<Result<BTreeSet<_>, _>>()?;
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
    let tier_uplifts = tiers
        .iter()
        .map(|tier| number_field(tier, "uniform_uplift_points"))
        .collect::<Result<Vec<_>, _>>()?;
    if int_field(&completion, "pulse")? != 477
        || string_field(&completion, "status")?
            != "ten_step_internal_analysis_complete_taxlane_recommendation_published"
        || int_field(reproducibility, "candidate_count")? != 14
        || int_field(reproducibility, "behavior_cases_per_candidate")? != 3
        || int_field(reproducibility, "combined_stress_cases_per_candidate")? != 9
        || tier_uplifts != [11.0, 12.0, 12.6]
        || track_review.len() != 15
        || observed_tracks != expected_tracks
        || int_field(aggregate, "tracks_reviewed")? != 15
        || int_field(aggregate, "tracks_with_changed_analysis")? != 2
        || int_field(aggregate, "tracks_retained_or_reviewed_unchanged")? != 13
        || number_field(aggregate, "new_spending_savings_admitted_billions")?.abs() > 0.0001
        || !bool_field(decision, "all_ten_steps_complete")?
        || (number_field(decision, "preferred_uniform_uplift_points")? - 11.0).abs() > 0.0001
        || !bool_field(decision, "taxlane_analytical_rate_finalized")?
        || bool_field(decision, "official_request_planned")?
        || bool_field(decision, "official_rate_claimed")?
        || bool_field(decision, "enacted_law_claimed")?
        || bool_field(decision, "balanced_budget_proven")?
    {
        return Err("REV internal completion contract failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_rev_level_7_integrated_certification_and_score_handoff(
    root: &Path,
) -> Result<(), String> {
    for path in [
        REV_LEVEL_7_CERTIFICATION_HANDOFF_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_integrated_certification_and_score_handoff.schema.md",
        "docs/reading/rev-level-7-integrated-certification-and-score-handoff.md",
        "reviews/2026-07-27-rev-level-7-integrated-certification-and-score-handoff-role-review.md",
        REV_LEVEL_7_POLICY_SPEC_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_scorer_ready_legislative_specification.schema.md",
        "docs/reading/rev-level-7-scorer-ready-legislative-specification.md",
        "reviews/2026-07-27-rev-level-7-scorer-ready-legislative-specification-role-review.md",
        REV_LEVEL_7_SCORE_WORKBOOK_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_official_score_request_workbook.schema.md",
        "docs/reading/rev-level-7-official-score-request-workbook.md",
        "reviews/2026-07-27-rev-level-7-official-score-request-workbook-role-review.md",
        REV_LEVEL_7_DISCUSSION_DRAFT_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_nonofficial_conforming_discussion_draft.schema.md",
        "docs/reading/rev-level-7-nonofficial-conforming-discussion-draft.md",
        "reviews/2026-07-27-rev-level-7-nonofficial-conforming-discussion-draft-role-review.md",
        REV_LEVEL_7_EXTERNAL_SUBMISSION_CONTROL_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_external_submission_control.schema.md",
        REV_LEVEL_7_EXTERNAL_RESPONSE_INTAKE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/rev_level_7_external_response_intake.schema.md",
        "docs/reading/rev-level-7-external-submission-cover-memo.md",
        "docs/reading/rev-level-7-external-submission-authorization-and-routing.md",
        "reviews/2026-07-27-rev-level-7-external-submission-control-role-review.md",
        "reviews/2026-07-27-rev-level-7-authenticated-response-state-machine-role-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-474-rev-level-7-authenticated-response-state-machine.md",
        REV_LEVEL_7_SUBMISSION_BUILDER_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!(
                "missing REV Level-7 certification artifact: {path}"
            ));
        }
    }

    let spec = read_json_artifact(root, REV_LEVEL_7_POLICY_SPEC_JSON_PATH)?;
    let identity = spec
        .get("policy_identity")
        .ok_or("REV Level-7 policy identity")?;
    let effective = spec
        .get("effective_and_duration")
        .ok_or("REV Level-7 effective and duration")?;
    let schedule = spec
        .get("rate_schedule")
        .ok_or("REV Level-7 rate schedule")?;
    let breakpoints = spec
        .get("ty2026_taxable_income_breakpoints_dollars")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 filing-status breakpoints")?;
    let mapping = spec
        .get("taxcalc_mapping")
        .ok_or("REV Level-7 TaxCalc mapping")?;
    let spec_checks = spec
        .get("specification_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 specification checks")?;
    let spec_decision = spec
        .get("decision")
        .ok_or("REV Level-7 specification decision")?;
    let expected_statuses = BTreeSet::from([
        "single".to_string(),
        "married_filing_jointly_or_qualifying_surviving_spouse".to_string(),
        "married_filing_separately".to_string(),
        "head_of_household".to_string(),
    ]);
    let observed_statuses = breakpoints
        .iter()
        .map(|row| string_field(row, "filing_status"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_rates = [21.0, 23.0, 33.0, 35.0, 43.0, 46.0, 48.0];
    let observed_rates = schedule
        .get("specified_percent")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 specified rates")?;
    let expected_breakpoints = [
        ("single", [12400, 50400, 105700, 201775, 256225, 640600]),
        (
            "married_filing_jointly_or_qualifying_surviving_spouse",
            [24800, 100800, 211400, 403550, 512450, 768700],
        ),
        (
            "married_filing_separately",
            [12400, 50400, 105700, 201775, 256225, 384350],
        ),
        (
            "head_of_household",
            [17700, 67450, 105700, 201750, 256200, 640600],
        ),
    ];
    let breakpoint_values_match = expected_breakpoints.iter().all(|(status, expected)| {
        breakpoints
            .iter()
            .find(|row| string_field(row, "filing_status").is_ok_and(|value| value == *status))
            .and_then(|row| row.get("breakpoints"))
            .and_then(serde_json::Value::as_array)
            .is_some_and(|values| {
                values.len() == expected.len()
                    && values
                        .iter()
                        .zip(expected)
                        .all(|(value, expected)| value.as_i64() == Some(*expected))
            })
    });
    if int_field(&spec, "pulse")? != 461
        || string_field(identity, "policy_type")? != "model_scoring_specification"
        || bool_field(identity, "legislative_or_enacted")?
        || bool_field(identity, "official_proposal")?
        || int_field(effective, "first_tax_year")? != 2026
        || string_field(effective, "duration")? != "permanent_for_scoring"
        || string_field(effective, "budget_window")? != "FY2026-FY2035"
        || (number_field(schedule, "uniform_uplift_points")? - 11.0).abs() > 0.0001
        || int_field(schedule, "rate_count")? != 7
        || !bool_field(schedule, "marginal_layering_unchanged")?
        || observed_rates.len() != expected_rates.len()
        || observed_rates
            .iter()
            .zip(expected_rates)
            .any(|(value, expected)| {
                value
                    .as_f64()
                    .is_none_or(|observed| (observed - expected).abs() > 0.0001)
            })
        || breakpoints.len() != 4
        || observed_statuses != expected_statuses
        || !breakpoint_values_match
        || breakpoints.iter().any(|row| {
            row.get("breakpoints")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.len() != 6)
        })
        || bool_field(mapping, "threshold_parameters_changed")?
        || bool_field(mapping, "other_parameters_changed")?
        || mapping
            .get("changed_parameters")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|values| values.len() != 7)
        || spec_checks.len() != 8
        || spec_checks
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|value| value != "pass"))
        || !bool_field(spec_decision, "scorer_ready_policy_specification_complete")?
        || !bool_field(spec_decision, "substantive_legislative_fields_complete")?
        || !bool_field(
            spec_decision,
            "legislative_counsel_conforming_text_required",
        )?
        || !bool_field(spec_decision, "official_score_may_be_requested")?
        || bool_field(spec_decision, "statutory_schedule_enacted")?
    {
        return Err("REV Level-7 scorer-ready policy specification failed".to_string());
    }
    validate_blocked_outputs_null(&spec, "REV Level-7 policy specification")?;

    let draft = read_json_artifact(root, REV_LEVEL_7_DISCUSSION_DRAFT_JSON_PATH)?;
    let draft_identity = draft
        .get("draft_identity")
        .ok_or("REV Level-7 discussion draft identity")?;
    let mechanic = draft
        .get("amendment_mechanic")
        .ok_or("REV Level-7 discussion draft mechanic")?;
    let substitutions = draft
        .get("rate_substitutions_percent")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 discussion draft rate substitutions")?;
    let threshold_rules = draft
        .get("threshold_and_computation_rules")
        .ok_or("REV Level-7 discussion draft threshold rules")?;
    let draft_effective = draft
        .get("effective_date")
        .ok_or("REV Level-7 discussion draft effective date")?;
    let draft_checks = draft
        .get("draft_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 discussion draft checks")?;
    let draft_decision = draft
        .get("decision")
        .ok_or("REV Level-7 discussion draft decision")?;
    let expected_substitutions = [
        (10.0, 21.0),
        (12.0, 23.0),
        (22.0, 33.0),
        (24.0, 35.0),
        (32.0, 43.0),
        (35.0, 46.0),
        (37.0, 48.0),
    ];
    let substitutions_match = substitutions.len() == expected_substitutions.len()
        && substitutions
            .iter()
            .zip(expected_substitutions)
            .all(|(row, (current, specified))| {
                number_field(row, "current").is_ok_and(|value| (value - current).abs() < 0.0001)
                    && number_field(row, "specified")
                        .is_ok_and(|value| (value - specified).abs() < 0.0001)
            });
    if int_field(&draft, "pulse")? != 464
        || string_field(&draft, "status")?
            != "nonofficial_discussion_draft_complete_legislative_counsel_conformance_required"
        || string_field(draft_identity, "document_type")? != "nonofficial_discussion_draft"
        || bool_field(draft_identity, "legislative_counsel_product")?
        || bool_field(draft_identity, "introduced_bill")?
        || bool_field(draft_identity, "official_proposal")?
        || bool_field(draft_identity, "enacted_law")?
        || string_field(mechanic, "code_section")? != "26 U.S.C. 1(j)"
        || string_field(mechanic, "new_paragraph")? != "1(j)(7)"
        || mechanic
            .get("applies_to_table_subparagraphs")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|values| values.len() != 4)
        || mechanic
            .get("does_not_apply_to")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|values| values.len() != 3)
        || !substitutions_match
        || bool_field(
            threshold_rules,
            "minimum_and_maximum_bracket_amounts_changed",
        )?
        || bool_field(threshold_rules, "section_1_j_3_indexing_changed")?
        || bool_field(threshold_rules, "rounding_changed")?
        || !bool_field(
            threshold_rules,
            "base_tax_amounts_recomputed_from_specified_marginal_rates",
        )?
        || bool_field(threshold_rules, "marginal_layering_changed")?
        || string_field(draft_effective, "applies_to_taxable_years_beginning_after")?
            != "2025-12-31"
        || bool_field(draft_effective, "section_15_proration_applies")?
        || !bool_field(draft_effective, "permanent_for_scoring")?
        || draft_checks.len() != 8
        || draft_checks
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|value| value != "pass"))
        || !bool_field(draft_decision, "nonofficial_discussion_draft_complete")?
        || !bool_field(draft_decision, "matches_scorer_ready_policy_specification")?
        || !bool_field(draft_decision, "ready_for_legislative_counsel_review")?
        || bool_field(
            draft_decision,
            "legislative_counsel_conforming_text_complete",
        )?
        || bool_field(draft_decision, "authorized_submission_completed")?
        || bool_field(draft_decision, "statutory_schedule_ready")?
        || bool_field(draft_decision, "official_rate_certified")?
        || bool_field(draft_decision, "balanced_budget_certified")?
    {
        return Err("REV Level-7 nonofficial discussion draft failed".to_string());
    }
    validate_blocked_outputs_null(&draft, "REV Level-7 nonofficial discussion draft")?;

    let workbook = read_json_artifact(root, REV_LEVEL_7_SCORE_WORKBOOK_JSON_PATH)?;
    let request = workbook
        .get("request_identity")
        .ok_or("REV Level-7 score request identity")?;
    let annual = workbook
        .get("annual_score_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 annual score rows")?;
    let workbook_checks = workbook
        .get("workbook_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 workbook checks")?;
    let workbook_decision = workbook
        .get("decision")
        .ok_or("REV Level-7 workbook decision")?;
    let official_fields = [
        "official_conventional_revenue_billions",
        "official_spending_effect_billions",
        "official_administration_cost_billions",
        "official_macro_revenue_feedback_billions",
        "official_primary_deficit_change_billions",
        "official_net_interest_change_billions",
        "official_total_deficit_change_billions",
    ];
    let observed_years = annual
        .iter()
        .map(|row| int_field(row, "fiscal_year"))
        .collect::<Result<Vec<_>, _>>()?;
    if int_field(&workbook, "pulse")? != 462
        || string_field(request, "request_type")? != "technical_scoring_handoff_not_submitted"
        || bool_field(request, "official_response_received")?
        || annual.len() != 10
        || observed_years != (2026_i64..=2035_i64).collect::<Vec<_>>()
        || annual.iter().any(|row| {
            official_fields
                .iter()
                .any(|field| row.get(*field).is_none_or(|value| !value.is_null()))
        })
        || (number_field(&annual[0], "taxlane_model_cash_proxy_billions")? - 819.220).abs() > 0.001
        || annual.iter().skip(1).any(|row| {
            row.get("taxlane_model_cash_proxy_billions")
                .is_none_or(|value| !value.is_null())
        })
        || workbook_checks.len() != 8
        || workbook_checks
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|value| value != "pass"))
        || !bool_field(workbook_decision, "score_request_workbook_complete")?
        || !bool_field(workbook_decision, "ready_for_authorized_submission")?
        || bool_field(workbook_decision, "external_request_submitted")?
        || bool_field(workbook_decision, "official_response_received")?
        || bool_field(workbook_decision, "formal_certification_complete")?
    {
        return Err("REV Level-7 score request workbook failed".to_string());
    }
    validate_blocked_outputs_null(&workbook, "REV Level-7 score request workbook")?;

    let record = read_json_artifact(root, REV_LEVEL_7_CERTIFICATION_HANDOFF_JSON_PATH)?;
    let internal = record
        .get("internal_certification_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 internal checks")?;
    let official = record
        .get("official_certification_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 official gates")?;
    let handoff = record
        .get("score_handoff_package")
        .ok_or("REV Level-7 score handoff")?;
    let decision = record
        .get("certification_decision")
        .ok_or("REV Level-7 certification decision")?;
    if int_field(&record, "pulse")? != 465
        || string_field(&record, "work_package_id")? != "REV-Level-7"
        || string_field(&record, "policy_specification_path")? != REV_LEVEL_7_POLICY_SPEC_JSON_PATH
        || string_field(&record, "score_request_workbook_path")?
            != REV_LEVEL_7_SCORE_WORKBOOK_JSON_PATH
        || string_field(&record, "nonofficial_discussion_draft_path")?
            != REV_LEVEL_7_DISCUSSION_DRAFT_JSON_PATH
        || internal.len() != 8
        || internal
            .iter()
            .any(|row| string_field(row, "status").is_ok_and(|value| value != "pass"))
        || official.len() != 9
        || official
            .iter()
            .filter(|row| {
                string_field(row, "disposition")
                    .is_ok_and(|value| value == "required_internal_ready")
            })
            .count()
            != 1
        || official.iter().any(|row| {
            string_field(row, "disposition").is_ok_and(|value| {
                ![
                    "required_internal_ready",
                    "required_blocked",
                    "required_external",
                ]
                .contains(&value.as_str())
            })
        })
        || (number_field(handoff, "model_fy2026_cash_target_billions")? - 813.727).abs() > 0.001
        || (number_field(handoff, "model_first_year_cash_proxy_billions")? - 819.220).abs() > 0.001
        || number_field(
            handoff,
            "admitted_fy2026_primary_spending_reduction_billions",
        )?
        .abs()
            > 0.0001
        || handoff
            .get("public_spending_instruments")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|rows| !rows.is_empty())
        || !bool_field(handoff, "policy_specification_complete")?
        || !bool_field(handoff, "ten_year_score_request_workbook_complete")?
        || !bool_field(handoff, "nonofficial_discussion_draft_complete")?
        || bool_field(handoff, "legislative_counsel_conforming_text_complete")?
        || !bool_field(handoff, "handoff_ready_for_external_scorer")?
        || bool_field(handoff, "authorized_submission_completed")?
        || bool_field(handoff, "official_score_received")?
        || !bool_field(decision, "internal_evidence_certification_complete")?
        || !bool_field(decision, "all_internal_checks_pass")?
        || !bool_field(decision, "substantive_policy_specification_complete")?
        || !bool_field(decision, "score_request_workbook_complete")?
        || !bool_field(decision, "nonofficial_discussion_draft_complete")?
        || bool_field(decision, "legislative_counsel_conforming_text_complete")?
        || !bool_field(decision, "official_score_handoff_ready")?
        || !bool_field(decision, "planning_rate_card_retained")?
        || bool_field(decision, "lower_rate_supported_by_admitted_spending")?
        || bool_field(decision, "formal_fiscal_certification_complete")?
        || bool_field(decision, "official_rate_certified")?
        || bool_field(decision, "statutory_schedule_ready")?
        || bool_field(decision, "balanced_budget_certified")?
        || !bool_field(decision, "rev_level_7_internal_work_complete")?
        || !bool_field(decision, "authorized_submission_pending")?
        || !bool_field(
            decision,
            "remaining_work_requires_external_official_score_or_new_admissible_evidence",
        )?
    {
        return Err("REV Level-7 certification and score handoff failed".to_string());
    }
    validate_blocked_outputs_null(&record, "REV Level-7 certification handoff")?;

    let control = read_json_artifact(root, REV_LEVEL_7_EXTERNAL_SUBMISSION_CONTROL_JSON_PATH)?;
    let payload = control
        .get("payload")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 external submission payload")?;
    let routing = control
        .get("routing")
        .ok_or("REV Level-7 external submission routing")?;
    let authorization = control
        .get("authorization")
        .ok_or("REV Level-7 external submission authorization")?;
    let preflight = control
        .get("preflight")
        .ok_or("REV Level-7 external submission preflight")?;
    let expected_bundle_id = "cc2718b3cd4a4723eba9846b29cc8335b4e94bab70c8783871ee8caa81a61be5";
    let required_true_preflight = [
        "payload_hashes_verified",
        "deterministic_bundle_built",
        "bundle_sha256_generated",
        "nonofficial_draft_label_present",
        "planning_numbers_not_official",
        "no_spending_savings_admitted",
        "no_secrets_or_personal_data_in_payload",
        "receipt_intake_ready",
        "response_intake_ready",
    ];
    let authorization_null_fields = [
        "requesting_office",
        "requesting_principal",
        "staff_contact_name",
        "staff_contact_official_email",
        "authority_basis",
        "approval_date",
        "authorized_signer",
        "approved_cover_message",
        "approved_bundle_id",
        "approved_bundle_sha256",
    ];
    if int_field(&control, "pulse")? != 473
        || string_field(&control, "status")?
            != "sealed_local_bundle_built_authorization_and_channel_pending"
        || string_field(&control, "submission_id")? != "taxlane-rev-level-7-fy2026-rate-only-v1"
        || string_field(&control, "bundle_id")? != expected_bundle_id
        || string_field(&control, "schema_path")?
            != "data/derived/breadth_benchmark_matrix/rev_level_7_external_submission_control.schema.md"
        || string_field(&control, "reader_path")?
            != "docs/reading/rev-level-7-external-submission-authorization-and-routing.md"
        || string_field(&control, "cover_memo_path")?
            != "docs/reading/rev-level-7-external-submission-cover-memo.md"
        || string_field(&control, "role_review_path")?
            != "reviews/2026-07-27-rev-level-7-external-submission-control-role-review.md"
        || string_field(&control, "builder_path")? != REV_LEVEL_7_SUBMISSION_BUILDER_PATH
        || payload.len() != 7
        || string_field(routing, "official_revenue_estimator")? != "Joint Committee on Taxation"
        || !bool_field(routing, "authorized_requester_required")?
        || bool_field(routing, "public_direct_official_score_route_available")?
        || [
            "selected_requesting_office",
            "selected_staff_contact",
            "selected_jct_channel",
            "selected_cbo_channel",
        ]
        .iter()
        .any(|field| routing.get(*field).is_none_or(|value| !value.is_null()))
        || authorization_null_fields.iter().any(|field| {
            authorization
                .get(*field)
                .is_none_or(|value| !value.is_null())
        })
        || bool_field(authorization, "exact_outbound_action_authorized")?
        || required_true_preflight.iter().any(|field| {
            bool_field(preflight, field).is_err() || !bool_field(preflight, field).unwrap_or(false)
        })
        || bool_field(preflight, "send_allowed")?
    {
        return Err("REV Level-7 external submission control failed".to_string());
    }

    let mut observed_bundle_paths = BTreeSet::new();
    let mut bundle_identity_lines = String::new();
    for item in payload {
        let path = string_field(item, "path")?;
        let bundle_path = string_field(item, "bundle_path")?;
        let expected_sha = string_field(item, "sha256")?;
        if path.contains("..")
            || bundle_path.contains("..")
            || !observed_bundle_paths.insert(bundle_path.clone())
            || !root.join(&path).is_file()
            || sha256_file(&root.join(&path))? != expected_sha
        {
            return Err(format!(
                "REV Level-7 sealed payload validation failed: {path}"
            ));
        }
        bundle_identity_lines.push_str(&bundle_path);
        bundle_identity_lines.push(':');
        bundle_identity_lines.push_str(&expected_sha);
        bundle_identity_lines.push('\n');
    }
    let mut bundle_hasher = Sha256::new();
    bundle_hasher.update(bundle_identity_lines.as_bytes());
    if format!("{:x}", bundle_hasher.finalize()) != expected_bundle_id {
        return Err("REV Level-7 deterministic bundle identity failed".to_string());
    }
    let builder = fs::read_to_string(root.join(REV_LEVEL_7_SUBMISSION_BUILDER_PATH))
        .map_err(|error| format!("failed to read REV Level-7 bundle builder: {error}"))?;
    for phrase in [
        "exact_outbound_action_authorized",
        "send_allowed",
        "outbound_action_performed = $false",
        "hash mismatch",
        "beneath the repository dist",
    ] {
        if !builder.contains(phrase) {
            return Err(format!(
                "REV Level-7 bundle builder missing guard: {phrase}"
            ));
        }
    }
    validate_blocked_outputs_null(&control, "REV Level-7 external submission control")?;

    let intake = read_json_artifact(root, REV_LEVEL_7_EXTERNAL_RESPONSE_INTAKE_JSON_PATH)?;
    validate_rev_level_7_external_response_intake_record(root, &intake, expected_bundle_id)?;
    Ok(())
}

pub(crate) fn validate_rev_level_7_custodied_file(
    root: &Path,
    relative_path: &str,
    expected_sha256: &str,
    label: &str,
) -> Result<(), String> {
    if relative_path.contains("..")
        || Path::new(relative_path).is_absolute()
        || expected_sha256.len() != 64
        || !expected_sha256
            .chars()
            .all(|value| value.is_ascii_hexdigit())
    {
        return Err(format!("REV Level-7 invalid {label} custody fields"));
    }
    let path = root.join(relative_path);
    if !path.is_file() || sha256_file(&path)? != expected_sha256.to_ascii_lowercase() {
        return Err(format!("REV Level-7 {label} custody mismatch"));
    }
    Ok(())
}

pub(crate) fn validate_rev_level_7_external_response_intake_record(
    root: &Path,
    intake: &serde_json::Value,
    expected_bundle_id: &str,
) -> Result<(), String> {
    let receipt = intake
        .get("submission_receipt")
        .and_then(serde_json::Value::as_object)
        .ok_or("REV Level-7 submission receipt intake")?;
    let response_identity = intake
        .get("response_identity")
        .and_then(serde_json::Value::as_object)
        .ok_or("REV Level-7 response identity intake")?;
    let response_assets = intake
        .get("response_assets")
        .and_then(serde_json::Value::as_object)
        .ok_or("REV Level-7 response asset intake")?;
    let intake_review = intake
        .get("review")
        .and_then(serde_json::Value::as_object)
        .ok_or("REV Level-7 response review intake")?;
    let transition = intake
        .get("transition_contract")
        .ok_or("REV Level-7 response transition contract")?;
    let accepted_offices = transition
        .get("accepted_responding_offices")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 accepted responding offices")?;
    let annual_fields = transition
        .get("required_annual_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 required annual fields")?;
    let annual_rows = intake
        .get("annual_score_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 annual response rows")?;
    let status = string_field(intake, "status")?;
    if int_field(intake, "pulse")? != 474
        || string_field(intake, "submission_control_path")?
            != REV_LEVEL_7_EXTERNAL_SUBMISSION_CONTROL_JSON_PATH
        || transition
            .get("allowed_statuses")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|values| {
                values.len() != 3 || !values.iter().any(|value| value.as_str() == Some(&status))
            })
        || accepted_offices.len() != 4
        || annual_fields.len() != 7
        || int_field(transition, "annual_fiscal_year_start")? != 2026
        || int_field(transition, "annual_fiscal_year_end")? != 2035
        || !bool_field(
            transition,
            "asset_objects_require_path_sha256_office_and_identifier",
        )?
        || !bool_field(
            transition,
            "conventional_revenue_required_for_rate_recertification",
        )?
        || !bool_field(transition, "independent_role_review_required")?
    {
        return Err("REV Level-7 response transition contract failed".to_string());
    }

    if status == "intake_ready_no_submission_or_response" {
        if receipt.values().any(|value| !value.is_null())
            || response_identity.iter().any(|(field, value)| {
                field == "official_status_verified" && value.as_bool() != Some(false)
                    || field != "official_status_verified" && !value.is_null()
            })
            || response_assets.values().any(|value| !value.is_null())
            || !annual_rows.is_empty()
            || intake_review
                .values()
                .any(|value| value.as_bool() != Some(false))
        {
            return Err("REV Level-7 empty response intake failed".to_string());
        }
        validate_blocked_outputs_null(intake, "REV Level-7 external response intake")?;
        return Ok(());
    }

    for field in transition
        .get("required_receipt_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 required receipt fields")?
    {
        let field = field.as_str().ok_or("REV Level-7 receipt field name")?;
        if string_field(&serde_json::Value::Object(receipt.clone()), field)?.is_empty() {
            return Err(format!("REV Level-7 missing receipt field: {field}"));
        }
    }
    let receipt_bundle_sha =
        string_field(&serde_json::Value::Object(receipt.clone()), "bundle_sha256")?;
    if string_field(&serde_json::Value::Object(receipt.clone()), "bundle_id")? != expected_bundle_id
        || receipt_bundle_sha.len() != 64
        || !receipt_bundle_sha
            .chars()
            .all(|value| value.is_ascii_hexdigit())
    {
        return Err("REV Level-7 receipt bundle identity or digest mismatch".to_string());
    }
    validate_rev_level_7_custodied_file(
        root,
        &string_field(
            &serde_json::Value::Object(receipt.clone()),
            "receipt_evidence_path",
        )?,
        &string_field(
            &serde_json::Value::Object(receipt.clone()),
            "receipt_evidence_sha256",
        )?,
        "submission receipt",
    )?;

    for field in transition
        .get("required_response_identity_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("REV Level-7 required response identity fields")?
    {
        let field = field
            .as_str()
            .ok_or("REV Level-7 response identity field name")?;
        if string_field(&serde_json::Value::Object(response_identity.clone()), field)?.is_empty() {
            return Err(format!(
                "REV Level-7 missing response identity field: {field}"
            ));
        }
    }
    let responding_office = string_field(
        &serde_json::Value::Object(response_identity.clone()),
        "responding_office",
    )?;
    if !accepted_offices
        .iter()
        .any(|value| value.as_str() == Some(&responding_office))
        || !bool_field(
            &serde_json::Value::Object(response_identity.clone()),
            "official_status_verified",
        )?
    {
        return Err("REV Level-7 responding office authentication failed".to_string());
    }
    validate_rev_level_7_custodied_file(
        root,
        &string_field(
            &serde_json::Value::Object(response_identity.clone()),
            "verification_evidence_path",
        )?,
        &string_field(
            &serde_json::Value::Object(response_identity.clone()),
            "verification_evidence_sha256",
        )?,
        "response verification",
    )?;

    let mut supplied_assets = BTreeSet::new();
    for (slot, value) in response_assets {
        if value.is_null() {
            continue;
        }
        let asset = value
            .as_object()
            .ok_or_else(|| format!("REV Level-7 response asset must be object: {slot}"))?;
        let asset_value = serde_json::Value::Object(asset.clone());
        let path = string_field(&asset_value, "path")?;
        let sha256 = string_field(&asset_value, "sha256")?;
        let office = string_field(&asset_value, "responding_office")?;
        if string_field(&asset_value, "document_identifier")?.is_empty()
            || !accepted_offices
                .iter()
                .any(|value| value.as_str() == Some(&office))
            || !supplied_assets.insert(path.clone())
        {
            return Err(format!(
                "REV Level-7 response asset identity failed: {slot}"
            ));
        }
        validate_rev_level_7_custodied_file(root, &path, &sha256, slot)?;
    }
    if supplied_assets.is_empty() {
        return Err("REV Level-7 received response has no custodied asset".to_string());
    }

    if !annual_rows.is_empty() {
        if annual_rows.len() != 10 {
            return Err("REV Level-7 response must contain zero or ten annual rows".to_string());
        }
        for (offset, row) in annual_rows.iter().enumerate() {
            if int_field(row, "fiscal_year")? != 2026 + offset as i64 {
                return Err("REV Level-7 response annual window mismatch".to_string());
            }
            let dispositions = row
                .get("scope_dispositions")
                .and_then(serde_json::Value::as_object)
                .ok_or("REV Level-7 annual scope dispositions")?;
            for field in annual_fields {
                let field = field.as_str().ok_or("REV Level-7 annual field name")?;
                let value = row
                    .get(field)
                    .ok_or_else(|| format!("REV Level-7 annual response missing field: {field}"))?;
                let disposition = dispositions
                    .get(field)
                    .and_then(serde_json::Value::as_str)
                    .ok_or("REV Level-7 annual scope disposition")?;
                if !["reported", "not_provided", "not_applicable"].contains(&disposition)
                    || value.is_number() && disposition != "reported"
                    || value.is_null() && disposition == "reported"
                    || !value.is_number() && !value.is_null()
                {
                    return Err(format!("REV Level-7 annual scope mismatch: {field}"));
                }
            }
        }
    }

    let eligible = bool_field(
        &serde_json::Value::Object(intake_review.clone()),
        "eligible_for_rate_recertification",
    )?;
    if status == "official_response_received_pending_review" {
        if eligible {
            return Err("REV Level-7 pending review cannot recertify rate".to_string());
        }
    } else if status == "authenticated_response_ready_for_rate_recertification" {
        let required_assets = transition
            .get("required_ready_assets")
            .and_then(serde_json::Value::as_array)
            .ok_or("REV Level-7 required ready assets")?;
        if annual_rows.len() != 10
            || required_assets.iter().any(|slot| {
                slot.as_str()
                    .and_then(|slot| response_assets.get(slot))
                    .is_none_or(serde_json::Value::is_null)
            })
            || annual_rows.iter().any(|row| {
                row.get("official_conventional_revenue_billions")
                    .is_none_or(|value| !value.is_number())
            })
            || intake_review
                .values()
                .any(|value| value.as_bool() != Some(true))
        {
            return Err("REV Level-7 response is not ready for rate recertification".to_string());
        }
    } else {
        return Err(format!("REV Level-7 unsupported response status: {status}"));
    }
    validate_blocked_outputs_null(intake, "REV Level-7 external response intake")?;
    Ok(())
}

