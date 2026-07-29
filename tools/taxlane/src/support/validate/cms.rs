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

pub(crate) fn validate_cms_hospital_quality_methodology_surface_context(root: &Path) -> Result<(), String> {
    for path in [
        CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH,
        CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_READER_PATH,
        "data/metadata/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24.2026-07-24.metadata.md",
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CMS hospital quality methodology surface artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH))
            .map_err(|err| {
            format!(
                "failed to read {CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH}: {err}"
            )
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "cms-hospital-quality-methodology-surface-context:v1"
        || string_field(&record, "record_family")?
            != "cms_hospital_quality_methodology_surface_context"
        || string_field(&record, "status")?
            != "draft_official_surface_custody_methodology_content_blocked"
        || int_field(&record, "pulse")? != 182
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "source_id")?
            != "SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24"
        || string_field(&record, "publisher")?
            != "Centers for Medicare & Medicaid Services (CMS) / QualityNet"
        || string_field(&record, "retrieval_date")? != "2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24.2026-07-24.metadata.md"
    {
        return Err("CMS hospital quality methodology surface identity failed".to_string());
    }

    let expected_raw = [
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/overall_star_rating_topic.html",
            2_196,
            "11612c367d9de8eaa47ad3b9cda178477e7fc9e12a30113d0e52b3eb029a7ab2",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/pdc_index.css",
            1_038_287,
            "5ff7f121fdded61309807712dcd65cf87389d55c08a3bcac355b87a062f232fa",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/pdc_index.js",
            5_017_533,
            "e1818bf6f8d685853e720d6be2969ac1d227b9bbab0ec0e508dcef3ef7dd2294",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/qualitynet_chunk_LP3O54EB.js",
            3_152_293,
            "87313d2bd597ac2233f41bd3bd3b6866fd1909aea4590a381bb816512f75fa1f",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/qualitynet_main.js",
            2_669_385,
            "817bfd72eeab649f2efc5dddfa20781b59d1ad75e36c4fb9302b176651ae3f64",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/qualitynet_mortality_methodology.html",
            595,
            "65c3ce0305aac61b15aa9feb89e194996617caf0b38629c0936593e9df94eae9",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/qualitynet_overall_ratings_resources.html",
            595,
            "65c3ce0305aac61b15aa9feb89e194996617caf0b38629c0936593e9df94eae9",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24/2026-07-24/qualitynet_scripts.js",
            2_286_292,
            "8d57265a794ca596e7cca2fe979da9a66a59a2de943ee1a3ecb9412adcd18058",
        ),
    ]
    .into_iter()
    .map(|(path, bytes, sha)| (path, (bytes, sha)))
    .collect::<BTreeMap<_, _>>();

    let surfaces = record
        .get("official_surfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CMS hospital quality methodology surfaces")?;
    if surfaces.len() != 3 {
        return Err("CMS hospital quality methodology surface count failed".to_string());
    }
    let mut observed_raw = BTreeSet::new();
    for surface in surfaces {
        if !string_field(surface, "url")?.starts_with("https://")
            || surface
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err("CMS hospital quality methodology surface boundary failed".to_string());
        }
        let raw_files = surface
            .get("raw_files")
            .and_then(serde_json::Value::as_array)
            .ok_or("CMS hospital quality methodology raw files")?;
        for raw in raw_files {
            let path = string_field(raw, "path")?;
            let (bytes, sha) = expected_raw
                .get(path.as_str())
                .ok_or("unexpected CMS hospital quality methodology raw file")?;
            if int_field(raw, "byte_count")? != *bytes
                || string_field(raw, "sha256")? != *sha
                || !root.join(&path).exists()
                || fs::metadata(root.join(&path))
                    .map_err(|err| format!("CMS methodology raw metadata failed: {err}"))?
                    .len()
                    != *bytes as u64
                || sha256_file(&root.join(&path))? != *sha
            {
                return Err(format!(
                    "CMS hospital quality methodology raw custody failed: {path}"
                ));
            }
            observed_raw.insert(path);
        }
    }
    let expected_raw_paths = expected_raw
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_raw != expected_raw_paths {
        return Err("CMS hospital quality methodology raw file set failed".to_string());
    }

    let summary = record
        .get("custody_summary")
        .ok_or("CMS hospital quality methodology custody summary")?;
    for field in [
        "official_public_surfaces_captured",
        "local_html_js_shell_custody_ready",
        "metadata_path_present",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "CMS hospital quality methodology summary {field} must be true"
            ));
        }
    }
    for field in [
        "methodology_report_content_custody_ready",
        "denominator_to_field_crosswalk_ready",
        "risk_adjustment_case_mix_methodology_ready",
        "rural_safety_net_capacity_series_ready",
        "floor_threshold_values_ready",
        "observed_floor_values_ready",
        "pass_fail_findings_ready",
        "solver_input_ready",
        "rate_calculation_ready",
        "savings_claim_ready",
        "balanced_budget_claim_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "CMS hospital quality methodology summary {field} must be false"
            ));
        }
    }
    if int_field(summary, "raw_file_count")? != 8
        || int_field(summary, "raw_total_byte_count")? != 14_167_176
    {
        return Err("CMS hospital quality methodology raw summary failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CMS hospital quality methodology blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "CMS hospital quality methodology blocked output must be null: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "Official CMS Provider Data Catalog and QualityNet methodology surfaces",
        "app-shell bytes",
        "do not establish complete methodology report content",
        "denominator-to-field crosswalk",
        "risk-adjustment case-mix lineage",
        "solver input",
        "balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "CMS hospital quality methodology warning missing: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        CMS_HOSPITAL_QUALITY_METHODOLOGY_SURFACE_CONTEXT_JSON_PATH,
        "CMS Provider Data Catalog Overall Hospital Quality Star Rating topic",
        "QualityNet inpatient mortality methodology route",
        "QualityNet overall ratings resources route",
        "raw file count: 8",
        "raw total byte count: 14167176",
        "methodology report content custody is not ready",
        "denominator-to-field crosswalk is not ready",
        "risk adjustment and case mix methodology is not ready",
        "app-shell bytes",
        "not complete methodology report content",
        "not pass/fail findings",
        "not solver input",
        "balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CMS hospital quality methodology reader missing: {phrase}"
            ));
        }
    }

    let metadata = fs::read_to_string(root.join(
        "data/metadata/SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24.2026-07-24.metadata.md",
    ))
    .map_err(|err| format!("failed to read CMS methodology metadata: {err}"))?;
    for phrase in [
        "SRC-CMS-HOSPITAL-QUALITY-METHODOLOGY-SURFACES-2026-07-24",
        "Total local raw bytes: 14167176",
        "not complete methodology content custody",
        "app-shell",
    ] {
        if !metadata.contains(phrase) {
            return Err(format!(
                "CMS hospital quality methodology metadata missing: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cms_hospital_measure_methodology_report_custody(root: &Path) -> Result<(), String> {
    for path in [
        CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH,
        CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_READER_PATH,
        "data/metadata/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02.2026-07-24.metadata.md",
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CMS hospital measure methodology report custody artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH),
    )
    .map_err(|err| {
        format!("failed to read {CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH}: {err}")
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "cms-hospital-measure-methodology-report-custody:v1"
        || string_field(&record, "record_family")?
            != "cms_hospital_measure_methodology_report_custody"
        || string_field(&record, "status")?
            != "draft_partial_cms_methodology_report_custody_floor_values_blocked"
        || int_field(&record, "pulse")? != 182
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "source_id")?
            != "SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02"
        || string_field(&record, "publisher")? != "Centers for Medicare & Medicaid Services (CMS)"
        || string_field(&record, "retrieval_date")? != "2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02.2026-07-24.metadata.md"
    {
        return Err("CMS hospital measure methodology report identity failed".to_string());
    }

    let expected_raw = [
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02/2026-07-24/cms_measure_methodology_page.html",
            229_684,
            "a72380b50fc71dcfbd74cb9a0fdd5362571227cc720b368f0cf13151417a57ff",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02/2026-07-24/hybrid_hospital_wide_risk_standardized_mortality_methodology_v2_1.pdf",
            2_480_474,
            "251baf82e0bac94861903b88ca852b107478925525c2bd97d748b115461db2ec",
        ),
        (
            "data/raw/cms/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02/2026-07-24/2022_condition_specific_mortality_measures_updates_specifications_report.pdf",
            3_807_935,
            "c84b314c95e6ddff5eb54eee72dfd5a5d245fb42a8e6a4cdc90e0b4be9c70d47",
        ),
    ]
    .into_iter()
    .map(|(path, bytes, sha)| (path, (bytes, sha)))
    .collect::<BTreeMap<_, _>>();

    let page = record
        .get("captured_page")
        .ok_or("CMS hospital measure methodology captured page")?;
    let page_path = string_field(page, "raw_artifact_path")?;
    let (page_bytes, page_sha) = expected_raw
        .get(page_path.as_str())
        .ok_or("unexpected CMS hospital measure methodology page")?;
    if int_field(page, "raw_byte_count")? != *page_bytes
        || string_field(page, "raw_sha256")? != *page_sha
        || page
            .get("context_use_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("CMS hospital measure methodology page custody failed".to_string());
    }

    let reports = record
        .get("captured_reports")
        .and_then(serde_json::Value::as_array)
        .ok_or("CMS hospital measure methodology reports")?;
    if reports.len() != 2 {
        return Err("CMS hospital measure methodology report count failed".to_string());
    }
    let mut observed_raw = BTreeSet::from([page_path]);
    for report in reports {
        let path = string_field(report, "raw_artifact_path")?;
        let (bytes, sha) = expected_raw
            .get(path.as_str())
            .ok_or("unexpected CMS hospital measure methodology report")?;
        if !string_field(report, "official_url")?.starts_with("https://www.cms.gov/")
            || int_field(report, "raw_byte_count")? != *bytes
            || string_field(report, "raw_sha256")? != *sha
            || report
                .get("methodology_context")
                .and_then(serde_json::Value::as_array)
                .is_none_or(|items| items.is_empty())
            || report
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "CMS hospital measure methodology report custody failed: {path}"
            ));
        }
        for field in [
            "may_populate_thresholds",
            "may_populate_observed_values",
            "may_populate_pass_fail",
            "may_populate_solver_inputs",
        ] {
            if report.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "CMS hospital measure methodology report must block {field}: {path}"
                ));
            }
        }
        observed_raw.insert(path);
    }
    let expected_raw_paths = expected_raw
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed_raw != expected_raw_paths {
        return Err("CMS hospital measure methodology raw file set failed".to_string());
    }
    for (path, (bytes, sha)) in expected_raw {
        let raw = root.join(path);
        if !raw.exists()
            || fs::metadata(&raw)
                .map_err(|err| format!("CMS measure methodology raw metadata failed: {err}"))?
                .len()
                != bytes as u64
            || sha256_file(&raw)? != sha
        {
            return Err(format!(
                "CMS hospital measure methodology raw file failed: {path}"
            ));
        }
    }

    let summary = record
        .get("custody_summary")
        .ok_or("CMS hospital measure methodology custody summary")?;
    for field in [
        "official_public_methodology_page_captured",
        "selected_mortality_methodology_reports_captured",
        "risk_adjusted_mortality_methodology_context_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "CMS hospital measure methodology summary {field} must be true"
            ));
        }
    }
    for field in [
        "complete_quality_access_methodology_custody_ready",
        "denominator_to_dataset_field_crosswalk_ready",
        "all_measure_family_case_mix_lineage_ready",
        "rural_safety_net_capacity_series_ready",
        "floor_threshold_values_ready",
        "observed_floor_values_ready",
        "pass_fail_findings_ready",
        "solver_input_ready",
        "rate_calculation_ready",
        "savings_claim_ready",
        "balanced_budget_claim_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "CMS hospital measure methodology summary {field} must be false"
            ));
        }
    }
    if int_field(summary, "raw_file_count")? != 3
        || int_field(summary, "raw_total_byte_count")? != 6_518_093
    {
        return Err("CMS hospital measure methodology raw summary failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CMS hospital measure methodology blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "CMS hospital measure methodology blocked output must be null: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "CMS Measure Methodology page custody",
        "selected CMS mortality methodology reports",
        "partial methodology custody",
        "not a complete CMS quality/access denominator-to-dataset field crosswalk",
        "not complete all-measure case-mix lineage",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "CMS hospital measure methodology warning missing: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        CMS_HOSPITAL_MEASURE_METHODOLOGY_REPORT_CUSTODY_JSON_PATH,
        "CMS Measure Methodology page",
        "Hybrid Hospital-Wide Risk-Standardized Mortality Methodology Report Version",
        "2022 Condition-Specific Mortality Measures Updates and Specifications Report",
        "raw file count: 3",
        "raw total byte count: 6518093",
        "risk-adjusted mortality methodology context ready",
        "complete quality/access methodology custody is not ready",
        "denominator-to-dataset field crosswalk is not ready",
        "all-measure case-mix lineage is not ready",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CMS hospital measure methodology reader missing: {phrase}"
            ));
        }
    }

    let metadata = fs::read_to_string(root.join(
        "data/metadata/SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02.2026-07-24.metadata.md",
    ))
    .map_err(|err| format!("failed to read CMS measure methodology metadata: {err}"))?;
    for phrase in [
        "SRC-CMS-HOSPITAL-MEASURE-METHODOLOGY-REPORTS-2026-01-02",
        "Total local raw bytes: 6518093",
        "draft partial methodology custody",
        "not complete quality/access",
    ] {
        if !metadata.contains(phrase) {
            return Err(format!(
                "CMS hospital measure methodology metadata missing: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cms_hospital_quality_dataset_field_crosswalk(root: &Path) -> Result<(), String> {
    for path in [
        CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH,
        CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CMS hospital quality dataset field crosswalk artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH}: {err}"
                )
            })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "cms-hospital-quality-dataset-field-crosswalk:v1"
        || string_field(&record, "record_family")? != "cms_hospital_quality_dataset_field_crosswalk"
        || string_field(&record, "status")?
            != "draft_partial_dataset_field_crosswalk_floor_values_blocked"
        || int_field(&record, "pulse")? != 182
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "health_quality_access_indicator_source_gap_path")?
            != HEALTH_QUALITY_ACCESS_INDICATOR_SOURCE_GAP_JSON_PATH
    {
        return Err("CMS hospital quality dataset field crosswalk identity failed".to_string());
    }

    let expected = [
        (
            "hospital_general_information",
            5432,
            38,
            0,
            "MORT Group Measure Count",
        ),
        ("complications_deaths", 95840, 18, 20, "Denominator"),
        (
            "healthcare_associated_infections",
            172512,
            15,
            36,
            "_ELIGCASES",
        ),
        ("unplanned_hospital_visits", 67088, 20, 14, "Denominator"),
        ("timely_effective_care", 138173, 16, 30, "Sample"),
        (
            "rural_emergency_hospital_timely_effective_care",
            164,
            16,
            4,
            "Sample",
        ),
    ]
    .into_iter()
    .map(|(key, rows, cols, measures, field)| (key, (rows, cols, measures, field)))
    .collect::<BTreeMap<_, _>>();

    let rows = record
        .get("crosswalk_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CMS hospital quality crosswalk rows")?;
    if rows.len() != expected.len() {
        return Err("CMS hospital quality crosswalk row count failed".to_string());
    }
    let mut observed = BTreeSet::new();
    let mut total_rows = 0;
    for row in rows {
        let key = string_field(row, "dataset_key")?;
        let (row_count, column_count, measure_count, required_field) =
            expected
                .get(key.as_str())
                .ok_or("unexpected CMS hospital quality crosswalk row")?;
        total_rows += int_field(row, "row_count")?;
        let fields = row
            .get("denominator_or_measure_count_fields")
            .and_then(serde_json::Value::as_array)
            .ok_or("CMS hospital quality denominator fields")?;
        if int_field(row, "row_count")? != *row_count
            || int_field(row, "column_count")? != *column_count
            || int_field(row, "measure_id_count")? != *measure_count
            || !root.join(string_field(row, "raw_artifact_path")?).exists()
            || string_field(row, "crosswalk_status")? == "complete"
            || !fields.iter().any(|field| {
                field
                    .as_str()
                    .is_some_and(|text| text.contains(required_field))
            })
        {
            return Err(format!("CMS hospital quality crosswalk row failed: {key}"));
        }
        for field in [
            "may_populate_thresholds",
            "may_populate_observed_values",
            "may_populate_pass_fail",
            "may_populate_solver_inputs",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "CMS hospital quality crosswalk row must block {field}: {key}"
                ));
            }
        }
        observed.insert(key);
    }
    let expected_keys = expected
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed != expected_keys || total_rows != 479_209 {
        return Err("CMS hospital quality crosswalk aggregate failed".to_string());
    }

    let summary = record
        .get("crosswalk_summary")
        .ok_or("CMS hospital quality crosswalk summary")?;
    if int_field(summary, "captured_dataset_count")? != 6
        || int_field(summary, "captured_total_rows")? != 479_209
        || summary
            .get("partial_denominator_or_measure_count_field_presence_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("CMS hospital quality crosswalk summary counts failed".to_string());
    }
    for field in [
        "complete_denominator_to_dataset_field_crosswalk_ready",
        "measure_methodology_to_dataset_join_ready",
        "all_measure_family_case_mix_lineage_ready",
        "floor_threshold_values_ready",
        "observed_floor_values_ready",
        "pass_fail_findings_ready",
        "solver_input_ready",
        "rate_calculation_ready",
        "balanced_budget_claim_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "CMS hospital quality crosswalk summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CMS hospital quality crosswalk blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "CMS hospital quality crosswalk blocked output must be null: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "partially ready for six captured Provider Data Catalog datasets",
        "Denominator, Sample, measure-count fields, and HAI measure-ID pattern context",
        "not a complete denominator-to-dataset field crosswalk",
        "not a methodology-to-dataset join",
        "not all-measure case-mix lineage",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "CMS hospital quality crosswalk warning missing: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_READER_PATH),
    )
    .map_err(|err| {
        format!("failed to read {CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_READER_PATH}: {err}")
    })?;
    for phrase in [
        CMS_HOSPITAL_QUALITY_DATASET_FIELD_CROSSWALK_JSON_PATH,
        "Hospital General Information",
        "Complications and Deaths - Hospital",
        "Healthcare Associated Infections - Hospital",
        "Unplanned Hospital Visits - Hospital",
        "Timely and Effective Care - Hospital",
        "Rural Emergency Hospital Timely and Effective Care - Hospital",
        "captured total rows: 479209",
        "complete denominator-to-dataset field crosswalk is not ready",
        "measure methodology to dataset join is not ready",
        "not pass/fail findings",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CMS hospital quality crosswalk reader missing: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_cms_hrsa_rural_safety_net_capacity_context(root: &Path) -> Result<(), String> {
    for path in [
        CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH,
        CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_READER_PATH,
        "data/metadata/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24.2026-07-24.metadata.md",
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing CMS/HRSA rural safety-net capacity artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH))
        .map_err(|err| {
            format!("failed to read {CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH}: {err}")
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!("failed to parse {CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH}: {err}")
    })?;

    if string_field(&record, "record_id")? != "cms-hrsa-rural-safety-net-capacity-context:v1"
        || string_field(&record, "record_family")? != "cms_hrsa_rural_safety_net_capacity_context"
        || string_field(&record, "status")?
            != "draft_partial_rural_safety_net_capacity_context_floor_values_blocked"
        || int_field(&record, "pulse")? != 182
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "source_id")?
            != "SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24"
        || string_field(&record, "metadata_path")?
            != "data/metadata/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24.2026-07-24.metadata.md"
    {
        return Err("CMS/HRSA rural safety-net capacity identity failed".to_string());
    }

    let expected_raw = [
        (
            "data/raw/cms/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24/2026-07-24/cms_team_safetynet_rural_fact_sheet.pdf",
            235_117,
            "14b998ce02e54d77e3a25f3dfb75a03a30bd2422eea44a731c8409c63b3de7ea",
        ),
        (
            "data/raw/cms/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24/2026-07-24/cms_provider_specific_data_public_use_page.html",
            213_804,
            "bcf66589f060a7389bd60947dc428e4465cedcbf0cd91eece91ac2b4049f9ca3",
        ),
        (
            "data/raw/cms/SRC-CMS-HRSA-RURAL-SAFETY-NET-CAPACITY-CONTEXT-2026-07-24/2026-07-24/cms_inpatient_psf_october_2025.zip",
            10_201_748,
            "dd245f3d013fa12de3a1710248a1bfebfe10cf77685736e87633498130b8d812",
        ),
    ]
    .into_iter()
    .map(|(path, bytes, sha)| (path, (bytes, sha)))
    .collect::<BTreeMap<_, _>>();

    let sources = record
        .get("local_cms_sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("CMS/HRSA rural safety-net local CMS sources")?;
    if sources.len() != 3 {
        return Err("CMS/HRSA rural safety-net source count failed".to_string());
    }
    let mut observed = BTreeSet::new();
    for source in sources {
        let path = string_field(source, "raw_artifact_path")?;
        let (bytes, sha) = expected_raw
            .get(path.as_str())
            .ok_or("unexpected CMS/HRSA rural safety-net raw file")?;
        if int_field(source, "raw_byte_count")? != *bytes
            || string_field(source, "raw_sha256")? != *sha
            || !root.join(&path).exists()
            || fs::metadata(root.join(&path))
                .map_err(|err| format!("CMS/HRSA rural safety-net metadata failed: {err}"))?
                .len()
                != *bytes as u64
            || sha256_file(&root.join(&path))? != *sha
            || source
                .get("context_use_only")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        {
            return Err(format!(
                "CMS/HRSA rural safety-net raw custody failed: {path}"
            ));
        }
        observed.insert(path);
    }
    let expected_paths = expected_raw
        .keys()
        .copied()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if observed != expected_paths {
        return Err("CMS/HRSA rural safety-net raw file set failed".to_string());
    }

    let psf = sources
        .iter()
        .find(|source| {
            string_field(source, "source_key").ok().as_deref()
                == Some("cms_inpatient_psf_october_2025_zip")
        })
        .ok_or("CMS/HRSA rural safety-net PSF source")?;
    let zip_entries = psf
        .get("zip_entries")
        .and_then(serde_json::Value::as_array)
        .ok_or("CMS/HRSA rural safety-net PSF zip entries")?;
    if zip_entries.len() != 3
        || !zip_entries.iter().any(|entry| {
            string_field(entry, "entry_name").ok().as_deref() == Some("IPSF_INP_2025-12-05.csv")
                && int_field(entry, "entry_byte_count").ok() == Some(102_215_932)
                && int_field(entry, "row_count").ok() == Some(332_369)
        })
        || !zip_entries.iter().any(|entry| {
            string_field(entry, "entry_name").ok().as_deref() == Some("IPSF_INP_LRO_2025-12-05.csv")
                && int_field(entry, "entry_byte_count").ok() == Some(2_837_534)
                && int_field(entry, "row_count").ok() == Some(9_413)
        })
        || psf
            .get("capacity_context_fields")
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| items.len() < 7)
    {
        return Err("CMS/HRSA rural safety-net PSF entry summary failed".to_string());
    }

    let hrsa = record
        .get("browser_visible_hrsa_context")
        .ok_or("CMS/HRSA rural safety-net HRSA context")?;
    if hrsa
        .get("browser_context_ready")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
        || hrsa
            .get("local_raw_custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || !string_field(hrsa, "command_line_access_boundary")?.contains("access denied")
    {
        return Err("CMS/HRSA rural safety-net HRSA boundary failed".to_string());
    }

    let summary = record
        .get("capacity_context_summary")
        .ok_or("CMS/HRSA rural safety-net summary")?;
    for field in [
        "cms_definition_context_captured",
        "cms_inpatient_psf_zip_captured",
        "cms_psf_capacity_fields_identified",
        "hrsa_browser_context_identified",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("CMS/HRSA rural safety-net {field} must be true"));
        }
    }
    for field in [
        "hrsa_local_raw_custody_ready",
        "facility_to_county_rural_join_ready",
        "complete_rural_safety_net_capacity_series_ready",
        "floor_threshold_values_ready",
        "observed_floor_values_ready",
        "pass_fail_findings_ready",
        "solver_input_ready",
        "rate_calculation_ready",
        "balanced_budget_claim_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("CMS/HRSA rural safety-net {field} must be false"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CMS/HRSA rural safety-net blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "CMS/HRSA rural safety-net blocked output must be null: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "CMS rural/safety-net definition context",
        "CMS inpatient PSF ZIP custody",
        "HRSA FORHP rural data files are browser-visible official context",
        "HRSA local raw custody and facility-to-county rural joins remain blocked",
        "not a complete rural/safety-net capacity series",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "CMS/HRSA rural safety-net warning missing: {phrase}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_READER_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_READER_PATH}: {err}"
                )
            })?;
    for phrase in [
        CMS_HRSA_RURAL_SAFETY_NET_CAPACITY_CONTEXT_JSON_PATH,
        "CMS TEAM safety-net and rural hospital fact sheet",
        "CMS Inpatient PSF October 2025 ZIP",
        "IPSF_INP_2025-12-05.csv`: 332369 rows",
        "bedSize",
        "supplementalSecurityIncomeRatio",
        "medicaidRatio",
        "operatingDsh",
        "HRSA local raw custody is not ready",
        "not a complete",
        "not pass/fail findings",
        "not solver input",
        "balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "CMS/HRSA rural safety-net reader missing: {phrase}"
            ));
        }
    }

    Ok(())
}

