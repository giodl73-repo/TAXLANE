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

pub(crate) fn repo_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|err| format!("failed to get current directory: {err}"))
}

pub(crate) fn parse_json(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))?;
    Ok(())
}

pub(crate) fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))
}

pub(crate) fn chart_treatment_for_lane(lane: &serde_json::Value) -> Result<&'static str, String> {
    match string_field(lane, "display_treatment")?.as_str() {
        "modeled_lane" => Ok("Modeled lane"),
        "dedicated_financing_caveat_required" => Ok("Dedicated-financing caveat"),
        "display_separately" => match string_field(lane, "spending_control")?.as_str() {
            "net-interest" => Ok("Financing cost"),
            "offsetting" => Ok("Offset"),
            other => Err(format!(
                "unknown display_separately spending_control {other:?}"
            )),
        },
        "negative_or_offset_sensitive_lane" => Ok("Offset-sensitive adjustment"),
        other => Err(format!("unknown display_treatment {other:?}")),
    }
}

pub(crate) fn assert_number_close(
    row: &serde_json::Value,
    field: &str,
    expected: f64,
    tolerance: f64,
    label: &str,
) -> Result<(), String> {
    let actual = number_field(row, field)?;
    if (actual - expected).abs() > tolerance {
        return Err(format!("{label}: expected {expected}, found {actual}"));
    }
    Ok(())
}

pub(crate) fn parse_table_3_2_lines(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<Vec<Table32Line>, String> {
    let mut lines = Vec::new();
    let mut current_function: Option<(String, String)> = None;
    for (row_num, cells) in sheet {
        if *row_num < 4 {
            continue;
        }
        let Some(label) = text_cell(cells.get("A")) else {
            continue;
        };
        if let Some((code, function_label)) = parse_table_3_2_function_header(&label) {
            if is_table_3_2_function_code(&code) {
                current_function = Some((code, function_label));
            }
            continue;
        }
        if label.starts_with('(')
            || label == "On-budget unless otherwise stated"
            || label == "N/A = Not available"
        {
            continue;
        }
        if label == "Total outlays" {
            lines.push(Table32Line {
                source_row: *row_num,
                function_code: "total-federal-outlays".to_string(),
                function_label: "Total outlays".to_string(),
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::GrandTotal,
            });
            continue;
        }
        if let Some(total_label) = label.strip_prefix("Total, ") {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!("Table 3.2 row {row_num} total without function"));
            };
            if total_label != function_label {
                return Err(format!(
                    "Table 3.2 row {row_num} total {total_label:?} does not match current function {function_label:?}"
                ));
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::FunctionTotal,
            });
            continue;
        }
        if let Some((subfunction_code, mut subfunction_label)) = parse_table_3_2_coded_label(&label)
        {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!(
                    "Table 3.2 row {row_num} subfunction without function"
                ));
            };
            if let Some(subtotal_label) = subfunction_label.strip_prefix("Subtotal, ") {
                subfunction_label = subtotal_label.to_string();
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: Some(subfunction_code),
                subfunction_label: Some(subfunction_label),
                source_label: label,
                kind: Table32LineKind::Subfunction,
            });
        }
    }
    Ok(lines)
}

pub(crate) fn parse_table_3_2_function_header(label: &str) -> Option<(String, String)> {
    let label = label.strip_suffix(':')?;
    parse_table_3_2_coded_label(label)
}

pub(crate) fn parse_table_3_2_coded_label(label: &str) -> Option<(String, String)> {
    let (code, rest) = label.split_once(' ')?;
    if code.len() == 3 && code.chars().all(|char| char.is_ascii_digit()) {
        Some((code.to_string(), rest.trim().to_string()))
    } else {
        None
    }
}

pub(crate) fn parse_table_2_2_year(label: &str) -> Option<(i64, &'static str)> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    if let Some(year) = trimmed.strip_suffix(" estimate") {
        return year.parse::<i64>().ok().map(|year| (year, "estimate"));
    }
    trimmed.parse::<i64>().ok().map(|year| {
        let status = if year <= 2025 { "actual" } else { "estimate" };
        (year, status)
    })
}

