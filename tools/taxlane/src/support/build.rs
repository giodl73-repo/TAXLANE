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

pub(crate) fn build_receipt_share_table_2_2(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_receipt_share_rows(root)?;
    validate_receipt_share_rows(&rows)?;
    let jsonl = receipt_share_jsonl(&rows);
    let markdown = receipt_share_profile_markdown(&rows)?;

    if check_only {
        compare_text(
            root,
            RECEIPT_SHARE_JSONL_PATH,
            &jsonl,
            "Table 2.2 receipt share JSONL",
        )?;
        compare_text(
            root,
            RECEIPT_SHARE_PROFILE_PATH,
            &markdown,
            "Table 2.2 receipt share profile",
        )?;
    } else {
        fs::write(root.join(RECEIPT_SHARE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(RECEIPT_SHARE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_PROFILE_PATH}: {err}"))?;
    }

    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    println!(
        "validated {} Table 2.2 receipt share rows for {}-{}",
        rows.len(),
        first_year,
        last_year
    );
    Ok(())
}

pub(crate) fn build_outlay_function_table_3_1(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_outlay_function_3_1_rows(root)?;
    validate_outlay_function_3_1_rows(&rows, &profile)?;
    let jsonl = outlay_function_3_1_jsonl(&rows);
    let markdown = outlay_function_3_1_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_JSONL_PATH,
            &jsonl,
            "Table 3.1 outlay function JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_PROFILE_PATH,
            &markdown,
            "Table 3.1 outlay function profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_1_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_1_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.1 outlay function rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

pub(crate) fn build_outlay_function_table_3_2_national_defense(
    root: &Path,
    check_only: bool,
) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_national_defense_rows(root)?;
    validate_table_3_2_national_defense_rows(&rows, &profile)?;
    let jsonl = table_3_2_national_defense_jsonl(&rows);
    let markdown = table_3_2_national_defense_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH,
            &jsonl,
            "Table 3.2 National Defense JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH,
            &markdown,
            "Table 3.2 National Defense profile",
        )?;
    } else {
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH),
            jsonl,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH}: {err}")
        })?;
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH),
            markdown,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH}: {err}")
        })?;
    }

    println!(
        "validated {} Table 3.2 National Defense rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

pub(crate) fn build_outlay_function_table_3_2(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_rows(root)?;
    validate_table_3_2_rows(&profile)?;
    let jsonl = table_3_2_jsonl(&rows);
    let markdown = table_3_2_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_JSONL_PATH,
            &jsonl,
            "Table 3.2 JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_PROFILE_PATH,
            &markdown,
            "Table 3.2 profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_2_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_2_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.2 rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

pub(crate) fn build_receipt_share_rows(root: &Path) -> Result<Vec<ReceiptShareRow>, String> {
    let sheet = read_sheet(&root.join(TABLE_2_2_PATH))?;
    let mut rows = Vec::new();

    for (row_num, cells) in &sheet {
        let Some(year_label) = table_2_2_year_label(cells.get("A")) else {
            continue;
        };
        let Some((year, actual_or_projection)) = parse_table_2_2_year(&year_label) else {
            continue;
        };

        for category in RECEIPT_SHARE_CATEGORIES {
            let Some(percent) = number_cell(cells.get(category.column)) else {
                return Err(format!(
                    "Table 2.2 row {row_num} missing percent in column {}",
                    category.column
                ));
            };
            rows.push(ReceiptShareRow {
                fiscal_year: year,
                source_row: *row_num,
                source_column: category.column,
                receipt_category: category.receipt_category,
                source_receipt_label: category.source_receipt_label,
                percent: round6(percent),
                actual_or_projection,
                allocation_status: category.allocation_status,
                notes: category.notes,
            });
        }
    }

    rows.sort_by_key(|row| {
        (
            row.fiscal_year,
            receipt_share_sort_key(row.receipt_category),
        )
    });
    Ok(rows)
}

pub(crate) fn build_outlay_function_3_1_rows(
    root: &Path,
) -> Result<(Vec<OutlayFunctionRow>, OutlayFunctionProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let sheet = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (years_31, t31) = parse_table_3_1(&sheet)?;
    let columns_by_year = table_3_1_year_columns(&sheet)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(source_column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.1 source column"));
            continue;
        };
        let Some(total_outlays_31) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        let broad_category_total: f64 = BROAD_CATEGORIES
            .iter()
            .map(|(key, _, _)| {
                t31.get(*key)
                    .and_then(|values| values.get(year))
                    .copied()
                    .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let total_difference = total_outlays_31 - table_11.total_outlays;
        let broad_category_difference = broad_category_total - total_outlays_31;
        if total_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }
        if broad_category_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.1 broad category total {broad_category_total} does not reconcile to total {total_outlays_31}"
            ));
        }

        for (key, label, source_row) in BROAD_CATEGORIES {
            let amount = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            rows.push(OutlayFunctionRow {
                fiscal_year: *year,
                source_column: source_column.clone(),
                function_code: (*key).to_string(),
                function_label: (*label).to_string(),
                source_row: *source_row,
                amount: round6(amount),
                actual_or_projection: "actual",
                offsetting_treatment: if *key == "undistributed-offsetting-receipts" {
                    "undistributed-offsetting-receipts"
                } else {
                    "net"
                },
                notes: outlay_function_notes(key),
                include_table_1_1_source: false,
                table_1_1_row: None,
            });
        }
        rows.push(OutlayFunctionRow {
            fiscal_year: *year,
            source_column: source_column.clone(),
            function_code: "total-federal-outlays".to_string(),
            function_label: "Total, Federal outlays".to_string(),
            source_row: 35,
            amount: round6(total_outlays_31),
            actual_or_projection: "actual",
            offsetting_treatment: "net",
            notes: "Total federal outlays reconciled to OMB Historical Table 1.1 total outlays within displayed precision.",
            include_table_1_1_source: true,
            table_1_1_row: Some(table_11.row),
        });

        checks.push(OutlayFunctionCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_total: total_outlays_31,
            broad_category_total,
            total_difference,
            broad_category_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let profile = OutlayFunctionProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

pub(crate) fn build_table_3_2_national_defense_rows(
    root: &Path,
) -> Result<(Vec<Table32OutlayFunctionRow>, Table32NationalDefenseProfile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    validate_table_3_2_national_defense_labels(&sheet_32)?;

    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();
    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_national_defense) = t31
            .get("national-defense")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 National Defense"));
            continue;
        };

        let mut subfunction_total = 0.0;
        let mut parent_total = None;
        for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
            let amount = table_3_2_number(&sheet_32, line.source_row, column)?;
            if line.subfunction_code.is_some() {
                subfunction_total += amount;
            } else {
                parent_total = Some(amount);
            }
            rows.push(Table32OutlayFunctionRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: "050",
                function_label: "National Defense",
                subfunction_code: line.subfunction_code,
                subfunction_label: line.subfunction_label,
                source_label: line.source_label,
                amount: round6(amount),
                notes: line.notes,
            });
        }

        let Some(table_3_2_national_defense) = parent_total else {
            errors.push(format!("{year}: missing Table 3.2 National Defense total"));
            continue;
        };
        let table_3_1_difference = table_3_2_national_defense - table_3_1_national_defense;
        let subfunction_difference = subfunction_total - table_3_2_national_defense;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense {table_3_2_national_defense} does not reconcile to Table 3.1 {table_3_1_national_defense}"
            ));
        }
        if subfunction_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense subfunctions {subfunction_total} do not reconcile to total {table_3_2_national_defense}"
            ));
        }
        checks.push(Table32NationalDefenseCheck {
            year: *year,
            table_3_1_national_defense,
            table_3_2_national_defense,
            subfunction_total,
            table_3_1_difference,
            subfunction_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let profile = Table32NationalDefenseProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

pub(crate) fn build_table_6_1_national_defense_rows(
    root: &Path,
) -> Result<
    (
        Vec<Table61NationalDefenseRow>,
        Table61NationalDefenseProfile,
    ),
    String,
> {
    let sheet = read_sheet(&root.join(TABLE_6_1_PATH))?;
    let columns_by_year = table_6_1_year_columns(&sheet)?;
    let gdp_section = table_6_1_section_row(&sheet, "As percentages of GDP:")?;
    let outlays_section = table_6_1_section_row(&sheet, "As percentages of outlays:")?;
    if outlays_section <= gdp_section {
        return Err("Table 6.1 section order unexpected".to_string());
    }
    let defense_row =
        table_6_1_label_row_between(&sheet, "National defense (1)", gdp_section, outlays_section)?;
    let total_row =
        table_6_1_label_row_between(&sheet, "Total outlays", gdp_section, outlays_section)?;

    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1940..=2025).contains(year))
        .collect();
    let sample_years = [1944, 1953, 1968, 1979, 1986, 2000, 2010, 2025];
    let mut rows = Vec::new();
    let mut samples = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 6.1 source column"));
            continue;
        };
        let Some(percent) = number_cell(sheet.get(&defense_row).and_then(|row| row.get(column)))
        else {
            errors.push(format!(
                "{year}: missing national-defense %GDP at {column}{defense_row}"
            ));
            continue;
        };
        if !(0.0..=50.0).contains(&percent) {
            errors.push(format!(
                "{year}: implausible national-defense %GDP {percent}"
            ));
        }
        rows.push(Table61NationalDefenseRow {
            fiscal_year: *year,
            source_column: column.clone(),
            source_row: defense_row,
            percent_of_gdp: round6(percent),
        });
        if sample_years.contains(year) {
            let total =
                number_cell(sheet.get(&total_row).and_then(|row| row.get(column))).unwrap_or(0.0);
            samples.push((*year, round6(percent), round6(total)));
        }
    }

    for (year, low, high) in [(1953_i64, 13.0_f64, 14.5_f64), (2025, 2.5, 3.5)] {
        match rows.iter().find(|row| row.fiscal_year == year) {
            Some(row) if (low..=high).contains(&row.percent_of_gdp) => {}
            Some(row) => errors.push(format!(
                "{year}: national-defense %GDP {} outside expected [{low}, {high}]",
                row.percent_of_gdp
            )),
            None => errors.push(format!("missing anchor year {year}")),
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 6.1 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 6.1 years".to_string())?;
    let profile = Table61NationalDefenseProfile {
        first_year,
        last_year,
        year_count: years.len(),
        samples,
    };
    Ok((rows, profile))
}

pub(crate) fn build_outlay_composition_table_6_1_national_defense(
    root: &Path,
    check_only: bool,
) -> Result<(), String> {
    let (rows, profile) = build_table_6_1_national_defense_rows(root)?;
    validate_table_6_1_national_defense_rows(&rows, &profile)?;
    let jsonl = table_6_1_national_defense_jsonl(&rows);
    let markdown = table_6_1_national_defense_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH,
            &jsonl,
            "Table 6.1 National Defense %GDP JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH,
            &markdown,
            "Table 6.1 National Defense %GDP profile",
        )?;
    } else {
        fs::write(
            root.join(OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH),
            jsonl,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_JSONL_PATH}: {err}")
        })?;
        fs::write(
            root.join(OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH),
            markdown,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_COMPOSITION_6_1_NATIONAL_DEFENSE_PROFILE_PATH}: {err}")
        })?;
    }

    println!(
        "validated {} Table 6.1 National Defense %GDP rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

pub(crate) fn build_table_3_2_rows(root: &Path) -> Result<(Vec<Table32Row>, Table32Profile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut grand_checks = Vec::new();
    let mut function_checks = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_total_outlays) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        let mut subfunction_totals: BTreeMap<String, f64> = BTreeMap::new();
        let mut explicit_function_totals: BTreeMap<String, (String, f64)> = BTreeMap::new();
        let mut table_3_2_total_outlays = None;

        for line in &lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            match line.kind {
                Table32LineKind::Subfunction => {
                    *subfunction_totals
                        .entry(line.function_code.clone())
                        .or_insert(0.0) += amount;
                }
                Table32LineKind::FunctionTotal => {
                    explicit_function_totals.insert(
                        line.function_code.clone(),
                        (line.function_label.clone(), amount),
                    );
                }
                Table32LineKind::GrandTotal => {
                    table_3_2_total_outlays = Some(amount);
                }
            }
            rows.push(Table32Row {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line.subfunction_code.clone(),
                subfunction_label: line.subfunction_label.clone(),
                source_label: line.source_label.clone(),
                amount: round6(amount),
                kind: line.kind.clone(),
            });
        }

        let Some(table_3_2_total_outlays) = table_3_2_total_outlays else {
            errors.push(format!("{year}: missing Table 3.2 total outlays"));
            continue;
        };
        let mut function_total_sum = 0.0;
        for (function_code, subfunction_total) in &subfunction_totals {
            if let Some((function_label, function_total)) =
                explicit_function_totals.get(function_code)
            {
                let difference = subfunction_total - function_total;
                if difference.abs() > 2.0 {
                    errors.push(format!(
                        "{year}: Table 3.2 function {function_code} subfunctions {subfunction_total} do not reconcile to total {function_total}"
                    ));
                }
                function_total_sum += function_total;
                function_checks.push(Table32FunctionCheck {
                    year: *year,
                    function_code: function_code.clone(),
                    function_label: function_label.clone(),
                    function_total: *function_total,
                    subfunction_total: *subfunction_total,
                    difference,
                });
            } else {
                function_total_sum += subfunction_total;
            }
        }
        let table_3_1_difference = table_3_2_total_outlays - table_3_1_total_outlays;
        let function_total_difference = function_total_sum - table_3_2_total_outlays;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 total {table_3_2_total_outlays} does not reconcile to Table 3.1 total {table_3_1_total_outlays}"
            ));
        }
        if function_total_difference.abs() > 5.0 {
            errors.push(format!(
                "{year}: Table 3.2 function totals {function_total_sum} do not reconcile to total outlays {table_3_2_total_outlays}"
            ));
        }
        grand_checks.push(Table32GrandCheck {
            year: *year,
            table_3_1_total_outlays,
            table_3_2_total_outlays,
            function_total_sum,
            table_3_1_difference,
            function_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let subfunction_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .count();
    let function_total_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::FunctionTotal))
        .count();
    let function_count = lines
        .iter()
        .filter(|line| !matches!(line.kind, Table32LineKind::GrandTotal))
        .map(|line| line.function_code.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let profile = Table32Profile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        line_count: lines.len(),
        subfunction_line_count,
        function_total_line_count,
        function_count,
        grand_checks,
        function_checks,
    };
    Ok((rows, profile))
}

