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

#[derive(Clone, Copy)]
pub(crate) struct Artifact {
    pub(crate) path: &'static str,
    pub(crate) role: &'static str,
    pub(crate) grain: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) canonical: &'static str,
}

impl Artifact {
    pub(crate) fn metadata(&self) -> ArtifactMetadata<'_> {
        ArtifactMetadata {
            path: self.path,
            role: self.role,
            grain: self.grain,
            kind: self.kind,
            canonical: self.canonical,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ReceiptShareCategory {
    pub(crate) column: &'static str,
    pub(crate) receipt_category: &'static str,
    pub(crate) source_receipt_label: &'static str,
    pub(crate) allocation_status: &'static str,
    pub(crate) notes: &'static str,
}

#[derive(Clone)]
pub(crate) struct ReceiptShareRow {
    pub(crate) fiscal_year: i64,
    pub(crate) source_row: i64,
    pub(crate) source_column: &'static str,
    pub(crate) receipt_category: &'static str,
    pub(crate) source_receipt_label: &'static str,
    pub(crate) percent: f64,
    pub(crate) actual_or_projection: &'static str,
    pub(crate) allocation_status: &'static str,
    pub(crate) notes: &'static str,
}

#[derive(Clone)]
pub(crate) struct OutlayFunctionRow {
    pub(crate) fiscal_year: i64,
    pub(crate) source_column: String,
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) source_row: i64,
    pub(crate) amount: f64,
    pub(crate) actual_or_projection: &'static str,
    pub(crate) offsetting_treatment: &'static str,
    pub(crate) notes: &'static str,
    pub(crate) include_table_1_1_source: bool,
    pub(crate) table_1_1_row: Option<i64>,
}

pub(crate) struct OutlayFunctionCheck {
    pub(crate) year: i64,
    pub(crate) table_1_1_outlays: f64,
    pub(crate) table_3_1_total: f64,
    pub(crate) broad_category_total: f64,
    pub(crate) total_difference: f64,
    pub(crate) broad_category_difference: f64,
}

pub(crate) struct OutlayFunctionProfile {
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) year_count: usize,
    pub(crate) record_count: usize,
    pub(crate) checks: Vec<OutlayFunctionCheck>,
}

#[derive(Clone, Copy)]
pub(crate) struct Table32NationalDefenseLine {
    pub(crate) source_row: i64,
    pub(crate) subfunction_code: Option<&'static str>,
    pub(crate) subfunction_label: Option<&'static str>,
    pub(crate) source_label: &'static str,
    pub(crate) notes: &'static str,
}

#[derive(Clone)]
pub(crate) struct Table32OutlayFunctionRow {
    pub(crate) fiscal_year: i64,
    pub(crate) source_column: String,
    pub(crate) source_row: i64,
    pub(crate) function_code: &'static str,
    pub(crate) function_label: &'static str,
    pub(crate) subfunction_code: Option<&'static str>,
    pub(crate) subfunction_label: Option<&'static str>,
    pub(crate) source_label: &'static str,
    pub(crate) amount: f64,
    pub(crate) notes: &'static str,
}

pub(crate) struct Table32NationalDefenseCheck {
    pub(crate) year: i64,
    pub(crate) table_3_1_national_defense: f64,
    pub(crate) table_3_2_national_defense: f64,
    pub(crate) subfunction_total: f64,
    pub(crate) table_3_1_difference: f64,
    pub(crate) subfunction_difference: f64,
}

pub(crate) struct Table32NationalDefenseProfile {
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) year_count: usize,
    pub(crate) record_count: usize,
    pub(crate) checks: Vec<Table32NationalDefenseCheck>,
}

#[derive(Clone)]
pub(crate) enum Table32LineKind {
    Subfunction,
    FunctionTotal,
    GrandTotal,
}

#[derive(Clone)]
pub(crate) struct Table32Line {
    pub(crate) source_row: i64,
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) subfunction_code: Option<String>,
    pub(crate) subfunction_label: Option<String>,
    pub(crate) source_label: String,
    pub(crate) kind: Table32LineKind,
}

#[derive(Clone)]
pub(crate) struct Table32Row {
    pub(crate) fiscal_year: i64,
    pub(crate) source_column: String,
    pub(crate) source_row: i64,
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) subfunction_code: Option<String>,
    pub(crate) subfunction_label: Option<String>,
    pub(crate) source_label: String,
    pub(crate) amount: f64,
    pub(crate) kind: Table32LineKind,
}

pub(crate) struct Table32FunctionCheck {
    pub(crate) year: i64,
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) function_total: f64,
    pub(crate) subfunction_total: f64,
    pub(crate) difference: f64,
}

pub(crate) struct Table32GrandCheck {
    pub(crate) year: i64,
    pub(crate) table_3_1_total_outlays: f64,
    pub(crate) table_3_2_total_outlays: f64,
    pub(crate) function_total_sum: f64,
    pub(crate) table_3_1_difference: f64,
    pub(crate) function_total_difference: f64,
}

pub(crate) struct Table32Profile {
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) year_count: usize,
    pub(crate) record_count: usize,
    pub(crate) line_count: usize,
    pub(crate) subfunction_line_count: usize,
    pub(crate) function_total_line_count: usize,
    pub(crate) function_count: usize,
    pub(crate) grand_checks: Vec<Table32GrandCheck>,
    pub(crate) function_checks: Vec<Table32FunctionCheck>,
}

