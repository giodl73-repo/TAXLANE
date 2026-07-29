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

pub(crate) fn check_accountability_readiness_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_readiness_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_READINESS_REPORT_PATH,
        &expected,
        "accountability readiness report",
    )?;
    println!("validated accountability readiness report");
    Ok(())
}

pub(crate) fn check_accountability_action_queue(root: &Path) -> Result<(), String> {
    let expected = build_accountability_action_queue(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_ACTION_QUEUE_PATH,
        &expected,
        "accountability action queue",
    )?;
    println!("validated accountability action queue");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_packet(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_packet(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH,
        &expected,
        "accountability performance demand packet",
    )?;
    println!("validated accountability performance demand packet");
    Ok(())
}

pub(crate) fn check_accountability_work_items(root: &Path) -> Result<(), String> {
    let expected = build_accountability_work_items_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH,
        &expected,
        "accountability work items",
    )?;
    println!("validated accountability work items");
    Ok(())
}

pub(crate) fn check_accountability_claim_guard_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_claim_guard_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH,
        &expected,
        "accountability claim guard report",
    )?;
    println!("validated accountability claim guard report");
    Ok(())
}

pub(crate) fn check_accountability_public_questions(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_questions(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH,
        &expected,
        "accountability public questions",
    )?;
    println!("validated accountability public questions");
    Ok(())
}

pub(crate) fn check_accountability_public_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_BRIEF_PATH,
        &expected,
        "accountability public brief",
    )?;
    println!("validated accountability public brief");
    Ok(())
}

pub(crate) fn check_accountability_public_brief_discovery(root: &Path) -> Result<(), String> {
    let root_readme = fs::read_to_string(root.join(README_PATH))
        .map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    if !root_readme.contains(ACCOUNTABILITY_PUBLIC_BRIEF_PATH) {
        return Err(format!(
            "{README_PATH} must link {ACCOUNTABILITY_PUBLIC_BRIEF_PATH}"
        ));
    }

    let reading_index = fs::read_to_string(root.join(READING_INDEX_PATH))
        .map_err(|err| format!("failed to read {READING_INDEX_PATH}: {err}"))?;
    if !reading_index.contains("accountability-public-brief.md") {
        return Err(format!(
            "{READING_INDEX_PATH} must link accountability-public-brief.md"
        ));
    }

    println!("validated accountability public brief discovery");
    Ok(())
}

