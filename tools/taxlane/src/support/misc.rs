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

pub(crate) fn outlay_function_notes(key: &str) -> &'static str {
    match key {
        "net-interest" => "Net interest is kept visible as its own outlay function.",
        "undistributed-offsetting-receipts" => {
            "Undistributed offsetting receipts are kept visible and negative as reported by OMB."
        }
        _ => "Broad Table 3.1 outlay function; no lane allocation applied yet.",
    }
}

pub(crate) fn outlay_function_3_1_jsonl(rows: &[OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let source_ids = if row.include_table_1_1_source {
            "\"SRC-OMB-HIST-3-1-FY2027\",\"SRC-OMB-HIST-1-1-FY2027\""
        } else {
            "\"SRC-OMB-HIST-3-1-FY2027\""
        };
        let reconciliation = row.table_1_1_row.map_or_else(String::new, |table_1_1_row| {
            format!("; reconciled to Table 1.1 row {table_1_1_row}")
        });
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":null,\"subfunction_label\":null,\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":{},\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:total:outlays",
                row.fiscal_year, row.function_code
            )),
            row.fiscal_year,
            source_ids,
            json_string("OMB Historical Table 3.1 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}{}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.function_label,
                reconciliation
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_amount(row.amount),
            json_string(row.actual_or_projection),
            json_string(row.offsetting_treatment),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn outlay_function_3_1_profile_markdown(profile: &OutlayFunctionProfile) -> String {
    let sample_years = [1940, 1950, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.1 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-1-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.push("| `total-federal-outlays` | Total, Federal outlays | 35 |".to_string());
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Broad category total is the sum of the six visible Table 3.1 rows above.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 total | Broad category total | Table total diff | Broad category diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_1_1_outlays, 0),
            comma_number(check.table_3_1_total, 0),
            comma_number(check.broad_category_total, 0),
            comma_number(check.total_difference, 0),
            comma_number(check.broad_category_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Net interest is extracted as its own visible outlay function.".to_string(),
        "- Undistributed offsetting receipts are extracted as negative amounts with `offsetting_treatment = \"undistributed-offsetting-receipts\"`.".to_string(),
        "- Function codes are TAXLANE slugs because Table 3.1 uses labels, not OMB numeric function codes.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn table_3_2_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Result<f64, String> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
        .ok_or_else(|| format!("missing Table 3.2 amount at {column}{row_num}"))
}

pub(crate) fn table_3_2_national_defense_jsonl(rows: &[Table32OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":\"net\",\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(row.function_code),
            json_string(row.function_label),
            json_option_string(row.subfunction_code),
            json_option_string(row.subfunction_label),
            json_amount(row.amount),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn table_3_2_national_defense_profile_markdown(profile: &Table32NationalDefenseProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 National Defense Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | Subfunction code | Source label | Table 3.2 row |".to_string(),
        "|---|---|---|---:|".to_string(),
    ];
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        lines.push(format!(
            "| `050` | {} | {} | {} |",
            line.subfunction_code
                .map(|code| format!("`{code}`"))
                .unwrap_or_else(|| "`null`".to_string()),
            line.source_label,
            line.source_row
        ));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Subfunction total is rows 13, 14, and 15.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 National Defense | Table 3.2 National Defense | Subfunction total | Table 3.1 diff | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_national_defense, 0),
            comma_number(check.table_3_2_national_defense, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.subfunction_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- This is a proof slice for function `050 National Defense`, not the full Table 3.2 extraction.".to_string(),
        "- Rows 6-12 are lower component rows inside subfunction `051`; this proof emits row 13 as the subfunction total instead.".to_string(),
        "- Parent total row 16 is emitted with `subfunction_code = null` so it can reconcile to Table 3.1.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn table_6_1_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    for cells in rows.values() {
        let mut columns = BTreeMap::new();
        for (column, value) in cells {
            let year = match value {
                CellValue::Number(number) if number.fract() == 0.0 => Some(*number as i64),
                CellValue::Text(text) => text.trim().parse::<i64>().ok(),
                _ => None,
            };
            if let Some(year) = year.filter(|year| (1940..=2031).contains(year)) {
                columns.insert(year, column.clone());
            }
        }
        if columns.contains_key(&1940) && columns.contains_key(&2025) {
            return Ok(columns);
        }
    }
    Err("missing Table 6.1 year header row (1940..2025)".to_string())
}

pub(crate) fn table_6_1_section_row(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    label: &str,
) -> Result<i64, String> {
    rows.iter()
        .find(|(_, cells)| text_cell(cells.get("A")).as_deref() == Some(label))
        .map(|(row_num, _)| *row_num)
        .ok_or_else(|| format!("missing Table 6.1 section {label:?}"))
}

pub(crate) fn table_6_1_label_row_between(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    label: &str,
    after_row: i64,
    before_row: i64,
) -> Result<i64, String> {
    rows.iter()
        .filter(|(row_num, _)| **row_num > after_row && **row_num < before_row)
        .find(|(_, cells)| text_cell(cells.get("A")).as_deref() == Some(label))
        .map(|(row_num, _)| *row_num)
        .ok_or_else(|| {
            format!("missing Table 6.1 row {label:?} between rows {after_row} and {before_row}")
        })
}

pub(crate) fn table_6_1_national_defense_jsonl(rows: &[Table61NationalDefenseRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_composition\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-6-1-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"function_code\":\"050\",\"function_label\":\"National Defense\",\"measure\":\"percent_of_gdp\",\"percent\":{},\"amount\":null,\"amount_units\":\"percent_of_gdp\",\"actual_or_projection\":\"actual\",\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-composition:{}:050:percent-of-gdp",
                row.fiscal_year
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 6.1 FY2027"),
            json_string(&format!(
                "Table!{}{}; National defense (1) (As percentages of GDP)",
                row.source_column, row.source_row
            )),
            json_amount(row.percent_of_gdp),
            json_string(OBSERVED_DATE_6_1),
            json_string(
                "National defense outlays as a percentage of GDP (OMB budget-function 050 basis); actual years only."
            ),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn table_6_1_national_defense_profile_markdown(profile: &Table61NationalDefenseProfile) -> String {
    let mut lines = vec![
        "# Table 6.1 National Defense (% of GDP) Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Source: `SRC-OMB-HIST-6-1-FY2027` (Composition of Outlays).".to_string(),
        "- Series: national-defense outlays as a percentage of GDP (OMB budget-function 050 basis).".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Sample Years".to_string(),
        String::new(),
        "| Fiscal year | National defense, % of GDP | Total outlays, % of GDP |".to_string(),
        "|---:|---:|---:|".to_string(),
    ];
    for (year, defense, total) in &profile.samples {
        lines.push(format!(
            "| {} | {} | {} |",
            year,
            comma_number(*defense, 1),
            comma_number(*total, 1)
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- This is the national-defense (function 050) row of OMB Table 6.1's \"As percentages of GDP\" section.".to_string(),
        "- It is the OMB budget-function basis, not the SIPRI/NATO definition; the two series are not merged.".to_string(),
        "- Values are OMB-reported to one decimal place.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn is_table_3_2_function_code(code: &str) -> bool {
    matches!(
        code,
        "050"
            | "150"
            | "250"
            | "270"
            | "300"
            | "350"
            | "370"
            | "400"
            | "450"
            | "500"
            | "550"
            | "570"
            | "600"
            | "650"
            | "700"
            | "750"
            | "800"
            | "900"
            | "920"
            | "950"
    )
}

pub(crate) fn table_3_2_optional_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Option<f64> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
}

pub(crate) fn table_3_2_jsonl(rows: &[Table32Row]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.as_deref().unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_owned_option_string(row.subfunction_code.as_ref()),
            json_owned_option_string(row.subfunction_label.as_ref()),
            json_amount(row.amount),
            json_string(table_3_2_offsetting_treatment(row)),
            json_string(OBSERVED_DATE),
            json_string(table_3_2_notes(row)),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn table_3_2_offsetting_treatment(row: &Table32Row) -> &'static str {
    if row.function_code == "950" {
        "undistributed-offsetting-receipts"
    } else if row.subfunction_code.as_deref() == Some("809") {
        "offsetting-receipts"
    } else {
        "net"
    }
}

pub(crate) fn table_3_2_notes(row: &Table32Row) -> &'static str {
    match row.kind {
        Table32LineKind::Subfunction => {
            "Table 3.2 subfunction row; lower component rows and parenthetical on/off-budget splits are not emitted."
        }
        Table32LineKind::FunctionTotal => {
            "Table 3.2 parent function total used for subfunction reconciliation."
        }
        Table32LineKind::GrandTotal => {
            "Table 3.2 total outlays reconciled to OMB Historical Table 3.1 total outlays."
        }
    }
}

pub(crate) fn table_3_2_profile_markdown(profile: &Table32Profile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        format!("- Source lines emitted: {}", profile.line_count),
        format!("- Function count: {}", profile.function_count),
        format!("- Subfunction lines: {}", profile.subfunction_line_count),
        format!(
            "- Explicit function-total lines: {}",
            profile.function_total_line_count
        ),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Function total sum uses explicit parent totals when Table 3.2 provides them, otherwise the emitted subfunction total.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 total outlays | Table 3.2 total outlays | Function total sum | Table 3.1 diff | Function total diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .grand_checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_total_outlays, 0),
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.function_total_sum, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.function_total_difference, 0),
        ));
    }
    if let Some(check) = profile.function_checks.iter().max_by(|left, right| {
        left.difference
            .abs()
            .partial_cmp(&right.difference.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        lines.extend([
            String::new(),
            "## Function Reconciliation Note".to_string(),
            String::new(),
            format!(
                "Largest displayed-source function subtotal difference: FY{} `{}` {} has subfunction total {} versus parent total {}, difference {}.",
                check.year,
                check.function_code,
                check.function_label,
                comma_number(check.subfunction_total, 0),
                comma_number(check.function_total, 0),
                comma_number(check.difference, 0),
            ),
        ]);
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Emit three-digit coded subfunction rows and explicit parent `Total, ...` rows.".to_string(),
        "- Emit `Total outlays` as a grand-total record for annual reconciliation.".to_string(),
        "- Skip lower component rows without OMB subfunction codes, including parenthetical on/off-budget splits.".to_string(),
        "- Keep TQ and FY2026-FY2031 estimate columns out of this actual-year draft.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn subfunction_model_jsonl(records: &[SubfunctionModelRow]) -> String {
    let mut lines = Vec::new();
    for row in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_subfunction_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-2-1-FY2027\",\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table_refs\":{{\"tax_receipts\":\"OMB Historical Table 2.1 FY2027\",\"outlay_subfunction\":{}}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"subfunction_outlays_amount\":{},\"total_outlays_amount\":{},\"subfunction_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by Table 3.2 subfunction outlay share; not legal dedication or program tracing.\"}}",
            json_string(&format!(
                "income-tax-outlay-subfunction-model:{}:{}:{}",
                row.fiscal_year, row.function_code, row.subfunction_code
            )),
            json_string(SUBFUNCTION_MODEL_ID),
            row.fiscal_year,
            json_string(&format!(
                "OMB Historical Table 3.2 FY2027 row {}, column {}",
                row.source_row, row.source_column
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_string(&row.subfunction_code),
            json_string(&row.subfunction_label),
            json_amount(row.subfunction_outlays_amount),
            json_amount(row.total_outlays_amount),
            json_amount(row.subfunction_total_outlays_amount),
            json_amount(row.individual_income_tax_receipts_amount),
            decimal_string(row.outlay_share_percent, 9),
            decimal_string(row.allocation_share_percent, 9),
            decimal_string(row.modeled_income_tax_allocation_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn subfunction_model_profile_markdown(profile: &SubfunctionModelProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Subfunction Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{SUBFUNCTION_MODEL_ID}`"),
        "- Tax receipt source: `SRC-OMB-HIST-2-1-FY2027`".to_string(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Subfunction count: {}", profile.subfunction_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. `Subfunction total` is the denominator used for modeled allocation.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.2 total outlays | Subfunction total | Income tax receipts | Modeled sum | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.individual_income_tax, 0),
            comma_number(check.modeled_sum, 3),
            comma_number(check.subfunction_total_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "This is a visibility model. It allocates ordinary individual income-tax receipts by reported Table 3.2 subfunction outlay shares. It is not a legal dedication, appropriation rule, or program-financing claim.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn subfunction_model_readme_markdown() -> String {
    [
        "# Individual Income-Tax Outlay Subfunction Model",
        "",
        "## Purpose",
        "",
        "This derived model estimates, by fiscal year and OMB Table 3.2 subfunction, how ordinary individual income-tax receipts would be allocated if allocated in proportion to that year's reported subfunction outlays.",
        "",
        "This is a visibility model. It is not a legal dedication, appropriation rule, or program-financing claim.",
        "",
        "## Model ID",
        "",
        "`individual-income-tax-proportional-subfunction-outlays-v1`",
        "",
        "## Inputs",
        "",
        "| Source ID | Role |",
        "|---|---|",
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount by fiscal year. |",
        "| `SRC-OMB-HIST-3-2-FY2027` | Function and subfunction outlays by fiscal year. |",
        "",
        "## Coverage",
        "",
        "The first draft model covers fiscal years 1962-2025, the overlap between Table 3.2 actual-year subfunction rows and Table 2.1 individual income-tax receipt rows.",
        "",
        "## Artifacts",
        "",
        "| Artifact | Role |",
        "|---|---|",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual modeled allocation rows by Table 3.2 subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready long CSV view with one row per fiscal year and subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade rollup by subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 ranked view for the largest modeled subfunction allocations. |",
        "",
        "## Method",
        "",
        "For each fiscal year and emitted Table 3.2 subfunction:",
        "",
        "```text",
        "outlay_share_percent = subfunction_outlays / total_federal_outlays * 100",
        "allocation_share_percent = subfunction_outlays / sum_of_subfunction_outlays * 100",
        "modeled_income_tax_allocation = individual_income_tax_receipts",
        "                                * subfunction_outlays",
        "                                / sum_of_subfunction_outlays",
        "```",
        "",
        "The allocation denominator uses the emitted subfunction rows so modeled rows sum back to individual income-tax receipts. Small differences from displayed total outlays are source rounding.",
        "",
        "## Decade Rollup Caveat",
        "",
        "The decade-long CSV sums modeled allocation dollars within each decade and then calculates each subfunction's share of that decade total. It is not an average of annual percentages or annual ranks.",
        "",
        "The 1960s bucket is partial because subfunction actual-year coverage starts in FY1962. The 2020s bucket is partial because the actual-year model currently ends in FY2025.",
        "",
        "## Regeneration",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check",
        "```",
        "",
        "## Validation Command",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay validate",
        "```",
        "",
    ]
    .join("\n")
}

pub(crate) fn table_2_2_year_label(value: Option<&CellValue>) -> Option<String> {
    text_cell(value).or_else(|| int_cell(value).map(|year| year.to_string()))
}

pub(crate) fn receipt_share_sort_key(category: &str) -> usize {
    RECEIPT_SHARE_CATEGORIES
        .iter()
        .position(|candidate| candidate.receipt_category == category)
        .unwrap_or(usize::MAX)
}

pub(crate) fn receipt_share_jsonl(rows: &[ReceiptShareRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let mut source_ids = vec!["SRC-OMB-HIST-2-2-FY2027"];
        if row.receipt_category == "individual-income-taxes" {
            source_ids.push("SRC-OMB-AP-13-FUNDS-FY2027");
        }
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"receipt_source\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":\"OMB Historical Table 2.2 FY2027\",\"source_row_ref\":{},\"receipt_category\":{},\"source_receipt_label\":{},\"measure\":\"share_of_total\",\"amount\":null,\"percent\":{},\"amount_units\":\"percent\",\"actual_or_projection\":{},\"fund_group_link\":null,\"allocation_status\":{},\"status\":\"draft\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "receipt:{}:{}:share-of-total",
                row.fiscal_year, row.receipt_category
            )),
            row.fiscal_year,
            source_ids
                .iter()
                .map(|source| json_string(source))
                .collect::<Vec<_>>()
                .join(","),
            json_string(&format!(
                "Table!A{}:{}{}; column {} {}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.source_column,
                row.source_receipt_label
            )),
            json_string(row.receipt_category),
            json_string(row.source_receipt_label),
            decimal_string(row.percent, 6),
            json_string(row.actual_or_projection),
            json_string(row.allocation_status),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn receipt_share_profile_markdown(rows: &[ReceiptShareRow]) -> Result<String, String> {
    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let year_count = rows.len() / RECEIPT_SHARE_CATEGORIES.len();
    let estimate_count = rows
        .iter()
        .filter(|row| row.actual_or_projection == "estimate")
        .map(|row| row.fiscal_year)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let sample_years = [1934, 1940, 1980, 2000, 2025, 2031];
    let mut by_year: BTreeMap<i64, BTreeMap<&str, f64>> = BTreeMap::new();
    for row in rows {
        by_year
            .entry(row.fiscal_year)
            .or_default()
            .insert(row.receipt_category, row.percent);
    }

    let mut lines = vec![
        "# OMB Table 2.2 Receipt Share Profile".to_string(),
        String::new(),
        "## Source".to_string(),
        String::new(),
        "- Source ID: `SRC-OMB-HIST-2-2-FY2027`".to_string(),
        "- Raw artifact: `data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx`"
            .to_string(),
        "- Table title: `Table 2.2 - PERCENTAGE COMPOSITION OF RECEIPTS BY SOURCE: 1934 - 2031`"
            .to_string(),
        String::new(),
        "## Coverage".to_string(),
        String::new(),
        format!("- Fiscal years emitted: {first_year}-{last_year}"),
        format!("- Year count: {year_count}"),
        format!("- Estimate years: {estimate_count}"),
        format!("- Record count: {}", rows.len()),
        String::new(),
        "## Extracted Columns".to_string(),
        String::new(),
        "| Column | Receipt category | Source label |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for category in RECEIPT_SHARE_CATEGORIES {
        lines.push(format!(
            "| {} | `{}` | {} |",
            category.column, category.receipt_category, category.source_receipt_label
        ));
    }
    lines.extend([
        String::new(),
        "## Sample Shares".to_string(),
        String::new(),
        "Percentages are OMB-reported shares of total receipts.".to_string(),
        String::new(),
        "| Fiscal year | Individual income | Corporation income | Social insurance | Excise | Other | Total receipts |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for year in sample_years {
        let categories = by_year
            .get(&year)
            .ok_or_else(|| format!("missing sample year {year}"))?;
        lines.push(format!(
            "| {year} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            categories["individual-income-taxes"],
            categories["corporation-income-taxes"],
            categories["social-insurance-and-retirement-receipts"],
            categories["excise-taxes"],
            categories["other-receipts"],
            categories["total-receipts"],
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Keep Table 2.2 percentage rows separate from Table 2.1 amount rows.".to_string(),
        "- Skip the transition-quarter `TQ` row because it is not a fiscal year.".to_string(),
        "- Preserve estimate years as `actual_or_projection = \"estimate\"`.".to_string(),
        "- Treat total receipts as `mixed` because it combines categories with different budget treatment.".to_string(),
        "- Keep non-individual receipt allocation labels as `unknown` pending narrower review.".to_string(),
        String::new(),
    ]);
    Ok(lines.join("\n"))
}

pub(crate) fn cell_column(reference: &str) -> String {
    reference
        .chars()
        .take_while(|char| char.is_ascii_alphabetic())
        .collect()
}

pub(crate) fn cell_value(raw: &str) -> Option<CellValue> {
    let value = raw.trim();
    if value.is_empty() || value == ".........." {
        return None;
    }
    if value == "-*" {
        return Some(CellValue::Number(0.0));
    }
    match value.parse::<f64>() {
        Ok(number) => Some(CellValue::Number(number)),
        Err(_) => Some(CellValue::Text(value.to_string())),
    }
}

pub(crate) fn table_3_1_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

pub(crate) fn table_3_2_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&3)
        .ok_or_else(|| "missing Table 3.2 header row 3".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        let year = match value {
            CellValue::Number(number) if number.fract() == 0.0 => Some(*number as i64),
            CellValue::Text(text) => parse_table_3_2_year(text),
            _ => None,
        };
        if let Some(year) = year {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

pub(crate) fn int_cell(value: Option<&CellValue>) -> Option<i64> {
    match value {
        Some(CellValue::Number(number)) if number.fract() == 0.0 => Some(*number as i64),
        _ => None,
    }
}

pub(crate) fn number_cell(value: Option<&CellValue>) -> Option<f64> {
    match value {
        Some(CellValue::Number(number)) => Some(*number),
        _ => None,
    }
}

pub(crate) fn text_cell(value: Option<&CellValue>) -> Option<String> {
    match value {
        Some(CellValue::Text(text)) => Some(text.clone()),
        _ => None,
    }
}

pub(crate) fn annual_model_jsonl(records: &[AnnualRecord]) -> String {
    let mut lines = Vec::new();
    for record in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table_refs\":{{\"fiscal_spine\":{},\"tax_receipts\":{},\"outlay_category\":{},\"outlay_total\":\"OMB Historical Table 3.1 FY2027 row 35\"}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"category_key\":{},\"category_label\":{},\"category_outlays_amount\":{},\"total_outlays_amount\":{},\"category_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"total_receipts_amount\":{},\"surplus_or_deficit_amount\":{},\"deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"category_total_reconciliation_difference_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by broad Table 3.1 outlay share, normalized over displayed broad-category rows to handle source rounding; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}", record.fiscal_year, record.category_key)),
            json_string(MODEL_ID),
            record.fiscal_year,
            SOURCE_IDS.iter().map(|source| json_string(source)).collect::<Vec<_>>().join(","),
            json_string(&format!("OMB Historical Table 1.1 FY2027 row {}", record.table_11_row)),
            json_string(&format!("OMB Historical Table 2.1 FY2027 row {}, column B", record.table_21_row)),
            json_string(&format!("OMB Historical Table 3.1 FY2027 row {}", record.table_31_row)),
            json_string(record.category_key),
            json_string(record.category_label),
            decimal_string(record.category_outlays_amount, 6),
            decimal_string(record.total_outlays_amount, 6),
            decimal_string(record.category_total_outlays_amount, 6),
            decimal_string(record.individual_income_tax_receipts_amount, 6),
            decimal_string(record.outlay_share_percent, 9),
            decimal_string(record.allocation_share_percent, 9),
            decimal_string(record.modeled_income_tax_allocation_amount, 6),
            decimal_string(record.total_receipts_amount, 6),
            decimal_string(record.surplus_or_deficit_amount, 6),
            annual_deficit_gap_string(record.deficit_gap_amount),
            decimal_string(record.borrowed_share_percent_of_outlays, 9),
            decimal_string(record.income_tax_coverage_percent_of_outlays, 9),
            decimal_string(record.category_total_reconciliation_difference_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn source_profile_markdown(profile: &AnnualProfile) -> String {
    let sample_years = [1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{MODEL_ID}`"),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Source Roles".to_string(),
        String::new(),
        "| Source ID | Use |".to_string(),
        "|---|---|".to_string(),
        "| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit. |"
            .to_string(),
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipts. |".to_string(),
        "| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays. |"
            .to_string(),
        String::new(),
        "## Broad Categories".to_string(),
        String::new(),
        "| Category key | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "All amounts are in millions of dollars. `Modeled sum` is the sum of".to_string(),
        "the six category allocation rows for the fiscal year.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 outlays | Category total | Income tax receipts | Modeled sum | Deficit gap |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for row in profile
        .annual_checks
        .iter()
        .filter(|row| sample_years.contains(&row.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.year,
            comma_number(row.table_1_1_outlays, 0),
            comma_number(row.table_3_1_outlays, 0),
            comma_number(row.category_total, 0),
            comma_number(row.income_tax, 0),
            comma_number(row.modeled_sum, 3),
            comma_number(row.deficit_gap, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "These rows allocate individual income-tax receipts by reported outlay".to_string(),
        "share, normalized over the displayed broad-category rows when source".to_string(),
        "rounding creates a small difference from the displayed total. They do".to_string(),
        "not claim that income-tax dollars were legally dedicated to the listed".to_string(),
        "outlay categories.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

pub(crate) fn decade_summary_jsonl(rows: &[DecadeSummaryRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model_decade_summary\",\"source_record_family\":\"income_tax_outlay_model\",\"model_id\":\"individual-income-tax-proportional-outlays-v1\",\"decade\":{},\"start_fiscal_year\":{},\"end_fiscal_year\":{},\"year_count\":{},\"coverage_note\":{},\"category_key\":{},\"category_label\":{},\"cumulative_modeled_income_tax_allocation_amount\":{},\"cumulative_individual_income_tax_receipts_amount\":{},\"category_percent_of_decade_income_tax\":{},\"cumulative_total_outlays_amount\":{},\"cumulative_total_receipts_amount\":{},\"cumulative_deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"notes\":\"Decade summary derived from annual modeled allocation rows; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}:decade-summary", row.decade, row.category_key)),
            json_string(&row.decade),
            row.start_fiscal_year,
            row.end_fiscal_year,
            row.year_count,
            json_string(row.coverage_note),
            json_string(&row.category_key),
            json_string(&row.category_label),
            decimal_string(row.cumulative_modeled_income_tax_allocation_amount, 6),
            decimal_string(row.cumulative_individual_income_tax_receipts_amount, 6),
            decimal_string(row.category_percent_of_decade_income_tax, 9),
            decimal_string(row.cumulative_total_outlays_amount, 6),
            decimal_string(row.cumulative_total_receipts_amount, 6),
            decimal_string(row.cumulative_deficit_gap_amount, 6),
            decimal_string(row.borrowed_share_percent_of_outlays, 9),
            decimal_string(row.income_tax_coverage_percent_of_outlays, 9),
        ));
    }
    lines.join("\n") + "\n"
}

pub(crate) fn decade_summary_markdown(rows: &[DecadeSummaryRow]) -> Result<String, String> {
    let mut by_decade: BTreeMap<&str, BTreeMap<&str, &DecadeSummaryRow>> = BTreeMap::new();
    for row in rows {
        by_decade
            .entry(&row.decade)
            .or_default()
            .insert(&row.category_key, row);
    }

    let mut lines = vec![
        "# Decade Summary: Modeled Income-Tax Outlay Allocation".to_string(),
        String::new(),
        "This table summarizes the annual draft model by decade. Category".to_string(),
        "percentages equal cumulative modeled category allocations divided by".to_string(),
        "cumulative individual income-tax receipts for the years in that decade.".to_string(),
        "The 2020s are partial because the current actual-year model ends in 2025.".to_string(),
        String::new(),
        "These are modeled allocations, not legal destinations for income-tax".to_string(),
        "receipts.".to_string(),
        String::new(),
        "| Decade | Years | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share of outlays | Income-tax coverage of outlays |".to_string(),
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ];

    for (decade, categories) in by_decade {
        let first = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let values: Vec<f64> = CATEGORY_FIELDS
            .iter()
            .map(|(category, _)| {
                categories
                    .get(category)
                    .map(|row| row.category_percent_of_decade_income_tax)
                    .ok_or_else(|| format!("{decade}: missing {category} row"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        lines.push(format!(
            "| {} | {}-{} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            decade,
            first.start_fiscal_year,
            first.end_fiscal_year,
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            first.borrowed_share_percent_of_outlays,
            first.income_tax_coverage_percent_of_outlays
        ));
    }
    Ok(lines.join("\n") + "\n")
}

pub(crate) fn export_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_annual_csv_rows(root)?;
    let decade = build_decade_csv_rows(root)?;
    validate_csv_rows(&annual, "annual", 86)?;
    validate_csv_rows(&decade, "decade", 9)?;

    if check_only {
        compare_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        compare_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    } else {
        write_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        write_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    }

    println!(
        "validated {} annual rows and {} decade rows",
        annual.len(),
        decade.len()
    );
    Ok(())
}

pub(crate) fn export_subfunction_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_subfunction_annual_csv_rows(root)?;
    let decade = build_subfunction_decade_csv_rows(root)?;
    let top = build_subfunction_fy2025_top_csv_rows(root, 25)?;
    validate_subfunction_csv_rows(&annual, "subfunction annual", 4691)?;
    validate_subfunction_decade_csv_rows(&decade)?;
    validate_subfunction_csv_rows(&top, "subfunction FY2025 top", 25)?;

    if check_only {
        compare_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    } else {
        write_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        write_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        write_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    }

    println!(
        "validated {} subfunction annual rows, {} decade rows, and {} FY2025 top rows",
        annual.len(),
        decade.len(),
        top.len()
    );
    Ok(())
}

pub(crate) fn subfunction_annual_csv_row(row: &serde_json::Value) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_json_number(
        &mut output,
        "individual_income_tax_receipts_millions",
        row,
        "individual_income_tax_receipts_amount",
    );
    insert_json_number(
        &mut output,
        "total_outlays_millions",
        row,
        "total_outlays_amount",
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_number(
        &mut output,
        "outlay_share_percent",
        number_field(row, "outlay_share_percent")?,
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
    output.insert(
        "actual_or_projection".to_string(),
        string_field(row, "actual_or_projection")?,
    );
    Ok(output)
}

pub(crate) fn subfunction_top_csv_row(
    rank: usize,
    row: &serde_json::Value,
) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert("rank".to_string(), rank.to_string());
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
    Ok(output)
}

pub(crate) fn compare_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let expected = normalize_newlines(&csv_text(headers, rows)?);
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale CSV export: run `cargo run -p taxlane-tools -- income-tax-outlay export`"
        ));
    }
    Ok(())
}

pub(crate) fn compare_text(
    root: &Path,
    relative_path: &str,
    expected: &str,
    label: &str,
) -> Result<(), String> {
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != normalize_newlines(expected) {
        return Err(format!("stale {label}"));
    }
    Ok(())
}

pub(crate) fn csv_text(headers: &[&str], rows: &[BTreeMap<String, String>]) -> Result<String, String> {
    if rows.is_empty() {
        return Err("no CSV rows".to_string());
    }
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record(headers.iter().copied())
        .map_err(|err| format!("failed to write CSV header: {err}"))?;
    for row in rows {
        let values: Vec<&str> = headers
            .iter()
            .map(|header| row.get(*header).map(String::as_str).unwrap_or(""))
            .collect();
        writer
            .write_record(values)
            .map_err(|err| format!("failed to write CSV row: {err}"))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|err| format!("failed to finish CSV: {err}"))?;
    String::from_utf8(bytes).map_err(|err| format!("invalid UTF-8 CSV: {err}"))
}

pub(crate) fn generated_accountability_performance_demand_response_status(
    root: &Path,
) -> Result<PerformanceDemandResponseStatus, String> {
    let status_text = build_accountability_performance_demand_response_status(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse generated response status: {err}"))?;
    status.validate()?;
    Ok(status)
}

pub(crate) fn performance_demand_response_bundle_artifacts(
    root: &Path,
) -> Result<Vec<PerformanceDemandResponseBundleArtifact>, String> {
    let rows = [
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH,
            "Source-custodied intake fixture row.",
            "jsonl",
            "Exercise importer parsing and record-id matching.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
            "Response-log rows after applying example intake.",
            "jsonl",
            "Inspect typed applied rows without changing canonical response status.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH,
            "Compact applied response counts.",
            "json",
            "Feed fixture counts into UI/API tests without recomputing rows.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH,
            "Human-readable applied response counts.",
            "markdown",
            "Scan importer behavior without opening JSON.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH,
            "Task routing for the applied fixture set.",
            "markdown",
            "Choose the right applied artifact by implementation task.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH,
            "Fixture artifact contract.",
            "markdown",
            "Confirm roles and guardrails for applied importer artifacts.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH,
            "Human-readable changed fields.",
            "markdown",
            "Inspect row-level changes after applying example intake.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH,
            "Machine-readable changed fields.",
            "jsonl",
            "Feed delta rows into UI/API diff consumers.",
        ),
        (
            ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH,
            "Delta row field contract.",
            "markdown",
            "Confirm field meanings and blocked-claim guardrails.",
        ),
    ];

    rows.into_iter()
        .map(|(artifact, role, kind, consumer_use)| {
            let path = root.join(artifact);
            Ok(PerformanceDemandResponseBundleArtifact {
                artifact: artifact.to_string(),
                role: role.to_string(),
                kind: kind.to_string(),
                row_count: count_rows(&path, kind)?,
                sha256: sha256_file(&path)?,
                consumer_use: consumer_use.to_string(),
            })
        })
        .collect()
}

pub(crate) fn bool_marker(changed: bool) -> &'static str {
    if changed { "changed" } else { "unchanged" }
}

pub(crate) fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

pub(crate) fn int_field(row: &serde_json::Value, field: &str) -> Result<i64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_i64)
        .ok_or_else(|| format!("missing integer field {field}"))
}

pub(crate) fn number_field(row: &serde_json::Value, field: &str) -> Result<f64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| format!("missing number field {field}"))
}

pub(crate) fn number_array_field(row: &serde_json::Value, field: &str) -> Result<Vec<f64>, String> {
    row.get(field)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("missing numeric array field {field}"))?
        .iter()
        .map(|value| {
            value
                .as_f64()
                .ok_or_else(|| format!("non-numeric value in array field {field}"))
        })
        .collect()
}

pub(crate) fn bool_field(row: &serde_json::Value, field: &str) -> Result<bool, String> {
    row.get(field)
        .and_then(serde_json::Value::as_bool)
        .ok_or_else(|| format!("missing boolean field {field}"))
}

pub(crate) fn string_field(row: &serde_json::Value, field: &str) -> Result<String, String> {
    row.get(field)
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("missing string field {field}"))
}

pub(crate) fn insert_number(row: &mut BTreeMap<String, String>, field: &str, value: f64) {
    row.insert(field.to_string(), compact_decimal(value));
}

pub(crate) fn insert_rounded_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    value: f64,
    decimals: usize,
) {
    row.insert(field.to_string(), rounded_decimal(value, decimals));
}

pub(crate) fn insert_json_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    source: &serde_json::Value,
    source_field: &str,
) {
    row.insert(field.to_string(), json_number_string(source, source_field));
}

pub(crate) fn json_number_string(row: &serde_json::Value, field: &str) -> String {
    let value = row
        .get(field)
        .unwrap_or_else(|| panic!("missing number field {field}"));
    if let Some(number) = value.as_i64() {
        number.to_string()
    } else if let Some(number) = value.as_f64() {
        compact_decimal(number)
    } else {
        panic!("missing number field {field}")
    }
}

pub(crate) fn round6(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

pub(crate) fn round9(value: f64) -> f64 {
    (value * 1_000_000_000.0).round() / 1_000_000_000.0
}

pub(crate) fn compact_decimal(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        let text = format!("{value:.12}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

pub(crate) fn rounded_decimal(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let factor = 10_i128.pow(decimals as u32);
    let scaled = (value * factor as f64).round() as i128;
    let sign = if scaled < 0 { "-" } else { "" };
    let absolute = scaled.abs();
    let integer = absolute / factor;
    let fraction = absolute % factor;

    if decimals == 0 || fraction == 0 {
        return format!("{sign}{integer}");
    }

    let mut fraction_text = format!("{fraction:0decimals$}");
    while fraction_text.ends_with('0') {
        fraction_text.pop();
    }
    format!("{sign}{integer}.{fraction_text}")
}

pub(crate) fn format_millions_as_billions_or_trillions(value_millions: f64) -> String {
    if value_millions.abs() >= 1_000_000.0 {
        format!("${:.3}T", value_millions / 1_000_000.0)
    } else {
        format!("${:.3}B", value_millions / 1_000.0)
    }
}

pub(crate) fn decimal_string(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0.0".to_string()
    } else if trimmed.contains('.') {
        trimmed.to_string()
    } else {
        format!("{trimmed}.0")
    }
}

pub(crate) fn json_amount(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        decimal_string(value, 6)
    }
}

pub(crate) fn comma_number(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let (sign, unsigned) = text
        .strip_prefix('-')
        .map_or(("", text.as_str()), |rest| ("-", rest));
    let (integer, fraction) = unsigned
        .split_once('.')
        .map_or((unsigned, None), |(integer, fraction)| {
            (integer, Some(fraction))
        });
    let mut grouped = String::new();
    for (index, char) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(char);
    }
    let integer = grouped.chars().rev().collect::<String>();
    match fraction {
        Some(fraction) => format!("{sign}{integer}.{fraction}"),
        None => format!("{sign}{integer}"),
    }
}

pub(crate) fn annual_deficit_gap_string(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        decimal_string(value, 6)
    }
}

pub(crate) fn decade_label(year: i64) -> String {
    let start = year - year % 10;
    format!("{start}s")
}

pub(crate) fn sum_field(rows: &[&serde_json::Value], field: &str) -> Result<f64, String> {
    rows.iter().map(|row| number_field(row, field)).sum()
}

pub(crate) fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serializing string should not fail")
}

pub(crate) fn json_option_string(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), json_string)
}

pub(crate) fn json_owned_option_string(value: Option<&String>) -> String {
    value.map_or_else(|| "null".to_string(), |value| json_string(value))
}

pub(crate) fn count_rows(path: &Path, kind: &str) -> Result<String, String> {
    match kind {
        "jsonl" => {
            let content = fs::read_to_string(path)
                .map_err(|err| format!("failed to read {:?}: {err}", path))?;
            let mut count = 0usize;
            for line in content.lines() {
                serde_json::from_str::<serde_json::Value>(line)
                    .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))?;
                count += 1;
            }
            Ok(count.to_string())
        }
        "csv" => {
            let mut reader = csv::Reader::from_path(path)
                .map_err(|err| format!("failed to read CSV {:?}: {err}", path))?;
            let count = reader.records().count();
            Ok(count.to_string())
        }
        _ => Ok("n/a".to_string()),
    }
}

pub(crate) fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024 * 64];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|err| format!("failed to read {:?}: {err}", path))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub(crate) fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

