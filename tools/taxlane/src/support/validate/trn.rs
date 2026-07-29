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

pub(crate) fn validate_trn_a_transportation_baseline_source_spine(root: &Path) -> Result<(), String> {
    for path in [
        TRN_A_BASELINE_SPINE_JSON_PATH,
        TRN_A_BASELINE_SPINE_SCHEMA_PATH,
        TRN_A_BASELINE_SPINE_READER_PATH,
        TRN_A_BASELINE_SPINE_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-A artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_A_BASELINE_SPINE_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-A spine: {err}"))?;
    let record: serde_json::Value =
        serde_json::from_str(&text).map_err(|err| format!("failed to parse TRN-A spine: {err}"))?;
    if string_field(&record, "record_id")? != "trn-a-transportation-baseline-source-spine:v1"
        || string_field(&record, "record_family")? != "trn_a_transportation_baseline_source_spine"
        || string_field(&record, "status")?
            != "trn_a_complete_bounded_current_law_baseline_source_spine"
        || string_field(&record, "track_wave_id")? != "TRN-A"
        || int_field(&record, "pulse")? != 238
        || string_field(&record, "schema_path")? != TRN_A_BASELINE_SPINE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != TRN_A_BASELINE_SPINE_READER_PATH
        || string_field(&record, "role_review_path")? != TRN_A_BASELINE_SPINE_ROLE_REVIEW_PATH
        || string_field(&record, "core_g_path")? != CORE_G_SOLVER_SPINE_JSON_PATH
    {
        return Err("TRN-A identity failed".to_string());
    }
    let spine = record
        .get("source_spine")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A source spine")?;
    if spine.len() != 6 {
        return Err("TRN-A must contain six source-spine components".to_string());
    }
    for component in spine {
        let path = string_field(component, "artifact_path")?;
        if !root.join(&path).exists()
            || string_field(component, "source_id")?.is_empty()
            || string_field(component, "coverage")?.is_empty()
            || string_field(component, "admission_status")?.is_empty()
            || bool_field(component, "solver_ready")?
        {
            return Err(format!("TRN-A source component failed: {path}"));
        }
    }
    let rows = record
        .get("bounded_outlay_baseline_musd")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A bounded baseline")?;
    if rows.len() != 7 {
        return Err("TRN-A bounded baseline must contain FY2025-FY2031".to_string());
    }
    for (index, row) in rows.iter().enumerate() {
        let year = int_field(row, "fiscal_year")?;
        let total = int_field(row, "total")?;
        let components = int_field(row, "ground")?
            + int_field(row, "air")?
            + int_field(row, "water")?
            + int_field(row, "other")?;
        if year != 2025 + index as i64
            || total != components
            || string_field(row, "status")? != if index == 0 { "actual" } else { "projection" }
        {
            return Err(format!("TRN-A bounded baseline failed: FY{year}"));
        }
    }
    let gaps = record
        .get("explicit_horizon_gaps")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A horizon gaps")?;
    if gaps.len() != 4 {
        return Err("TRN-A must expose four horizon gaps".to_string());
    }
    for (index, gap) in gaps.iter().enumerate() {
        if int_field(gap, "fiscal_year")? != 2032 + index as i64
            || !gap
                .get("function_400_outlays_musd")
                .is_some_and(serde_json::Value::is_null)
            || !gap
                .get("named_fund_income_outgo")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("TRN-A horizon gap must remain explicit and null".to_string());
        }
    }
    let findings = record
        .get("core_h_interface_findings")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A CORE-H findings")?;
    let gates = record
        .get("completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-A completion gates")?;
    if findings.len() != 10
        || gates.len() != 10
        || gates
            .iter()
            .any(|gate| bool_field(gate, "ready").ok() != Some(true))
    {
        return Err("TRN-A findings or completion gates failed".to_string());
    }
    let aggregate = record.get("aggregate_status").ok_or("TRN-A aggregate")?;
    if int_field(aggregate, "source_component_count")? != 6
        || int_field(aggregate, "bounded_outlay_rows")? != 7
        || int_field(aggregate, "explicit_null_horizon_rows")? != 4
        || !bool_field(aggregate, "trn_a_done")?
        || !bool_field(aggregate, "core_h_interface_findings_ready")?
        || bool_field(aggregate, "full_fy2025_fy2035_lane_solver_path_ready")?
        || bool_field(aggregate, "trn_b_may_start")?
    {
        return Err("TRN-A aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-A blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-A blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-A claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "trn_a_source_spine_published" | "trn_a_done" | "core_h_interface_findings_ready"
        );
        if value.as_bool().ok_or("TRN-A claim bool")? != expected {
            return Err(format!("TRN-A claim boundary failed: {field}"));
        }
    }
    for (path, phrase) in [
        (
            TRN_A_BASELINE_SPINE_SCHEMA_PATH,
            "bounded source spine is complete",
        ),
        (TRN_A_BASELINE_SPINE_READER_PATH, "TRN-A is complete"),
        (
            TRN_A_BASELINE_SPINE_ROLE_REVIEW_PATH,
            "Approved as a complete bounded transportation baseline",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN-A prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_trn_b_transportation_accounting_start_gate(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_START_GATE_JSON_PATH,
        TRN_B_START_GATE_SCHEMA_PATH,
        TRN_B_START_GATE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B start artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_START_GATE_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B start gate: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B start gate: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-transportation-accounting-start-gate:v1"
        || string_field(&record, "record_family")? != "trn_b_transportation_accounting_start_gate"
        || string_field(&record, "status")? != "trn_b_completed_after_verified_dependency_start"
        || string_field(&record, "track_wave_id")? != "TRN-B"
        || int_field(&record, "pulse")? != 242
        || string_field(&record, "schema_path")? != TRN_B_START_GATE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != TRN_B_START_GATE_READER_PATH
        || string_field(&record, "trn_a_path")? != TRN_A_BASELINE_SPINE_JSON_PATH
        || string_field(&record, "core_h_path")? != CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH
        || string_field(&record, "active_work_artifact_path")? != TRN_B_NAMED_FUND_ADAPTER_JSON_PATH
        || string_field(&record, "source_bridge_decisions_path")? != TRN_B_SOURCE_BRIDGE_JSON_PATH
        || string_field(&record, "function_400_mapping_path")?
            != TRN_B_FUNCTION_400_MAPPING_JSON_PATH
        || string_field(&record, "accounting_schedules_path")?
            != TRN_B_ACCOUNTING_SCHEDULES_JSON_PATH
        || string_field(&record, "closure_path")? != TRN_B_CLOSURE_JSON_PATH
    {
        return Err("TRN-B start-gate identity failed".to_string());
    }
    let dependency = record.get("dependency_gate").ok_or("TRN-B dependencies")?;
    for field in [
        "trn_a_done",
        "core_h_done",
        "all_dependencies_complete",
        "trn_b_may_start",
        "trn_b_started",
    ] {
        if !bool_field(dependency, field)? {
            return Err(format!("TRN-B dependency failed: {field}"));
        }
    }
    let packages = record
        .get("work_packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B work packages")?;
    if packages.len() != 6 {
        return Err("TRN-B must define six work packages".to_string());
    }
    for (index, package) in packages.iter().enumerate() {
        let expected_status = "complete";
        if string_field(package, "work_package_id")? != format!("TRN-B-{:02}", index + 1)
            || string_field(package, "title")?.is_empty()
            || string_field(package, "objective")?.is_empty()
            || string_field(package, "status")? != expected_status
        {
            return Err("TRN-B work-package ordering failed".to_string());
        }
    }
    let rules = record
        .get("admission_rules")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B admission rules")?;
    if rules.len() != 6
        || rules
            .iter()
            .any(|value| value.as_str().is_none_or(str::is_empty))
    {
        return Err("TRN-B admission rules failed".to_string());
    }
    let aggregate = record.get("aggregate_status").ok_or("TRN-B aggregate")?;
    if int_field(aggregate, "dependency_count")? != 2
        || int_field(aggregate, "ready_dependencies")? != 2
        || int_field(aggregate, "work_package_count")? != 6
        || int_field(aggregate, "in_progress_work_packages")? != 0
        || int_field(aggregate, "completed_work_packages")? != 6
        || !bool_field(aggregate, "trn_b_started")?
        || !bool_field(aggregate, "trn_b_done")?
        || !bool_field(aggregate, "trn_c_may_start")?
    {
        return Err("TRN-B aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-B blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "trn_b_start_gate_published"
                | "trn_a_done"
                | "core_h_done"
                | "trn_b_started"
                | "trn_b_done"
                | "trn_c_may_start"
        );
        if value.as_bool().ok_or("TRN-B claim bool")? != expected {
            return Err(format!("TRN-B claim boundary failed: {field}"));
        }
    }
    let reader = fs::read_to_string(root.join(TRN_B_START_GATE_READER_PATH))
        .map_err(|err| format!("failed to read TRN-B reader: {err}"))?;
    if !reader.contains("TRN-B started against completed TRN-A and CORE-H")
        || !reader.contains("TRN-B is now complete")
    {
        return Err("TRN-B reader boundary failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_b_named_fund_adapter_rows(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_NAMED_FUND_ADAPTER_JSON_PATH,
        TRN_B_NAMED_FUND_ADAPTER_SCHEMA_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B adapter artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_NAMED_FUND_ADAPTER_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B adapters: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B adapters: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-named-fund-adapter-rows:v1"
        || string_field(&record, "record_family")? != "trn_b_named_fund_adapter_rows"
        || string_field(&record, "status")?
            != "trn_b_01_complete_fy2025_fy2031_named_fund_rows_admitted"
        || string_field(&record, "track_wave_id")? != "TRN-B"
        || string_field(&record, "work_package_id")? != "TRN-B-01"
        || int_field(&record, "pulse")? != 243
        || string_field(&record, "schema_path")? != TRN_B_NAMED_FUND_ADAPTER_SCHEMA_PATH
        || string_field(&record, "core_h_path")? != CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH
        || string_field(&record, "trn_b_start_path")? != TRN_B_START_GATE_JSON_PATH
        || string_field(&record, "unit")? != "tenths_billions_usd"
    {
        return Err("TRN-B adapter identity failed".to_string());
    }
    let rows = record
        .get("adapter_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B adapter rows")?;
    let expected_keys = ["airport_and_airway_trust_fund", "highway_trust_fund"]
        .into_iter()
        .flat_map(|fund| (2025_i64..=2031).map(move |year| format!("{fund}:{year}")))
        .collect::<BTreeSet<_>>();
    let observed_keys = rows
        .iter()
        .map(|row| {
            Ok(format!(
                "{}:{}",
                string_field(row, "fund_id")?,
                int_field(row, "fiscal_year")?
            ))
        })
        .collect::<Result<BTreeSet<_>, String>>()?;
    if rows.len() != 14 || observed_keys != expected_keys {
        return Err("TRN-B must contain fourteen FY2025-FY2031 named-fund rows".to_string());
    }
    for row in rows {
        if !expected_keys.contains(&format!(
            "{}:{}",
            string_field(row, "fund_id")?,
            int_field(row, "fiscal_year")?
        )) || string_field(row, "source_vintage")? != "OMB_FY2027"
            || string_field(row, "source_perimeter")? != "OMB_AP_Table_13_4"
            || !bool_field(row, "ready")?
        {
            return Err("TRN-B adapter lineage failed".to_string());
        }
        let input = row.get("core_h_input").ok_or("TRN-B CORE-H input")?;
        let output = row.get("core_h_output").ok_or("TRN-B CORE-H output")?;
        let primary = int_field(input, "gross_program_outlays")?
            + int_field(input, "implementation_outlays")?
            + int_field(input, "fallback_remediation_outlays")?;
        let net_cash = primary - int_field(input, "credited_offsetting_collections")?;
        let income = int_field(input, "dedicated_receipts")?
            + int_field(input, "explicit_general_fund_transfer")?
            + int_field(input, "other_scored_fund_income")?;
        let change = income - net_cash
            + int_field(input, "balance_adjustments")?
            + int_field(input, "explicit_rounding_line")?;
        let closing = int_field(input, "opening_balance")? + change;
        if int_field(output, "primary_outlays")? != primary
            || int_field(output, "net_cash_requirement")? != net_cash
            || int_field(output, "total_fund_income")? != income
            || int_field(output, "fund_balance_change")? != change
            || int_field(output, "closing_balance")? != closing
            || int_field(row, "source_closing_balance")? != closing
            || int_field(row, "reconciliation_difference")? != 0
            || int_field(row, "reported_change_difference")?
                != change - int_field(row, "source_reported_fund_balance_change")?
        {
            return Err(format!(
                "TRN-B adapter arithmetic failed: {} FY{}",
                string_field(row, "fund_id")?,
                int_field(row, "fiscal_year")?
            ));
        }
    }
    let status = record
        .get("work_package_status")
        .ok_or("TRN-B adapter status")?;
    if int_field(status, "fy2025_fy2031_named_fund_rows_admitted")? != 14
        || int_field(status, "remaining_fy2026_fy2031_rows")? != 0
        || !bool_field(status, "work_package_started")?
        || !bool_field(status, "work_package_done")?
        || bool_field(status, "trn_b_done")?
    {
        return Err("TRN-B adapter work-package boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B adapter blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-B adapter blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B adapter claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "trn_b_01_adapter_started" | "fy2025_fy2031_named_fund_rows_admitted" | "trn_b_01_done"
        );
        if value.as_bool().ok_or("TRN-B adapter claim bool")? != expected {
            return Err(format!("TRN-B adapter claim boundary failed: {field}"));
        }
    }
    let schema = fs::read_to_string(root.join(TRN_B_NAMED_FUND_ADAPTER_SCHEMA_PATH))
        .map_err(|err| format!("failed to read TRN-B adapter schema: {err}"))?;
    if !schema.contains("All fourteen FY2025-FY2031 OMB rows are required") {
        return Err("TRN-B adapter schema boundary failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_b_source_bridge_decisions(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_SOURCE_BRIDGE_JSON_PATH,
        TRN_B_SOURCE_BRIDGE_SCHEMA_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B source bridge artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_SOURCE_BRIDGE_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B bridge decisions: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B bridge decisions: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-source-bridge-decisions:v1"
        || string_field(&record, "record_family")? != "trn_b_source_bridge_decisions"
        || string_field(&record, "status")?
            != "trn_b_02_and_03_complete_separate_perimeters_no_mechanical_stitch"
        || string_field(&record, "track_wave_id")? != "TRN-B"
        || int_field(&record, "pulse")? != 246
        || string_field(&record, "schema_path")? != TRN_B_SOURCE_BRIDGE_SCHEMA_PATH
    {
        return Err("TRN-B source bridge identity failed".to_string());
    }
    let treasury = record
        .get("fy2025_omb_treasury_review")
        .ok_or("TRN-B OMB/Treasury review")?;
    if string_field(treasury, "decision")?
        != "OMB_Table_13_4_controls_named_fund_ledger_Treasury_MTS_remains_final_anchor_diagnostic"
        || bool_field(treasury, "bridge_admitted")?
        || bool_field(
            treasury,
            "unexplained_difference_allocated_to_transfer_offset_or_savings",
        )?
        || !bool_field(treasury, "work_package_done")?
    {
        return Err("TRN-B OMB/Treasury bridge decision failed".to_string());
    }
    let comparisons = treasury
        .get("comparisons_musd")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B Treasury comparisons")?;
    if comparisons.len() != 4
        || !comparisons[3]
            .get("difference_not_computed")
            .is_some_and(serde_json::Value::is_null)
        || string_field(&comparisons[3], "perimeter_status")?
            != "incomparable_agency_total_not_highway_trust_fund_total"
    {
        return Err("TRN-B Treasury comparison boundary failed".to_string());
    }
    for row in &comparisons[..3] {
        let observed = number_field(row, "treasury_minus_omb")?;
        let expected = number_field(row, "treasury")? - number_field(row, "omb")?;
        if (observed - expected).abs() > 0.000_000_1
            || string_field(row, "perimeter_status")? != "diagnostic_not_bridge"
        {
            return Err("TRN-B Treasury comparison arithmetic failed".to_string());
        }
    }
    let cbo = record
        .get("fy2031_omb_cbo_review")
        .ok_or("TRN-B OMB/CBO review")?;
    if string_field(cbo, "decision")? != "retain_separate_OMB_and_CBO_vintages_no_stitch"
        || bool_field(cbo, "bridge_admitted")?
        || !bool_field(cbo, "mechanical_stitch_prohibited")?
        || !bool_field(cbo, "work_package_done")?
        || !cbo
            .get("fy2032_fy2035_income_outgo_rows")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("TRN-B OMB/CBO bridge decision failed".to_string());
    }
    let cbo_rows = cbo
        .get("comparisons_busd")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B CBO comparisons")?;
    if cbo_rows.len() != 2 {
        return Err("TRN-B CBO comparison count failed".to_string());
    }
    for row in cbo_rows {
        let observed = number_field(row, "cbo_minus_omb")?;
        let expected =
            number_field(row, "cbo_balance_end")? - number_field(row, "omb_balance_end")?;
        if (observed - expected).abs() > 0.000_000_1 {
            return Err("TRN-B CBO comparison arithmetic failed".to_string());
        }
    }
    let aggregate = record
        .get("aggregate_status")
        .ok_or("TRN-B bridge aggregate")?;
    if int_field(aggregate, "work_packages_completed")? != 2
        || int_field(aggregate, "bridge_decisions")? != 2
        || int_field(aggregate, "bridges_admitted")? != 0
        || !bool_field(aggregate, "omb_ledger_horizon_ready")?
        || bool_field(aggregate, "full_fy2025_fy2035_named_fund_ledger_ready")?
        || bool_field(aggregate, "trn_b_done")?
    {
        return Err("TRN-B bridge aggregate failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B bridge blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-B bridge blocked outputs must remain null".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_b_function_400_mapping(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_FUNCTION_400_MAPPING_JSON_PATH,
        TRN_B_FUNCTION_400_MAPPING_SCHEMA_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B Function 400 artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_FUNCTION_400_MAPPING_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B Function 400 mapping: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B Function 400 mapping: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-function-400-mapping:v1"
        || string_field(&record, "record_family")? != "trn_b_function_400_mapping"
        || string_field(&record, "status")? != "trn_b_04_complete_reviewed_perimeter_crosswalk"
        || string_field(&record, "work_package_id")? != "TRN-B-04"
        || int_field(&record, "pulse")? != 247
        || string_field(&record, "schema_path")? != TRN_B_FUNCTION_400_MAPPING_SCHEMA_PATH
        || string_field(&record, "unit")? != "millions_usd"
    {
        return Err("TRN-B Function 400 identity failed".to_string());
    }
    let contract = record
        .get("mapping_contract")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B mapping contract")?;
    if contract.len() != 4
        || contract
            .iter()
            .any(|row| bool_field(row, "direct_allocation_allowed").ok() != Some(false))
    {
        return Err("TRN-B mapping contract failed".to_string());
    }
    let rows = record
        .get("annual_crosswalk_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B mapping rows")?;
    if rows.len() != 7 {
        return Err("TRN-B mapping must contain FY2025-FY2031".to_string());
    }
    for (index, row) in rows.iter().enumerate() {
        let total = int_field(row, "function_400_total")?;
        let subfunctions = int_field(row, "ground")?
            + int_field(row, "air")?
            + int_field(row, "water")?
            + int_field(row, "other")?;
        let funds = int_field(row, "highway_fund_outgo")?
            + int_field(row, "airport_and_airway_fund_outgo")?;
        let residual = int_field(row, "ground_perimeter_residual")?
            + int_field(row, "air_perimeter_residual")?
            + int_field(row, "water_and_other")?;
        if int_field(row, "fiscal_year")? != 2025 + index as i64
            || total != subfunctions
            || int_field(row, "named_fund_outgo_sum")? != funds
            || int_field(row, "non_named_fund_and_perimeter_residual")? != residual
            || total != funds + residual
            || int_field(row, "identity_difference")? != 0
        {
            return Err("TRN-B Function 400 crosswalk arithmetic failed".to_string());
        }
    }
    let decision = record
        .get("review_decision")
        .ok_or("TRN-B mapping decision")?;
    if !bool_field(decision, "mapping_ready")?
        || bool_field(decision, "residual_is_general_fund_amount")?
        || !decision
            .get("general_fund_amount")
            .is_some_and(serde_json::Value::is_null)
        || bool_field(decision, "residual_is_savings")?
        || bool_field(decision, "fund_outgo_equals_subfunction_outlay")?
        || bool_field(decision, "solver_allocation_ready")?
        || !bool_field(decision, "work_package_done")?
    {
        return Err("TRN-B Function 400 review boundary failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_b_accounting_schedules(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_ACCOUNTING_SCHEDULES_JSON_PATH,
        TRN_B_ACCOUNTING_SCHEDULES_SCHEMA_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B schedule artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_ACCOUNTING_SCHEDULES_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B schedules: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B schedules: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-accounting-schedules:v1"
        || string_field(&record, "record_family")? != "trn_b_accounting_schedules"
        || string_field(&record, "status")?
            != "trn_b_05_complete_evidence_supported_schedules_with_nulls"
        || string_field(&record, "work_package_id")? != "TRN-B-05"
        || int_field(&record, "pulse")? != 248
        || string_field(&record, "schema_path")? != TRN_B_ACCOUNTING_SCHEDULES_SCHEMA_PATH
        || string_field(&record, "unit")? != "tenths_billions_usd"
    {
        return Err("TRN-B schedule identity failed".to_string());
    }
    let decisions = record
        .get("field_decisions")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B schedule decisions")?;
    if decisions.len() != 5 {
        return Err("TRN-B must define five schedule field decisions".to_string());
    }
    let rows = record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B schedule rows")?;
    if rows.len() != 14 {
        return Err("TRN-B schedule must contain fourteen rows".to_string());
    }
    let adapter_text = fs::read_to_string(root.join(TRN_B_NAMED_FUND_ADAPTER_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B adapters for schedules: {err}"))?;
    let adapter: serde_json::Value = serde_json::from_str(&adapter_text)
        .map_err(|err| format!("failed to parse TRN-B adapters for schedules: {err}"))?;
    let adapter_rows = adapter
        .get("adapter_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B adapter rows for schedules")?;
    let adapter_by_key = adapter_rows
        .iter()
        .map(|row| {
            Ok((
                format!(
                    "{}:{}",
                    string_field(row, "fund_id")?,
                    int_field(row, "fiscal_year")?
                ),
                row.get("core_h_input")
                    .ok_or("TRN-B schedule adapter input")?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    for row in rows {
        for field in [
            "explicit_general_fund_transfer",
            "credited_offsetting_collections",
            "reserve_contribution",
            "reserve_withdrawal",
            "reserve_balance",
        ] {
            if !row.get(field).is_some_and(serde_json::Value::is_null) {
                return Err(format!(
                    "TRN-B unsupported schedule must remain null: {field}"
                ));
            }
        }
        let key = format!(
            "{}:{}",
            string_field(row, "fund_id")?,
            int_field(row, "fiscal_year")?
        );
        let input = adapter_by_key
            .get(&key)
            .ok_or("TRN-B schedule row lookup")?;
        if int_field(row, "balance_adjustments")? != int_field(input, "balance_adjustments")?
            || int_field(row, "explicit_rounding_line")?
                != int_field(input, "explicit_rounding_line")?
        {
            return Err(format!("TRN-B schedule mismatch: {key}"));
        }
    }
    let aggregate = record
        .get("aggregate_status")
        .ok_or("TRN-B schedule aggregate")?;
    if int_field(aggregate, "annual_row_count")? != 14
        || int_field(aggregate, "unsupported_transfer_rows_kept_null")? != 14
        || int_field(aggregate, "unsupported_offset_rows_kept_null")? != 14
        || int_field(aggregate, "unsupported_reserve_rows_kept_null")? != 14
        || !bool_field(aggregate, "work_package_done")?
        || bool_field(aggregate, "numeric_reserve_parameters_ready")?
        || bool_field(aggregate, "trn_b_done")?
    {
        return Err("TRN-B schedule aggregate failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_b_transportation_accounting_closure(root: &Path) -> Result<(), String> {
    for path in [
        TRN_B_CLOSURE_JSON_PATH,
        TRN_B_CLOSURE_SCHEMA_PATH,
        TRN_B_CLOSURE_READER_PATH,
        TRN_B_CLOSURE_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-B closure artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_B_CLOSURE_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-B closure: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-B closure: {err}"))?;
    if string_field(&record, "record_id")? != "trn-b-transportation-accounting-closure:v1"
        || string_field(&record, "record_family")? != "trn_b_transportation_accounting_closure"
        || string_field(&record, "status")?
            != "trn_b_complete_bounded_transportation_accounting_substrate"
        || string_field(&record, "track_wave_id")? != "TRN-B"
        || int_field(&record, "pulse")? != 249
        || string_field(&record, "schema_path")? != TRN_B_CLOSURE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != TRN_B_CLOSURE_READER_PATH
        || string_field(&record, "role_review_path")? != TRN_B_CLOSURE_ROLE_REVIEW_PATH
        || string_field(&record, "trn_a_path")? != TRN_A_BASELINE_SPINE_JSON_PATH
        || string_field(&record, "core_h_path")? != CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH
    {
        return Err("TRN-B closure identity failed".to_string());
    }
    let packages = record
        .get("work_package_artifacts")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B closure packages")?;
    if packages.len() != 6 {
        return Err("TRN-B closure must include six work packages".to_string());
    }
    for (index, package) in packages.iter().enumerate() {
        let path = string_field(package, "artifact_path")?;
        if string_field(package, "work_package_id")? != format!("TRN-B-{:02}", index + 1)
            || string_field(package, "status")? != "complete"
            || !root.join(&path).exists()
        {
            return Err("TRN-B closure work-package proof failed".to_string());
        }
    }
    let summary = record
        .get("reconciliation_summary")
        .ok_or("TRN-B reconciliation summary")?;
    if int_field(summary, "named_fund_annual_rows")? != 14
        || int_field(summary, "named_fund_closing_balance_rows_reconciled")? != 14
        || int_field(summary, "function_400_crosswalk_rows")? != 7
        || int_field(summary, "function_400_identity_rows_reconciled")? != 7
        || int_field(summary, "source_bridge_decisions")? != 2
        || int_field(summary, "mechanical_cross_source_bridges_admitted")? != 0
        || bool_field(summary, "full_horizon_income_outgo_rows_ready")?
        || !summary
            .get("fy2032_fy2035_income_outgo_rows")
            .is_some_and(serde_json::Value::is_null)
        || bool_field(summary, "numeric_reserve_parameters_ready")?
        || bool_field(summary, "explicit_general_fund_transfer_schedule_ready")?
        || bool_field(summary, "credited_offsetting_collection_schedule_ready")?
    {
        return Err("TRN-B reconciliation summary failed".to_string());
    }
    let gates = record
        .get("completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-B closure gates")?;
    if gates.len() != 10
        || gates
            .iter()
            .any(|gate| bool_field(gate, "ready").ok() != Some(true))
    {
        return Err("TRN-B closure gates failed".to_string());
    }
    let aggregate = record
        .get("aggregate_status")
        .ok_or("TRN-B closure aggregate")?;
    if int_field(aggregate, "completed_work_packages")? != 6
        || int_field(aggregate, "ready_completion_gates")? != 10
        || !bool_field(aggregate, "trn_b_done")?
        || !bool_field(aggregate, "trn_c_may_start")?
        || bool_field(aggregate, "full_lane_solver_path_ready")?
        || bool_field(aggregate, "real_reform_score_ready")?
        || bool_field(aggregate, "solver_ready")?
    {
        return Err("TRN-B closure aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B closure blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-B closure blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-B closure claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "trn_b_closure_published" | "trn_b_done" | "trn_c_may_start"
        );
        if value.as_bool().ok_or("TRN-B closure claim bool")? != expected {
            return Err(format!("TRN-B closure claim boundary failed: {field}"));
        }
    }
    for (path, phrase) in [
        (
            TRN_B_CLOSURE_SCHEMA_PATH,
            "all six work packages are complete",
        ),
        (TRN_B_CLOSURE_READER_PATH, "TRN-B is complete"),
        (
            TRN_B_CLOSURE_ROLE_REVIEW_PATH,
            "Approved as the completed bounded transportation accounting substrate",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN-B closure prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_trn_c_real_reform_start_gate(root: &Path) -> Result<(), String> {
    for path in [
        TRN_C_START_GATE_JSON_PATH,
        TRN_C_START_GATE_SCHEMA_PATH,
        TRN_C_START_GATE_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-C start artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(TRN_C_START_GATE_JSON_PATH))
        .map_err(|err| format!("failed to read TRN-C start gate: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse TRN-C start gate: {err}"))?;
    if string_field(&record, "record_id")? != "trn-c-real-reform-start-gate:v1"
        || string_field(&record, "record_family")? != "trn_c_real_reform_start_gate"
        || string_field(&record, "status")? != "trn_c_completed_after_verified_dependency_start"
        || string_field(&record, "track_wave_id")? != "TRN-C"
        || int_field(&record, "pulse")? != 250
        || string_field(&record, "schema_path")? != TRN_C_START_GATE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != TRN_C_START_GATE_READER_PATH
        || string_field(&record, "trn_b_closure_path")? != TRN_B_CLOSURE_JSON_PATH
        || string_field(&record, "candidate_screen_path")? != TRN_C_CANDIDATE_SCREEN_JSON_PATH
        || string_field(&record, "scenario_path")? != TRN_C_SCENARIO_JSON_PATH
        || string_field(&record, "core_i_path")? != CORE_I_REFORM_ADMISSION_JSON_PATH
        || string_field(&record, "closure_path")? != TRN_C_CLOSURE_JSON_PATH
    {
        return Err("TRN-C start identity failed".to_string());
    }
    let dependency = record.get("dependency_gate").ok_or("TRN-C dependency")?;
    for field in ["trn_b_done", "trn_c_may_start", "trn_c_started"] {
        if !bool_field(dependency, field)? {
            return Err(format!("TRN-C dependency failed: {field}"));
        }
    }
    let packages = record
        .get("work_packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C work packages")?;
    if packages.len() != 5 {
        return Err("TRN-C must define five work packages".to_string());
    }
    for (index, package) in packages.iter().enumerate() {
        if string_field(package, "work_package_id")? != format!("TRN-C-{:02}", index + 1)
            || string_field(package, "status")? != "complete"
            || string_field(package, "objective")?.is_empty()
        {
            return Err("TRN-C work-package ordering failed".to_string());
        }
    }
    let rules = record
        .get("admission_rules")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C admission rules")?;
    if rules.len() != 6 {
        return Err("TRN-C admission rule count failed".to_string());
    }
    let aggregate = record.get("aggregate_status").ok_or("TRN-C aggregate")?;
    if int_field(aggregate, "ready_dependencies")? != 1
        || int_field(aggregate, "work_package_count")? != 5
        || int_field(aggregate, "in_progress_work_packages")? != 0
        || int_field(aggregate, "completed_work_packages")? != 5
        || !bool_field(aggregate, "trn_c_started")?
        || !bool_field(aggregate, "trn_c_done")?
        || !bool_field(aggregate, "trn_d_may_start")?
    {
        return Err("TRN-C aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-C blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-C blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-C claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "trn_c_start_gate_published"
                | "trn_b_done"
                | "trn_c_started"
                | "trn_c_done"
                | "real_reform_score_published"
                | "trn_d_may_start"
        );
        if value.as_bool().ok_or("TRN-C claim bool")? != expected {
            return Err(format!("TRN-C claim boundary failed: {field}"));
        }
    }
    let reader = fs::read_to_string(root.join(TRN_C_START_GATE_READER_PATH))
        .map_err(|err| format!("failed to read TRN-C reader: {err}"))?;
    if !reader.contains("TRN-C started because TRN-B was complete and has now closed")
        || !reader.contains("TRN-D may start")
    {
        return Err("TRN-C reader boundary failed".to_string());
    }
    Ok(())
}

pub(crate) fn validate_trn_c_candidate_screen(root: &Path) -> Result<(), String> {
    for path in [
        TRN_C_CANDIDATE_SCREEN_JSON_PATH,
        TRN_C_CANDIDATE_SCREEN_SCHEMA_PATH,
        TRN_C_CANDIDATE_SCREEN_READER_PATH,
        HR2247_BILL_SOURCE_PATH,
        HR2247_SCORE_SOURCE_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-C candidate artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, TRN_C_CANDIDATE_SCREEN_JSON_PATH)?;
    if string_field(&record, "record_id")? != "trn-c-candidate-screen:v1"
        || string_field(&record, "status")? != "trn_c_01_complete_one_current_candidate_selected"
        || int_field(&record, "pulse")? != 252
        || string_field(&record, "selected_candidate_id")?
            != "hr2247_airmen_certificate_accessibility"
    {
        return Err("TRN-C candidate screen identity failed".to_string());
    }
    let candidates = record
        .get("candidates")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C candidates")?;
    if candidates.len() != 3
        || candidates
            .iter()
            .filter(|candidate| {
                candidate
                    .get("selected")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
            })
            .count()
            != 1
    {
        return Err("TRN-C candidate selection failed".to_string());
    }
    let aggregate = record
        .get("aggregate_status")
        .ok_or("TRN-C candidate aggregate")?;
    if int_field(aggregate, "candidates_screened")? != 3
        || int_field(aggregate, "candidates_selected")? != 1
        || !bool_field(aggregate, "official_cost_score_ready")?
        || !bool_field(aggregate, "trn_c_01_done")?
    {
        return Err("TRN-C candidate aggregate failed".to_string());
    }
    validate_claim_boundary(
        &record,
        "TRN-C candidate",
        &[
            "candidate_screen_published",
            "trn_c_01_done",
            "candidate_selected",
        ],
    )?;
    for (path, phrase) in [
        (
            HR2247_BILL_SOURCE_PATH,
            "Airmen Certificate Accessibility Act",
        ),
        (HR2247_SCORE_SOURCE_PATH, "18"),
        (TRN_C_CANDIDATE_SCREEN_READER_PATH, "H.R. 2247"),
    ] {
        let prose = fs::read_to_string(root.join(path)).map_err(|err| err.to_string())?;
        if !prose.contains(phrase) {
            return Err(format!("TRN-C candidate evidence missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_trn_c_airmen_certificate_reform_scenario(root: &Path) -> Result<(), String> {
    for path in [
        TRN_C_SCENARIO_JSON_PATH,
        TRN_C_SCENARIO_SCHEMA_PATH,
        TRN_C_SCENARIO_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-C scenario artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, TRN_C_SCENARIO_JSON_PATH)?;
    if string_field(&record, "record_id")? != "trn-c-airmen-certificate-reform-scenario:v1"
        || string_field(&record, "status")? != "admitted_cost_only_real_reform_no_savings_credit"
        || int_field(&record, "pulse")? != 253
        || string_field(&record, "candidate_screen_path")? != TRN_C_CANDIDATE_SCREEN_JSON_PATH
    {
        return Err("TRN-C scenario identity failed".to_string());
    }
    let effects = record
        .get("annual_federal_effects")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C annual effects")?;
    if effects.len() != 7
        || int_field(&effects[0], "fiscal_year")? != 2025
        || int_field(&effects[6], "fiscal_year")? != 2031
        || !effects[1]
            .get("estimated_outlays_millions")
            .is_some_and(serde_json::Value::is_null)
        || effects[1]
            .get("outlay_upper_bound_exclusive_millions")
            .and_then(serde_json::Value::as_f64)
            != Some(0.5)
    {
        return Err("TRN-C bounded annual effects failed".to_string());
    }
    let exact_sum = effects[2..]
        .iter()
        .map(|effect| {
            effect
                .get("estimated_outlays_millions")
                .and_then(serde_json::Value::as_i64)
                .ok_or("exact outlay")
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<i64>();
    let totals = record.get("score_totals").ok_or("TRN-C totals")?;
    if exact_sum != 18
        || int_field(totals, "fy2026_fy2031_estimated_outlays_millions")? != 18
        || string_field(totals, "budget_function")? != "400_transportation"
    {
        return Err("TRN-C official score totals failed".to_string());
    }
    let floors = record
        .get("applicable_floor_review")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C floors")?;
    if floors.len() != 6 {
        return Err("TRN-C floor review count failed".to_string());
    }
    for floor in floors {
        if bool_field(floor, "applicable")? {
            if !bool_field(floor, "policy_pass")? || !bool_field(floor, "stress_pass")? {
                return Err("TRN-C applicable floor failed".to_string());
            }
        } else if !floor
            .get("policy_pass")
            .is_some_and(serde_json::Value::is_null)
            || !floor
                .get("stress_pass")
                .is_some_and(serde_json::Value::is_null)
        {
            return Err("TRN-C non-applicable floor carries a synthetic pass".to_string());
        }
    }
    let admission = record.get("admission_decision").ok_or("TRN-C admission")?;
    if !bool_field(admission, "cost_only_reform_scenario_admitted")?
        || bool_field(admission, "lower_cost_scenario_admitted")?
        || bool_field(admission, "target_outlay_reduction_admitted")?
    {
        return Err("TRN-C cost-only admission boundary failed".to_string());
    }
    validate_claim_boundary(
        &record,
        "TRN-C scenario",
        &[
            "real_reform_scenario_published",
            "official_federal_effect_published",
            "transition_cost_published",
        ],
    )
}

pub(crate) fn validate_trn_c_real_reform_closure(root: &Path) -> Result<(), String> {
    for path in [
        TRN_C_CLOSURE_JSON_PATH,
        TRN_C_CLOSURE_SCHEMA_PATH,
        TRN_C_CLOSURE_READER_PATH,
        TRN_C_CLOSURE_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing TRN-C closure artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, TRN_C_CLOSURE_JSON_PATH)?;
    if string_field(&record, "record_id")? != "trn-c-real-reform-closure:v1"
        || string_field(&record, "status")? != "trn_c_complete_cost_only_reform_admitted"
        || int_field(&record, "pulse")? != 255
    {
        return Err("TRN-C closure identity failed".to_string());
    }
    let packages = record
        .get("work_package_results")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-C closure packages")?;
    if packages.len() != 5
        || packages.iter().any(|package| {
            package.get("status").and_then(serde_json::Value::as_str) != Some("complete")
        })
    {
        return Err("TRN-C closure packages failed".to_string());
    }
    let decision = record
        .get("closure_decision")
        .ok_or("TRN-C closure decision")?;
    if int_field(decision, "fy2026_fy2031_cbo_outlays_millions")? != 18
        || !bool_field(decision, "cost_only_reform_admitted")?
        || !bool_field(decision, "trn_c_done")?
        || !bool_field(decision, "trn_d_may_start")?
        || bool_field(decision, "target_cost_admitted")?
    {
        return Err("TRN-C closure decision failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("TRN-C closure blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("TRN-C closure blocked outputs must be null".to_string());
    }
    validate_claim_boundary(
        &record,
        "TRN-C closure",
        &[
            "trn_c_closure_published",
            "trn_c_done",
            "trn_d_may_start",
            "real_reform_score_published",
        ],
    )
}

pub(crate) fn validate_trn_level_1_core_lessons_audit(root: &Path) -> Result<(), String> {
    for path in [
        TRN_LEVEL_1_CORE_LESSONS_JSON_PATH,
        TRN_LEVEL_1_CORE_LESSONS_SCHEMA_PATH,
        TRN_LEVEL_1_CORE_LESSONS_READER_PATH,
        TRN_LEVEL_1_CORE_LESSONS_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing TRN Level-1 CORE lessons artifact: {path}"));
        }
    }

    let audit = read_json_artifact(root, TRN_LEVEL_1_CORE_LESSONS_JSON_PATH)?;
    for field in [
        "trn_candidate_path",
        "trn_d_closure_path",
        "trn_e_readiness_path",
        "trn_e_closure_path",
        "lane_f_contract_path",
        "advancement_queue_path",
    ] {
        let path = string_field(&audit, field)?;
        if !root.join(&path).is_file() {
            return Err(format!(
                "TRN Level-1 CORE lessons dependency missing: {path}"
            ));
        }
    }
    let candidate = audit
        .get("candidate_finding")
        .ok_or("TRN Level-1 candidate finding")?;
    let coverage = audit
        .get("existing_core_coverage")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-1 existing CORE coverage")?;
    let lessons = audit
        .get("reusable_lessons")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-1 reusable lessons")?;
    let recommendation = audit
        .get("core_recommendation")
        .ok_or("TRN Level-1 CORE recommendation")?;
    let work_packages = recommendation
        .get("proposed_work_packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M proposed work packages")?;
    let trn_work = audit
        .get("trn_specific_work_remaining")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-specific work remaining")?;
    let expected_core = ["CORE-G", "CORE-H", "CORE-I", "CORE-J", "CORE-K", "CORE-L"];
    if string_field(&audit, "record_id")? != "trn-level-1-core-lessons-audit:v1"
        || string_field(&audit, "record_family")? != "trn_level_1_core_lessons_audit"
        || string_field(&audit, "status")?
            != "trn_level_1_core_lessons_audited_core_m_discovery_recommended"
        || int_field(&audit, "pulse")? != 383
        || string_field(&audit, "schema_path")? != TRN_LEVEL_1_CORE_LESSONS_SCHEMA_PATH
        || string_field(&audit, "reader_path")? != TRN_LEVEL_1_CORE_LESSONS_READER_PATH
        || string_field(&audit, "role_review_path")? != TRN_LEVEL_1_CORE_LESSONS_REVIEW_PATH
        || string_field(candidate, "candidate_id")? != "hr2247_airmen_certificate_accessibility"
        || string_field(candidate, "candidate_class")? != "cost_only_service_modernization"
        || !bool_field(candidate, "official_incremental_outlay_score_present")?
        || bool_field(candidate, "lower_cost_target_supported")?
        || bool_field(candidate, "savings_supported")?
        || bool_field(candidate, "receipt_or_fee_instrument_present")?
        || bool_field(candidate, "named_fund_or_account_supported")?
        || bool_field(candidate, "candidate_enacted")?
        || coverage.len() != 6
        || coverage.iter().zip(expected_core).any(|(row, expected)| {
            string_field(row, "core_wave").ok().as_deref() != Some(expected)
                || string_field(row, "covered").is_err()
                || string_field(row, "remaining_gap").is_err()
        })
        || lessons.len() != 6
        || lessons.iter().any(|row| {
            string_field(row, "lesson_id").is_err()
                || string_field(row, "finding").is_err()
                || string_field(row, "shared_need").is_err()
        })
        || string_field(recommendation, "recommended_wave_id")? != "CORE-M"
        || string_field(recommendation, "working_title")?
            != "Candidate Dossier and Typed Release Profiles"
        || !bool_field(recommendation, "discovery_warranted")?
        || bool_field(recommendation, "implementation_started")?
        || bool_field(recommendation, "contract_complete")?
        || work_packages.len() != 6
        || work_packages
            .iter()
            .any(|item| item.as_str().is_none_or(str::is_empty))
        || trn_work.len() != 5
        || trn_work
            .iter()
            .any(|item| item.as_str().is_none_or(str::is_empty))
    {
        return Err("TRN Level-1 CORE lessons audit failed".to_string());
    }
    validate_blocked_outputs_null(&audit, "TRN Level-1 CORE lessons")?;
    validate_claim_boundary(
        &audit,
        "TRN Level-1 CORE lessons",
        &[
            "trn_level_1_core_lessons_audit_published",
            "existing_core_coverage_mapped",
            "core_m_discovery_recommended",
        ],
    )?;

    for (path, phrase) in [
        (
            TRN_LEVEL_1_CORE_LESSONS_SCHEMA_PATH,
            "candidate class controls applicable solver",
        ),
        (TRN_LEVEL_1_CORE_LESSONS_READER_PATH, "CORE-M"),
        (
            TRN_LEVEL_1_CORE_LESSONS_REVIEW_PATH,
            "CORE-M remains a recommendation only",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN Level-1 CORE lessons prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_trn_level_1_hr2247_candidate_dossier(root: &Path) -> Result<(), String> {
    for path in [
        TRN_LEVEL_1_DOSSIER_JSON_PATH,
        TRN_LEVEL_1_DOSSIER_SCHEMA_PATH,
        TRN_LEVEL_1_DOSSIER_READER_PATH,
        TRN_LEVEL_1_DOSSIER_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing TRN Level-1 dossier artifact: {path}"));
        }
    }
    let dossier = read_json_artifact(root, TRN_LEVEL_1_DOSSIER_JSON_PATH)?;
    for field in [
        "core_m_path",
        "scenario_path",
        "legal_financing_path",
        "administration_behavior_path",
        "incidence_fairness_path",
        "interactions_path",
        "prior_e_closure_path",
    ] {
        let path = string_field(&dossier, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("TRN Level-1 dossier dependency missing: {path}"));
        }
    }
    let profile = dossier
        .get("candidate_profile")
        .ok_or("TRN Level-1 candidate profile")?;
    let financing = dossier
        .get("financing_roles")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-1 financing roles")?;
    let gates = dossier
        .get("gate_reviews")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-1 gate reviews")?;
    let outputs = dossier
        .get("requested_release_outputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-1 requested outputs")?;
    let decision = dossier
        .get("selection_decision")
        .ok_or("TRN Level-1 selection decision")?;
    let gate_ids = gates
        .iter()
        .map(|row| string_field(row, "gate_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let disposition_count = |expected: &str| {
        gates
            .iter()
            .filter(|row| {
                row.get("disposition").and_then(serde_json::Value::as_str) == Some(expected)
            })
            .count()
    };
    if string_field(&dossier, "record_id")? != "trn-level-1-hr2247-candidate-dossier:v1"
        || string_field(&dossier, "record_family")? != "trn_level_1_hr2247_candidate_dossier"
        || string_field(&dossier, "status")?
            != "trn_level_1_complete_hr2247_selected_cost_only_e_rerun_required"
        || int_field(&dossier, "pulse")? != 392
        || string_field(&dossier, "track_prefix")? != "TRN"
        || string_field(&dossier, "lane_id")? != "transportation-infrastructure"
        || string_field(&dossier, "schema_path")? != TRN_LEVEL_1_DOSSIER_SCHEMA_PATH
        || string_field(&dossier, "reader_path")? != TRN_LEVEL_1_DOSSIER_READER_PATH
        || string_field(&dossier, "role_review_path")? != TRN_LEVEL_1_DOSSIER_REVIEW_PATH
        || string_field(profile, "candidate_id")? != "hr2247_airmen_certificate_accessibility"
        || string_field(profile, "objective_profile")? != "cost_only_modernization"
        || !bool_field(profile, "conditional_on_enactment_and_appropriation")?
        || int_field(profile, "official_incremental_outlays_millions")? != 18
        || financing.len() != 3
        || financing.iter().any(|row| {
            !bool_field(row, "source_supported").unwrap_or(false)
                || string_field(row, "role").is_err()
                || string_field(row, "source_id").is_err()
        })
        || gates.len() != 10
        || gate_ids.len() != 10
        || disposition_count("required_ready") != 5
        || disposition_count("reviewed_not_applicable") != 4
        || disposition_count("required_blocked") != 1
        || outputs.len() != 2
        || outputs[0].as_str() != Some("candidate_explanation")
        || outputs[1].as_str() != Some("official_incremental_cost")
        || !bool_field(decision, "selection_complete")?
        || !bool_field(decision, "candidate_selected")?
        || string_field(decision, "selected_for_profile")? != "cost_only_modernization"
        || bool_field(decision, "selected_for_rate_or_savings")?
        || !bool_field(decision, "dossier_valid")?
        || bool_field(decision, "all_required_gates_ready")?
        || bool_field(decision, "release_ready")?
        || !bool_field(decision, "trn_level_1_done")?
        || !bool_field(decision, "trn_level_2_may_start")?
    {
        return Err("TRN Level-1 H.R. 2247 candidate dossier failed".to_string());
    }
    validate_blocked_outputs_null(&dossier, "TRN Level-1 dossier")?;
    validate_claim_boundary(
        &dossier,
        "TRN Level-1 dossier",
        &[
            "trn_level_1_candidate_dossier_published",
            "core_m_profile_applied",
            "candidate_selected",
            "cost_only_profile_selected",
            "trn_level_1_done",
            "trn_level_2_may_start",
        ],
    )?;
    for (path, phrase) in [
        (TRN_LEVEL_1_DOSSIER_SCHEMA_PATH, "profile-specific"),
        (TRN_LEVEL_1_DOSSIER_READER_PATH, "TRN Level 1 is complete"),
        (
            TRN_LEVEL_1_DOSSIER_REVIEW_PATH,
            "approve H.R. 2247 for the CORE-M cost-only modernization profile",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN Level-1 dossier prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_trn_level_2_hr2247_output_ready_e_rerun(root: &Path) -> Result<(), String> {
    for path in [
        TRN_LEVEL_2_E_RERUN_JSON_PATH,
        TRN_LEVEL_2_E_RERUN_SCHEMA_PATH,
        TRN_LEVEL_2_E_RERUN_READER_PATH,
        TRN_LEVEL_2_E_RERUN_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing TRN Level-2 E-rerun artifact: {path}"));
        }
    }
    let rerun = read_json_artifact(root, TRN_LEVEL_2_E_RERUN_JSON_PATH)?;
    for field in [
        "candidate_dossier_path",
        "prior_e_closure_path",
        "core_m_path",
        "lane_f_contract_path",
        "scenario_path",
    ] {
        let path = string_field(&rerun, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("TRN Level-2 E-rerun dependency missing: {path}"));
        }
    }
    let profile = rerun
        .get("rerun_profile")
        .ok_or("TRN Level-2 rerun profile")?;
    let gates = rerun
        .get("f_gate_audit")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN Level-2 F-gate audit")?;
    let admitted = rerun
        .get("admitted_outputs")
        .ok_or("TRN Level-2 admitted outputs")?;
    let cost = admitted
        .get("official_incremental_cost")
        .ok_or("TRN Level-2 official incremental cost")?;
    let decision = rerun
        .get("closure_decision")
        .ok_or("TRN Level-2 closure decision")?;
    let gate_ids = gates
        .iter()
        .map(|row| string_field(row, "gate_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let disposition_count = |expected: &str| {
        gates
            .iter()
            .filter(|row| {
                row.get("disposition").and_then(serde_json::Value::as_str) == Some(expected)
            })
            .count()
    };
    if string_field(&rerun, "record_id")? != "trn-level-2-hr2247-output-ready-e-rerun:v1"
        || string_field(&rerun, "record_family")? != "trn_level_2_hr2247_output_ready_e_rerun"
        || string_field(&rerun, "status")?
            != "trn_level_2_complete_output_ready_cost_only_trn_f_may_start"
        || int_field(&rerun, "pulse")? != 398
        || string_field(&rerun, "track_prefix")? != "TRN"
        || string_field(&rerun, "lane_id")? != "transportation-infrastructure"
        || string_field(&rerun, "track_wave_id")? != "TRN-E"
        || string_field(&rerun, "schema_path")? != TRN_LEVEL_2_E_RERUN_SCHEMA_PATH
        || string_field(&rerun, "reader_path")? != TRN_LEVEL_2_E_RERUN_READER_PATH
        || string_field(&rerun, "role_review_path")? != TRN_LEVEL_2_E_RERUN_REVIEW_PATH
        || string_field(profile, "candidate_id")? != "hr2247_airmen_certificate_accessibility"
        || string_field(profile, "objective_profile")? != "cost_only_modernization"
        || !bool_field(profile, "conditional_on_enactment_and_appropriation")?
        || int_field(profile, "official_incremental_outlays_millions")? != 18
        || bool_field(profile, "integrated_solver_required")?
        || bool_field(profile, "rate_solver_required")?
        || bool_field(profile, "savings_solver_required")?
        || gates.len() != 10
        || gate_ids.len() != 10
        || disposition_count("required_ready") != 7
        || disposition_count("reviewed_not_applicable") != 3
        || gates.iter().any(|row| {
            let disposition = string_field(row, "disposition").ok();
            match disposition.as_deref() {
                Some("required_ready") => {
                    string_field(row, "evidence_path").is_err() || row.get("rationale").is_some()
                }
                Some("reviewed_not_applicable") => {
                    string_field(row, "rationale").is_err() || row.get("evidence_path").is_some()
                }
                _ => true,
            }
        })
        || string_field(admitted, "candidate_explanation")?
            .trim()
            .is_empty()
        || int_field(cost, "amount_millions")? != 18
        || string_field(cost, "horizon")? != "FY2026-FY2031"
        || string_field(cost, "source_id")? != "SRC-GPO-HRPT119-551-2026"
        || !bool_field(cost, "conditional_on_enactment_and_appropriation")?
        || string_field(decision, "completion_class")? != "output_ready_typed_cost_only"
        || !bool_field(decision, "candidate_selected")?
        || !bool_field(decision, "all_ten_f_gates_dispositioned")?
        || !bool_field(decision, "all_profile_required_gates_ready")?
        || !bool_field(decision, "e_output_ready")?
        || !bool_field(decision, "trn_level_2_done")?
        || !bool_field(decision, "trn_f_may_start")?
        || bool_field(decision, "solver_run_performed")?
        || bool_field(decision, "numeric_solver_completion")?
        || bool_field(decision, "rate_or_savings_release_ready")?
    {
        return Err("TRN Level-2 H.R. 2247 output-ready E rerun failed".to_string());
    }
    validate_blocked_outputs_null(&rerun, "TRN Level-2 E rerun")?;
    validate_claim_boundary(
        &rerun,
        "TRN Level-2 E rerun",
        &[
            "trn_level_2_e_rerun_published",
            "candidate_selected",
            "cost_only_profile_applied",
            "e_output_ready",
            "trn_level_2_done",
            "trn_f_may_start",
        ],
    )?;
    for (path, phrase) in [
        (
            TRN_LEVEL_2_E_RERUN_SCHEMA_PATH,
            "TRN-F may start only within the admitted output scope",
        ),
        (TRN_LEVEL_2_E_RERUN_READER_PATH, "TRN Level 2 is complete"),
        (
            TRN_LEVEL_2_E_RERUN_REVIEW_PATH,
            "approve the TRN-E rerun as output-ready",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN Level-2 E-rerun prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_trn_f_hr2247_cost_note(root: &Path) -> Result<(), String> {
    for path in [
        TRN_F_COST_NOTE_JSON_PATH,
        TRN_F_COST_NOTE_SCHEMA_PATH,
        TRN_F_COST_NOTE_READER_PATH,
        TRN_F_COST_NOTE_REVIEW_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing TRN-F cost-note artifact: {path}"));
        }
    }
    let note = read_json_artifact(root, TRN_F_COST_NOTE_JSON_PATH)?;
    for field in [
        "core_n_path",
        "trn_level_2_path",
        "candidate_dossier_path",
        "scenario_path",
    ] {
        let path = string_field(&note, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("TRN-F cost-note dependency missing: {path}"));
        }
    }
    let identity = note
        .get("release_identity")
        .ok_or("TRN-F release identity")?;
    let public_note = note
        .get("public_cost_note")
        .ok_or("TRN-F public cost note")?;
    let checks = note
        .get("release_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("TRN-F release checks")?;
    let decision = note
        .get("closure_decision")
        .ok_or("TRN-F closure decision")?;
    if string_field(&note, "record_id")? != "trn-f-hr2247-cost-note:v1"
        || string_field(&note, "record_family")? != "trn_f_hr2247_cost_note"
        || string_field(&note, "status")? != "trn_f_complete_public_cost_note_only"
        || int_field(&note, "pulse")? != 406
        || string_field(&note, "track_wave_id")? != "TRN-F"
        || string_field(&note, "schema_path")? != TRN_F_COST_NOTE_SCHEMA_PATH
        || string_field(&note, "reader_path")? != TRN_F_COST_NOTE_READER_PATH
        || string_field(&note, "role_review_path")? != TRN_F_COST_NOTE_REVIEW_PATH
        || string_field(identity, "surface")? != "cost_note"
        || string_field(identity, "candidate_id")? != "hr2247_airmen_certificate_accessibility"
        || string_field(identity, "objective_profile")? != "cost_only_modernization"
        || !bool_field(identity, "conditional_on_enactment_and_appropriation")?
        || int_field(public_note, "official_incremental_cost_millions")? != 18
        || string_field(public_note, "score_horizon")? != "FY2026-FY2031"
        || string_field(public_note, "score_source_id")? != "SRC-GPO-HRPT119-551-2026"
        || string_field(public_note, "public_claim_status")? != "publishable_cost_note_only"
        || checks.len() != 6
        || checks
            .iter()
            .any(|row| string_field(row, "status").ok().as_deref() != Some("pass"))
        || !bool_field(decision, "trn_f_started")?
        || !bool_field(decision, "trn_f_done")?
        || string_field(decision, "surface")? != "cost_note"
        || !bool_field(decision, "public_cost_note_published")?
        || bool_field(decision, "public_rate_card_published")?
        || bool_field(decision, "solver_run_performed")?
    {
        return Err("TRN-F H.R. 2247 cost note failed".to_string());
    }
    validate_blocked_outputs_null(&note, "TRN-F cost note")?;
    validate_claim_boundary(
        &note,
        "TRN-F cost note",
        &[
            "trn_f_cost_note_published",
            "core_n_surface_applied",
            "trn_f_started",
            "trn_f_done",
        ],
    )?;
    for (path, phrase) in [
        (TRN_F_COST_NOTE_SCHEMA_PATH, "contain exactly a"),
        (TRN_F_COST_NOTE_READER_PATH, "TRN-F is complete"),
        (
            TRN_F_COST_NOTE_REVIEW_PATH,
            "approve the H.R. 2247 cost note",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("TRN-F cost-note prose missing: {phrase}"));
        }
    }
    Ok(())
}

