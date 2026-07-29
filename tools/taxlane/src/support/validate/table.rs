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

pub(crate) fn validate_table_3_2_national_defense_rows(
    rows: &[Table32OutlayFunctionRow],
    profile: &Table32NationalDefenseProfile,
) -> Result<(), String> {
    let expected_rows = profile.year_count * TABLE_3_2_NATIONAL_DEFENSE_LINES.len();
    if rows.len() != expected_rows {
        return Err(format!(
            "expected {expected_rows} Table 3.2 National Defense rows, found {}",
            rows.len()
        ));
    }
    for check in &profile.checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 National Defense difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.subfunction_difference.abs() > 2.0 {
            return Err(format!(
                "{}: National Defense subfunction difference {}",
                check.year, check.subfunction_difference
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_table_3_2_national_defense_labels(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(), String> {
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        let label = sheet
            .get(&line.source_row)
            .and_then(|row| text_cell(row.get("A")))
            .ok_or_else(|| format!("missing Table 3.2 row {} label", line.source_row))?;
        if label != line.source_label {
            return Err(format!(
                "Unexpected Table 3.2 row {}: {label:?}",
                line.source_row
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_table_6_1_national_defense_rows(
    rows: &[Table61NationalDefenseRow],
    profile: &Table61NationalDefenseProfile,
) -> Result<(), String> {
    if rows.len() != profile.year_count {
        return Err(format!(
            "expected {} Table 6.1 National Defense rows, found {}",
            profile.year_count,
            rows.len()
        ));
    }
    for row in rows {
        if !(0.0..=50.0).contains(&row.percent_of_gdp) {
            return Err(format!(
                "{}: implausible national-defense %GDP {}",
                row.fiscal_year, row.percent_of_gdp
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_table_3_2_rows(profile: &Table32Profile) -> Result<(), String> {
    for check in &profile.grand_checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 total difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.function_total_difference.abs() > 5.0 {
            return Err(format!(
                "{}: Table 3.2 function total difference {}",
                check.year, check.function_total_difference
            ));
        }
    }
    for check in &profile.function_checks {
        if check.difference.abs() > 2.0 {
            return Err(format!(
                "{} {}: Table 3.2 function difference {}",
                check.year, check.function_code, check.difference
            ));
        }
    }
    Ok(())
}

