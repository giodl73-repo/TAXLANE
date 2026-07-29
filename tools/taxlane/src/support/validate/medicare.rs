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

pub(crate) fn validate_medicare_hi_receipt_base_reconciliation(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH,
        MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_SCHEMA_PATH,
        MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI receipt base reconciliation artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-receipt-base-reconciliation:v1"
        || string_field(&record, "record_family")? != "medicare_hi_receipt_base_reconciliation"
        || int_field(&record, "pulse")? != 139
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "receipt_base_official_source_capture_path")?
            != RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH
        || string_field(&record, "receipt_base_reconciliation_gap_path")?
            != RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH
        || string_field(&record, "current_law_fy2025_dedicated_receipt_anchors_path")?
            != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("Medicare HI receipt base reconciliation identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI receipt base reconciliation status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "cms_medicare_trustees_raw_custody_ready",
        "omb_hi_anchor_raw_custody_ready",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "reconciliation_context_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI receipt base reconciliation status {field} must be true"
            ));
        }
    }
    for field in [
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI receipt base reconciliation status {field} must be false"
            ));
        }
    }

    let context = record
        .get("source_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI source context")?;
    let taxable_payroll = context
        .get("cms_hi_taxable_payroll_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("cms_hi_taxable_payroll_musd")?;
    let cms_yield = context
        .get("cms_hi_payroll_tax_yield_context_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("cms_hi_payroll_tax_yield_context_musd")?;
    let omb_anchor = context
        .get("omb_hospital_insurance_receipt_anchor_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("omb_hospital_insurance_receipt_anchor_musd")?;
    if (taxable_payroll - 13_277_000.0).abs() > 0.001
        || (cms_yield - 400_622.16).abs() > 0.001
        || (omb_anchor - 395_350.0).abs() > 0.001
    {
        return Err("Medicare HI source context values failed".to_string());
    }
    let ratio = cms_yield / taxable_payroll;
    let recorded_ratio = context
        .get("source_yield_to_payroll_context_ratio")
        .and_then(serde_json::Value::as_f64)
        .ok_or("source_yield_to_payroll_context_ratio")?;
    let recorded_percent = context
        .get("source_yield_to_payroll_context_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("source_yield_to_payroll_context_percent")?;
    if (ratio - recorded_ratio).abs() > 0.000001
        || ((ratio * 100.0) - recorded_percent).abs() > 0.0001
        || !string_field(
            record.get("source_context").ok_or("source_context")?,
            "ratio_role",
        )?
        .contains("not_statutory_rate_not_effective_rate")
    {
        return Err("Medicare HI diagnostic ratio failed".to_string());
    }

    let reconciliation = record
        .get("reconciliation")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI reconciliation")?;
    let difference = cms_yield - omb_anchor;
    for field in [
        "cms_payroll_tax_yield_minus_omb_hi_anchor_musd",
        "absolute_difference_musd",
    ] {
        let observed = reconciliation
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or(field)?;
        if (observed - difference).abs() > 0.001 {
            return Err(format!("Medicare HI reconciliation {field} failed"));
        }
    }
    let share_cms = reconciliation
        .get("difference_as_share_of_cms_payroll_tax_yield_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("difference_as_share_of_cms_payroll_tax_yield_percent")?;
    let share_omb = reconciliation
        .get("difference_as_share_of_omb_hi_anchor_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("difference_as_share_of_omb_hi_anchor_percent")?;
    if ((difference / cms_yield * 100.0) - share_cms).abs() > 0.0001
        || ((difference / omb_anchor * 100.0) - share_omb).abs() > 0.0001
        || string_field(
            record.get("reconciliation").ok_or("reconciliation")?,
            "reconciliation_status",
        )? != "values_are_close_but_not_interchangeable_without_source_perimeter_bridge"
    {
        return Err("Medicare HI reconciliation shares failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI blocked outputs")?;
    for field in [
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!("Medicare HI blocked output {field} must be null"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI claims")?;
    if claims
        .get("medicare_hi_reconciliation_context_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI published flag must be true".to_string());
    }
    for field in [
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH,
        "The Medicare HI source reconciliation ratio is a diagnostic context ratio, not a statutory rate or effective rate.",
        "CMS payroll-tax yield context and the OMB Hospital Insurance receipt anchor are not interchangeable without a perimeter bridge.",
        "Medicare HI remains a separate trust-fund path; it is not combined Medicare financing.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI receipt base reconciliation reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_perimeter_bridge_requirements(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH,
        MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_SCHEMA_PATH,
        MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI perimeter bridge requirements artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-perimeter-bridge-requirements:v1"
        || string_field(&record, "record_family")? != "medicare_hi_perimeter_bridge_requirements"
        || int_field(&record, "pulse")? != 140
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_receipt_base_reconciliation_path")?
            != MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH
        || string_field(&record, "receipt_base_reconciliation_gap_path")?
            != RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("Medicare HI perimeter bridge requirements identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI perimeter bridge status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "bridge_requirements_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI perimeter bridge status {field} must be true"
            ));
        }
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI perimeter bridge status {field} must be false"
            ));
        }
    }

    let scope = record
        .get("bridge_scope")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge scope")?;
    for field in [
        "trust_fund_separation_required",
        "combined_medicare_prohibited",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Medicare HI bridge scope {field} must be true"));
        }
    }
    if scope
        .get("diagnostic_ratio_publishable_as_rate")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("Medicare HI diagnostic ratio must not be publishable as rate".to_string());
    }
    let cms_values = scope
        .get("cms_context_values_musd")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI CMS context values")?;
    let omb_values = scope
        .get("omb_context_values_musd")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB context values")?;
    if cms_values
        .get("hi_taxable_payroll")
        .and_then(serde_json::Value::as_f64)
        != Some(13_277_000.0)
        || cms_values
            .get("hi_payroll_tax_yield_context")
            .and_then(serde_json::Value::as_f64)
            != Some(400_622.16)
        || omb_values
            .get("hospital_insurance_receipt_anchor")
            .and_then(serde_json::Value::as_f64)
            != Some(395_350.0)
    {
        return Err("Medicare HI bridge scope values failed".to_string());
    }
    let difference = scope
        .get("unreconciled_difference_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("unreconciled_difference_musd")?;
    if (difference - 5_272.16).abs() > 0.001 {
        return Err("Medicare HI bridge difference failed".to_string());
    }

    let components = record
        .get("required_bridge_components")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI bridge components")?;
    if components.len() != 6 {
        return Err("Medicare HI bridge component count failed".to_string());
    }
    let expected_components = [
        "payroll_tax_yield_perimeter",
        "taxation_of_benefits_and_other_income_split",
        "legal_base_definition",
        "economic_base_definition",
        "solver_yield_mapping",
        "behavior_and_reform_yield",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_components = components
        .iter()
        .map(|row| string_field(row, "component_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_components != expected_components {
        return Err("Medicare HI bridge component set failed".to_string());
    }
    for row in components {
        if string_field(row, "status")? != "required_not_complete"
            || row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("Medicare HI bridge component readiness failed".to_string());
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge summary")?;
    for (field, expected) in [
        ("bridge_component_count", 6),
        ("ready_component_count", 0),
        ("blocked_component_count", 6),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("Medicare HI bridge summary {field} failed"));
        }
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI bridge summary {field} must be false"));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge blocked outputs")?;
    for field in [
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI bridge blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge claims")?;
    if claims
        .get("medicare_hi_bridge_requirements_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI bridge published flag must be true".to_string());
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI bridge claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH,
        "These are Medicare HI perimeter-bridge requirements, not a completed perimeter bridge.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "Medicare HI trust-fund separation remains required; combined Medicare financing is prohibited for this bridge.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI perimeter bridge reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_payroll_tax_perimeter_bridge(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH,
        MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_SCHEMA_PATH,
        MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI payroll tax perimeter bridge artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-payroll-tax-perimeter-bridge:v1"
        || string_field(&record, "record_family")? != "medicare_hi_payroll_tax_perimeter_bridge"
        || int_field(&record, "pulse")? != 141
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_receipt_base_reconciliation_path")?
            != MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "component_id")? != "payroll_tax_yield_perimeter"
    {
        return Err("Medicare HI payroll tax perimeter bridge identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI payroll tax perimeter bridge status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "cms_medicare_trustees_raw_custody_ready",
        "omb_hi_anchor_raw_custody_ready",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "payroll_tax_yield_perimeter_partially_evidenced",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI payroll tax perimeter bridge status {field} must be true"
            ));
        }
    }
    for field in [
        "payroll_tax_yield_perimeter_complete",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI payroll tax perimeter bridge status {field} must be false"
            ));
        }
    }

    let rows = record
        .get("cms_evidence_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI payroll tax perimeter bridge CMS rows")?;
    if rows.len() != 3 {
        return Err("Medicare HI payroll tax perimeter bridge CMS row count failed".to_string());
    }
    let payroll = rows
        .iter()
        .find(|row| string_field(row, "field").ok().as_deref() == Some("payroll_taxes"))
        .ok_or("Medicare HI payroll taxes row missing")?;
    if string_field(payroll, "period")? != "FY2025"
        || payroll
            .get("source_value_usd")
            .and_then(serde_json::Value::as_i64)
            != Some(400_622_160_000)
        || payroll
            .get("amount_musd")
            .and_then(serde_json::Value::as_f64)
            != Some(400_622.16)
        || payroll
            .get("included_in_component")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Medicare HI payroll taxes row failed".to_string());
    }
    let oasdi_benefits = rows
        .iter()
        .find(|row| {
            string_field(row, "field").ok().as_deref()
                == Some("income_from_taxation_of_oasdi_benefits")
        })
        .ok_or("Medicare HI OASDI benefits taxation row missing")?;
    if oasdi_benefits
        .get("source_value_usd")
        .and_then(serde_json::Value::as_i64)
        != Some(41_054_000_000)
        || oasdi_benefits
            .get("amount_musd")
            .and_then(serde_json::Value::as_f64)
            != Some(41_054.0)
        || oasdi_benefits
            .get("included_in_payroll_tax_yield_component")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Medicare HI OASDI benefits taxation row failed".to_string());
    }
    let definition = rows
        .iter()
        .find(|row| {
            string_field(row, "field").ok().as_deref() == Some("taxable_payroll_definition")
        })
        .ok_or("Medicare HI taxable payroll definition row missing")?;
    if definition.get("value") != Some(&serde_json::Value::Null)
        || !string_field(definition, "plain_meaning")?
            .contains("taxable wages and taxable self-employment income")
        || definition
            .get("included_in_component")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Medicare HI taxable payroll definition row failed".to_string());
    }

    let omb = record
        .get("omb_anchor_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI payroll tax perimeter bridge OMB anchor")?;
    if omb.get("amount_musd").and_then(serde_json::Value::as_f64) != Some(395_350.0)
        || omb
            .get("payroll_tax_only_perimeter_confirmed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Medicare HI payroll tax perimeter bridge OMB anchor failed".to_string());
    }

    let reconciliation = record
        .get("reconciliation")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI payroll tax perimeter bridge reconciliation")?;
    for (field, expected) in [
        ("cms_payroll_taxes_musd", 400_622.16),
        ("omb_hospital_insurance_anchor_musd", 395_350.0),
        ("cms_minus_omb_musd", 5_272.16),
        (
            "cms_payroll_tax_yield_to_hi_taxable_payroll_percent",
            3.0175,
        ),
    ] {
        let observed = reconciliation
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or(format!("Medicare HI reconciliation {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!("Medicare HI reconciliation {field} failed"));
        }
    }
    let observed_difference = reconciliation
        .get("cms_minus_omb_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("cms_minus_omb_musd")?;
    let recomputed_difference = reconciliation
        .get("cms_payroll_taxes_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("cms payroll taxes")?
        - reconciliation
            .get("omb_hospital_insurance_anchor_musd")
            .and_then(serde_json::Value::as_f64)
            .ok_or("omb hospital insurance anchor")?;
    if (observed_difference - recomputed_difference).abs() > 0.001 {
        return Err("Medicare HI CMS minus OMB formula failed".to_string());
    }
    if string_field(
        &serde_json::Value::Object(reconciliation.clone()),
        "component_status",
    )? != "partial_cms_payroll_tax_perimeter_evidenced_omb_perimeter_bridge_incomplete"
        || reconciliation
            .get("component_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || reconciliation
            .get("perimeter_bridge_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Medicare HI reconciliation status failed".to_string());
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI payroll tax perimeter still required")?;
    if still_required.len() != 5 {
        return Err("Medicare HI payroll tax perimeter still-required count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI payroll tax perimeter blocked outputs")?;
    for field in [
        "completed_payroll_tax_yield_perimeter",
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI payroll tax perimeter blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI payroll tax perimeter claims")?;
    if claims
        .get("medicare_hi_payroll_tax_perimeter_partial_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI payroll tax perimeter partial flag must be true".to_string());
    }
    for field in [
        "payroll_tax_yield_perimeter_complete",
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI payroll tax perimeter claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH,
        "The CMS payroll-tax-yield perimeter is partially evidenced, but the OMB Hospital Insurance anchor perimeter is not bridged.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "Taxation of OASDI benefits is identified separately and is not part of the payroll-tax-yield component.",
        "Medicare HI trust-fund separation remains required; combined Medicare financing is prohibited.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI payroll tax perimeter reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_benefits_tax_income_split(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH,
        MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_SCHEMA_PATH,
        MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI benefits tax income split artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-benefits-tax-income-split:v1"
        || string_field(&record, "record_family")? != "medicare_hi_benefits_tax_income_split"
        || int_field(&record, "pulse")? != 142
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_payroll_tax_perimeter_bridge_path")?
            != MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "component_id")? != "taxation_of_benefits_and_other_income_split"
    {
        return Err("Medicare HI benefits tax income split identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI benefits tax income split status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "cms_medicare_trustees_raw_custody_ready",
        "omb_hi_anchor_raw_custody_ready",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_hi_income_split_evidenced",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI benefits tax income split status {field} must be true"
            ));
        }
    }
    for field in [
        "omb_receipt_row_mapping_complete",
        "component_ready",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI benefits tax income split status {field} must be false"
            ));
        }
    }

    let source_table = record
        .get("source_table")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI benefits tax income split source table")?;
    if source_table
        .get("source_id")
        .and_then(serde_json::Value::as_str)
        != Some("SRC-CMS-MEDICARE-TRUSTEES-2026")
        || source_table
            .get("period")
            .and_then(serde_json::Value::as_str)
            != Some("FY2025")
        || source_table
            .get("total_revenue_musd")
            .and_then(serde_json::Value::as_f64)
            != Some(458_772.597)
    {
        return Err("Medicare HI benefits tax income split source table failed".to_string());
    }

    let split = record
        .get("cms_income_split_musd")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI CMS income split")?;
    for (field, expected) in [
        ("payroll_taxes", 400_622.16),
        ("income_from_taxation_of_oasdi_benefits", 41_054.0),
        ("other_non_payroll_income", 17_096.437),
        ("total_non_payroll_income", 58_150.437),
        ("total_revenue", 458_772.597),
    ] {
        let observed = split
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or(format!("Medicare HI split {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!("Medicare HI split {field} failed"));
        }
    }

    let rows = record
        .get("cms_revenue_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI revenue rows")?;
    if rows.len() != 20 {
        return Err("Medicare HI revenue row count failed".to_string());
    }
    let row_sum = rows
        .iter()
        .map(|row| {
            row.get("amount_musd")
                .and_then(serde_json::Value::as_f64)
                .ok_or("Medicare HI revenue row amount")
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();
    if (row_sum - 458_772.597).abs() > 0.001 {
        return Err("Medicare HI revenue row sum failed".to_string());
    }
    let payroll_rows = rows
        .iter()
        .filter(|row| {
            row.get("payroll_tax_yield_component")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
        })
        .collect::<Vec<_>>();
    if payroll_rows.len() != 1 || string_field(payroll_rows[0], "field")? != "payroll_taxes" {
        return Err("Medicare HI payroll component membership failed".to_string());
    }
    let benefit_taxation = rows
        .iter()
        .find(|row| {
            string_field(row, "field").ok().as_deref()
                == Some("income_from_taxation_of_oasdi_benefits")
        })
        .ok_or("Medicare HI benefit taxation row missing")?;
    if benefit_taxation
        .get("payroll_tax_yield_component")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
        || string_field(benefit_taxation, "category")? != "benefit_taxation"
    {
        return Err("Medicare HI benefit taxation category failed".to_string());
    }
    let other_non_payroll_sum = rows
        .iter()
        .filter(|row| {
            string_field(row, "category").ok().as_deref() == Some("other_non_payroll_income")
        })
        .map(|row| {
            row.get("amount_musd")
                .and_then(serde_json::Value::as_f64)
                .ok_or("Medicare HI other non-payroll row amount")
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();
    if (other_non_payroll_sum - 17_096.437).abs() > 0.001 {
        return Err("Medicare HI other non-payroll sum failed".to_string());
    }

    let reconciliation = record
        .get("reconciliation")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI benefits tax income split reconciliation")?;
    for (field, expected) in [
        ("row_sum_musd", 458_772.597),
        ("published_total_revenue_musd", 458_772.597),
        ("rounding_residual_musd", 0.0),
        ("payroll_taxes_musd", 400_622.16),
        ("benefit_taxation_musd", 41_054.0),
        ("other_non_payroll_income_musd", 17_096.437),
        ("total_non_payroll_income_musd", 58_150.437),
        (
            "cms_payroll_tax_yield_to_total_revenue_share_percent",
            87.326,
        ),
        (
            "cms_non_payroll_income_to_total_revenue_share_percent",
            12.674,
        ),
    ] {
        let observed = reconciliation
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or(format!("Medicare HI reconciliation {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!("Medicare HI reconciliation {field} failed"));
        }
    }
    let total_revenue = reconciliation
        .get("published_total_revenue_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI total revenue")?;
    let payroll = reconciliation
        .get("payroll_taxes_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI payroll taxes")?;
    let benefit = reconciliation
        .get("benefit_taxation_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI benefit taxation")?;
    let other = reconciliation
        .get("other_non_payroll_income_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI other income")?;
    if ((payroll + benefit + other) - total_revenue).abs() > 0.001
        || ((benefit + other) - 58_150.437).abs() > 0.001
    {
        return Err("Medicare HI split formulas failed".to_string());
    }
    if string_field(
        &serde_json::Value::Object(reconciliation.clone()),
        "component_status",
    )? != "cms_hi_income_split_evidenced_omb_receipt_row_mapping_incomplete"
        || reconciliation
            .get("component_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || reconciliation
            .get("perimeter_bridge_complete")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Medicare HI income split reconciliation status failed".to_string());
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI benefits tax income split still required")?;
    if still_required.len() != 5 {
        return Err(
            "Medicare HI benefits tax income split still-required count failed".to_string(),
        );
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI benefits tax income split blocked outputs")?;
    for field in [
        "completed_taxation_of_benefits_and_other_income_split",
        "omb_receipt_row_mapping",
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI benefits tax income split blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI benefits tax income split claims")?;
    if claims
        .get("cms_hi_income_split_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err(
            "Medicare HI benefits tax income split published flag must be true".to_string(),
        );
    }
    for field in [
        "omb_receipt_row_mapping_complete",
        "component_ready",
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI benefits tax income split claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH,
        "The CMS HI income split is evidenced, but the OMB Hospital Insurance receipt-row mapping is not complete.",
        "Income from taxation of OASDI benefits is non-payroll income and must not be folded into the payroll-tax-yield component.",
        "General-fund transfers, interfund interest, premiums, reimbursements, and fraud-and-abuse-control receipts are trust-fund income categories, not assigned receipt bases.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "Medicare HI trust-fund separation remains required; combined Medicare financing is prohibited.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI benefits tax income split reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_legal_base_definition_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH,
        MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_SCHEMA_PATH,
        MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI legal base definition gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-legal-base-definition-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_legal_base_definition_gap"
        || int_field(&record, "pulse")? != 143
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_payroll_tax_perimeter_bridge_path")?
            != MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH
        || string_field(&record, "medicare_hi_benefits_tax_income_split_path")?
            != MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "component_id")? != "legal_base_definition"
    {
        return Err("Medicare HI legal base definition gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base definition gap status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "cms_medicare_trustees_raw_custody_ready",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_glossary_definitions_evidenced",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI legal base definition gap status {field} must be true"
            ));
        }
    }
    for field in [
        "legal_perimeter_text_custody_ready",
        "additional_medicare_tax_treatment_ready",
        "legal_base_definition_complete",
        "economic_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base definition gap status {field} must be false"
            ));
        }
    }

    let evidence = record
        .get("cms_definition_evidence")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI legal base definition evidence")?;
    if evidence.len() != 3 {
        return Err("Medicare HI legal base definition evidence count failed".to_string());
    }
    let expected_fields = ["payroll_taxes", "taxable_earnings", "taxable_payroll"]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let observed_fields = evidence
        .iter()
        .map(|row| string_field(row, "field"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_fields != expected_fields {
        return Err("Medicare HI legal base definition evidence field set failed".to_string());
    }
    for row in evidence {
        if string_field(row, "source_id")? != "SRC-CMS-MEDICARE-TRUSTEES-2026"
            || string_field(row, "supports_component")? != "source terminology only"
            || row
                .get("legal_base_complete")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
        {
            return Err("Medicare HI legal base definition evidence row failed".to_string());
        }
    }

    let gap = record
        .get("legal_base_gap")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base gap")?;
    let terms = gap
        .get("candidate_terms_identified")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI candidate terms")?;
    if terms.len() != 5 {
        return Err("Medicare HI candidate term count failed".to_string());
    }
    for field in [
        "selected_legal_base",
        "legal_receipt_base_amount_musd",
        "statutory_perimeter_text",
        "covered_earnings_definition",
        "self_employment_treatment",
        "additional_medicare_tax_treatment",
        "wage_cap_or_no_cap_treatment",
        "tax_year_to_fiscal_year_bridge",
    ] {
        if gap.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!("Medicare HI legal base gap {field} must be null"));
        }
    }
    if gap.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
        return Err("Medicare HI legal base gap ready must be false".to_string());
    }

    let boundary = record
        .get("boundary_findings")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base boundary findings")?;
    for (field, expected) in [
        ("cms_taxable_payroll_context_musd", 13_277_000.0),
        ("cms_payroll_tax_yield_context_musd", 400_622.16),
        ("diagnostic_ratio_percent", 3.0175),
    ] {
        let observed = boundary
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or(format!("Medicare HI boundary {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!("Medicare HI boundary {field} failed"));
        }
    }
    for field in [
        "diagnostic_ratio_publishable_as_rate",
        "cms_definition_can_select_legal_base",
        "cms_definition_can_populate_assigned_base_amount",
        "cms_definition_can_populate_statutory_rate",
        "cms_definition_can_populate_effective_rate",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI boundary {field} must be false"));
        }
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI legal base still required")?;
    if still_required.len() != 5 {
        return Err("Medicare HI legal base still-required count failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base blocked outputs")?;
    for field in [
        "completed_legal_base_definition",
        "selected_legal_base",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI legal base blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base claims")?;
    if claims
        .get("cms_glossary_terms_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI legal base glossary flag must be true".to_string());
    }
    for field in [
        "legal_base_definition_complete",
        "economic_base_definition_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH,
        "CMS glossary terms clarify source terminology but do not complete the Medicare HI legal-base definition.",
        "The legal base remains unselected; gross wages, net self-employment earnings, taxable wages, taxable self-employment income, and taxable payroll are not interchangeable.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "Additional Medicare tax treatment remains unresolved and cannot be inferred from the CMS glossary alone.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI legal base reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_economic_base_definition_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH,
        MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_SCHEMA_PATH,
        MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI economic base definition gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-economic-base-definition-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_economic_base_definition_gap"
        || int_field(&record, "pulse")? != 144
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_legal_base_definition_gap_path")?
            != MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH
        || string_field(&record, "distribution_incidence_source_gap_path")?
            != DISTRIBUTION_INCIDENCE_SOURCE_GAP_JSON_PATH
        || string_field(&record, "administration_compliance_burden_source_gap_path")?
            != ADMINISTRATION_COMPLIANCE_BURDEN_SOURCE_GAP_JSON_PATH
        || string_field(&record, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
        || string_field(&record, "component_id")? != "economic_base_definition"
    {
        return Err("Medicare HI economic base definition gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base definition gap status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "economic_base_gap_defined",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI economic base definition gap status {field} must be true"
            ));
        }
    }
    for field in [
        "incidence_model_ready",
        "employer_burden_model_ready",
        "household_burden_model_ready",
        "distribution_by_income_ready",
        "administration_compliance_ready",
        "economic_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic base definition gap status {field} must be false"
            ));
        }
    }

    let components = record
        .get("required_model_components")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI economic model components")?;
    if components.len() != 5 {
        return Err("Medicare HI economic model component count failed".to_string());
    }
    let expected_components = [
        "employer_burden_model",
        "employee_burden_model",
        "household_burden_model",
        "distribution_by_income",
        "administration_and_compliance_burden",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_components = components
        .iter()
        .map(|row| string_field(row, "component"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_components != expected_components {
        return Err("Medicare HI economic model component set failed".to_string());
    }
    for row in components {
        if row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || row
                .get("required_inputs")
                .and_then(serde_json::Value::as_array)
                .map(Vec::is_empty)
                != Some(false)
        {
            return Err("Medicare HI economic model component readiness failed".to_string());
        }
    }

    let boundary = record
        .get("boundary_findings")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic boundary findings")?;
    let ratio = boundary
        .get("diagnostic_ratio_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI economic diagnostic ratio")?;
    if (ratio - 3.0175).abs() > 0.001 {
        return Err("Medicare HI economic diagnostic ratio failed".to_string());
    }
    for field in [
        "legal_base_definition_complete",
        "economic_base_definition_complete",
        "diagnostic_ratio_publishable_as_rate",
        "cms_taxable_payroll_can_substitute_for_economic_base",
        "economic_base_can_be_assumed_equal_to_legal_base",
        "rate_can_be_published_without_distribution",
        "solver_can_use_unincidenced_base",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic boundary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic blocked outputs")?;
    for field in [
        "completed_economic_base_definition",
        "incidence_model",
        "employer_burden_model",
        "employee_burden_model",
        "household_burden_model",
        "distribution_by_income",
        "administration_compliance_burden",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "public_rate_card",
        "solver_input_row",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI economic blocked output {field} must be null"
            ));
        }
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI economic still required")?;
    if still_required.len() != 5 {
        return Err("Medicare HI economic still-required count failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic claims")?;
    if claims
        .get("economic_base_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI economic gap published flag must be true".to_string());
    }
    for field in [
        "legal_base_definition_complete",
        "economic_base_definition_complete",
        "incidence_model_ready",
        "distribution_by_income_ready",
        "administration_compliance_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI economic claim {field} must be false"));
        }
    }

    let reader =
        fs::read_to_string(root.join(MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH,
        "The Medicare HI economic base is not defined by CMS taxable payroll alone.",
        "The legal base and economic burden base remain separate; neither may be silently substituted for the other.",
        "No Medicare HI rate can be published without incidence, distribution, administration, avoidance, and compliance modeling.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "No Medicare HI assigned base, rate, solver input, public rate card, tax proposal, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI economic base reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_solver_yield_mapping_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH,
        MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_SCHEMA_PATH,
        MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI solver yield mapping gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-solver-yield-mapping-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_solver_yield_mapping_gap"
        || int_field(&record, "pulse")? != 145
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_receipt_base_reconciliation_path")?
            != MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH
        || string_field(&record, "medicare_hi_benefits_tax_income_split_path")?
            != MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH
        || string_field(&record, "current_law_named_fund_balance_transfer_gap_path")?
            != CURRENT_LAW_NAMED_FUND_BALANCE_TRANSFER_GAP_JSON_PATH
        || string_field(&record, "component_id")? != "solver_yield_mapping"
    {
        return Err("Medicare HI solver yield mapping gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI solver yield mapping gap status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "solver_yield_mapping_gap_defined",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI solver yield mapping gap status {field} must be true"
            ));
        }
    }
    for field in [
        "omb_receipt_row_mapping_complete",
        "trust_fund_accounting_ready",
        "fund_balance_path_ready",
        "transfer_schedule_ready",
        "current_law_yield_matched_to_solver_ready",
        "solver_row_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI solver yield mapping gap status {field} must be false"
            ));
        }
    }

    let context = record
        .get("current_law_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI solver yield current-law context")?;
    let expected_numbers = [
        ("cms_payroll_taxes_musd", 400622.16),
        ("omb_hospital_insurance_anchor_musd", 395350.0),
        ("cms_minus_omb_musd", 5272.16),
        ("cms_total_hi_revenue_musd", 458772.597),
        ("cms_non_payroll_income_musd", 58150.437),
        ("diagnostic_ratio_percent", 3.0175),
    ];
    for (field, expected) in expected_numbers {
        let observed = context
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| format!("Medicare HI solver yield context {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!(
                "Medicare HI solver yield context {field} expected {expected}, observed {observed}"
            ));
        }
    }
    for field in [
        "diagnostic_ratio_publishable_as_rate",
        "cms_payroll_tax_yield_can_substitute_for_omb_anchor",
        "cms_total_hi_revenue_can_substitute_for_omb_anchor",
        "omb_anchor_can_substitute_for_solver_yield",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI solver yield context {field} must be false"
            ));
        }
    }

    let requirements = record
        .get("mapping_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI solver yield mapping requirements")?;
    if requirements.len() != 6 {
        return Err("Medicare HI solver yield requirement count failed".to_string());
    }
    let expected_requirements = [
        "current_law_yield_mapping",
        "explicit_trust_fund_accounting",
        "fund_balance_path",
        "transfer_schedule",
        "timing_rounding_bridge",
        "solver_row_contract",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_requirements = requirements
        .iter()
        .map(|row| string_field(row, "component"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_requirements != expected_requirements {
        return Err("Medicare HI solver yield requirement set failed".to_string());
    }
    for row in requirements {
        if row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || row
                .get("required_inputs")
                .and_then(serde_json::Value::as_array)
                .map(Vec::is_empty)
                != Some(false)
        {
            return Err("Medicare HI solver yield requirement readiness failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI solver yield blocked outputs")?;
    for field in [
        "completed_solver_yield_mapping",
        "current_law_yield_matched_to_solver",
        "solver_input_row",
        "fund_balance_path",
        "transfer_schedule",
        "explicit_general_fund_transfer",
        "interfund_transfer_schedule",
        "reserve_contribution",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI solver yield blocked output {field} must be null"
            ));
        }
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI solver yield still required")?;
    if still_required.len() != 6 {
        return Err("Medicare HI solver yield still-required count failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI solver yield claims")?;
    if claims
        .get("solver_yield_mapping_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI solver yield published flag must be true".to_string());
    }
    for field in [
        "omb_receipt_row_mapping_complete",
        "trust_fund_accounting_ready",
        "fund_balance_path_ready",
        "transfer_schedule_ready",
        "current_law_yield_matched_to_solver_ready",
        "solver_row_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI solver yield claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH,
        "The Medicare HI current-law yield mapping is not solver-ready.",
        "CMS payroll-tax yield, CMS total HI revenue, and the OMB Hospital Insurance anchor are different perimeters and cannot be substituted.",
        "Trust-fund income, explicit general-fund transfers, interfund transfers, fund balances, and timing/rounding bridges must be mapped before solver use.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "No Medicare HI solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI solver yield reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_behavior_reform_yield_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH,
        MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_SCHEMA_PATH,
        MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI behavior reform yield gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-behavior-reform-yield-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_behavior_reform_yield_gap"
        || int_field(&record, "pulse")? != 146
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
        || string_field(&record, "medicare_hi_legal_base_definition_gap_path")?
            != MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_economic_base_definition_gap_path")?
            != MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_solver_yield_mapping_gap_path")?
            != MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH
        || string_field(&record, "component_id")? != "behavior_and_reform_yield"
    {
        return Err("Medicare HI behavior reform yield gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI behavior reform yield gap status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "behavior_reform_yield_gap_defined",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI behavior reform yield gap status {field} must be true"
            ));
        }
    }
    for field in [
        "policy_instrument_selected",
        "elasticity_ready",
        "avoidance_response_ready",
        "compliance_response_ready",
        "administration_cost_ready",
        "incidence_distribution_ready",
        "trust_fund_solver_mapping_ready",
        "reform_yield_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI behavior reform yield gap status {field} must be false"
            ));
        }
    }

    let context = record
        .get("current_law_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI behavior reform current-law context")?;
    let expected_numbers = [
        ("cms_payroll_taxes_musd", 400622.16),
        ("omb_hospital_insurance_anchor_musd", 395350.0),
        ("cms_minus_omb_musd", 5272.16),
        ("cms_total_hi_revenue_musd", 458772.597),
        ("diagnostic_ratio_percent", 3.0175),
    ];
    for (field, expected) in expected_numbers {
        let observed = context
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| format!("Medicare HI behavior reform context {field} missing"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!(
                "Medicare HI behavior reform context {field} expected {expected}, observed {observed}"
            ));
        }
    }
    for field in [
        "diagnostic_ratio_publishable_as_rate",
        "current_law_context_can_supply_reform_yield",
        "current_law_context_can_supply_elasticity",
        "current_law_context_can_supply_avoidance_or_compliance",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI behavior reform context {field} must be false"
            ));
        }
    }

    let requirements = record
        .get("reform_yield_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI behavior reform requirements")?;
    if requirements.len() != 7 {
        return Err("Medicare HI behavior reform requirement count failed".to_string());
    }
    let expected_requirements = [
        "specific_policy_instrument",
        "elasticity",
        "avoidance_response",
        "compliance_response",
        "administration_cost",
        "incidence_distribution_interaction",
        "trust_fund_reform_yield_mapping",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_requirements = requirements
        .iter()
        .map(|row| string_field(row, "component"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_requirements != expected_requirements {
        return Err("Medicare HI behavior reform requirement set failed".to_string());
    }
    for row in requirements {
        if row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || row
                .get("required_inputs")
                .and_then(serde_json::Value::as_array)
                .map(Vec::is_empty)
                != Some(false)
        {
            return Err("Medicare HI behavior reform requirement readiness failed".to_string());
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI behavior reform blocked outputs")?;
    for field in [
        "completed_behavior_reform_yield",
        "policy_instrument",
        "matched_receipt_base",
        "behavioral_elasticity",
        "avoidance_response",
        "compliance_response",
        "administration_cost",
        "incidence_distribution_interaction",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "reform_delta",
        "solver_input_row",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI behavior reform blocked output {field} must be null"
            ));
        }
    }

    let still_required = record
        .get("still_required")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI behavior reform still required")?;
    if still_required.len() != 6 {
        return Err("Medicare HI behavior reform still-required count failed".to_string());
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI behavior reform claims")?;
    if claims
        .get("behavior_reform_yield_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI behavior reform published flag must be true".to_string());
    }
    for field in [
        "policy_instrument_selected",
        "matched_receipt_bases_ready",
        "elasticity_ready",
        "avoidance_response_ready",
        "compliance_response_ready",
        "administration_cost_ready",
        "incidence_distribution_ready",
        "trust_fund_solver_mapping_ready",
        "reform_yield_ready",
        "assigned_receipt_base_published",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI behavior reform claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH,
        "Medicare HI behavior and reform yield are not modeled.",
        "No reform-yield value may be inferred from current-law CMS payroll-tax yield, CMS total HI revenue, or the OMB Hospital Insurance anchor.",
        "Elasticity, avoidance, compliance, administration, incidence, distribution, and trust-fund solver mapping must be completed before reform yield use.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "No Medicare HI reform yield, solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI behavior reform reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_bridge_status_rollup(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_BRIDGE_STATUS_ROLLUP_JSON_PATH,
        MEDICARE_HI_BRIDGE_STATUS_ROLLUP_SCHEMA_PATH,
        MEDICARE_HI_BRIDGE_STATUS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI bridge rollup artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_BRIDGE_STATUS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-bridge-status-rollup:v1"
        || string_field(&record, "record_family")? != "medicare_hi_bridge_status_rollup"
        || int_field(&record, "pulse")? != 147
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_perimeter_bridge_requirements_path")?
            != MEDICARE_HI_PERIMETER_BRIDGE_REQUIREMENTS_JSON_PATH
    {
        return Err("Medicare HI bridge rollup identity failed".to_string());
    }

    let paths = record
        .get("component_record_paths")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge rollup paths")?;
    let expected_paths = [
        (
            "payroll_tax_yield_perimeter",
            MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH,
        ),
        (
            "taxation_of_benefits_and_other_income_split",
            MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH,
        ),
        (
            "legal_base_definition",
            MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH,
        ),
        (
            "economic_base_definition",
            MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH,
        ),
        (
            "solver_yield_mapping",
            MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH,
        ),
        (
            "behavior_and_reform_yield",
            MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH,
        ),
    ];
    for (field, expected) in expected_paths {
        if paths.get(field).and_then(serde_json::Value::as_str) != Some(expected) {
            return Err(format!("Medicare HI bridge rollup path {field} failed"));
        }
        if !root.join(expected).exists() {
            return Err(format!(
                "Medicare HI bridge rollup source record missing: {expected}"
            ));
        }
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge rollup status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "rollup_published",
        "all_component_records_present",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI bridge rollup status {field} must be true"
            ));
        }
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI bridge rollup status {field} must be false"
            ));
        }
    }

    let rows = record
        .get("component_status_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI bridge rollup rows")?;
    if rows.len() != 6 {
        return Err("Medicare HI bridge rollup row count failed".to_string());
    }
    let expected_components = [
        "payroll_tax_yield_perimeter",
        "taxation_of_benefits_and_other_income_split",
        "legal_base_definition",
        "economic_base_definition",
        "solver_yield_mapping",
        "behavior_and_reform_yield",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_components = rows
        .iter()
        .map(|row| string_field(row, "component_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_components != expected_components {
        return Err("Medicare HI bridge rollup component set failed".to_string());
    }
    let mut partial_count = 0;
    let mut gap_count = 0;
    for row in rows {
        let status_text = string_field(row, "status")?;
        if status_text == "partial_context_evidenced_not_complete" {
            partial_count += 1;
        } else if status_text == "gap_defined_not_complete" {
            gap_count += 1;
        } else {
            return Err(format!(
                "Medicare HI bridge rollup unexpected row status: {status_text}"
            ));
        }
        if row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || row.get("value") != Some(&serde_json::Value::Null)
            || string_field(row, "remaining_blocker")?.is_empty()
        {
            return Err("Medicare HI bridge rollup row readiness failed".to_string());
        }
    }
    if partial_count != 2 || gap_count != 4 {
        return Err("Medicare HI bridge rollup row status counts failed".to_string());
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge rollup summary")?;
    for (field, expected) in [
        ("bridge_component_count", 6),
        ("component_record_count", 6),
        ("ready_component_count", 0),
        ("blocked_component_count", 6),
        ("partial_context_component_count", 2),
        ("gap_component_count", 4),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("Medicare HI bridge rollup summary {field} failed"));
        }
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
        "public_claim_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI bridge rollup summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge rollup blocked outputs")?;
    for field in [
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI bridge rollup blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge rollup claims")?;
    if claims
        .get("medicare_hi_bridge_rollup_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI bridge rollup published flag must be true".to_string());
    }
    for field in [
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI bridge rollup claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_BRIDGE_STATUS_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_BRIDGE_STATUS_ROLLUP_JSON_PATH,
        "The Medicare HI bridge is not complete.",
        "All six Medicare HI bridge components remain not ready for assigned-base, rate, or solver use.",
        "Partial CMS context is not a matched receipt base, reform yield, assigned-base rate, or solver input.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI bridge rollup reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_bridge_closure_work_queue(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH,
        MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_SCHEMA_PATH,
        MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI bridge closure work queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let queue: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&queue, "record_id")? != "medicare-hi-bridge-closure-work-queue:v1"
        || string_field(&queue, "record_family")? != "medicare_hi_bridge_closure_work_queue"
        || int_field(&queue, "pulse")? != 148
        || string_field(&queue, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&queue, "medicare_hi_bridge_status_rollup_path")?
            != MEDICARE_HI_BRIDGE_STATUS_ROLLUP_JSON_PATH
    {
        return Err("Medicare HI bridge closure work queue identity failed".to_string());
    }

    let rules = queue
        .get("sequence_rules")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge closure sequence rules")?;
    for field in [
        "omb_cms_perimeter_bridge_before_values",
        "legal_base_before_economic_base",
        "economic_incidence_before_rates",
        "trust_fund_mapping_before_solver_rows",
        "policy_instrument_before_reform_yield",
        "behavior_before_reform_yield",
        "missing_values_remain_null",
        "blocked_gates_remain_false",
        "no_public_rate_or_solver_claim_before_all_items_ready",
    ] {
        if rules.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI bridge closure sequence rule {field} must be true"
            ));
        }
    }

    let rows = queue
        .get("work_queue")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI bridge closure work queue rows")?;
    if rows.len() != 7 {
        return Err("Medicare HI bridge closure work queue count failed".to_string());
    }
    let expected_work_ids = [
        "omb_cms_receipt_row_perimeter_bridge",
        "hi_income_category_split_to_omb_rows",
        "legal_receipt_base_definition",
        "economic_base_incidence_distribution",
        "trust_fund_solver_yield_mapping",
        "policy_behavior_reform_yield_model",
        "medicare_hi_rate_solver_readiness_review",
    ];
    let expected_component_ids = [
        "payroll_tax_yield_perimeter",
        "taxation_of_benefits_and_other_income_split",
        "legal_base_definition",
        "economic_base_definition",
        "solver_yield_mapping",
        "behavior_and_reform_yield",
        "bridge_closure_review",
    ];
    for (idx, row) in rows.iter().enumerate() {
        if int_field(row, "rank")? != (idx as i64 + 1)
            || string_field(row, "work_id")? != expected_work_ids[idx]
            || string_field(row, "component_id")? != expected_component_ids[idx]
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
            || row.get("value") != Some(&serde_json::Value::Null)
            || row
                .get("required_artifacts_before_complete")
                .and_then(serde_json::Value::as_array)
                .map(Vec::is_empty)
                != Some(false)
            || row
                .get("blocked_outputs")
                .and_then(serde_json::Value::as_array)
                .map(Vec::is_empty)
                != Some(false)
        {
            return Err(format!(
                "Medicare HI bridge closure work queue row {} failed",
                idx + 1
            ));
        }
    }

    let aggregate = queue
        .get("aggregate_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge closure aggregate status")?;
    for (field, expected) in [
        ("work_items", 7),
        ("ready_items", 0),
        ("required_bridge_components", 6),
        ("ready_bridge_components", 0),
    ] {
        if aggregate.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI bridge closure aggregate status {field} failed"
            ));
        }
    }
    for field in [
        "closure_review_ready",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
    ] {
        if aggregate.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI bridge closure aggregate status {field} must be false"
            ));
        }
    }

    let blocked = queue
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge closure blocked outputs")?;
    for field in [
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "behavioral_elasticity",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI bridge closure blocked output {field} must be null"
            ));
        }
    }

    let claims = queue
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI bridge closure claims")?;
    if claims
        .get("medicare_hi_bridge_closure_work_queue_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI bridge closure queue published flag must be true".to_string());
    }
    for field in [
        "work_item_completed",
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI bridge closure claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH,
        "The Medicare HI bridge closure queue orders work; it does not complete any bridge component.",
        "All Medicare HI bridge closure work items remain not ready.",
        "No work item may populate a value until its required artifacts are source-custodied and reconciled.",
        "Medicare HI trust-fund separation remains required; combined Medicare financing is prohibited for this bridge.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI bridge closure reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_omb_cms_receipt_row_perimeter_evidence(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH,
        MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_SCHEMA_PATH,
        MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI OMB/CMS perimeter evidence artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "medicare-hi-omb-cms-receipt-row-perimeter-evidence:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_omb_cms_receipt_row_perimeter_evidence"
        || int_field(&record, "pulse")? != 149
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_receipt_base_reconciliation_path")?
            != MEDICARE_HI_RECEIPT_BASE_RECONCILIATION_JSON_PATH
        || string_field(&record, "medicare_hi_payroll_tax_perimeter_bridge_path")?
            != MEDICARE_HI_PAYROLL_TAX_PERIMETER_BRIDGE_JSON_PATH
        || string_field(&record, "receipt_base_official_source_capture_path")?
            != RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH
    {
        return Err("Medicare HI OMB/CMS perimeter evidence identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_medicare_trustees_raw_custody_ready",
        "omb_hi_anchor_raw_custody_ready",
        "omb_cms_perimeter_evidence_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "omb_included_receipt_types_confirmed",
        "omb_excluded_receipt_types_confirmed",
        "source_row_crosswalk_complete",
        "timing_rounding_bridge_complete",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(1)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("omb_cms_receipt_row_perimeter_bridge")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("payroll_tax_yield_perimeter")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI OMB/CMS perimeter evidence work item failed".to_string());
    }

    let evidence = record
        .get("evidence_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI OMB/CMS perimeter evidence rows")?;
    if evidence.len() != 3 {
        return Err("Medicare HI OMB/CMS perimeter evidence row count failed".to_string());
    }
    let observed_fields = evidence
        .iter()
        .map(|row| string_field(row, "field"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_fields = [
        "payroll_taxes",
        "hospital_insurance_receipt_anchor",
        "income_from_taxation_of_oasdi_benefits",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_fields != expected_fields {
        return Err("Medicare HI OMB/CMS perimeter evidence field set failed".to_string());
    }
    for row in evidence {
        if row
            .get("value_ready_for_bridge")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        {
            return Err(
                "Medicare HI OMB/CMS perimeter evidence row bridge readiness failed".to_string(),
            );
        }
    }

    let reconciliation = record
        .get("reconciliation")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence reconciliation")?;
    let cms = reconciliation
        .get("cms_payroll_taxes_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI OMB/CMS cms payroll taxes")?;
    let omb = reconciliation
        .get("omb_hospital_insurance_anchor_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI OMB/CMS omb anchor")?;
    let difference = reconciliation
        .get("cms_minus_omb_musd")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI OMB/CMS difference")?;
    if (cms - 400622.16).abs() > 0.001
        || (omb - 395350.0).abs() > 0.001
        || (difference - (cms - omb)).abs() > 0.001
        || (difference - 5272.16).abs() > 0.001
    {
        return Err("Medicare HI OMB/CMS perimeter evidence arithmetic failed".to_string());
    }
    let cms_share = reconciliation
        .get("difference_as_share_of_cms_payroll_tax_yield_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI OMB/CMS cms share")?;
    let omb_share = reconciliation
        .get("difference_as_share_of_omb_hi_anchor_percent")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI OMB/CMS omb share")?;
    if (cms_share - ((difference / cms) * 100.0)).abs() > 0.01
        || (omb_share - ((difference / omb) * 100.0)).abs() > 0.01
        || reconciliation
            .get("bridge_status")
            .and_then(serde_json::Value::as_str)
            != Some("evidence_boundary_published_perimeter_bridge_not_complete")
    {
        return Err("Medicare HI OMB/CMS perimeter evidence share arithmetic failed".to_string());
    }

    let reqs = record
        .get("perimeter_bridge_requirements_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI OMB/CMS perimeter evidence requirements")?;
    if reqs.len() != 7 {
        return Err("Medicare HI OMB/CMS perimeter evidence requirement count failed".to_string());
    }
    let ready_count = reqs
        .iter()
        .filter(|row| row.get("ready").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let blocked_count = reqs
        .iter()
        .filter(|row| row.get("ready").and_then(serde_json::Value::as_bool) == Some(false))
        .count();
    if ready_count != 2 || blocked_count != 5 {
        return Err(
            "Medicare HI OMB/CMS perimeter evidence requirement readiness failed".to_string(),
        );
    }
    for row in reqs {
        if row.get("ready").and_then(serde_json::Value::as_bool) == Some(false)
            && row.get("value") != Some(&serde_json::Value::Null)
        {
            return Err(
                "Medicare HI OMB/CMS perimeter evidence blocked requirement value failed"
                    .to_string(),
            );
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence summary")?;
    for (field, expected) in [
        ("evidence_rows", 3),
        ("requirements", 7),
        ("ready_requirements", 2),
        ("blocked_requirements", 5),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence summary {field} failed"
            ));
        }
    }
    for field in [
        "perimeter_bridge_complete",
        "work_queue_item_completed",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence blocked outputs")?;
    for field in [
        "completed_omb_cms_receipt_row_perimeter_bridge",
        "perimeter_bridge_value",
        "omb_included_receipt_types",
        "omb_excluded_receipt_types",
        "source_row_crosswalk",
        "timing_rounding_bridge",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI OMB/CMS perimeter evidence claims")?;
    if claims
        .get("medicare_hi_omb_cms_perimeter_evidence_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI OMB/CMS perimeter evidence published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "perimeter_bridge_complete",
        "omb_included_receipt_types_confirmed",
        "omb_excluded_receipt_types_confirmed",
        "source_row_crosswalk_complete",
        "timing_rounding_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH,
        "The Medicare HI OMB/CMS receipt-row perimeter evidence boundary is published, but the perimeter bridge is not complete.",
        "CMS payroll taxes and the OMB Hospital Insurance receipt anchor remain different source perimeters and cannot be substituted.",
        "OMB included and excluded Hospital Insurance receipt types remain unconfirmed in this bridge.",
        "The Medicare HI diagnostic ratio remains blocked from statutory-rate and effective-rate publication.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI OMB/CMS perimeter evidence reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_income_category_omb_mapping_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH,
        MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_SCHEMA_PATH,
        MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI income category OMB mapping gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-income-category-omb-mapping-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_income_category_omb_mapping_gap"
        || int_field(&record, "pulse")? != 150
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_benefits_tax_income_split_path")?
            != MEDICARE_HI_BENEFITS_TAX_INCOME_SPLIT_JSON_PATH
        || string_field(
            &record,
            "medicare_hi_omb_cms_receipt_row_perimeter_evidence_path",
        )? != MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH
    {
        return Err("Medicare HI income category OMB mapping gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category OMB mapping status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_hi_income_split_evidenced",
        "income_category_omb_mapping_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI income category OMB mapping status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "payroll_tax_omb_mapping_complete",
        "benefit_taxation_omb_mapping_complete",
        "other_income_omb_mapping_complete",
        "omb_cms_crosswalk_complete",
        "residual_explanation_complete",
        "component_ready",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI income category OMB mapping status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category OMB mapping work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(2)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("hi_income_category_split_to_omb_rows")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("taxation_of_benefits_and_other_income_split")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI income category OMB mapping work item failed".to_string());
    }

    let context = record
        .get("cms_income_category_context_musd")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category context")?;
    let payroll = context
        .get("payroll_taxes")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI payroll context")?;
    let benefits = context
        .get("income_from_taxation_of_oasdi_benefits")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI benefits taxation context")?;
    let other = context
        .get("other_non_payroll_income")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI other income context")?;
    let non_payroll = context
        .get("total_non_payroll_income")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI non-payroll context")?;
    let total = context
        .get("total_revenue")
        .and_then(serde_json::Value::as_f64)
        .ok_or("Medicare HI total revenue context")?;
    if (payroll - 400622.16).abs() > 0.001
        || (benefits - 41054.0).abs() > 0.001
        || (other - 17096.437).abs() > 0.001
        || (non_payroll - (benefits + other)).abs() > 0.001
        || (total - (payroll + non_payroll)).abs() > 0.001
        || (total - 458772.597).abs() > 0.001
    {
        return Err("Medicare HI income category arithmetic failed".to_string());
    }

    let groups = record
        .get("cms_income_category_groups")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI income category groups")?;
    if groups.len() != 3 {
        return Err("Medicare HI income category group count failed".to_string());
    }
    for group in groups {
        if group.get("omb_row_mapping") != Some(&serde_json::Value::Null)
            || group.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("Medicare HI income category group readiness failed".to_string());
        }
    }

    let reqs = record
        .get("mapping_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI income category mapping requirements")?;
    if reqs.len() != 6
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI income category mapping requirements failed".to_string());
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category summary")?;
    for (field, expected) in [
        ("cms_income_category_groups", 3),
        ("mapping_requirements", 6),
        ("ready_mapping_requirements", 0),
        ("blocked_mapping_requirements", 6),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI income category summary {field} failed"
            ));
        }
    }
    for field in [
        "work_queue_item_completed",
        "component_ready",
        "perimeter_bridge_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI income category summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category blocked outputs")?;
    for field in [
        "completed_income_category_omb_mapping",
        "omb_receipt_row_mapping",
        "payroll_tax_omb_mapping",
        "benefit_taxation_omb_mapping",
        "other_income_omb_mapping",
        "excluded_categories",
        "omb_cms_crosswalk",
        "residual_explanation",
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI income category blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI income category claims")?;
    if claims
        .get("medicare_hi_income_category_omb_mapping_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI income category published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "payroll_tax_omb_mapping_complete",
        "benefit_taxation_omb_mapping_complete",
        "other_income_omb_mapping_complete",
        "omb_cms_crosswalk_complete",
        "residual_explanation_complete",
        "component_ready",
        "perimeter_bridge_complete",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI income category claim {field} must be false"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH,
        "The CMS HI income categories are evidenced, but mapping them to OMB Hospital Insurance receipt rows remains incomplete.",
        "Income from taxation of OASDI benefits is non-payroll income and must not be folded into the payroll-tax-yield component.",
        "General-fund transfers, interfund interest, premiums, reimbursements, and fraud-and-abuse-control receipts are trust-fund income categories, not assigned receipt bases.",
        "No OMB/CMS income-category crosswalk, excluded-category list, or residual explanation is complete.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI income category reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_legal_base_closure_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_SCHEMA_PATH,
        MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI legal base closure gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-legal-base-closure-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_legal_base_closure_gap"
        || int_field(&record, "pulse")? != 151
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_legal_base_definition_gap_path")?
            != MEDICARE_HI_LEGAL_BASE_DEFINITION_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_income_category_omb_mapping_gap_path")?
            != MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH
    {
        return Err("Medicare HI legal base closure gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base closure status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "cms_glossary_definitions_evidenced",
        "legal_base_closure_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI legal base closure status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "official_legal_perimeter_text_ready",
        "covered_earnings_definition_ready",
        "self_employment_treatment_ready",
        "additional_medicare_tax_treatment_ready",
        "tax_year_to_fiscal_year_bridge_ready",
        "source_custodied_base_amount_ready",
        "legal_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base closure status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base closure work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(3)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("legal_receipt_base_definition")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("legal_base_definition")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI legal base closure work item failed".to_string());
    }

    let candidates = record
        .get("candidate_terms")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI legal base closure candidates")?;
    if candidates.len() != 5 {
        return Err("Medicare HI legal base candidate count failed".to_string());
    }

    let reqs = record
        .get("legal_base_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI legal base requirements")?;
    if reqs.len() != 6
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI legal base requirements must remain blocked".to_string());
    }

    let gap = record
        .get("legal_base_gap")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base gap")?;
    for field in [
        "selected_legal_base",
        "legal_receipt_base_amount_musd",
        "statutory_perimeter_text",
        "covered_earnings_definition",
        "self_employment_treatment",
        "additional_medicare_tax_treatment",
        "wage_cap_or_no_cap_treatment",
        "tax_year_to_fiscal_year_bridge",
    ] {
        if gap.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!("Medicare HI legal base gap {field} must be null"));
        }
    }
    if gap.get("ready").and_then(serde_json::Value::as_bool) != Some(false) {
        return Err("Medicare HI legal base gap ready must be false".to_string());
    }

    let context = record
        .get("context_values")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base context")?;
    if context
        .get("cms_taxable_payroll_context_musd")
        .and_then(serde_json::Value::as_i64)
        != Some(13277000)
        || (context
            .get("cms_payroll_tax_yield_context_musd")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Medicare HI payroll tax yield context")?
            - 400622.16)
            .abs()
            > 0.001
        || (context
            .get("diagnostic_ratio_percent")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Medicare HI diagnostic ratio")?
            - 3.0175)
            .abs()
            > 0.001
    {
        return Err("Medicare HI legal base context values failed".to_string());
    }
    for field in [
        "diagnostic_ratio_publishable_as_rate",
        "cms_taxable_payroll_can_be_selected_as_legal_base",
        "cms_payroll_tax_yield_can_define_legal_base",
        "cms_glossary_can_resolve_additional_medicare_tax",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base context {field} must be false"
            ));
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base closure summary")?;
    for (field, expected) in [
        ("candidate_terms", 5),
        ("legal_base_requirements", 6),
        ("ready_requirements", 0),
        ("blocked_requirements", 6),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("Medicare HI legal base summary {field} failed"));
        }
    }
    for field in [
        "work_queue_item_completed",
        "legal_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base blocked outputs")?;
    for field in [
        "completed_legal_base_definition",
        "selected_legal_base",
        "official_legal_perimeter_text",
        "covered_earnings_definition",
        "self_employment_treatment",
        "additional_medicare_tax_treatment",
        "tax_year_to_fiscal_year_bridge",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI legal base blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI legal base claims")?;
    if claims
        .get("medicare_hi_legal_base_closure_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI legal base published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "legal_base_definition_complete",
        "official_legal_perimeter_text_ready",
        "covered_earnings_definition_ready",
        "self_employment_treatment_ready",
        "additional_medicare_tax_treatment_ready",
        "tax_year_to_fiscal_year_bridge_ready",
        "source_custodied_base_amount_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI legal base claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH,
        "The Medicare HI legal-base closure gap is published, but the legal base remains unselected.",
        "CMS glossary terms clarify source terminology but do not complete the Medicare HI legal-base definition.",
        "Gross wages, net self-employment earnings, taxable wages, taxable self-employment income, and taxable payroll are not interchangeable.",
        "Additional Medicare tax treatment remains unresolved and cannot be inferred from the CMS glossary alone.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI legal base reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_economic_base_closure_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_SCHEMA_PATH,
        MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI economic base closure gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-economic-base-closure-gap:v1"
        || string_field(&record, "record_family")? != "medicare_hi_economic_base_closure_gap"
        || int_field(&record, "pulse")? != 152
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_economic_base_definition_gap_path")?
            != MEDICARE_HI_ECONOMIC_BASE_DEFINITION_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_legal_base_closure_gap_path")?
            != MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH
    {
        return Err("Medicare HI economic base closure gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base closure status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "economic_base_closure_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI economic base closure status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "legal_base_selected",
        "incidence_model_ready",
        "employer_burden_model_ready",
        "employee_burden_model_ready",
        "household_burden_model_ready",
        "distribution_by_income_ready",
        "administration_compliance_ready",
        "avoidance_compliance_baseline_ready",
        "economic_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic base closure status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base closure work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(4)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("economic_base_incidence_distribution")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("economic_base_definition")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI economic base closure work item failed".to_string());
    }

    let reqs = record
        .get("economic_base_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI economic base requirements")?;
    if reqs.len() != 7
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI economic base requirements must remain blocked".to_string());
    }

    let components = record
        .get("model_component_gaps")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI economic base model components")?;
    if components.len() != 6
        || components
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || components
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI economic base model components must remain blocked".to_string());
    }
    let observed_components = components
        .iter()
        .map(|row| string_field(row, "component"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_components = [
        "employer_burden_model",
        "employee_burden_model",
        "household_burden_model",
        "distribution_by_income",
        "administration_and_compliance_burden",
        "tax_interaction_scoring",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_components != expected_components {
        return Err("Medicare HI economic base component set failed".to_string());
    }

    let context = record
        .get("context_values")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base context")?;
    if context
        .get("cms_taxable_payroll_context_musd")
        .and_then(serde_json::Value::as_i64)
        != Some(13277000)
        || (context
            .get("cms_payroll_tax_yield_context_musd")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Medicare HI economic payroll tax yield context")?
            - 400622.16)
            .abs()
            > 0.001
        || (context
            .get("diagnostic_ratio_percent")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Medicare HI economic diagnostic ratio")?
            - 3.0175)
            .abs()
            > 0.001
    {
        return Err("Medicare HI economic base context values failed".to_string());
    }
    for field in [
        "diagnostic_ratio_publishable_as_rate",
        "cms_taxable_payroll_can_substitute_for_economic_base",
        "legal_base_can_substitute_for_economic_base",
        "rate_can_be_published_without_distribution",
        "solver_can_use_unincidenced_base",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic base context {field} must be false"
            ));
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base summary")?;
    for (field, expected) in [
        ("economic_base_requirements", 7),
        ("ready_requirements", 0),
        ("blocked_requirements", 7),
        ("model_component_gaps", 6),
        ("ready_model_components", 0),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("Medicare HI economic base summary {field} failed"));
        }
    }
    for field in [
        "work_queue_item_completed",
        "economic_base_definition_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic base summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base blocked outputs")?;
    for field in [
        "completed_economic_base_definition",
        "incidence_model",
        "employer_burden_model",
        "employee_burden_model",
        "household_burden_model",
        "distribution_by_income",
        "administration_compliance_burden",
        "avoidance_and_compliance_baseline",
        "tax_interaction_scoring",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI economic base blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI economic base claims")?;
    if claims
        .get("medicare_hi_economic_base_closure_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI economic base published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "legal_base_selected",
        "economic_base_definition_complete",
        "incidence_model_ready",
        "employer_burden_model_ready",
        "employee_burden_model_ready",
        "household_burden_model_ready",
        "distribution_by_income_ready",
        "administration_compliance_ready",
        "avoidance_compliance_baseline_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI economic base claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH,
        "The Medicare HI economic-base closure gap is published, but the economic base remains undefined.",
        "The Medicare HI economic base is not defined by CMS taxable payroll alone.",
        "The legal base and economic burden base remain separate; neither may be silently substituted for the other.",
        "No Medicare HI rate can be published without incidence, distribution, administration, avoidance, and compliance modeling.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI economic base reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_trust_fund_solver_yield_closure_gap(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_SCHEMA_PATH,
        MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI trust-fund solver-yield closure gap artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH))
            .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-trust-fund-solver-yield-closure-gap:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_trust_fund_solver_yield_closure_gap"
        || int_field(&record, "pulse")? != 153
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_solver_yield_mapping_gap_path")?
            != MEDICARE_HI_SOLVER_YIELD_MAPPING_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_economic_base_closure_gap_path")?
            != MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH
    {
        return Err("Medicare HI trust-fund solver-yield closure gap identity failed".to_string());
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield closure status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "trust_fund_solver_yield_closure_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "current_law_yield_selection_ready",
        "trust_fund_income_fields_ready",
        "explicit_general_fund_transfers_ready",
        "interfund_transfers_ready",
        "fund_balance_path_ready",
        "timing_bridge_ready",
        "rounding_line_ready",
        "solver_row_contract_ready",
        "solver_yield_mapping_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(5)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("trust_fund_solver_yield_mapping")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("solver_yield_mapping")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI trust-fund solver-yield work item failed".to_string());
    }

    let context = record
        .get("current_law_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield current-law context")?;
    for (field, expected) in [
        ("cms_payroll_taxes_musd", 400622.16),
        ("omb_hospital_insurance_anchor_musd", 395350.0),
        ("cms_minus_omb_musd", 5272.16),
        ("cms_total_hi_revenue_musd", 458772.597),
        ("cms_non_payroll_income_musd", 58150.437),
        ("diagnostic_ratio_percent", 3.0175),
    ] {
        let observed = context
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| format!("Medicare HI trust-fund solver-yield context {field}"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!(
                "Medicare HI trust-fund solver-yield context {field} failed"
            ));
        }
    }
    for field in [
        "cms_payroll_tax_yield_can_substitute_for_solver_yield",
        "cms_total_hi_revenue_can_substitute_for_solver_yield",
        "omb_anchor_can_substitute_for_solver_yield",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield context {field} must be false"
            ));
        }
    }

    let reqs = record
        .get("solver_yield_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI trust-fund solver-yield requirements")?;
    if reqs.len() != 8
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err(
            "Medicare HI trust-fund solver-yield requirements must remain blocked".to_string(),
        );
    }
    let observed_requirements = reqs
        .iter()
        .map(|row| string_field(row, "required_artifact"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_requirements = [
        "current-law yield selection",
        "trust-fund income fields",
        "explicit general-fund transfers",
        "interfund transfers",
        "fund balance path",
        "timing bridge",
        "rounding line",
        "solver row contract",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_requirements != expected_requirements {
        return Err("Medicare HI trust-fund solver-yield requirement set failed".to_string());
    }

    let blocked_fields = record
        .get("blocked_solver_fields")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund blocked solver fields")?;
    for field in [
        "current_law_yield_matched_to_solver",
        "trust_fund_income_fields",
        "explicit_general_fund_transfer",
        "interfund_transfer_schedule",
        "fund_balance_path",
        "timing_bridge",
        "rounding_line",
        "solver_row_contract",
    ] {
        if blocked_fields.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI trust-fund blocked solver field {field} must be null"
            ));
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield summary")?;
    for (field, expected) in [
        ("solver_yield_requirements", 8),
        ("ready_requirements", 0),
        ("blocked_requirements", 8),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield summary {field} failed"
            ));
        }
    }
    for field in [
        "work_queue_item_completed",
        "solver_yield_mapping_complete",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield blocked outputs")?;
    for field in [
        "completed_solver_yield_mapping",
        "current_law_yield_matched_to_solver",
        "trust_fund_income_fields",
        "explicit_general_fund_transfer",
        "interfund_transfer_schedule",
        "fund_balance_path",
        "timing_bridge",
        "rounding_line",
        "solver_row_contract",
        "solver_input_row",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI trust-fund solver-yield claims")?;
    if claims
        .get("medicare_hi_trust_fund_solver_yield_closure_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI trust-fund solver-yield published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "solver_yield_mapping_complete",
        "current_law_yield_selection_ready",
        "trust_fund_income_fields_ready",
        "explicit_general_fund_transfers_ready",
        "interfund_transfers_ready",
        "fund_balance_path_ready",
        "timing_bridge_ready",
        "rounding_line_ready",
        "solver_row_contract_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield claim {field} must be false"
            ));
        }
    }

    let warnings = record
        .get("public_warning_phrases")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI trust-fund solver-yield warnings")?;
    for phrase in [
        "The Medicare HI trust-fund solver-yield closure gap is published, but no solver-yield mapping is complete.",
        "CMS payroll-tax yield, CMS total HI revenue, and the OMB Hospital Insurance anchor remain different perimeters and cannot be substituted for solver yield.",
        "Medicare HI must remain a separate trust fund with explicit transfers, fund balances, timing bridge, and rounding line before solver use.",
        "No Medicare HI solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "This record is not solver input, not a solver run, not a target-cost selection, not a rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.",
    ] {
        if !warnings
            .iter()
            .any(|warning| warning.as_str() == Some(phrase))
        {
            return Err(format!(
                "Medicare HI trust-fund solver-yield warning missing phrase: {phrase}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH,
        "The Medicare HI trust-fund solver-yield closure gap is published, but no solver-yield mapping is complete.",
        "CMS payroll-tax yield, CMS total HI revenue, and the OMB Hospital Insurance anchor remain different perimeters and cannot be substituted for solver yield.",
        "Medicare HI must remain a separate trust fund with explicit transfers, fund balances, timing bridge, and rounding line before solver use.",
        "No Medicare HI solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI trust-fund solver-yield reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_policy_behavior_reform_yield_closure_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_SCHEMA_PATH,
        MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI policy-behavior reform-yield closure gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "medicare-hi-policy-behavior-reform-yield-closure-gap:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_policy_behavior_reform_yield_closure_gap"
        || int_field(&record, "pulse")? != 154
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(&record, "medicare_hi_behavior_reform_yield_gap_path")?
            != MEDICARE_HI_BEHAVIOR_REFORM_YIELD_GAP_JSON_PATH
        || string_field(
            &record,
            "medicare_hi_trust_fund_solver_yield_closure_gap_path",
        )? != MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH
    {
        return Err(
            "Medicare HI policy-behavior reform-yield closure gap identity failed".to_string(),
        );
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield closure status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "policy_behavior_reform_yield_closure_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "policy_instrument_selected",
        "phase_in_ready",
        "matched_base_ready",
        "elasticity_ready",
        "avoidance_response_ready",
        "compliance_response_ready",
        "administration_cost_ready",
        "incidence_distribution_ready",
        "trust_fund_reform_delta_ready",
        "reform_yield_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(6)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("policy_behavior_reform_yield_model")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("behavior_and_reform_yield")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI policy-behavior reform-yield work item failed".to_string());
    }

    let context = record
        .get("current_law_context")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield current-law context")?;
    for (field, expected) in [
        ("cms_payroll_taxes_musd", 400622.16),
        ("omb_hospital_insurance_anchor_musd", 395350.0),
        ("cms_minus_omb_musd", 5272.16),
        ("cms_total_hi_revenue_musd", 458772.597),
        ("diagnostic_ratio_percent", 3.0175),
    ] {
        let observed = context
            .get(field)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| format!("Medicare HI policy-behavior context {field}"))?;
        if (observed - expected).abs() > 0.001 {
            return Err(format!(
                "Medicare HI policy-behavior context {field} failed"
            ));
        }
    }
    for field in [
        "current_law_context_can_supply_policy_instrument",
        "current_law_context_can_supply_reform_yield",
        "current_law_context_can_supply_elasticity",
        "current_law_context_can_supply_avoidance_or_compliance",
        "phi_sensitivity_can_supply_reform_yield",
    ] {
        if context.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI policy-behavior context {field} must be false"
            ));
        }
    }

    let reqs = record
        .get("reform_yield_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI policy-behavior reform-yield requirements")?;
    if reqs.len() != 9
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err(
            "Medicare HI policy-behavior reform-yield requirements must remain blocked".to_string(),
        );
    }
    let observed_requirements = reqs
        .iter()
        .map(|row| string_field(row, "required_artifact"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_requirements = [
        "specific Medicare HI policy instrument",
        "annual phase-in and effective-date schedule",
        "matched legal and economic receipt base",
        "elasticity model with source provenance",
        "avoidance and reclassification response",
        "compliance and enforcement response",
        "administration, employer, and taxpayer burden cost",
        "incidence and distribution by income",
        "trust-fund reform delta and solver-row mapping",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_requirements != expected_requirements {
        return Err("Medicare HI policy-behavior reform-yield requirement set failed".to_string());
    }

    let blocked_model = record
        .get("blocked_model_fields")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior blocked model fields")?;
    for field in [
        "policy_instrument",
        "phase_in_schedule",
        "matched_legal_base",
        "matched_economic_base",
        "behavioral_elasticity",
        "avoidance_response",
        "compliance_response",
        "administration_cost",
        "employer_burden",
        "taxpayer_burden",
        "agency_burden",
        "incidence_distribution",
        "trust_fund_reform_delta",
        "reform_yield",
    ] {
        if blocked_model.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI policy-behavior blocked model field {field} must be null"
            ));
        }
    }

    let summary = record
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield summary")?;
    for (field, expected) in [
        ("reform_yield_requirements", 9),
        ("ready_requirements", 0),
        ("blocked_requirements", 9),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield summary {field} failed"
            ));
        }
    }
    for field in [
        "work_queue_item_completed",
        "policy_behavior_reform_yield_model_complete",
        "reform_yield_ready",
        "assigned_base_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield blocked outputs")?;
    for field in [
        "completed_policy_behavior_reform_yield_model",
        "policy_instrument",
        "phase_in_schedule",
        "matched_receipt_base",
        "behavioral_elasticity",
        "avoidance_response",
        "compliance_response",
        "administration_cost",
        "incidence_distribution",
        "trust_fund_reform_delta",
        "reform_yield",
        "reform_delta",
        "solver_input_row",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI policy-behavior reform-yield claims")?;
    if claims
        .get("medicare_hi_policy_behavior_reform_yield_closure_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI policy-behavior reform-yield published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "policy_behavior_reform_yield_model_complete",
        "policy_instrument_selected",
        "phase_in_ready",
        "matched_receipt_bases_ready",
        "elasticity_ready",
        "avoidance_response_ready",
        "compliance_response_ready",
        "administration_cost_ready",
        "incidence_distribution_ready",
        "trust_fund_reform_delta_ready",
        "reform_yield_ready",
        "assigned_receipt_base_published",
        "rate_publication_ready",
        "solver_inputs_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield claim {field} must be false"
            ));
        }
    }

    let warnings = record
        .get("public_warning_phrases")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI policy-behavior reform-yield warnings")?;
    for phrase in [
        "The Medicare HI policy-behavior reform-yield closure gap is published, but no policy instrument or reform-yield model is complete.",
        "No reform-yield value may be inferred from current-law CMS payroll-tax yield, CMS total HI revenue, the OMB Hospital Insurance anchor, or private-insurance payment sensitivity.",
        "Policy instrument, phase-in, matched base, elasticity, avoidance, compliance, administration, incidence, distribution, and trust-fund reform-delta mapping must be completed before reform-yield use.",
        "No Medicare HI reform yield, solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "This record is not solver input, not a solver run, not a target-cost selection, not a rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.",
    ] {
        if !warnings
            .iter()
            .any(|warning| warning.as_str() == Some(phrase))
        {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH,
        "The Medicare HI policy-behavior reform-yield closure gap is published, but no policy instrument or reform-yield model is complete.",
        "No reform-yield value may be inferred from current-law CMS payroll-tax yield, CMS total HI revenue, the OMB Hospital Insurance anchor, or private-insurance payment sensitivity.",
        "Policy instrument, phase-in, matched base, elasticity, avoidance, compliance, administration, incidence, distribution, and trust-fund reform-delta mapping must be completed before reform-yield use.",
        "No Medicare HI reform yield, solver row, assigned base, rate, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI policy-behavior reform-yield reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_rate_solver_readiness_review_closure_gap(
    root: &Path,
) -> Result<(), String> {
    for path in [
        MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_SCHEMA_PATH,
        MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI rate and solver readiness review closure gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_JSON_PATH),
    )
    .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")?
        != "medicare-hi-rate-solver-readiness-review-closure-gap:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_rate_solver_readiness_review_closure_gap"
        || int_field(&record, "pulse")? != 155
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
        || string_field(
            &record,
            "medicare_hi_omb_cms_receipt_row_perimeter_evidence_path",
        )? != MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH
        || string_field(&record, "medicare_hi_income_category_omb_mapping_gap_path")?
            != MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_legal_base_closure_gap_path")?
            != MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH
        || string_field(&record, "medicare_hi_economic_base_closure_gap_path")?
            != MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH
        || string_field(
            &record,
            "medicare_hi_trust_fund_solver_yield_closure_gap_path",
        )? != MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH
        || string_field(
            &record,
            "medicare_hi_policy_behavior_reform_yield_closure_gap_path",
        )? != MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH
    {
        return Err(
            "Medicare HI rate and solver readiness review closure gap identity failed".to_string(),
        );
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI rate and solver readiness review status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "rate_solver_readiness_review_closure_gap_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI rate and solver readiness review status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "all_six_bridge_items_ready",
        "null_false_audit_passed_for_readiness",
        "public_warning_review_passed",
        "rate_readiness_check_passed",
        "solver_readiness_check_passed",
        "manifest_update_ready_for_closure",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI rate and solver readiness review status {field} must be false"
            ));
        }
    }

    let item = record
        .get("work_queue_item")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI rate and solver readiness review work item")?;
    if item.get("rank").and_then(serde_json::Value::as_i64) != Some(7)
        || item.get("work_id").and_then(serde_json::Value::as_str)
            != Some("medicare_hi_rate_solver_readiness_review")
        || item.get("component_id").and_then(serde_json::Value::as_str)
            != Some("bridge_closure_review")
        || item.get("completed").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        || item.get("value") != Some(&serde_json::Value::Null)
    {
        return Err("Medicare HI rate and solver readiness review work item failed".to_string());
    }

    let prereqs = record
        .get("prerequisite_bridge_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI readiness prerequisite bridge items")?;
    if prereqs.len() != 6
        || prereqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || prereqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI prerequisite bridge items must remain blocked".to_string());
    }
    let observed_prereqs = prereqs
        .iter()
        .map(|row| Ok((int_field(row, "rank")?, string_field(row, "work_id")?)))
        .collect::<Result<BTreeSet<_>, String>>()?;
    let expected_prereqs = [
        (1, "omb_cms_receipt_row_perimeter_bridge"),
        (2, "hi_income_category_split_to_omb_rows"),
        (3, "legal_receipt_base_definition"),
        (4, "economic_base_incidence_distribution"),
        (5, "trust_fund_solver_yield_mapping"),
        (6, "policy_behavior_reform_yield_model"),
    ]
    .into_iter()
    .map(|(rank, work_id)| (rank, work_id.to_string()))
    .collect::<BTreeSet<_>>();
    if observed_prereqs != expected_prereqs {
        return Err("Medicare HI prerequisite bridge item set failed".to_string());
    }

    let reqs = record
        .get("readiness_review_requirements")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI readiness review requirements")?;
    if reqs.len() != 6
        || reqs
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || reqs
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err("Medicare HI readiness review requirements must remain blocked".to_string());
    }
    let observed_requirements = reqs
        .iter()
        .map(|row| string_field(row, "required_artifact"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_requirements = [
        "all six bridge items ready",
        "null/false audit passed",
        "public warning review",
        "rate readiness check",
        "solver readiness check",
        "manifest update",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_requirements != expected_requirements {
        return Err("Medicare HI readiness review requirement set failed".to_string());
    }

    let summary = record
        .get("readiness_summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI readiness review summary")?;
    for (field, expected) in [
        ("work_queue_items", 7),
        ("prerequisite_bridge_items", 6),
        ("ready_prerequisite_bridge_items", 0),
        ("readiness_review_requirements", 6),
        ("ready_review_requirements", 0),
        ("blocked_review_requirements", 6),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "Medicare HI readiness review summary {field} failed"
            ));
        }
    }
    for field in [
        "work_queue_item_completed",
        "closure_review_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI readiness review summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI readiness review blocked outputs")?;
    for field in [
        "completed_readiness_review",
        "all_six_bridge_items_ready",
        "null_false_audit",
        "public_warning_review",
        "rate_readiness_check",
        "solver_readiness_check",
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI readiness review blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI readiness review claims")?;
    if claims
        .get("medicare_hi_rate_solver_readiness_review_closure_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("Medicare HI readiness review published flag failed".to_string());
    }
    for field in [
        "work_item_completed",
        "all_six_bridge_items_ready",
        "closure_review_ready",
        "null_false_audit_passed_for_readiness",
        "public_warning_review_passed",
        "rate_readiness_check_passed",
        "solver_readiness_check_passed",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI readiness review claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_READER_PATH),
    )
    .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_JSON_PATH,
        "The Medicare HI rate and solver readiness review gap is published, but the readiness review is not complete.",
        "Zero of six Medicare HI prerequisite bridge items are ready.",
        "Medicare HI cannot publish assigned-base rates, solver rows, public rate cards, tax proposals, savings estimates, or balanced-budget claims before all bridge items are complete.",
        "Medicare HI remains a separate trust fund; combined Medicare financing is prohibited for this readiness review.",
        "No Medicare HI rate publication, solver input, public claim, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI readiness review reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_closure_series_rollup(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_CLOSURE_SERIES_ROLLUP_JSON_PATH,
        MEDICARE_HI_CLOSURE_SERIES_ROLLUP_SCHEMA_PATH,
        MEDICARE_HI_CLOSURE_SERIES_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI closure series rollup artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(MEDICARE_HI_CLOSURE_SERIES_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&record, "record_id")? != "medicare-hi-closure-series-rollup:v1"
        || string_field(&record, "record_family")? != "medicare_hi_closure_series_rollup"
        || int_field(&record, "pulse")? != 156
        || string_field(&record, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&record, "medicare_hi_bridge_closure_work_queue_path")?
            != MEDICARE_HI_BRIDGE_CLOSURE_WORK_QUEUE_JSON_PATH
    {
        return Err("Medicare HI closure series rollup identity failed".to_string());
    }

    let paths = record
        .get("closure_packet_paths")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI closure packet paths")?;
    let expected_paths = [
        MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH,
        MEDICARE_HI_INCOME_CATEGORY_OMB_MAPPING_GAP_JSON_PATH,
        MEDICARE_HI_LEGAL_BASE_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_ECONOMIC_BASE_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_TRUST_FUND_SOLVER_YIELD_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_POLICY_BEHAVIOR_REFORM_YIELD_CLOSURE_GAP_JSON_PATH,
        MEDICARE_HI_RATE_SOLVER_READINESS_REVIEW_CLOSURE_GAP_JSON_PATH,
    ];
    if paths.len() != expected_paths.len() {
        return Err("Medicare HI closure packet path count failed".to_string());
    }
    for expected in expected_paths {
        if !paths.iter().any(|path| path.as_str() == Some(expected)) {
            return Err(format!(
                "Medicare HI closure series missing packet path: {expected}"
            ));
        }
        if !root.join(expected).exists() {
            return Err(format!(
                "Medicare HI closure series packet path does not exist: {expected}"
            ));
        }
    }

    let status = record
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI closure series status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "closure_series_rollup_published",
        "all_closure_packets_published",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI closure series status {field} must be true"
            ));
        }
    }
    for field in [
        "new_external_download_performed",
        "any_bridge_item_completed",
        "all_six_bridge_items_ready",
        "closure_review_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI closure series status {field} must be false"
            ));
        }
    }

    let rows = record
        .get("closure_packet_status")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI closure packet status")?;
    if rows.len() != 7
        || rows.iter().any(|row| {
            row.get("packet_published")
                .and_then(serde_json::Value::as_bool)
                != Some(true)
        })
        || rows
            .iter()
            .any(|row| row.get("completed").and_then(serde_json::Value::as_bool) != Some(false))
        || rows
            .iter()
            .any(|row| row.get("ready").and_then(serde_json::Value::as_bool) != Some(false))
        || rows
            .iter()
            .any(|row| row.get("value") != Some(&serde_json::Value::Null))
    {
        return Err(
            "Medicare HI closure packet rows must remain published but blocked".to_string(),
        );
    }
    let observed_rows = rows
        .iter()
        .map(|row| Ok((int_field(row, "rank")?, string_field(row, "work_id")?)))
        .collect::<Result<BTreeSet<_>, String>>()?;
    let expected_rows = [
        (1, "omb_cms_receipt_row_perimeter_bridge"),
        (2, "hi_income_category_split_to_omb_rows"),
        (3, "legal_receipt_base_definition"),
        (4, "economic_base_incidence_distribution"),
        (5, "trust_fund_solver_yield_mapping"),
        (6, "policy_behavior_reform_yield_model"),
        (7, "medicare_hi_rate_solver_readiness_review"),
    ]
    .into_iter()
    .map(|(rank, work_id)| (rank, work_id.to_string()))
    .collect::<BTreeSet<_>>();
    if observed_rows != expected_rows {
        return Err("Medicare HI closure packet status set failed".to_string());
    }

    let summary = record
        .get("series_summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI closure series summary")?;
    for (field, expected) in [
        ("closure_packets_expected", 7),
        ("closure_packets_published", 7),
        ("work_items_completed", 0),
        ("work_items_ready", 0),
        ("required_bridge_components", 6),
        ("ready_bridge_components", 0),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("Medicare HI closure series summary {field} failed"));
        }
    }
    for field in [
        "closure_review_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI closure series summary {field} must be false"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI closure series blocked outputs")?;
    for field in [
        "perimeter_bridge_value",
        "legal_receipt_base_amount",
        "economic_receipt_base_amount",
        "matched_receipt_base",
        "current_law_yield_matched_to_solver",
        "reform_yield",
        "solver_input_row",
        "assigned_base_rate",
        "statutory_rate",
        "effective_rate",
        "public_rate_card",
        "tax_proposal_fields",
        "balanced_budget_fields",
        "target_cost",
        "federal_effect",
        "gross_savings",
        "net_savings",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "Medicare HI closure series blocked output {field} must be null"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI closure series claims")?;
    for field in [
        "medicare_hi_closure_series_rollup_published",
        "all_closure_packets_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI closure series claim {field} must be true"
            ));
        }
    }
    for field in [
        "work_item_completed",
        "any_bridge_item_completed",
        "all_six_bridge_items_ready",
        "closure_review_ready",
        "assigned_receipt_base_published",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
        "public_claims_ready",
        "statutory_rate_claim",
        "effective_rate_claim",
        "public_rate_card_claim",
        "tax_proposal_claim",
        "balanced_budget_claim",
        "target_cost_claim",
        "federal_effect_claim",
        "gross_savings_claim",
        "net_savings_claim",
        "waste_finding_claim",
        "fraud_finding_claim",
        "technology_savings_claim",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI closure series claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(MEDICARE_HI_CLOSURE_SERIES_ROLLUP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        MEDICARE_HI_CLOSURE_SERIES_ROLLUP_JSON_PATH,
        "The Medicare HI closure series rollup is published, but it does not complete any Medicare HI bridge item.",
        "Seven Medicare HI closure packets are published; zero bridge items are ready.",
        "The closure packets are blocker packets, not solver inputs and not rate-publication packets.",
        "Medicare HI remains a separate trust fund; combined Medicare financing remains prohibited.",
        "No Medicare HI assigned base, rate, reform yield, solver row, public rate card, tax proposal, savings estimate, or balanced-budget value is populated.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "not solver input",
        "not a solver run",
        "not a target-cost selection",
        "not a rate calculation",
        "not a public rate card",
        "not a tax proposal",
        "not a savings estimate",
        "not a waste finding",
        "not a fraud finding",
        "not a department-cut instruction",
        "not a technology-savings claim",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI closure series reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_cy2025_2035_current_law_context_path(root: &Path) -> Result<(), String> {
    for path in [
        MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI current-law context path artifact: {path}"
            ));
        }
    }

    let text =
        fs::read_to_string(root.join(MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH))
            .map_err(|err| {
            format!(
                "failed to read {MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH}: {err}"
            )
        })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")? != "medicare-hi-cy2025-2035-current-law-context-path:v1"
        || string_field(&record, "record_family")? != "medicare_hi_current_law_context_path"
        || string_field(&record, "status")?
            != "draft_official_calendar_year_path_fiscal_solver_blocked"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "source_capture_status_path")?
            != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(&record, "source_id")? != "SRC-CMS-MEDICARE-TRUSTEES-2026"
        || string_field(&record, "raw_artifact_path")?
            != "data/raw/cms/SRC-CMS-MEDICARE-TRUSTEES-2026/2026-07-19/2026-medicare-trustees-report.pdf"
        || int_field(&record, "raw_byte_count")? != 2_844_621
        || string_field(&record, "raw_sha256")?
            != "ffa56b9137006872300b0346149eae1613d09a172b6ba118aad48e66dfc48fa8"
        || string_field(&record, "year_basis")? != "calendar_year"
    {
        return Err("Medicare HI current-law context path identity failed".to_string());
    }

    let status = record
        .get("path_status")
        .ok_or("Medicare HI current-law context path status")?;
    for (field, expected) in [
        ("official_cy2025_cy2035_rows_present", true),
        ("local_raw_custody_ready", true),
        ("fiscal_year_path_ready", false),
        ("omb_cms_receipt_row_bridge_ready", false),
        ("solver_ready", false),
        ("rate_ready", false),
        ("savings_ready", false),
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(expected) {
            return Err(format!(
                "Medicare HI current-law context path status {field} failed"
            ));
        }
    }
    if int_field(status, "row_count")? != 11
        || int_field(status, "actual_rows")? != 1
        || int_field(status, "intermediate_projection_rows")? != 10
        || int_field(status, "hypothetical_post_depletion_rows")? != 3
    {
        return Err("Medicare HI current-law context path counts failed".to_string());
    }

    let rows = record
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI current-law context path rows")?;
    if rows.len() != 11 {
        return Err("Medicare HI current-law context path must contain 11 rows".to_string());
    }
    let mut years = BTreeSet::new();
    let mut post_depletion_count = 0usize;
    for row in rows {
        let year = int_field(row, "calendar_year")?;
        years.insert(year);
        if year >= 2033 {
            let note = string_field(row, "row_note")?;
            if !note.contains("depletion") {
                return Err("Medicare HI post-depletion rows need depletion note".to_string());
            }
            post_depletion_count += 1;
        }
        for field in [
            "total_income",
            "total_expenditures",
            "change_in_fund",
            "fund_at_year_end",
        ] {
            if row.get(field).and_then(serde_json::Value::as_f64).is_none() {
                return Err(format!("Medicare HI row {year} missing numeric {field}"));
            }
        }
    }
    let expected_years = (2025..=2035).map(i64::from).collect::<BTreeSet<_>>();
    if years != expected_years || post_depletion_count != 3 {
        return Err("Medicare HI current-law context path year coverage failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI current-law context path blocked outputs")?;
    for field in [
        "fiscal_year_hi_path",
        "omb_cms_receipt_row_bridge",
        "matched_solver_yield",
        "solver_input",
        "rate_calculation",
        "public_rate_card",
    ] {
        if !blocked.get(field).is_some_and(serde_json::Value::is_null) {
            return Err(format!("Medicare HI blocked output {field} must stay null"));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI current-law context path claims")?;
    for field in [
        "medicare_hi_cy2025_2035_current_law_context_path_published",
        "official_cy2025_cy2035_hi_rows_present",
        "local_raw_byte_custody_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Medicare HI claim {field} must be true"));
        }
    }
    for field in [
        "fiscal_year_hi_path_ready",
        "omb_cms_receipt_row_bridge_ready",
        "floor_values_ready",
        "pass_fail_findings_populated",
        "solver_input_ready",
        "rate_calculation_published",
        "public_rate_card_published",
        "gross_savings_published",
        "net_savings_published",
        "technology_savings_claim_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Medicare HI claim {field} must be false"));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_READER_PATH}: {err}"
        )
    })?;
    for required in [
        MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        "CY2025-CY2035",
        "source-custodied CMS Trustees context path",
        "hypothetical-after-depletion boundary",
        "fiscal-year HI path values for OMB solver use",
        "not a fiscal-year solver input",
    ] {
        if !reader.contains(required) {
            return Err(format!(
                "Medicare HI current-law context reader missing: {required}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_cms_omb_fy2025_timing_perimeter_diagnostic(
    root: &Path,
) -> Result<(), String> {
    for path in [
        MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH,
        MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing Medicare HI CMS/OMB timing diagnostic artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(
        root.join(MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "medicare-hi-cms-omb-fy2025-timing-perimeter-diagnostic:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_cms_omb_fy2025_timing_perimeter_diagnostic"
        || string_field(&record, "status")?
            != "draft_cy_fy_timing_perimeter_diagnostic_fiscal_bridge_blocked"
        || string_field(&record, "as_of_date")? != "2026-07-24"
        || string_field(&record, "lane_id")? != "health-medicare"
        || string_field(&record, "cms_hi_cy_context_path")?
            != MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH
        || string_field(&record, "omb_receipt_anchor_path")?
            != CURRENT_LAW_FY2025_DEDICATED_RECEIPT_ANCHORS_JSON_PATH
        || string_field(&record, "omb_outlay_anchor_path")?
            != CURRENT_LAW_FY2025_NAMED_TRUST_FUND_OUTLAY_ANCHORS_JSON_PATH
    {
        return Err("Medicare HI CMS/OMB timing diagnostic identity failed".to_string());
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("Medicare HI CMS/OMB timing source boundary")?;
    for field in [
        "official_public_sources",
        "local_raw_custody_present",
        "diagnostic_only",
        "not_calendar_to_fiscal_conversion",
        "not_omb_cms_receipt_row_bridge",
        "not_fiscal_year_hi_path",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI CMS/OMB timing boundary {field} failed"
            ));
        }
    }
    if string_field(boundary, "cms_year_basis")? != "calendar_year_2025"
        || string_field(boundary, "omb_year_basis")? != "fiscal_year_2025"
        || string_field(boundary, "unit")? != "millions_usd"
    {
        return Err("Medicare HI CMS/OMB timing boundary basis failed".to_string());
    }

    let rows = record
        .get("comparison_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI CMS/OMB timing comparison rows")?;
    let expected = [
        (
            "hi_income_receipt_anchor",
            462_400,
            395_350,
            67_050,
            Some(16.95966),
        ),
        (
            "hi_expenditures_outlay_anchor",
            444_200,
            444_832,
            -632,
            Some(-0.14208),
        ),
        ("hi_net_change_context", 18_200, -49_482, 67_682, None),
    ]
    .into_iter()
    .map(|(id, cms, omb, diff, pct)| (id, (cms, omb, diff, pct)))
    .collect::<BTreeMap<_, _>>();
    if rows.len() != expected.len() {
        return Err("Medicare HI CMS/OMB timing row count failed".to_string());
    }
    for row in rows {
        let comparison_id = string_field(row, "comparison_id")?;
        let (cms, omb, diff, pct) = expected
            .get(comparison_id.as_str())
            .ok_or("unexpected Medicare HI CMS/OMB timing comparison id")?;
        if int_field(row, "cms_amount_musd")? != *cms
            || int_field(row, "omb_amount_musd")? != *omb
            || int_field(row, "cms_minus_omb_musd")? != *diff
            || string_field(row, "diagnostic_boundary")?.is_empty()
        {
            return Err(format!(
                "Medicare HI CMS/OMB timing comparison values failed: {comparison_id}"
            ));
        }
        match pct {
            Some(expected_pct) => {
                if (number_field(row, "cms_minus_omb_pct_of_omb")? - expected_pct).abs() > 0.0001 {
                    return Err(format!(
                        "Medicare HI CMS/OMB timing comparison pct failed: {comparison_id}"
                    ));
                }
            }
            None => {
                if !row
                    .get("cms_minus_omb_pct_of_omb")
                    .is_some_and(serde_json::Value::is_null)
                {
                    return Err("Medicare HI net-change pct must remain null".to_string());
                }
            }
        }
    }

    for array_name in ["diagnostic_findings", "blocked_model_steps"] {
        if record
            .get(array_name)
            .and_then(serde_json::Value::as_array)
            .is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "Medicare HI CMS/OMB timing {array_name} must be nonempty"
            ));
        }
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI CMS/OMB timing blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Medicare HI CMS/OMB timing blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI CMS/OMB timing claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("Medicare HI CMS/OMB timing claim bool")?;
        if matches!(
            field.as_str(),
            "medicare_hi_cms_omb_timing_perimeter_diagnostic_published"
                | "fy2025_anchor_comparison_recorded"
        ) {
            if !observed {
                return Err(format!(
                    "Medicare HI CMS/OMB timing claim should be true: {field}"
                ));
            }
        } else if observed {
            return Err(format!(
                "Medicare HI CMS/OMB timing claim must be false: {field}"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "diagnostic context only",
        "not a calendar-to-fiscal conversion",
        "not a fiscal-year HI path",
        "not an OMB/CMS receipt-row bridge",
        "not a matched solver yield",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "Medicare HI CMS/OMB timing warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH,
        "67050",
        "-632",
        "does not prove a fiscal-year bridge",
        "not a calendar-to-fiscal conversion",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI CMS/OMB timing reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_medicare_hi_treasury_mts_fy2025_trust_fund_anchor_context(
    root: &Path,
) -> Result<(), String> {
    for path in [
        MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH,
        MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing Medicare HI Treasury MTS artifact: {path}"));
        }
    }

    let text = fs::read_to_string(
        root.join(MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH}: {err}"
        )
    })?;
    let record: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        format!(
            "failed to parse {MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH}: {err}"
        )
    })?;

    if string_field(&record, "record_id")?
        != "medicare-hi-treasury-mts-fy2025-trust-fund-anchor-context:v1"
        || string_field(&record, "record_family")?
            != "medicare_hi_treasury_mts_trust_fund_anchor_context"
        || string_field(&record, "status")?
            != "draft_fy2025_mts_anchor_context_fiscal_bridge_blocked"
        || string_field(&record, "lane_id")? != "health-medicare"
        || int_field(&record, "fiscal_year")? != 2025
        || string_field(&record, "record_date")? != "2025-09-30"
    {
        return Err("Medicare HI Treasury MTS identity failed".to_string());
    }

    let custody = record
        .get("source_custody")
        .ok_or("Medicare HI Treasury MTS source custody")?;
    if string_field(custody, "source_id")? != "SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025"
        || string_field(custody, "publisher")?
            != "Bureau of the Fiscal Service, U.S. Department of the Treasury"
        || string_field(custody, "retrieval_date")? != "2026-07-24"
        || string_field(custody, "metadata_path")?
            != "data/metadata/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025.2026-07-24.metadata.md"
        || string_field(custody, "review_status")? != "source_metadata_present_and_hash_matched"
        || !root.join(string_field(custody, "metadata_path")?).exists()
    {
        return Err("Medicare HI Treasury MTS custody identity failed".to_string());
    }
    let files = custody
        .get("raw_context_files")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI Treasury MTS raw files")?;
    let expected_files = [
        (
            "mts_table_4",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv",
                15_442,
                "f82fdcae4b28e3a9a66dfeb20726d1a81d900ca5eabc3559741882e9258fb204",
                57,
            ),
        ),
        (
            "mts_table_5",
            (
                "data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv",
                203_342,
                "fb1646d18d9cc05a217b3b6ac084fd006e0bf01fa26c8ee8815b881579cea66a",
                811,
            ),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    if files.len() != expected_files.len() {
        return Err("Medicare HI Treasury MTS raw file count failed".to_string());
    }
    for file in files {
        let table = string_field(file, "table")?;
        let (path, bytes, sha, rows) = expected_files
            .get(table.as_str())
            .ok_or("unexpected Medicare HI Treasury MTS table")?;
        let raw = root.join(path);
        if string_field(file, "raw_artifact_path")? != *path
            || int_field(file, "raw_byte_count")? != *bytes
            || string_field(file, "raw_sha256")? != *sha
            || int_field(file, "row_count")? != *rows
            || !raw.exists()
            || fs::metadata(&raw).map_err(|err| err.to_string())?.len() != *bytes as u64
            || sha256_file(&raw)? != *sha
        {
            return Err(format!(
                "Medicare HI Treasury MTS raw custody failed: {table}"
            ));
        }
    }

    let boundary = record
        .get("source_boundary")
        .ok_or("Medicare HI Treasury MTS source boundary")?;
    for field in [
        "official_public_source",
        "local_raw_custody_ready",
        "record_date_is_final_fy2025_mts",
        "fiscal_year_anchor_context_ready",
        "not_calendar_to_fiscal_conversion",
        "not_fy2026_fy2035_hi_path",
        "not_income_category_crosswalk",
        "not_solver_input",
    ] {
        if boundary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI Treasury MTS boundary {field} must be true"
            ));
        }
    }

    let rows = record
        .get("fy2025_hi_anchor_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI Treasury MTS anchor rows")?;
    if rows.len() != 2 {
        return Err("Medicare HI Treasury MTS anchor row count failed".to_string());
    }
    let expected = [
        (
            "hi_receipts_mts_table_4",
            27,
            395_350_359_469.67,
            395_350.35946967,
        ),
        (
            "hi_outlays_mts_table_5",
            221,
            444_832_699_854.51,
            444_832.69985451,
        ),
    ]
    .into_iter()
    .map(|(id, line, usd, musd)| (id, (line, usd, musd)))
    .collect::<BTreeMap<_, _>>();
    for row in rows {
        let anchor_id = string_field(row, "anchor_id")?;
        let (line, usd, musd) = expected
            .get(anchor_id.as_str())
            .ok_or("unexpected Medicare HI Treasury MTS anchor")?;
        if int_field(row, "source_line_number")? != *line
            || (number_field(row, "current_fytd_net_amount_usd")? - usd).abs() > 0.01
            || (number_field(row, "current_fytd_net_amount_musd")? - musd).abs() > 0.00001
        {
            return Err(format!(
                "Medicare HI Treasury MTS anchor values failed: {anchor_id}"
            ));
        }
    }

    let observed_negative = record
        .get("negative_hi_outlay_rows_observed_not_used_as_standalone_bridge")
        .and_then(serde_json::Value::as_array)
        .ok_or("Medicare HI Treasury MTS negative rows")?;
    if observed_negative.len() != 2
        || !observed_negative.iter().any(|row| {
            int_field(row, "source_line_number").ok() == Some(772)
                && (number_field(row, "current_fytd_net_amount_usd").unwrap_or_default()
                    + 8_347_247_839.20)
                    .abs()
                    < 0.01
        })
        || !observed_negative.iter().all(|row| {
            string_field(row, "boundary").is_ok_and(|boundary| {
                boundary.contains("not separately netted into a solver bridge")
            })
        })
    {
        return Err("Medicare HI Treasury MTS negative row boundary failed".to_string());
    }

    let comparison = record
        .get("comparison_to_existing_context")
        .ok_or("Medicare HI Treasury MTS comparison")?;
    if string_field(comparison, "cms_omb_timing_perimeter_diagnostic_path")?
        != MEDICARE_HI_CMS_OMB_FY2025_TIMING_PERIMETER_DIAGNOSTIC_JSON_PATH
        || (number_field(comparison, "mts_minus_omb_hi_receipt_anchor_musd")? - 0.35946967).abs()
            > 0.00001
        || (number_field(comparison, "mts_minus_omb_hi_outlay_anchor_musd")? - 0.69985451).abs()
            > 0.00001
        || !string_field(comparison, "diagnostic_note")?.contains("not converted by this packet")
    {
        return Err("Medicare HI Treasury MTS comparison failed".to_string());
    }

    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI Treasury MTS blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "Medicare HI Treasury MTS blocked output must be null: {field}"
            ));
        }
    }

    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("Medicare HI Treasury MTS claims")?;
    for field in [
        "medicare_hi_treasury_mts_fy2025_anchor_context_published",
        "local_raw_custody_ready",
        "fy2025_mts_hi_receipt_outlay_anchors_ready",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Medicare HI Treasury MTS claim {field} must be true"
            ));
        }
    }
    for field in [
        "fiscal_year_hi_path_fy2025_2035_ready",
        "calendar_to_fiscal_conversion_ready",
        "income_category_crosswalk_ready",
        "matched_solver_yield_ready",
        "solver_input_ready",
        "solver_run_published",
        "rate_calculation_published",
        "public_rate_card_published",
        "savings_estimate_published",
        "balanced_budget_claim_published",
    ] {
        if claims.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Medicare HI Treasury MTS claim {field} must be false"
            ));
        }
    }

    let warning = string_field(&record, "public_warning")?;
    for phrase in [
        "local raw custody",
        "FY2025 Federal Hospital Insurance Trust Fund receipt and outlay anchors",
        "not a calendar-to-fiscal conversion",
        "not a FY2025-FY2035 Medicare HI fiscal-year path",
        "not solver input",
        "not a rate calculation",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "Medicare HI Treasury MTS warning missing phrase: {phrase}"
            ));
        }
    }

    let reader = fs::read_to_string(
        root.join(MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_READER_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_READER_PATH}: {err}"
        )
    })?;
    for phrase in [
        MEDICARE_HI_TREASURY_MTS_FY2025_TRUST_FUND_ANCHOR_CONTEXT_JSON_PATH,
        "MTS Table 4 line 27",
        "MTS Table 5 line 221",
        "395350.35946967",
        "444832.69985451",
        "not a calendar-to-fiscal conversion",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "Medicare HI Treasury MTS reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