pub(crate) fn check_accountability_artifact_map(root: &Path) -> Result<(), String> {
    let expected = build_accountability_artifact_map();
    compare_text(
        root,
        ACCOUNTABILITY_ARTIFACT_MAP_PATH,
        &expected,
        "accountability artifact map",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("artifact-map.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link artifact-map.md".to_string(),
        );
    }

    let artifact_map = fs::read_to_string(root.join(ACCOUNTABILITY_ARTIFACT_MAP_PATH))
        .map_err(|err| format!("failed to read {ACCOUNTABILITY_ARTIFACT_MAP_PATH}: {err}"))?;
    for required in [
        "performance-demand-dashboard.md",
        "performance-demand-claim-gates.json",
        "performance-demand-checklist.jsonl",
        "performance-demand-checklist.schema.md",
        "performance-demand-response-log.md",
        "performance-demand-response-log.jsonl",
        "performance-demand-response-log.schema.md",
        "performance-demand-response-status.json",
        "performance-demand-response-dashboard.md",
        "performance-demand-response-handoff.md",
        "performance-demand-response-intake.md",
        "performance-demand-response-intake.schema.md",
        "performance-demand-response-intake.example.jsonl",
        "performance-demand-response-log.applied-example.jsonl",
        "performance-demand-response-status.applied-example.json",
        "performance-demand-response-dashboard.applied-example.md",
        "performance-demand-response-handoff.applied-example.md",
        "performance-demand-response-applied-example.schema.md",
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.md",
        "performance-demand-response-bundle.applied-example.json",
        "performance-demand-response-bundle.applied-example.schema.md",
    ] {
        if !artifact_map.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_ARTIFACT_MAP_PATH} must route {required}"
            ));
        }
    }

    println!("validated accountability artifact map");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_checklist(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH,
        &expected,
        "accountability performance demand checklist",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand checklist");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH,
        &expected,
        "accountability performance demand checklist JSONL",
    )?;

    let rows: Vec<PerformanceDemandChecklistRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("accountability performance demand checklist JSONL: {err}")
                })
            })
            .collect::<Result<_, _>>()?;
    if rows.is_empty() {
        return Err("accountability performance demand checklist JSONL has no rows".to_string());
    }
    let mut expected_rows = read_accountability_evidence_records(root)?;
    expected_rows.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let expected_rows: Vec<PerformanceDemandChecklistRecord> = expected_rows
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    if rows != expected_rows {
        return Err(
            "accountability performance demand checklist JSONL does not match core records"
                .to_string(),
        );
    }
    for row in rows {
        row.validate()?;
        if row.public_claim_allowed {
            return Err(
                "accountability performance demand checklist JSONL unexpectedly allows a public claim"
                    .to_string(),
            );
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.jsonl"
                .to_string(),
        );
    }
    let schema_filename = ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH
        .rsplit('/')
        .next()
        .unwrap_or(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH);
    if !index.contains(schema_filename) {
        return Err(format!(
            "data/derived/accountability_evidence/README.md must link {schema_filename}"
        ));
    }

    println!("validated accountability performance demand checklist JSONL");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_claim_gates(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_claim_gates(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH,
        &expected,
        "accountability performance demand claim gates",
    )?;

    let parsed_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH),
    )
    .map_err(|err| {
        format!("failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let parsed: serde_json::Value = serde_json::from_str(&parsed_text).map_err(|err| {
        format!("failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let total_rows = parsed
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing total_rows".to_string())?;
    let blocked_rows = parsed
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_blocked".to_string())?;
    let allowed_rows = parsed
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_allowed".to_string())?;
    if total_rows != blocked_rows + allowed_rows {
        return Err(
            "performance demand claim gates total does not match allowed plus blocked".to_string(),
        );
    }
    if allowed_rows != 0 {
        return Err("performance demand claim gates unexpectedly allow a public claim".to_string());
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-claim-gates.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-claim-gates.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand claim gates");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_dashboard(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_dashboard(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH,
        &expected,
        "accountability performance demand dashboard",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-dashboard.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-dashboard.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand dashboard");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH,
        &expected,
        "accountability performance demand brief",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-brief.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-brief.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand brief");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_letter(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_letter(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_LETTER_PATH,
        &expected,
        "accountability performance demand letter",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-letter.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-letter.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand letter");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_rubric(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_rubric(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_RUBRIC_PATH,
        &expected,
        "accountability performance demand response rubric",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-rubric.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-rubric.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response rubric");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_followup(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_followup(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_FOLLOWUP_PATH,
        &expected,
        "accountability performance demand follow-up",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-followup.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-followup.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand follow-up");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_log(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_PATH,
        &expected,
        "accountability performance demand response log",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_log_jsonl(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH,
        &expected,
        "accountability performance demand response log JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseLogRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row)
                    .map_err(|err| format!("response log JSONL: invalid row shape: {err}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err("performance demand response log JSONL has no rows".to_string());
    }
    let mut expected_records: Vec<PerformanceDemandResponseLogRecord> =
        read_accountability_evidence_records(root)?
            .into_iter()
            .map(|record| record.performance_demand_response_log_record())
            .collect();
    expected_records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    if rows != expected_records {
        return Err("response log JSONL rows do not match core-derived records".to_string());
    }
    for row in rows {
        row.validate()?;
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log JSONL");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_log_schema(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_log_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_SCHEMA_PATH,
        &expected,
        "accountability performance demand response log schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log schema");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_status(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_status(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH,
        &expected,
        "accountability performance demand response status",
    )?;

    let parsed_text =
        fs::read_to_string(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH))
            .map_err(|err| {
                format!(
                    "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH}: {err}"
                )
            })?;
    let parsed: PerformanceDemandResponseStatus =
        serde_json::from_str(&parsed_text).map_err(|err| {
            format!(
                "failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_PATH}: {err}"
            )
        })?;
    parsed.validate()?;
    if parsed.total_rows != parsed.not_yet_received {
        return Err("all generated response status rows must be not-yet-received".to_string());
    }
    if parsed.public_claim_allowed != 0 {
        return Err("response status unexpectedly allows a public claim".to_string());
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-status.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-status.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response status");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_dashboard(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_dashboard(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_PATH,
        &expected,
        "accountability performance demand response dashboard",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-dashboard.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-dashboard.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response dashboard");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_handoff(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_handoff(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_PATH,
        &expected,
        "accountability performance demand response handoff",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-handoff.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-handoff.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response handoff");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_intake(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_PATH,
        &expected,
        "accountability performance demand response intake",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_intake_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response intake schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake schema");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_intake_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_intake_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response intake example JSONL",
    )?;

    let intake_rows: Vec<PerformanceDemandResponseIntakeRecord> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_INTAKE_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row)
            .map_err(|err| format!("response intake example JSONL: invalid row shape: {err}"))
    })
    .collect::<Result<Vec<_>, _>>()?;
    if intake_rows.is_empty() {
        return Err("performance demand response intake example JSONL has no rows".to_string());
    }

    let mut log_rows: BTreeMap<String, PerformanceDemandResponseLogRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                let record: PerformanceDemandResponseLogRecord = serde_json::from_value(row)
                    .map_err(|err| format!("response log JSONL: invalid row shape: {err}"))?;
                Ok((record.record_id.clone(), record))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;

    for intake in intake_rows {
        intake.validate()?;
        let log_record = log_rows.remove(&intake.record_id).ok_or_else(|| {
            format!(
                "response intake example row has no matching response log row: {}",
                intake.record_id
            )
        })?;
        let updated = log_record.apply_intake(&intake)?;
        updated.validate()?;
        if updated.public_claim_allowed {
            return Err("response intake example unexpectedly allowed a public claim".to_string());
        }
        if updated.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err("response intake example changed the blocked claim gate".to_string());
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-intake.example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-intake.example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response intake example JSONL");
    Ok(())
}

pub(crate) fn check_external_accountability_claim_intake(root: &Path) -> Result<(), String> {
    let rows: Vec<ExternalAccountabilityClaimIntakeRecord> =
        read_jsonl(root.join(EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row)
                    .map_err(|err| format!("external accountability claim intake: {err}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
    if rows.len() != 5 {
        return Err(format!(
            "external accountability claim intake must contain exactly 5 rows, found {}",
            rows.len()
        ));
    }

    let mut record_ids = BTreeSet::new();
    let mut group_ids = BTreeSet::new();
    let mut publication_source_ids = BTreeSet::new();
    for row in &rows {
        row.validate()?;
        if !record_ids.insert(row.record_id.as_str()) {
            return Err(format!(
                "duplicate external claim record_id: {}",
                row.record_id
            ));
        }
        if !group_ids.insert(row.claim_group_id.as_str()) {
            return Err(format!(
                "duplicate external claim claim_group_id: {}",
                row.claim_group_id
            ));
        }
        for publication in &row.publications {
            if !publication_source_ids.insert(publication.source_id.as_str()) {
                return Err(format!(
                    "external claim publication source ID reused across rows: {}",
                    publication.source_id
                ));
            }
        }
    }
    if record_ids.len() != 5 || group_ids.len() != 5 || publication_source_ids.len() != 13 {
        return Err("external claim intake requires 5 unique records/groups and 13 unique publication source IDs".to_string());
    }

    struct ExpectedPublication<'a> {
        source_id: &'a str,
        source_url: &'a str,
        publisher: &'a str,
        ledger_publisher: &'a str,
        published_date: Option<&'a str>,
        observed_date: &'a str,
        publication_kind: ExternalClaimPublicationKind,
        evidence_relation: ExternalClaimEvidenceRelation,
    }
    struct ExpectedExternalClaim<'a> {
        record_id: &'a str,
        claim_group_id: &'a str,
        claim_type: ExternalClaimType,
        paraphrase: &'a str,
        value: f64,
        unit: &'a str,
        semantic: ExternalClaimAmountSemantic,
        derivation: ExternalClaimAmountDerivation,
        publications: &'a [ExpectedPublication<'a>],
    }
    let expected = [
        ExpectedExternalClaim {
            record_id: "external-claim:nick-shirley:2026-07-10:nyc-care:amount:01",
            claim_group_id: "external-claim-group:nick-shirley:2026-07-10:nyc-care",
            claim_type: ExternalClaimType::AggregateFraudAllegation,
            paraphrase: "Nick Shirley alleges more than $190 million in fraud involving New York City adult day care and personal home care activity.",
            value: 190.0,
            unit: "millions",
            semantic: ExternalClaimAmountSemantic::AllegedFraudExposure,
            derivation: ExternalClaimAmountDerivation::SourceStatedLowerBound,
            publications: &[
                ExpectedPublication {
                    source_id: "SRC-NICK-SHIRLEY-NYC-CARE-YOUTUBE-2026-07-10",
                    source_url: "https://www.youtube.com/watch?v=Ji3KpgOT0zM",
                    publisher: "Nick Shirley",
                    ledger_publisher: "Nick Shirley / YouTube",
                    published_date: Some("2026-07-10"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::OriginalVideo,
                    evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
                },
                ExpectedPublication {
                    source_id: "SRC-MEDIAITE-NYC-CARE-COVERAGE-2026-07",
                    source_url: "https://www.mediaite.com/online/major-red-flags-dr-oz-joins-maga-influencer-nick-shirley-to-confront-alleged-fraudsters/",
                    publisher: "Mediaite",
                    ledger_publisher: "Mediaite",
                    published_date: None,
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::Article,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
                ExpectedPublication {
                    source_id: "SRC-DOJ-NYC-ADULT-DAY-CARE-PLEAS-2026-01-15",
                    source_url: "https://www.justice.gov/opa/pr/two-individuals-plead-guilty-68m-adult-day-care-fraud-scheme",
                    publisher: "U.S. Department of Justice",
                    ledger_publisher: "U.S. Department of Justice",
                    published_date: Some("2026-01-15"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AgencyRelease,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
            ],
        },
        ExpectedExternalClaim {
            record_id: "external-claim:nick-shirley:2026-06-28:national-savings:amount:01",
            claim_group_id: "external-claim-group:nick-shirley:2026-06-28:national-savings",
            claim_type: ExternalClaimType::SavingsAssertion,
            paraphrase: "Nick Shirley claims that his reporting saved the United States more than $250 billion.",
            value: 250.0,
            unit: "billions",
            semantic: ExternalClaimAmountSemantic::SourceStatedSavingsTotal,
            derivation: ExternalClaimAmountDerivation::SourceStatedLowerBound,
            publications: &[
                ExpectedPublication {
                    source_id: "SRC-NICK-SHIRLEY-NATIONAL-SAVINGS-X-2026-06-28",
                    source_url: "https://x.com/nickshirleyy/status/2071317393058455930",
                    publisher: "Nick Shirley",
                    ledger_publisher: "Nick Shirley / X",
                    published_date: Some("2026-06-28"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::SocialPost,
                    evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
                },
                ExpectedPublication {
                    source_id: "SRC-DOJ-NATIONAL-HEALTH-CARE-FRAUD-TAKEDOWN-2026-06-23",
                    source_url: "https://www.justice.gov/opa/pr/national-health-care-fraud-takedown-results-455-defendants-charged-connection-over-65",
                    publisher: "U.S. Department of Justice",
                    ledger_publisher: "U.S. Department of Justice",
                    published_date: Some("2026-06-23"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AgencyRelease,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
            ],
        },
        ExpectedExternalClaim {
            record_id: "external-claim:nick-shirley:2025-12-26:minnesota:amount:01",
            claim_group_id: "external-claim-group:nick-shirley:2025-12-26:minnesota",
            claim_type: ExternalClaimType::AggregateFraudAllegation,
            paraphrase: "Nick Shirley alleges that more than $110 million in fraud was uncovered in Minnesota in one day.",
            value: 110.0,
            unit: "millions",
            semantic: ExternalClaimAmountSemantic::AllegedFraudExposure,
            derivation: ExternalClaimAmountDerivation::SourceStatedLowerBound,
            publications: &[
                ExpectedPublication {
                    source_id: "SRC-NICK-SHIRLEY-MINNESOTA-X-2025-12-26",
                    source_url: "https://x.com/nickshirleyy/status/2004642794862961123",
                    publisher: "Nick Shirley",
                    ledger_publisher: "Nick Shirley / X",
                    published_date: Some("2025-12-26"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::SocialPost,
                    evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
                },
                ExpectedPublication {
                    source_id: "SRC-HHS-OIG-MINNESOTA-CCAP-ATTENDANCE-2025",
                    source_url: "https://oig.hhs.gov/reports/all/2025/minnesota-could-better-ensure-that-childcare-assistance-providers-comply-with-attendance-requirements/",
                    publisher: "U.S. Department of Health and Human Services, Office of Inspector General",
                    ledger_publisher: "U.S. Department of Health and Human Services, Office of Inspector General",
                    published_date: Some("2025"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AuditReport,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
                ExpectedPublication {
                    source_id: "SRC-DOJ-MINNESOTA-HEALTH-CARE-FRAUD-CASE-SUMMARIES-2026",
                    source_url: "https://www.justice.gov/criminal/criminal-fraud/health-care-fraud-unit/2026-minnesota-hcf-case-summaries",
                    publisher: "U.S. Department of Justice",
                    ledger_publisher: "U.S. Department of Justice",
                    published_date: Some("2026"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AgencyRelease,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
            ],
        },
        ExpectedExternalClaim {
            record_id: "external-claim:nick-shirley:2026-03:california-care:amount:01",
            claim_group_id: "external-claim-group:nick-shirley:2026-03:california-care",
            claim_type: ExternalClaimType::AggregateFraudAllegation,
            paraphrase: "Nick Shirley alleges more than $170 million in fraud involving California daycare and hospice activity.",
            value: 170.0,
            unit: "millions",
            semantic: ExternalClaimAmountSemantic::AllegedFraudExposure,
            derivation: ExternalClaimAmountDerivation::SourceStatedLowerBound,
            publications: &[
                ExpectedPublication {
                    source_id: "SRC-FOXLA-SHIRLEY-CALIFORNIA-CARE-2026-03",
                    source_url: "https://www.foxla.com/news/nick-shirley-california-daycare-fraud-dr-oz-hospice",
                    publisher: "FOX 11 Los Angeles",
                    ledger_publisher: "FOX 11 Los Angeles",
                    published_date: None,
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::Article,
                    evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
                },
                ExpectedPublication {
                    source_id: "SRC-CDPH-CALIFORNIA-HOSPICE-LOCATION-REVIEW-2026-04",
                    source_url: "https://www.cdph.ca.gov/Programs/OPA/Pages/NR26-014.aspx",
                    publisher: "California Department of Public Health",
                    ledger_publisher: "California Department of Public Health",
                    published_date: Some("2026-04"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AgencyRelease,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
                ExpectedPublication {
                    source_id: "SRC-CA-DOJ-HEALTH-CARE-FRAUD-CHARGES-267M-2026",
                    source_url: "https://oag.ca.gov/node/621529",
                    publisher: "California Department of Justice",
                    ledger_publisher: "California Department of Justice",
                    published_date: Some("2026"),
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::AgencyRelease,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
            ],
        },
        ExpectedExternalClaim {
            record_id: "external-claim:nick-shirley:2026:house-testimony-quality-learing-center:amount:01",
            claim_group_id: "external-claim-group:nick-shirley:2026:house-testimony-quality-learing-center",
            claim_type: ExternalClaimType::PaymentOrBilling,
            paraphrase: "In House-hosted written testimony, Nick Shirley claims that downtown Minneapolis's Quality Learing Center received $1.9 million in Child Care Assistance Program funding for calendar year 2025.",
            value: 1.9,
            unit: "millions",
            semantic: ExternalClaimAmountSemantic::PaidAmount,
            derivation: ExternalClaimAmountDerivation::SourceStatedExact,
            publications: &[
                ExpectedPublication {
                    source_id: "SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026",
                    source_url: "https://judiciary.house.gov/sites/evo-subsites/republicans-judiciary.house.gov/files/evo-media-document/shirley-testimony.pdf",
                    publisher: "U.S. House Committee on the Judiciary",
                    ledger_publisher: "U.S. House Committee on the Judiciary",
                    published_date: None,
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::WrittenTestimony,
                    evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
                },
                ExpectedPublication {
                    source_id: "SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22",
                    source_url: "https://www.house.mn.gov/comm/docs/oyZeI7aBIUu8IIo8wDk6qw.pdf",
                    publisher: "Minnesota Department of Children, Youth, and Families",
                    ledger_publisher: "Minnesota Department of Children, Youth, and Families",
                    published_date: None,
                    observed_date: "2026-07-14",
                    publication_kind: ExternalClaimPublicationKind::Dataset,
                    evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
                },
            ],
        },
    ];

    let source_ledger = fs::read_to_string(root.join("docs/sources/source-version-ledger.md"))
        .map_err(|err| format!("failed to read source version ledger: {err}"))?;
    for expected_row in expected {
        let row = rows
            .iter()
            .find(|row| row.record_id == expected_row.record_id)
            .ok_or_else(|| {
                format!(
                    "missing expected external claim row: {}",
                    expected_row.record_id
                )
            })?;
        if row.claim_group_id != expected_row.claim_group_id
            || row.claim_atom.claim_type != expected_row.claim_type
            || row.claim_atom.neutral_paraphrase != expected_row.paraphrase
            || row.amount_assertion.value != expected_row.value
            || row.amount_assertion.unit != expected_row.unit
            || row.amount_assertion.amount_semantic != expected_row.semantic
            || row.amount_assertion.derivation != expected_row.derivation
            || row.publications.len() != expected_row.publications.len()
        {
            return Err(format!(
                "external claim expected configuration mismatch: {}",
                expected_row.record_id
            ));
        }
        for (publication, expected_publication) in
            row.publications.iter().zip(expected_row.publications)
        {
            if publication.source_id != expected_publication.source_id
                || publication.source_url != expected_publication.source_url
                || publication.publisher != expected_publication.publisher
                || publication.published_date.as_deref() != expected_publication.published_date
                || publication.observed_date != expected_publication.observed_date
                || publication.publication_kind != expected_publication.publication_kind
                || publication.evidence_relation != expected_publication.evidence_relation
            {
                return Err(format!(
                    "external claim publication configuration mismatch: {} / {}",
                    expected_row.record_id, expected_publication.source_id
                ));
            }
            let ledger_marker = format!("| `{}` |", expected_publication.source_id);
            let ledger_line = source_ledger
                .lines()
                .find(|line| line.starts_with(&ledger_marker));
            let ledger_cells =
                ledger_line.map(|line| line.split('|').map(str::trim).collect::<Vec<_>>());
            let ledger_identity_matches = ledger_cells.as_ref().is_some_and(|cells| {
                cells.get(2) == Some(&expected_publication.ledger_publisher)
                    && cells.get(3).map(|cell| cell.as_ref())
                        == Some(format!("<{}>", expected_publication.source_url).as_str())
                    && cells.get(4).is_some_and(|observed| {
                        *observed == expected_publication.observed_date
                            || observed
                                .starts_with(&format!("{};", expected_publication.observed_date))
                    })
            });
            if !ledger_identity_matches {
                return Err(format!(
                    "external claim source-ledger identity mismatch: {}",
                    expected_publication.source_id
                ));
            }
        }
    }

    let house_record = rows
        .iter()
        .find(|row| {
            row.record_id
                == "external-claim:nick-shirley:2026:house-testimony-quality-learing-center:amount:01"
        })
        .ok_or("missing custody-backed House testimony claim atom")?;
    let house_publication = house_record
        .publications
        .first()
        .ok_or("House testimony claim atom has no publication")?;
    if house_record.claim_status != ExternalClaimStatus::AttributedClaimSupported
        || house_record.review_status != ExternalClaimReviewStatus::SourceReviewed
        || !house_record.claim_atom.exact_text_verified
        || house_record.official_response.respondent.as_deref() != Some("Quality Learing Center")
        || house_publication.custody_status != ExternalClaimCustodyStatus::OfficialCopyCaptured
        || house_publication.custody_path.as_deref() != Some(HOUSE_SHIRLEY_TESTIMONY_RAW_PATH)
        || house_publication.sha256.as_deref() != Some(HOUSE_SHIRLEY_TESTIMONY_SHA256)
        || house_record.claim_atom.object != "Child Care Assistance Program funding"
        || house_record.claim_atom.coverage_period != "calendar_year_2025"
        || house_record.amount_assertion.period != "calendar_year_2025"
        || house_record.amount_assertion.overlap_group
            != "house-testimony-quality-learing-center-ccap-cy2025"
        || house_record.comparison_basis
            != "Checksum-verified House-hosted testimony supports only that Shirley made the attributed CCAP payment assertion for calendar year 2025. A separately owned DCYF provider table identifies Quality Learning Center Inc, license 1087038 at 1411 Nicollet Ave, and reports full-CY2025 CCAP payments of $2,150,964. Shirley's source-stated CY2025 $1.9 million does not equal that annual value, but the testimony follows a December 16 visit narrative and does not disclose whether its amount is year-to-date, its data cutoff, or its calculation basis. The table therefore supplies identity and payment context only; it does not corroborate or counter the exact amount, establish impropriety, or record a recipient response. The same table records the license status as Closed with a license inactive date of 1/6/2026, but does not state who initiated closure, the authority or reason, or any causal relationship among closure, CCAP payments, complaints, violations, the assessed and repaid overpayment, or the testimony."
        || !house_record
            .due_process_caveat
            .contains("December 16/year-to-date cutoff")
        || !house_record
            .due_process_caveat
            .contains("calculation basis remain undisclosed")
        || !house_record
            .due_process_caveat
            .contains("exact amount is correct or incorrect")
        || !house_record
            .due_process_caveat
            .contains("that closure is related to the testimony or payments")
    {
        return Err("House testimony claim atom custody/status configuration mismatch".to_string());
    }

    let house_raw_path = root.join(HOUSE_SHIRLEY_TESTIMONY_RAW_PATH);
    let house_raw_bytes = fs::metadata(&house_raw_path)
        .map_err(|err| format!("failed to inspect {HOUSE_SHIRLEY_TESTIMONY_RAW_PATH}: {err}"))?
        .len();
    let house_raw_sha256 = sha256_file(&house_raw_path)?;
    if house_raw_bytes != HOUSE_SHIRLEY_TESTIMONY_BYTES
        || house_raw_sha256 != HOUSE_SHIRLEY_TESTIMONY_SHA256
    {
        return Err("House testimony raw PDF bytes or SHA-256 mismatch".to_string());
    }
    let house_metadata = fs::read_to_string(root.join(HOUSE_SHIRLEY_TESTIMONY_METADATA_PATH))
        .map_err(|err| format!("failed to read {HOUSE_SHIRLEY_TESTIMONY_METADATA_PATH}: {err}"))?;
    for required in [
        "`SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026`",
        "U.S. House Committee on the Judiciary",
        HOUSE_SHIRLEY_TESTIMONY_RAW_PATH,
        "`60433`",
        "`E90266A876DCB6882593A1A63DF70646270C7F9A037F6BA49D20F9E310C040C5`",
        "1 PDF file page.",
        "`secret_scan`",
        "No credential, token, private-key, password, or authorization-header patterns found",
        "All locations below are PDF file page 1.",
        "does not support the separate more-than-$110-million",
    ] {
        if !house_metadata.contains(required) {
            return Err(format!(
                "House testimony metadata missing custody token: {required}"
            ));
        }
    }
    let house_ledger_row = source_ledger
        .lines()
        .find(|line| line.starts_with("| `SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026` |"))
        .ok_or("House testimony source ledger row missing")?;
    for required in [
        "official PDF captured and checksum-verified",
        "60,433 bytes",
        "E90266A876DCB6882593A1A63DF70646270C7F9A037F6BA49D20F9E310C040C5",
        "Use PDF page 1 only",
        "do not establish truth",
    ] {
        if !house_ledger_row.contains(required) {
            return Err(format!(
                "House testimony source ledger row missing custody token: {required}"
            ));
        }
    }

    let dcyf_publication = house_record
        .publications
        .get(1)
        .ok_or("Quality Learing Center atom missing DCYF context publication")?;
    if dcyf_publication.custody_status != ExternalClaimCustodyStatus::OfficialCopyCaptured
        || dcyf_publication.custody_path.as_deref() != Some(MN_DCYF_CCAP_PROVIDER_RAW_PATH)
        || dcyf_publication.sha256.as_deref() != Some(MN_DCYF_CCAP_PROVIDER_SHA256)
        || dcyf_publication.evidence_relation != ExternalClaimEvidenceRelation::SuppliesContext
        || !house_record.corroborating_source_ids.is_empty()
        || !house_record.counterevidence_source_ids.is_empty()
        || !house_record
            .official_response
            .response_source_ids
            .is_empty()
        || house_record.official_response.requested_at.is_some()
        || house_record.legal_or_administrative_status
            != ExternalClaimLegalOrAdministrativeStatus::NoneEstablished
        || !house_record.claim_gates.all_false()
    {
        return Err(
            "DCYF provider table must remain captured context with no claim-status transition"
                .to_string(),
        );
    }
    let dcyf_raw_path = root.join(MN_DCYF_CCAP_PROVIDER_RAW_PATH);
    let dcyf_raw_bytes = fs::metadata(&dcyf_raw_path)
        .map_err(|err| format!("failed to inspect {MN_DCYF_CCAP_PROVIDER_RAW_PATH}: {err}"))?
        .len();
    let dcyf_raw_sha256 = sha256_file(&dcyf_raw_path)?;
    if dcyf_raw_bytes != MN_DCYF_CCAP_PROVIDER_BYTES
        || dcyf_raw_sha256 != MN_DCYF_CCAP_PROVIDER_SHA256
    {
        return Err("DCYF provider table raw PDF bytes or SHA-256 mismatch".to_string());
    }
    let dcyf_metadata = fs::read_to_string(root.join(MN_DCYF_CCAP_PROVIDER_METADATA_PATH))
        .map_err(|err| format!("failed to read {MN_DCYF_CCAP_PROVIDER_METADATA_PATH}: {err}"))?;
    for required in [
        "`SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22`",
        "Minnesota Department of Children, Youth, and Families",
        "<https://www.house.mn.gov/comm/docs/oyZeI7aBIUu8IIo8wDk6qw.pdf>",
        MN_DCYF_CCAP_PROVIDER_RAW_PATH,
        "`1277757`",
        "`E7068E1198D8DCE851907B60FC4A2A16FEDD5DE7A1D41AFCD2B02DCAABF3DEC1`",
        "7 PDF file pages.",
        "`secret_scan`",
        "`published_date` | Not established by the captured PDF.",
        "production/document date, not an independently verified House posting or publication date",
        "support entity identity and the reported",
        "annual payment totals",
        "cannot establish the testimony's data cutoff",
        "calculation basis",
        "`License Status` as `Closed`",
        "`License Inactive Date` of 1/6/2026",
        "does not state who",
        "initiated closure, the authority or reason",
        "Treat the closed status and inactive date as license context only",
    ] {
        if !dcyf_metadata.contains(required) {
            return Err(format!(
                "DCYF provider table metadata missing token: {required}"
            ));
        }
    }
    let dcyf_ledger_row = source_ledger
        .lines()
        .find(|line| line.starts_with("| `SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22` |"))
        .ok_or("DCYF provider table source ledger row missing")?;
    for required in [
        "official PDF captured and checksum-verified",
        "1,277,757 bytes",
        "E7068E1198D8DCE851907B60FC4A2A16FEDD5DE7A1D41AFCD2B02DCAABF3DEC1",
        "identity, annual-payment, and license-status context only",
        "not corroboration, counterevidence",
        "records the license as Closed with a 1/6/2026 inactive date",
        "does not state the closure initiator, authority, reason",
    ] {
        if !dcyf_ledger_row.contains(required) {
            return Err(format!(
                "DCYF provider table source ledger row missing token: {required}"
            ));
        }
    }

    for (path, required_tokens) in [
        (
            "data/derived/accountability_evidence/README.md",
            vec![
                "external-accountability-claim-intake.v1.draft.jsonl",
                "external-accountability-claim-intake.schema.md",
                "external-accountability-claim-intake.md",
            ],
        ),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-53-external-accountability-claim-intake.md"],
        ),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-53-external-accountability-claim-intake.md",
            vec!["WP-TAX-071", "EVID-TAX-071", "VAL-TAX-071"],
        ),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/pulses/pulse-54-house-testimony-quality-learing-center-claim-atom.md",
            vec!["$1.9 million", "WP-TAX-072", "EVID-TAX-072", "VAL-TAX-072"],
        ),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-071"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-071", "EVID-TAX-071"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-071"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-071", "EVID-TAX-071"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-071"]),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-072"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-072", "EVID-TAX-072"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-072"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-072", "EVID-TAX-072"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-072"]),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-54-house-testimony-quality-learing-center-claim-atom.md"],
        ),
        (
            HOUSE_SHIRLEY_TESTIMONY_REVIEW_PATH,
            vec!["$1.9 million", "All twelve claim gates remain false"],
        ),
        (
            PULSE_55_QUALITY_LEARNING_CENTER_OFFICIAL_CONTEXT_PATH,
            vec!["WP-TAX-073", "EVID-TAX-073", "VAL-TAX-073"],
        ),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-073"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-073", "EVID-TAX-073"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-073"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-073", "EVID-TAX-073"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-073"]),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-55-quality-learning-center-official-context.md"],
        ),
        (
            MN_DCYF_CCAP_PROVIDER_REVIEW_PATH,
            vec![
                "All twelve claim gates remain false",
                "`supplies_context`",
                "Internal quarantine use only",
            ],
        ),
        (
            PULSE_56_QUALITY_LEARNING_CENTER_LICENSE_CLOSURE_CONTEXT_PATH,
            vec!["WP-TAX-074", "EVID-TAX-074", "VAL-TAX-074"],
        ),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-074"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-074", "EVID-TAX-074"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-074"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-074", "EVID-TAX-074"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-074"]),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-56-quality-learning-center-license-closure-context.md"],
        ),
        (
            MN_DCYF_CCAP_PROVIDER_CLOSURE_REVIEW_PATH,
            vec![
                "All twelve claim",
                "gates remain false",
                "`none_established`",
                "Internal quarantine use only",
            ],
        ),
        (
            PULSE_57_QUALITY_LEARNING_CENTER_CY2025_PERIOD_CORRECTION_PATH,
            vec!["WP-TAX-075", "EVID-TAX-075", "VAL-TAX-075"],
        ),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-075"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-075", "EVID-TAX-075"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-075"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-075", "EVID-TAX-075"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-075"]),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-57-quality-learning-center-cy2025-period-correction.md"],
        ),
        (
            QUALITY_LEARNING_CENTER_CY2025_PERIOD_CORRECTION_REVIEW_PATH,
            vec![
                "calendar_year_2025",
                "`published_date` null",
                "unresolved December",
                "16/year-to-date cutoff",
                "calculation basis",
                "All twelve claim",
                "gates remain false",
                "Internal quarantine use only",
            ],
        ),
    ] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        for required in required_tokens {
            if !text.contains(required) {
                return Err(format!(
                    "external claim integration {path} missing {required}"
                ));
            }
        }
    }

    let trace = fs::read_to_string(root.join("docs/vtrace/TRACE.md"))
        .map_err(|err| format!("failed to read docs/vtrace/TRACE.md: {err}"))?;
    for required in [
        "| REQ-TAX-004 | SPEC-TAX-004 | WP-TAX-073 | EVID-TAX-073 | current |",
        "| REQ-TAX-006 | SPEC-TAX-006 | WP-TAX-073 | EVID-TAX-073 | current |",
    ] {
        if !trace.lines().any(|line| line == required) {
            return Err(format!("Pulse 55 trace binding missing: {required}"));
        }
    }
    let verification = fs::read_to_string(root.join("docs/vtrace/VERIFICATION.md"))
        .map_err(|err| format!("failed to read docs/vtrace/VERIFICATION.md: {err}"))?;
    for requirement in ["REQ-TAX-004", "REQ-TAX-006"] {
        if !verification.lines().any(|line| {
            line.starts_with(&format!("| {requirement} |")) && line.contains("EVID-TAX-073")
        }) {
            return Err(format!(
                "Pulse 55 verification binding missing: {requirement} / EVID-TAX-073"
            ));
        }
    }

    for required in [
        "| REQ-TAX-004 | SPEC-TAX-004 | WP-TAX-074 | EVID-TAX-074 | current |",
        "| REQ-TAX-006 | SPEC-TAX-006 | WP-TAX-074 | EVID-TAX-074 | current |",
    ] {
        if !trace.lines().any(|line| line == required) {
            return Err(format!("Pulse 56 trace binding missing: {required}"));
        }
    }
    for requirement in ["REQ-TAX-004", "REQ-TAX-006"] {
        if !verification.lines().any(|line| {
            line.starts_with(&format!("| {requirement} |")) && line.contains("EVID-TAX-074")
        }) {
            return Err(format!(
                "Pulse 56 verification binding missing: {requirement} / EVID-TAX-074"
            ));
        }
    }

    for required in [
        "| REQ-TAX-004 | SPEC-TAX-004 | WP-TAX-075 | EVID-TAX-075 | current |",
        "| REQ-TAX-006 | SPEC-TAX-006 | WP-TAX-075 | EVID-TAX-075 | current |",
    ] {
        if !trace.lines().any(|line| line == required) {
            return Err(format!("Pulse 57 trace binding missing: {required}"));
        }
    }
    for requirement in ["REQ-TAX-004", "REQ-TAX-006"] {
        if !verification.lines().any(|line| {
            line.starts_with(&format!("| {requirement} |"))
                && line.ends_with("| current | EVID-TAX-075 |")
        }) {
            return Err(format!(
                "Pulse 57 verification binding/status missing: {requirement} / EVID-TAX-075"
            ));
        }
    }
    for (path, id, terminal) in [
        ("docs/vtrace/WORK_PACKAGES.md", "WP-TAX-075", "| complete |"),
        ("docs/vtrace/VALIDATION.md", "VAL-TAX-075", "| current |"),
        ("docs/vtrace/EVIDENCE.md", "EVID-TAX-075", "| passed |"),
    ] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !text
            .lines()
            .any(|line| line.starts_with(&format!("| {id} |")) && line.ends_with(terminal))
        {
            return Err(format!("Pulse 57 VTRACE status missing: {id} / {terminal}"));
        }
    }

    let internal_intake_filenames = [
        "external-accountability-claim-intake.v1.draft.jsonl",
        "external-accountability-claim-intake.schema.md",
        "external-accountability-claim-intake.md",
    ];
    for path in ["README.md", "docs/reading/README.md"] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        for filename in internal_intake_filenames {
            if text.contains(filename) {
                return Err(format!(
                    "internal external claim intake must not be routed from {path}: {filename}"
                ));
            }
        }
    }

    println!("validated external accountability claim intake");
    Ok(())
}

pub(crate) fn check_mn_ccap_cy2025_request_specification(root: &Path) -> Result<(), String> {
    let spec_text = fs::read_to_string(root.join(MN_CCAP_CY2025_REQUEST_SPEC_PATH))
        .map_err(|err| format!("failed to read {MN_CCAP_CY2025_REQUEST_SPEC_PATH}: {err}"))?;
    let spec: serde_json::Value = serde_json::from_str(&spec_text)
        .map_err(|err| format!("failed to parse {MN_CCAP_CY2025_REQUEST_SPEC_PATH}: {err}"))?;
    if string_field(&spec, "record_id")?
        != "external-claim-existing-records-request-specification:mn-dcyf:quality-learning-center:license-1087038:cy2025"
        || string_field(&spec, "record_family")?
            != "minnesota_ccap_provider_payment_existing_records_request_specification"
        || string_field(&spec, "status")?
            != "draft_not_submitted_existing_records_only_privacy_aware_request_specification"
        || string_field(&spec, "submission_status")?
            != "draft_not_submitted_owner_authorization_required"
        || string_field(&spec, "schema_version")? != "v1"
        || string_field(&spec, "upstream_claim_record_id")?
            != "external-claim:nick-shirley:2026:house-testimony-quality-learing-center:amount:01"
        || string_field(&spec, "period")? != "calendar_year_2025"
    {
        return Err("Minnesota CCAP request identity/submission boundary failed".to_string());
    }
    let provider = spec
        .get("provider")
        .ok_or("Minnesota CCAP request provider")?;
    if string_field(provider, "official_name")? != "Quality Learning Center Inc"
        || string_field(provider, "license_number")? != "1087038"
        || !string_field(provider, "site_address")?.contains("1411 Nicollet Ave")
    {
        return Err("Minnesota CCAP request provider identity failed".to_string());
    }
    let source_ids = spec
        .get("source_ids")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request source IDs")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<BTreeSet<_>>();
    if source_ids
        != BTreeSet::from([
            MN_DCYF_DATA_REQUESTS_SOURCE_ID,
            MN_DCYF_PUBLIC_DATA_GUIDE_SOURCE_ID,
            MN_STAT_13_03_SOURCE_ID,
            MN_STAT_142E_02_SOURCE_ID,
        ])
    {
        return Err("Minnesota CCAP request source set failed".to_string());
    }
    let source_references = spec
        .get("source_references")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request source references")?;
    let expected_references = [
        (
            MN_DCYF_DATA_REQUESTS_SOURCE_ID,
            "Minnesota Department of Children, Youth, and Families",
            "https://dcyf.mn.gov/about-us/data-requests",
            "submission route",
        ),
        (
            MN_DCYF_PUBLIC_DATA_GUIDE_SOURCE_ID,
            "Minnesota Department of Children, Youth, and Families",
            "https://dcyf.mn.gov/sites/default/files/2024-06/DCYF%20Guide%20for%20Requesting%20Public%20Data.pdf",
            "response boundaries",
        ),
        (
            MN_STAT_13_03_SOURCE_ID,
            "Minnesota Office of the Revisor of Statutes",
            "https://www.revisor.mn.gov/statutes/cite/13.03",
            "electronic form",
        ),
        (
            MN_STAT_142E_02_SOURCE_ID,
            "Minnesota Office of the Revisor of Statutes",
            "https://www.revisor.mn.gov/statutes/cite/142E.02",
            "without asserting",
        ),
    ];
    if source_references.len() != expected_references.len() {
        return Err("Minnesota CCAP request requires four source references".to_string());
    }
    let mut referenced_ids = BTreeSet::new();
    for (source_id, publisher, url, use_token) in expected_references {
        let reference = source_references
            .iter()
            .find(|value| {
                value.get("source_id").and_then(serde_json::Value::as_str) == Some(source_id)
            })
            .ok_or_else(|| {
                format!("Minnesota CCAP request source reference missing: {source_id}")
            })?;
        if !referenced_ids.insert(source_id)
            || string_field(reference, "publisher")? != publisher
            || string_field(reference, "source_url")? != url
            || !string_field(reference, "use")?.contains(use_token)
        {
            return Err(format!(
                "Minnesota CCAP request source reference drift: {source_id}"
            ));
        }
    }
    let gap = spec
        .get("evidence_gap")
        .ok_or("Minnesota CCAP request evidence gap")?;
    if int_field(gap, "source_stated_amount_usd")? != 1_900_000
        || int_field(gap, "official_full_calendar_year_amount_usd")? != 2_150_964
        || string_field(gap, "testimony_visit_narrative_date")? != "2025-12-16"
        || !string_field(gap, "comparison_result")?
            .contains("neither_corroboration_nor_counterevidence")
    {
        return Err("Minnesota CCAP request evidence-gap boundary failed".to_string());
    }
    let route = spec
        .get("filing_route")
        .ok_or("Minnesota CCAP request filing route")?;
    if string_field(route, "request_law")? != "Minnesota Government Data Practices Act"
        || string_field(route, "email")? != "dcyf.datarequest@state.mn.us"
        || !string_field(route, "mailing_address")?.contains("444 Lafayette Rd")
        || route
            .get("written_request_required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || route
            .get("specific_form_required")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || route
            .get("one_channel_only")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || !route
            .get("route_selected")
            .is_some_and(serde_json::Value::is_null)
    {
        return Err("Minnesota CCAP request filing-route gate failed".to_string());
    }
    let scope = spec
        .get("existing_records_only_scope")
        .ok_or("Minnesota CCAP request existing-records scope")?;
    if string_field(scope, "date_start")? != "2025-01-01"
        || string_field(scope, "date_end")? != "2025-12-31"
    {
        return Err("Minnesota CCAP request period scope failed".to_string());
    }
    for field in [
        "records_only_not_questions",
        "no_new_record_creation",
        "no_new_query_or_custom_export_required",
        "no_new_reconciliation_or_calculation",
        "no_research_or_narrative_answer_required",
        "no_request_to_calculate_1900000",
        "component_records_accepted_if_no_single_reconciliation_exists",
    ] {
        if scope.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!("Minnesota CCAP request scope gate failed: {field}"));
        }
    }
    let record_types = scope
        .get("requested_existing_record_types")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request record types")?;
    let requested_fields = scope
        .get("requested_existing_fields")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request fields")?;
    if record_types.len() != 5
        || requested_fields.len() != 8
        || !record_types.iter().any(|value| {
            value
                .as_str()
                .is_some_and(|text| text.contains("full-CY2025 provider total of $2,150,964"))
        })
        || !requested_fields.iter().any(|value| {
            value
                .as_str()
                .is_some_and(|text| text.contains("reversal, void, cancellation, or reissue"))
        })
    {
        return Err("Minnesota CCAP request record/field scope failed".to_string());
    }
    let privacy = spec
        .get("privacy_and_classification_boundary")
        .ok_or("Minnesota CCAP request privacy boundary")?;
    for field in [
        "provider_level_aggregate_accepted",
        "redacted_or_deidentified_transaction_rows_accepted",
        "stable_non_person_keys_only_if_already_maintained",
        "lawful_redaction_accepted",
    ] {
        if privacy.get(field).and_then(serde_json::Value::as_bool) != Some(true) {
            return Err(format!(
                "Minnesota CCAP request privacy gate failed: {field}"
            ));
        }
    }
    for field in ["classification_prediction_made", "release_prediction_made"] {
        if privacy.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Minnesota CCAP request privacy prediction failed: {field}"
            ));
        }
    }
    let exclusions = privacy
        .get("exclude")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request privacy exclusions")?;
    let exclusion_text = exclusions
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    if exclusions.len() < 6
        || !exclusion_text.contains("child, parent, guardian, caregiver, or recipient")
        || !exclusion_text.contains("Social Security")
        || !exclusion_text.contains("attendance")
        || !exclusion_text.contains("free text")
        || !exclusion_text.contains("bank account")
        || !exclusion_text.contains("staff personal information")
    {
        return Err("Minnesota CCAP request privacy exclusions incomplete".to_string());
    }
    let cost = spec
        .get("format_and_cost")
        .ok_or("Minnesota CCAP request format/cost")?;
    if string_field(cost, "access_requested")? != "electronic copies"
        || !cost
            .get("copy_fee_cap_usd")
            .is_some_and(serde_json::Value::is_null)
        || string_field(cost, "copy_fee_cap_status")? != "required_missing_owner_approval"
        || cost
            .get("advance_notice_before_costs_exceed_cap_required")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || cost
            .get("fee_commitment_executed")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
    {
        return Err("Minnesota CCAP request format/cost gate failed".to_string());
    }
    let preflight = spec
        .get("preflight")
        .ok_or("Minnesota CCAP request preflight")?;
    for field in [
        "owner_authorization",
        "requester_name",
        "requester_contact_method",
        "request_date",
        "single_submission_channel",
        "copy_fee_cap_usd",
        "final_scope_review",
        "privacy_review",
    ] {
        if string_field(preflight, field)? != "required_missing" {
            return Err(format!(
                "Minnesota CCAP request preflight must remain missing: {field}"
            ));
        }
    }
    if preflight
        .get("all_preflight_gates_passed")
        .and_then(serde_json::Value::as_bool)
        != Some(false)
    {
        return Err("Minnesota CCAP request preflight must remain blocked".to_string());
    }
    if string_field(preflight, "public_duplicate_check")? != "required_before_submission" {
        return Err("Minnesota CCAP request public-duplicate preflight missing".to_string());
    }
    let state = spec
        .get("internal_state")
        .ok_or("Minnesota CCAP request internal state")?;
    if string_field(state, "template_path")? != MN_CCAP_CY2025_REQUEST_TEMPLATE_PATH
        || state
            .get("draft_created")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Minnesota CCAP request draft state failed".to_string());
    }
    for field in [
        "owner_authorization_obtained",
        "request_submitted",
        "external_message_sent",
        "portal_or_email_opened_for_submission",
        "fee_commitment_made",
        "outbound_state_changed",
    ] {
        if state.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Minnesota CCAP outbound state must remain false: {field}"
            ));
        }
    }
    let guardrails = spec
        .get("guardrails")
        .and_then(serde_json::Value::as_array)
        .ok_or("Minnesota CCAP request guardrails")?
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    for token in [
        "internal draft, not a submitted request",
        "targets existing records only",
        "not evidence that responsive records exist",
        "no-records response would establish only",
        "administrative event, not evidence",
        "separate evidence adoption",
        "No fraud, waste, debt",
    ] {
        if !guardrails.contains(token) {
            return Err(format!("Minnesota CCAP request guardrail missing: {token}"));
        }
    }
    let preservation = spec
        .get("claim_state_preservation")
        .ok_or("Minnesota CCAP request claim-state preservation")?;
    if string_field(preservation, "official_response_request_status")? != "not_recorded"
        || string_field(preservation, "claim_status")? != "attributed_claim_supported"
        || string_field(preservation, "review_status")? != "source-reviewed"
        || string_field(preservation, "legal_or_administrative_status")? != "none_established"
        || string_field(preservation, "summability")? != "not_summable"
        || preservation
            .get("all_claim_gates_false")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("Minnesota CCAP request claim-state preservation failed".to_string());
    }
    for field in [
        "response_source_ids",
        "corroborating_source_ids",
        "counterevidence_source_ids",
    ] {
        if preservation
            .get(field)
            .and_then(serde_json::Value::as_array)
            .is_none_or(|items| !items.is_empty())
        {
            return Err(format!("Minnesota CCAP request must keep {field} empty"));
        }
    }
    let impact = spec
        .get("evidence_impact")
        .ok_or("Minnesota CCAP request evidence impact")?;
    for field in [
        "responsive_records_received",
        "new_evidence_adopted",
        "claim_status_changed",
        "response_state_changed",
        "field_or_component_closed",
    ] {
        if impact.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!(
                "Minnesota CCAP request evidence impact must remain false: {field}"
            ));
        }
    }
    for field in [
        "public_claim_allowed",
        "underlying_factual_claim_allowed",
        "misconduct_signal_claim_allowed",
        "official_finding_claim_allowed",
        "performance_claim_allowed",
        "fraud_claim_allowed",
        "waste_claim_allowed",
        "debt_claim_allowed",
        "collectibility_claim_allowed",
        "recovery_claim_allowed",
        "prevention_claim_allowed",
        "savings_estimate_allowed",
    ] {
        if spec.get(field).and_then(serde_json::Value::as_bool) != Some(false) {
            return Err(format!("Minnesota CCAP request must keep {field} false"));
        }
    }
    for (raw_path, bytes, checksum) in [
        (
            MN_DCYF_DATA_REQUESTS_RAW_PATH,
            MN_DCYF_DATA_REQUESTS_BYTES,
            MN_DCYF_DATA_REQUESTS_SHA256,
        ),
        (
            MN_DCYF_PUBLIC_DATA_GUIDE_RAW_PATH,
            MN_DCYF_PUBLIC_DATA_GUIDE_BYTES,
            MN_DCYF_PUBLIC_DATA_GUIDE_SHA256,
        ),
        (
            MN_STAT_13_03_RAW_PATH,
            MN_STAT_13_03_BYTES,
            MN_STAT_13_03_SHA256,
        ),
        (
            MN_STAT_142E_02_RAW_PATH,
            MN_STAT_142E_02_BYTES,
            MN_STAT_142E_02_SHA256,
        ),
    ] {
        let raw = root.join(raw_path);
        if fs::metadata(&raw)
            .map_err(|err| format!("failed to inspect {raw_path}: {err}"))?
            .len()
            != bytes
            || sha256_file(&raw)? != checksum
        {
            return Err(format!(
                "Minnesota CCAP request source custody failed: {raw_path}"
            ));
        }
    }
    for (metadata_path, source_id, raw_path, bytes, checksum, required) in [
        (
            MN_DCYF_DATA_REQUESTS_METADATA_PATH,
            MN_DCYF_DATA_REQUESTS_SOURCE_ID,
            MN_DCYF_DATA_REQUESTS_RAW_PATH,
            MN_DCYF_DATA_REQUESTS_BYTES,
            MN_DCYF_DATA_REQUESTS_SHA256,
            "dcyf.datarequest@state.mn.us",
        ),
        (
            MN_DCYF_PUBLIC_DATA_GUIDE_METADATA_PATH,
            MN_DCYF_PUBLIC_DATA_GUIDE_SOURCE_ID,
            MN_DCYF_PUBLIC_DATA_GUIDE_RAW_PATH,
            MN_DCYF_PUBLIC_DATA_GUIDE_BYTES,
            MN_DCYF_PUBLIC_DATA_GUIDE_SHA256,
            "three general",
        ),
        (
            MN_STAT_13_03_METADATA_PATH,
            MN_STAT_13_03_SOURCE_ID,
            MN_STAT_13_03_RAW_PATH,
            MN_STAT_13_03_BYTES,
            MN_STAT_13_03_SHA256,
            "electronic form",
        ),
        (
            MN_STAT_142E_02_METADATA_PATH,
            MN_STAT_142E_02_SOURCE_ID,
            MN_STAT_142E_02_RAW_PATH,
            MN_STAT_142E_02_BYTES,
            MN_STAT_142E_02_SHA256,
            "specific assistance recipient",
        ),
    ] {
        let metadata = fs::read_to_string(root.join(metadata_path))
            .map_err(|err| format!("failed to read {metadata_path}: {err}"))?;
        for token in [
            format!("| `source_id` | `{source_id}` |"),
            format!("| `raw_path` | `{raw_path}` |"),
            format!("| `bytes` | `{bytes}` |"),
            format!(
                "| `checksum_sha256` | `{}` |",
                checksum.to_ascii_uppercase()
            ),
            "`source-reviewed`".to_string(),
            "No credential, token, private-key, password, or authorization-header patterns found"
                .to_string(),
            required.to_string(),
        ] {
            if !metadata.contains(&token) {
                return Err(format!(
                    "Minnesota CCAP request metadata {metadata_path} missing {token}"
                ));
            }
        }
    }
    let ledger = fs::read_to_string(root.join("docs/sources/source-version-ledger.md"))
        .map_err(|err| format!("failed to read source ledger: {err}"))?;
    for (source_id, required) in [
        (MN_DCYF_DATA_REQUESTS_SOURCE_ID, "no request was submitted"),
        (MN_DCYF_PUBLIC_DATA_GUIDE_SOURCE_ID, "no response, fee"),
        (MN_STAT_13_03_SOURCE_ID, "new record or format"),
        (
            MN_STAT_142E_02_SOURCE_ID,
            "provider-public/recipient-private boundary",
        ),
    ] {
        let marker = format!("| `{source_id}` |");
        let row = ledger
            .lines()
            .find(|line| line.starts_with(&marker))
            .ok_or_else(|| format!("Minnesota CCAP request ledger row missing: {source_id}"))?;
        if !row.contains(required) {
            return Err(format!(
                "Minnesota CCAP request ledger row {source_id} missing {required}"
            ));
        }
    }
    for (path, tokens) in [
        (
            MN_CCAP_CY2025_REQUEST_TEMPLATE_PATH,
            vec![
                "# DRAFT — NOT SUBMITTED",
                "Explicit owner authorization is required",
                "[OWNER-APPROVED COPY FEE CAP]",
                "existing records only",
                "does not ask DCYF to answer",
                "questions, create a record",
                "No fee commitment is made by this draft.",
                "Internal Preflight — Must Not Accompany Submission",
            ],
        ),
        (
            MN_CCAP_CY2025_REQUEST_READER_PATH,
            vec![
                MN_CCAP_CY2025_REQUEST_SPEC_PATH,
                MN_CCAP_CY2025_REQUEST_TEMPLATE_PATH,
                "not_recorded",
                "all twelve claim gates remain unchanged",
                "Internal quarantine use only",
            ],
        ),
        (
            MN_CCAP_CY2025_REQUEST_REVIEW_PATH,
            vec![
                "All twelve",
                "claim gates remain false",
                "owner must explicitly authorize submission",
                "outbound_state_changed` remain false",
            ],
        ),
        (
            PULSE_58_MN_CCAP_CY2025_REQUEST_SPEC_PATH,
            vec!["WP-TAX-076", "EVID-TAX-076", "VAL-TAX-076", "Do not submit"],
        ),
        (
            "context/waves/2026-07-12-breadth-depth-benchmark-matrix/WAVE.md",
            vec!["pulse-58-minnesota-ccap-quality-learning-center-cy2025-request-specification.md"],
        ),
        ("docs/vtrace/WORK_PACKAGES.md", vec!["WP-TAX-076"]),
        ("docs/vtrace/TRACE.md", vec!["WP-TAX-076", "EVID-TAX-076"]),
        ("docs/vtrace/VERIFICATION.md", vec!["EVID-TAX-076"]),
        (
            "docs/vtrace/VALIDATION.md",
            vec!["VAL-TAX-076", "EVID-TAX-076"],
        ),
        ("docs/vtrace/EVIDENCE.md", vec!["EVID-TAX-076"]),
    ] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        for token in tokens {
            if !text.contains(token) {
                return Err(format!(
                    "Minnesota CCAP request integration {path} missing {token}"
                ));
            }
        }
    }
    let trace = fs::read_to_string(root.join("docs/vtrace/TRACE.md"))
        .map_err(|err| format!("failed to read TRACE: {err}"))?;
    for row in [
        "| REQ-TAX-004 | SPEC-TAX-004 | WP-TAX-076 | EVID-TAX-076 | current |",
        "| REQ-TAX-006 | SPEC-TAX-006 | WP-TAX-076 | EVID-TAX-076 | current |",
    ] {
        if !trace.lines().any(|line| line == row) {
            return Err(format!("Pulse 58 trace binding missing: {row}"));
        }
    }
    for (path, id, terminal) in [
        ("docs/vtrace/WORK_PACKAGES.md", "WP-TAX-076", "| complete |"),
        ("docs/vtrace/VALIDATION.md", "VAL-TAX-076", "| current |"),
        ("docs/vtrace/EVIDENCE.md", "EVID-TAX-076", "| passed |"),
    ] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if !text
            .lines()
            .any(|line| line.starts_with(&format!("| {id} |")) && line.ends_with(terminal))
        {
            return Err(format!("Pulse 58 VTRACE status missing: {id} / {terminal}"));
        }
    }
    let verification = fs::read_to_string(root.join("docs/vtrace/VERIFICATION.md"))
        .map_err(|err| format!("failed to read VERIFICATION: {err}"))?;
    for requirement in ["REQ-TAX-004", "REQ-TAX-006"] {
        if !verification.lines().any(|line| {
            line.starts_with(&format!("| {requirement} |"))
                && line.ends_with("| current | EVID-TAX-076 |")
        }) {
            return Err(format!(
                "Pulse 58 verification binding missing: {requirement}"
            ));
        }
    }
    let claim_text = fs::read_to_string(root.join(EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_JSONL_PATH))
        .map_err(|err| format!("failed to read external-claim intake: {err}"))?;
    let claim = claim_text
        .lines()
        .filter_map(|line| serde_json::from_str::<ExternalAccountabilityClaimIntakeRecord>(line).ok())
        .find(|record| record.record_id == "external-claim:nick-shirley:2026:house-testimony-quality-learing-center:amount:01")
        .ok_or("Minnesota CCAP upstream claim missing")?;
    if claim.official_response.request_status != ExternalClaimResponseRequestStatus::NotRecorded
        || claim.official_response.requested_at.is_some()
        || !claim.official_response.response_source_ids.is_empty()
        || !claim.corroborating_source_ids.is_empty()
        || !claim.counterevidence_source_ids.is_empty()
        || !claim.amount_assertion.lineage_ids.is_empty()
        || claim.legal_or_administrative_status
            != ExternalClaimLegalOrAdministrativeStatus::NoneEstablished
        || !claim.claim_gates.all_false()
    {
        return Err("Minnesota CCAP request draft changed upstream claim state".to_string());
    }
    for path in ["README.md", "docs/reading/README.md"] {
        let text = fs::read_to_string(root.join(path))
            .map_err(|err| format!("failed to read {path}: {err}"))?;
        if text.contains(
            "minnesota-ccap-quality-learning-center-cy2025-existing-records-request-specification",
        ) {
            return Err(format!(
                "Minnesota CCAP internal request must not be routed from {path}"
            ));
        }
    }
    println!("validated Minnesota CCAP CY2025 existing-records request specification");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_log_applied_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_log_applied_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response log applied example JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseLogRecord> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_LOG_APPLIED_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row)
            .map_err(|err| format!("response log applied example JSONL: invalid row shape: {err}"))
    })
    .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(
            "performance demand response log applied example JSONL has no rows".to_string(),
        );
    }

    let updated_rows = rows
        .iter()
        .filter(|row| row.response_class != PerformanceDemandResponseLogClass::NotYetReceived)
        .count();
    if updated_rows == 0 {
        return Err(
            "performance demand response log applied example JSONL has no updated rows".to_string(),
        );
    }

    for row in rows {
        row.validate()?;
        if row.public_claim_allowed {
            return Err(
                "response log applied example unexpectedly allowed a public claim".to_string(),
            );
        }
        if row.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err("response log applied example changed the blocked claim gate".to_string());
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-log.applied-example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-log.applied-example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response log applied example JSONL");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_status_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_status_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response status applied example",
    )?;

    let parsed_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    let status: PerformanceDemandResponseStatus =
        serde_json::from_str(&parsed_text).map_err(|err| {
            format!(
                "failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_STATUS_APPLIED_EXAMPLE_PATH}: {err}"
            )
        })?;
    status.validate()?;
    if status.total_rows == status.not_yet_received {
        return Err(
            "response status applied example must include at least one updated row".to_string(),
        );
    }
    if status.public_claim_allowed != 0 {
        return Err(
            "response status applied example unexpectedly allows a public claim".to_string(),
        );
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-status.applied-example.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-status.applied-example.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response status applied example");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_dashboard_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_dashboard_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DASHBOARD_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response dashboard applied example",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-dashboard.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-dashboard.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response dashboard applied example");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_handoff_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_handoff_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response handoff applied example",
    )?;

    let handoff_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.md",
        "performance-demand-response-bundle.applied-example.json",
        "performance-demand-response-bundle.applied-example.schema.md",
    ] {
        if !handoff_text.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_HANDOFF_APPLIED_EXAMPLE_PATH} must route {required}"
            ));
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-handoff.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-handoff.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response handoff applied example");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response applied example schema",
    )?;

    let schema_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
    ] {
        if !schema_text.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_APPLIED_EXAMPLE_SCHEMA_PATH} must document {required}"
            ));
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-applied-example.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response applied example schema");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_delta_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_delta_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response delta applied example",
    )?;

    let delta_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    if !delta_text.contains("- Updated rows: 1") {
        return Err("response delta applied example must report one updated row".to_string());
    }
    if !delta_text.contains(PUBLIC_CLAIM_BLOCKED_LABEL) {
        return Err(
            "response delta applied example must preserve blocked public-claim gates".to_string(),
        );
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_delta_applied_example_jsonl(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_delta_applied_example_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH,
        &expected,
        "accountability performance demand response delta applied example JSONL",
    )?;

    let rows: Vec<PerformanceDemandResponseDeltaRow> = read_jsonl(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_JSONL_PATH),
    )?
    .into_iter()
    .map(|row| {
        serde_json::from_value(row).map_err(|err| {
            format!("response delta applied example JSONL: invalid row shape: {err}")
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    if rows.is_empty() {
        return Err(
            "performance demand response delta applied example JSONL has no rows".to_string(),
        );
    }
    for row in rows {
        row.validate()?;
        if row.after_claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err(
                "response delta applied example JSONL changed the blocked claim gate".to_string(),
            );
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.jsonl"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example JSONL");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_delta_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_delta_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_DELTA_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response delta applied example schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-delta.applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-delta.applied-example.schema.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response delta applied example schema");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_bundle_applied_example(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_bundle_applied_example(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH,
        &expected,
        "accountability performance demand response bundle applied example",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.md"
                .to_string(),
        );
    }

    let bundle = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH}: {err}"
        )
    })?;
    for required in [
        "performance-demand-response-intake.example.jsonl",
        "performance-demand-response-log.applied-example.jsonl",
        "performance-demand-response-status.applied-example.json",
        "performance-demand-response-dashboard.applied-example.md",
        "performance-demand-response-handoff.applied-example.md",
        "performance-demand-response-applied-example.schema.md",
        "performance-demand-response-delta.applied-example.md",
        "performance-demand-response-delta.applied-example.jsonl",
        "performance-demand-response-delta.applied-example.schema.md",
        "performance-demand-response-bundle.applied-example.json",
    ] {
        if !bundle.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_PATH} must include {required}"
            ));
        }
    }

    println!("validated accountability performance demand response bundle applied example");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_bundle_applied_example_json(
    root: &Path,
) -> Result<(), String> {
    let expected =
        build_accountability_performance_demand_response_bundle_applied_example_json(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH,
        &expected,
        "accountability performance demand response bundle applied example JSON",
    )?;

    let manifest_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_JSON_PATH}: {err}"
        )
    })?;
    let manifest: PerformanceDemandResponseBundleManifest = serde_json::from_str(&manifest_text)
        .map_err(|err| format!("failed to parse applied response bundle JSON: {err}"))?;
    manifest.validate()?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand response bundle applied example JSON");
    Ok(())
}