pub(crate) fn build_subfunction_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_subfunction_model_records(root)?;
    validate_subfunction_model_records(&records, &profile)?;
    let jsonl = subfunction_model_jsonl(&records);
    let profile_markdown = subfunction_model_profile_markdown(&profile);
    let readme = subfunction_model_readme_markdown();

    if check_only {
        compare_text(
            root,
            SUBFUNCTION_MODEL_JSONL_PATH,
            &jsonl,
            "subfunction model JSONL",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_PROFILE_PATH,
            &profile_markdown,
            "subfunction model profile",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_README_PATH,
            &readme,
            "subfunction model README",
        )?;
    } else {
        fs::create_dir_all(root.join("data/derived/income_tax_outlay_subfunction_model"))
            .map_err(|err| format!("failed to create subfunction model directory: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_PROFILE_PATH), profile_markdown)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_PROFILE_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_README_PATH), readme)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_README_PATH}: {err}"))?;
    }

    println!(
        "validated {} subfunction model rows for {}-{}",
        records.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

pub(crate) fn build_subfunction_model_records(
    root: &Path,
) -> Result<(Vec<SubfunctionModelRow>, SubfunctionModelProfile), String> {
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let subfunction_lines: Vec<Table32Line> = lines
        .into_iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .collect();
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let total_outlays = table_3_2_optional_number(&sheet_32, 140, column)
            .ok_or_else(|| format!("{year}: missing Table 3.2 total outlays"))?;
        let mut subfunction_total = 0.0;
        let mut year_values = Vec::new();
        for line in &subfunction_lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            subfunction_total += amount;
            year_values.push((line, amount));
        }
        let subfunction_total_difference = subfunction_total - total_outlays;
        if subfunction_total_difference.abs() > 10.0 {
            errors.push(format!(
                "{year}: Table 3.2 subfunction total {subfunction_total} does not reconcile to total outlays {total_outlays}"
            ));
        }
        let income_tax = table_21.individual_income_tax;
        let mut modeled_sum = 0.0;
        for (line, amount) in year_values {
            let modeled_amount = income_tax * amount / subfunction_total;
            modeled_sum += modeled_amount;
            records.push(SubfunctionModelRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line
                    .subfunction_code
                    .clone()
                    .ok_or_else(|| "missing subfunction code".to_string())?,
                subfunction_label: line
                    .subfunction_label
                    .clone()
                    .ok_or_else(|| "missing subfunction label".to_string())?,
                subfunction_outlays_amount: round6(amount),
                subfunction_total_outlays_amount: round6(subfunction_total),
                total_outlays_amount: round6(total_outlays),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(amount / total_outlays * 100.0),
                allocation_share_percent: round9(amount / subfunction_total * 100.0),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
            });
        }
        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: subfunction modeled sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        checks.push(SubfunctionModelCheck {
            year: *year,
            table_3_2_total_outlays: total_outlays,
            subfunction_total,
            individual_income_tax: income_tax,
            modeled_sum,
            subfunction_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let subfunction_count = subfunction_lines.len();
    let profile = SubfunctionModelProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: records.len(),
        subfunction_count,
        checks,
    };
    Ok((records, profile))
}

