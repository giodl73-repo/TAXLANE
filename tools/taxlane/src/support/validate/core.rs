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

pub(crate) fn validate_core_g_official_current_law_solver_spine(root: &Path) -> Result<(), String> {
    for path in [
        CORE_G_SOLVER_SPINE_JSON_PATH,
        CORE_G_SOLVER_SPINE_SCHEMA_PATH,
        CORE_G_SOLVER_SPINE_READER_PATH,
        CORE_G_SOLVER_SPINE_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing CORE-G solver spine artifact: {path}"));
        }
    }

    let text = fs::read_to_string(root.join(CORE_G_SOLVER_SPINE_JSON_PATH))
        .map_err(|err| format!("failed to read CORE-G spine: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse CORE-G spine: {err}"))?;
    if string_field(&record, "record_id")? != "core-g-official-current-law-solver-spine:v1"
        || string_field(&record, "record_family")? != "core_g_official_current_law_solver_spine"
        || string_field(&record, "status")?
            != "core_g_complete_official_current_law_topline_spine_admitted"
        || string_field(&record, "core_wave_id")? != "CORE-G"
        || int_field(&record, "pulse")? != 236
        || string_field(&record, "schema_path")? != CORE_G_SOLVER_SPINE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != CORE_G_SOLVER_SPINE_READER_PATH
        || string_field(&record, "contract_path")? != WAVE_G_SOLVER_SPINE_CONTRACT_JSON_PATH
        || string_field(&record, "corpus_plan_path")? != CORPUS_TRACK_PLAN_JSON_PATH
        || string_field(&record, "role_review_path")? != CORE_G_SOLVER_SPINE_ROLE_REVIEW_PATH
    {
        return Err("CORE-G spine identity failed".to_string());
    }

    let packets = record
        .get("source_packets")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-G source packets")?;
    if packets.len() != 2 {
        return Err("CORE-G must have exactly two source packets".to_string());
    }
    for packet in packets {
        if string_field(packet, "publisher")? != "Congressional Budget Office"
            || string_field(packet, "source_vintage")? != "2026-02"
            || string_field(packet, "retrieval_date")? != "2026-07-23"
            || packet
                .get("variables")
                .and_then(serde_json::Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err("CORE-G source packet metadata failed".to_string());
        }
        let raw_path = string_field(packet, "raw_artifact_path")?;
        let raw = root.join(&raw_path);
        let metadata_path = string_field(packet, "metadata_path")?;
        if !raw.exists() || !root.join(&metadata_path).exists() {
            return Err(format!("CORE-G source custody missing: {raw_path}"));
        }
        let observed_bytes = fs::metadata(&raw)
            .map_err(|err| format!("failed to stat {raw_path}: {err}"))?
            .len();
        if observed_bytes != int_field(packet, "byte_count")? as u64
            || sha256_file(&raw)? != string_field(packet, "sha256")?
        {
            return Err(format!("CORE-G source checksum failed: {raw_path}"));
        }
    }

    let extraction = record
        .get("extraction_contract")
        .ok_or("CORE-G extraction contract")?;
    if string_field(extraction, "source_unit")? != "billions_usd"
        || string_field(extraction, "published_unit")? != "millions_usd"
        || string_field(extraction, "projection_vintage")? != "2026-02"
        || !bool_field(extraction, "source_precision_preserved")?
        || bool_field(extraction, "interpolation_used")?
        || !bool_field(extraction, "fy2025_actual_from_same_projection_release")?
        || bool_field(extraction, "cross_vintage_stitching_used")?
    {
        return Err("CORE-G extraction contract failed".to_string());
    }

    let rows = record
        .get("annual_rows")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-G annual rows")?;
    if rows.len() != 11 {
        return Err("CORE-G must contain exactly eleven annual rows".to_string());
    }
    let mut prior_end = None;
    for (index, row) in rows.iter().enumerate() {
        let year = int_field(row, "fiscal_year")?;
        let expected_year = 2025 + index as i64;
        if year != expected_year
            || string_field(row, "actual_or_projection")?
                != if index == 0 { "actual" } else { "projection" }
            || string_field(row, "source_id")? != "SRC-CBO-OPEN-DATA-TEN-YEAR-BUDGET-2026-02"
            || string_field(row, "source_vintage")? != "2026-02"
        {
            return Err(format!("CORE-G annual row lineage failed: FY{year}"));
        }
        let gdp = int_field(row, "gdp_musd")?;
        let receipts = int_field(row, "total_receipts_musd")?;
        let primary_outlays = int_field(row, "primary_outlays_musd")?;
        let interest = int_field(row, "net_interest_musd")?;
        let total_outlays = int_field(row, "total_outlays_musd")?;
        let primary_deficit = int_field(row, "primary_deficit_musd")?;
        let total_deficit = int_field(row, "total_deficit_musd")?;
        let financing = int_field(row, "other_financing_and_timing_musd")?;
        let reported_financing = int_field(row, "reported_other_financing_musd")?;
        let timing = int_field(row, "timing_residual_musd")?;
        let begin_debt = int_field(row, "debt_held_by_public_begin_musd")?;
        let end_debt = int_field(row, "debt_held_by_public_end_musd")?;
        let average_rate = row
            .get("average_interest_rate_percent")
            .and_then(serde_json::Value::as_f64)
            .ok_or("CORE-G average rate")?;
        if gdp <= 0
            || average_rate <= 0.0
            || total_outlays != primary_outlays + interest
            || primary_deficit != primary_outlays - receipts
            || total_deficit != total_outlays - receipts
            || total_deficit != primary_deficit + interest
            || end_debt != begin_debt + total_deficit + financing
            || timing != financing - reported_financing
            || prior_end.is_some_and(|value| value != begin_debt)
        {
            return Err(format!("CORE-G identity failed: FY{year}"));
        }
        prior_end = Some(end_debt);
    }
    if int_field(&rows[0], "timing_residual_musd")? != -13
        || rows[1..]
            .iter()
            .any(|row| int_field(row, "timing_residual_musd").ok() != Some(0))
    {
        return Err("CORE-G timing-residual disclosure failed".to_string());
    }

    let gates = record
        .get("completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-G completion gates")?;
    let expected_gates = BTreeSet::from([
        "source_custody",
        "complete_horizon",
        "vintage_control",
        "topline_reconciliation",
        "debt_rollforward",
        "interest_lineage",
        "boundary_review",
        "deterministic_validation",
    ]);
    let observed_gates = gates
        .iter()
        .map(|gate| string_field(gate, "gate_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if gates.len() != 8
        || observed_gates != expected_gates.into_iter().map(str::to_string).collect()
        || gates
            .iter()
            .any(|gate| bool_field(gate, "ready").ok() != Some(true))
    {
        return Err("CORE-G completion gates failed".to_string());
    }

    let aggregate = record.get("aggregate_status").ok_or("CORE-G aggregate")?;
    if int_field(aggregate, "annual_row_count")? != 11
        || int_field(aggregate, "actual_rows")? != 1
        || int_field(aggregate, "projection_rows")? != 10
        || int_field(aggregate, "source_packet_count")? != 2
        || int_field(aggregate, "completion_gate_count")? != 8
        || int_field(aggregate, "ready_completion_gates")? != 8
        || !bool_field(aggregate, "all_topline_identities_pass")?
        || !bool_field(aggregate, "all_debt_rollforwards_pass")?
        || !bool_field(aggregate, "core_g_done")?
        || !bool_field(aggregate, "trn_a_may_start")?
        || bool_field(aggregate, "core_h_done")?
        || bool_field(aggregate, "full_solver_ready")?
    {
        return Err("CORE-G aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-G blocked outputs")?;
    if blocked.is_empty() || blocked.values().any(|value| !value.is_null()) {
        return Err("CORE-G blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-G claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "core_g_spine_published" | "core_g_done" | "trn_a_may_start"
        );
        if value.as_bool().ok_or("CORE-G claim must be bool")? != expected {
            return Err(format!("CORE-G claim boundary failed: {field}"));
        }
    }

    for (path, phrase) in [
        (
            CORE_G_SOLVER_SPINE_SCHEMA_PATH,
            "exactly eleven FY2025-FY2035 federal topline rows",
        ),
        (CORE_G_SOLVER_SPINE_READER_PATH, "CORE-G is complete"),
        (
            CORE_G_SOLVER_SPINE_ROLE_REVIEW_PATH,
            "Approved as the official current-law federal topline spine",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("CORE-G prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_core_h_shared_accounting_substrate(root: &Path) -> Result<(), String> {
    for path in [
        CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH,
        CORE_H_ACCOUNTING_SUBSTRATE_SCHEMA_PATH,
        CORE_H_ACCOUNTING_SUBSTRATE_READER_PATH,
        CORE_H_ACCOUNTING_SUBSTRATE_ROLE_REVIEW_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing CORE-H artifact: {path}"));
        }
    }
    let text = fs::read_to_string(root.join(CORE_H_ACCOUNTING_SUBSTRATE_JSON_PATH))
        .map_err(|err| format!("failed to read CORE-H substrate: {err}"))?;
    let record: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse CORE-H substrate: {err}"))?;
    if string_field(&record, "record_id")? != "core-h-shared-accounting-substrate:v1"
        || string_field(&record, "record_family")? != "core_h_shared_accounting_substrate"
        || string_field(&record, "status")?
            != "core_h_complete_shared_accounting_interfaces_and_engine"
        || string_field(&record, "core_wave_id")? != "CORE-H"
        || int_field(&record, "pulse")? != 241
        || string_field(&record, "schema_path")? != CORE_H_ACCOUNTING_SUBSTRATE_SCHEMA_PATH
        || string_field(&record, "reader_path")? != CORE_H_ACCOUNTING_SUBSTRATE_READER_PATH
        || string_field(&record, "role_review_path")?
            != CORE_H_ACCOUNTING_SUBSTRATE_ROLE_REVIEW_PATH
        || string_field(&record, "core_g_path")? != CORE_G_SOLVER_SPINE_JSON_PATH
        || string_field(&record, "trn_a_path")? != TRN_A_BASELINE_SPINE_JSON_PATH
        || string_field(&record, "implementation_path")? != "crates/taxlane-core/src/lib.rs"
        || !root.join("crates/taxlane-core/src/lib.rs").exists()
    {
        return Err("CORE-H identity failed".to_string());
    }
    let interfaces = record
        .get("implemented_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-H interfaces")?;
    let expected_interfaces = BTreeSet::from([
        "named_fund_year".to_string(),
        "reserve_year".to_string(),
        "federal_year".to_string(),
        "endogenous_interest".to_string(),
    ]);
    let observed_interfaces = interfaces
        .iter()
        .map(|row| string_field(row, "interface_id"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if interfaces.len() != 4
        || observed_interfaces != expected_interfaces
        || interfaces
            .iter()
            .any(|row| bool_field(row, "ready").ok() != Some(true))
    {
        return Err("CORE-H interface set failed".to_string());
    }
    let implementation = fs::read_to_string(root.join("crates/taxlane-core/src/lib.rs"))
        .map_err(|err| format!("failed to read CORE-H implementation: {err}"))?;
    for symbol in [
        "pub struct FundYearInput",
        "pub fn reconcile_fund_year",
        "pub struct ReserveYearInput",
        "pub fn reconcile_reserve_year",
        "pub struct FederalYearInput",
        "pub fn reconcile_federal_year",
        "pub fn compute_interest_from_rate_millionths",
    ] {
        if !implementation.contains(symbol) {
            return Err(format!("CORE-H implementation symbol missing: {symbol}"));
        }
    }
    let controls = record
        .get("interface_controls")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-H controls")?;
    if controls.len() != 12 || controls.values().any(|value| value.as_bool() != Some(true)) {
        return Err("CORE-H interface controls failed".to_string());
    }
    let findings = record
        .get("trn_a_findings_resolved")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-H TRN-A findings")?;
    let gates = record
        .get("completion_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-H gates")?;
    if findings.len() != 10
        || findings
            .iter()
            .any(|row| bool_field(row, "ready").ok() != Some(true))
        || gates.len() != 9
        || gates
            .iter()
            .any(|gate| bool_field(gate, "ready").ok() != Some(true))
    {
        return Err("CORE-H findings or gates failed".to_string());
    }
    let acceptance = record
        .get("deterministic_acceptance")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-H deterministic acceptance")?;
    if acceptance
        .values()
        .any(|value| value.as_bool() != Some(true))
    {
        return Err("CORE-H deterministic acceptance failed".to_string());
    }
    let aggregate = record.get("aggregate_status").ok_or("CORE-H aggregate")?;
    if int_field(aggregate, "implemented_interface_count")? != 4
        || int_field(aggregate, "trn_a_findings_resolved")? != 10
        || !bool_field(aggregate, "core_h_done")?
        || !bool_field(aggregate, "trn_a_done")?
        || !bool_field(aggregate, "trn_b_may_start")?
        || bool_field(aggregate, "numeric_lane_parameters_ready")?
        || bool_field(aggregate, "lane_solver_ready")?
    {
        return Err("CORE-H aggregate boundary failed".to_string());
    }
    let blocked = record
        .get("blocked_outputs")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-H blocked outputs")?;
    if blocked.values().any(|value| !value.is_null()) {
        return Err("CORE-H blocked outputs must remain null".to_string());
    }
    let claims = record
        .get("claim_booleans")
        .and_then(serde_json::Value::as_object)
        .ok_or("CORE-H claims")?;
    for (field, value) in claims {
        let expected = matches!(
            field.as_str(),
            "core_h_substrate_published" | "core_h_done" | "trn_a_done" | "trn_b_may_start"
        );
        if value.as_bool().ok_or("CORE-H claim bool")? != expected {
            return Err(format!("CORE-H claim boundary failed: {field}"));
        }
    }
    for (path, phrase) in [
        (
            CORE_H_ACCOUNTING_SUBSTRATE_SCHEMA_PATH,
            "reusable checked-integer interfaces",
        ),
        (
            CORE_H_ACCOUNTING_SUBSTRATE_READER_PATH,
            "CORE-H is complete",
        ),
        (
            CORE_H_ACCOUNTING_SUBSTRATE_ROLE_REVIEW_PATH,
            "Approved as the completed shared accounting substrate",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("CORE-H prose missing: {phrase}"));
        }
    }
    Ok(())
}

pub(crate) fn validate_core_i_shared_reform_admission_contract(root: &Path) -> Result<(), String> {
    for path in [
        CORE_I_REFORM_ADMISSION_JSON_PATH,
        CORE_I_REFORM_ADMISSION_SCHEMA_PATH,
        CORE_I_REFORM_ADMISSION_READER_PATH,
    ] {
        if !root.join(path).exists() {
            return Err(format!("missing CORE-I artifact: {path}"));
        }
    }
    let record = read_json_artifact(root, CORE_I_REFORM_ADMISSION_JSON_PATH)?;
    if string_field(&record, "record_id")? != "core-i-shared-reform-admission-contract:v1"
        || string_field(&record, "status")? != "core_i_complete_shared_reform_admission_interfaces"
        || int_field(&record, "pulse")? != 254
    {
        return Err("CORE-I identity failed".to_string());
    }
    let interfaces = record
        .get("required_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-I interfaces")?;
    if interfaces.len() != 8
        || interfaces
            .iter()
            .any(|item| item.get("implemented").and_then(serde_json::Value::as_bool) != Some(true))
    {
        return Err("CORE-I implementation interface failed".to_string());
    }
    let rust = fs::read_to_string(root.join("crates/taxlane-core/src/lib.rs"))
        .map_err(|err| err.to_string())?;
    for symbol in record
        .get("rust_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-I Rust interfaces")?
    {
        let symbol = symbol.as_str().ok_or("CORE-I Rust symbol")?;
        if !rust.contains(symbol) {
            return Err(format!("CORE-I Rust interface missing: {symbol}"));
        }
    }
    let aggregate = record.get("aggregate_status").ok_or("CORE-I aggregate")?;
    if int_field(aggregate, "implemented_interfaces")? != 8
        || !bool_field(aggregate, "core_i_done")?
        || !bool_field(aggregate, "trn_c_may_close")?
    {
        return Err("CORE-I aggregate failed".to_string());
    }
    validate_claim_boundary(
        &record,
        "CORE-I",
        &[
            "core_i_contract_published",
            "core_i_done",
            "shared_reform_admission_interface_ready",
        ],
    )
}

pub(crate) fn validate_core_m_candidate_dossier_typed_release(root: &Path) -> Result<(), String> {
    for path in [
        CORE_M_CONTRACT_JSON_PATH,
        CORE_M_CONTRACT_SCHEMA_PATH,
        CORE_M_CONTRACT_READER_PATH,
        CORE_M_CONTRACT_REVIEW_PATH,
        CORE_M_CLOSURE_JSON_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing CORE-M artifact: {path}"));
        }
    }

    let contract = read_json_artifact(root, CORE_M_CONTRACT_JSON_PATH)?;
    for field in [
        "lessons_audit_path",
        "core_i_path",
        "core_j_path",
        "core_k_path",
        "core_l_path",
        "implementation_path",
    ] {
        let path = string_field(&contract, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("CORE-M dependency missing: {path}"));
        }
    }
    let profiles = contract
        .get("candidate_objective_profiles")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M candidate profiles")?;
    let financing_roles = contract
        .get("financing_roles")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M financing roles")?;
    let dispositions = contract
        .get("gate_dispositions")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M gate dispositions")?;
    let outputs = contract
        .get("release_outputs")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M release outputs")?;
    let interfaces = contract
        .get("required_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M required interfaces")?;
    let rust_interfaces = contract
        .get("rust_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M Rust interfaces")?;
    let cases = contract
        .get("validation_cases")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M validation cases")?;
    let implementation = contract
        .get("implementation_status")
        .ok_or("CORE-M implementation status")?;
    if string_field(&contract, "record_id")? != "core-m-candidate-dossier-typed-release-contract:v1"
        || string_field(&contract, "record_family")?
            != "core_m_candidate_dossier_typed_release_contract"
        || string_field(&contract, "status")? != "core_m_interfaces_implemented_role_review_ready"
        || int_field(&contract, "pulse")? != 386
        || string_field(&contract, "core_wave_id")? != "CORE-M"
        || string_field(&contract, "schema_path")? != CORE_M_CONTRACT_SCHEMA_PATH
        || string_field(&contract, "reader_path")? != CORE_M_CONTRACT_READER_PATH
        || profiles.len() != 6
        || financing_roles.len() != 8
        || dispositions.len() != 3
        || outputs.len() != 9
        || interfaces.len() != 6
        || interfaces.iter().any(|row| {
            !bool_field(row, "implemented").unwrap_or(false)
                || string_field(row, "interface_id").is_err()
                || string_field(row, "requirement").is_err()
        })
        || rust_interfaces.len() != 9
        || cases.len() != 4
        || int_field(implementation, "required_interfaces")? != 6
        || int_field(implementation, "implemented_interfaces")? != 6
        || int_field(implementation, "rust_interfaces")? != 9
        || int_field(implementation, "validation_cases")? != 4
        || int_field(implementation, "unit_tests_added")? != 6
        || !bool_field(implementation, "core_m_implementation_done")?
        || !bool_field(implementation, "core_m_role_review_may_start")?
    {
        return Err("CORE-M contract failed".to_string());
    }
    let core_source = fs::read_to_string(root.join("crates/taxlane-core/src/lib.rs"))
        .map_err(|err| format!("failed to read CORE-M implementation: {err}"))?;
    for symbol in rust_interfaces {
        let symbol = symbol.as_str().ok_or("CORE-M Rust interface name")?;
        if !core_source.contains(symbol) {
            return Err(format!("CORE-M Rust interface missing: {symbol}"));
        }
    }
    validate_blocked_outputs_null(&contract, "CORE-M contract")?;
    validate_claim_boundary(
        &contract,
        "CORE-M contract",
        &[
            "core_m_contract_published",
            "shared_interfaces_implemented",
            "four_profile_cases_validated",
            "core_m_implementation_done",
            "core_m_role_review_may_start",
        ],
    )?;

    let closure = read_json_artifact(root, CORE_M_CLOSURE_JSON_PATH)?;
    let checks = closure
        .get("closure_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-M closure checks")?;
    let decision = closure
        .get("dependency_decision")
        .ok_or("CORE-M dependency decision")?;
    if string_field(&closure, "record_id")? != "core-m-candidate-dossier-typed-release-closure:v1"
        || string_field(&closure, "record_family")?
            != "core_m_candidate_dossier_typed_release_closure"
        || string_field(&closure, "status")?
            != "core_m_complete_candidate_dossier_typed_release_interfaces"
        || int_field(&closure, "pulse")? != 387
        || string_field(&closure, "core_wave_id")? != "CORE-M"
        || string_field(&closure, "contract_path")? != CORE_M_CONTRACT_JSON_PATH
        || string_field(&closure, "role_review_path")? != CORE_M_CONTRACT_REVIEW_PATH
        || checks.len() != 10
        || checks
            .iter()
            .any(|row| row.get("status").and_then(serde_json::Value::as_str) != Some("pass"))
        || !bool_field(decision, "core_m_done")?
        || !bool_field(decision, "trn_level_1_may_resume")?
        || bool_field(decision, "trn_e_output_ready_rerun_allowed")?
    {
        return Err("CORE-M closure failed".to_string());
    }
    validate_blocked_outputs_null(&closure, "CORE-M closure")?;
    validate_claim_boundary(
        &closure,
        "CORE-M closure",
        &[
            "core_m_closure_published",
            "role_review_complete",
            "core_m_done",
            "trn_level_1_may_resume",
        ],
    )?;

    for (path, phrase) in [
        (
            CORE_M_CONTRACT_SCHEMA_PATH,
            "Cost-only modernization cannot emit target-cost",
        ),
        (
            CORE_M_CONTRACT_READER_PATH,
            "CORE-M does not select a candidate",
        ),
        (
            CORE_M_CONTRACT_REVIEW_PATH,
            "CORE-M is complete as shared infrastructure",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("CORE-M prose missing: {phrase}"));
        }
    }

    Ok(())
}

pub(crate) fn validate_core_n_typed_public_release_surfaces(root: &Path) -> Result<(), String> {
    for path in [
        CORE_N_CONTRACT_JSON_PATH,
        CORE_N_CONTRACT_SCHEMA_PATH,
        CORE_N_CONTRACT_READER_PATH,
        CORE_N_CONTRACT_REVIEW_PATH,
        CORE_N_CLOSURE_JSON_PATH,
    ] {
        if !root.join(path).is_file() {
            return Err(format!("missing CORE-N artifact: {path}"));
        }
    }
    let contract = read_json_artifact(root, CORE_N_CONTRACT_JSON_PATH)?;
    for field in [
        "core_m_path",
        "lane_f_path",
        "legacy_rate_card_path",
        "trn_level_2_path",
    ] {
        let path = string_field(&contract, field)?;
        if !root.join(&path).is_file() {
            return Err(format!("CORE-N dependency missing: {path}"));
        }
    }
    let surfaces = contract
        .get("release_surfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-N release surfaces")?;
    let surface_ids = surfaces
        .iter()
        .map(|row| string_field(row, "surface"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let rule = contract.get("release_rule").ok_or("CORE-N release rule")?;
    let rust_interfaces = contract
        .get("rust_interfaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-N Rust interfaces")?;
    let implementation = contract
        .get("implementation_status")
        .ok_or("CORE-N implementation status")?;
    if string_field(&contract, "record_id")? != "core-n-typed-public-release-surface-contract:v1"
        || string_field(&contract, "record_family")?
            != "core_n_typed_public_release_surface_contract"
        || string_field(&contract, "status")? != "core_n_interfaces_implemented_role_review_ready"
        || int_field(&contract, "pulse")? != 404
        || string_field(&contract, "core_wave_id")? != "CORE-N"
        || string_field(&contract, "schema_path")? != CORE_N_CONTRACT_SCHEMA_PATH
        || string_field(&contract, "reader_path")? != CORE_N_CONTRACT_READER_PATH
        || surfaces.len() != 5
        || surface_ids.len() != 5
        || ![
            "cost_note",
            "rate_card",
            "savings_result",
            "integrity_overlay_report",
            "endogenous_effect_report",
        ]
        .iter()
        .all(|surface| surface_ids.contains(*surface))
        || [
            "surface_must_match_profile",
            "outputs_must_be_core_m_admitted",
            "surface_required_output_must_be_present",
            "all_profile_required_gates_must_be_ready",
            "reviewed_non_applicability_cannot_create_output",
            "role_review_required",
            "public_language_review_required",
            "reproducible_release_required",
            "one_surface_cannot_supply_another_surface_result",
        ]
        .iter()
        .any(|field| !bool_field(rule, field).unwrap_or(false))
        || rust_interfaces.len() != 4
        || int_field(implementation, "release_surfaces")? != 5
        || int_field(implementation, "rust_interfaces")? != 4
        || int_field(implementation, "unit_tests_added")? != 5
        || !bool_field(implementation, "core_n_implementation_done")?
        || !bool_field(implementation, "core_n_role_review_may_start")?
    {
        return Err("CORE-N typed public-release contract failed".to_string());
    }
    let core_source = fs::read_to_string(root.join("crates/taxlane-core/src/lib.rs"))
        .map_err(|err| format!("failed to read CORE-N implementation: {err}"))?;
    for symbol in rust_interfaces {
        let symbol = symbol.as_str().ok_or("CORE-N Rust interface name")?;
        if !core_source.contains(symbol) {
            return Err(format!("CORE-N Rust interface missing: {symbol}"));
        }
    }
    validate_blocked_outputs_null(&contract, "CORE-N contract")?;
    validate_claim_boundary(
        &contract,
        "CORE-N contract",
        &[
            "core_n_contract_published",
            "typed_public_release_surfaces_implemented",
            "five_surface_cases_validated",
            "core_n_implementation_done",
            "core_n_role_review_may_start",
        ],
    )?;

    let closure = read_json_artifact(root, CORE_N_CLOSURE_JSON_PATH)?;
    let checks = closure
        .get("closure_checks")
        .and_then(serde_json::Value::as_array)
        .ok_or("CORE-N closure checks")?;
    let decision = closure
        .get("dependency_decision")
        .ok_or("CORE-N dependency decision")?;
    if string_field(&closure, "record_id")? != "core-n-typed-public-release-surface-closure:v1"
        || string_field(&closure, "status")? != "core_n_complete_five_typed_public_release_surfaces"
        || int_field(&closure, "pulse")? != 405
        || string_field(&closure, "contract_path")? != CORE_N_CONTRACT_JSON_PATH
        || string_field(&closure, "role_review_path")? != CORE_N_CONTRACT_REVIEW_PATH
        || checks.len() != 11
        || !bool_field(decision, "core_n_done")?
        || !bool_field(decision, "trn_f_cost_note_may_complete")?
        || !bool_field(decision, "rev_rate_card_requires_matched_base")?
    {
        return Err("CORE-N closure failed".to_string());
    }
    validate_blocked_outputs_null(&closure, "CORE-N closure")?;
    validate_claim_boundary(
        &closure,
        "CORE-N closure",
        &[
            "core_n_closure_published",
            "role_review_complete",
            "core_n_done",
            "trn_f_cost_note_may_complete",
        ],
    )?;
    for (path, phrase) in [
        (
            CORE_N_CONTRACT_SCHEMA_PATH,
            "A cost note cannot be labeled a rate card",
        ),
        (CORE_N_CONTRACT_READER_PATH, "Five surfaces are implemented"),
        (
            CORE_N_CONTRACT_REVIEW_PATH,
            "approve CORE-N as shared public-release infrastructure",
        ),
    ] {
        let prose = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !prose.contains(phrase) {
            return Err(format!("CORE-N prose missing: {phrase}"));
        }
    }
    Ok(())
}

