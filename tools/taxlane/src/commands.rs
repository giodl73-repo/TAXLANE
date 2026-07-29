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

pub(crate) fn run_income_tax_outlay_validation() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };

    if let Err(err) = build_annual_model(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = build_decade_summary(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = export_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = export_subfunction_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_manifest(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_accountability_evidence_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_program_lane_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_spend_category_map(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_global_country_comparison_coverage(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_breadth_benchmark_matrix(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_fcic_payment_integrity_bridge(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_headline_basis_crosswalk(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_efficiency_pressure_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_per_unit_display_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_readiness_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_action_queue(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_packet(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_work_items(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_claim_guard_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_questions(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief_discovery(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_artifact_map(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_claim_gates(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_dashboard(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_brief(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_letter(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_rubric(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_followup(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_log_schema(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_status(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_dashboard(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_handoff(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake_schema(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_intake_example_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_external_accountability_claim_intake(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_mn_ccap_cy2025_request_specification(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_log_applied_example_jsonl(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_status_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_dashboard_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_handoff_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_delta_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_delta_applied_example_jsonl(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_delta_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_response_bundle_applied_example(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_bundle_applied_example_json(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) =
        check_accountability_performance_demand_response_bundle_applied_example_schema(&root)
    {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    for spec in CHART_SPECS {
        if let Err(err) = parse_json(&root.join(spec)) {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
        println!("validated JSON spec: {spec}");
    }

    if let Err(err) = validate_placeholder_receipt_chart_sync(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    println!(
        "validated income-tax outlay model checks and {} chart specs",
        CHART_SPECS.len()
    );
    ExitCode::SUCCESS
}

pub(crate) fn run_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_subfunction_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_subfunction_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_subfunction_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_subfunction_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_summary_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_summary_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_manifest_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match check_manifest(&root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_manifest_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_manifest(&root).and_then(|manifest| {
        fs::write(root.join(MANIFEST_PATH), manifest)
            .map_err(|err| format!("failed to write {MANIFEST_PATH}: {err}"))
    }) {
        Ok(()) => {
            println!("wrote {MANIFEST_PATH}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_2_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_2_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_1_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_1_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_2_national_defense_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_2_national_defense_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_6_1_national_defense_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_composition_table_6_1_national_defense(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_6_1_national_defense_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_composition_table_6_1_national_defense(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

pub(crate) fn run_table_3_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