pub(crate) fn read_sheet(path: &Path) -> Result<BTreeMap<i64, BTreeMap<String, CellValue>>, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut archive =
        ZipArchive::new(file).map_err(|err| format!("failed to read XLSX {:?}: {err}", path))?;
    let shared = read_shared_strings(&mut archive)?;
    let mut sheet_xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .map_err(|err| format!("failed to read sheet1.xml from {:?}: {err}", path))?
        .read_to_string(&mut sheet_xml)
        .map_err(|err| format!("failed to decode sheet1.xml from {:?}: {err}", path))?;
    let doc = Document::parse(&sheet_xml)
        .map_err(|err| format!("failed to parse sheet1.xml from {:?}: {err}", path))?;
    let mut rows = BTreeMap::new();
    for row in doc.descendants().filter(|node| node.has_tag_name("row")) {
        let row_num = row
            .attribute("r")
            .and_then(|value| value.parse::<i64>().ok())
            .ok_or_else(|| format!("sheet row without numeric r in {:?}", path))?;
        let mut cells = BTreeMap::new();
        for cell in row.children().filter(|node| node.has_tag_name("c")) {
            let Some(reference) = cell.attribute("r") else {
                continue;
            };
            let column = cell_column(reference);
            if column.is_empty() {
                continue;
            }
            let cell_type = cell.attribute("t");
            let raw = cell
                .children()
                .find(|node| node.has_tag_name("v"))
                .and_then(|node| node.text());
            let value = match (cell_type, raw) {
                (Some("s"), Some(raw)) => shared
                    .get(raw.parse::<usize>().map_err(|err| {
                        format!("invalid shared string index {raw:?} in {:?}: {err}", path)
                    })?)
                    .cloned(),
                (Some("inlineStr"), _) => Some(
                    cell.descendants()
                        .filter(|node| node.has_tag_name("t"))
                        .filter_map(|node| node.text())
                        .collect::<String>(),
                ),
                (_, Some(raw)) => Some(raw.to_string()),
                _ => None,
            };
            if let Some(value) = value.and_then(|value| cell_value(&value)) {
                cells.insert(column, value);
            }
        }
        rows.insert(row_num, cells);
    }
    Ok(rows)
}

pub(crate) fn read_shared_strings<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<Vec<String>, String> {
    let Ok(mut file) = archive.by_name("xl/sharedStrings.xml") else {
        return Ok(Vec::new());
    };
    let mut xml = String::new();
    file.read_to_string(&mut xml)
        .map_err(|err| format!("failed to decode sharedStrings.xml: {err}"))?;
    let doc =
        Document::parse(&xml).map_err(|err| format!("failed to parse sharedStrings.xml: {err}"))?;
    let strings = doc
        .descendants()
        .filter(|node| node.has_tag_name("si"))
        .map(|si| {
            si.descendants()
                .filter(|node| node.has_tag_name("t"))
                .filter_map(|node| node.text())
                .collect::<String>()
        })
        .collect();
    Ok(strings)
}

pub(crate) fn parse_table_1_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table11Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let Some(year) = int_cell(cells.get("A")) else {
            continue;
        };
        let (Some(receipts), Some(outlays), Some(surplus_or_deficit)) = (
            number_cell(cells.get("B")),
            number_cell(cells.get("C")),
            number_cell(cells.get("D")),
        ) else {
            continue;
        };
        output.insert(
            year,
            Table11Row {
                row: *row_num,
                total_receipts: receipts,
                total_outlays: outlays,
                surplus_or_deficit,
            },
        );
    }
    output
}

pub(crate) fn parse_table_2_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table21Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let (Some(year), Some(amount)) = (int_cell(cells.get("A")), number_cell(cells.get("B")))
        else {
            continue;
        };
        output.insert(
            year,
            Table21Row {
                row: *row_num,
                individual_income_tax: amount,
            },
        );
    }
    output
}

pub(crate) fn parse_table_3_1(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(Vec<i64>, BTreeMap<String, BTreeMap<i64, f64>>), String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut years_by_col = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            years_by_col.insert(column.clone(), year);
        }
    }

    let mut categories = BTreeMap::new();
    let mut table_rows: Vec<(&str, &str, i64)> = BROAD_CATEGORIES.to_vec();
    table_rows.push(("total-federal-outlays", "Total, Federal outlays", 35));
    for (key, label, row_num) in table_rows {
        let cells = rows
            .get(&row_num)
            .ok_or_else(|| format!("missing Table 3.1 row {row_num}"))?;
        if text_cell(cells.get("A")).as_deref() != Some(label) {
            return Err(format!(
                "Unexpected Table 3.1 row {row_num}: {:?}",
                text_cell(cells.get("A"))
            ));
        }
        let mut values = BTreeMap::new();
        for (column, year) in &years_by_col {
            if let Some(value) = number_cell(cells.get(column)) {
                values.insert(*year, value);
            }
        }
        categories.insert(key.to_string(), values);
    }
    let mut years = years_by_col.values().copied().collect::<Vec<_>>();
    years.sort_unstable();
    Ok((years, categories))
}

