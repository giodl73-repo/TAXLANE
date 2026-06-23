use serde::{Deserialize, Serialize};

pub const ACCOUNTABILITY_RECORD_FAMILY: &str = "accountability_evidence";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactMetadata<'a> {
    pub path: &'a str,
    pub role: &'a str,
    pub grain: &'a str,
    pub kind: &'a str,
    pub canonical: &'a str,
}

pub fn validate_artifact_metadata(artifacts: &[ArtifactMetadata<'_>]) -> Result<(), String> {
    if artifacts.is_empty() {
        return Err("artifact manifest has no artifacts".to_string());
    }

    for artifact in artifacts {
        validate_required("artifact path", artifact.path)?;
        validate_required("artifact role", artifact.role)?;
        validate_required("artifact grain", artifact.grain)?;
        validate_required("artifact kind", artifact.kind)?;
        validate_required("artifact canonical", artifact.canonical)?;

        if artifact.path.contains('\\') {
            return Err(format!(
                "artifact path must use repo-relative forward slashes: {}",
                artifact.path
            ));
        }
        if artifact.path.starts_with('/') || artifact.path.contains("..") {
            return Err(format!(
                "artifact path is not repo-relative: {}",
                artifact.path
            ));
        }
        if !matches!(
            artifact.kind,
            "jsonl" | "json" | "csv" | "markdown" | "toml" | "rust" | "text"
        ) {
            return Err(format!(
                "artifact {} has unsupported kind {}",
                artifact.path, artifact.kind
            ));
        }
        if !matches!(
            artifact.canonical,
            "yes" | "no" | "view" | "supporting" | "source" | "generated"
        ) {
            return Err(format!(
                "artifact {} has unsupported canonical value {}",
                artifact.path, artifact.canonical
            ));
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountabilityEvidenceRecord {
    pub record_id: String,
    pub record_family: String,
    pub lane_id: Option<String>,
    pub program_or_account_id: Option<String>,
    pub source_ids: Vec<String>,
    pub observed_date: String,
    pub coverage_period: String,
    pub evidence_kind: EvidenceKind,
    pub indicator_value: Option<String>,
    pub indicator_units: Option<String>,
    pub comparison_basis: String,
    pub anomaly_class: AnomalyClass,
    pub allegation_status: AllegationStatus,
    pub review_status: ReviewStatus,
    pub due_process_caveat: String,
    pub public_summary: String,
}

impl AccountabilityEvidenceRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        if self.record_family != ACCOUNTABILITY_RECORD_FAMILY {
            return Err(format!(
                "record_family must be {ACCOUNTABILITY_RECORD_FAMILY}, got {}",
                self.record_family
            ));
        }
        if self.lane_id.is_none() && self.program_or_account_id.is_none() {
            return Err(
                "accountability evidence needs lane_id or program_or_account_id".to_string(),
            );
        }
        if self.source_ids.is_empty() {
            return Err("accountability evidence needs at least one source_id".to_string());
        }
        for source_id in &self.source_ids {
            validate_required("source_id", source_id)?;
        }
        validate_required("observed_date", &self.observed_date)?;
        validate_required("coverage_period", &self.coverage_period)?;
        validate_required("comparison_basis", &self.comparison_basis)?;
        validate_required("due_process_caveat", &self.due_process_caveat)?;
        validate_required("public_summary", &self.public_summary)?;

        if self.indicator_value.is_some() && self.indicator_units.is_none() {
            return Err("indicator_units is required when indicator_value exists".to_string());
        }

        if self.has_possible_misconduct_signal() && !self.has_reviewed_signal_status() {
            return Err(
                "possible fraud, waste, or abuse signals require source/accountability/role review"
                    .to_string(),
            );
        }

        if self.has_public_accusation_wording() && !self.has_official_allegation_status() {
            return Err(
                "public accusation wording requires official_finding or adjudicated status"
                    .to_string(),
            );
        }

        Ok(())
    }

    fn has_possible_misconduct_signal(&self) -> bool {
        matches!(
            self.anomaly_class,
            AnomalyClass::PossibleWaste | AnomalyClass::PossibleFraud | AnomalyClass::PossibleAbuse
        )
    }

    fn has_reviewed_signal_status(&self) -> bool {
        matches!(
            self.review_status,
            ReviewStatus::SourceReviewed
                | ReviewStatus::AccountabilityReviewed
                | ReviewStatus::RoleReviewed
        )
    }

    fn has_official_allegation_status(&self) -> bool {
        matches!(
            self.allegation_status,
            AllegationStatus::OfficialFinding | AllegationStatus::Adjudicated
        )
    }

    fn has_public_accusation_wording(&self) -> bool {
        let summary = self.public_summary.to_ascii_lowercase();
        [
            "committed fraud",
            "is fraud",
            "was fraud",
            "wasted money",
            "waste occurred",
            "abused funds",
            "stole",
            "stolen",
        ]
        .iter()
        .any(|phrase| summary.contains(phrase))
    }

    pub fn public_claim_readiness(&self) -> PublicClaimReadiness {
        if self.has_official_allegation_status() && self.review_status == ReviewStatus::RoleReviewed
        {
            PublicClaimReadiness::PublicClaimEligible
        } else if matches!(
            self.review_status,
            ReviewStatus::SourceReviewed | ReviewStatus::AccountabilityReviewed
        ) {
            PublicClaimReadiness::NeedsRoleReview
        } else {
            PublicClaimReadiness::EvidenceOnly
        }
    }

    pub fn accountability_next_action(&self) -> &'static str {
        let readiness = self.public_claim_readiness();
        if readiness == PublicClaimReadiness::PublicClaimEligible {
            return "Prepare exact public wording with source citations.";
        }
        if self.anomaly_class == AnomalyClass::MissingEvidence {
            return "Attach reviewed performance targets or outcome evidence before making a performance claim.";
        }
        if readiness == PublicClaimReadiness::NeedsRoleReview {
            return "Complete role review before any public claim wording.";
        }
        "Continue source custody and evidence review before public use."
    }

    pub fn accountability_demand_question(&self) -> &'static str {
        let readiness = self.public_claim_readiness();
        if readiness == PublicClaimReadiness::PublicClaimEligible {
            return "What exact public wording and source citations should be used for this reviewed finding?";
        }
        if self.anomaly_class == AnomalyClass::MissingEvidence {
            return "What reviewed performance target, outcome measure, or audit source should be attached before comparing spending to performance?";
        }
        if readiness == PublicClaimReadiness::NeedsRoleReview {
            return "What exact public wording, if any, can role review approve from the cited source record?";
        }
        "What source, comparison basis, or review step is needed before this record can support a public accountability claim?"
    }

    pub fn accountability_public_use_blocker(&self) -> &'static str {
        let readiness = self.public_claim_readiness();
        if readiness == PublicClaimReadiness::PublicClaimEligible {
            return "No blocker in readiness state; exact public wording still needs source citations.";
        }
        if self.anomaly_class == AnomalyClass::MissingEvidence {
            return "Reviewed performance target or outcome evidence is missing.";
        }
        if readiness == PublicClaimReadiness::NeedsRoleReview {
            return "Role review has not approved exact public wording.";
        }
        "Record remains internal evidence only."
    }

    pub fn accountability_work_item(&self) -> AccountabilityWorkItem<'_> {
        let readiness = self.public_claim_readiness();
        AccountabilityWorkItem {
            record_id: &self.record_id,
            lane_id: self.lane_id.as_deref(),
            program_or_account_id: self.program_or_account_id.as_deref(),
            readiness: readiness.as_str(),
            next_action: self.accountability_next_action(),
            demand_question: self.accountability_demand_question(),
            public_use_blocker: self.accountability_public_use_blocker(),
            public_claim_allowed: readiness == PublicClaimReadiness::PublicClaimEligible,
            public_summary: &self.public_summary,
        }
    }

    pub fn performance_demand_checklist_row(&self) -> PerformanceDemandChecklistRow<'_> {
        let work_item = self.accountability_work_item();
        PerformanceDemandChecklistRow {
            record_id: work_item.record_id,
            lane_id: work_item.lane_id,
            program_or_account_id: work_item.program_or_account_id,
            demand_question: work_item.demand_question,
            demand_evidence: PERFORMANCE_DEMAND_EVIDENCE,
            do_not_accept_yet: work_item.public_use_blocker,
            public_claim_allowed: work_item.public_claim_allowed,
            claim_gate: if work_item.public_claim_allowed {
                "Public claim allowed."
            } else {
                "Public claim blocked."
            },
            use_rule: PERFORMANCE_DEMAND_USE_RULE,
        }
    }

    pub fn performance_demand_checklist_record(&self) -> PerformanceDemandChecklistRecord {
        let row = self.performance_demand_checklist_row();
        PerformanceDemandChecklistRecord {
            record_id: row.record_id.to_string(),
            lane_id: row.lane_id.map(ToString::to_string),
            program_or_account_id: row.program_or_account_id.map(ToString::to_string),
            demand_question: row.demand_question.to_string(),
            demand_evidence: row
                .demand_evidence
                .iter()
                .map(|item| (*item).to_string())
                .collect(),
            do_not_accept_yet: row.do_not_accept_yet.to_string(),
            public_claim_allowed: row.public_claim_allowed,
            claim_gate: row.claim_gate.to_string(),
            use_rule: row.use_rule.to_string(),
        }
    }

    pub fn performance_demand_response_log_record(&self) -> PerformanceDemandResponseLogRecord {
        let checklist = self.performance_demand_checklist_record();
        PerformanceDemandResponseLogRecord {
            record_id: checklist.record_id,
            lane_id: checklist.lane_id,
            program_or_account_id: checklist.program_or_account_id,
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: checklist.do_not_accept_yet,
            claim_gate: checklist.claim_gate,
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountabilityWorkItem<'a> {
    pub record_id: &'a str,
    pub lane_id: Option<&'a str>,
    pub program_or_account_id: Option<&'a str>,
    pub readiness: &'static str,
    pub next_action: &'static str,
    pub demand_question: &'static str,
    pub public_use_blocker: &'static str,
    pub public_claim_allowed: bool,
    pub public_summary: &'a str,
}

pub const PERFORMANCE_DEMAND_EVIDENCE: &[&str] = &[
    "source record and source version",
    "reviewed performance target, outcome measure, audit source, or official finding",
    "role-approved exact public wording",
    "public-claim eligibility",
];

pub const PERFORMANCE_DEMAND_USE_RULE: &str = "Demand evidence and reviewed wording; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance.";
pub const PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION: &str =
    "Send or resend public-safe evidence request; keep claim gate blocked.";
pub const PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE: &str = "Track response status and remaining evidence gaps; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.";
pub const PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE: &str = "Capture reply custody and classification; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandChecklistRow<'a> {
    pub record_id: &'a str,
    pub lane_id: Option<&'a str>,
    pub program_or_account_id: Option<&'a str>,
    pub demand_question: &'static str,
    pub demand_evidence: &'static [&'static str],
    pub do_not_accept_yet: &'static str,
    pub public_claim_allowed: bool,
    pub claim_gate: &'static str,
    pub use_rule: &'static str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandChecklistRecord {
    pub record_id: String,
    pub lane_id: Option<String>,
    pub program_or_account_id: Option<String>,
    pub demand_question: String,
    pub demand_evidence: Vec<String>,
    pub do_not_accept_yet: String,
    pub public_claim_allowed: bool,
    pub claim_gate: String,
    pub use_rule: String,
}

impl PerformanceDemandChecklistRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        if self.lane_id.is_none() && self.program_or_account_id.is_none() {
            return Err(
                "performance demand checklist record needs lane_id or program_or_account_id"
                    .to_string(),
            );
        }
        validate_required("demand_question", &self.demand_question)?;
        validate_required("do_not_accept_yet", &self.do_not_accept_yet)?;
        validate_required("claim_gate", &self.claim_gate)?;
        validate_required("use_rule", &self.use_rule)?;

        let expected_evidence: Vec<String> = PERFORMANCE_DEMAND_EVIDENCE
            .iter()
            .map(|item| (*item).to_string())
            .collect();
        if self.demand_evidence != expected_evidence {
            return Err(
                "performance demand checklist record has unexpected demand_evidence".to_string(),
            );
        }
        if self.use_rule != PERFORMANCE_DEMAND_USE_RULE {
            return Err("performance demand checklist record has unexpected use_rule".to_string());
        }
        if self.public_claim_allowed && self.claim_gate != "Public claim allowed." {
            return Err(
                "performance demand checklist record allowed claim has wrong claim_gate"
                    .to_string(),
            );
        }
        if !self.public_claim_allowed && self.claim_gate != "Public claim blocked." {
            return Err(
                "performance demand checklist record blocked claim has wrong claim_gate"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerformanceDemandResponseLogClass {
    NotYetReceived,
    CompleteEvidenceResponse,
    PartialEvidenceResponse,
    ProcessOnlyResponse,
    NoEvidenceResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseLogRecord {
    pub record_id: String,
    pub lane_id: Option<String>,
    pub program_or_account_id: Option<String>,
    pub response_class: PerformanceDemandResponseLogClass,
    pub evidence_received: Vec<String>,
    pub missing_evidence: String,
    pub claim_gate: String,
    pub public_claim_allowed: bool,
    pub next_action: String,
    pub use_rule: String,
}

impl PerformanceDemandResponseLogRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        if self.lane_id.is_none() && self.program_or_account_id.is_none() {
            return Err(
                "performance demand response log record needs lane_id or program_or_account_id"
                    .to_string(),
            );
        }
        validate_required("missing_evidence", &self.missing_evidence)?;
        validate_required("claim_gate", &self.claim_gate)?;
        validate_required("next_action", &self.next_action)?;
        validate_required("use_rule", &self.use_rule)?;

        for evidence in &self.evidence_received {
            validate_required("evidence_received item", evidence)?;
        }

        if self.use_rule != PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE {
            return Err(
                "performance demand response log record has unexpected use_rule".to_string(),
            );
        }
        if self.public_claim_allowed {
            return Err(
                "performance demand response log record must keep public_claim_allowed false"
                    .to_string(),
            );
        }
        if self.claim_gate != "Public claim blocked." {
            return Err(
                "performance demand response log record must preserve blocked claim_gate"
                    .to_string(),
            );
        }
        if self.response_class == PerformanceDemandResponseLogClass::NotYetReceived
            && !self.evidence_received.is_empty()
        {
            return Err(
                "not-yet-received response log records must not list evidence_received".to_string(),
            );
        }
        if matches!(
            self.response_class,
            PerformanceDemandResponseLogClass::ProcessOnlyResponse
                | PerformanceDemandResponseLogClass::NoEvidenceResponse
        ) && !self.evidence_received.is_empty()
        {
            return Err(
                "process-only and no-evidence response log records must not list evidence_received"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerformanceDemandResponseClass {
    CompleteEvidenceResponse,
    PartialEvidenceResponse,
    ProcessOnlyResponse,
    NoEvidenceResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseIntakeRecord {
    pub record_id: String,
    pub reply_source_id: String,
    pub reply_received_date: String,
    pub sender_or_office: String,
    pub response_class: PerformanceDemandResponseClass,
    pub evidence_received: Vec<String>,
    pub missing_evidence: String,
    pub role_review_needed: bool,
    pub public_claim_allowed: bool,
    pub use_rule: String,
}

impl PerformanceDemandResponseIntakeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("reply_source_id", &self.reply_source_id)?;
        validate_required("reply_received_date", &self.reply_received_date)?;
        validate_iso_date("reply_received_date", &self.reply_received_date)?;
        validate_required("sender_or_office", &self.sender_or_office)?;
        validate_required("missing_evidence", &self.missing_evidence)?;
        validate_required("use_rule", &self.use_rule)?;

        for evidence in &self.evidence_received {
            validate_required("evidence_received item", evidence)?;
        }

        if self.use_rule != PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE {
            return Err(
                "performance demand response intake record has unexpected use_rule".to_string(),
            );
        }
        if !self.role_review_needed {
            return Err(
                "performance demand response intake record must keep role_review_needed true"
                    .to_string(),
            );
        }
        if self.public_claim_allowed {
            return Err(
                "performance demand response intake record must keep public_claim_allowed false"
                    .to_string(),
            );
        }
        if self.response_class == PerformanceDemandResponseClass::CompleteEvidenceResponse
            && self.evidence_received.is_empty()
        {
            return Err("complete evidence response intake requires evidence_received".to_string());
        }
        if matches!(
            self.response_class,
            PerformanceDemandResponseClass::ProcessOnlyResponse
                | PerformanceDemandResponseClass::NoEvidenceResponse
        ) && !self.evidence_received.is_empty()
        {
            return Err(
                "process-only and no-evidence response intake records must not list evidence_received"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicClaimReadiness {
    EvidenceOnly,
    NeedsRoleReview,
    PublicClaimEligible,
}

impl PublicClaimReadiness {
    pub fn as_str(&self) -> &'static str {
        match self {
            PublicClaimReadiness::EvidenceOnly => "EvidenceOnly",
            PublicClaimReadiness::NeedsRoleReview => "NeedsRoleReview",
            PublicClaimReadiness::PublicClaimEligible => "PublicClaimEligible",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Performance,
    SpendingVariance,
    DuplicateAward,
    VendorConcentration,
    EligibilityMismatch,
    AuditFinding,
    IgFinding,
    GaoFinding,
    DataQuality,
    Other,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyClass {
    None,
    Variance,
    Outlier,
    MissingEvidence,
    SourceConflict,
    ControlFailure,
    PossibleWaste,
    PossibleFraud,
    PossibleAbuse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllegationStatus {
    NotAnAllegation,
    ReferredForReview,
    OfficialFinding,
    Adjudicated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStatus {
    Draft,
    SourceReviewed,
    AccountabilityReviewed,
    RoleReviewed,
    Superseded,
    Retired,
}

fn validate_required(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is required"))
    } else {
        Ok(())
    }
}

fn validate_iso_date(label: &str, value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
    {
        Ok(())
    } else {
        Err(format!("{label} must use YYYY-MM-DD"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_accountability_record_boundary() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("public-goods".to_string()),
            program_or_account_id: None,
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::SpendingVariance,
            indicator_value: Some("12.5".to_string()),
            indicator_units: Some("percent".to_string()),
            comparison_basis: "prior year".to_string(),
            anomaly_class: AnomalyClass::Variance,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::SourceReviewed,
            due_process_caveat: "Evidence signal only; not an allegation.".to_string(),
            public_summary: "Variance against the named comparison basis.".to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_public_fraud_wording_without_finding() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("public-goods".to_string()),
            program_or_account_id: None,
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::SpendingVariance,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "audit rule".to_string(),
            anomaly_class: AnomalyClass::PossibleFraud,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::SourceReviewed,
            due_process_caveat: "Evidence signal only; not an allegation.".to_string(),
            public_summary: "This vendor committed fraud.".to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_possible_misconduct_signal_without_review() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("public-goods".to_string()),
            program_or_account_id: None,
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::SpendingVariance,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "audit rule".to_string(),
            anomaly_class: AnomalyClass::PossibleWaste,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::Draft,
            due_process_caveat: "Evidence signal only; not an allegation.".to_string(),
            public_summary: "This record has a source pending review.".to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn allows_possible_misconduct_signal_with_reviewed_non_accusatory_wording() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("public-goods".to_string()),
            program_or_account_id: None,
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::SpendingVariance,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "audit rule".to_string(),
            anomaly_class: AnomalyClass::PossibleWaste,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::SourceReviewed,
            due_process_caveat: "Evidence signal only; not an allegation.".to_string(),
            public_summary: "This record has an evidence signal that requires audit review."
                .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn classifies_source_reviewed_record_as_needing_role_review() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("transportation".to_string()),
            program_or_account_id: Some("omb-function-400".to_string()),
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::Performance,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "source custody".to_string(),
            anomaly_class: AnomalyClass::None,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::SourceReviewed,
            due_process_caveat: "Evidence signal only; not an allegation.".to_string(),
            public_summary: "Source-backed record for later review.".to_string(),
        };

        assert_eq!(
            record.public_claim_readiness(),
            PublicClaimReadiness::NeedsRoleReview
        );
        assert_eq!(
            record.accountability_next_action(),
            "Complete role review before any public claim wording."
        );
        assert_eq!(
            record.accountability_public_use_blocker(),
            "Role review has not approved exact public wording."
        );
    }

    #[test]
    fn classifies_role_reviewed_official_finding_as_public_claim_eligible() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("transportation".to_string()),
            program_or_account_id: Some("omb-function-400".to_string()),
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::AuditFinding,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "official audit finding".to_string(),
            anomaly_class: AnomalyClass::ControlFailure,
            allegation_status: AllegationStatus::OfficialFinding,
            review_status: ReviewStatus::RoleReviewed,
            due_process_caveat: "Official finding; public wording still requires source context."
                .to_string(),
            public_summary: "Official finding is available for reviewed public use.".to_string(),
        };

        assert_eq!(
            record.public_claim_readiness(),
            PublicClaimReadiness::PublicClaimEligible
        );
        assert_eq!(
            record.accountability_demand_question(),
            "What exact public wording and source citations should be used for this reviewed finding?"
        );
    }

    #[test]
    fn maps_missing_evidence_to_performance_demand_question() {
        let record = AccountabilityEvidenceRecord {
            record_id: "accountability-evidence:test".to_string(),
            record_family: ACCOUNTABILITY_RECORD_FAMILY.to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            source_ids: vec!["SRC-TEST".to_string()],
            observed_date: "2026-06-23".to_string(),
            coverage_period: "FY2025".to_string(),
            evidence_kind: EvidenceKind::DataQuality,
            indicator_value: None,
            indicator_units: None,
            comparison_basis: "source custody".to_string(),
            anomaly_class: AnomalyClass::MissingEvidence,
            allegation_status: AllegationStatus::NotAnAllegation,
            review_status: ReviewStatus::Draft,
            due_process_caveat: "Evidence gap only; not an allegation.".to_string(),
            public_summary: "Performance baseline is not attached.".to_string(),
        };

        assert_eq!(
            record.accountability_next_action(),
            "Attach reviewed performance targets or outcome evidence before making a performance claim."
        );
        assert_eq!(
            record.accountability_demand_question(),
            "What reviewed performance target, outcome measure, or audit source should be attached before comparing spending to performance?"
        );
        assert_eq!(
            record.accountability_public_use_blocker(),
            "Reviewed performance target or outcome evidence is missing."
        );
        assert_eq!(
            record.accountability_work_item(),
            AccountabilityWorkItem {
                record_id: "accountability-evidence:test",
                lane_id: Some("health"),
                program_or_account_id: Some("omb-function-550"),
                readiness: "EvidenceOnly",
                next_action: "Attach reviewed performance targets or outcome evidence before making a performance claim.",
                demand_question: "What reviewed performance target, outcome measure, or audit source should be attached before comparing spending to performance?",
                public_use_blocker: "Reviewed performance target or outcome evidence is missing.",
                public_claim_allowed: false,
                public_summary: "Performance baseline is not attached.",
            }
        );
        assert_eq!(
            record.performance_demand_checklist_row(),
            PerformanceDemandChecklistRow {
                record_id: "accountability-evidence:test",
                lane_id: Some("health"),
                program_or_account_id: Some("omb-function-550"),
                demand_question: "What reviewed performance target, outcome measure, or audit source should be attached before comparing spending to performance?",
                demand_evidence: PERFORMANCE_DEMAND_EVIDENCE,
                do_not_accept_yet: "Reviewed performance target or outcome evidence is missing.",
                public_claim_allowed: false,
                claim_gate: "Public claim blocked.",
                use_rule: PERFORMANCE_DEMAND_USE_RULE,
            }
        );
        let checklist_record = record.performance_demand_checklist_record();
        checklist_record.validate().unwrap();
        assert_eq!(
            checklist_record,
            PerformanceDemandChecklistRecord {
                record_id: "accountability-evidence:test".to_string(),
                lane_id: Some("health".to_string()),
                program_or_account_id: Some("omb-function-550".to_string()),
                demand_question: "What reviewed performance target, outcome measure, or audit source should be attached before comparing spending to performance?".to_string(),
                demand_evidence: PERFORMANCE_DEMAND_EVIDENCE
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                do_not_accept_yet: "Reviewed performance target or outcome evidence is missing.".to_string(),
                public_claim_allowed: false,
                claim_gate: "Public claim blocked.".to_string(),
                use_rule: PERFORMANCE_DEMAND_USE_RULE.to_string(),
            }
        );
        let response_log_record = record.performance_demand_response_log_record();
        response_log_record.validate().unwrap();
        assert_eq!(
            response_log_record,
            PerformanceDemandResponseLogRecord {
                record_id: "accountability-evidence:test".to_string(),
                lane_id: Some("health".to_string()),
                program_or_account_id: Some("omb-function-550".to_string()),
                response_class: PerformanceDemandResponseLogClass::NotYetReceived,
                evidence_received: Vec::new(),
                missing_evidence: "Reviewed performance target or outcome evidence is missing."
                    .to_string(),
                claim_gate: "Public claim blocked.".to_string(),
                public_claim_allowed: false,
                next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
                use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
            }
        );
    }

    #[test]
    fn blocks_response_log_public_claim_bypass() {
        let record = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::PartialEvidenceResponse,
            evidence_received: vec!["audit memo URL".to_string()],
            missing_evidence: "Role review remains missing.".to_string(),
            claim_gate: "Public claim blocked.".to_string(),
            public_claim_allowed: true,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_not_yet_received_response_log_with_evidence() {
        let record = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: vec!["audit memo URL".to_string()],
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: "Public claim blocked.".to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_response_intake_record_guardrails() {
        let record = PerformanceDemandResponseIntakeRecord {
            record_id: "accountability-evidence:test".to_string(),
            reply_source_id: "SRC-REPLY-TEST".to_string(),
            reply_received_date: "2026-06-23".to_string(),
            sender_or_office: "Example Office".to_string(),
            response_class: PerformanceDemandResponseClass::PartialEvidenceResponse,
            evidence_received: vec!["audit memo URL".to_string()],
            missing_evidence: "Role-approved public wording remains missing.".to_string(),
            role_review_needed: true,
            public_claim_allowed: false,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_response_intake_public_claim_bypass() {
        let record = PerformanceDemandResponseIntakeRecord {
            record_id: "accountability-evidence:test".to_string(),
            reply_source_id: "SRC-REPLY-TEST".to_string(),
            reply_received_date: "2026-06-23".to_string(),
            sender_or_office: "Example Office".to_string(),
            response_class: PerformanceDemandResponseClass::CompleteEvidenceResponse,
            evidence_received: vec!["official finding URL".to_string()],
            missing_evidence: "Role review is still pending.".to_string(),
            role_review_needed: true,
            public_claim_allowed: true,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_process_only_response_intake_with_evidence() {
        let record = PerformanceDemandResponseIntakeRecord {
            record_id: "accountability-evidence:test".to_string(),
            reply_source_id: "SRC-REPLY-TEST".to_string(),
            reply_received_date: "2026-06-23".to_string(),
            sender_or_office: "Example Office".to_string(),
            response_class: PerformanceDemandResponseClass::ProcessOnlyResponse,
            evidence_received: vec!["process note".to_string()],
            missing_evidence: "Requested performance evidence remains missing.".to_string(),
            role_review_needed: true,
            public_claim_allowed: false,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_INTAKE_USE_RULE.to_string(),
        };

        assert!(record.validate().is_err());
    }
}