pub(crate) fn build_annual_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_annual_records(root)?;
    let jsonl = annual_model_jsonl(&records);
    let markdown = source_profile_markdown(&profile);

    if check_only {
        compare_text(root, ANNUAL_JSONL_PATH, &jsonl, "annual model JSONL")?;
        compare_text(root, SOURCE_PROFILE_PATH, &markdown, "source profile")?;
    } else {
        fs::write(root.join(ANNUAL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {ANNUAL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SOURCE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {SOURCE_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} rows for {}-{}",
        profile.record_count, profile.first_year, profile.last_year
    );
    Ok(())
}

pub(crate) fn build_annual_records(root: &Path) -> Result<(Vec<AnnualRecord>, AnnualProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let (years_31, t31) = parse_table_3_1(&read_sheet(&root.join(TABLE_3_1_PATH))?)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut errors = Vec::new();
    let mut annual_checks = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let Some(total_outlays_31) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        if (table_11.total_outlays - total_outlays_31).abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }

        let category_total: f64 = BROAD_CATEGORIES
            .iter()
            .map(|(key, _, _)| {
                t31.get(*key)
                    .and_then(|values| values.get(year))
                    .copied()
                    .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let category_total_difference = category_total - total_outlays_31;
        if (category_total - total_outlays_31).abs() > 2.0 {
            errors.push(format!(
                "{year}: category total {category_total} does not reconcile to Table 3.1 total {total_outlays_31}"
            ));
        }

        let income_tax = table_21.individual_income_tax;
        let total_receipts = table_11.total_receipts;
        let surplus_or_deficit = table_11.surplus_or_deficit;
        let deficit_gap = (total_outlays_31 - total_receipts).max(0.0);
        let borrowed_share = deficit_gap / total_outlays_31 * 100.0;
        let income_tax_coverage = income_tax / total_outlays_31 * 100.0;
        let mut modeled_sum = 0.0;

        for (key, label, table_row) in BROAD_CATEGORIES {
            let category_outlays = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            let outlay_share = category_outlays / total_outlays_31 * 100.0;
            let allocation_share = category_outlays / category_total * 100.0;
            let modeled_amount = income_tax * category_outlays / category_total;
            modeled_sum += modeled_amount;
            records.push(AnnualRecord {
                fiscal_year: *year,
                category_key: key,
                category_label: label,
                table_11_row: table_11.row,
                table_21_row: table_21.row,
                table_31_row: *table_row,
                category_outlays_amount: round6(category_outlays),
                total_outlays_amount: round6(total_outlays_31),
                category_total_outlays_amount: round6(category_total),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(outlay_share),
                allocation_share_percent: round9(allocation_share),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
                total_receipts_amount: round6(total_receipts),
                surplus_or_deficit_amount: round6(surplus_or_deficit),
                deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
                category_total_reconciliation_difference_amount: round6(category_total_difference),
            });
        }

        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: modeled allocation sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        annual_checks.push(AnnualCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_outlays: total_outlays_31,
            category_total,
            income_tax,
            modeled_sum,
            deficit_gap,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years.first().ok_or_else(|| "no annual years".to_string())?;
    let last_year = *years.last().ok_or_else(|| "no annual years".to_string())?;
    let profile = AnnualProfile {
        year_count: years.len(),
        first_year,
        last_year,
        record_count: records.len(),
        annual_checks,
    };
    Ok((records, profile))
}

pub(crate) fn build_decade_summary(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_decade_summary_rows(root)?;
    validate_decade_summary_rows(&rows)?;
    let jsonl = decade_summary_jsonl(&rows);
    let markdown = decade_summary_markdown(&rows)?;

    if check_only {
        compare_text(root, DECADE_JSONL_PATH, &jsonl, "decade JSONL")?;
        compare_text(root, DECADE_MD_PATH, &markdown, "decade Markdown")?;
    } else {
        fs::write(root.join(DECADE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {DECADE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(DECADE_MD_PATH), markdown)
            .map_err(|err| format!("failed to write {DECADE_MD_PATH}: {err}"))?;
    }
    println!("validated {} decade summary rows", rows.len());
    Ok(())
}

pub(crate) fn build_decade_summary_rows(root: &Path) -> Result<Vec<DecadeSummaryRow>, String> {
    let annual_rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut by_decade: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
    for row in annual_rows {
        let year = int_field(&row, "fiscal_year")?;
        by_decade.entry(decade_label(year)).or_default().push(row);
    }

    let mut output = Vec::new();
    for (decade, decade_rows) in by_decade {
        let mut years: Vec<i64> = decade_rows
            .iter()
            .map(|row| int_field(row, "fiscal_year"))
            .collect::<Result<Vec<_>, _>>()?;
        years.sort_unstable();
        years.dedup();

        for year in &years {
            let count = decade_rows
                .iter()
                .filter(|row| int_field(row, "fiscal_year").ok() == Some(*year))
                .count();
            if count != CATEGORY_FIELDS.len() {
                return Err(format!(
                    "{decade}: expected six category rows for fiscal year {year}, found {count}"
                ));
            }
        }

        let anchors: Vec<&serde_json::Value> = decade_rows
            .iter()
            .filter(|row| {
                string_field(row, "category_key").ok().as_deref() == Some("national-defense")
            })
            .collect();
        let income_tax_total = sum_field(&anchors, "individual_income_tax_receipts_amount")?;
        let total_outlays = sum_field(&anchors, "total_outlays_amount")?;
        let total_receipts = sum_field(&anchors, "total_receipts_amount")?;
        let deficit_gap = sum_field(&anchors, "deficit_gap_amount")?;
        let borrowed_share = if total_outlays == 0.0 {
            0.0
        } else {
            deficit_gap / total_outlays * 100.0
        };
        let income_tax_coverage = if total_outlays == 0.0 {
            0.0
        } else {
            income_tax_total / total_outlays * 100.0
        };

        let mut percent_sum = 0.0;
        for (category_key, _) in CATEGORY_FIELDS {
            let category_rows: Vec<&serde_json::Value> = decade_rows
                .iter()
                .filter(|row| {
                    string_field(row, "category_key").ok().as_deref() == Some(*category_key)
                })
                .collect();
            if category_rows.len() != years.len() {
                return Err(format!("{decade}: missing {category_key} rows"));
            }
            let modeled_total = sum_field(&category_rows, "modeled_income_tax_allocation_amount")?;
            let category_percent = modeled_total / income_tax_total * 100.0;
            percent_sum += category_percent;
            output.push(DecadeSummaryRow {
                decade: decade.clone(),
                start_fiscal_year: *years.first().ok_or_else(|| format!("{decade}: no years"))?,
                end_fiscal_year: *years.last().ok_or_else(|| format!("{decade}: no years"))?,
                year_count: years.len(),
                coverage_note: if years.len() < 10 {
                    "partial_decade"
                } else {
                    "full_decade"
                },
                category_key: (*category_key).to_string(),
                category_label: string_field(category_rows[0], "category_label")?,
                cumulative_modeled_income_tax_allocation_amount: round6(modeled_total),
                cumulative_individual_income_tax_receipts_amount: round6(income_tax_total),
                category_percent_of_decade_income_tax: round9(category_percent),
                cumulative_total_outlays_amount: round6(total_outlays),
                cumulative_total_receipts_amount: round6(total_receipts),
                cumulative_deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
            });
        }
        if (percent_sum - 100.0).abs() > 0.00001 {
            return Err(format!(
                "{decade}: category percentages sum to {percent_sum}"
            ));
        }
    }
    Ok(output)
}

pub(crate) fn build_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut grouped: BTreeMap<i64, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let year = int_field(&row, "fiscal_year")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(year).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (year, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{year}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("fiscal_year".to_string(), year.to_string());
        row.insert("coverage_note".to_string(), "full_year".to_string());
        insert_json_number(
            &mut row,
            "individual_income_tax_receipts_millions",
            anchor,
            "individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "total_outlays_millions",
            anchor,
            "total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "total_receipts_millions",
            anchor,
            "total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "deficit_gap_millions",
            anchor,
            "deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("{year}: missing {category_key} row"))?;
            let percent = number_field(category, "allocation_share_percent")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

pub(crate) fn build_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(DECADE_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let decade = string_field(&row, "decade")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(decade).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (decade, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("decade".to_string(), decade);
        row.insert(
            "start_fiscal_year".to_string(),
            int_field(anchor, "start_fiscal_year")?.to_string(),
        );
        row.insert(
            "end_fiscal_year".to_string(),
            int_field(anchor, "end_fiscal_year")?.to_string(),
        );
        row.insert(
            "year_count".to_string(),
            int_field(anchor, "year_count")?.to_string(),
        );
        row.insert(
            "coverage_note".to_string(),
            string_field(anchor, "coverage_note")?,
        );
        insert_json_number(
            &mut row,
            "cumulative_individual_income_tax_receipts_millions",
            anchor,
            "cumulative_individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_outlays_millions",
            anchor,
            "cumulative_total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_receipts_millions",
            anchor,
            "cumulative_total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_deficit_gap_millions",
            anchor,
            "cumulative_deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("missing {category_key} row"))?;
            let percent = number_field(category, "category_percent_of_decade_income_tax")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

pub(crate) fn build_subfunction_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    rows.iter().map(subfunction_annual_csv_row).collect()
}

pub(crate) fn build_subfunction_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<(String, String), SubfunctionDecadeRollup>> =
        BTreeMap::new();
    let mut decade_receipts: BTreeMap<String, BTreeMap<i64, f64>> = BTreeMap::new();

    for row in &rows {
        let year = int_field(row, "fiscal_year")?;
        let decade = decade_label(year);
        let income_tax = number_field(row, "individual_income_tax_receipts_amount")?;
        decade_receipts
            .entry(decade.clone())
            .or_default()
            .entry(year)
            .or_insert(income_tax);

        let function_code = string_field(row, "function_code")?;
        let subfunction_code = string_field(row, "subfunction_code")?;
        let rollup = grouped
            .entry(decade)
            .or_default()
            .entry((function_code, subfunction_code))
            .or_insert_with(|| SubfunctionDecadeRollup {
                function_code: string_field(row, "function_code").unwrap_or_default(),
                function_label: string_field(row, "function_label").unwrap_or_default(),
                subfunction_code: string_field(row, "subfunction_code").unwrap_or_default(),
                subfunction_label: string_field(row, "subfunction_label").unwrap_or_default(),
                subfunction_outlays: 0.0,
                modeled_allocation: 0.0,
            });
        rollup.subfunction_outlays += number_field(row, "subfunction_outlays_amount")?;
        rollup.modeled_allocation += number_field(row, "modeled_income_tax_allocation_amount")?;
    }

    let mut output = Vec::new();
    for (decade, mut subfunctions) in grouped {
        let receipts_by_year = decade_receipts
            .get(&decade)
            .ok_or_else(|| format!("{decade}: missing receipt denominator"))?;
        let start_year = *receipts_by_year
            .keys()
            .next()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let end_year = *receipts_by_year
            .keys()
            .next_back()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let year_count = receipts_by_year.len();
        let income_tax: f64 = receipts_by_year.values().sum();
        let coverage_note = if year_count == 10 {
            "full_decade"
        } else {
            "partial_decade"
        };

        let mut rows: Vec<SubfunctionDecadeRollup> =
            subfunctions.values_mut().map(|row| row.clone()).collect();
        rows.sort_by(|left, right| {
            right
                .modeled_allocation
                .partial_cmp(&left.modeled_allocation)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.subfunction_label.cmp(&right.subfunction_label))
        });

        for row in rows {
            let mut output_row = BTreeMap::new();
            output_row.insert("decade".to_string(), decade.clone());
            output_row.insert("start_fiscal_year".to_string(), start_year.to_string());
            output_row.insert("end_fiscal_year".to_string(), end_year.to_string());
            output_row.insert("year_count".to_string(), year_count.to_string());
            output_row.insert("coverage_note".to_string(), coverage_note.to_string());
            output_row.insert("function_code".to_string(), row.function_code);
            output_row.insert("function_label".to_string(), row.function_label);
            output_row.insert("subfunction_code".to_string(), row.subfunction_code);
            output_row.insert("subfunction_label".to_string(), row.subfunction_label);
            insert_rounded_number(
                &mut output_row,
                "cumulative_individual_income_tax_receipts_millions",
                income_tax,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_subfunction_outlays_millions",
                row.subfunction_outlays,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_modeled_income_tax_allocation_millions",
                row.modeled_allocation,
                6,
            );
            insert_number(
                &mut output_row,
                "decade_allocation_share_percent",
                round9(row.modeled_allocation / income_tax * 100.0),
            );
            output_row.insert(
                "allocation_method".to_string(),
                "proportional_outlay_share".to_string(),
            );
            output_row.insert(
                "legal_allocation_status".to_string(),
                "modeled_not_legal_dedication".to_string(),
            );
            output_row.insert("actual_or_projection".to_string(), "actual".to_string());
            output.push(output_row);
        }
    }
    Ok(output)
}

pub(crate) fn build_subfunction_fy2025_top_csv_rows(
    root: &Path,
    count: usize,
) -> Result<Vec<BTreeMap<String, String>>, String> {
    let mut rows: Vec<serde_json::Value> = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?
        .into_iter()
        .filter(|row| int_field(row, "fiscal_year") == Ok(2025))
        .collect();
    rows.sort_by(|left, right| {
        number_field(right, "modeled_income_tax_allocation_amount")
            .unwrap_or(0.0)
            .partial_cmp(&number_field(left, "modeled_income_tax_allocation_amount").unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    rows.iter()
        .take(count)
        .enumerate()
        .map(|(index, row)| subfunction_top_csv_row(index + 1, row))
        .collect()
}

pub(crate) fn build_spend_category_dashboard(rows: &[SpendCategoryMapRecord]) -> Result<String, String> {
    let total_outlays: f64 = rows
        .iter()
        .map(|row| row.subfunction_outlays_millions)
        .sum();
    let total_share: f64 = rows
        .iter()
        .map(|row| row.share_of_total_outlays_percent)
        .sum();
    let total_modeled_income_tax: f64 = rows
        .iter()
        .map(|row| row.modeled_income_tax_allocation_millions)
        .sum();
    let top_five_share: f64 = rows
        .iter()
        .take(5)
        .map(|row| row.share_of_total_outlays_percent)
        .sum();

    let mut lines = vec![
        "# Spend Category Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes the draft FY2025 spend-category map. It is a question-routing view, not taxpayer-dollar tracing, legal dedication, recipient-level spending, or a performance finding.".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- Rows: {}", rows.len()),
        format!(
            "- Top 15 outlays: {}",
            format_millions_as_billions_or_trillions(total_outlays)
        ),
        format!("- Share represented: {:.2}%", total_share),
        format!(
            "- Modeled income-tax allocation represented: {}",
            format_millions_as_billions_or_trillions(total_modeled_income_tax)
        ),
        format!("- Top five share: {:.2}%", top_five_share),
        String::new(),
        "## Rows".to_string(),
        String::new(),
        "| Rank | Category | OMB function | Outlays | Share | Modeled income-tax allocation | Next source need |".to_string(),
        "|---:|---|---|---:|---:|---:|---|".to_string(),
    ];

    for row in rows {
        lines.push(format!(
            "| {} | {} | {} | {} | {:.2}% | {} | {} |",
            row.rank,
            escape_table_cell(&row.subfunction_label),
            escape_table_cell(&row.function_label),
            format_millions_as_billions_or_trillions(row.subfunction_outlays_millions),
            row.share_of_total_outlays_percent,
            format_millions_as_billions_or_trillions(row.modeled_income_tax_allocation_millions),
            escape_table_cell(&row.next_source_need),
        ));
    }

    lines.extend([
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Every row remains `question_surface_only`. Use this dashboard to choose the next source to inspect; do not use it to claim fraud, waste, abuse, poor performance, legal dedication of income-tax dollars, or recipient-level outlays.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_readiness_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut lines = vec![
        "# Accountability Evidence Readiness Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This report classifies draft accountability evidence records by public-claim readiness.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Readiness States".to_string(),
        String::new(),
        "| State | Meaning |".to_string(),
        "|---|---|".to_string(),
        "| `EvidenceOnly` | Internal evidence review only; not ready for public claims. |".to_string(),
        "| `NeedsRoleReview` | Source/accountability reviewed and waiting for public wording review. |".to_string(),
        "| `PublicClaimEligible` | Role reviewed with official finding or adjudicated status. |".to_string(),
        String::new(),
        "## Records".to_string(),
        String::new(),
        "| Record ID | Lane | Evidence Kind | Anomaly Class | Allegation Status | Review Status | Readiness | Next Action | Public Summary |".to_string(),
        "|---|---|---|---|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        let readiness = record.public_claim_readiness();
        lines.push(format!(
            "| `{}` | {} | {:?} | {:?} | {:?} | {:?} | `{}` | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.evidence_kind,
            record.anomaly_class,
            record.allegation_status,
            record.review_status,
            readiness.as_str(),
            record.accountability_next_action().replace('|', "\\|"),
            record.public_summary.replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Records marked `EvidenceOnly` or `NeedsRoleReview` must not be presented as public fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_action_queue(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut queue: BTreeMap<&'static str, Vec<AccountabilityEvidenceRecord>> = BTreeMap::new();
    for record in records {
        queue
            .entry(record.accountability_next_action())
            .or_default()
            .push(record);
    }

    let mut lines = vec![
        "# Accountability Evidence Action Queue".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated queue turns draft accountability evidence records into reviewer work."
            .to_string(),
        "It is not a public fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Queue".to_string(),
    ];

    for (action, mut records) in queue {
        records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
        lines.push(String::new());
        lines.push(format!("### {action}"));
        lines.push(String::new());
        lines.push("| Record ID | Lane | Readiness | Public-Use Blocker |".to_string());
        lines.push("|---|---|---|---|".to_string());
        for record in records {
            lines.push(format!(
                "| `{}` | {} | `{}` | {} |",
                record.record_id,
                record.lane_id.as_deref().unwrap_or("n/a"),
                record.public_claim_readiness().as_str(),
                record
                    .accountability_public_use_blocker()
                    .replace('|', "\\|")
            ));
        }
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Queue entries are tasks for evidence review. They are not publishable claims by themselves."
            .to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_packet(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Accountability Performance Demand Packet".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated packet turns accountability evidence blockers into questions people can ask before demanding performance on public money.".to_string(),
        "It explains what TAXLANE can say now, what evidence is still missing, and what claim boundary remains in force.".to_string(),
        String::new(),
        "## Demand Questions".to_string(),
        String::new(),
        "| Record ID | Lane | What TAXLANE Can Say Now | Demand Question | Claim Boundary |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.public_summary.replace('|', "\\|"),
            record.accountability_demand_question().replace('|', "\\|"),
            record
                .accountability_public_use_blocker()
                .replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Public-Use Rule".to_string());
    lines.push(String::new());
    lines.push(
        "Use these rows to request evidence, reviewed wording, or official findings. Do not present them as fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_work_items_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let mut lines = Vec::new();
    for record in records {
        lines.push(
            serde_json::to_string(&record.accountability_work_item())
                .map_err(|err| format!("failed to serialize accountability work item: {err}"))?,
        );
    }
    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_claim_guard_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut readiness_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut blocker_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut public_claim_allowed = 0usize;

    for record in &records {
        let work_item = record.accountability_work_item();
        *readiness_counts.entry(work_item.readiness).or_default() += 1;
        *blocker_counts
            .entry(work_item.public_use_blocker)
            .or_default() += 1;
        if work_item.public_claim_allowed {
            public_claim_allowed += 1;
        }
    }

    let mut lines = vec![
        "# Accountability Claim Guard Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated report summarizes whether accountability evidence records can support public claims.".to_string(),
        "It is a guardrail report, not a fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Claim Guard Summary".to_string(),
        String::new(),
        format!("- Total records: {}", records.len()),
        format!("- Public claims currently allowed: {public_claim_allowed}"),
        format!(
            "- Public claims currently blocked: {}",
            records.len().saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Readiness Counts".to_string(),
        String::new(),
        "| Readiness | Records |".to_string(),
        "|---|---:|".to_string(),
    ];

    for (readiness, count) in readiness_counts {
        lines.push(format!("| `{readiness}` | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Public-Use Blockers".to_string(),
        String::new(),
        "| Blocker | Records |".to_string(),
        "|---|---:|".to_string(),
    ]);

    for (blocker, count) in blocker_counts {
        let escaped_blocker = blocker.replace('|', "\\|");
        lines.push(format!("| {escaped_blocker} | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Allowed Public Use".to_string(),
        String::new(),
        "Current safe use: ask the demand questions and request the missing reviewed evidence or role-approved wording.".to_string(),
        "Current unsafe use: present these draft records as fraud, waste, abuse, or performance findings.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_public_questions(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Public Accountability Questions".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "These generated questions are safe to ask publicly because they request reviewed evidence or role-approved wording.".to_string(),
        "They are not findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Questions".to_string(),
        String::new(),
        "| Lane | Public-Safe Question | Why This Is Still Blocked |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use these questions to ask for evidence. Do not present the underlying draft records as public claims.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_public_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let total_records = records.len();
    let public_claim_allowed = records
        .iter()
        .filter(|record| record.accountability_work_item().public_claim_allowed)
        .count();

    let mut lines = vec![
        "# Accountability Public Brief".to_string(),
        String::new(),
        "## What TAXLANE Can Say Now".to_string(),
        String::new(),
        "TAXLANE can model how ordinary individual income-tax receipts compare with broad federal outlay categories.".to_string(),
        "That model is a visibility tool, not a legal claim that a taxpayer's dollars are dedicated to a specific program.".to_string(),
        String::new(),
        "TAXLANE can also ask accountability questions about whether spending has reviewed performance evidence.".to_string(),
        "Current accountability records are not public findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Current Claim Guard".to_string(),
        String::new(),
        format!("- Accountability records reviewed for public use: {total_records}"),
        format!("- Records currently public-claim eligible: {public_claim_allowed}"),
        format!(
            "- Records still blocked from public claims: {}",
            total_records.saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Safe Public Questions".to_string(),
        String::new(),
        "| Lane | Question To Ask | Why It Matters |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use / Avoid".to_string(),
        String::new(),
        "| Use | Avoid |".to_string(),
        "|---|---|".to_string(),
        "| Ask for reviewed performance targets, outcome measures, audit sources, or role-approved wording. | Do not say TAXLANE found fraud, waste, abuse, or poor performance from these draft records. |".to_string(),
        "| Use modeled allocation language when explaining income-tax visibility. | Do not say ordinary income-tax dollars are legally dedicated to the displayed lanes. |".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_artifact_map() -> String {
    let rows = [
        (
            "accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl",
            "Internal evidence reviewers",
            "Validate source-custodied evidence shape.",
            "Do not publish as findings.",
        ),
        (
            "readiness-report.md",
            "Accountability researchers",
            "See readiness and next action per record.",
            "Do not treat readiness as a performance score.",
        ),
        (
            "action-queue.md",
            "Review leads",
            "Work records by next task.",
            "Do not publish queue rows as claims.",
        ),
        (
            "performance-demand-packet.md",
            "Accountability researchers",
            "Ask what evidence, reviewed wording, or official finding is missing.",
            "Do not allege misconduct.",
        ),
        (
            "accountability-work-items.jsonl",
            "Product implementers",
            "Feed future UI/API workflow from structured fields.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "claim-guard-report.md",
            "Review leads",
            "Check allowed versus blocked public claims.",
            "Do not publish findings from blocked records.",
        ),
        (
            "public-questions.md",
            "Citizen readers",
            "Ask safe public questions about performance evidence.",
            "Do not expose raw draft evidence as claims.",
        ),
        (
            "performance-demand-checklist.md",
            "Citizen readers",
            "Demand source, performance, official-finding, wording, and claim-gate evidence.",
            "Do not treat demand rows as findings.",
        ),
        (
            "performance-demand-dashboard.md",
            "Citizen readers",
            "Scan demand-row claim gates before public use.",
            "Do not publish blocked rows as claims.",
        ),
        (
            "performance-demand-brief.md",
            "Citizen readers",
            "Use a compact ask packet for current blocked demand rows.",
            "Do not present the brief as a finding or scorecard.",
        ),
        (
            "performance-demand-letter.md",
            "Citizen readers",
            "Adapt a public-safe evidence request template.",
            "Do not send it as an accusation or legal conclusion.",
        ),
        (
            "performance-demand-response-rubric.md",
            "Citizen readers",
            "Classify replies to evidence requests.",
            "Do not turn incomplete replies into findings.",
        ),
        (
            "performance-demand-followup.md",
            "Citizen readers",
            "Send a narrower follow-up for missing evidence.",
            "Do not escalate missing evidence into accusations.",
        ),
        (
            "performance-demand-response-log.md",
            "Citizen readers",
            "Track replies and remaining missing evidence.",
            "Do not treat log status as a finding.",
        ),
        (
            "performance-demand-response-log.jsonl",
            "Product implementers",
            "Feed neutral response log rows into future UI/API surfaces.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "performance-demand-response-log.schema.md",
            "Product implementers",
            "Inspect the response log row contract.",
            "Do not add UI/API fields that weaken the use rule.",
        ),
        (
            "performance-demand-response-status.json",
            "Product implementers",
            "Display response-log counts without recomputing rows.",
            "Do not treat status counts as findings.",
        ),
        (
            "performance-demand-response-dashboard.md",
            "Citizen readers",
            "Scan response-log counts without opening JSON.",
            "Do not treat dashboard counts as findings.",
        ),
        (
            "performance-demand-response-handoff.md",
            "Citizen readers / product implementers",
            "Choose the response tracking artifact for each task.",
            "Do not treat navigation guidance as findings.",
        ),
        (
            "performance-demand-response-intake.md",
            "Citizen readers / product implementers",
            "Capture reply evidence before updating response status.",
            "Do not treat unreviewed replies as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-intake.schema.md",
            "Product implementers",
            "Inspect the reply intake field contract.",
            "Do not add importer fields that bypass role review or claim gates.",
        ),
        (
            "performance-demand-response-intake.example.jsonl",
            "Product implementers",
            "Exercise the typed intake-to-log importer handoff.",
            "Do not treat example replies as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-log.applied-example.jsonl",
            "Product implementers",
            "Inspect response-log rows after applying example intake.",
            "Do not treat applied example rows as findings or claim eligibility.",
        ),
        (
            "performance-demand-response-status.applied-example.json",
            "Product implementers",
            "Display applied response-log counts without recomputing rows.",
            "Do not treat applied status counts as findings.",
        ),
        (
            "performance-demand-response-dashboard.applied-example.md",
            "Product implementers",
            "Scan applied response-log counts without opening JSON.",
            "Do not treat applied dashboard counts as findings.",
        ),
        (
            "performance-demand-response-handoff.applied-example.md",
            "Product implementers",
            "Route the response importer fixture artifacts by task.",
            "Do not treat applied handoff guidance as findings.",
        ),
        (
            "performance-demand-response-applied-example.schema.md",
            "Product implementers",
            "Inspect the applied importer fixture artifact contract.",
            "Do not weaken intake, log, status, or claim-gate guardrails.",
        ),
        (
            "performance-demand-response-delta.applied-example.md",
            "Product implementers",
            "Inspect exact row-level changes after applying example intake.",
            "Do not treat applied deltas as findings or canonical status.",
        ),
        (
            "performance-demand-response-delta.applied-example.jsonl",
            "Product implementers",
            "Feed exact applied response delta rows into future UI/API surfaces.",
            "Do not treat applied delta rows as findings or canonical status.",
        ),
        (
            "performance-demand-response-delta.applied-example.schema.md",
            "Product implementers",
            "Inspect the applied response delta row contract.",
            "Do not add UI/API fields that weaken fixture or claim-gate guardrails.",
        ),
        (
            "performance-demand-response-bundle.applied-example.md",
            "Product implementers",
            "Open one index for every applied response importer fixture artifact.",
            "Do not treat bundle membership as canonical response status or findings.",
        ),
        (
            "performance-demand-response-bundle.applied-example.json",
            "Product implementers",
            "Load the applied response importer fixture bundle without scraping Markdown.",
            "Do not treat manifest rows as canonical response status or findings.",
        ),
        (
            "performance-demand-response-bundle.applied-example.schema.md",
            "Product implementers",
            "Inspect the applied response bundle JSON contract.",
            "Do not add fields that weaken fixture-only or blocked-claim boundaries.",
        ),
        (
            "performance-demand-checklist.jsonl",
            "Product implementers",
            "Feed demand rows into future UI/API surfaces.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "performance-demand-claim-gates.json",
            "Product implementers",
            "Display allowed versus blocked demand-row counts.",
            "Do not recompute or override claim gates downstream.",
        ),
        (
            "performance-demand-checklist.schema.md",
            "Product implementers",
            "Inspect the demand checklist row contract.",
            "Do not add UI/API fields that weaken the use rule.",
        ),
        (
            "docs/reading/accountability-public-brief.md",
            "Citizen readers",
            "Read the current public handoff.",
            "Do not describe modeled allocation as legal dedication.",
        ),
    ];

    let mut lines = vec![
        "# Accountability Artifact Map".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This map shows which accountability artifact to use for evidence review, performance-demand questions, and public-safe reader handoff.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with the draft JSONL records for source custody.".to_string(),
        "2. Use readiness, queue, demand, work-item, and claim-guard artifacts for internal review workflow.".to_string(),
        "3. Use public questions and the public brief only for outward-facing questions and handoff wording.".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Artifact | Audience | Use | Avoid |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for (artifact, audience, use_case, avoid) in rows {
        lines.push(format!(
            "| `{artifact}` | {audience} | {use_case} | {avoid} |"
        ));
    }

    lines.extend([
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Public artifacts may ask for performance evidence and official findings. They must not claim fraud, waste, abuse, legal dedication of income taxes, or program performance without reviewed evidence and claim eligibility.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_checklist(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Checklist".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated checklist turns TAXLANE accountability blockers into evidence requests a citizen can make before accepting performance or misconduct claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Before Accepting A Claim".to_string(),
        String::new(),
        "- Ask for the source record and source version.".to_string(),
        "- Ask for the reviewed performance target, outcome measure, audit source, or official finding.".to_string(),
        "- Ask whether role review approved the exact public wording.".to_string(),
        "- Ask whether the record is public-claim eligible.".to_string(),
        String::new(),
        "## Record Checklist".to_string(),
        String::new(),
        "| Lane | Demand This Evidence | Do Not Accept Yet | Claim Gate |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        let claim_gate = if work_item.public_claim_allowed {
            PUBLIC_CLAIM_ALLOWED_LABEL
        } else {
            PUBLIC_CLAIM_BLOCKED_LABEL
        };
        lines.push(format!(
            "| {} | {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|"),
            claim_gate
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this checklist to demand performance evidence and reviewed wording. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = Vec::new();
    for record in records {
        let row = record.performance_demand_checklist_row();
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize demand checklist row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_claim_gates(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let rows: Vec<PerformanceDemandChecklistRecord> = records
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    let total_rows = rows.len();
    let public_claim_allowed = rows.iter().filter(|row| row.public_claim_allowed).count();
    let public_claim_blocked = total_rows.saturating_sub(public_claim_allowed);
    let mut gate_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &rows {
        *gate_counts.entry(&row.claim_gate).or_default() += 1;
    }
    let claim_gates: Vec<String> = gate_counts
        .into_iter()
        .map(|(claim_gate, rows)| {
            format!(
                "    {{\"claim_gate\":{},\"rows\":{rows}}}",
                json_string(claim_gate)
            )
        })
        .collect();

    Ok(format!(
        concat!(
            "{{\n",
            "  \"artifact\": {},\n",
            "  \"total_rows\": {},\n",
            "  \"public_claim_allowed\": {},\n",
            "  \"public_claim_blocked\": {},\n",
            "  \"claim_gates\": [\n",
            "{}\n",
            "  ],\n",
            "  \"use_rule\": {}\n",
            "}}\n"
        ),
        json_string(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH),
        total_rows,
        public_claim_allowed,
        public_claim_blocked,
        claim_gates.join(",\n"),
        json_string(
            "Demand evidence and reviewed wording; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance."
        )
    ))
}

pub(crate) fn build_accountability_performance_demand_dashboard(root: &Path) -> Result<String, String> {
    let claim_gates_text = build_accountability_performance_demand_claim_gates(root)?;
    let claim_gates: serde_json::Value =
        serde_json::from_str(&claim_gates_text).map_err(|err| {
            format!("failed to parse generated performance demand claim gates: {err}")
        })?;
    let total_rows = claim_gates
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing total_rows".to_string())?;
    let allowed_rows = claim_gates
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_allowed".to_string())?;
    let blocked_rows = claim_gates
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_blocked".to_string())?;
    let use_rule = claim_gates
        .get("use_rule")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "generated claim gates missing use_rule".to_string())?;

    let mut lines = vec![
        "# Performance Demand Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes whether performance demand checklist rows can support public claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, or poor performance.".to_string(),
        String::new(),
        "## Claim Gate Summary".to_string(),
        String::new(),
        format!("- Demand rows: {total_rows}"),
        format!("- Public claims currently allowed: {allowed_rows}"),
        format!("- Public claims currently blocked: {blocked_rows}"),
        String::new(),
        "## Claim Gates".to_string(),
        String::new(),
        "| Claim Gate | Rows |".to_string(),
        "|---|---:|".to_string(),
    ];

    let gate_rows = claim_gates
        .get("claim_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "generated claim gates missing claim_gates".to_string())?;
    for gate in gate_rows {
        let claim_gate = gate
            .get("claim_gate")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "generated claim gate row missing claim_gate".to_string())?;
        let rows = gate
            .get("rows")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| "generated claim gate row missing rows".to_string())?;
        lines.push(format!("| {} | {rows} |", claim_gate.replace('|', "\\|")));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        use_rule.to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Brief".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated brief turns blocked performance demand rows into a compact ask packet for citizen readers.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Current Claim Status".to_string(),
        String::new(),
        "TAXLANE currently has no performance demand rows that are public-claim eligible.".to_string(),
        "Use the rows below to ask for evidence, not to assert wrongdoing or performance failure.".to_string(),
        String::new(),
        "## Ask Packet".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("### {label}"),
            String::new(),
            format!("- Ask: {}", row.demand_question),
            format!("- Do not accept yet: {}", row.do_not_accept_yet),
            format!("- Claim gate: {}", row.claim_gate),
            format!("- Public claim allowed: {}", row.public_claim_allowed),
            "- Required evidence:".to_string(),
        ]);
        for evidence in row.demand_evidence {
            lines.push(format!("  - {evidence}"));
        }
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this brief to demand source records, reviewed performance evidence, official findings, role-approved wording, and public-claim eligibility. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_letter(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Letter Template".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated template helps a citizen ask for performance evidence behind public-money claims.".to_string(),
        "It is not an accusation, legal demand, fraud finding, waste finding, abuse finding, or performance scorecard.".to_string(),
        String::new(),
        "## Template".to_string(),
        String::new(),
        "Subject: Request for reviewed performance evidence and public-claim basis".to_string(),
        String::new(),
        "To [office or program contact],".to_string(),
        String::new(),
        "I am reviewing public-money claims with TAXLANE's modeled income-tax visibility materials. I understand those materials do not show legal dedication of income-tax dollars and do not, by themselves, prove fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "Before I accept or repeat a public claim, please provide the evidence listed below or identify where it is already published.".to_string(),
        String::new(),
        "Requested evidence:".to_string(),
        String::new(),
        "- Source record and source version.".to_string(),
        "- Reviewed performance target, outcome measure, audit source, or official finding.".to_string(),
        "- Exact public wording approved by role review, if any.".to_string(),
        "- Public-claim eligibility basis for any performance or misconduct statement.".to_string(),
        String::new(),
        "Current TAXLANE demand rows to resolve:".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("- {label}: {}", row.demand_question),
            format!("  Blocker: {}", row.do_not_accept_yet),
            format!("  Claim gate: {}", row.claim_gate),
        ]);
    }

    lines.extend([
        String::new(),
        "Please treat this as a request for evidence and reviewed wording, not as an allegation that misconduct or poor performance occurred.".to_string(),
        String::new(),
        "Sincerely,".to_string(),
        String::new(),
        "[name]".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Customize names, dates, and addressee details before use. Keep the modeled-not-legal tax boundary and no-finding language intact unless reviewed evidence and public-claim eligibility support a stronger statement.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_rubric(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Response Rubric".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated rubric helps classify replies to performance evidence requests.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Score Meanings".to_string(),
        String::new(),
        "| Response Class | Meaning | Next Action |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for response_class in PerformanceDemandResponseLogClass::rubric_classes() {
        lines.push(format!(
            "| {} | {} | {} |",
            response_class.label(),
            response_class.rubric_meaning(),
            response_class.rubric_next_action()
        ));
    }

    lines.extend([
        String::new(),
        "## Row-Specific Checks".to_string(),
        String::new(),
        "| Lane | Original Ask | Current Blocker | Response Must Provide |".to_string(),
        "|---|---|---|---|".to_string(),
    ]);

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.push(format!(
            "| {label} | {} | {} | source version; reviewed performance evidence or official finding; role-approved wording; public-claim basis |",
            row.demand_question.replace('|', "\\|"),
            row.do_not_accept_yet.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this rubric to decide what evidence is still missing after a reply. Do not use an incomplete, process-only, or no-evidence response to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_followup(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Follow-Up Template".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated template helps a citizen follow up when a reply is partial, process-only, or provides no evidence.".to_string(),
        "It is not an accusation, legal demand, fraud finding, waste finding, abuse finding, or performance scorecard.".to_string(),
        String::new(),
        "## Template".to_string(),
        String::new(),
        "Subject: Follow-up request for missing performance evidence".to_string(),
        String::new(),
        "To [office or program contact],".to_string(),
        String::new(),
        "Thank you for the response. I am treating it as an evidence response, not as proof of misconduct or poor performance.".to_string(),
        String::new(),
        "The reply appears to leave at least one requested item missing or unclear. Please provide the missing item, identify where it is published, or state that the office does not have it.".to_string(),
        String::new(),
        "Missing evidence to clarify:".to_string(),
        String::new(),
        "- Source record and source version, if not already provided.".to_string(),
        "- Reviewed performance target, outcome measure, audit source, or official finding, if not already provided.".to_string(),
        "- Exact role-approved public wording, if any.".to_string(),
        "- Public-claim eligibility basis for any performance or misconduct statement.".to_string(),
        String::new(),
        "Current unresolved TAXLANE demand rows:".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("- {label}: {}", row.do_not_accept_yet),
            format!("  Follow-up ask: {}", row.demand_question),
            format!("  Claim gate remains: {}", row.claim_gate),
        ]);
    }

    lines.extend([
        String::new(),
        "Please keep this as an evidence clarification request. If the evidence does not exist or is not yet reviewed, a clear statement of that status is useful.".to_string(),
        String::new(),
        "Sincerely,".to_string(),
        String::new(),
        "[name]".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this follow-up only after a reply leaves requested evidence missing or unclear. Do not use a missing or incomplete reply to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_log(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Response Log".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated log gives each current performance demand row a neutral place to track replies and remaining evidence gaps.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseLogClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.rubric_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Current Log".to_string(),
        String::new(),
        "| Lane | Response Class | Evidence Received | Missing Evidence | Claim Gate | Next Action |"
            .to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ]);

    for record in records {
        let row = record.performance_demand_response_log_record();
        row.validate()?;
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        let evidence_received = if row.evidence_received.is_empty() {
            "none logged".to_string()
        } else {
            row.evidence_received.join("; ")
        };
        lines.push(format!(
            "| {label} | `{}` | {} | {} | {} | {} |",
            row.response_class.wire_value(),
            evidence_received.replace('|', "\\|"),
            row.missing_evidence.replace('|', "\\|"),
            row.claim_gate.replace('|', "\\|"),
            row.next_action.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this log to track response status and remaining evidence gaps. Do not use an empty, partial, process-only, or no-evidence log row to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_log_jsonl(
    root: &Path,
) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = Vec::new();
    for record in records {
        let row = record.performance_demand_response_log_record();
        row.validate()?;
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize response log row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_log_schema() -> String {
    let mut lines = vec![
        "# Performance Demand Response Log JSONL Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents the generated `performance-demand-response-log.jsonl` rows."
            .to_string(),
        "Rows track replies to evidence requests without changing claim gates or creating findings."
            .to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID. |".to_string(),
        "| `lane_id` | string or null | conditional | Public-purpose lane when available. |"
            .to_string(),
        "| `program_or_account_id` | string or null | conditional | Program, account, or OMB function identifier when available. |".to_string(),
        "| `response_class` | string | yes | Current response status. Initial generated value is `not-yet-received`. |".to_string(),
        "| `evidence_received` | array of strings | yes | Evidence items logged from a reply. Initial generated value is empty. |".to_string(),
        "| `missing_evidence` | string | yes | Current blocker or missing evidence item. |"
            .to_string(),
        "| `claim_gate` | string | yes | Human-readable claim-gate label. Initial generated value is `Public claim blocked.` |".to_string(),
        "| `public_claim_allowed` | boolean | yes | Explicit claim gate for public use. Initial generated value is `false`. |".to_string(),
        "| `next_action` | string | yes | Safe next workflow action. |".to_string(),
        "| `use_rule` | string | yes | Boundary rule for using the row. |".to_string(),
        String::new(),
        "At least one of `lane_id` or `program_or_account_id` must be present.".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseLogClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.rubric_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- `evidence_received` must be non-empty when `response_class` is `complete-evidence-response` or `partial-evidence-response`.".to_string(),
        "- `evidence_received` must be empty when `response_class` is `not-yet-received`, `process-only-response`, or `no-evidence-response`.".to_string(),
        "- `public_claim_allowed` must remain `false` unless a separate reviewed evidence record and public-claim gate allow a public statement.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Rows may support response tracking. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_status(root: &Path) -> Result<String, String> {
    let response_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let rows: Vec<PerformanceDemandResponseLogRecord> = response_log
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| format!("failed to parse generated response log row: {err}"))
        })
        .collect::<Result<_, _>>()?;
    let status = PerformanceDemandResponseStatus::from_response_log_records(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH,
        &rows,
    )?;
    status.validate()?;
    serde_json::to_string_pretty(&status)
        .map(|text| text + "\n")
        .map_err(|err| format!("failed to serialize response status: {err}"))
}

pub(crate) fn build_accountability_performance_demand_response_dashboard(
    root: &Path,
) -> Result<String, String> {
    let status = generated_accountability_performance_demand_response_status(root)?;

    let lines = vec![
        "# Performance Demand Response Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes response-log status for performance demand rows."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Response Status Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_handoff(root: &Path) -> Result<String, String> {
    let status = generated_accountability_performance_demand_response_status(root)?;

    let lines = vec![
        "# Performance Demand Response Handoff".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated handoff routes readers and implementers through response tracking artifacts."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with `performance-demand-response-dashboard.md` to scan response counts."
            .to_string(),
        "2. Use `performance-demand-response-log.md` to track current reply status and missing evidence.".to_string(),
        "3. Use `performance-demand-response-rubric.md` to classify replies as complete, partial, process-only, or no-evidence.".to_string(),
        "4. Use `performance-demand-followup.md` when a reply leaves requested evidence missing or unclear.".to_string(),
        "5. Use `performance-demand-response-intake.md` when a real reply arrives and source custody must be captured before updating the log.".to_string(),
        "6. Use `performance-demand-response-log.jsonl`, `performance-demand-response-log.schema.md`, and `performance-demand-response-status.json` for UI/API consumers.".to_string(),
        String::new(),
        "## Current Status".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Track response status and missing evidence; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_intake() -> String {
    let mut lines = vec![
        "# Performance Demand Response Intake".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "Use this generated intake template when a reply arrives for a performance demand."
            .to_string(),
        "It records source custody and classification inputs before any response-log row is updated."
            .to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Required Capture".to_string(),
        String::new(),
        "| Field | Capture Rule |".to_string(),
        "|---|---|".to_string(),
        "| `record_id` | Copy from `performance-demand-response-log.jsonl`. |".to_string(),
        "| `reply_source_id` | Assign or cite a source-ledger ID for the reply artifact. |".to_string(),
        "| `reply_received_date` | Record the received date as `YYYY-MM-DD`. |".to_string(),
        "| `sender_or_office` | Name the responding office or official exactly as written. |"
            .to_string(),
        "| `response_class` | Choose one class from the response log schema. |".to_string(),
        "| `evidence_received` | List concrete documents, datasets, citations, or official findings supplied by the reply. |".to_string(),
        "| `missing_evidence` | State the remaining missing source, performance, wording, or claim-basis evidence. |".to_string(),
        "| `role_review_needed` | Keep `true` until role review approves exact public wording. |".to_string(),
        "| `public_claim_allowed` | Keep `false` unless the claim gate is explicitly revalidated. |".to_string(),
        String::new(),
        "## Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseClass::all_classes() {
        lines.push(format!(
            "- `{}`: {}",
            response_class.wire_value(),
            response_class.intake_meaning()
        ));
    }

    lines.extend([
        String::new(),
        "## Update Rule".to_string(),
        String::new(),
        "After intake, update `performance-demand-response-log.jsonl` only with source-custodied reply evidence and rerun validation.".to_string(),
        "Do not convert a reply into a fraud, waste, abuse, legal dedication, poor performance, or reform-benefit claim without reviewed evidence and an explicit public-claim gate.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_intake_schema() -> String {
    let mut lines = vec![
        "# Performance Demand Response Intake Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents the fields a future UI/API importer should capture from `performance-demand-response-intake.md`.".to_string(),
        "It defines an intake contract only; it does not authorize public claims or response-log updates without validation.".to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID copied from the response log row. |".to_string(),
        "| `reply_source_id` | string | yes | Source-ledger identifier or custody pointer for the received reply artifact. |".to_string(),
        "| `reply_received_date` | string | yes | ISO date (`YYYY-MM-DD`) when the reply was received. |".to_string(),
        "| `sender_or_office` | string | yes | Responding office or official exactly as written in the reply. |".to_string(),
        "| `response_class` | string | yes | One allowed response class from this schema. |".to_string(),
        "| `evidence_received` | array of strings | yes | Concrete documents, datasets, citations, or official findings supplied by the reply. |".to_string(),
        "| `missing_evidence` | string | yes | Remaining source, performance, wording, or claim-basis evidence gap. |".to_string(),
        "| `role_review_needed` | boolean | yes | Must remain `true` until exact public wording receives role review. |".to_string(),
        "| `public_claim_allowed` | boolean | yes | Must remain `false` unless claim gates are explicitly revalidated. |".to_string(),
        "| `use_rule` | string | yes | Boundary rule for using the intake row. |".to_string(),
        String::new(),
        "## Allowed Response Classes".to_string(),
        String::new(),
    ];

    for response_class in PerformanceDemandResponseClass::all_classes() {
        lines.push(format!("- `{}`", response_class.wire_value()));
    }

    lines.extend([
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- `role_review_needed` must be `true` for unreviewed replies.".to_string(),
        "- `public_claim_allowed` must be `false` until the response log, role review, and claim gates are revalidated.".to_string(),
        "- `evidence_received` must be non-empty when `response_class` is `complete-evidence-response` or `partial-evidence-response`.".to_string(),
        "- `evidence_received` must be empty when `response_class` is `process-only-response` or `no-evidence-response`.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
    ]);

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_intake_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let record = records.first().ok_or_else(|| {
        "cannot build performance demand response intake example without accountability records"
            .to_string()
    })?;

    let intake = PerformanceDemandResponseIntakeRecord {
        record_id: record.record_id.clone(),
        reply_source_id: "SRC-REPLY-EXAMPLE".to_string(),
        reply_received_date: "2026-06-23".to_string(),
        sender_or_office: "Example program office".to_string(),
        response_class: PerformanceDemandResponseClass::PartialEvidenceResponse,
        evidence_received: vec![
            "Example reply cites a source record and a performance target.".to_string(),
        ],
        missing_evidence: "Role-approved public wording and public-claim basis remain missing."
            .to_string(),
        role_review_needed: true,
        public_claim_allowed: false,
        use_rule: PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
    };
    intake.validate()?;

    serde_json::to_string(&intake)
        .map(|line| format!("{line}\n"))
        .map_err(|err| format!("failed to serialize response intake example row: {err}"))
}

pub(crate) fn build_accountability_performance_demand_response_log_applied_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let response_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let intake_jsonl = build_accountability_performance_demand_response_intake_example_jsonl(root)?;
    let mut log_rows: BTreeMap<String, PerformanceDemandResponseLogRecord> = response_log
        .lines()
        .map(|line| {
            let record: PerformanceDemandResponseLogRecord = serde_json::from_str(line)
                .map_err(|err| format!("failed to parse generated response log row: {err}"))?;
            Ok((record.record_id.clone(), record))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;

    for line in intake_jsonl.lines() {
        let intake: PerformanceDemandResponseIntakeRecord = serde_json::from_str(line)
            .map_err(|err| format!("failed to parse generated intake example row: {err}"))?;
        let log_record = log_rows.remove(&intake.record_id).ok_or_else(|| {
            format!(
                "response intake example row has no matching response log row: {}",
                intake.record_id
            )
        })?;
        let updated = log_record.apply_intake(&intake)?;
        log_rows.insert(updated.record_id.clone(), updated);
    }

    let mut lines = Vec::new();
    for row in log_rows.values() {
        row.validate()?;
        lines.push(
            serde_json::to_string(row)
                .map_err(|err| format!("failed to serialize applied response log row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_status_applied_example(
    root: &Path,
) -> Result<String, String> {
    let applied_log =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    let rows: Vec<PerformanceDemandResponseLogRecord> = applied_log
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .map_err(|err| format!("failed to parse applied response log row: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let status = PerformanceDemandResponseStatus::from_response_log_records(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
        &rows,
    )?;
    status.validate()?;

    serde_json::to_string_pretty(&status)
        .map(|text| format!("{text}\n"))
        .map_err(|err| format!("failed to serialize applied response status: {err}"))
}

pub(crate) fn build_accountability_performance_demand_response_dashboard_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes the importer fixture after applying example intake rows.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Applied Response Status Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Fixture Boundary".to_string(),
        String::new(),
        "Use this dashboard to inspect importer behavior only. Do not treat applied example rows as canonical response status or public-claim eligibility.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_handoff_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Handoff".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated handoff routes implementers through the response importer fixture artifacts.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with `performance-demand-response-intake.example.jsonl` to inspect a source-custodied intake row.".to_string(),
        "2. Use `performance-demand-response-log.applied-example.jsonl` to inspect typed response-log rows after intake application.".to_string(),
        "3. Use `performance-demand-response-status.applied-example.json` when a UI/API needs compact applied counts.".to_string(),
        "4. Use `performance-demand-response-dashboard.applied-example.md` for quick human inspection of importer behavior.".to_string(),
        "5. Use `performance-demand-response-delta.applied-example.md` to inspect row-level changes without opening JSONL.".to_string(),
        "6. Use `performance-demand-response-delta.applied-example.jsonl` and `performance-demand-response-delta.applied-example.schema.md` for UI/API delta consumers.".to_string(),
        "7. Use `performance-demand-response-bundle.applied-example.md` to inspect the complete applied fixture bundle.".to_string(),
        "8. Use `performance-demand-response-bundle.applied-example.json` and `performance-demand-response-bundle.applied-example.schema.md` for UI/API bundle consumers.".to_string(),
        String::new(),
        "## Applied Fixture Status".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Applied example artifacts are importer fixtures, not canonical response status. Do not use them as public-claim eligibility, misconduct findings, or performance findings.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_bundle_applied_example(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    status.validate()?;
    let updated_rows = status.total_rows.saturating_sub(status.not_yet_received);

    let lines = vec![
        "# Performance Demand Response Applied Example Bundle".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated bundle index gives importer and UI/API consumers one place to find every applied response fixture artifact.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Applied Fixture Summary".to_string(),
        String::new(),
        format!("- Response rows: {}", status.total_rows),
        format!("- Updated rows: {updated_rows}"),
        format!("- Not-yet-received rows: {}", status.not_yet_received),
        format!(
            "- Public claims currently allowed: {}",
            status.public_claim_allowed
        ),
        format!(
            "- Public claims currently blocked: {}",
            status.public_claim_blocked
        ),
        String::new(),
        "## Bundle Artifacts".to_string(),
        String::new(),
        "| Artifact | Role | Consumer Use |".to_string(),
        "|---|---|---|".to_string(),
        "| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row. | Exercise importer parsing and record-id matching. |".to_string(),
        "| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying example intake. | Inspect typed applied rows without changing canonical response status. |".to_string(),
        "| `performance-demand-response-status.applied-example.json` | Compact applied response counts. | Feed fixture counts into UI/API tests without recomputing rows. |".to_string(),
        "| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied response counts. | Scan importer behavior without opening JSON. |".to_string(),
        "| `performance-demand-response-handoff.applied-example.md` | Task routing for the applied fixture set. | Choose the right applied artifact by implementation task. |".to_string(),
        "| `performance-demand-response-applied-example.schema.md` | Fixture artifact contract. | Confirm roles and guardrails for applied importer artifacts. |".to_string(),
        "| `performance-demand-response-delta.applied-example.md` | Human-readable changed fields. | Inspect row-level changes after applying example intake. |".to_string(),
        "| `performance-demand-response-delta.applied-example.jsonl` | Machine-readable changed fields. | Feed delta rows into UI/API diff consumers. |".to_string(),
        "| `performance-demand-response-delta.applied-example.schema.md` | Delta row field contract. | Confirm field meanings and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.json` | Machine-readable bundle manifest. | Load fixture artifact roles, row counts, hashes, and boundaries without scraping Markdown. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.schema.md` | Bundle manifest field contract. | Inspect manifest and artifact field meanings. |".to_string(),
        String::new(),
        "## Boundary".to_string(),
        String::new(),
        "Bundle artifacts are importer fixtures, not canonical response status. Do not use them as public-claim eligibility, misconduct findings, performance findings, or proof of reform benefits.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        status.use_rule,
    ];

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_bundle_applied_example_json(
    root: &Path,
) -> Result<String, String> {
    let status_text =
        build_accountability_performance_demand_response_status_applied_example(root)?;
    let status: PerformanceDemandResponseStatus = serde_json::from_str(&status_text)
        .map_err(|err| format!("failed to parse applied response status: {err}"))?;
    let manifest = PerformanceDemandResponseBundleManifest::from_status(
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH,
        &status,
        performance_demand_response_bundle_artifacts(root)?,
    )?;
    serde_json::to_string_pretty(&manifest)
        .map(|text| text + "\n")
        .map_err(|err| format!("failed to serialize applied response bundle manifest: {err}"))
}

pub(crate) fn build_accountability_performance_demand_response_bundle_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Bundle Applied Example JSON Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents `performance-demand-response-bundle.applied-example.json` fields.".to_string(),
        "The JSON is serialized from `PerformanceDemandResponseBundleManifest` and lists `PerformanceDemandResponseBundleArtifact` rows for importer and UI/API consumers.".to_string(),
        String::new(),
        "## Manifest Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `artifact` | string | yes | Repo-relative path for this bundle manifest JSON. |".to_string(),
        "| `bundle_kind` | string | yes | Fixed value `applied-response-importer-fixture`. |".to_string(),
        "| `total_rows` | integer | yes | Applied response-log row count from the fixture status. |".to_string(),
        "| `updated_rows` | integer | yes | Rows changed by applying example intake. Must not exceed `total_rows`. |".to_string(),
        "| `public_claim_allowed` | integer | yes | Must remain `0` for the applied fixture bundle. |".to_string(),
        "| `public_claim_blocked` | integer | yes | Blocked public-claim row count. With allowed count, must sum to `total_rows`. |".to_string(),
        "| `artifacts` | array | yes | Ordered applied fixture artifact entries. Must include intake, applied log, applied status, dashboard, handoff, applied schema, delta Markdown, delta JSONL, and delta schema artifacts. |".to_string(),
        "| `boundary` | string | yes | Fixture-only boundary statement. |".to_string(),
        "| `use_rule` | string | yes | Response tracking use rule; must match the core response-log use rule. |".to_string(),
        String::new(),
        "## Artifact Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `artifact` | string | yes | Repo-relative artifact path using forward slashes. |".to_string(),
        "| `role` | string | yes | Artifact role in the applied fixture bundle. |".to_string(),
        "| `kind` | string | yes | One of `jsonl`, `json`, or `markdown`. |".to_string(),
        "| `row_count` | string | yes | JSONL row count as a string, or `n/a` for non-JSONL artifacts. |".to_string(),
        "| `sha256` | string | yes | SHA-256 digest of the generated artifact bytes. |".to_string(),
        "| `consumer_use` | string | yes | Intended importer or UI/API use. |".to_string(),
        String::new(),
        "## Validation Rules".to_string(),
        String::new(),
        "- JSON must deserialize as `PerformanceDemandResponseBundleManifest`.".to_string(),
        "- Every artifact entry must validate as `PerformanceDemandResponseBundleArtifact`.".to_string(),
        "- The manifest must include all required applied fixture artifacts.".to_string(),
        "- Every artifact entry must include row-count and SHA-256 integrity metadata.".to_string(),
        "- Public claims must remain blocked for this fixture manifest.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "The manifest is fixture metadata only. It must not be used as canonical response status, public-claim eligibility, a finding of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Applied Example Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema note documents the generated response importer fixture artifacts.".to_string(),
        "It does not authorize public claims, findings, or canonical response-log updates.".to_string(),
        String::new(),
        "## Artifact Roles".to_string(),
        String::new(),
        "| Artifact | Role | Guardrail |".to_string(),
        "|---|---|---|".to_string(),
        "| `performance-demand-response-intake.example.jsonl` | Source-custodied intake fixture row parsed as `PerformanceDemandResponseIntakeRecord`. | Must keep `role_review_needed: true`, `public_claim_allowed: false`, and the intake use rule. |".to_string(),
        "| `performance-demand-response-log.applied-example.jsonl` | Response-log rows after applying intake fixture rows through `PerformanceDemandResponseLogRecord::apply_intake`. | Must validate as response-log records and keep `Public claim blocked.`. |".to_string(),
        "| `performance-demand-response-status.applied-example.json` | Compact counts aggregated from applied response-log rows through `PerformanceDemandResponseStatus`. | Must report zero allowed public claims and at least one updated row. |".to_string(),
        "| `performance-demand-response-dashboard.applied-example.md` | Human-readable applied status summary. | Must state fixture-only and no-finding boundaries. |".to_string(),
        "| `performance-demand-response-handoff.applied-example.md` | Task routing for importer fixture consumers. | Must not describe applied examples as canonical status or public-claim eligibility. |".to_string(),
        "| `performance-demand-response-delta.applied-example.md` | Row-level comparison between canonical response-log rows and applied example rows. | Must show changed fields while preserving blocked public-claim gates. |".to_string(),
        "| `performance-demand-response-delta.applied-example.jsonl` | Machine-readable delta rows serialized from `PerformanceDemandResponseDeltaRow`. | Must validate as core delta rows and preserve blocked public-claim gates. |".to_string(),
        "| `performance-demand-response-delta.applied-example.schema.md` | Field contract for machine-readable applied delta rows. | Must preserve fixture-only and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.md` | Human-readable index for the complete applied fixture bundle. | Must preserve fixture-only and blocked-claim guardrails. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.json` | Machine-readable bundle manifest serialized from `PerformanceDemandResponseBundleManifest`. | Must validate through core and keep public claims blocked. |".to_string(),
        "| `performance-demand-response-bundle.applied-example.schema.md` | Field contract for the machine-readable bundle manifest. | Must document manifest fields and blocked-claim guardrails. |".to_string(),
        String::new(),
        "## Importer Rule".to_string(),
        String::new(),
        "Importers may use these artifacts to test response intake handling. They must not treat example rows as real agency replies, public fraud/waste/abuse findings, legal dedication of income taxes, poor-performance findings, or reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_delta_applied_example_schema() -> String {
    let lines = vec![
        "# Performance Demand Response Delta Applied Example JSONL Schema".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This schema documents `performance-demand-response-delta.applied-example.jsonl` rows.".to_string(),
        "Rows are generated from `PerformanceDemandResponseDeltaRow` to show importer fixture changes without creating findings.".to_string(),
        String::new(),
        "## Row Fields".to_string(),
        String::new(),
        "| Field | Type | Required | Meaning |".to_string(),
        "|---|---|---|---|".to_string(),
        "| `record_id` | string | yes | Accountability evidence record ID for the changed response row. |".to_string(),
        "| `before_response_class` | string | yes | Response-log class before applying the intake fixture. |".to_string(),
        "| `after_response_class` | string | yes | Response-log class after applying the intake fixture. |".to_string(),
        "| `before_evidence_received_count` | integer | yes | Count of evidence items before applying intake. |".to_string(),
        "| `after_evidence_received_count` | integer | yes | Count of evidence items after applying intake. |".to_string(),
        "| `missing_evidence_changed` | boolean | yes | Whether the missing-evidence text changed. |".to_string(),
        "| `next_action_changed` | boolean | yes | Whether the next-action text changed. |".to_string(),
        "| `before_claim_gate` | string | yes | Claim-gate label before applying intake. Must remain `Public claim blocked.`. |".to_string(),
        "| `after_claim_gate` | string | yes | Claim-gate label after applying intake. Must remain `Public claim blocked.`. |".to_string(),
        String::new(),
        "## Gate Rules".to_string(),
        String::new(),
        "- Rows must validate through `PerformanceDemandResponseDeltaRow`.".to_string(),
        "- Both claim-gate fields must remain `Public claim blocked.`.".to_string(),
        "- Rows describe fixture deltas only; they are not canonical response status.".to_string(),
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Rows may support importer and UI/API testing. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ];

    lines.join("\n") + "\n"
}

pub(crate) fn build_accountability_performance_demand_response_delta_applied_example_jsonl(
    root: &Path,
) -> Result<String, String> {
    let rows = build_accountability_performance_demand_response_delta_rows(root)?;
    let mut lines = Vec::new();
    for row in rows {
        row.validate()?;
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize applied response delta row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_delta_applied_example(
    root: &Path,
) -> Result<String, String> {
    let changed_rows = build_accountability_performance_demand_response_delta_rows(root)?;
    let canonical_rows = parse_response_log_jsonl(
        &build_accountability_performance_demand_response_log_jsonl(root)?,
        "canonical response log",
    )?;
    let applied_rows = parse_response_log_jsonl(
        &build_accountability_performance_demand_response_log_applied_example_jsonl(root)?,
        "applied response log",
    )?;

    let mut lines = vec![
        "# Performance Demand Response Applied Example Delta".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated delta compares canonical response-log rows with the importer fixture after applying example intake rows.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- Canonical rows: {}", canonical_rows.len()),
        format!("- Applied rows: {}", applied_rows.len()),
        format!("- Updated rows: {}", changed_rows.len()),
        String::new(),
        "## Row Changes".to_string(),
        String::new(),
        "| Record ID | Before response class | After response class | Evidence received change | Missing evidence change | Next action change | Claim gate |".to_string(),
        "|---|---|---|---|---|---|---|".to_string(),
    ];

    for row in changed_rows {
        row.validate()?;
        lines.push(format!(
            "| `{}` | `{}` | `{}` | {} -> {} item(s) | {} | {} | {} -> {} |",
            escape_table_cell(&row.record_id),
            row.before_response_class.wire_value(),
            row.after_response_class.wire_value(),
            row.before_evidence_received_count,
            row.after_evidence_received_count,
            bool_marker(row.missing_evidence_changed),
            bool_marker(row.next_action_changed),
            escape_table_cell(&row.before_claim_gate),
            escape_table_cell(&row.after_claim_gate),
        ));
    }

    lines.extend([
        String::new(),
        "## Fixture Boundary".to_string(),
        String::new(),
        "Use this delta to inspect importer behavior only. Do not treat changed fixture rows as canonical response status, public-claim eligibility, misconduct findings, performance findings, or reform benefits.".to_string(),
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Applied response deltas are implementation fixtures. Public wording must keep source custody, role review, public-claim gates, and no-finding boundaries intact.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

pub(crate) fn build_accountability_performance_demand_response_delta_rows(
    root: &Path,
) -> Result<Vec<PerformanceDemandResponseDeltaRow>, String> {
    let canonical_log = build_accountability_performance_demand_response_log_jsonl(root)?;
    let applied_log =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    let canonical_rows = parse_response_log_jsonl(&canonical_log, "canonical response log")?;
    let applied_rows = parse_response_log_jsonl(&applied_log, "applied response log")?;
    PerformanceDemandResponseDeltaRow::from_response_log_records(&canonical_rows, &applied_rows)
}

pub(crate) fn build_manifest(root: &Path) -> Result<String, String> {
    let metadata: Vec<ArtifactMetadata<'_>> = ARTIFACTS.iter().map(Artifact::metadata).collect();
    taxlane_core::validate_artifact_metadata(&metadata)?;

    let mut rows = Vec::new();
    for artifact in ARTIFACTS {
        let path = root.join(artifact.path);
        if !path.exists() {
            return Err(format!("missing artifact: {}", artifact.path));
        }
        rows.push((
            artifact,
            count_rows(&path, artifact.kind)?,
            sha256_file(&path)?,
        ));
    }

    let mut lines = vec![
        "# Income-Tax Outlay Model Artifact Manifest".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This manifest records the artifact chain for modeled allocations of".to_string(),
        "ordinary individual income-tax receipts by OMB outlay share.".to_string(),
        String::new(),
        "The annual, decade, and subfunction JSONL files are canonical model".to_string(),
        "outputs. CSV files, Markdown notes, and chart specs are derived or".to_string(),
        "supporting views.".to_string(),
        String::new(),
        "## Model".to_string(),
        String::new(),
        "- Broad model ID: `individual-income-tax-proportional-outlays-v1`".to_string(),
        "- Subfunction model ID: `individual-income-tax-proportional-subfunction-outlays-v1`"
            .to_string(),
        "- Broad coverage: fiscal years 1940-2025 for annual actual-year rows".to_string(),
        "- Subfunction coverage: fiscal years 1962-2025 for Table 3.2 actual-year rows".to_string(),
        "- Projection treatment: FY2026-FY2031 excluded".to_string(),
        "- Legal status: modeled allocation, not legal dedication".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Path | Role | Grain | Rows | Canonical | SHA-256 |".to_string(),
        "|---|---|---|---:|---|---|".to_string(),
    ];

    for (artifact, rows, sha) in rows {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} | `{}` |",
            artifact.path, artifact.role, artifact.grain, rows, artifact.canonical, sha
        ));
    }

    lines.extend([
        String::new(),
        "## Regeneration Order".to_string(),
        String::new(),
        "1. `cargo run -p taxlane-tools -- income-tax-outlay model`".to_string(),
        "2. `cargo run -p taxlane-tools -- income-tax-outlay summary`".to_string(),
        "3. `cargo run -p taxlane-tools -- income-tax-outlay export`".to_string(),
        "4. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model`".to_string(),
        "5. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export`".to_string(),
        "6. `cargo run -p taxlane-tools -- income-tax-outlay manifest`".to_string(),
        String::new(),
        "Run validation after regeneration:".to_string(),
        String::new(),
        "```powershell".to_string(),
        "cargo run -p taxlane-tools -- income-tax-outlay validate".to_string(),
        "```".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

