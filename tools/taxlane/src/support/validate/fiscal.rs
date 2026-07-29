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

pub(crate) fn validate_fiscal_country_panel(root: &Path) -> Result<(), String> {
    for (path, checksum) in [
        (IMF_FISCAL_PANEL_RAW_PATH, IMF_FISCAL_PANEL_RAW_SHA256),
        (
            OECD_TOTAL_TAX_REVENUE_RAW_PATH,
            OECD_TOTAL_TAX_REVENUE_RAW_SHA256,
        ),
        (OECD_TAX_MIX_RAW_PATH, OECD_TAX_MIX_RAW_SHA256),
        (OECD_GDP_RAW_PATH, OECD_GDP_RAW_SHA256),
        (OECD_GOV_INTEREST_RAW_PATH, OECD_GOV_INTEREST_RAW_SHA256),
        (
            OECD_GOV_NET_INTEREST_RAW_PATH,
            OECD_GOV_NET_INTEREST_RAW_SHA256,
        ),
    ] {
        let raw = root.join(path);
        if !raw.exists() || sha256_file(&raw)? != checksum {
            return Err(format!("fiscal country panel raw custody failed: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(FISCAL_COUNTRY_PANEL_JSON_PATH))
        .map_err(|err| format!("failed to read {FISCAL_COUNTRY_PANEL_JSON_PATH}: {err}"))?;
    let panel: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse {FISCAL_COUNTRY_PANEL_JSON_PATH}: {err}"))?;
    if string_field(&panel, "record_family")? != "fiscal_country_comparison"
        || number_field(&panel, "data_year")? != 2022.0
        || number_field(&panel, "country_count")? != 12.0
        || !string_field(&panel, "interest_boundary")?
            .contains("Primary minus overall balance is never used as interest")
        || !string_field(&panel, "interest_boundary")?
            .contains("Gross and net interest are never substituted")
        || string_field(&panel, "debt_boundary")?
            != "Gross and net debt are parallel measures and are never substituted for each other"
        || string_field(&panel, "ranking_status")? != "blocked"
        || string_field(&panel, "savings_status")? != "blocked_not_scored"
    {
        return Err("fiscal country panel identity, debt boundary, or gates failed".to_string());
    }
    let source_ids: BTreeSet<String> = panel
        .get("source_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("fiscal country panel needs source_ids")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| "fiscal country panel source_id must be a string".to_string())
        })
        .collect::<Result<_, _>>()?;
    let required_source_ids: BTreeSet<String> = [
        "SRC-IMF-FM-OCT2025-FISCAL-PANEL-2022".to_string(),
        "SRC-OECD-REVSTATS-PANEL-2022".to_string(),
        "SRC-OECD-GOV-INTEREST-PANEL-2022".to_string(),
        "SRC-OECD-GOV-NET-INTEREST-PANEL-2022".to_string(),
    ]
    .into_iter()
    .collect();
    if source_ids != required_source_ids {
        return Err("fiscal country panel required source set failed".to_string());
    }

    let records = panel
        .get("country_records")
        .and_then(serde_json::Value::as_array)
        .ok_or("fiscal country panel needs country_records")?;
    if records.len() != 12 {
        return Err(format!(
            "fiscal country panel must contain 12 records, got {}",
            records.len()
        ));
    }

    let expected_countries: BTreeSet<String> = [
        "USA", "DEU", "FRA", "GBR", "SWE", "NLD", "POL", "JPN", "KOR", "CAN", "AUS", "SGP",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let core_countries: BTreeSet<String> = expected_countries
        .iter()
        .filter(|country| country.as_str() != "SGP")
        .cloned()
        .collect();

    let mut gdp_by_country = BTreeMap::new();
    let mut gdp_reader = csv::Reader::from_path(root.join(OECD_GDP_RAW_PATH))
        .map_err(|err| format!("failed to read {OECD_GDP_RAW_PATH}: {err}"))?;
    for result in gdp_reader.deserialize::<BTreeMap<String, String>>() {
        let row = result.map_err(|err| format!("failed to parse {OECD_GDP_RAW_PATH}: {err}"))?;
        let country = row
            .get("REF_AREA")
            .ok_or("OECD GDP raw row needs REF_AREA")?
            .to_string();
        if row.get("TRANSACTION").map(String::as_str) != Some("B1GQ")
            || row.get("TIME_PERIOD").map(String::as_str) != Some("2022")
            || row.get("UNIT_MEASURE").map(String::as_str) != Some("XDC")
            || row.get("UNIT_MULT").map(String::as_str) != Some("6")
        {
            return Err(format!("invalid OECD GDP denominator row for {country}"));
        }
        let value = row
            .get("OBS_VALUE")
            .ok_or_else(|| format!("OECD GDP row {country} needs OBS_VALUE"))?
            .parse::<f64>()
            .map_err(|err| format!("OECD GDP row {country} value failed to parse: {err}"))?;
        if gdp_by_country.insert(country.clone(), value).is_some() {
            return Err(format!("duplicate OECD GDP row for {country}"));
        }
    }
    if gdp_by_country.keys().cloned().collect::<BTreeSet<_>>() != core_countries {
        return Err("OECD GDP denominator must contain the exact 11 core countries".to_string());
    }

    let mut interest_by_country = BTreeMap::new();
    let mut interest_reader = csv::Reader::from_path(root.join(OECD_GOV_INTEREST_RAW_PATH))
        .map_err(|err| format!("failed to read {OECD_GOV_INTEREST_RAW_PATH}: {err}"))?;
    for result in interest_reader.deserialize::<BTreeMap<String, String>>() {
        let row =
            result.map_err(|err| format!("failed to parse {OECD_GOV_INTEREST_RAW_PATH}: {err}"))?;
        let country = row
            .get("REF_AREA")
            .ok_or("OECD interest raw row needs REF_AREA")?
            .to_string();
        if row.get("TRANSACTION").map(String::as_str) != Some("D41")
            || row.get("SECTOR").map(String::as_str) != Some("S13")
            || row.get("ACCOUNTING_ENTRY").map(String::as_str) != Some("D")
            || row.get("TIME_PERIOD").map(String::as_str) != Some("2022")
            || row.get("UNIT_MEASURE").map(String::as_str) != Some("XDC")
            || row.get("UNIT_MULT").map(String::as_str) != Some("6")
        {
            return Err(format!("invalid OECD D41 interest row for {country}"));
        }
        let value = row
            .get("OBS_VALUE")
            .ok_or_else(|| format!("OECD interest row {country} needs OBS_VALUE"))?
            .parse::<f64>()
            .map_err(|err| format!("OECD interest row {country} value failed to parse: {err}"))?;
        if interest_by_country.insert(country.clone(), value).is_some() {
            return Err(format!("duplicate OECD D41 interest row for {country}"));
        }
    }
    if interest_by_country.keys().cloned().collect::<BTreeSet<_>>() != core_countries {
        return Err("OECD D41 interest raw must contain the exact 11 core rows".to_string());
    }

    let mut net_interest_by_country = BTreeMap::new();
    let mut net_interest_reader = csv::Reader::from_path(root.join(OECD_GOV_NET_INTEREST_RAW_PATH))
        .map_err(|err| format!("failed to read {OECD_GOV_NET_INTEREST_RAW_PATH}: {err}"))?;
    for result in net_interest_reader.deserialize::<BTreeMap<String, String>>() {
        let row = result
            .map_err(|err| format!("failed to parse {OECD_GOV_NET_INTEREST_RAW_PATH}: {err}"))?;
        let country = row
            .get("REF_AREA")
            .ok_or("OECD net-interest raw row needs REF_AREA")?
            .to_string();
        if row.get("MEASURE").map(String::as_str) != Some("GGINTN")
            || row.get("UNIT_MEASURE").map(String::as_str) != Some("PT_B1GQ")
            || row.get("SECTOR").map(String::as_str) != Some("S13")
            || row.get("TIME_PERIOD").map(String::as_str) != Some("2022")
        {
            return Err(format!(
                "invalid OECD GGINTN net-interest row for {country}"
            ));
        }
        let value = row
            .get("OBS_VALUE")
            .ok_or_else(|| format!("OECD net-interest row {country} needs OBS_VALUE"))?
            .parse::<f64>()
            .map_err(|err| {
                format!("OECD net-interest row {country} value failed to parse: {err}")
            })?;
        if net_interest_by_country
            .insert(country.clone(), value)
            .is_some()
        {
            return Err(format!(
                "duplicate OECD GGINTN net-interest row for {country}"
            ));
        }
    }
    if net_interest_by_country
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != core_countries
        || net_interest_by_country.get("KOR") != Some(&-0.05)
    {
        return Err(
            "OECD GGINTN net-interest raw must contain the exact 11 core rows and preserve KOR -0.05"
                .to_string(),
        );
    }

    let expected_tax_categories: BTreeSet<String> =
        ["T_1000", "T_2000", "T_3000", "T_4000", "T_5000", "T_6000"]
            .into_iter()
            .map(str::to_string)
            .collect();
    let mut countries = BTreeSet::new();
    let mut tax_total_count = 0usize;
    let mut tax_mix_count = 0usize;
    for record in records {
        let country = string_field(record, "country_code")?;
        if !countries.insert(country.clone()) {
            return Err(format!("duplicate fiscal country record {country}"));
        }
        if number_field(record, "year")? != 2022.0 {
            return Err(format!(
                "fiscal country record {country} must use data year 2022"
            ));
        }
        let tax_mix = record
            .get("tax_mix")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("fiscal country record {country} needs tax_mix"))?;

        let revenue_percent_gdp = number_field(record, "general_government_revenue_percent_gdp")?;
        number_field(record, "overall_balance_percent_gdp")?;
        number_field(record, "gross_debt_percent_gdp")?;
        if record.get("derived_interest_expense_percent_gdp").is_some()
            || record.get("interest_derivation_status").is_some()
        {
            return Err(format!(
                "{country} must not infer interest from primary and overall balances"
            ));
        }

        if country == "SGP" {
            for field in [
                "tax_revenue_percent_gdp",
                "primary_balance_percent_gdp",
                "net_debt_percent_gdp",
            ] {
                if !record.get(field).is_some_and(serde_json::Value::is_null) {
                    return Err(format!("SGP fiscal field {field} must remain missing"));
                }
            }
            if !record
                .get("direct_interest_expense_percent_gdp")
                .is_some_and(serde_json::Value::is_null)
                || !record
                    .get("direct_interest_expense_percent_revenue")
                    .is_some_and(serde_json::Value::is_null)
                || string_field(record, "direct_interest_status")? != "blocked_no_direct_series"
                || !record
                    .get("net_interest_spending_percent_gdp")
                    .is_some_and(serde_json::Value::is_null)
                || string_field(record, "net_interest_status")? != "blocked_no_direct_series"
                || string_field(record, "observation_status")?
                    != "partial_missing_tax_primary_net_debt_and_interest"
                || !tax_mix.is_empty()
            {
                return Err("SGP tax and direct-interest fields must remain blocked".to_string());
            }
        } else {
            number_field(record, "tax_revenue_percent_gdp")?;
            number_field(record, "primary_balance_percent_gdp")?;
            number_field(record, "net_debt_percent_gdp")?;
            let interest_percent_gdp = number_field(record, "direct_interest_expense_percent_gdp")?;
            let interest_percent_revenue =
                number_field(record, "direct_interest_expense_percent_revenue")?;
            let net_interest_percent_gdp =
                number_field(record, "net_interest_spending_percent_gdp")?;
            let interest_value = interest_by_country
                .get(&country)
                .ok_or_else(|| format!("missing OECD D41 interest row for {country}"))?;
            let gdp_value = gdp_by_country
                .get(&country)
                .ok_or_else(|| format!("missing OECD GDP denominator for {country}"))?;
            let expected_percent_gdp = 100.0 * interest_value / gdp_value;
            let expected_percent_revenue = 100.0 * interest_percent_gdp / revenue_percent_gdp;
            let expected_net_interest_percent_gdp = net_interest_by_country
                .get(&country)
                .ok_or_else(|| format!("missing OECD GGINTN net-interest row for {country}"))?;
            if (interest_percent_gdp - expected_percent_gdp).abs() > 0.00000001
                || (interest_percent_revenue - expected_percent_revenue).abs() > 0.00000001
                || net_interest_percent_gdp != *expected_net_interest_percent_gdp
                || string_field(record, "direct_interest_status")? != "observed_oecd_d41"
                || string_field(record, "net_interest_status")? != "observed_oecd_ggintn"
                || string_field(record, "observation_status")? != "complete"
            {
                return Err(format!(
                    "{country} OECD D41 direct-interest derivation or status failed"
                ));
            }
            tax_total_count += 1;
            tax_mix_count += tax_mix.len();
            let categories: BTreeSet<String> = tax_mix
                .iter()
                .map(|item| string_field(item, "category_code"))
                .collect::<Result<_, _>>()?;
            if tax_mix.len() != 6 || categories != expected_tax_categories {
                return Err(format!(
                    "fiscal country record {country} must contain the exact six tax-mix categories"
                ));
            }
            for item in tax_mix {
                number_field(item, "share_total_tax_revenue_percent")?;
            }
        }
    }

    if countries != expected_countries {
        return Err("fiscal country panel does not contain the exact 12-country set".to_string());
    }
    if tax_total_count != 11 || tax_mix_count != 66 {
        return Err(format!(
            "fiscal country panel counts failed: tax totals={tax_total_count}, tax mix={tax_mix_count}"
        ));
    }

    Ok(())
}

pub(crate) fn validate_fiscal_debt_dynamics(root: &Path) -> Result<(), String> {
    let text =
        fs::read_to_string(root.join(FISCAL_DEBT_DYNAMICS_JSON_PATH)).map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let baseline = card
        .get("baseline_rows")
        .and_then(|v| v.as_array())
        .ok_or("fiscal debt baseline rows")?;
    if baseline.len() != 12 {
        return Err("fiscal debt baseline must contain FY2025-FY2036".to_string());
    }
    for pair in baseline.windows(2) {
        let prior_debt = pair[0][6].as_f64().ok_or("prior baseline debt")?;
        let row = pair[1].as_array().ok_or("fiscal baseline row")?;
        let debt = row[6].as_f64().ok_or("baseline debt")?;
        let deficit = row[4].as_f64().ok_or("baseline deficit")?;
        let other = row[5].as_f64().ok_or("baseline other financing")?;
        if (prior_debt + deficit + other - debt).abs() > 0.002 {
            return Err("fiscal debt baseline identity does not reconcile".to_string());
        }
    }
    let scenarios = card
        .get("scenarios")
        .and_then(|v| v.as_array())
        .ok_or("fiscal debt scenarios")?;
    if scenarios.len() != 3
        || string_field(&card, "net_budget_score_status")? != "blocked_not_policy_specific"
    {
        return Err("fiscal debt scenarios or scoring boundary invalid".to_string());
    }
    for scenario in scenarios {
        if scenario
            .get("rows")
            .and_then(|v| v.as_array())
            .map(Vec::len)
            != Some(11)
        {
            return Err("each fiscal debt scenario must contain FY2026-FY2036".to_string());
        }
    }
    let reader = fs::read_to_string(root.join(FISCAL_DEBT_DYNAMICS_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [FISCAL_DEBT_DYNAMICS_JSON_PATH, "not CBO scores", "107.6%"] {
        if !reader.contains(required) {
            return Err(format!("fiscal debt reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_fiscal_policy_baskets(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(FISCAL_POLICY_BASKETS_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if string_field(&card, "comparison_status")? != "scale_comparison_not_package_score"
        || string_field(&card, "package_score_status")? != "blocked_requires_updated_joint_score"
    {
        return Err("fiscal policy basket scoring boundary invalid".to_string());
    }
    let baskets = card
        .get("illustrative_baskets")
        .and_then(|v| v.as_array())
        .ok_or("fiscal policy baskets")?;
    if baskets.len() != 3 {
        return Err("fiscal policy card must contain three scale baskets".to_string());
    }
    for basket in baskets {
        let sum = number_field(basket, "arithmetic_sum_billions")?;
        let target = number_field(basket, "target_scale_billions")?;
        let difference = number_field(basket, "arithmetic_difference_billions")?;
        if (sum - target - difference).abs() > 0.001 {
            return Err("fiscal policy basket arithmetic does not reconcile".to_string());
        }
    }
    let reader = fs::read_to_string(root.join(FISCAL_POLICY_BASKETS_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        FISCAL_POLICY_BASKETS_JSON_PATH,
        "package-sized",
        "not a valid combined score",
    ] {
        if !reader.contains(required) {
            return Err(format!("fiscal policy basket reader missing {required}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_fiscal_policy_distribution(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(FISCAL_POLICY_DISTRIBUTION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if string_field(&card, "screen_status")?
        != "qualitative_incidence_screen_not_distribution_score"
        || string_field(&card, "package_distribution_status")?
            != "blocked_requires_joint_microsimulation"
    {
        return Err("fiscal policy distribution boundary invalid".to_string());
    }
    let rows = card
        .get("rows")
        .and_then(|v| v.as_array())
        .ok_or("fiscal policy distribution rows")?;
    if rows.len() != 8
        || rows
            .iter()
            .any(|row| row.as_array().map(Vec::len) != Some(6))
    {
        return Err(
            "fiscal policy distribution screen must contain eight complete rows".to_string(),
        );
    }
    let reader = fs::read_to_string(root.join(FISCAL_POLICY_DISTRIBUTION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        FISCAL_POLICY_DISTRIBUTION_JSON_PATH,
        "who ultimately bears the economic burden",
        "not a distributional score",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "fiscal policy distribution reader missing {required}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_fiscal_path_scenarios(root: &Path) -> Result<(), String> {
    let text = fs::read_to_string(root.join(FISCAL_PATH_SCENARIOS_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let card: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let baseline = card.get("baseline").ok_or("fiscal path baseline")?;
    let deficit = number_field(baseline, "nominal_deficit_usd_trillions_2036")?;
    let deficit_share = number_field(baseline, "total_deficit_percent_gdp_2036")? / 100.0;
    let gdp = number_field(baseline, "implied_nominal_gdp_usd_trillions_2036")?;
    if (deficit / deficit_share - gdp).abs() > 0.01
        || (number_field(baseline, "debt_held_by_public_percent_gdp_end")? - 120.210).abs() > 0.001
    {
        return Err("fiscal path baseline does not reconcile".to_string());
    }
    let scenarios = card
        .get("scenarios")
        .and_then(|v| v.as_array())
        .ok_or("fiscal path scenarios")?;
    if scenarios.len() != 4 {
        return Err("fiscal path must contain four scenarios".to_string());
    }
    for scenario in scenarios {
        let adjustment = number_field(scenario, "adjustment_from_baseline_percent_gdp")?;
        let amount = number_field(scenario, "annual_adjustment_equivalent_usd_billions_2036")?;
        if (amount - adjustment / 100.0 * gdp * 1000.0).abs() > 0.001 {
            return Err("fiscal path adjustment amount does not reconcile".to_string());
        }
    }
    if string_field(&card, "debt_stabilization_status")?
        != "first_order_scenarios_available_not_policy_score"
        || !string_field(&card, "net_budget_score_status")?.contains("blocked")
    {
        return Err("fiscal path must block dynamic debt and budget scoring".to_string());
    }
    let reader = fs::read_to_string(root.join(FISCAL_PATH_SCENARIOS_READER_PATH))
        .map_err(|e| e.to_string())?;
    for required in [
        FISCAL_PATH_SCENARIOS_JSON_PATH,
        "annual adjustment equivalent != ten-year score != debt stabilization",
    ] {
        if !reader.contains(required) {
            return Err(format!("fiscal path reader missing {required}"));
        }
    }
    Ok(())
}

