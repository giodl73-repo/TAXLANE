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

pub(crate) fn validate_cbo_open_data_fy2032_2035_current_law_extension_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH,
        CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CBO current-law extension context artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(
        CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH,
    ))
    .map_err(|err| {
        format!(
            "failed to read {CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "cbo-open-data-fy2032-2035-current-law-extension-context:v1"
        || string_field(&record, "record_family")? != "cbo_open_data_current_law_extension_context"
        || string_field(&record, "status")?
            != "draft_official_cbo_extension_context_not_omb_17_row_reconciliation"
    {
        return Err("CBO current-law extension context identity failed".to_string());
    }

    let packets = record
        .get("source_custody_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO current-law extension source packets")?;
    let expected_packets = [
        (
            "SRC-CBO-OPEN-DATA-TEN-YEAR-BUDGET-2026-02",
            "data/raw/cbo/SRC-CBO-OPEN-DATA-TEN-YEAR-BUDGET-2026-02/2026-07-23/annual_fy_2026-02.csv",
            145_228,
            "6a2d727e70fb53512e45afdcc8d145f7dc952b2d4e87ca03591cb4e5bd63f0db",
            3333,
        ),
        (
            "SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02",
            "data/raw/cbo/SRC-CBO-OPEN-DATA-TRUST-FUND-2026-02/2026-07-23/annual_fy_2026-02.csv",
            25_466,
            "4fcc0f725e6ab002107bedf461e21dacbdc6ac49e85475b8ff3e8aa20c3cdaab",
            728,
        ),
        (
            "SRC-CBO-OPEN-DATA-REVENUE-DETAIL-2026-02",
            "data/raw/cbo/SRC-CBO-OPEN-DATA-REVENUE-DETAIL-2026-02/2026-07-23/annual_fy_2026-02.csv",
            76_738,
            "86b5f5ec7142533875b2d69b69dfa5b259cb2b477c37c403a3414012fd04b241",
            1700,
        ),
    ]
    .into_iter()
    .map(|(source_id, path, bytes, sha, rows)| {
        (
            source_id.to_string(),
            (path.to_string(), bytes, sha.to_string(), rows),
        )
    })
    .collect::<BTreeMap<_, _>>();
    if packets.len() != expected_packets.len() {
        return Err("CBO current-law extension packet count failed".to_string());
    }
    for packet in packets {
        let source_id = string_field(packet, "source_id")?;
        let (expected_path, expected_bytes, expected_sha, expected_rows) = expected_packets
            .get(&source_id)
            .ok_or("unexpected CBO current-law extension source packet")?;
        if string_field(packet, "publisher")? != "Congressional Budget Office"
            || string_field(packet, "raw_artifact_path")? != *expected_path
            || int_field(packet, "byte_count")? != *expected_bytes
            || string_field(packet, "sha256")? != *expected_sha
            || int_field(packet, "row_count")? != *expected_rows
            || string_field(packet, "review_status")? != "captured_context_only"
        {
            return Err(format!(
                "CBO current-law extension source packet failed: {source_id}"
            ));
        }
        let file = root.join(expected_path);
        if !file.exists() {
            return Err(format!(
                "CBO current-law extension raw file missing: {expected_path}"
            ));
        }
        if fs::metadata(&file).map_err(|err| err.to_string())?.len() as i64 != *expected_bytes {
            return Err(format!(
                "CBO current-law extension byte count failed: {source_id}"
            ));
        }
        if sha256_file(&file)? != *expected_sha {
            return Err(format!(
                "CBO current-law extension hash failed: {source_id}"
            ));
        }
        let metadata_path = string_field(packet, "metadata_path")?;
        if !root.join(&metadata_path).exists() {
            return Err(format!(
                "CBO current-law extension metadata missing: {metadata_path}"
            ));
        }
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("CBO current-law extension extraction scope")?;
    for field in [
        "official_sources_only",
        "direct_cbo_workbook_download_blocked_by_site_protection",
        "official_github_open_data_used",
        "no_interpolation_used",
        "not_omb_pbd_row_mapping",
        "not_17_row_lane_ledger",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO current-law extension scope {field} failed"));
        }
    }

    let rows = record
        .get("topline_extension_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO current-law extension topline rows")?;
    let expected_outlays = [
        (2032, 9569.1, 7129.682, -2439.418, 1670.447),
        (2033, 10172.211, 7391.476, -2780.735, 1784.364),
        (2034, 10487.423, 7668.923, -2818.5, 1903.751),
        (2035, 10750.897, 7971.863, -2779.034, 2019.072),
    ]
    .into_iter()
    .map(|(year, outlays, revenues, deficit, interest)| {
        (year, (outlays, revenues, deficit, interest))
    })
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected_outlays.len() {
        return Err("CBO current-law extension topline row count failed".to_string());
    }
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        let (outlays, revenues, deficit, interest) = expected_outlays
            .get(&year)
            .ok_or("unexpected CBO current-law extension topline year")?;
        if (number_field(row, "outlays_total")? - outlays).abs() > 0.0001
            || (number_field(row, "revenues_total")? - revenues).abs() > 0.0001
            || (number_field(row, "deficit_total")? - deficit).abs() > 0.0001
            || (number_field(row, "net_interest_outlays")? - interest).abs() > 0.0001
        {
            return Err(format!(
                "CBO current-law extension topline values failed for FY{year}"
            ));
        }
    }

    let trust_rows = record
        .get("trust_fund_balance_extension_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO current-law extension trust rows")?;
    if trust_rows.len() != 4
        || !trust_rows.iter().any(|row| {
            int_field(row, "fiscal_year").ok() == Some(2035)
                && number_field(row, "airport_and_airway").ok() == Some(52.21)
                && number_field(row, "di").ok() == Some(801.398)
        })
    {
        return Err("CBO current-law extension trust rows failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO current-law extension blocked outputs")?;
    for field in [
        "omb_17_row_fy2032_fy2035_ledger",
        "lane_level_fy2032_fy2035_outlay_rows",
        "trust_fund_income_outgo_reconciliation",
        "receipt_rate_bridge",
        "solver_input_rows",
        "solver_run",
        "target_cost",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "savings_estimate",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "CBO current-law extension blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO current-law extension claims")?;
    for field in [
        "cbo_open_data_current_law_extension_context_published",
        "source_custody_ready",
        "fy2032_fy2035_topline_context_present",
        "fy2032_fy2035_revenue_detail_context_present",
        "fy2032_fy2035_trust_fund_balance_context_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "CBO current-law extension claim {field} must be true"
            ));
        }
    }
    for field in [
        "omb_17_row_fy2032_fy2035_ledger_ready",
        "all_current_law_paths_complete",
        "trust_fund_reconciliation_ready",
        "receipt_rate_bridge_ready",
        "solver_input_ready",
        "solver_run_published",
        "target_cost_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "savings_claim_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "CBO current-law extension claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        CBO_OPEN_DATA_FY2032_2035_CURRENT_LAW_EXTENSION_CONTEXT_JSON_PATH,
        "FY2032-FY2035",
        "official CBO open-data CSVs",
        "not an OMB",
        "not a lane-level baseline",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CBO current-law extension reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cbo_major_outlay_category_fy2032_2035_context(root: &Path) -> Result<(), String> {
    for path in [
        CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH,
        CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CBO major outlay category artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!("failed to read {CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH}: {err}")
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "cbo-major-outlay-category-fy2032-2035-context:v1"
        || string_field(&record, "record_family")? != "cbo_major_outlay_category_context"
        || string_field(&record, "status")?
            != "draft_official_cbo_category_context_not_omb_lane_ledger"
    {
        return Err("CBO major outlay category context identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("CBO major outlay category source custody")?;
    if string_field(custody, "source_id")? != "SRC-CBO-OPEN-DATA-TEN-YEAR-BUDGET-2026-02"
        || string_field(custody, "publisher")? != "Congressional Budget Office"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/cbo/SRC-CBO-OPEN-DATA-TEN-YEAR-BUDGET-2026-02/2026-07-23/annual_fy_2026-02.csv"
        || int_field(custody, "byte_count")? != 145_228
        || string_field(custody, "sha256")?
            != "6a2d727e70fb53512e45afdcc8d145f7dc952b2d4e87ca03591cb4e5bd63f0db"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("CBO major outlay category source custody failed".to_string());
    }
    let raw_path = string_field(custody, "raw_artifact_path")?;
    let raw_file = root.join(&raw_path);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 145_228
        || sha256_file(&raw_file)?
            != "6a2d727e70fb53512e45afdcc8d145f7dc952b2d4e87ca03591cb4e5bd63f0db"
    {
        return Err("CBO major outlay category raw custody file failed".to_string());
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("CBO major outlay category extraction scope")?;
    if string_field(scope, "projection_release")? != "2026-02"
        || string_field(scope, "source_unit")? != "billions_usd"
        || int_field(scope, "selected_variable_count_per_year")? != 23
    {
        return Err("CBO major outlay category extraction scope fields failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "not_omb_function_mapping",
        "not_taxlane_17_row_ledger",
        "not_lane_level_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO major outlay category scope {field} failed"));
        }
    }

    let rows = record
        .get("annual_category_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO major outlay category annual rows")?;
    if rows.len() != 10 {
        return Err("CBO major outlay category annual row count failed".to_string());
    }
    let expected = [
        (2026, 884.513, 1665.942, 1286.868, 389.09, 300.733),
        (2027, 901.101, 1768.983, 1373.623, 389.067, 324.065),
        (2028, 928.172, 1875.116, 1533.279, 397.755, 367.414),
        (2029, 938.416, 1978.541, 1470.2, 393.372, 338.315),
        (2030, 965.808, 2082.769, 1648.614, 404.541, 380.136),
        (2031, 985.535, 2189.776, 1752.64, 411.167, 393.56),
        (2032, 1006.461, 2298.339, 1866.597, 420.87, 411.281),
        (2033, 1034.483, 2408.571, 2122.822, 434.134, 458.452),
        (2034, 1050.676, 2519.408, 2153.252, 436.146, 451.943),
        (2035, 1068.288, 2631.845, 2154.893, 435.381, 439.039),
    ]
    .into_iter()
    .map(
        |(year, defense, social_security, medicare, income_security, veterans)| {
            (
                year,
                (
                    defense,
                    social_security,
                    medicare,
                    income_security,
                    veterans,
                ),
            )
        },
    )
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (defense, social_security, medicare, income_security, veterans) =
            expected
                .get(&year)
                .ok_or("unexpected CBO major outlay category year")?;
        if (number_field(row, "discretionary_defense")? - defense).abs() > 0.0001
            || (number_field(row, "mandatory_social_security")? - social_security).abs() > 0.0001
            || (number_field(row, "mandatory_medicare")? - medicare).abs() > 0.0001
            || (number_field(row, "mandatory_income_security")? - income_security).abs() > 0.0001
            || (number_field(row, "mandatory_veterans")? - veterans).abs() > 0.0001
        {
            return Err(format!(
                "CBO major outlay category values failed for FY{year}"
            ));
        }
        for field in [
            "mandatory_oasi",
            "mandatory_di",
            "mandatory_medicaid",
            "mandatory_health_care",
            "mandatory_snap",
            "mandatory_family_support",
            "mandatory_child_nutrition",
            "mandatory_agriculture",
            "mandatory_higher_education",
            "mandatory_admin_justice",
            "expired_total_outlays",
        ] {
            number_field(row, field)?;
        }
    }
    if observed_years != (2026..=2035).collect::<BTreeSet<_>>() {
        return Err("CBO major outlay category year coverage failed".to_string());
    }

    let lane_rows = record
        .get("lane_context_boundaries")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO major outlay category lane boundaries")?;
    if lane_rows.len() != 9 {
        return Err("CBO major outlay category lane boundary count failed".to_string());
    }
    let lane_ids = lane_rows
        .iter()
        .map(|row| string_field(row, "lane_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "national-defense",
        "social-security",
        "health-medicare",
        "income-security-family",
        "veterans",
        "agriculture",
        "transportation-infrastructure",
        "education-workforce",
        "justice-courts-public-safety",
    ] {
        if !lane_ids.contains(required) {
            return Err(format!("CBO major outlay category missing lane {required}"));
        }
    }
    for row in lane_rows {
        if row
            .get("context_fields")
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
            || string_field(row, "blocked_boundary")?.is_empty()
        {
            return Err("CBO major outlay category lane boundary fields failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO major outlay category blocked outputs")?;
    for field in [
        "omb_17_row_fy2032_fy2035_ledger",
        "taxlane_lane_baseline_rows",
        "component_policy_paths",
        "outcome_floor_values",
        "transition_model",
        "solver_input_rows",
        "solver_run",
        "target_cost",
        "public_rate_card",
        "savings_estimate",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "CBO major outlay category blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO major outlay category claims")?;
    for field in [
        "cbo_major_outlay_category_context_published",
        "source_custody_ready",
        "fy2026_fy2035_category_context_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "CBO major outlay category claim {field} must be true"
            ));
        }
    }
    for field in [
        "omb_17_row_fy2032_fy2035_ledger_ready",
        "taxlane_lane_baseline_ready",
        "component_policy_paths_ready",
        "outcome_floor_values_ready",
        "transition_model_ready",
        "solver_input_ready",
        "solver_run_published",
        "target_cost_published",
        "public_rate_card_published",
        "savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "CBO major outlay category claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_READER_PATH}: {err}")
    })?;
    for phrase in [
        CBO_MAJOR_OUTLAY_CATEGORY_FY2032_2035_CONTEXT_JSON_PATH,
        "FY2026-FY2035",
        "official CBO February 2026 open-data category context",
        "not an OMB",
        "not a Taxlane lane baseline",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CBO major outlay category reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cbo_revenue_detail_fy2026_2035_context(root: &Path) -> Result<(), String> {
    for path in [
        CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH,
        CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing CBO revenue detail artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH}: {err}")
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "cbo-revenue-detail-fy2026-2035-context:v1"
        || string_field(&record, "record_family")? != "cbo_revenue_detail_context"
        || string_field(&record, "status")?
            != "draft_official_cbo_revenue_context_not_receipt_rate_bridge"
    {
        return Err("CBO revenue detail context identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("CBO revenue detail source custody")?;
    if string_field(custody, "source_id")? != "SRC-CBO-OPEN-DATA-REVENUE-DETAIL-2026-02"
        || string_field(custody, "publisher")? != "Congressional Budget Office"
        || string_field(custody, "raw_artifact_path")?
            != "data/raw/cbo/SRC-CBO-OPEN-DATA-REVENUE-DETAIL-2026-02/2026-07-23/annual_fy_2026-02.csv"
        || int_field(custody, "byte_count")? != 76_738
        || string_field(custody, "sha256")?
            != "86b5f5ec7142533875b2d69b69dfa5b259cb2b477c37c403a3414012fd04b241"
        || string_field(custody, "review_status")? != "captured_context_only"
    {
        return Err("CBO revenue detail source custody failed".to_string());
    }
    let raw_path = string_field(custody, "raw_artifact_path")?;
    let raw_file = root.join(&raw_path);
    if !raw_file.exists()
        || fs::metadata(&raw_file)
            .map_err(|err| err.to_string())?
            .len()
            != 76_738
        || sha256_file(&raw_file)?
            != "86b5f5ec7142533875b2d69b69dfa5b259cb2b477c37c403a3414012fd04b241"
    {
        return Err("CBO revenue detail raw custody file failed".to_string());
    }

    let scope = record
        .get("extraction_scope")
        .ok_or("CBO revenue detail extraction scope")?;
    if string_field(scope, "projection_release")? != "2026-02"
        || string_field(scope, "source_unit")? != "billions_usd"
        || int_field(scope, "selected_variable_count_per_year")? != 9
    {
        return Err("CBO revenue detail extraction scope fields failed".to_string());
    }
    for field in [
        "no_interpolation_used",
        "not_legal_or_economic_receipt_base",
        "not_incidence_or_distribution_model",
        "not_rate_bridge",
        "not_solver_input",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO revenue detail scope {field} failed"));
        }
    }

    let rows = record
        .get("annual_revenue_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO revenue detail annual rows")?;
    if rows.len() != 10 {
        return Err("CBO revenue detail annual row count failed".to_string());
    }
    let expected = [
        (2026, 2751.291, 1825.573, 403.979, 5595.916),
        (2027, 2947.025, 1896.664, 419.298, 5885.198),
        (2028, 3044.174, 1970.161, 432.534, 6071.468),
        (2029, 3193.323, 2048.466, 451.539, 6319.789),
        (2030, 3338.979, 2131.557, 477.34, 6594.999),
        (2031, 3461.053, 2216.305, 493.465, 6869.493),
        (2032, 3594.393, 2301.556, 509.962, 7129.682),
        (2033, 3743.854, 2388.562, 525.923, 7391.476),
        (2034, 3902.609, 2477.672, 539.067, 7668.923),
        (2035, 4071.909, 2570.009, 551.906, 7971.863),
    ]
    .into_iter()
    .map(
        |(year, individual_income, payroll, corporate_income, total)| {
            (year, (individual_income, payroll, corporate_income, total))
        },
    )
    .collect::<BTreeMap<_, _>>();
    let mut observed_years = BTreeSet::new();
    for row in rows {
        let year = int_field(row, "fiscal_year")? as i32;
        observed_years.insert(year);
        let (individual_income, payroll, corporate_income, total) = expected
            .get(&year)
            .ok_or("unexpected CBO revenue detail year")?;
        if (number_field(row, "individual_income")? - individual_income).abs() > 0.0001
            || (number_field(row, "payroll")? - payroll).abs() > 0.0001
            || (number_field(row, "corporate_income")? - corporate_income).abs() > 0.0001
            || (number_field(row, "total")? - total).abs() > 0.0001
        {
            return Err(format!("CBO revenue detail values failed for FY{year}"));
        }
        for field in [
            "excise",
            "customs",
            "estate_gift",
            "federal_reserve",
            "misc_fees",
        ] {
            number_field(row, field)?;
        }
    }
    if observed_years != (2026..=2035).collect::<BTreeSet<_>>() {
        return Err("CBO revenue detail year coverage failed".to_string());
    }

    let boundaries = record
        .get("receipt_context_boundaries")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO revenue detail receipt boundaries")?;
    if boundaries.len() != 8 {
        return Err("CBO revenue detail receipt boundary count failed".to_string());
    }
    let receipt_ids = boundaries
        .iter()
        .map(|row| string_field(row, "receipt_context_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required in [
        "individual_income",
        "payroll",
        "corporate_income",
        "excise",
        "customs",
        "estate_gift",
        "federal_reserve",
        "misc_fees",
    ] {
        if !receipt_ids.contains(required) {
            return Err(format!("CBO revenue detail missing receipt {required}"));
        }
    }
    for row in boundaries {
        if string_field(row, "context_field")?.is_empty()
            || string_field(row, "blocked_boundary")?.is_empty()
        {
            return Err("CBO revenue detail boundary fields failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO revenue detail blocked outputs")?;
    for field in [
        "matched_receipt_bases",
        "legal_economic_base",
        "payer_universe",
        "incidence_distribution_model",
        "administration_burden",
        "current_law_solver_yield",
        "reform_yield",
        "solver_input_rows",
        "solver_run",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "savings_estimate",
        "tax_proposal",
        "balanced_budget_claim",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!(
                "CBO revenue detail blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO revenue detail claims")?;
    for field in [
        "cbo_revenue_detail_context_published",
        "source_custody_ready",
        "fy2026_fy2035_revenue_context_present",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO revenue detail claim {field} must be true"));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "legal_economic_base_ready",
        "payer_universe_ready",
        "incidence_distribution_model_ready",
        "administration_burden_ready",
        "current_law_solver_yield_ready",
        "reform_yield_ready",
        "solver_input_ready",
        "solver_run_published",
        "statutory_rate_published",
        "effective_rate_published",
        "public_rate_card_published",
        "tax_proposal_published",
        "savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("CBO revenue detail claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_READER_PATH))
        .map_err(|err| {
            format!("failed to read {CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_READER_PATH}: {err}")
        })?;
    for phrase in [
        CBO_REVENUE_DETAIL_FY2026_2035_CONTEXT_JSON_PATH,
        "FY2026-FY2035",
        "official CBO February 2026 revenue-detail context",
        "not a legal/economic base",
        "not a rate bridge",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CBO revenue detail reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cbo_health_insurance_baseline_browser_context_fy2026_2036(
    root: &Path,
) -> Result<(), String> {
    for path in [
        CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH,
        CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CBO health browser context artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "cbo-health-insurance-baseline-browser-context-fy2026-2036:v1"
        || string_field(&record, "record_family")?
            != "cbo_health_insurance_baseline_browser_context"
        || string_field(&record, "status")?
            != "draft_manual_local_raw_custody_captured_latest_publication_raw_blocked"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "as_of_date")? != "2026-07-24"
    {
        return Err("CBO health browser context identity failed".to_string());
    }

    let source = record
        .get("official_source_context")
        .ok_or("CBO health browser source context")?;
    if string_field(source, "publisher")? != "Congressional Budget Office"
        || string_field(source, "selected_programs_page_url")?
            != "https://www.cbo.gov/data/baseline-projections-selected-programs"
        || source
            .get("selected_programs_page_browser_verified")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || string_field(source, "browser_visible_pdf_url")?
            != "https://www.cbo.gov/system/files/2026-02/51298-2026-02-healthinsurance.pdf"
        || string_field(source, "browser_visible_pdf_title")?
            != "Federal Subsidies for Health Insurance Baseline--02-2026"
        || int_field(source, "browser_visible_pdf_page_count")? != 5
        || !string_field(source, "browser_visible_pdf_context")?
            .contains("discretionary outlays and federal-employer outlays excluded")
    {
        return Err("CBO health browser source context failed".to_string());
    }
    let latest = source
        .get("latest_official_publication_context")
        .ok_or("CBO health latest official publication context")?;
    if string_field(latest, "publication_url")? != "https://www.cbo.gov/publication/62539"
        || string_field(latest, "publication_title")?
            != "Federal Subsidies for Health Insurance, 2026 to 2036"
        || string_field(latest, "publication_date")? != "2026-07-23"
        || latest
            .get("search_index_or_homepage_visible")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || !string_field(latest, "context_summary")?.contains("403")
        || !string_field(latest, "context_summary")?.contains("no local raw byte custody")
    {
        return Err("CBO health latest publication boundary failed".to_string());
    }
    let may_presentation = source
        .get("may_2026_presentation_context")
        .ok_or("CBO health May presentation context")?;
    if string_field(may_presentation, "publication_url")? != "https://www.cbo.gov/publication/62380"
        || string_field(may_presentation, "publication_title")?
            != "CBO's Baseline Projections of Federal Subsidies for Health Insurance"
        || string_field(may_presentation, "publication_date")? != "2026-05-11"
        || string_field(may_presentation, "publication_type")? != "Presentation"
        || string_field(may_presentation, "browser_visible_document_url")?
            != "https://www.cbo.gov/system/files/2026-05/62380-Federal-Health-Subsidies.pdf"
        || int_field(may_presentation, "browser_visible_document_page_count")? != 23
        || string_field(may_presentation, "browser_visible_data_url")?
            != "https://www.cbo.gov/system/files/2026-05/62380-Data.xlsx"
        || !string_field(may_presentation, "context_summary")?.contains("HTTP 403")
        || !string_field(may_presentation, "context_summary")?.contains("no local byte custody")
    {
        return Err("CBO health May presentation boundary failed".to_string());
    }
    let categories = source
        .get("selected_programs_health_categories")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health selected-program categories")?;
    for category in [
        "Children's Health Insurance Program",
        "Federal Subsidies for Health Insurance",
        "Medicaid",
        "Medicare",
        "Premium Tax Credit and Related Spending",
    ] {
        if !categories
            .iter()
            .any(|value| value.as_str() == Some(category))
        {
            return Err(format!("CBO health missing category {category}"));
        }
    }

    let custody = record
        .get("local_raw_custody")
        .ok_or("CBO health local raw custody")?;
    if custody
        .get("manual_browser_download_raw_custody_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || custody
            .get("command_line_raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02.2026-07-24.metadata.md"
        || string_field(custody, "source_id")?
            != "SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02"
        || string_field(custody, "review_status")?
            != "manual_local_raw_custody_captured_for_february_2026_baseline"
        || !string_field(custody, "access_boundary")?.contains("anti-bot/JavaScript challenge")
    {
        return Err("CBO health local raw custody boundary failed".to_string());
    }
    if !root.join(string_field(custody, "metadata_path")?).exists() {
        return Err("CBO health metadata file missing".to_string());
    }
    let raw_files = custody
        .get("raw_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health local raw files")?;
    let expected_raw = [
        (
            "official_pdf",
            (
                "data/raw/cbo/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02/2026-07-24/51298-2026-02-healthinsurance.pdf",
                747_901,
                "2c24c10b855be1e1a9e9c87a30ddaf5b4c62a8dbe9d92d3e2ebf18524a54d349",
            ),
        ),
        (
            "official_spreadsheet",
            (
                "data/raw/cbo/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02/2026-07-24/51298-2026-02-healthinsurance.xlsx",
                42_861,
                "f2d7cc186f3a0afa909e648f8224a1f7f80af202db234848b507fd49416e1001",
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if raw_files.len() != expected_raw.len() {
        return Err("CBO health local raw file count failed".to_string());
    }
    for file in raw_files {
        let role = string_field(file, "file_role")?;
        let (path, bytes, sha) = expected_raw
            .get(role.as_str())
            .ok_or("unexpected CBO health raw file role")?;
        let raw = root.join(path);
        if string_field(file, "raw_artifact_path")? != *path
            || int_field(file, "byte_count")? != *bytes
            || string_field(file, "sha256")? != *sha
            || !raw.exists()
            || fs::metadata(&raw).map_err(|err| err.to_string())?.len() != *bytes as u64
            || sha256_file(&raw)? != *sha
        {
            return Err(format!("CBO health raw file custody failed: {role}"));
        }
        if role == "official_spreadsheet" {
            let sheets = file
                .get("workbook_sheets")
                .and_then(serde_json::Value::as_array)
                .ok_or("CBO health workbook sheets")?;
            if sheets.len() != 2
                || !sheets.iter().any(|sheet| {
                    sheet.get("sheet_name").and_then(serde_json::Value::as_str)
                        == Some("healthinsuranceT1_02-2026")
                        && sheet.get("dimension").and_then(serde_json::Value::as_str)
                            == Some("A1:AA76")
                        && sheet.get("row_count").and_then(serde_json::Value::as_i64) == Some(76)
                })
                || !sheets.iter().any(|sheet| {
                    sheet.get("sheet_name").and_then(serde_json::Value::as_str)
                        == Some("healthinsuranceT2_02-2026")
                        && sheet.get("dimension").and_then(serde_json::Value::as_str)
                            == Some("A1:Y175")
                        && sheet.get("row_count").and_then(serde_json::Value::as_i64) == Some(175)
                })
            {
                return Err("CBO health workbook sheet custody failed".to_string());
            }
        }
    }

    let attempts = record
        .get("attempted_command_line_retrievals")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health attempted command-line retrievals")?;
    if attempts.len() != 8
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some(
                    "https://www.cbo.gov/system/files/2026-02/51298-2026-02-healthinsurance.xlsx",
                )
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("result").and_then(serde_json::Value::as_str)
                == Some("blocked_by_datadome_anti_bot_javascript_challenge")
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some("https://www.cbo.gov/publication/62539")
                && attempt.get("result").and_then(serde_json::Value::as_str)
                    == Some("blocked_by_403_or_login_redirect_from_command_line")
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some("https://www.cbo.gov/system/files/2026-07/62539-health-insurance.pdf")
                && attempt.get("result").and_then(serde_json::Value::as_str)
                    == Some("blocked_by_403_from_command_line")
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some("https://www.cbo.gov/publication/62380")
                && attempt.get("result").and_then(serde_json::Value::as_str)
                    == Some("browser_visible_command_line_page_blocked_or_unreliable")
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some(
                    "https://www.cbo.gov/system/files/2026-05/62380-Federal-Health-Subsidies.pdf",
                )
                && attempt.get("result").and_then(serde_json::Value::as_str)
                    == Some("browser_visible_pdf_command_line_raw_custody_blocked_by_403")
        })
        || !attempts.iter().any(|attempt| {
            attempt.get("url").and_then(serde_json::Value::as_str)
                == Some("https://www.cbo.gov/system/files/2026-05/62380-Data.xlsx")
                && attempt.get("result").and_then(serde_json::Value::as_str)
                    == Some("browser_visible_data_workbook_command_line_raw_custody_blocked_by_403")
        })
    {
        return Err("CBO health command-line attempt record failed".to_string());
    }

    let rows = record
        .get("browser_visible_numeric_rows_unassigned")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health unassigned rows")?;
    if rows.len() != 4 {
        return Err("CBO health unassigned row count failed".to_string());
    }
    for row in rows {
        if string_field(row, "table")? != "Table 2"
            || string_field(row, "row_label_assignment_status")? != "blocked_by_pdf_text_ordering"
            || string_field(row, "unit")? != "billions_of_dollars"
            || row
                .get("values")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|values| values.len() != 13)
        {
            return Err("CBO health unassigned row boundary failed".to_string());
        }
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("CBO health source boundary")?;
    for field in [
        "official_public_source_visible_in_browser",
        "local_raw_byte_custody_ready",
        "spreadsheet_custody_ready",
        "pdf_local_custody_ready",
        "may_2026_presentation_browser_context_ready",
        "row_labels_assigned_to_values",
        "not_solver_input",
        "not_rate_calculation",
        "not_balanced_budget_claim",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO health boundary {field} must be true"));
        }
    }
    for field in [
        "july_2026_latest_publication_raw_custody_ready",
        "may_2026_presentation_raw_custody_ready",
        "health_component_policy_score_ready",
        "current_law_health_solver_path_ready",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("CBO health boundary {field} must be false"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO health blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!("CBO health blocked output must be null: {field}"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO health claims")?;
    for (field, value) in claims {
        let observed = value.as_bool().ok_or("CBO health claim bool")?;
        if matches!(
            field.as_str(),
            "browser_visible_official_context_recorded"
                | "manual_february_2026_raw_custody_ready"
                | "source_custody_ready"
                | "local_raw_byte_custody_ready"
                | "may_2026_presentation_browser_context_ready"
        ) {
            if !observed {
                return Err(format!("CBO health claim should be true: {field}"));
            }
        } else if observed {
            return Err(format!("CBO health claim must be false: {field}"));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "local raw custody is captured",
        "May 11, 2026 CBO health-subsidy presentation",
        "May 2026 local raw byte custody remains blocked",
        "July 23, 2026 CBO Federal Subsidies for Health Insurance, 2026 to 2036",
        "July 2026 local raw byte custody remains blocked",
        "403 responses",
        "not a health federal policy score",
        "not solver input",
        "not a rate calculation",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!("CBO health warning missing phrase: {phrase}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        CBO_HEALTH_INSURANCE_BASELINE_BROWSER_CONTEXT_FY2026_2036_JSON_PATH,
        "local raw custody",
        "May 11, 2026 CBO presentation",
        "62380-Data.xlsx",
        "July 23, 2026 CBO publication",
        "PDF byte count: 747901",
        "spreadsheet byte count: 42861",
        "healthinsuranceT2_02-2026",
        "anti-bot/JavaScript challenge",
        "Datadome access boundary",
        "403 responses",
        "not a health federal policy score",
        "not solver input",
        "not a rate calculation",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!("CBO health reader missing phrase: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_cbo_health_insurance_table2_browser_rowmap_fy2026_2036(
    root: &Path,
) -> Result<(), String> {
    for path in [
        CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH,
        CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CBO health Table 2 rowmap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "cbo-health-insurance-table2-browser-rowmap-fy2026-2036:v1"
        || string_field(&record, "record_family")? != "cbo_health_insurance_table2_browser_rowmap"
        || string_field(&record, "status")? != "draft_spreadsheet_supported_rowmap_solver_blocked"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "as_of_date")? != "2026-07-24"
    {
        return Err("CBO health Table 2 rowmap identity failed".to_string());
    }

    let source = record
        .get("source_context")
        .ok_or("CBO health Table 2 source context")?;
    if string_field(source, "publisher")? != "Congressional Budget Office"
        || string_field(source, "selected_programs_page_url")?
            != "https://www.cbo.gov/data/baseline-projections-selected-programs"
        || string_field(source, "pdf_url")?
            != "https://www.cbo.gov/system/files/2026-02/51298-2026-02-healthinsurance.pdf"
        || !string_field(source, "table")?.contains("Net Federal Subsidies")
        || string_field(source, "unit")? != "billions_of_dollars_by_fiscal_year"
        || string_field(source, "visual_review_page")? != "PDF page 4 of 5"
        || string_field(source, "browser_review_date")? != "2026-07-24"
        || source
            .get("local_raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || string_field(source, "raw_artifact_path")?
            != "data/raw/cbo/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02/2026-07-24/51298-2026-02-healthinsurance.xlsx"
        || int_field(source, "byte_count")? != 42_861
        || string_field(source, "sha256")?
            != "f2d7cc186f3a0afa909e648f8224a1f7f80af202db234848b507fd49416e1001"
        || string_field(source, "metadata_path")?
            != "data/metadata/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02.2026-07-24.metadata.md"
    {
        return Err("CBO health Table 2 source boundary failed".to_string());
    }
    for path in [
        string_field(source, "raw_artifact_path")?,
        string_field(source, "metadata_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!("CBO health Table 2 source path missing: {path}"));
        }
    }

    let columns = record
        .get("columns")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health Table 2 columns")?;
    if columns.len() != 13
        || columns.first().and_then(serde_json::Value::as_str) != Some("2025")
        || columns.last().and_then(serde_json::Value::as_str) != Some("2027_2036_total")
    {
        return Err("CBO health Table 2 columns failed".to_string());
    }

    let rows = record
        .get("browser_verified_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CBO health Table 2 rows")?;
    if rows.len() != 21 {
        return Err("CBO health Table 2 row count failed".to_string());
    }
    let expected_rows = [
        (
            "employment_based_coverage.subtotal",
            ("Subtotal, Employment-Based Coverage", "n.a.", 478, 6687),
        ),
        (
            "medicaid_chip.subtotal",
            ("Subtotal, Medicaid and CHIP", "629", 668, 7691),
        ),
        ("medicare", ("Medicare", "977", 1051, 15015)),
        (
            "premium_tax_credit_related.subtotal",
            (
                "Subtotal, Premium Tax Credit and Related Spending",
                "n.a.",
                117,
                1110,
            ),
        ),
        ("net_subsidies", ("Net Subsidies", "n.a.", 2365, 31185)),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    for row in rows {
        let row_id = string_field(row, "row_id")?;
        let values = row
            .get("values")
            .and_then(serde_json::Value::as_array)
            .ok_or("CBO health Table 2 row values")?;
        if string_field(row, "label")?.is_empty() || values.len() != 13 {
            return Err(format!("CBO health Table 2 row malformed: {row_id}"));
        }
        if let Some((label, first_value, value_2026, total)) = expected_rows.get(row_id.as_str()) {
            let observed_first = values[0]
                .as_str()
                .map(str::to_string)
                .or_else(|| values[0].as_i64().map(|value| value.to_string()))
                .ok_or("CBO health Table 2 first value")?;
            if string_field(row, "label")? != *label
                || observed_first != *first_value
                || values[1].as_i64() != Some(*value_2026)
                || values[12].as_i64() != Some(*total)
            {
                return Err(format!(
                    "CBO health Table 2 expected row values failed: {row_id}"
                ));
            }
        }
    }

    let boundary = record
        .get("rowmap_boundary")
        .ok_or("CBO health Table 2 rowmap boundary")?;
    for field in [
        "browser_visual_row_order_assigned",
        "local_raw_byte_custody_ready",
        "spreadsheet_custody_ready",
        "pdf_local_custody_ready",
        "not_rate_calculation",
        "not_public_rate_card",
        "not_savings_estimate",
        "not_balanced_budget_claim",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CBO health Table 2 boundary {field} must be true"));
        }
    }
    for field in [
        "may_populate_current_law_health_path",
        "may_populate_policy_score",
        "may_populate_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("CBO health Table 2 boundary {field} must be false"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CBO health Table 2 blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "CBO health Table 2 blocked output must be null: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "row order is assigned",
        "supported by local raw custody",
        "not a current-law health solver path",
        "not solver input",
        "not a rate calculation",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!("CBO health Table 2 warning missing: {phrase}"));
        }
    }

    let reader = fs::read_to_string(
        root.join(CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        CBO_HEALTH_INSURANCE_TABLE2_BROWSER_ROWMAP_FY2026_2036_JSON_PATH,
        "row map",
        "PDF page 4 of 5",
        "assigns 21 browser-visible rows",
        "local raw byte custody is ready",
        "not a current-law health solver path",
        "federal policy",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!("CBO health Table 2 reader missing: {phrase}"));
        }
    }

    Ok(())
}

