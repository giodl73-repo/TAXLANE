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

pub(crate) fn validate_subfunction_model_records(
    records: &[SubfunctionModelRow],
    profile: &SubfunctionModelProfile,
) -> Result<(), String> {
    if records.is_empty() {
        return Err("no subfunction model rows".to_string());
    }
    for check in &profile.checks {
        if check.subfunction_total_difference.abs() > 10.0 {
            return Err(format!(
                "{}: subfunction total difference {}",
                check.year, check.subfunction_total_difference
            ));
        }
        if (check.modeled_sum - check.individual_income_tax).abs() > 0.0005 {
            return Err(format!(
                "{}: modeled sum {} does not equal income tax {}",
                check.year, check.modeled_sum, check.individual_income_tax
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_subfunction_csv_rows(
    rows: &[BTreeMap<String, String>],
    label: &str,
    expected_count: usize,
) -> Result<(), String> {
    if rows.len() != expected_count {
        return Err(format!(
            "{label}: expected {expected_count} rows, found {}",
            rows.len()
        ));
    }
    for row in rows {
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!("{label}: missing modeled legal status for {row:?}"));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!("{label}: unexpected allocation method for {row:?}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_subfunction_decade_csv_rows(rows: &[BTreeMap<String, String>]) -> Result<(), String> {
    if rows.is_empty() {
        return Err("subfunction decade: no rows".to_string());
    }
    let mut percent_sums: BTreeMap<String, f64> = BTreeMap::new();
    for row in rows {
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!(
                "subfunction decade: missing modeled legal status for {row:?}"
            ));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!(
                "subfunction decade: unexpected allocation method for {row:?}"
            ));
        }
        let decade = row
            .get("decade")
            .ok_or_else(|| "subfunction decade: missing decade".to_string())?;
        let percent = row
            .get("decade_allocation_share_percent")
            .ok_or_else(|| "subfunction decade: missing percent".to_string())?
            .parse::<f64>()
            .map_err(|err| format!("subfunction decade: invalid percent: {err}"))?;
        *percent_sums.entry(decade.to_string()).or_default() += percent;
    }
    for (decade, percent_sum) in percent_sums {
        if (percent_sum - 100.0).abs() > 0.0001 {
            return Err(format!(
                "subfunction decade: {decade} percent sum {percent_sum}"
            ));
        }
    }
    Ok(())
}

