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

pub(crate) fn validate_omb_receipt_category_fy2025_2031_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing OMB receipt category artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH}: {err}")
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "omb-receipt-category-fy2025-2031-context:v1"
        || string_field(&record, "record_family")? != "omb_receipt_category_multi_year_context"
        || string_field(&record, "status")?
            != "draft_official_omb_receipt_category_context_not_assigned_base"
    {
        return Err("OMB receipt category FY2025-FY2031 identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("OMB receipt category source custody")?;
    if string_field(custody, "source_id")? != "SRC-OMB-HIST-2-1-FY2027"
        || string_field(custody, "publisher")? != "Office of Management and Budget"
        || string_field(custody, "source_table")?
            != "Historical Table 2.1 - Receipts by Source: 1934-2031"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx"
        || int_field(custody, "byte_count")? != 12_393
        || string_field(custody, "sha256")?
            != "1212da86947a71d9a0268e23d3de789402bcfb80b308cd48a32926a65b2e7bc3"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("OMB receipt category source custody failed".to_string());
    }
    let raw_path = string_field(custody, "raw_artifact_path")?;
    let raw_file = root.join(&raw_path);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 12_393
        || sha256_file(&raw_file)?
            != "1212da86947a71d9a0268e23d3de789402bcfb80b308cd48a32926a65b2e7bc3"
    {
        return Err("OMB receipt category raw custody file failed".to_string());
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("OMB receipt category extraction scope")?;
    if string_field(scope, "worksheet")? != "Table"
        || string_field(scope, "source_unit")? != "millions_of_dollars"
    {
        return Err("OMB receipt category extraction scope fields failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "not_legal_or_economic_receipt_base",
        "not_assigned_receipt_base",
        "not_incidence_or_distribution_model",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB receipt category scope {field} failed"));
        }
    }

    let rows = record
        .get("annual_receipt_category_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB receipt category annual rows")?;
    if rows.len() != 7 {
        return Err("OMB receipt category annual row count failed".to_string());
    }
    let expected = [
        (
            2025, "actual", 2_656_044, 452_089, 1_748_294, 105_937, 274_057, 5_236_421,
        ),
        (
            2026, "estimate", 2_629_899, 398_640, 1_849_815, 105_588, 491_763, 5_475_705,
        ),
        (
            2027, "estimate", 2_869_270, 440_066, 1_934_509, 107_440, 569_666, 5_920_951,
        ),
        (
            2028, "estimate", 3_074_004, 455_515, 2_053_856, 107_576, 597_456, 6_288_407,
        ),
        (
            2029, "estimate", 3_280_032, 481_816, 2_158_527, 110_233, 629_713, 6_660_321,
        ),
        (
            2030, "estimate", 3_564_198, 516_336, 2_276_402, 113_113, 667_232, 7_137_281,
        ),
        (
            2031, "estimate", 3_802_690, 540_758, 2_395_136, 113_550, 707_255, 7_559_389,
        ),
    ]
    .into_iter()
    .map(
        |(year, status, individual, corporate, social, excise, other, total)| {
            (
                year,
                (
                    status.to_string(),
                    individual,
                    corporate,
                    social,
                    excise,
                    other,
                    total,
                ),
            )
        },
    )
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (status, individual, corporate, social, excise, other, total) = expected
            .get(&year)
            .ok_or("unexpected OMB receipt category year")?;
        if string_field(row, "year_status")? != *status
            || int_field(row, "individual_income_taxes")? != *individual
            || int_field(row, "corporation_income_taxes")? != *corporate
            || int_field(row, "social_insurance_retirement_total")? != *social
            || int_field(row, "excise_taxes")? != *excise
            || int_field(row, "other_receipts")? != *other
            || int_field(row, "total_receipts")? != *total
        {
            return Err(format!("OMB receipt category values failed for FY{year}"));
        }
        let component_sum = int_field(row, "individual_income_taxes")?
            + int_field(row, "corporation_income_taxes")?
            + int_field(row, "social_insurance_retirement_total")?
            + int_field(row, "excise_taxes")?
            + int_field(row, "other_receipts")?;
        if component_sum != int_field(row, "total_receipts")? {
            return Err(format!(
                "OMB receipt category component sum failed for FY{year}"
            ));
        }
        if int_field(row, "total_on_budget")? + int_field(row, "total_off_budget")?
            != int_field(row, "total_receipts")?
        {
            return Err(format!(
                "OMB receipt category budget split failed for FY{year}"
            ));
        }
    }
    if observed_years != (2025..=2031).collect::<BTreeSet<_>>() {
        return Err("OMB receipt category year coverage failed".to_string());
    }

    let boundaries = record
        .get("category_boundaries")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB receipt category boundaries")?;
    if boundaries.len() != 5 {
        return Err("OMB receipt category boundary count failed".to_string());
    }
    for row in boundaries {
        if string_field(row, "category_id")?.is_empty()
            || string_field(row, "blocked_boundary")?.is_empty()
        {
            return Err("OMB receipt category boundary fields failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "incidence_distribution_model",
        "administration_burden",
        "current_law_solver_yields",
        "reform_yields",
        "solver_input_rows",
        "solver_run",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "OMB receipt category blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category claims")?;
    for field in [
        "omb_receipt_category_context_published",
        "source_custody_ready",
        "fy2025_fy2031_receipt_category_context_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB receipt category claim {field} must be true"));
        }
    }
    for field in [
        "legal_receipt_base_ready",
        "economic_receipt_base_ready",
        "matched_receipt_bases_ready",
        "incidence_distribution_model_ready",
        "administration_burden_ready",
        "current_law_solver_yields_ready",
        "reform_yields_ready",
        "solver_input_ready",
        "solver_run_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("OMB receipt category claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_READER_PATH}: {err}"
                )
            })?;
    for phrase in [
        OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH,
        "FY2025-FY2031",
        "FY2025 is actual",
        "FY2026-FY2031 are OMB estimates",
        "not legal/economic receipt bases",
        "not rate bridges",
        "not solver inputs",
        "not balanced-budget claims",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB receipt category reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_receipt_detail_table_2_4_fy2025_2031_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing OMB Table 2.4 detail artifact: {path}"));
        }
    }

    let text =
        fs::read_to_string(root.join(OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH))
            .map_err(|err| {
            format!(
                "failed to read {OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH}: {err}"
            )
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "omb-receipt-detail-table-2-4-fy2025-2031-context:v1"
        || string_field(&record, "record_family")?
            != "omb_receipt_detail_table_2_4_multi_year_context"
        || string_field(&record, "status")?
            != "draft_official_omb_receipt_detail_context_not_assigned_base"
    {
        return Err("OMB Table 2.4 detail identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("OMB Table 2.4 detail source custody")?;
    if string_field(custody, "source_id")? != "SRC-OMB-HIST-2-4-FY2027"
        || string_field(custody, "publisher")? != "Office of Management and Budget"
        || string_field(custody, "source_table")?
            != "Historical Table 2.4 - Composition of Social Insurance and Retirement Receipts and of Excise Taxes: 1940-2031"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-HIST-2-4-FY2027/2026-06-21/hist02z4_fy2027.xlsx"
        || int_field(custody, "byte_count")? != 26_752
        || string_field(custody, "sha256")?
            != "21d071576d5627a18c3f62de86bfc7faeced1a68265f2db87b4f737b2773c5bd"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("OMB Table 2.4 detail source custody failed".to_string());
    }
    let raw_path = string_field(custody, "raw_artifact_path")?;
    let raw_file = root.join(&raw_path);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 26_752
        || sha256_file(&raw_file)?
            != "21d071576d5627a18c3f62de86bfc7faeced1a68265f2db87b4f737b2773c5bd"
    {
        return Err("OMB Table 2.4 detail raw custody file failed".to_string());
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("OMB Table 2.4 detail extraction scope")?;
    if string_field(scope, "worksheet")? != "Table"
        || string_field(scope, "source_unit")? != "millions_of_dollars"
        || string_field(scope, "source_column_range")?
            != "FY2025 column 88 through FY2031 Estimate column 94"
    {
        return Err("OMB Table 2.4 detail extraction scope failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "source_display_omissions_preserved_as_null",
        "not_legal_or_economic_receipt_base",
        "not_assigned_receipt_base",
        "not_incidence_or_distribution_model",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB Table 2.4 detail scope {field} failed"));
        }
    }

    let rows = record
        .get("annual_detail_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB Table 2.4 detail annual rows")?;
    if rows.len() != 7 {
        return Err("OMB Table 2.4 detail annual row count failed".to_string());
    }
    let expected = [
        (
            2025, "actual", 1_097_382, 186_354, 395_350, 1_748_294, 43_768, 23_118, 105_937,
        ),
        (
            2026, "estimate", 1_169_124, 198_531, 412_571, 1_849_815, 46_970, 21_334, 105_588,
        ),
        (
            2027, "estimate", 1_217_535, 206_752, 437_596, 1_934_509, 47_112, 22_636, 107_440,
        ),
        (
            2028, "estimate", 1_292_143, 219_420, 466_447, 2_053_856, 46_848, 23_952, 107_576,
        ),
        (
            2029, "estimate", 1_357_440, 230_510, 492_277, 2_158_527, 46_264, 25_340, 110_233,
        ),
        (
            2030, "estimate", 1_431_016, 243_003, 520_973, 2_276_402, 46_037, 26_804, 113_113,
        ),
        (
            2031, "estimate", 1_505_256, 255_610, 550_209, 2_395_136, 45_742, 28_309, 113_550,
        ),
    ]
    .into_iter()
    .map(
        |(year, status, oas, di, hi, social, transport, airport, excise)| {
            (
                year,
                (
                    status.to_string(),
                    oas,
                    di,
                    hi,
                    social,
                    transport,
                    airport,
                    excise,
                ),
            )
        },
    )
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (status, oas, di, hi, social, transport, airport, excise) = expected
            .get(&year)
            .ok_or("unexpected OMB Table 2.4 detail year")?;
        if string_field(row, "year_status")? != *status
            || int_field(row, "oas_trust_funds_off_budget")? != *oas
            || int_field(row, "di_off_budget")? != *di
            || int_field(row, "hospital_insurance")? != *hi
            || int_field(row, "social_insurance_retirement_total")? != *social
            || int_field(row, "transportation_excise_trust_funds")? != *transport
            || int_field(row, "airport_airway_excise_trust_funds")? != *airport
            || int_field(row, "excise_taxes_total")? != *excise
        {
            return Err(format!("OMB Table 2.4 detail values failed for FY{year}"));
        }
        let employment_sum = int_field(row, "oas_trust_funds_off_budget")?
            + int_field(row, "di_off_budget")?
            + int_field(row, "hospital_insurance")?
            + int_field(row, "railroad_retirement_trust_funds")?
            + int_field(row, "railroad_social_security_equivalent_account")?;
        if employment_sum != int_field(row, "employment_general_retirement_total")? {
            return Err(format!(
                "OMB Table 2.4 employment subtotal failed for FY{year}"
            ));
        }
        let social_sum = int_field(row, "employment_general_retirement_total")?
            + int_field(row, "unemployment_insurance_total")?
            + int_field(row, "other_retirement_total")?;
        if social_sum != int_field(row, "social_insurance_retirement_total")? {
            return Err(format!(
                "OMB Table 2.4 social-insurance subtotal failed for FY{year}"
            ));
        }
        if int_field(row, "excise_federal_funds_total")?
            + int_field(row, "excise_trust_funds_total")?
            != int_field(row, "excise_taxes_total")?
        {
            return Err(format!("OMB Table 2.4 excise subtotal failed for FY{year}"));
        }
    }
    if observed_years != (2025..=2031).collect::<BTreeSet<_>>() {
        return Err("OMB Table 2.4 detail year coverage failed".to_string());
    }

    let omissions = record
        .get("source_display_omissions")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB Table 2.4 detail source display omissions")?;
    if omissions.len() != 2 {
        return Err("OMB Table 2.4 detail omission count failed".to_string());
    }
    let row_2027 = rows
        .iter()
        .find(|row| int_field(row, "fiscal_year").ok() == Some(2027))
        .ok_or("missing FY2027 OMB Table 2.4 row")?;
    if !row_2027
        .get("oil_spill_liability_excise_trust_funds")
        .is_some_and(serde_json::Value::is_null)
        || !row_2027
            .get("tobacco_assessments_excise_trust_funds")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("OMB Table 2.4 detail null source-display omissions failed".to_string());
    }

    let boundaries = record
        .get("lane_context_boundaries")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB Table 2.4 detail lane context boundaries")?;
    if boundaries.len() != 4 {
        return Err("OMB Table 2.4 detail lane boundary count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB Table 2.4 detail blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "incidence_distribution_model",
        "administration_burden",
        "current_law_solver_yields",
        "reform_yields",
        "solver_input_rows",
        "solver_run",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "OMB Table 2.4 detail blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB Table 2.4 detail claims")?;
    for field in [
        "omb_receipt_detail_context_published",
        "source_custody_ready",
        "fy2025_fy2031_social_insurance_excise_detail_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB Table 2.4 detail claim {field} must be true"));
        }
    }
    for field in [
        "legal_receipt_base_ready",
        "economic_receipt_base_ready",
        "matched_receipt_bases_ready",
        "incidence_distribution_model_ready",
        "administration_burden_ready",
        "current_law_solver_yields_ready",
        "reform_yields_ready",
        "solver_input_ready",
        "solver_run_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("OMB Table 2.4 detail claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH,
        "FY2025-FY2031",
        "FY2025 is actual",
        "FY2026-FY2031 are OMB estimates",
        "source-display omissions as null",
        "not taxable payroll bases",
        "not an HI payroll base",
        "not statutory user-fee bases",
        "not rate bridges",
        "not solver inputs",
        "not balanced-budget claims",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB Table 2.4 detail reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_receipt_share_table_2_2_fy2025_2031_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing OMB Table 2.2 share artifact: {path}"));
        }
    }

    let text = fs::read_to_string(
        root.join(OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!("failed to read {OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH}: {err}")
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "omb-receipt-share-table-2-2-fy2025-2031-context:v1"
        || string_field(&record, "record_family")?
            != "omb_receipt_share_table_2_2_multi_year_context"
        || string_field(&record, "status")?
            != "draft_official_omb_receipt_share_context_not_assigned_base"
    {
        return Err("OMB Table 2.2 share identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("OMB Table 2.2 share source custody")?;
    if string_field(custody, "source_id")? != "SRC-OMB-HIST-2-2-FY2027"
        || string_field(custody, "publisher")? != "Office of Management and Budget"
        || string_field(custody, "source_table")?
            != "Historical Table 2.2 - Percentage Composition of Receipts by Source: 1934-2031"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx"
        || int_field(custody, "byte_count")? != 10_253
        || string_field(custody, "sha256")?
            != "9e2bc741ca3a92929551ca65c6db55901a1d96bac0cae43788c89cfac4f0fe55"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("OMB Table 2.2 share source custody failed".to_string());
    }
    let raw_path = string_field(custody, "raw_artifact_path")?;
    let raw_file = root.join(&raw_path);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 10_253
        || sha256_file(&raw_file)?
            != "9e2bc741ca3a92929551ca65c6db55901a1d96bac0cae43788c89cfac4f0fe55"
    {
        return Err("OMB Table 2.2 share raw custody file failed".to_string());
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("OMB Table 2.2 share extraction scope")?;
    if string_field(scope, "worksheet")? != "Table"
        || string_field(scope, "source_unit")? != "percent_of_total_receipts"
        || number_field(scope, "rounding_tolerance_percent_points")? != 0.25
    {
        return Err("OMB Table 2.2 share extraction scope failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "not_receipt_amounts",
        "not_legal_or_economic_receipt_base",
        "not_assigned_receipt_base",
        "not_incidence_or_distribution_model",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB Table 2.2 share scope {field} failed"));
        }
    }

    let rows = record
        .get("annual_receipt_share_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB Table 2.2 share annual rows")?;
    if rows.len() != 7 {
        return Err("OMB Table 2.2 share annual row count failed".to_string());
    }
    let expected = [
        (2025, "actual", 50.7, 8.6, 33.4, 2.0, 5.2, 99.9, 96),
        (2026, "estimate", 48.0, 7.3, 33.8, 1.9, 9.0, 100.0, 97),
        (2027, "estimate", 48.5, 7.4, 32.7, 1.8, 9.6, 100.0, 98),
        (2028, "estimate", 48.9, 7.2, 32.7, 1.7, 9.5, 100.0, 99),
        (2029, "estimate", 49.2, 7.2, 32.4, 1.7, 9.5, 100.0, 100),
        (2030, "estimate", 49.9, 7.2, 31.9, 1.6, 9.3, 99.9, 101),
        (2031, "estimate", 50.3, 7.2, 31.7, 1.5, 9.4, 100.1, 102),
    ]
    .into_iter()
    .map(
        |(year, status, individual, corporate, social, excise, other, sum, source_row)| {
            (
                year,
                (
                    status.to_string(),
                    individual,
                    corporate,
                    social,
                    excise,
                    other,
                    sum,
                    source_row,
                ),
            )
        },
    )
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (status, individual, corporate, social, excise, other, sum, source_row) = expected
            .get(&year)
            .ok_or("unexpected OMB Table 2.2 share year")?;
        if string_field(row, "year_status")? != *status
            || (number_field(row, "individual_income_taxes_share_pct")? - *individual).abs()
                > 0.000_001
            || (number_field(row, "corporation_income_taxes_share_pct")? - *corporate).abs()
                > 0.000_001
            || (number_field(row, "social_insurance_retirement_receipts_share_pct")? - *social)
                .abs()
                > 0.000_001
            || (number_field(row, "excise_taxes_share_pct")? - *excise).abs() > 0.000_001
            || (number_field(row, "other_receipts_share_pct")? - *other).abs() > 0.000_001
            || (number_field(row, "component_share_sum_pct")? - *sum).abs() > 0.000_001
            || (number_field(row, "total_receipts_share_pct")? - 100.0).abs() > 0.000_001
            || int_field(row, "source_row")? != *source_row
        {
            return Err(format!("OMB Table 2.2 share values failed for FY{year}"));
        }
        let component_sum = number_field(row, "individual_income_taxes_share_pct")?
            + number_field(row, "corporation_income_taxes_share_pct")?
            + number_field(row, "social_insurance_retirement_receipts_share_pct")?
            + number_field(row, "excise_taxes_share_pct")?
            + number_field(row, "other_receipts_share_pct")?;
        if (component_sum - number_field(row, "component_share_sum_pct")?).abs() > 0.000_001 {
            return Err(format!(
                "OMB Table 2.2 share component sum failed for FY{year}"
            ));
        }
        if (component_sum - 100.0).abs() > 0.25 {
            return Err(format!(
                "OMB Table 2.2 share rounding tolerance failed for FY{year}"
            ));
        }
    }
    if observed_years != (2025..=2031).collect::<BTreeSet<_>>() {
        return Err("OMB Table 2.2 share year coverage failed".to_string());
    }

    let boundaries = record
        .get("category_boundaries")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB Table 2.2 share category boundaries")?;
    if boundaries.len() != 5 {
        return Err("OMB Table 2.2 share boundary count failed".to_string());
    }

    let checks = record
        .get("reconciliation_checks")
        .ok_or("OMB Table 2.2 share reconciliation checks")?;
    if int_field(checks, "rows_present")? != 7
        || int_field(checks, "receipt_categories_per_year")? != 6
        || number_field(checks, "total_receipts_share_pct_all_years")? != 100.0
        || checks
            .get("component_sum_within_rounding_tolerance")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("OMB Table 2.2 share reconciliation checks failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB Table 2.2 share blocked outputs")?;
    for field in [
        "receipt_amounts",
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "incidence_distribution_model",
        "administration_burden",
        "current_law_solver_yields",
        "reform_yields",
        "solver_input_rows",
        "solver_run",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "OMB Table 2.2 share blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB Table 2.2 share claims")?;
    for field in [
        "omb_receipt_share_context_published",
        "source_custody_ready",
        "fy2025_fy2031_receipt_share_context_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB Table 2.2 share claim {field} must be true"));
        }
    }
    for field in [
        "receipt_amounts_ready",
        "legal_receipt_base_ready",
        "economic_receipt_base_ready",
        "matched_receipt_bases_ready",
        "incidence_distribution_model_ready",
        "administration_burden_ready",
        "current_law_solver_yields_ready",
        "reform_yields_ready",
        "solver_input_ready",
        "solver_run_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("OMB Table 2.2 share claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH,
        "FY2025-FY2031",
        "FY2025 is actual",
        "FY2026-FY2031 are OMB estimates",
        "percent of total receipts",
        "99.9 or 100.1",
        "not receipt amounts",
        "not legal/economic receipt bases",
        "not assigned bases",
        "not rate bridges",
        "not solver inputs",
        "not balanced-budget claims",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB Table 2.2 share reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_receipt_amount_share_reconciliation_fy2025_2031_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing OMB receipt amount/share reconciliation artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "omb-receipt-amount-share-reconciliation-fy2025-2031-context:v1"
        || string_field(&record, "record_family")?
            != "omb_receipt_amount_share_reconciliation_context"
        || string_field(&record, "status")?
            != "draft_official_omb_amount_share_reconciliation_not_base_or_rate"
    {
        return Err("OMB receipt amount/share reconciliation identity failed".to_string());
    }

    let inputs = record
        .get("input_artifacts")
        .ok_or("OMB receipt amount/share reconciliation inputs")?;
    if string_field(inputs, "receipt_amount_context_path")?
        != OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(inputs, "receipt_share_context_path")?
            != OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH
    {
        return Err("OMB receipt amount/share reconciliation input paths failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB receipt amount/share reconciliation source custody")?;
    if custody.len() != 2 {
        return Err("OMB receipt amount/share reconciliation custody count failed".to_string());
    }
    let custody_by_id = custody
        .iter()
        .map(|entry| Ok((string_field(entry, "source_id")?, entry)))
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for (source_id, expected_len, expected_hash) in [
        (
            "SRC-OMB-HIST-2-1-FY2027",
            12_393,
            "1212da86947a71d9a0268e23d3de789402bcfb80b308cd48a32926a65b2e7bc3",
        ),
        (
            "SRC-OMB-HIST-2-2-FY2027",
            10_253,
            "9e2bc741ca3a92929551ca65c6db55901a1d96bac0cae43788c89cfac4f0fe55",
        ),
    ] {
        let entry = custody_by_id
            .get(source_id)
            .ok_or("missing OMB receipt reconciliation custody source")?;
        if int_field(entry, "byte_count")? != expected_len
            || string_field(entry, "sha256")? != expected_hash
            || string_field(entry, "review_status")? != "captured_context_only"
        {
            return Err(format!(
                "OMB receipt amount/share reconciliation custody failed for {source_id}"
            ));
        }
        let raw_path = string_field(entry, "raw_artifact_path")?;
        let raw_file = root.join(&raw_path);
        if !raw_file.exists()
            || fs::metadata(&raw_file)
                .map_err(|err| err.to_string())?
                .len()
                != expected_len as u64
            || sha256_file(&raw_file)? != expected_hash
        {
            return Err(format!(
                "OMB receipt amount/share reconciliation raw custody file failed for {source_id}"
            ));
        }
    }

    let method = record
        .get("method")
        .ok_or("OMB receipt amount/share reconciliation method")?;
    if string_field(method, "amount_unit")? != "millions_of_dollars"
        || string_field(method, "share_unit")? != "percent_of_total_receipts"
        || int_field(method, "recalculated_share_decimal_places")? != 3
        || int_field(method, "source_share_decimal_places")? != 1
    {
        return Err("OMB receipt amount/share reconciliation method failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "not_receipt_amount_source",
        "not_legal_or_economic_receipt_base",
        "not_assigned_receipt_base",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if method.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "OMB receipt amount/share reconciliation method {field} failed"
            ));
        }
    }

    let rows = record
        .get("annual_reconciliation_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB receipt amount/share reconciliation rows")?;
    if rows.len() != 7 {
        return Err("OMB receipt amount/share reconciliation row count failed".to_string());
    }
    let expected_totals = [
        (2025, "actual", 5_236_421, 1_763, 2_618),
        (2026, "estimate", 5_475_705, 1_561, 2_738),
        (2027, "estimate", 5_920_951, 2_391, 2_960),
        (2028, "estimate", 6_288_407, 2_750, 3_144),
        (2029, "estimate", 6_660_321, 3_154, 3_330),
        (2030, "estimate", 7_137_281, 3_465, 3_569),
        (2031, "estimate", 7_559_389, 3_518, 3_780),
    ]
    .into_iter()
    .map(|(year, status, total, max_diff, tolerance)| {
        (year, (status.to_string(), total, max_diff, tolerance))
    })
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (status, total, max_diff, tolerance) = expected_totals
            .get(&year)
            .ok_or("unexpected OMB receipt amount/share reconciliation year")?;
        if string_field(row, "year_status")? != *status
            || int_field(row, "total_receipts_musd")? != *total
            || int_field(row, "max_abs_rounding_difference_musd")? != *max_diff
            || int_field(row, "one_decimal_share_half_rounding_tolerance_musd")? != *tolerance
            || row
                .get("all_category_differences_within_rounding_tolerance")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "OMB receipt amount/share reconciliation row failed for FY{year}"
            ));
        }
        let checks = row
            .get("category_checks")
            .and_then(serde_json::Value::as_array)
            .ok_or("OMB receipt amount/share category checks")?;
        if checks.len() != 5 {
            return Err(format!(
                "OMB receipt amount/share category count failed for FY{year}"
            ));
        }
        for check in checks {
            if string_field(check, "category_id")?.is_empty()
                || int_field(check, "amount_musd")? <= 0
                || number_field(check, "amount_share_pct")? <= 0.0
                || number_field(check, "source_share_pct")? <= 0.0
                || int_field(check, "rounded_share_implied_amount_musd")? <= 0
            {
                return Err(format!(
                    "OMB receipt amount/share category fields failed for FY{year}"
                ));
            }
            if int_field(check, "rounding_difference_musd")?.abs() > *tolerance {
                return Err(format!(
                    "OMB receipt amount/share category tolerance failed for FY{year}"
                ));
            }
        }
    }
    if observed_years != (2025..=2031).collect::<BTreeSet<_>>() {
        return Err("OMB receipt amount/share reconciliation year coverage failed".to_string());
    }

    let aggregate = record
        .get("aggregate_reconciliation")
        .ok_or("OMB receipt amount/share aggregate reconciliation")?;
    if int_field(aggregate, "year_count")? != 7
        || int_field(aggregate, "categories_per_year")? != 5
        || aggregate
            .get("all_year_category_checks_within_rounding_tolerance")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || int_field(aggregate, "largest_abs_rounding_difference_musd")? != 3_518
        || int_field(aggregate, "largest_abs_rounding_difference_year")? != 2031
        || string_field(aggregate, "largest_abs_rounding_difference_category")?
            != "corporation_income_taxes"
    {
        return Err("OMB receipt amount/share aggregate reconciliation failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt amount/share blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "incidence_distribution_model",
        "administration_burden",
        "current_law_solver_yields",
        "reform_yields",
        "solver_input_rows",
        "solver_run",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "OMB receipt amount/share blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt amount/share claims")?;
    for field in [
        "omb_amount_share_reconciliation_published",
        "source_custody_ready",
        "fy2025_fy2031_amount_share_reconciliation_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "OMB receipt amount/share claim {field} must be true"
            ));
        }
    }
    for field in [
        "legal_receipt_base_ready",
        "economic_receipt_base_ready",
        "matched_receipt_bases_ready",
        "incidence_distribution_model_ready",
        "administration_burden_ready",
        "current_law_solver_yields_ready",
        "reform_yields_ready",
        "solver_input_ready",
        "solver_run_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "OMB receipt amount/share claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH,
        "FY2025-FY2031",
        "FY2025 is actual",
        "FY2026-FY2031 are OMB estimates",
        "one-decimal percentage",
        "half-tenth share rounding tolerance",
        "$3,518 million",
        "not a legal/economic receipt base",
        "not an assigned base",
        "not a rate bridge",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB receipt amount/share reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_cbo_revenue_overlap_reconciliation_fy2026_2031_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH,
        OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing OMB/CBO revenue overlap reconciliation artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "omb-cbo-revenue-overlap-reconciliation-fy2026-2031-context:v1"
        || string_field(&record, "record_family")?
            != "omb_cbo_revenue_overlap_reconciliation_context"
        || string_field(&record, "status")?
            != "draft_official_revenue_overlap_context_not_base_or_rate"
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "revenue-solvency"
        || string_field(&record, "omb_receipt_category_context_path")?
            != OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(&record, "cbo_revenue_detail_context_path")?
            != CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH
    {
        return Err("OMB/CBO revenue overlap identity failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("OMB/CBO revenue overlap source boundary")?;
    for field in [
        "official_public_sources",
        "local_raw_custody_present_for_both_sources",
        "omb_values_converted_from_millions",
        "not_legal_or_economic_receipt_base",
        "not_assigned_receipt_base",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB/CBO revenue overlap boundary {field} failed"));
        }
    }
    if string_field(boundary, "unit")? != "billions_usd"
        || boundary
            .get("overlap_years")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|years| years.len() != 6)
    {
        return Err("OMB/CBO revenue overlap boundary years failed".to_string());
    }

    let mapping = record
        .get("category_mapping")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB/CBO revenue overlap category mapping")?;
    if mapping.len() != 5 {
        return Err("OMB/CBO revenue overlap category mapping count failed".to_string());
    }
    let observed_mapping = mapping
        .iter()
        .map(|item| string_field(item, "comparison_id"))
        .collect::<Result<BTreeSet<_>, String>>()?;
    let expected_mapping = [
        "individual_income",
        "corporate_income",
        "payroll_social_insurance",
        "excise",
        "total_receipts",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_mapping != expected_mapping {
        return Err("OMB/CBO revenue overlap category mapping ids failed".to_string());
    }

    let rows = record
        .get("annual_summary_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB/CBO revenue overlap summary rows")?;
    let expected_rows = [
        (2026, 5475.705, 5595.916, 120.211, 2.195),
        (2027, 5920.951, 5885.198, -35.753, -0.604),
        (2028, 6288.407, 6071.468, -216.939, -3.45),
        (2029, 6660.321, 6319.789, -340.532, -5.113),
        (2030, 7137.281, 6594.999, -542.282, -7.598),
        (2031, 7559.389, 6869.493, -689.896, -9.126),
    ]
    .into_iter()
    .map(|(year, omb, cbo, diff, pct)| (year, (omb, cbo, diff, pct)))
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected_rows.len() {
        return Err("OMB/CBO revenue overlap row count failed".to_string());
    }
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (omb, cbo, diff, pct) = expected_rows
            .get(&year)
            .ok_or("unexpected OMB/CBO revenue overlap year")?;
        for (field, expected) in [
            ("omb_total_receipts", *omb),
            ("cbo_total_receipts", *cbo),
            ("cbo_minus_omb_total", *diff),
            ("cbo_minus_omb_total_pct_of_omb", *pct),
        ] {
            if (number_field(row, field)? - expected).abs() > 0.0001 {
                return Err(format!("OMB/CBO revenue overlap FY{year} {field} failed"));
            }
        }
    }
    if observed_years != (2026..=2031).collect::<BTreeSet<_>>() {
        return Err("OMB/CBO revenue overlap year coverage failed".to_string());
    }

    let extremes = record
        .get("category_difference_extremes")
        .ok_or("OMB/CBO revenue overlap extremes")?;
    let positive = extremes
        .get("largest_positive_cbo_minus_omb")
        .ok_or("OMB/CBO revenue overlap positive extreme")?;
    let negative = extremes
        .get("largest_negative_cbo_minus_omb")
        .ok_or("OMB/CBO revenue overlap negative extreme")?;
    let component = extremes
        .get("largest_negative_component_cbo_minus_omb")
        .ok_or("OMB/CBO revenue overlap component extreme")?;
    if int_field(positive, "fiscal_year")? != 2026
        || string_field(positive, "comparison_id")? != "individual_income"
        || (number_field(positive, "difference_billions")? - 121.392).abs() > 0.0001
        || int_field(negative, "fiscal_year")? != 2031
        || string_field(negative, "comparison_id")? != "total_receipts"
        || (number_field(negative, "difference_billions")? - -689.896).abs() > 0.0001
        || int_field(component, "fiscal_year")? != 2031
        || string_field(component, "comparison_id")? != "individual_income"
        || (number_field(component, "difference_billions")? - -341.637).abs() > 0.0001
    {
        return Err("OMB/CBO revenue overlap extremes failed".to_string());
    }

    for array_name in ["diagnostic_findings", "blocked_model_steps"] {
        if record
            .get(array_name)
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "OMB/CBO revenue overlap {array_name} must be nonempty"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB/CBO revenue overlap blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "OMB/CBO revenue overlap blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB/CBO revenue overlap claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("OMB/CBO revenue overlap claim bool")?;
        if matches!(
            field.as_str(),
            "omb_cbo_revenue_overlap_reconciliation_published" | "official_overlap_context_ready"
        ) {
            if !observed {
                return Err(format!(
                    "OMB/CBO revenue overlap claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "OMB/CBO revenue overlap claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "diagnostic source context only",
        "not a legal or economic receipt base",
        "not an assigned receipt base",
        "not a rate bridge",
        "not solver input",
        "not a public rate card",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "OMB/CBO revenue overlap warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH,
        "FY2026-FY2031",
        "120.211",
        "689.896",
        "341.637",
        "not a legal or economic receipt base",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB/CBO revenue overlap reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_receipt_category_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH,
        OMB_RECEIPT_CATEGORY_CONTEXT_SCHEMA_PATH,
        OMB_RECEIPT_CATEGORY_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing OMB receipt category context artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let context: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&context, "record_id")? != "omb-receipt-category-context:fy2025:v1"
        || string_field(&context, "record_family")? != "omb_receipt_category_context"
        || int_field(&context, "pulse")? != 134
        || int_field(&context, "fiscal_year")? != 2025
        || string_field(&context, "year_basis")? != "fiscal_year"
        || string_field(&context, "unit")? != "millions_of_dollars"
        || string_field(&context, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&context, "receipt_base_source_work_queue_path")?
            != RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&context, "receipt_base_local_source_inventory_path")?
            != RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH
    {
        return Err("OMB receipt category context identity failed".to_string());
    }

    for path in [
        string_field(&context, "contract_path")?,
        string_field(&context, "receipt_base_source_work_queue_path")?,
        string_field(&context, "receipt_base_local_source_inventory_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "OMB receipt category referenced path missing: {path}"
            ));
        }
    }

    let custody = context
        .get("source_custody")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category source custody")?;
    if custody.get("source_id").and_then(serde_json::Value::as_str)
        != Some("SRC-OMB-HIST-2-4-FY2027")
        || custody
            .get("raw_byte_count")
            .and_then(serde_json::Value::as_i64)
            != Some(26752)
        || custody
            .get("metadata_byte_count")
            .and_then(serde_json::Value::as_i64)
            != Some(909)
        || custody
            .get("raw_sha256")
            .and_then(serde_json::Value::as_str)
            != Some("21d071576d5627a18c3f62de86bfc7faeced1a68265f2db87b4f737b2773c5bd")
        || custody
            .get("metadata_sha256")
            .and_then(serde_json::Value::as_str)
            != Some("939d86fcbe66a7c289b7e23a4a7979fbdcac030fd138cdfe0107b1aa1cfeac46")
    {
        return Err("OMB receipt category custody metadata failed".to_string());
    }
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
    ] {
        if custody.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB receipt category custody {field} must be true"));
        }
    }
    for path_field in ["raw_artifact_path", "metadata_path"] {
        let path = custody
            .get(path_field)
            .and_then(serde_json::Value::as_str)
            .ok_or("OMB receipt category custody path")?;
        if !root.join(path).exists() {
            return Err(format!("OMB receipt category custody path missing: {path}"));
        }
    }

    let extraction = context
        .get("extraction")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category extraction")?;
    if extraction
        .get("source_column_year")
        .and_then(serde_json::Value::as_str)
        != Some("2025")
        || extraction
            .get("source_column_index")
            .and_then(serde_json::Value::as_i64)
            != Some(88)
        || extraction
            .get("extracted_rows_are_receipt_category_context")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || extraction
            .get("extracted_rows_are_assigned_receipt_bases")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("OMB receipt category extraction flags failed".to_string());
    }

    let rows = context
        .get("receipt_category_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB receipt category rows")?;
    if rows.len() != 34 {
        return Err("OMB receipt category row count failed".to_string());
    }
    let mut amounts = BTreeMap::new();
    for row in rows {
        amounts.insert(string_field(row, "row_id")?, int_field(row, "amount_musd")?);
    }

    let employment = amounts["oas_trust_funds_off_budget"]
        + amounts["di_off_budget"]
        + amounts["hospital_insurance"]
        + amounts["railroad_retirement_trust_funds"]
        + amounts["railroad_social_security_equivalent_account"];
    let social = amounts["employment_general_retirement_total"]
        + amounts["unemployment_insurance_total"]
        + amounts["other_retirement_total"];
    let excise_federal = amounts["alcohol_excise_federal_funds"]
        + amounts["tobacco_excise_federal_funds"]
        + amounts["telephone_excise_federal_funds"]
        + amounts["transportation_fuels_excise_federal_funds"]
        + amounts["corporate_stock_repurchases_excise_federal_funds"]
        + amounts["indoor_tanning_services_excise_federal_funds"]
        + amounts["other_excise_federal_funds"];
    let excise_trust = amounts["transportation_excise_trust_funds"]
        + amounts["airport_airway_excise_trust_funds"]
        + amounts["black_lung_disability_excise_trust_funds"]
        + amounts["inland_waterway_excise_trust_funds"]
        + amounts["hazardous_substance_superfund_excise_trust_funds"]
        + amounts["oil_spill_liability_excise_trust_funds"]
        + amounts["aquatic_resources_excise_trust_funds"]
        + amounts["leaking_underground_storage_tank_excise_trust_funds"]
        + amounts["tobacco_assessments_excise_trust_funds"]
        + amounts["vaccine_injury_compensation_excise_trust_funds"]
        + amounts["supplementary_medical_insurance_excise_trust_funds"]
        + amounts["patient_centered_outcomes_research_excise_trust_funds"];
    let excise = amounts["excise_federal_funds_total"] + amounts["excise_trust_funds_total"];
    let context_total =
        amounts["social_insurance_retirement_total"] + amounts["excise_taxes_total"];

    let checks = context
        .get("reconciliation_checks")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category checks")?;
    for (field, value) in [
        (
            "employment_general_retirement_components_sum_musd",
            employment,
        ),
        (
            "employment_general_retirement_total_musd",
            amounts["employment_general_retirement_total"],
        ),
        ("social_insurance_retirement_components_sum_musd", social),
        (
            "social_insurance_retirement_total_musd",
            amounts["social_insurance_retirement_total"],
        ),
        ("excise_federal_funds_components_sum_musd", excise_federal),
        (
            "excise_federal_funds_total_musd",
            amounts["excise_federal_funds_total"],
        ),
        ("excise_trust_funds_components_sum_musd", excise_trust),
        (
            "excise_trust_funds_total_musd",
            amounts["excise_trust_funds_total"],
        ),
        ("excise_components_sum_musd", excise),
        ("excise_total_musd", amounts["excise_taxes_total"]),
        ("context_total_musd", context_total),
    ] {
        if checks.get(field).and_then(serde_json::Value::as_i64) != Some(value) {
            return Err(format!("OMB receipt category check {field} failed"));
        }
    }
    if employment != amounts["employment_general_retirement_total"]
        || social != amounts["social_insurance_retirement_total"]
        || excise_federal != amounts["excise_federal_funds_total"]
        || excise_trust != amounts["excise_trust_funds_total"]
        || excise != amounts["excise_taxes_total"]
        || context_total != 1_854_231
    {
        return Err("OMB receipt category recomputation failed".to_string());
    }

    let blocked = context
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "assigned_base_rates",
        "behavioral_elasticities",
        "current_law_yields",
        "reform_yields",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "OMB receipt category blocked output {field} must be null"
            ));
        }
    }

    let claims = context
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB receipt category claims")?;
    if claims
        .get("receipt_category_context_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("OMB receipt category context published flag must be true".to_string());
    }
    for field in [
        "assigned_receipt_base_published",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("OMB receipt category claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(OMB_RECEIPT_CATEGORY_CONTEXT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH,
        "OMB receipt categories are fiscal receipt context, not legal or economic assigned receipt bases.",
        "The extracted FY2025 values do not authorize statutory rates, effective rates, public rate cards, solver inputs, tax proposals, or balanced-budget claims.",
        "A receipt category amount is not the denominator for an effective tax rate.",
        "Missing source-display rows remain omitted or null, never zero-filled.",
        "No external request was submitted and no agency or person was contacted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "OMB receipt category reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_pbd_fy2027_user_guide_horizon_boundary_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH,
        OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing OMB PBD guide context artifact: {path}"));
        }
    }

    let text = fs::read_to_string(
        root.join(OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "omb-pbd-fy2027-user-guide-horizon-boundary-context:v1"
        || string_field(&record, "record_family")? != "omb_pbd_user_guide_horizon_boundary_context"
        || string_field(&record, "status")?
            != "draft_source_custody_documentation_added_values_still_blocked"
        || string_field(&record, "gap_id")? != "fy2032_fy2035_omb_17_row_ledger"
    {
        return Err("OMB PBD guide context identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("OMB PBD guide custody")?;
    if string_field(custody, "source_id")? != "SRC-OMB-PBD-GUIDE-FY2027"
        || string_field(custody, "publisher")? != "Office of Management and Budget"
        || string_field(custody, "url")?
            != "https://www.whitehouse.gov/wp-content/uploads/2026/04/db_guide_fy2027.pdf"
        || string_field(custody, "retrieval_date")? != "2026-07-24"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-PBD-GUIDE-FY2027/2026-07-24/db_guide_fy2027.pdf"
        || int_field(custody, "raw_byte_count")? != 249_763
        || string_field(custody, "raw_sha256")?
            != "cc4871365009d485dae32bc6aeaa402005f06151516399efd8499c72f0542bb3"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-OMB-PBD-GUIDE-FY2027.2026-07-24.metadata.md"
        || string_field(custody, "review_status")? != "reviewed_for_pbd_file_boundary_not_values"
    {
        return Err("OMB PBD guide custody fields failed".to_string());
    }
    let raw_path = root.join(string_field(custody, "raw_artifact_path")?);
    if !raw_path.exists()
        || raw_path.metadata().map_err(|err| err.to_string())?.len() != 249_763
        || sha256_file(&raw_path)?
            != "cc4871365009d485dae32bc6aeaa402005f06151516399efd8499c72f0542bb3"
        || !root.join(string_field(custody, "metadata_path")?).exists()
    {
        return Err("OMB PBD guide raw custody failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("OMB PBD guide source boundary")?;
    for field in [
        "official_public_source",
        "local_raw_custody",
        "documentation_only",
        "not_omb_17_row_values",
        "not_interpolation",
        "not_solver_input",
        "not_rate_or_savings_claim",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB PBD guide boundary {field} must be true"));
        }
    }

    let extract = record
        .get("guide_extract_boundary")
        .ok_or("OMB PBD guide extract boundary")?;
    if extract
        .get("describes_three_data_files")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || extract
            .get("account_level_spreadsheet_format")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || string_field(extract, "source_unit")? != "thousands_of_dollars"
        || !string_field(extract, "guide_year_horizon_text")?.contains("FY2031")
        || !string_field(extract, "limitation_boundary")?
            .contains("does not provide FY2032-FY2035 OMB 17-row rows")
    {
        return Err("OMB PBD guide extract boundary failed".to_string());
    }

    let files = record
        .get("pbd_data_file_boundary")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB PBD guide file boundary")?;
    let expected_files = [
        ("outlays", "outlays.xlsx", "1962-2031"),
        ("receipts", "receipts.xlsx", "1962-2031"),
        ("budget_authority", "budauth.xlsx", "1976-2031"),
    ]
    .into_iter()
    .collect::<Vec<_>>();
    if files.len() != expected_files.len() {
        return Err("OMB PBD guide file boundary count failed".to_string());
    }
    for (role, name, years) in expected_files {
        let file = files
            .iter()
            .find(|file| file.get("file_role").and_then(serde_json::Value::as_str) == Some(role))
            .ok_or("OMB PBD guide expected file role")?;
        if string_field(file, "guide_file_name")? != name
            || !string_field(file, "guide_content")?.contains(years)
        {
            return Err(format!("OMB PBD guide file role failed: {role}"));
        }
    }

    let ledger = record
        .get("local_ledger_boundary")
        .ok_or("OMB PBD guide local ledger boundary")?;
    if string_field(ledger, "existing_omb_17_row_context_path")?
        != CURRENT_LAW_17_ROW_PBD_FY2025_2031_CONTEXT_PATH_JSON_PATH
        || string_field(ledger, "existing_cbo_topline_context_path")?
            != CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH
        || string_field(ledger, "existing_cbo_major_outlay_category_context_path")?
            != CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH
        || ledger
            .get("cbo_context_available_for_missing_years")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || ledger
            .get("cbo_context_is_not_omb_17_row_ledger")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || ledger
            .get("fy2032_fy2035_values_remain_null")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("OMB PBD guide local ledger boundary failed".to_string());
    }
    let covered = ledger
        .get("omb_pbd_outlay_covered_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB PBD guide covered years")?
        .iter()
        .map(|value| value.as_i64().ok_or("OMB PBD guide covered year int"))
        .collect::<Result<Vec<_>, _>>()?;
    let missing = ledger
        .get("omb_17_row_missing_years")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB PBD guide missing years")?
        .iter()
        .map(|value| value.as_i64().ok_or("OMB PBD guide missing year int"))
        .collect::<Result<Vec<_>, _>>()?;
    if covered != (2025..=2031).map(i64::from).collect::<Vec<_>>()
        || missing != (2032..=2035).map(i64::from).collect::<Vec<_>>()
    {
        return Err("OMB PBD guide year boundary failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB PBD guide blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("OMB PBD guide blocked output {field} must be null"));
        }
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB PBD guide claims")?;
    for field in [
        "omb_pbd_user_guide_source_custody_published",
        "pbd_file_boundary_documented",
        "omb_pbd_outlay_horizon_boundary_documented",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB PBD guide claim {field} must be true"));
        }
    }
    for field in [
        "fy2032_fy2035_omb_17_row_values_ready",
        "complete_fy2025_fy2035_omb_17_row_path_ready",
        "unified_omb_cbo_reconciliation_ready",
        "solver_input_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("OMB PBD guide claim {field} must be false"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "Public Budget Database User's Guide",
        "three data files",
        "FY2031 PBD horizon boundary",
        "not FY2032-FY2035 OMB 17-row values",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!("OMB PBD guide warning missing: {phrase}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_PBD_FY2027_USER_GUIDE_HORIZON_BOUNDARY_CONTEXT_JSON_PATH,
        "Public Budget Database User's Guide",
        "three data files",
        "FY2032-FY2035",
        "not OMB 17-row values",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!("OMB PBD guide reader missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_omb_ap13_fund_group_reconciliation_detail_context(root: &Path) -> Result<(), String> {
    for path in [
        OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH,
        OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing OMB AP13 fund-group reconciliation detail artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "omb-ap13-fund-group-reconciliation-detail-fy2025-context:v1"
        || string_field(&record, "record_family")?
            != "omb_ap13_fund_group_reconciliation_detail_context"
        || string_field(&record, "status")?
            != "draft_fund_group_reconciliation_detail_context_general_fund_blocked"
        || int_field(&record, "fiscal_year")? != 2025
        || string_field(&record, "unit")? != "billions_of_dollars"
        || string_field(&record, "source_id")? != "SRC-OMB-AP-13-TABLES-FY2027"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/omb/SRC-OMB-AP-13-TABLES-FY2027/2026-07-23/ap_13_tables_fy2027.xlsx"
        || int_field(&record, "raw_byte_count")? != 47_862
        || string_field(&record, "raw_sha256")?
            != "86e550d366f218435f3ef9af43bafe37ff5a2e496680013a7d1dbcab7737c505"
        || !root
            .join(string_field(&record, "raw_artifact_path")?)
            .exists()
    {
        return Err("OMB AP13 fund-group detail identity failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("OMB AP13 source boundary")?;
    for field in [
        "official_public_source",
        "local_raw_custody_ready",
        "context_only",
        "federal_funds_not_general_fund",
        "trust_fund_group_not_named_trust_funds",
        "not_forward_annual_path",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("OMB AP13 boundary flag {field} must be true"));
        }
    }

    let tables = boundary
        .get("tables_extracted")
        .and_then(serde_json::Value::as_array)
        .ok_or("OMB AP13 tables extracted")?;
    let observed_tables = tables
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| "OMB AP13 table id string".to_string())
                .map(str::to_string)
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_tables = ["13-1", "13-2", "13-3"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_tables != expected_tables {
        return Err("OMB AP13 table set failed".to_string());
    }

    let check_number =
        |object: &serde_json::Value, field: &str, expected: f64| -> Result<(), String> {
            let observed = object
                .get(field)
                .and_then(serde_json::Value::as_f64)
                .ok_or_else(|| format!("OMB AP13 numeric field missing: {field}"))?;
            if (observed - expected).abs() > 0.000_001 {
                return Err(format!("OMB AP13 numeric field failed: {field}"));
            }
            Ok(())
        };

    let table_13_1 = record
        .get("table_13_1_receipts_outlays_surplus_deficit_by_fund_group")
        .ok_or("OMB AP13 table 13-1")?;
    for (field, expected) in [
        ("total_federal_funds_cash_income", 4050.1),
        ("total_trust_funds_cash_income", 3300.2),
        ("total_unified_budget_receipts", 5236.4),
        ("total_unified_budget_outlays", 7011.1),
        ("surplus_deficit_federal_funds", -1871.0),
        ("surplus_deficit_trust_funds", 96.3),
        ("total_unified_surplus_deficit", -1774.7),
    ] {
        check_number(table_13_1, field, expected)?;
    }

    let table_13_2 = record
        .get("table_13_2_receipts_reconciliation")
        .ok_or("OMB AP13 table 13-2")?;
    for (field, expected) in [
        ("total_gross_federal_and_trust_fund_cash_income", 7901.1),
        ("cash_income_net_federal_funds", 4050.1),
        ("cash_income_net_trust_funds", 3300.2),
        ("general_fund_payments_to_medicare_parts_b_and_d", -549.7),
        ("subtotal_trust_fund_receipts_from_federal_funds", -1184.1),
        ("total_unified_budget_receipts", 5236.4),
    ] {
        check_number(table_13_2, field, expected)?;
    }

    let table_13_3 = record
        .get("table_13_3_trust_funds_group_income_outgo_balances")
        .ok_or("OMB AP13 table 13-3")?;
    for (field, expected) in [
        ("total_balance_start_of_year", 6185.3),
        ("total_income_during_year", 3368.0),
        ("outgo", -3271.7),
        ("subtotal_surplus_deficit", 96.3),
        ("total_change_in_fund_balance", 96.4),
        ("balance_end_of_year", 6281.7),
    ] {
        check_number(table_13_3, field, expected)?;
    }

    let checks = record
        .get("reconciliation_checks")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB AP13 reconciliation checks")?;
    for (field, value) in checks {
        if value.as_bool() != Some(true) {
            return Err(format!(
                "OMB AP13 reconciliation check must be true: {field}"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB AP13 blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("OMB AP13 blocked output must be null: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("OMB AP13 claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("OMB AP13 claim bool")?;
        if matches!(
            field.as_str(),
            "omb_ap13_fund_group_reconciliation_detail_context_published"
                | "local_raw_custody_ready"
                | "fy2025_fund_group_detail_context_ready"
        ) {
            if !observed {
                return Err(format!("OMB AP13 claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("OMB AP13 claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "FY2025 fund-group reconciliation detail",
        "Federal funds are still broader than the general fund",
        "not a general-fund annual path",
        "not named trust-fund reconciliation",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!("OMB AP13 warning missing phrase: {phrase}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        OMB_AP13_FUND_GROUP_RECONCILIATION_DETAIL_CONTEXT_JSON_PATH,
        "Table 13-1",
        "Table 13-2",
        "Table 13-3",
        "Federal funds remain broader than the general fund",
        "not OASDI, Medicare HI, or transportation trust-fund paths",
    ] {
        if !reader.contains(phrase) {
            return Err(format!("OMB AP13 reader missing phrase: {phrase}"));
        }
    }

    Ok(())
}