#[derive(Clone)]
pub(crate) struct SubfunctionModelRow {
    pub(crate) fiscal_year: i64,
    pub(crate) source_column: String,
    pub(crate) source_row: i64,
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) subfunction_code: String,
    pub(crate) subfunction_label: String,
    pub(crate) subfunction_outlays_amount: f64,
    pub(crate) subfunction_total_outlays_amount: f64,
    pub(crate) total_outlays_amount: f64,
    pub(crate) individual_income_tax_receipts_amount: f64,
    pub(crate) outlay_share_percent: f64,
    pub(crate) allocation_share_percent: f64,
    pub(crate) modeled_income_tax_allocation_amount: f64,
}

pub(crate) struct SubfunctionModelCheck {
    pub(crate) year: i64,
    pub(crate) table_3_2_total_outlays: f64,
    pub(crate) subfunction_total: f64,
    pub(crate) individual_income_tax: f64,
    pub(crate) modeled_sum: f64,
    pub(crate) subfunction_total_difference: f64,
}

pub(crate) struct SubfunctionModelProfile {
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) year_count: usize,
    pub(crate) record_count: usize,
    pub(crate) subfunction_count: usize,
    pub(crate) checks: Vec<SubfunctionModelCheck>,
}

pub(crate) struct Table61NationalDefenseRow {
    pub(crate) fiscal_year: i64,
    pub(crate) source_column: String,
    pub(crate) source_row: i64,
    pub(crate) percent_of_gdp: f64,
}

pub(crate) struct Table61NationalDefenseProfile {
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) year_count: usize,
    pub(crate) samples: Vec<(i64, f64, f64)>,
}

#[derive(Clone)]
pub(crate) enum CellValue {
    Number(f64),
    Text(String),
}

#[derive(Clone)]
pub(crate) struct Table11Row {
    pub(crate) row: i64,
    pub(crate) total_receipts: f64,
    pub(crate) total_outlays: f64,
    pub(crate) surplus_or_deficit: f64,
}

#[derive(Clone)]
pub(crate) struct Table21Row {
    pub(crate) row: i64,
    pub(crate) individual_income_tax: f64,
}

#[derive(Clone)]
pub(crate) struct AnnualRecord {
    pub(crate) fiscal_year: i64,
    pub(crate) category_key: &'static str,
    pub(crate) category_label: &'static str,
    pub(crate) table_11_row: i64,
    pub(crate) table_21_row: i64,
    pub(crate) table_31_row: i64,
    pub(crate) category_outlays_amount: f64,
    pub(crate) total_outlays_amount: f64,
    pub(crate) category_total_outlays_amount: f64,
    pub(crate) individual_income_tax_receipts_amount: f64,
    pub(crate) outlay_share_percent: f64,
    pub(crate) allocation_share_percent: f64,
    pub(crate) modeled_income_tax_allocation_amount: f64,
    pub(crate) total_receipts_amount: f64,
    pub(crate) surplus_or_deficit_amount: f64,
    pub(crate) deficit_gap_amount: f64,
    pub(crate) borrowed_share_percent_of_outlays: f64,
    pub(crate) income_tax_coverage_percent_of_outlays: f64,
    pub(crate) category_total_reconciliation_difference_amount: f64,
}

pub(crate) struct AnnualCheck {
    pub(crate) year: i64,
    pub(crate) table_1_1_outlays: f64,
    pub(crate) table_3_1_outlays: f64,
    pub(crate) category_total: f64,
    pub(crate) income_tax: f64,
    pub(crate) modeled_sum: f64,
    pub(crate) deficit_gap: f64,
}

pub(crate) struct AnnualProfile {
    pub(crate) year_count: usize,
    pub(crate) first_year: i64,
    pub(crate) last_year: i64,
    pub(crate) record_count: usize,
    pub(crate) annual_checks: Vec<AnnualCheck>,
}

#[derive(Clone)]
pub(crate) struct DecadeSummaryRow {
    pub(crate) decade: String,
    pub(crate) start_fiscal_year: i64,
    pub(crate) end_fiscal_year: i64,
    pub(crate) year_count: usize,
    pub(crate) coverage_note: &'static str,
    pub(crate) category_key: String,
    pub(crate) category_label: String,
    pub(crate) cumulative_modeled_income_tax_allocation_amount: f64,
    pub(crate) cumulative_individual_income_tax_receipts_amount: f64,
    pub(crate) category_percent_of_decade_income_tax: f64,
    pub(crate) cumulative_total_outlays_amount: f64,
    pub(crate) cumulative_total_receipts_amount: f64,
    pub(crate) cumulative_deficit_gap_amount: f64,
    pub(crate) borrowed_share_percent_of_outlays: f64,
    pub(crate) income_tax_coverage_percent_of_outlays: f64,
}

#[derive(Clone)]
pub(crate) struct SubfunctionDecadeRollup {
    pub(crate) function_code: String,
    pub(crate) function_label: String,
    pub(crate) subfunction_code: String,
    pub(crate) subfunction_label: String,
    pub(crate) subfunction_outlays: f64,
    pub(crate) modeled_allocation: f64,
}

