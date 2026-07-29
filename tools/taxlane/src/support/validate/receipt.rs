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

pub(crate) fn validate_receipt_share_rows(rows: &[ReceiptShareRow]) -> Result<(), String> {
    if rows.len() != 588 {
        return Err(format!(
            "expected 588 Table 2.2 receipt share rows, found {}",
            rows.len()
        ));
    }

    let mut by_year: BTreeMap<i64, Vec<&ReceiptShareRow>> = BTreeMap::new();
    for row in rows {
        if !(0.0..=100.0).contains(&row.percent) {
            return Err(format!(
                "{} {} percent out of range: {}",
                row.fiscal_year, row.receipt_category, row.percent
            ));
        }
        by_year.entry(row.fiscal_year).or_default().push(row);
    }

    for (year, year_rows) in by_year {
        if year_rows.len() != RECEIPT_SHARE_CATEGORIES.len() {
            return Err(format!(
                "{year}: expected {} share rows, found {}",
                RECEIPT_SHARE_CATEGORIES.len(),
                year_rows.len()
            ));
        }
        let category_sum: f64 = year_rows
            .iter()
            .filter(|row| row.receipt_category != "total-receipts")
            .map(|row| row.percent)
            .sum();
        if (category_sum - 100.0).abs() > 0.25 {
            return Err(format!(
                "{year}: receipt-source shares sum to {category_sum}"
            ));
        }
        let total = year_rows
            .iter()
            .find(|row| row.receipt_category == "total-receipts")
            .map(|row| row.percent)
            .ok_or_else(|| format!("{year}: missing total receipts share"))?;
        if (total - 100.0).abs() > 0.000001 {
            return Err(format!("{year}: total receipts share is {total}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_receipt_base_local_source_inventory(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH,
        RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_SCHEMA_PATH,
        RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt base local inventory artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let inventory: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&inventory, "record_id")? != "receipt-base-local-source-inventory:v1"
        || string_field(&inventory, "record_family")? != "receipt_base_local_source_inventory"
        || int_field(&inventory, "pulse")? != 132
        || string_field(&inventory, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&inventory, "assigned_receipt_base_source_gap_path")?
            != ASSIGNED_RECEIPT_BASE_SOURCE_GAP_JSON_PATH
        || string_field(&inventory, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("receipt base local inventory identity failed".to_string());
    }

    for path in [
        string_field(&inventory, "contract_path")?,
        string_field(&inventory, "assigned_receipt_base_source_gap_path")?,
        string_field(&inventory, "rate_publication_readiness_rollup_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "receipt base local inventory referenced path missing: {path}"
            ));
        }
    }

    let status = inventory
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base local inventory custody status")?;
    for field in [
        "official_sources_only",
        "local_repo_inventory_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "irs_pub1304_ty2023_raw_custody_ready",
        "ssa_trustees_official_source_browser_review_ready",
        "ssa_calendar_year_taxable_payroll_context_ready",
        "cms_medicare_trustees_raw_custody_ready",
        "medicare_hi_calendar_year_financing_context_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base local inventory status {field} must be true"
            ));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base local inventory status {field} must be false"
            ));
        }
    }

    let local_rows = inventory
        .get("local_source_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base local source rows")?;
    let expected_sources = [
        "SRC-IRS-SOI-HT23",
        "SRC-IRS-SOI-PUB1304-TABLE-1-1-TY2023",
        "SRC-IRS-SOI-1304",
        "SRC-SSA-TRUSTEES-2026",
        "SRC-CMS-MEDICARE-TRUSTEES-2026",
        "SRC-OMB-HIST-2-4-FY2027",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_sources = local_rows
        .iter()
        .map(|row| string_field(row, "source_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_sources != expected_sources {
        return Err("receipt base local source set failed".to_string());
    }
    for row in local_rows {
        if row.get("baseline_amount_musd") != Some(&serde_json::Value::Null)
            || row.get("matched_year") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("receipt base local source rows must keep values null/false".to_string());
        }
        if row.get("local_artifact_path") != Some(&serde_json::Value::Null) {
            let path = string_field(row, "local_artifact_path")?;
            if !root.join(&path).exists() {
                return Err(format!("receipt base local artifact missing: {path}"));
            }
        }
        if row.get("metadata_path") != Some(&serde_json::Value::Null) {
            let path = string_field(row, "metadata_path")?;
            if !root.join(&path).exists() {
                return Err(format!("receipt base metadata artifact missing: {path}"));
            }
        }
    }

    let blocked_rows = inventory
        .get("blocked_base_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base blocked rows")?;
    let expected_bases = [
        "individual_income_agi",
        "individual_income_taxable_income",
        "oasdi_taxable_payroll",
        "medicare_hi_taxable_payroll",
        "transportation_excises_and_user_fees",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_bases = blocked_rows
        .iter()
        .map(|row| string_field(row, "base_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_bases != expected_bases {
        return Err("receipt base blocked row set failed".to_string());
    }
    for row in blocked_rows {
        if row.get("baseline_amount_musd") != Some(&serde_json::Value::Null)
            || row.get("matched_year") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("receipt base blocked rows must keep values null/false".to_string());
        }
    }

    let blocked = inventory
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base local inventory blocked outputs")?;
    for field in [
        "assigned_base_amounts",
        "matched_receipt_bases",
        "behavioral_elasticities",
        "current_law_yields",
        "reform_yields",
        "assigned_base_rates",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "receipt base local inventory blocked output {field} must be null"
            ));
        }
    }

    let claims = inventory
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base local inventory claims")?;
    if claims
        .get("local_source_inventory_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("receipt base local inventory published flag must be true".to_string());
    }
    for field in [
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
                "receipt base local inventory claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH,
        "This is a local source inventory, not a receipt-base extraction.",
        "Local HT23 custody supports rate and bracket context only, not AGI, taxable-income, payroll, or fee base amounts.",
        "IRS Pub 1304 TY2023 raw custody supports individual-income context only, not a matched FY2025 assigned base.",
        "SSA Trustees calendar-year taxable-payroll context cannot substitute for a fiscal-year OMB-reconciled assigned receipt base.",
        "CMS Medicare Trustees raw custody supports calendar-year HI context only, not a fiscal-year OMB-reconciled assigned receipt base.",
        "No assigned base amount, elasticity, yield, rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
        "No external request was submitted and no agency or person was contacted.",
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
                "receipt base local inventory reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_receipt_base_source_work_queue(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH,
        RECEIPT_BASE_SOURCE_WORK_QUEUE_SCHEMA_PATH,
        RECEIPT_BASE_SOURCE_WORK_QUEUE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt base source work queue artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let queue: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&queue, "record_id")? != "receipt-base-source-work-queue:v1"
        || string_field(&queue, "record_family")? != "receipt_base_source_work_queue"
        || int_field(&queue, "pulse")? != 133
        || string_field(&queue, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&queue, "receipt_base_local_source_inventory_path")?
            != RECEIPT_BASE_LOCAL_SOURCE_INVENTORY_JSON_PATH
        || string_field(&queue, "assigned_receipt_base_source_gap_path")?
            != ASSIGNED_RECEIPT_BASE_SOURCE_GAP_JSON_PATH
        || string_field(&queue, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("receipt base source work queue identity failed".to_string());
    }

    for path in [
        string_field(&queue, "contract_path")?,
        string_field(&queue, "receipt_base_local_source_inventory_path")?,
        string_field(&queue, "assigned_receipt_base_source_gap_path")?,
        string_field(&queue, "rate_publication_readiness_rollup_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "receipt base source work queue referenced path missing: {path}"
            ));
        }
    }

    let status = queue
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base source work queue custody status")?;
    for field in [
        "official_sources_only",
        "work_queue_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "context_progress_recorded",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base source work queue status {field} must be true"
            ));
        }
    }
    for field in [
        "matched_assigned_base_progress_recorded",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base source work queue status {field} must be false"
            ));
        }
    }

    let rows = queue
        .get("work_queue_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base source work queue rows")?;
    let expected = [
        "capture-irs-soi-pub-1304-individual-income-base",
        "capture-ssa-oasdi-taxable-payroll-base",
        "capture-medicare-hi-taxable-payroll-base",
        "extract-omb-receipt-category-reconciliation",
        "capture-transportation-excise-user-fee-base",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed = rows
        .iter()
        .map(|row| string_field(row, "work_item_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed != expected {
        return Err("receipt base source work queue row set failed".to_string());
    }
    for row in rows {
        let row_status = string_field(row, "status")?;
        let allowed_statuses = [
            "not_started_no_external_request",
            "ty2023_context_capture_complete_not_matched_fy2025_assigned_base",
            "calendar_year_context_capture_complete_fiscal_bridge_missing",
            "calendar_year_hi_context_capture_complete_fiscal_bridge_missing",
            "expanded_omb_context_capture_complete_not_assigned_base",
            "receipt_yield_context_capture_complete_legal_and_economic_base_missing",
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if !allowed_statuses.contains(row_status.as_str())
            || row
                .get("external_contact_allowed")
                .and_then(serde_json::Value::as_bool)
                != Some(false)
            || row.get("value") != Some(&serde_json::Value::Null)
            || row.get("ready").and_then(serde_json::Value::as_bool) != Some(false)
        {
            return Err("receipt base source work queue rows must stay blocked".to_string());
        }
    }

    let summary = queue
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base source work queue summary")?;
    if summary
        .get("work_item_count")
        .and_then(serde_json::Value::as_i64)
        != Some(5)
        || summary
            .get("ready_count")
            .and_then(serde_json::Value::as_i64)
            != Some(0)
        || summary
            .get("blocked_count")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
        || summary
            .get("context_progress_count")
            .and_then(serde_json::Value::as_i64)
            != Some(5)
        || summary
            .get("not_started_count")
            .and_then(serde_json::Value::as_i64)
            != Some(0)
        || summary
            .get("external_contact_allowed_count")
            .and_then(serde_json::Value::as_i64)
            != Some(0)
    {
        return Err("receipt base source work queue summary counts failed".to_string());
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base source work queue summary {field} must be false"
            ));
        }
    }

    let blocked = queue
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base source work queue blocked outputs")?;
    for field in [
        "captured_new_sources",
        "assigned_base_amounts",
        "matched_receipt_bases",
        "behavioral_elasticities",
        "current_law_yields",
        "reform_yields",
        "assigned_base_rates",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "receipt base source work queue blocked output {field} must be null"
            ));
        }
    }

    let claims = queue
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base source work queue claims")?;
    if claims
        .get("work_queue_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("receipt base source work queue published flag must be true".to_string());
    }
    for field in [
        "source_capture_completed",
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
                "receipt base source work queue claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(RECEIPT_BASE_SOURCE_WORK_QUEUE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH,
        "This is a receipt-base source work queue, not source capture or receipt-base extraction.",
        "No external request was submitted and no agency or person was contacted.",
        "All five work items have context progress, but every work item remains not ready and external contact is false.",
        "No assigned base amount, elasticity, yield, rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
        "Source work may use only existing local artifacts or official public source capture; it may not contact an agency or person.",
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
                "receipt base source work queue reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_receipt_base_work_item_completion(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH,
        RECEIPT_BASE_WORK_ITEM_COMPLETION_SCHEMA_PATH,
        RECEIPT_BASE_WORK_ITEM_COMPLETION_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt base work item completion artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let completion: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&completion, "record_id")? != "receipt-base-work-item-completion:v1"
        || string_field(&completion, "record_family")? != "receipt_base_work_item_completion"
        || int_field(&completion, "pulse")? != 135
        || string_field(&completion, "contract_path")?
            != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&completion, "receipt_base_source_work_queue_path")?
            != RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&completion, "omb_receipt_category_context_path")?
            != OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH
        || string_field(&completion, "omb_receipt_category_fy2025_2031_context_path")?
            != OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(
            &completion,
            "omb_receipt_share_table_2_2_fy2025_2031_context_path",
        )? != OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(
            &completion,
            "omb_receipt_detail_table_2_4_fy2025_2031_context_path",
        )? != OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(
            &completion,
            "omb_receipt_amount_share_reconciliation_fy2025_2031_context_path",
        )? != OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH
        || string_field(
            &completion,
            "irs_soi_pub1304_ty2023_individual_income_base_context_path",
        )? != IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_JSON_PATH
        || string_field(
            &completion,
            "social_security_taxable_payroll_base_bridge_path",
        )? != SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH
        || string_field(
            &completion,
            "health_medicare_trustees_source_capture_status_path",
        )? != HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH
        || string_field(
            &completion,
            "medicare_hi_cy2025_2035_current_law_context_path",
        )? != MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH
        || string_field(
            &completion,
            "transportation_receipt_base_work_item_progress_path",
        )? != TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH
        || string_field(&completion, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("receipt base work item completion identity failed".to_string());
    }

    for path in [
        string_field(&completion, "contract_path")?,
        string_field(&completion, "receipt_base_source_work_queue_path")?,
        string_field(&completion, "omb_receipt_category_context_path")?,
        string_field(&completion, "omb_receipt_category_fy2025_2031_context_path")?,
        string_field(
            &completion,
            "omb_receipt_share_table_2_2_fy2025_2031_context_path",
        )?,
        string_field(
            &completion,
            "omb_receipt_detail_table_2_4_fy2025_2031_context_path",
        )?,
        string_field(
            &completion,
            "omb_receipt_amount_share_reconciliation_fy2025_2031_context_path",
        )?,
        string_field(
            &completion,
            "irs_soi_pub1304_ty2023_individual_income_base_context_path",
        )?,
        string_field(
            &completion,
            "social_security_taxable_payroll_base_bridge_path",
        )?,
        string_field(
            &completion,
            "health_medicare_trustees_source_capture_status_path",
        )?,
        string_field(
            &completion,
            "medicare_hi_cy2025_2035_current_law_context_path",
        )?,
        string_field(
            &completion,
            "transportation_receipt_base_work_item_progress_path",
        )?,
        string_field(&completion, "rate_publication_readiness_rollup_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "receipt base work item completion referenced path missing: {path}"
            ));
        }
    }

    let completed = completion
        .get("completed_work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base completed work items")?;
    if completed.len() != 5 {
        return Err("receipt base completed work item count failed".to_string());
    }
    let expected_completed_ids = [
        "capture-irs-soi-pub-1304-individual-income-base",
        "capture-ssa-oasdi-taxable-payroll-base",
        "capture-medicare-hi-taxable-payroll-base",
        "extract-omb-receipt-category-reconciliation",
        "capture-transportation-excise-user-fee-base",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut observed_completed_ids = BTreeSet::new();
    let mut observed_artifact_paths = BTreeSet::new();
    let mut observed_source_ids = BTreeSet::new();
    for item in completed {
        observed_completed_ids.insert(string_field(item, "work_item_id")?);
        let artifact_paths = item
            .get("completion_artifact_paths")
            .and_then(serde_json::Value::as_array)
            .ok_or("receipt base completion artifact paths")?;
        if artifact_paths.is_empty() {
            return Err("receipt base completion item needs artifact paths".to_string());
        }
        for path in artifact_paths {
            observed_artifact_paths.insert(
                path.as_str()
                    .map(str::to_string)
                    .ok_or("completion artifact path must be string".to_string())?,
            );
        }
        let source_ids = item
            .get("source_ids")
            .and_then(serde_json::Value::as_array)
            .ok_or("receipt base completion source ids")?;
        if source_ids.is_empty() {
            return Err("receipt base completion item needs source ids".to_string());
        }
        for source_id in source_ids {
            observed_source_ids.insert(
                source_id
                    .as_str()
                    .map(str::to_string)
                    .ok_or("completion source id must be string".to_string())?,
            );
        }
        for field in [
            "ready_for_assigned_base",
            "ready_for_rate_publication",
            "ready_for_solver",
        ] {
            if item.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!("receipt base completed item {field} must be false"));
            }
        }
    }
    if observed_completed_ids != expected_completed_ids {
        return Err("receipt base completed work item id set failed".to_string());
    }
    let expected_artifact_paths = [
        IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_JSON_PATH,
        SOCIAL_SECURITY_TAXABLE_PAYROLL_BASE_BRIDGE_JSON_PATH,
        HEALTH_MEDICARE_TRUSTEES_SOURCE_CAPTURE_STATUS_JSON_PATH,
        MEDICARE_HI_CY2025_2035_CURRENT_LAW_CONTEXT_PATH_JSON_PATH,
        OMB_RECEIPT_CATEGORY_CONTEXT_JSON_PATH,
        OMB_RECEIPT_CATEGORY_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_SHARE_TABLE_2_2_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_DETAIL_TABLE_2_4_FY2025_2031_CONTEXT_JSON_PATH,
        OMB_RECEIPT_AMOUNT_SHARE_RECONCILIATION_FY2025_2031_CONTEXT_JSON_PATH,
        TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH,
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_artifact_paths != expected_artifact_paths {
        return Err("receipt base completion artifact path set failed".to_string());
    }
    for path in observed_artifact_paths {
        if !root.join(&path).exists() {
            return Err(format!("receipt base completion artifact missing: {path}"));
        }
    }
    let expected_source_ids = [
        "SRC-IRS-SOI-PUB1304-TABLE-1-1-TY2023",
        "SRC-SSA-TRUSTEES-2026",
        "SRC-CMS-MEDICARE-TRUSTEES-2026",
        "SRC-OMB-HIST-2-1-FY2027",
        "SRC-OMB-HIST-2-2-FY2027",
        "SRC-OMB-HIST-2-4-FY2027",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_source_ids != expected_source_ids {
        return Err("receipt base completion source id set failed".to_string());
    }

    let remaining = completion
        .get("remaining_work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base remaining work items")?;
    let observed_remaining = remaining
        .iter()
        .map(|row| {
            row.as_str()
                .map(str::to_string)
                .ok_or("remaining work item must be string".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if !observed_remaining.is_empty() {
        return Err("receipt base remaining context work item set must be empty".to_string());
    }
    let assigned_base_open = completion
        .get("assigned_base_open_work_items")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base assigned-base open work items")?;
    let observed_assigned_base_open = assigned_base_open
        .iter()
        .map(|row| {
            row.as_str()
                .map(str::to_string)
                .ok_or("assigned-base open work item must be string".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected_assigned_base_open = [
        "capture-irs-soi-pub-1304-individual-income-base",
        "capture-ssa-oasdi-taxable-payroll-base",
        "capture-medicare-hi-taxable-payroll-base",
        "extract-omb-receipt-category-reconciliation",
        "capture-transportation-excise-user-fee-base",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    if observed_assigned_base_open != expected_assigned_base_open {
        return Err("receipt base assigned-base open work item set failed".to_string());
    }

    let summary = completion
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base completion summary")?;
    for (field, expected) in [
        ("work_item_count", 5),
        ("context_complete_count", 5),
        ("context_artifact_count", 10),
        ("assigned_base_ready_count", 0),
        ("remaining_context_work_item_count", 0),
        ("assigned_base_open_work_item_count", 5),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!("receipt base completion summary {field} failed"));
        }
    }
    for field in [
        "multi_year_omb_receipt_context_ready",
        "all_receipt_base_work_items_have_context_progress",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base completion summary {field} must be true"
            ));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base completion summary {field} must be false"
            ));
        }
    }

    let status = completion
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base completion source custody status")?;
    for field in [
        "official_sources_only",
        "no_external_request_submitted_this_pulse",
        "no_agency_or_person_contacted",
        "completion_record_only",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base completion status {field} must be true"
            ));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base completion status {field} must be false"
            ));
        }
    }

    let blocked = completion
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base completion blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "current_law_yields",
        "reform_yields",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "receipt base completion blocked output {field} must be null"
            ));
        }
    }

    let claims = completion
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base completion claims")?;
    if claims
        .get("work_item_completion_record_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("receipt base completion published flag must be true".to_string());
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
            return Err(format!(
                "receipt base completion claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(RECEIPT_BASE_WORK_ITEM_COMPLETION_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH,
        "All five receipt-base work items have context progress, but no assigned receipt base is ready.",
        "OMB Table 2.1, Table 2.2, Table 2.4, and amount/share reconciliation context are captured for FY2025-FY2031.",
        "IRS Pub 1304 TY2023, SSA Trustees calendar-year OASDI taxable-payroll, CMS Medicare Trustees calendar-year HI, and transportation receipt-yield context are captured with assignment gates closed.",
        "OMB receipt-category context is not a legal or economic assigned receipt base.",
        "All five assigned-base work items remain open.",
        "No rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
        "No external request was submitted and no agency or person was contacted.",
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
                "receipt base completion reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_receipt_base_official_source_capture(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH,
        RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_SCHEMA_PATH,
        RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt base official source capture artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let capture: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&capture, "record_id")? != "receipt-base-official-source-capture:v1"
        || string_field(&capture, "record_family")? != "receipt_base_official_source_capture"
        || int_field(&capture, "pulse")? != 137
        || string_field(&capture, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&capture, "receipt_base_source_work_queue_path")?
            != RECEIPT_BASE_SOURCE_WORK_QUEUE_JSON_PATH
        || string_field(&capture, "receipt_base_work_item_completion_path")?
            != RECEIPT_BASE_WORK_ITEM_COMPLETION_JSON_PATH
        || string_field(
            &capture,
            "transportation_receipt_base_work_item_progress_path",
        )? != TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH
        || string_field(&capture, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("receipt base official source capture identity failed".to_string());
    }

    let status = capture
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base official source capture status")?;
    for field in [
        "official_sources_only",
        "official_public_files_downloaded",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "irs_pub_1304_table_1_1_raw_custody_ready",
        "cms_medicare_trustees_raw_custody_ready",
        "fhwa_highway_statistics_raw_custody_ready",
        "ssa_trustees_calendar_year_context_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base official source capture status {field} must be true"
            ));
        }
    }
    for field in [
        "ssa_trustees_raw_custody_ready",
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base official source capture status {field} must be false"
            ));
        }
    }

    let packets = capture
        .get("captured_source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base official source capture packets")?;
    if packets.len() != 6 {
        return Err("receipt base official source capture packet count failed".to_string());
    }
    for packet in packets {
        let raw_path = string_field(packet, "raw_artifact_path")?;
        let metadata_path = string_field(packet, "metadata_path")?;
        let full_raw = root.join(&raw_path);
        if !full_raw.exists() || !root.join(&metadata_path).exists() {
            return Err(format!(
                "receipt base source packet paths missing: {raw_path}"
            ));
        }
        let byte_count = fs::metadata(&full_raw).map_err(|e| e.to_string())?.len() as i64;
        if packet
            .get("raw_byte_count")
            .and_then(serde_json::Value::as_i64)
            != Some(byte_count)
        {
            return Err(format!(
                "receipt base source packet byte count mismatch: {raw_path}"
            ));
        }
        if sha256_file(&full_raw)? != string_field(packet, "raw_sha256")? {
            return Err(format!(
                "receipt base source packet hash mismatch: {raw_path}"
            ));
        }
        if packet
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        {
            return Err("receipt base source packet custody must be true".to_string());
        }
    }

    let blocked_sources = capture
        .get("blocked_source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base official source blocked packets")?;
    if blocked_sources.len() != 1 {
        return Err("receipt base official source blocked packet count failed".to_string());
    }
    let blocked_ssa = &blocked_sources[0];
    if string_field(blocked_ssa, "work_item_id")? != "capture-ssa-oasdi-taxable-payroll-base"
        || string_field(blocked_ssa, "source_id")? != "SRC-SSA-TRUSTEES-2026"
        || string_field(blocked_ssa, "block_status")?
            != "official_site_returned_http_403_to_direct_raw_download"
        || blocked_ssa.get("raw_artifact_path") != Some(&serde_json::Value::Null)
        || blocked_ssa.get("raw_byte_count") != Some(&serde_json::Value::Null)
        || blocked_ssa.get("raw_sha256") != Some(&serde_json::Value::Null)
        || blocked_ssa
            .get("custody_ready")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("receipt base official source blocked SSA packet failed".to_string());
    }

    let rows = capture
        .get("extracted_context_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base official source context rows")?;
    if rows.len() != 7 {
        return Err("receipt base official source context row count failed".to_string());
    }
    let mut values = BTreeMap::new();
    for row in rows {
        for field in [
            "ready_for_assigned_base",
            "ready_for_rate_publication",
            "ready_for_solver",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "receipt base official source context {field} must be false"
                ));
            }
        }
        values.insert(
            string_field(row, "base_id")?,
            row.get("amount_musd")
                .and_then(serde_json::Value::as_f64)
                .ok_or("amount_musd")?,
        );
    }
    for (base_id, expected) in [
        ("individual_income_agi", 15_286_017.359),
        ("individual_income_taxable_income", 11_625_278.987),
        (
            "individual_income_tax_after_credits_yield_context",
            2_108_587.001,
        ),
        ("oasdi_taxable_payroll", 10_562_000.0),
        ("medicare_hi_taxable_payroll", 13_277_000.0),
        ("medicare_hi_payroll_tax_yield_context", 400_622.16),
        (
            "transportation_highway_user_receipt_yield_context",
            37_512.192,
        ),
    ] {
        let observed = values
            .get(base_id)
            .ok_or_else(|| format!("missing receipt base source value: {base_id}"))?;
        if (observed - expected).abs() > 0.0001 {
            return Err(format!(
                "receipt base source value mismatch for {base_id}: {observed} != {expected}"
            ));
        }
    }

    let transportation = rows
        .iter()
        .find(|row| {
            row.get("base_id").and_then(serde_json::Value::as_str)
                == Some("transportation_highway_user_receipt_yield_context")
        })
        .ok_or("transportation highway user context row")?;
    let components = transportation
        .get("component_values_musd")
        .and_then(serde_json::Value::as_object)
        .ok_or("transportation component values")?;
    let component_sum: f64 = [
        "motor_fuel_total",
        "other_federal_use_tax",
        "trucks_and_trailers",
        "tires",
    ]
    .iter()
    .map(|field| {
        components
            .get(*field)
            .and_then(serde_json::Value::as_f64)
            .ok_or_else(|| format!("transportation component missing: {field}"))
    })
    .collect::<Result<Vec<_>, _>>()?
    .iter()
    .sum();
    if (component_sum - 37_512.192).abs() > 0.0001 {
        return Err("transportation FHWA FE-9 component sum failed".to_string());
    }

    let summary = capture
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base official source capture summary")?;
    if summary
        .get("source_packets_captured")
        .and_then(serde_json::Value::as_i64)
        != Some(6)
        || summary
            .get("blocked_source_packets")
            .and_then(serde_json::Value::as_i64)
            != Some(1)
        || summary
            .get("extracted_context_rows")
            .and_then(serde_json::Value::as_i64)
            != Some(7)
        || summary
            .get("assigned_base_ready_count")
            .and_then(serde_json::Value::as_i64)
            != Some(0)
    {
        return Err("receipt base official source capture summary counts failed".to_string());
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base official source capture summary {field} must be false"
            ));
        }
    }

    let blocked_outputs = capture
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base official source capture blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yields_matched_to_solver",
        "reform_yields",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked_outputs.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "receipt base official source capture blocked output {field} must be null"
            ));
        }
    }

    let claims = capture
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base official source capture claims")?;
    if claims
        .get("official_source_capture_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("official source capture published flag must be true".to_string());
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
            return Err(format!(
                "receipt base official source capture claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH,
        "Pulse 137 captured official public source files and guarded context values; it did not publish assigned receipt bases.",
        "TY2023 IRS SOI values are not FY2025 matched assigned bases.",
        "CMS Medicare HI taxable payroll context is not a public rate calculation.",
        "SSA OASDI calendar-year taxable-payroll context is not a fiscal-year assigned receipt base.",
        "FHWA highway-user receipt and legal-rate context is not a complete economic transportation fee base.",
        "SSA OASDI raw source custody remains blocked in this environment because the official site returned HTTP 403 to direct raw download.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "No rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
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
                "receipt base official source capture reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_receipt_base_reconciliation_gap(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH,
        RECEIPT_BASE_RECONCILIATION_GAP_SCHEMA_PATH,
        RECEIPT_BASE_RECONCILIATION_GAP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt base reconciliation gap artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let gap: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&gap, "record_id")? != "receipt-base-reconciliation-gap:v1"
        || string_field(&gap, "record_family")? != "receipt_base_reconciliation_gap"
        || int_field(&gap, "pulse")? != 138
        || string_field(&gap, "contract_path")? != PROGRAM_LANE_TARGET_COST_CONTRACT_JSON_PATH
        || string_field(&gap, "receipt_base_official_source_capture_path")?
            != RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH
        || string_field(&gap, "rate_publication_readiness_rollup_path")?
            != RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH
    {
        return Err("receipt base reconciliation gap identity failed".to_string());
    }

    for path in [
        string_field(&gap, "contract_path")?,
        string_field(&gap, "receipt_base_official_source_capture_path")?,
        string_field(&gap, "rate_publication_readiness_rollup_path")?,
    ] {
        if !root.join(&path).exists() {
            return Err(format!(
                "receipt base reconciliation gap referenced path missing: {path}"
            ));
        }
    }

    let status = gap
        .get("source_custody_status")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base reconciliation status")?;
    for field in [
        "official_sources_only",
        "used_existing_captured_sources_only",
        "no_foia_or_records_request_submitted",
        "no_agency_or_person_contacted",
        "source_capture_available_for_three_work_items",
        "official_context_available_for_all_four_reconciliation_rows",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "receipt base reconciliation status {field} must be true"
            ));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_inputs_ready",
    ] {
        if status.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base reconciliation status {field} must be false"
            ));
        }
    }

    let rows = gap
        .get("reconciliation_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt base reconciliation rows")?;
    if rows.len() != 4 {
        return Err("receipt base reconciliation row count failed".to_string());
    }
    let mut statuses = BTreeMap::new();
    for row in rows {
        let work_item_id = string_field(row, "work_item_id")?;
        statuses.insert(work_item_id.clone(), string_field(row, "readiness_status")?);
        for field in [
            "ready_for_assigned_base",
            "ready_for_rate_publication",
            "ready_for_solver",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "receipt base reconciliation row {work_item_id} {field} must be false"
                ));
            }
        }
    }
    for (work_item, expected_status) in [
        (
            "capture-irs-soi-pub-1304-individual-income-base",
            "source_captured_context_only_year_mismatch",
        ),
        (
            "capture-ssa-oasdi-taxable-payroll-base",
            "official_calendar_year_context_raw_custody_blocked_fiscal_bridge_missing",
        ),
        (
            "capture-medicare-hi-taxable-payroll-base",
            "source_captured_year_matched_but_model_gates_blocked",
        ),
        (
            "capture-transportation-excise-user-fee-base",
            "source_captured_receipt_yield_and_legal_rate_context_only",
        ),
    ] {
        if statuses.get(work_item).map(String::as_str) != Some(expected_status) {
            return Err(format!(
                "receipt base reconciliation status mismatch for {work_item}"
            ));
        }
    }

    let summary = gap
        .get("summary")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base reconciliation summary")?;
    for (field, expected) in [
        ("reconciliation_row_count", 4),
        ("source_captured_context_row_count", 4),
        ("raw_custody_blocked_row_count", 1),
        ("year_matched_context_row_count", 1),
        ("calendar_year_context_row_count", 1),
        ("assigned_base_ready_count", 0),
    ] {
        if summary.get(field).and_then(serde_json::Value::as_i64) != Some(expected) {
            return Err(format!(
                "receipt base reconciliation summary {field} failed"
            ));
        }
    }
    for field in [
        "matched_receipt_bases_ready",
        "rate_publication_ready",
        "solver_ready",
    ] {
        if summary.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "receipt base reconciliation summary {field} must be false"
            ));
        }
    }

    let blocked = gap
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base reconciliation blocked outputs")?;
    for field in [
        "legal_receipt_base_amounts",
        "economic_receipt_base_amounts",
        "matched_receipt_bases",
        "assigned_base_rates",
        "behavioral_elasticities",
        "avoidance_and_compliance",
        "administration_burden",
        "distribution_by_income",
        "current_law_yields_matched_to_solver",
        "reform_yields",
        "public_rate_cards",
        "solver_input_rows",
        "tax_proposal_fields",
        "balanced_budget_fields",
    ] {
        if blocked.get(field) != Some(&serde_json::Value::Null) {
            return Err(format!(
                "receipt base reconciliation blocked output {field} must be null"
            ));
        }
    }

    let claims = gap
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt base reconciliation claims")?;
    if claims
        .get("reconciliation_gap_published")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return Err("receipt base reconciliation published flag must be true".to_string());
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
            return Err(format!(
                "receipt base reconciliation claim {field} must be false"
            ));
        }
    }

    let reader = fs::read_to_string(root.join(RECEIPT_BASE_RECONCILIATION_GAP_READER_PATH))
        .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH,
        "Captured context values are not matched assigned receipt bases.",
        "Medicare HI has FY2025 source context, but rate and solver gates remain blocked.",
        "SSA OASDI has CY2025 taxable-payroll context, but raw-byte custody and fiscal-year reconciliation remain blocked.",
        "IRS TY2023 values require a tax-year-to-fiscal-year bridge before FY2025 use.",
        "FHWA FY2024 receipt-yield context is not a FY2025 legal or economic transportation base.",
        "SSA OASDI raw source custody remains blocked even though calendar-year context is available.",
        "No FOIA request, records request, form, email, phone call, or agency/person contact was submitted.",
        "No rate, public rate card, solver input, tax proposal, or balanced-budget value is populated.",
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
                "receipt base reconciliation reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_receipt_base_rate_bridge_readiness_rollup(root: &Path) -> Result<(), String> {
    for path in [
        RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_JSON_PATH,
        RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_SCHEMA_PATH,
        RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!(
                "missing receipt-base/rate-bridge readiness artifact: {path}"
            ));
        }
    }

    let text = fs::read_to_string(root.join(RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_JSON_PATH))
        .map_err(|e| e.to_string())?;
    let rollup: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    if string_field(&rollup, "record_id")? != "receipt-base-rate-bridge-readiness-rollup:v1"
        || string_field(&rollup, "record_family")? != "receipt_base_rate_bridge_readiness_rollup"
        || string_field(&rollup, "status")?
            != "draft_context_present_assigned_bases_and_rates_blocked"
        || string_field(&rollup, "as_of_date")? != "2026-07-25"
    {
        return Err("receipt-base/rate-bridge readiness identity failed".to_string());
    }

    let paths = rollup
        .get("source_context_paths")
        .ok_or("receipt-base/rate-bridge source context paths")?;
    let expected_paths = [
        (
            "assigned_receipt_base_source_gap_path",
            ASSIGNED_RECEIPT_BASE_SOURCE_GAP_JSON_PATH,
        ),
        (
            "receipt_base_official_source_capture_path",
            RECEIPT_BASE_OFFICIAL_SOURCE_CAPTURE_JSON_PATH,
        ),
        (
            "receipt_base_reconciliation_gap_path",
            RECEIPT_BASE_RECONCILIATION_GAP_JSON_PATH,
        ),
        (
            "omb_cbo_revenue_overlap_reconciliation_path",
            OMB_CBO_REVENUE_OVERLAP_RECONCILIATION_FY2026_2031_CONTEXT_JSON_PATH,
        ),
        (
            "irs_individual_income_context_path",
            IRS_SOI_PUB1304_TY2023_INDIVIDUAL_INCOME_BASE_CONTEXT_JSON_PATH,
        ),
        (
            "irs_corporate_income_context_path",
            IRS_SOI_CORPORATION_COMPLETE_TABLE_2_3_TY2022_CORPORATE_INCOME_BASE_CONTEXT_JSON_PATH,
        ),
        (
            "social_security_oasdi_receipt_yield_boundary_path",
            SOCIAL_SECURITY_OASDI_RECEIPT_YIELD_BOUNDARY_JSON_PATH,
        ),
        (
            "medicare_hi_omb_cms_receipt_row_perimeter_evidence_path",
            MEDICARE_HI_OMB_CMS_RECEIPT_ROW_PERIMETER_EVIDENCE_JSON_PATH,
        ),
        (
            "transportation_receipt_base_work_item_progress_path",
            TRANSPORTATION_RECEIPT_BASE_WORK_ITEM_PROGRESS_JSON_PATH,
        ),
        (
            "rate_publication_readiness_rollup_path",
            RATE_PUBLICATION_READINESS_ROLLUP_JSON_PATH,
        ),
    ];
    for (field, expected_path) in expected_paths {
        if string_field(paths, field)? != expected_path || !root.join(expected_path).exists() {
            return Err(format!(
                "receipt-base/rate-bridge referenced path failed: {field}"
            ));
        }
    }

    let rows = rollup
        .get("context_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt-base/rate-bridge context rows")?;
    if rows.len() != 6 {
        return Err("receipt-base/rate-bridge rollup must contain six rows".to_string());
    }
    let expected_contexts = [
        "individual_income_ty2023_irs_soi",
        "corporate_income_ty2022_irs_soi",
        "omb_cbo_receipt_overlap_fy2026_2031",
        "social_security_oasdi_fy2025_cy2025_boundary",
        "medicare_hi_fy2025_cms_omb_perimeter_evidence",
        "transportation_fy2024_fhwa_receipt_yield",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let observed_contexts = rows
        .iter()
        .map(|row| string_field(row, "context_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if observed_contexts != expected_contexts {
        return Err("receipt-base/rate-bridge context set failed".to_string());
    }
    for row in rows {
        for field in [
            "ready_for_assigned_base",
            "ready_for_rate_publication",
            "ready_for_solver",
        ] {
            if row.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
                return Err(format!(
                    "receipt-base/rate-bridge context row {field} must be false"
                ));
            }
        }
        match string_field(row, "context_id")?.as_str() {
            "individual_income_ty2023_irs_soi" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("individual income context values")?;
                if (number_field(values, "agi")? - 15_286_017.359).abs() > 0.001
                    || (number_field(values, "taxable_income")? - 11_625_278.987).abs() > 0.001
                    || string_field(row, "matched_year")? != "TY2023"
                {
                    return Err("receipt-base/rate-bridge individual context failed".to_string());
                }
            }
            "corporate_income_ty2022_irs_soi" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("corporate income context values")?;
                if (number_field(values, "business_receipts")? - 29_513_404.734).abs() > 0.001
                    || (number_field(values, "income_subject_to_tax")? - 2_878_327.977).abs()
                        > 0.001
                    || string_field(row, "matched_year")? != "TY2022"
                {
                    return Err("receipt-base/rate-bridge corporate context failed".to_string());
                }
            }
            "omb_cbo_receipt_overlap_fy2026_2031" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("OMB/CBO receipt overlap values")?;
                if int_field(values, "fy2026_cbo_minus_omb_total_receipts")? != 120_211
                    || int_field(values, "fy2031_cbo_minus_omb_total_receipts")? != -689_896
                {
                    return Err("receipt-base/rate-bridge OMB/CBO context failed".to_string());
                }
            }
            "social_security_oasdi_fy2025_cy2025_boundary" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("OASDI context values")?;
                if int_field(values, "omb_oasdi_receipt_anchor")? != 1_283_736
                    || int_field(values, "ssa_cy2025_taxable_payroll")? != 10_562_000
                    || int_field(values, "absolute_difference")? != 25_952
                {
                    return Err("receipt-base/rate-bridge OASDI context failed".to_string());
                }
            }
            "medicare_hi_fy2025_cms_omb_perimeter_evidence" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("Medicare HI context values")?;
                if (number_field(values, "cms_payroll_taxes")? - 400_622.16).abs() > 0.001
                    || int_field(values, "omb_hospital_insurance_anchor")? != 395_350
                    || (number_field(values, "cms_minus_omb")? - 5_272.16).abs() > 0.001
                {
                    return Err("receipt-base/rate-bridge Medicare HI context failed".to_string());
                }
            }
            "transportation_fy2024_fhwa_receipt_yield" => {
                let values = row
                    .get("context_values_musd")
                    .ok_or("transportation receipt context values")?;
                if (number_field(values, "highway_user_receipt_yield")? - 37_512.192).abs() > 0.001
                    || string_field(row, "matched_year")? != "FY2024"
                {
                    return Err(
                        "receipt-base/rate-bridge transportation context failed".to_string()
                    );
                }
            }
            other => {
                return Err(format!(
                    "receipt-base/rate-bridge unexpected context: {other}"
                ));
            }
        }
    }

    let summary = rollup
        .get("readiness_summary")
        .ok_or("receipt-base/rate-bridge readiness summary")?;
    if int_field(summary, "context_row_count")? != 6 {
        return Err("receipt-base/rate-bridge summary row count failed".to_string());
    }
    for field in [
        "assigned_base_ready_count",
        "legal_economic_base_ready_count",
        "incidence_distribution_ready_count",
        "administration_burden_ready_count",
        "current_law_solver_yield_ready_count",
        "reform_yield_ready_count",
        "solver_ready_count",
        "rate_publication_ready_count",
        "public_rate_card_ready_count",
    ] {
        if int_field(summary, field)? != 0 {
            return Err(format!(
                "receipt-base/rate-bridge summary count must be zero: {field}"
            ));
        }
    }

    let requirements = rollup
        .get("required_before_any_rate")
        .and_then(serde_json::Value::as_array)
        .ok_or("receipt-base/rate-bridge requirements")?;
    if requirements.len() != 10 {
        return Err("receipt-base/rate-bridge requirement count failed".to_string());
    }

    let blocked = rollup
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt-base/rate-bridge blocked outputs")?;
    for (field, value) in blocked {
        if !value.is_null() {
            return Err(format!(
                "receipt-base/rate-bridge blocked output must be null: {field}"
            ));
        }
    }

    let claims = rollup
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("receipt-base/rate-bridge claims")?;
    for (field, value) in claims {
        let observed = value
            .as_bool()
            .ok_or("receipt-base/rate-bridge claim bool")?;
        match field.as_str() {
            "receipt_base_rate_bridge_readiness_rollup_published"
            | "official_context_rows_present" => {
                if !observed {
                    return Err(format!(
                        "receipt-base/rate-bridge claim should be true: {field}"
                    ));
                }
            }
            _ if observed => {
                return Err(format!(
                    "receipt-base/rate-bridge downstream claim must be false: {field}"
                ));
            }
            _ => {}
        }
    }

    let warning = string_field(&rollup, "public_warning")?;
    for phrase in [
        "context readiness only",
        "not matched assigned receipt bases",
        "not a legal or economic receipt base",
        "not an incidence or distribution model",
        "not administration burden",
        "not current-law solver yield",
        "not reform yield",
        "not solver input",
        "not a rate calculation",
        "not a public rate card",
        "not a balanced-budget claim",
    ] {
        if !warning.contains(phrase) {
            return Err(format!(
                "receipt-base/rate-bridge warning missing phrase: {phrase}"
            ));
        }
    }

    let reader =
        fs::read_to_string(root.join(RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_READER_PATH))
            .map_err(|e| e.to_string())?;
    for phrase in [
        RECEIPT_BASE_RATE_BRIDGE_READINESS_ROLLUP_JSON_PATH,
        "zero matched assigned receipt bases",
        "zero legal/economic bases",
        "zero incidence or distribution models",
        "zero administration-burden models",
        "zero current-law solver yields",
        "zero reform yields",
        "zero solver-ready rows",
        "zero public-rate-ready rows",
        "not a matched assigned receipt base",
        "not solver input",
        "not a balanced-budget claim",
    ] {
        if !reader.contains(phrase) {
            return Err(format!(
                "receipt-base/rate-bridge reader missing phrase: {phrase}"
            ));
        }
    }

    Ok(())
}