pub(crate) fn parse_table_3_2_year(label: &str) -> Option<i64> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    trimmed
        .strip_suffix(" estimate")
        .unwrap_or(trimmed)
        .parse::<i64>()
        .ok()
}

pub(crate) fn write_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let text = csv_text(headers, rows)?;
    fs::write(root.join(relative_path), text)
        .map_err(|err| format!("failed to write {relative_path}: {err}"))
}

pub(crate) fn read_jsonl(path: PathBuf) -> Result<Vec<serde_json::Value>, String> {
    let content =
        fs::read_to_string(&path).map_err(|err| format!("failed to read {:?}: {err}", path))?;
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))
        })
        .collect()
}

pub(crate) fn read_qpsd_component(
    root: &Path,
    path: &str,
    indicator_id: &str,
    indicator_label: &str,
) -> Result<BTreeMap<String, Option<f64>>, String> {
    let text = fs::read_to_string(root.join(path))
        .map_err(|err| format!("failed to read {path}: {err}"))?;
    let payload: serde_json::Value =
        serde_json::from_str(&text).map_err(|err| format!("failed to parse {path}: {err}"))?;
    let envelope = payload
        .as_array()
        .ok_or_else(|| format!("{path} must be a World Bank API array"))?;
    if envelope.len() != 2
        || envelope[0]
            .get("sourceid")
            .and_then(serde_json::Value::as_str)
            != Some("20")
        || envelope[0].get("total").and_then(serde_json::Value::as_u64) != Some(11)
    {
        return Err(format!("{path} QPSD API envelope failed"));
    }
    let rows = envelope[1]
        .as_array()
        .ok_or_else(|| format!("{path} QPSD observations must be an array"))?;
    if rows.len() != 11 {
        return Err(format!("{path} must contain 11 source rows"));
    }

    let expected_countries: BTreeSet<String> = [
        "USA", "DEU", "FRA", "GBR", "SWE", "NLD", "POL", "JPN", "KOR", "CAN", "AUS",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let mut values = BTreeMap::new();
    for row in rows {
        let country = string_field(row, "countryiso3code")?;
        let indicator = row
            .get("indicator")
            .ok_or_else(|| format!("{path} row {country} needs indicator"))?;
        if string_field(indicator, "id")? != indicator_id
            || string_field(indicator, "value")? != indicator_label
            || string_field(row, "date")? != "2022Q4"
            || !indicator_label.contains("General Gov.")
            || !indicator_label.contains("Nominal Value, % of GDP")
        {
            return Err(format!("{path} indicator semantics failed for {country}"));
        }
        let value =
            match row.get("value") {
                Some(value) if value.is_null() => None,
                Some(value) => Some(value.as_f64().ok_or_else(|| {
                    format!("{path} row {country} value must be numeric or null")
                })?),
                None => return Err(format!("{path} row {country} needs value")),
            };
        if values.insert(country.clone(), value).is_some() {
            return Err(format!("{path} has duplicate country {country}"));
        }
    }
    if values.keys().cloned().collect::<BTreeSet<_>>() != expected_countries
        || values.get("CAN") != Some(&None)
        || values.contains_key("SGP")
    {
        return Err(format!(
            "{path} must contain the exact 11-country source set with CAN null and SGP absent"
        ));
    }
    Ok(values)
}

pub(crate) fn read_pension_replacement_component(
    root: &Path,
    path: &str,
    measure: &str,
    unit: &str,
) -> Result<BTreeMap<String, f64>, String> {
    let expected_countries: BTreeSet<String> = [
        "USA", "DEU", "FRA", "GBR", "SWE", "NLD", "POL", "JPN", "KOR", "CAN", "AUS",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let mut values = BTreeMap::new();
    let mut reader = csv::Reader::from_path(root.join(path))
        .map_err(|err| format!("failed to read {path}: {err}"))?;
    for result in reader.deserialize::<BTreeMap<String, String>>() {
        let row = result.map_err(|err| format!("failed to parse {path}: {err}"))?;
        let country = row
            .get("REF_AREA")
            .ok_or_else(|| format!("{path} row needs REF_AREA"))?
            .to_string();
        for (field, expected) in [
            ("DATAFLOW", "OECD.ELS.SPD:DSD_PAG@DF_PRR(1.0)"),
            ("FREQ", "A"),
            ("MEASURE", measure),
            ("UNIT_MEASURE", unit),
            ("SEX", "M"),
            ("AGE", "_Z"),
            ("OPTIONALITY", "M"),
            ("TIME_PERIOD", "2024"),
            ("OBS_STATUS", "A"),
            ("UNIT_MULT", "0"),
            ("DECIMALS", "1"),
        ] {
            if row.get(field).map(String::as_str) != Some(expected) {
                return Err(format!(
                    "{path} row {country} field {field} must be {expected}"
                ));
            }
        }
        let value = row
            .get("OBS_VALUE")
            .ok_or_else(|| format!("{path} row {country} needs OBS_VALUE"))?
            .parse::<f64>()
            .map_err(|err| format!("{path} row {country} value failed to parse: {err}"))?;
        if values.insert(country.clone(), value).is_some() {
            return Err(format!("{path} has duplicate country {country}"));
        }
    }
    if values.len() != 11
        || values.keys().cloned().collect::<BTreeSet<_>>() != expected_countries
        || values.contains_key("SGP")
    {
        return Err(format!(
            "{path} must contain the exact 11-country OECD pension panel"
        ));
    }
    Ok(values)
}

pub(crate) fn read_idd_age_poverty_raw(
    root: &Path,
    path: &str,
    age: &str,
    expected_cells: &BTreeSet<(String, u16)>,
) -> Result<BTreeMap<(String, u16), (f64, String)>, String> {
    let mut values = BTreeMap::new();
    let mut reader = csv::Reader::from_path(root.join(path))
        .map_err(|err| format!("failed to read {path}: {err}"))?;
    for result in reader.deserialize::<BTreeMap<String, String>>() {
        let row = result.map_err(|err| format!("failed to parse {path}: {err}"))?;
        let country = row
            .get("REF_AREA")
            .ok_or("IDD row needs REF_AREA")?
            .to_string();
        let year = row
            .get("TIME_PERIOD")
            .ok_or_else(|| format!("IDD row {country} needs TIME_PERIOD"))?
            .parse::<u16>()
            .map_err(|err| format!("IDD row {country} year failed: {err}"))?;
        let cell = (country.clone(), year);
        let status = row
            .get("OBS_STATUS")
            .ok_or_else(|| format!("IDD row {country}/{year} needs OBS_STATUS"))?
            .to_string();
        for (field, expected) in [
            ("DATAFLOW", "OECD.WISE.INE:DSD_WISE_IDD@DF_IDD(1.0)"),
            ("FREQ", "A"),
            ("MEASURE", "PR_INC_DISP"),
            ("STATISTICAL_OPERATION", "_Z"),
            ("UNIT_MEASURE", "PT_POP"),
            ("AGE", age),
            ("METHODOLOGY", "METH2012"),
            ("DEFINITION", "D_CUR"),
            ("POVERTY_LINE", "PL_50"),
        ] {
            if row.get(field).map(String::as_str) != Some(expected) {
                return Err(format!(
                    "IDD row {country}/{year} field {field} must be {expected}"
                ));
            }
        }
        if !expected_cells.contains(&cell) || !matches!(status.as_str(), "A" | "P") {
            return Err(format!(
                "unexpected IDD poverty cell or status {country}/{year}"
            ));
        }
        let value = row
            .get("OBS_VALUE")
            .ok_or_else(|| format!("IDD row {country}/{year} needs OBS_VALUE"))?
            .parse::<f64>()
            .map_err(|err| format!("IDD row {country}/{year} value failed: {err}"))?;
        if values.insert(cell, (value, status)).is_some() {
            return Err(format!("duplicate IDD poverty row {country}/{year}"));
        }
    }
    if values.keys().cloned().collect::<BTreeSet<_>>() != *expected_cells {
        return Err(format!(
            "{path} does not contain its exact country/year range"
        ));
    }
    Ok(values)
}

pub(crate) fn read_json_artifact(root: &Path, path: &str) -> Result<serde_json::Value, String> {
    let text = fs::read_to_string(root.join(path))
        .map_err(|err| format!("failed to read {path}: {err}"))?;
    serde_json::from_str(&text).map_err(|err| format!("failed to parse {path}: {err}"))
}

pub(crate) fn parse_response_log_jsonl(
    text: &str,
    label: &str,
) -> Result<Vec<PerformanceDemandResponseLogRecord>, String> {
    text.lines()
        .map(|line| {
            let record: PerformanceDemandResponseLogRecord = serde_json::from_str(line)
                .map_err(|err| format!("failed to parse {label} row: {err}"))?;
            record.validate()?;
            Ok(record)
        })
        .collect::<Result<Vec<_>, String>>()
}

pub(crate) fn read_accountability_evidence_records(
    root: &Path,
) -> Result<Vec<AccountabilityEvidenceRecord>, String> {
    read_jsonl(root.join(ACCOUNTABILITY_EVIDENCE_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|err| format!("accountability evidence: invalid record shape: {err}"))
        })
        .collect()
}

