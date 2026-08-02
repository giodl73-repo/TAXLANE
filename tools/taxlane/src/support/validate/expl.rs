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

pub(crate) fn validate_expl_a_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_A_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_a_narrative_evidence_foundation_closure.schema.md",
        "docs/reading/expl-a-narrative-evidence-foundation-closure.md",
        "reviews/2026-07-27-expl-a-foundation-round-1-roles-review.md",
        "reviews/2026-07-27-expl-a-foundation-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-481-expl-a-narrative-evidence-foundation-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-A closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_A_CLOSURE_JSON_PATH)?;
    let deliverables = record
        .get("deliverables")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-A deliverables")?;
    let review = record.get("review_cycle").ok_or("EXPL-A review cycle")?;
    let gates = record.get("gates").ok_or("EXPL-A gates")?;
    for row in deliverables {
        let path = string_field(row, "path")?;
        if !root.join(&path).is_file()
            || string_field(row, "id").is_err()
            || string_field(row, "kind").is_err()
        {
            return Err(format!("EXPL-A deliverable failed: {path}"));
        }
    }
    if int_field(&record, "pulse")? != 481
        || string_field(&record, "status")?
            != "expl_a_complete_round_two_roles_accepted_expl_b_ready"
        || deliverables.len() != 5
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 2
        || int_field(review, "round_1_p2")? != 6
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "p3_disposition_recorded")?
        || !bool_field(review, "round_2_accepted")?
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || !bool_field(gates, "canonical_claims_traceable")?
        || !bool_field(gates, "numbers_traceable")?
        || !bool_field(gates, "marginal_rate_definition_visible")?
        || !bool_field(gates, "fund_and_allocation_boundary_visible")?
        || !bool_field(gates, "source_vintage_visible")?
        || !bool_field(gates, "beneficiary_and_compliance_limits_visible")?
        || !bool_field(gates, "one_year_and_long_run_separated")?
        || !bool_field(gates, "authority_and_false_precision_limits_visible")?
        || bool_field(gates, "external_release_authorized")?
        || bool_field(gates, "deployment_allowed")?
        || string_field(&record, "next_wave")? != "EXPL-B"
    {
        return Err("EXPL-A closure failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_expl_b_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_B_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_b_citizen_teaching_guides_closure.schema.md",
        "docs/reading/expl-b-citizen-teaching-guides-closure.md",
        "reviews/2026-07-27-expl-b-guides-round-1-roles-review.md",
        "reviews/2026-07-27-expl-b-guides-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-482-expl-b-citizen-teaching-guides-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-B closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_B_CLOSURE_JSON_PATH)?;
    let deliverables = record
        .get("deliverables")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-B deliverables")?;
    let review = record.get("review_cycle").ok_or("EXPL-B review cycle")?;
    let gates = record.get("gates").ok_or("EXPL-B gates")?;
    for row in deliverables {
        let path = string_field(row, "path")?;
        if !root.join(&path).is_file() {
            return Err(format!("EXPL-B deliverable failed: {path}"));
        }
    }
    if int_field(&record, "pulse")? != 482
        || string_field(&record, "status")?
            != "expl_b_complete_round_two_roles_accepted_expl_c_ready"
        || string_field(&record, "depends_on")? != EXPL_A_CLOSURE_JSON_PATH
        || deliverables.len() != 6
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 1
        || int_field(review, "round_1_p2")? != 7
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "p3_applied")?
        || !bool_field(review, "round_2_accepted")?
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || !bool_field(gates, "plain_language_entry")?
        || !bool_field(gates, "marginal_rate_teaching")?
        || !bool_field(gates, "non_comparable_rails_fixed")?
        || !bool_field(gates, "exact_evidence_routes")?
        || !bool_field(gates, "public_purpose_categories_separated")?
        || !bool_field(gates, "service_continuity_rule_visible")?
        || !bool_field(gates, "receipt_return_complexity_separated")?
        || !bool_field(gates, "borrowing_and_horizon_limits_visible")?
        || !bool_field(gates, "normative_choice_and_authority_visible")?
        || bool_field(gates, "external_release_authorized")?
        || string_field(&record, "next_wave")? != "EXPL-C"
    {
        return Err("EXPL-B closure failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_expl_c_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_C_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_c_research_paper_series_closure.schema.md",
        "docs/reading/expl-c-research-paper-series-closure.md",
        "reviews/2026-07-27-expl-c-papers-round-1-roles-review.md",
        "reviews/2026-07-27-expl-c-papers-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-483-expl-c-research-paper-series-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-C closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_C_CLOSURE_JSON_PATH)?;
    let papers = record
        .get("papers")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-C papers")?;
    let review = record.get("review_cycle").ok_or("EXPL-C review cycle")?;
    let gates = record.get("gates").ok_or("EXPL-C gates")?;
    for row in papers {
        for key in ["markdown", "panel", "pdf"] {
            let path = string_field(row, key)?;
            let metadata = std::fs::metadata(root.join(&path))
                .map_err(|_| format!("EXPL-C paper artifact missing: {path}"))?;
            if !metadata.is_file() || metadata.len() == 0 {
                return Err(format!("EXPL-C paper artifact empty: {path}"));
            }
        }
    }
    if int_field(&record, "pulse")? != 483
        || string_field(&record, "status")?
            != "expl_c_complete_round_two_roles_pf_accepted_expl_d_ready"
        || papers.len() != 4
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 1
        || int_field(review, "round_1_p2")? != 6
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "round_2_accepted")?
        || string_field(review, "pf_2_comparability")? != "pass"
        || string_field(review, "pf_5_distribution")? != "pass_with_limits"
        || string_field(review, "pf_6_skepticism")? != "pass"
        || string_field(review, "pf_7_sourcing")? != "pass"
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || !bool_field(gates, "markdown_canonical")?
        || !bool_field(gates, "inline_figure_ledger_routing")?
        || !bool_field(gates, "fiscal_object_boundary")?
        || !bool_field(gates, "service_continuity_rule")?
        || !bool_field(gates, "distribution_limits")?
        || !bool_field(gates, "compliance_limits")?
        || !bool_field(gates, "analytical_not_legal_adaptation")?
        || !bool_field(gates, "pdfs_rendered_repository_only")?
        || bool_field(gates, "external_release_authorized")?
        || string_field(&record, "next_wave")? != "EXPL-D"
    {
        return Err("EXPL-C closure failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_expl_d_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_D_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_d_presentation_system_closure.schema.md",
        "docs/reading/expl-d-presentation-system-closure.md",
        "reviews/2026-07-27-expl-d-presentations-round-1-roles-review.md",
        "reviews/2026-07-27-expl-d-presentations-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-484-expl-d-presentation-system-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-D closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_D_CLOSURE_JSON_PATH)?;
    let presentations = record
        .get("presentations")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-D presentations")?;
    let review = record.get("review_cycle").ok_or("EXPL-D review cycle")?;
    let gates = record.get("gates").ok_or("EXPL-D gates")?;
    for row in presentations {
        for key in ["source", "preview"] {
            let path = string_field(row, key)?;
            let content = std::fs::read_to_string(root.join(&path))
                .map_err(|_| format!("EXPL-D artifact missing: {path}"))?;
            if content.is_empty()
                || (key == "preview"
                    && (content.contains("http://") || content.contains("https://")))
            {
                return Err(format!("EXPL-D artifact failed: {path}"));
            }
        }
    }
    if int_field(&record, "pulse")? != 484
        || string_field(&record, "status")?
            != "expl_d_complete_round_two_roles_accepted_expl_e_ready"
        || string_field(&record, "depends_on")? != EXPL_C_CLOSURE_JSON_PATH
        || presentations.len() != 3
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 1
        || int_field(review, "round_1_p2")? != 6
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "p3_applied")?
        || !bool_field(review, "round_2_accepted")?
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || !bool_field(gates, "speaker_notes")?
        || !bool_field(gates, "caveat_slide")?
        || !bool_field(gates, "fifteen_track_view")?
        || !bool_field(gates, "pay_net_rev_identity")?
        || !bool_field(gates, "three_rate_rails")?
        || !bool_field(gates, "reopening_slide")?
        || !bool_field(gates, "inline_ledger_routes")?
        || !bool_field(gates, "marginal_rate_teaching")?
        || !bool_field(gates, "local_previews_no_remote_assets")?
        || bool_field(gates, "external_release_authorized")?
        || bool_field(gates, "external_presentation_authorized")?
        || string_field(&record, "next_wave")? != "EXPL-E"
    {
        return Err("EXPL-D closure failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_expl_e_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_E_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_e_local_html_experience_closure.schema.md",
        "docs/reading/expl-e-local-html-experience-closure.md",
        "reviews/2026-07-27-expl-e-local-html-round-1-roles-review.md",
        "reviews/2026-07-27-expl-e-local-html-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-485-expl-e-local-html-experience-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-E closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_E_CLOSURE_JSON_PATH)?;
    let pages = record
        .get("pages")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-E pages")?;
    let review = record.get("review_cycle").ok_or("EXPL-E review cycle")?;
    let gates = record.get("gates").ok_or("EXPL-E gates")?;
    let site_root = root.join(string_field(&record, "site_root")?);
    let current_pages = [
        "index.html",
        "tracks.html",
        "owners.html",
        "rates.html",
        "method.html",
        "evidence.html",
        "glossary.html",
    ];
    for name in current_pages {
        let content = std::fs::read_to_string(site_root.join(name))
            .map_err(|_| format!("EXPL-E page missing: {name}"))?;
        if !content.contains("<main id=\"content\">")
            || !content.contains("class=\"skip\"")
            || !content.contains("<nav aria-label=\"Primary\">")
            || !content.contains("repository-only local preview")
            || content.contains("http://")
            || content.contains("https://")
            || content.contains("<form")
            || content.to_ascii_lowercase().contains("analytics") && name != "index.html"
        {
            return Err(format!("EXPL-E page safety/accessibility failed: {name}"));
        }
        let page_path = site_root.join(name);
        let page_dir = page_path.parent().ok_or("EXPL-E page parent")?;
        for tail in content.split("href=\"").skip(1) {
            let target = tail.split('"').next().ok_or("EXPL-E href")?;
            if target.starts_with('#') {
                continue;
            }
            if !page_dir.join(target).is_file() {
                return Err(format!("EXPL-E broken local link in {name}: {target}"));
            }
        }
    }
    let css = std::fs::read_to_string(site_root.join("styles.css"))
        .map_err(|_| "EXPL-E stylesheet missing".to_string())?;
    if !css.contains(":focus-visible")
        || !css.contains("prefers-reduced-motion")
        || !css.contains("overflow-x:auto")
    {
        return Err("EXPL-E stylesheet accessibility failed".to_string());
    }
    let index = std::fs::read_to_string(site_root.join("index.html")).unwrap_or_default();
    let rates = std::fs::read_to_string(site_root.join("rates.html")).unwrap_or_default();
    if !index.contains("$813.727B")
        || !index.contains("$0.000B")
        || !index.contains("21/23/33/35/43/46/48")
        || !rates.contains("$813.727B")
        || !rates.contains("21/23/33/35/43/46/48")
        || !rates.contains("22/24/34/36/44/47/49")
        || !rates.contains("22.6/24.6/34.6/36.6/44.6/47.6/49.6")
    {
        return Err("EXPL-E claim sync failed".to_string());
    }
    if int_field(&record, "pulse")? != 485
        || string_field(&record, "status")?
            != "expl_e_complete_round_two_roles_accepted_expl_f_ready"
        || string_field(&record, "depends_on")? != EXPL_D_CLOSURE_JSON_PATH
        || pages.len() != 6
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 1
        || int_field(review, "round_1_p2")? != 6
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "p3_applied")?
        || !bool_field(review, "round_2_accepted")?
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || !bool_field(gates, "semantic_landmarks")?
        || !bool_field(gates, "skip_links_and_focus")?
        || !bool_field(gates, "responsive_tables")?
        || !bool_field(gates, "reduced_motion")?
        || !bool_field(gates, "claim_sync")?
        || !bool_field(gates, "local_links_checked")?
        || int_field(gates, "remote_urls")? != 0
        || int_field(gates, "remote_assets")? != 0
        || int_field(gates, "forms")? != 0
        || int_field(gates, "analytics_or_tracking")? != 0
        || int_field(gates, "deployment_files")? != 0
        || bool_field(gates, "external_release_authorized")?
        || bool_field(gates, "deployment_allowed")?
        || string_field(&record, "next_wave")? != "EXPL-F"
    {
        return Err("EXPL-E closure failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_expl_f_closure(root: &Path) -> Result<(), String> {
    for path in [
        EXPL_F_CLOSURE_JSON_PATH,
        "data/derived/breadth_benchmark_matrix/expl_f_integrated_repository_readiness_closure.schema.md",
        "docs/reading/expl-f-integrated-repository-readiness-closure.md",
        "reviews/2026-07-27-expl-f-integrated-round-1-roles-review.md",
        "reviews/2026-07-27-expl-f-integrated-round-2-roles-review.md",
        "context/waves/2026-07-18-adaptive-rate-performance-system/pulses/pulse-486-expl-f-integrated-repository-readiness-closure.md",
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing EXPL-F closure artifact: {path}"));
        }
    }
    let canonical_deliverables = [
        "docs/explanation/foundation/canonical-result-statement.md",
        "docs/explanation/foundation/claim-ledger.md",
        "docs/explanation/foundation/number-ledger.md",
        "docs/explanation/foundation/terminology-and-visual-grammar.md",
        "docs/explanation/foundation/artifact-inventory-and-downstream-outlines.md",
        "docs/explanation/guides/one-page-result.md",
        "docs/explanation/guides/citizen-guide.md",
        "docs/explanation/guides/fifteen-track-guide.md",
        "docs/explanation/guides/frequently-asked-questions.md",
        "docs/explanation/guides/glossary.md",
        "docs/explanation/guides/educator-discussion-guide.md",
        "research/publications/the-taxlane-result/paper.md",
        "research/publications/why-zero-is-a-result/paper.md",
        "research/publications/fifteen-tracks-one-accounting-spine/paper.md",
        "research/publications/spending-evidence-to-adaptive-rate/paper.md",
        "docs/explanation/presentations/5-minute-overview.md",
        "docs/explanation/presentations/20-minute-civic-briefing.md",
        "docs/explanation/presentations/45-minute-technical-walkthrough.md",
        "docs/explanation/site/index.html",
        "docs/explanation/final/briefing-bundle-index.md",
        "docs/explanation/final/cross-format-consistency-report.md",
    ];
    for path in canonical_deliverables {
        let metadata = std::fs::metadata(root.join(path))
            .map_err(|_| format!("EXPL-F canonical deliverable missing: {path}"))?;
        if metadata.len() == 0 {
            return Err(format!("EXPL-F canonical deliverable empty: {path}"));
        }
    }
    let record = read_json_artifact(root, EXPL_F_CLOSURE_JSON_PATH)?;
    let dependencies = record
        .get("depends_on")
        .and_then(serde_json::Value::as_array)
        .ok_or("EXPL-F dependencies")?;
    let aggregate = record.get("aggregate").ok_or("EXPL-F aggregate")?;
    let review = record.get("review_cycle").ok_or("EXPL-F review cycle")?;
    let validation = record.get("validation").ok_or("EXPL-F validation")?;
    let gates = record.get("gates").ok_or("EXPL-F gates")?;
    let expected_dependencies = [
        EXPL_A_CLOSURE_JSON_PATH,
        EXPL_B_CLOSURE_JSON_PATH,
        EXPL_C_CLOSURE_JSON_PATH,
        EXPL_D_CLOSURE_JSON_PATH,
        EXPL_E_CLOSURE_JSON_PATH,
    ];
    let observed_dependencies = dependencies
        .iter()
        .map(|value| value.as_str().ok_or("EXPL-F dependency"))
        .collect::<Result<Vec<_>, _>>()?;
    let report = std::fs::read_to_string(
        root.join("docs/explanation/final/cross-format-consistency-report.md"),
    )
    .map_err(|_| "EXPL-F consistency report".to_string())?;
    for value in [
        "$0.000B",
        "$813.727B",
        "21/23/33/35/43/46/48",
        "22/24/34/36/44/47/49",
        "22.6/24.6/34.6/36.6/44.6/47.6/49.6",
    ] {
        if !report.contains(value) {
            return Err(format!("EXPL-F parity value missing: {value}"));
        }
    }
    if int_field(&record, "pulse")? != 486
        || string_field(&record, "status")?
            != "expl_a_through_f_complete_repository_ready_external_release_blocked"
        || observed_dependencies != expected_dependencies
        || int_field(aggregate, "waves_complete")? != 6
        || int_field(aggregate, "canonical_deliverables")? != 21
        || int_field(aggregate, "role_review_rounds")? != 12
        || int_field(aggregate, "role_lenses_per_round")? != 8
        || int_field(aggregate, "open_p1")? != 0
        || int_field(aggregate, "open_p2")? != 0
        || int_field(review, "role_count")? != 8
        || int_field(review, "round_1_p1")? != 1
        || int_field(review, "round_1_p2")? != 6
        || !bool_field(review, "all_p1_applied")?
        || !bool_field(review, "all_p2_applied")?
        || !bool_field(review, "p3_applied")?
        || !bool_field(review, "round_2_accepted")?
        || int_field(review, "open_p1")? != 0
        || int_field(review, "open_p2")? != 0
        || int_field(validation, "taxlane_core_tests")? != 152
        || int_field(validation, "taxlane_tools_tests")? != 236
        || int_field(validation, "workspace_tests")? != 388
        || !bool_field(validation, "full_workspace_passed")?
        || !bool_field(validation, "domain_validator_passed")?
        || !bool_field(validation, "manifest_current")?
        || !bool_field(gates, "headline_parity")?
        || !bool_field(gates, "boundary_parity")?
        || !bool_field(gates, "review_chain_sequential")?
        || !bool_field(gates, "canonical_over_convenience_views")?
        || !bool_field(gates, "paper_pdfs_nonempty")?
        || !bool_field(gates, "presentation_previews_local")?
        || !bool_field(gates, "site_accessibility_and_links")?
        || !bool_field(gates, "no_release_mechanism_added")?
        || !bool_field(gates, "repository_ready")?
        || bool_field(gates, "external_release_authorized")?
        || bool_field(gates, "deployment_allowed")?
        || bool_field(gates, "official_request_authorized")?
        || !record
            .get("next_wave")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("EXPL-F closure failed".to_string());
    }
    Ok(())
}