pub(crate) fn check_accountability_performance_demand_response_bundle_applied_example_schema(
    root: &Path,
) -> Result<(), String> {
    let expected = build_accountability_performance_demand_response_bundle_applied_example_schema();
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH,
        &expected,
        "accountability performance demand response bundle applied example schema",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-response-bundle.applied-example.schema.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-response-bundle.applied-example.schema.md"
                .to_string(),
        );
    }

    let schema = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH),
    )
    .map_err(|err| {
        format!(
            "failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH}: {err}"
        )
    })?;
    for required in [
        "PerformanceDemandResponseBundleManifest",
        "PerformanceDemandResponseBundleArtifact",
        "`artifact`",
        "`bundle_kind`",
        "`total_rows`",
        "`updated_rows`",
        "`public_claim_allowed`",
        "`public_claim_blocked`",
        "`artifacts`",
        "`boundary`",
        "`use_rule`",
        "`row_count`",
        "`sha256`",
    ] {
        if !schema.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_PERFORMANCE_DEMAND_RESPONSE_BUNDLE_APPLIED_EXAMPLE_SCHEMA_PATH} must document {required}"
            ));
        }
    }

    println!("validated accountability performance demand response bundle applied example schema");
    Ok(())
}

pub(crate) fn check_manifest(root: &Path) -> Result<(), String> {
    let expected = normalize_newlines(&build_manifest(root)?);
    let current = fs::read_to_string(root.join(MANIFEST_PATH))
        .map_err(|err| format!("failed to read {MANIFEST_PATH}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale manifest: run `cargo run -p taxlane-tools -- income-tax-outlay manifest`"
        ));
    }
    println!("validated income-tax outlay artifact manifest");
    Ok(())
}

