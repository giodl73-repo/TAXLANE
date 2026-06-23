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
    }
}
