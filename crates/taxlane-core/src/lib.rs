use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub const ACCOUNTABILITY_RECORD_FAMILY: &str = "accountability_evidence";
pub const SPEND_CATEGORY_MAP_MODEL_ID: &str = "spend-category-map-v1";
pub const BREADTH_BENCHMARK_RECORD_FAMILY: &str = "breadth_benchmark_matrix";
pub const HEADLINE_BASIS_RECORD_FAMILY: &str = "headline_basis_crosswalk";

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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SpendCategoryMapRecord {
    pub model_id: String,
    pub record_id: String,
    pub fiscal_year: u16,
    pub rank: u16,
    pub source_level: String,
    pub source_id: String,
    pub function_code: String,
    pub function_label: String,
    pub subfunction_code: String,
    pub subfunction_label: String,
    pub subfunction_outlays_millions: f64,
    pub share_of_total_outlays_percent: f64,
    pub modeled_income_tax_allocation_millions: f64,
    pub allocation_method: String,
    pub legal_allocation_status: String,
    pub funding_caveat: String,
    pub next_source_need: String,
    pub accountability_status: String,
}

impl SpendCategoryMapRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("model_id", &self.model_id)?;
        validate_required("record_id", &self.record_id)?;
        validate_required("source_level", &self.source_level)?;
        validate_required("source_id", &self.source_id)?;
        validate_required("function_code", &self.function_code)?;
        validate_required("function_label", &self.function_label)?;
        validate_required("subfunction_code", &self.subfunction_code)?;
        validate_required("subfunction_label", &self.subfunction_label)?;
        validate_required("allocation_method", &self.allocation_method)?;
        validate_required("legal_allocation_status", &self.legal_allocation_status)?;
        validate_required("funding_caveat", &self.funding_caveat)?;
        validate_required("next_source_need", &self.next_source_need)?;
        validate_required("accountability_status", &self.accountability_status)?;

        if self.model_id != SPEND_CATEGORY_MAP_MODEL_ID {
            return Err(format!(
                "spend category model_id must be {SPEND_CATEGORY_MAP_MODEL_ID}, got {}",
                self.model_id
            ));
        }
        if self.fiscal_year != 2025 {
            return Err(format!(
                "spend category map v1 only covers FY2025, got {}",
                self.fiscal_year
            ));
        }
        if self.rank == 0 {
            return Err("spend category rank must be positive".to_string());
        }
        if self.source_level != "omb_subfunction" {
            return Err(format!(
                "spend category source_level must be omb_subfunction, got {}",
                self.source_level
            ));
        }
        if self.source_id != "SRC-OMB-HIST-3-2-FY2027" {
            return Err(format!(
                "spend category source_id must be SRC-OMB-HIST-3-2-FY2027, got {}",
                self.source_id
            ));
        }
        if self.allocation_method != "proportional_outlay_share" {
            return Err(format!(
                "spend category allocation_method must be proportional_outlay_share, got {}",
                self.allocation_method
            ));
        }
        if self.legal_allocation_status != "modeled_not_legal_dedication" {
            return Err(format!(
                "spend category legal_allocation_status must be modeled_not_legal_dedication, got {}",
                self.legal_allocation_status
            ));
        }
        if self.accountability_status != "question_surface_only" {
            return Err(format!(
                "spend category accountability_status must be question_surface_only, got {}",
                self.accountability_status
            ));
        }
        if self.subfunction_outlays_millions <= 0.0 {
            return Err("spend category outlays must be positive".to_string());
        }
        if self.share_of_total_outlays_percent <= 0.0 || self.share_of_total_outlays_percent > 100.0
        {
            return Err("spend category outlay share must be between 0 and 100".to_string());
        }
        if self.modeled_income_tax_allocation_millions <= 0.0 {
            return Err(
                "spend category modeled income-tax allocation must be positive".to_string(),
            );
        }
        if self.funding_caveat.to_ascii_lowercase().contains("fraud")
            || self.funding_caveat.to_ascii_lowercase().contains("waste")
        {
            return Err("spend category funding caveat must not imply fraud or waste".to_string());
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BreadthBenchmarkRecord {
    pub record_id: String,
    pub record_family: String,
    pub lane_id: String,
    pub metric_label: String,
    pub depth_tier: String,
    pub coverage_status: String,
    pub current_value: Option<f64>,
    pub current_unit: String,
    pub current_period: String,
    pub current_basis: String,
    pub benchmark_low: Option<f64>,
    pub benchmark_high: Option<f64>,
    pub benchmark_unit: String,
    pub benchmark_period: String,
    pub benchmark_type: String,
    pub gap_direction: String,
    pub comparability_grade: String,
    pub source_ids: Vec<String>,
    pub efficiency_gap_status: String,
    pub improper_payment_amount_millions: Option<f64>,
    pub improper_payment_rate_percent: Option<f64>,
    pub improper_payment_scope: String,
    pub fraud_amount_millions: Option<f64>,
    pub fraud_status: String,
    pub recoverable_savings_millions: Option<f64>,
    pub savings_status: String,
    pub next_depth_need: String,
}

impl BreadthBenchmarkRecord {
    pub fn validate(&self) -> Result<(), String> {
        for (label, value) in [
            ("record_id", self.record_id.as_str()),
            ("record_family", self.record_family.as_str()),
            ("lane_id", self.lane_id.as_str()),
            ("metric_label", self.metric_label.as_str()),
            ("depth_tier", self.depth_tier.as_str()),
            ("coverage_status", self.coverage_status.as_str()),
            ("current_unit", self.current_unit.as_str()),
            ("current_period", self.current_period.as_str()),
            ("current_basis", self.current_basis.as_str()),
            ("benchmark_unit", self.benchmark_unit.as_str()),
            ("benchmark_period", self.benchmark_period.as_str()),
            ("benchmark_type", self.benchmark_type.as_str()),
            ("gap_direction", self.gap_direction.as_str()),
            ("comparability_grade", self.comparability_grade.as_str()),
            ("efficiency_gap_status", self.efficiency_gap_status.as_str()),
            (
                "improper_payment_scope",
                self.improper_payment_scope.as_str(),
            ),
            ("fraud_status", self.fraud_status.as_str()),
            ("savings_status", self.savings_status.as_str()),
            ("next_depth_need", self.next_depth_need.as_str()),
        ] {
            validate_required(label, value)?;
        }
        validate_required_vec("source_ids", &self.source_ids)?;

        if self.record_family != BREADTH_BENCHMARK_RECORD_FAMILY {
            return Err(format!(
                "breadth benchmark record_family must be {BREADTH_BENCHMARK_RECORD_FAMILY}, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.depth_tier.as_str(),
            "tier_1_full" | "tier_2_card" | "tier_3_gap"
        ) {
            return Err(format!("unsupported depth_tier {}", self.depth_tier));
        }
        if !matches!(
            self.coverage_status.as_str(),
            "full_comparison" | "topline_only" | "coverage_gap"
        ) {
            return Err(format!(
                "unsupported coverage_status {}",
                self.coverage_status
            ));
        }
        if !matches!(
            self.comparability_grade.as_str(),
            "A" | "B" | "C" | "not_scored"
        ) {
            return Err(format!(
                "unsupported comparability_grade {}",
                self.comparability_grade
            ));
        }
        if self.coverage_status == "coverage_gap" && self.current_value.is_some() {
            return Err(
                "coverage-gap rows must not publish an unsupported current value".to_string(),
            );
        }
        if self.current_value.is_some_and(|value| value < 0.0) {
            return Err("current_value must be nonnegative".to_string());
        }
        match (self.benchmark_low, self.benchmark_high) {
            (Some(low), Some(high)) if low >= 0.0 && high >= low => {
                if self.current_value.is_none() {
                    return Err("benchmarked rows need a current value".to_string());
                }
                if self.current_unit != self.benchmark_unit {
                    return Err(
                        "current and benchmark units must match before computing a gap".to_string(),
                    );
                }
            }
            (None, None) => {
                if self.gap_direction != "not_benchmarked" {
                    return Err(
                        "rows without a benchmark must use gap_direction=not_benchmarked"
                            .to_string(),
                    );
                }
            }
            _ => {
                return Err(
                    "benchmark_low and benchmark_high must be supplied together".to_string()
                );
            }
        }
        if !matches!(
            self.efficiency_gap_status.as_str(),
            "observed_comparison_not_causal" | "not_benchmarked" | "coverage_gap"
        ) {
            return Err(format!(
                "unsupported efficiency_gap_status {}",
                self.efficiency_gap_status
            ));
        }
        if self.fraud_amount_millions.is_some() || self.fraud_status != "not_measured_not_inferred"
        {
            return Err("benchmark rows must not infer or publish a fraud amount".to_string());
        }
        if self.recoverable_savings_millions.is_some()
            || self.savings_status != "blocked_not_scored"
        {
            return Err("benchmark rows must not convert gaps or improper payments into recoverable savings".to_string());
        }
        if self.improper_payment_amount_millions.is_some()
            || self.improper_payment_rate_percent.is_some()
        {
            if self.improper_payment_scope == "none_attached" {
                return Err("improper-payment values need a named program scope".to_string());
            }
        } else if self.improper_payment_scope != "none_attached" {
            return Err(
                "improper-payment scope must be none_attached when no value is present".to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HeadlineBasisRecord {
    pub record_id: String,
    pub record_family: String,
    pub comparison_group: String,
    pub measure_label: String,
    pub value: f64,
    pub unit: String,
    pub period: String,
    pub government_scope: String,
    pub accounting_scope: String,
    pub source_ids: Vec<String>,
    pub headline_use: String,
    pub substitution_status: String,
    pub cannot_substitute_for: Vec<String>,
    pub interpretation: String,
}

impl HeadlineBasisRecord {
    pub fn validate(&self) -> Result<(), String> {
        for (label, value) in [
            ("record_id", self.record_id.as_str()),
            ("record_family", self.record_family.as_str()),
            ("comparison_group", self.comparison_group.as_str()),
            ("measure_label", self.measure_label.as_str()),
            ("unit", self.unit.as_str()),
            ("period", self.period.as_str()),
            ("government_scope", self.government_scope.as_str()),
            ("accounting_scope", self.accounting_scope.as_str()),
            ("headline_use", self.headline_use.as_str()),
            ("substitution_status", self.substitution_status.as_str()),
            ("interpretation", self.interpretation.as_str()),
        ] {
            validate_required(label, value)?;
        }
        validate_required_vec("source_ids", &self.source_ids)?;
        validate_required_vec("cannot_substitute_for", &self.cannot_substitute_for)?;
        if self.record_family != HEADLINE_BASIS_RECORD_FAMILY {
            return Err(format!(
                "headline basis record_family must be {HEADLINE_BASIS_RECORD_FAMILY}"
            ));
        }
        if self.value < 0.0 {
            return Err("headline basis value must be nonnegative".to_string());
        }
        if !matches!(
            self.headline_use.as_str(),
            "canonical" | "supporting" | "comparison_context"
        ) {
            return Err(format!("unsupported headline_use {}", self.headline_use));
        }
        if self.substitution_status != "not_interchangeable" {
            return Err("headline basis rows must remain not_interchangeable".to_string());
        }
        if self.cannot_substitute_for.contains(&self.record_id) {
            return Err(
                "headline basis row cannot list itself as an incompatible substitute".to_string(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PerUnitDisplayReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub display_status: String,
    pub lane_id: String,
    pub public_label: String,
    pub numerator_label: String,
    pub numerator_value: f64,
    pub numerator_unit: String,
    pub denominator_id: String,
    pub denominator_value: Option<f64>,
    pub denominator_unit: String,
    pub computed_value_usd: Option<f64>,
    pub year: String,
    pub year_basis: String,
    pub source_ids: Vec<String>,
    pub source_record_ids: Vec<String>,
    pub public_use_rule: String,
}

impl PerUnitDisplayReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("display_status", &self.display_status)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("public_label", &self.public_label)?;
        validate_required("numerator_label", &self.numerator_label)?;
        validate_required("numerator_unit", &self.numerator_unit)?;
        validate_required("denominator_id", &self.denominator_id)?;
        validate_required("denominator_unit", &self.denominator_unit)?;
        validate_required("year", &self.year)?;
        validate_required("year_basis", &self.year_basis)?;
        validate_required("public_use_rule", &self.public_use_rule)?;
        validate_required_vec("source_ids", &self.source_ids)?;
        validate_required_vec("source_record_ids", &self.source_record_ids)?;

        if self.record_family != "per_unit_display_readiness" {
            return Err(format!(
                "per-unit readiness record_family must be per_unit_display_readiness, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.display_status.as_str(),
            "ready_same_source_year_basis"
                | "illustrative_cross_basis"
                | "blocked_missing_denominator"
        ) {
            return Err(format!(
                "unsupported per-unit display_status {}",
                self.display_status
            ));
        }
        if self.numerator_value <= 0.0 {
            return Err("per-unit numerator_value must be positive".to_string());
        }

        match self.display_status.as_str() {
            "blocked_missing_denominator" => {
                if self.denominator_value.is_some() || self.computed_value_usd.is_some() {
                    return Err(
                        "blocked per-unit readiness records must not publish computed values"
                            .to_string(),
                    );
                }
                if !contains_case_insensitive(&self.public_use_rule, "blocked") {
                    return Err(
                        "blocked per-unit readiness records must name the blocked status"
                            .to_string(),
                    );
                }
                if !contains_case_insensitive(&self.public_use_rule, "do not substitute") {
                    return Err(
                        "blocked per-unit readiness records must forbid denominator substitution"
                            .to_string(),
                    );
                }
            }
            "illustrative_cross_basis" => {
                validate_positive_option(
                    "denominator_value",
                    self.denominator_value,
                    "illustrative per-unit readiness records",
                )?;
                validate_positive_option(
                    "computed_value_usd",
                    self.computed_value_usd,
                    "illustrative per-unit readiness records",
                )?;
                if self.year_basis != "fiscal_year_over_calendar_year" {
                    return Err(
                        "illustrative per-unit readiness records must use fiscal_year_over_calendar_year"
                            .to_string(),
                    );
                }
                let rule = self.public_use_rule.to_ascii_lowercase();
                if !(rule.contains("illustration")
                    && rule.contains("not")
                    && rule.contains("liability"))
                {
                    return Err(
                        "illustrative per-unit readiness records must visibly block liability wording"
                            .to_string(),
                    );
                }
            }
            "ready_same_source_year_basis" => {
                validate_positive_option(
                    "denominator_value",
                    self.denominator_value,
                    "ready per-unit readiness records",
                )?;
                validate_positive_option(
                    "computed_value_usd",
                    self.computed_value_usd,
                    "ready per-unit readiness records",
                )?;
                if self.year_basis != "calendar_year" {
                    return Err(
                        "ready per-unit readiness records must use calendar_year".to_string()
                    );
                }
                if contains_case_insensitive(&self.public_use_rule, "liability calculation")
                    && !contains_case_insensitive(&self.public_use_rule, "not")
                {
                    return Err(
                        "ready per-unit readiness records must not imply individual liability"
                            .to_string(),
                    );
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PerUnitReceiptCardRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_readiness_record_id: String,
    pub card_status: String,
    pub lane_id: String,
    pub headline: String,
    pub amount_usd: Option<f64>,
    pub basis_label: String,
    pub visible_caveat: String,
    pub allowed_public_use: String,
    pub blocked_public_use: String,
}

impl PerUnitReceiptCardRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_readiness_record_id",
            &self.source_readiness_record_id,
        )?;
        validate_required("card_status", &self.card_status)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("headline", &self.headline)?;
        validate_required("basis_label", &self.basis_label)?;
        validate_required("visible_caveat", &self.visible_caveat)?;
        validate_required("allowed_public_use", &self.allowed_public_use)?;
        validate_required("blocked_public_use", &self.blocked_public_use)?;

        if self.record_family != "per_unit_receipt_cards" {
            return Err(format!(
                "per-unit receipt card record_family must be per_unit_receipt_cards, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.card_status.as_str(),
            "source_basis_context" | "illustrative_cross_basis" | "blocked_missing_denominator"
        ) {
            return Err(format!(
                "unsupported per-unit receipt card_status {}",
                self.card_status
            ));
        }

        match self.card_status.as_str() {
            "blocked_missing_denominator" => {
                if self.amount_usd.is_some() {
                    return Err("blocked per-unit cards must not publish an amount".to_string());
                }
                if !(contains_case_insensitive(&self.headline, "blocked")
                    && contains_case_insensitive(&self.visible_caveat, "do not substitute"))
                {
                    return Err(
                        "blocked per-unit cards must visibly name the block and substitution guard"
                            .to_string(),
                    );
                }
            }
            "illustrative_cross_basis" => {
                validate_positive_option(
                    "amount_usd",
                    self.amount_usd,
                    "illustrative per-unit cards",
                )?;
                let caveat = self.visible_caveat.to_ascii_lowercase();
                if !(caveat.contains("cross-basis")
                    && caveat.contains("not")
                    && caveat.contains("tax liability")
                    && caveat.contains("personal benefit")
                    && caveat.contains("legal dedication"))
                {
                    return Err(
                        "illustrative per-unit cards must block liability, benefit, and legal-dedication wording"
                            .to_string(),
                    );
                }
            }
            "source_basis_context" => {
                validate_positive_option("amount_usd", self.amount_usd, "source-basis cards")?;
                let public_text = format!(
                    "{} {} {}",
                    self.headline, self.visible_caveat, self.blocked_public_use
                )
                .to_ascii_lowercase();
                if (public_text.contains("what any enrollee paid")
                    || public_text.contains("equal tax liability"))
                    && !public_text.contains("do not")
                {
                    return Err(
                        "source-basis per-unit cards must not imply personal payment or tax liability"
                            .to_string(),
                    );
                }
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EfficiencyPressureRecord {
    pub record_id: String,
    pub record_family: String,
    pub fiscal_year: u16,
    pub surface: String,
    pub related_spend_categories: Vec<String>,
    pub pressure_basis: Vec<String>,
    pub pressure_level: String,
    pub not_a_finding: bool,
    pub cost_down_levers: Vec<String>,
    pub outcome_floor: String,
    pub evidence_needed: Vec<String>,
    pub public_claim_status: String,
}

impl EfficiencyPressureRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("surface", &self.surface)?;
        validate_required("pressure_level", &self.pressure_level)?;
        validate_required("outcome_floor", &self.outcome_floor)?;
        validate_required("public_claim_status", &self.public_claim_status)?;
        validate_required_vec("related_spend_categories", &self.related_spend_categories)?;
        validate_required_vec("pressure_basis", &self.pressure_basis)?;
        validate_required_vec("cost_down_levers", &self.cost_down_levers)?;
        validate_required_vec("evidence_needed", &self.evidence_needed)?;

        if self.record_family != "efficiency_pressure" {
            return Err(format!(
                "efficiency pressure record_family must be efficiency_pressure, got {}",
                self.record_family
            ));
        }
        if self.fiscal_year != 2025 {
            return Err(format!(
                "efficiency pressure v1 only covers FY2025, got {}",
                self.fiscal_year
            ));
        }
        if !matches!(self.pressure_level.as_str(), "highest" | "high" | "watch") {
            return Err(format!(
                "unsupported efficiency pressure_level {}",
                self.pressure_level
            ));
        }
        if !self.not_a_finding {
            return Err("efficiency pressure rows must remain not_a_finding=true".to_string());
        }
        if self.public_claim_status != "blocked_question_surface_only" {
            return Err(format!(
                "efficiency pressure public_claim_status must be blocked_question_surface_only, got {}",
                self.public_claim_status
            ));
        }
        if self.pressure_basis.len() < 2 {
            return Err(
                "efficiency pressure rows need at least two pressure-basis entries".to_string(),
            );
        }
        if self.cost_down_levers.len() < 3 {
            return Err(
                "efficiency pressure rows need at least three cost-down levers".to_string(),
            );
        }
        if self.evidence_needed.len() < 3 {
            return Err("efficiency pressure rows need at least three evidence needs".to_string());
        }
        if !contains_any_case_insensitive(
            &self.outcome_floor,
            &["preserve", "must remain", "must not", "must"],
        ) {
            return Err("efficiency pressure rows must carry an outcome floor".to_string());
        }
        let public_text = format!(
            "{} {} {} {}",
            self.surface,
            self.pressure_basis.join(" "),
            self.cost_down_levers.join(" "),
            self.outcome_floor
        )
        .to_ascii_lowercase();
        for blocked_phrase in [
            "fraud finding",
            "waste finding",
            "abuse finding",
            "found fraud",
            "found waste",
            "proves waste",
        ] {
            if public_text.contains(blocked_phrase) {
                return Err(format!(
                    "efficiency pressure rows must not make finding claims: {blocked_phrase}"
                ));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CostDownBacklogRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_pressure_record_id: String,
    pub lane_id: String,
    pub lever_id: String,
    pub lever_label: String,
    pub lever_type: String,
    pub action_question: String,
    pub required_evidence: Vec<String>,
    pub measurement_metric: String,
    pub outcome_floor: String,
    pub time_horizon: String,
    pub estimated_savings_usd: Option<f64>,
    pub savings_claim_status: String,
    pub public_use_rule: String,
}

impl CostDownBacklogRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("source_pressure_record_id", &self.source_pressure_record_id)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("lever_id", &self.lever_id)?;
        validate_required("lever_label", &self.lever_label)?;
        validate_required("lever_type", &self.lever_type)?;
        validate_required("action_question", &self.action_question)?;
        validate_required_vec("required_evidence", &self.required_evidence)?;
        validate_required("measurement_metric", &self.measurement_metric)?;
        validate_required("outcome_floor", &self.outcome_floor)?;
        validate_required("time_horizon", &self.time_horizon)?;
        validate_required("savings_claim_status", &self.savings_claim_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "cost_down_backlog" {
            return Err(format!(
                "cost-down backlog record_family must be cost_down_backlog, got {}",
                self.record_family
            ));
        }
        if !self
            .source_pressure_record_id
            .starts_with("efficiency-pressure:")
        {
            return Err(
                "cost-down backlog rows must point to an efficiency pressure record".to_string(),
            );
        }
        if !matches!(
            self.lever_type.as_str(),
            "price_discipline"
                | "administrative_simplification"
                | "procurement_control"
                | "fiscal_balance"
                | "risk_mitigation"
                | "payment_integrity"
        ) {
            return Err(format!(
                "unsupported cost-down lever_type {}",
                self.lever_type
            ));
        }
        if !matches!(
            self.time_horizon.as_str(),
            "near_term" | "medium_term" | "long_term"
        ) {
            return Err(format!(
                "unsupported cost-down time_horizon {}",
                self.time_horizon
            ));
        }
        if self.estimated_savings_usd.is_some() {
            return Err(
                "cost-down backlog rows must not publish savings estimates yet".to_string(),
            );
        }
        if self.savings_claim_status != "blocked_no_estimate" {
            return Err(format!(
                "cost-down savings_claim_status must be blocked_no_estimate, got {}",
                self.savings_claim_status
            ));
        }
        if self.required_evidence.len() < 2 {
            return Err(
                "cost-down backlog rows need at least two evidence requirements".to_string(),
            );
        }
        if !contains_any_case_insensitive(
            &self.outcome_floor,
            &["preserve", "must remain", "must not", "maintain", "improve"],
        ) {
            return Err("cost-down backlog rows must carry an outcome floor".to_string());
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "cost-down backlog public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CostDownSourcePacketRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_backlog_record_id: String,
    pub source_pressure_record_id: String,
    pub lane_id: String,
    pub packet_status: String,
    pub source_ids: Vec<String>,
    pub evidence_summary: Vec<String>,
    pub metric_candidates: Vec<String>,
    pub outcome_floor_checks: Vec<String>,
    pub missing_before_estimate: Vec<String>,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl CostDownSourcePacketRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("source_backlog_record_id", &self.source_backlog_record_id)?;
        validate_required("source_pressure_record_id", &self.source_pressure_record_id)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("packet_status", &self.packet_status)?;
        validate_required_vec("source_ids", &self.source_ids)?;
        validate_required_vec("evidence_summary", &self.evidence_summary)?;
        validate_required_vec("metric_candidates", &self.metric_candidates)?;
        validate_required_vec("outcome_floor_checks", &self.outcome_floor_checks)?;
        validate_required_vec("missing_before_estimate", &self.missing_before_estimate)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "cost_down_source_packet" {
            return Err(format!(
                "cost-down source packet record_family must be cost_down_source_packet, got {}",
                self.record_family
            ));
        }
        if !self.source_backlog_record_id.starts_with("cost-down:") {
            return Err(
                "cost-down source packets must point to a cost-down backlog row".to_string(),
            );
        }
        if !self
            .source_pressure_record_id
            .starts_with("efficiency-pressure:")
        {
            return Err(
                "cost-down source packets must point to an efficiency pressure row".to_string(),
            );
        }
        if self.packet_status != "reviewed_source_packet_no_savings_estimate" {
            return Err(format!(
                "cost-down source packet status must be reviewed_source_packet_no_savings_estimate, got {}",
                self.packet_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "cost-down source packets must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        if self.source_ids.len() < 3 {
            return Err("cost-down source packets need at least three source IDs".to_string());
        }
        if self.metric_candidates.len() < 2 {
            return Err("cost-down source packets need at least two metric candidates".to_string());
        }
        if self.outcome_floor_checks.len() < 2 {
            return Err(
                "cost-down source packets need at least two outcome-floor checks".to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not a savings estimate")
            && public_rule.contains("not a finding")
            && public_rule.contains("source packet"))
        {
            return Err(
                "cost-down source packet public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CostDownEvidenceQueueRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_packet_record_id: String,
    pub source_backlog_record_id: String,
    pub source_pressure_record_id: String,
    pub lane_id: String,
    pub extraction_priority: String,
    pub primary_source_ids: Vec<String>,
    pub extract_question: String,
    pub first_extract: String,
    pub extract_grain: String,
    pub query_lock_fields: Vec<String>,
    pub output_artifact_candidate: String,
    pub scoring_blockers: Vec<String>,
    pub outcome_floor: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl CostDownEvidenceQueueRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("source_packet_record_id", &self.source_packet_record_id)?;
        validate_required("source_backlog_record_id", &self.source_backlog_record_id)?;
        validate_required("source_pressure_record_id", &self.source_pressure_record_id)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("extraction_priority", &self.extraction_priority)?;
        validate_required_vec("primary_source_ids", &self.primary_source_ids)?;
        validate_required("extract_question", &self.extract_question)?;
        validate_required("first_extract", &self.first_extract)?;
        validate_required("extract_grain", &self.extract_grain)?;
        validate_required_vec("query_lock_fields", &self.query_lock_fields)?;
        validate_required("output_artifact_candidate", &self.output_artifact_candidate)?;
        validate_required_vec("scoring_blockers", &self.scoring_blockers)?;
        validate_required("outcome_floor", &self.outcome_floor)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "cost_down_evidence_queue" {
            return Err(format!(
                "cost-down evidence queue record_family must be cost_down_evidence_queue, got {}",
                self.record_family
            ));
        }
        if !self
            .source_packet_record_id
            .starts_with("cost-down-source-packet:")
        {
            return Err("evidence queue rows must point to a cost-down source packet".to_string());
        }
        if !self.source_backlog_record_id.starts_with("cost-down:") {
            return Err("evidence queue rows must point to a cost-down backlog row".to_string());
        }
        if !self
            .source_pressure_record_id
            .starts_with("efficiency-pressure:")
        {
            return Err("evidence queue rows must point to an efficiency pressure row".to_string());
        }
        if !matches!(
            self.extraction_priority.as_str(),
            "first_pass" | "follow_up" | "blocked"
        ) {
            return Err(format!(
                "unknown evidence queue extraction_priority {}",
                self.extraction_priority
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "evidence queue rows must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }

        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "evidence queue public_use_rule must block savings and finding claims".to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityPortalProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub observed_date: String,
    pub page_url: String,
    pub row_kind: String,
    pub agency_code: String,
    pub agency_name: String,
    pub high_priority_program_count: u16,
    pub improper_payment_percentage: f64,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityPortalProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("page_url", &self.page_url)?;
        validate_required("row_kind", &self.row_kind)?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("agency_name", &self.agency_name)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_portal_probe" {
            return Err(format!(
                "payment integrity portal probe record_family must be payment_integrity_portal_probe, got {}",
                self.record_family
            ));
        }
        if !self
            .source_evidence_queue_record_id
            .starts_with("cost-down-evidence-queue:payment-integrity:")
        {
            return Err(
                "payment integrity portal probes must point to a payment-integrity evidence queue row"
                    .to_string(),
            );
        }
        if self.source_id != "SRC-OMB-PAYMENTACCURACY" {
            return Err(format!(
                "payment integrity portal probe source_id must be SRC-OMB-PAYMENTACCURACY, got {}",
                self.source_id
            ));
        }
        if !matches!(
            self.row_kind.as_str(),
            "homepage_highest_performing_agency" | "homepage_lowest_performing_agency"
        ) {
            return Err(format!(
                "unsupported payment integrity portal probe row_kind {}",
                self.row_kind
            ));
        }
        if self.improper_payment_percentage < 0.0 || self.improper_payment_percentage > 100.0 {
            return Err(
                "payment integrity portal probe improper_payment_percentage must be 0..100"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity portal probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity portal probe public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityScorecardProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub observed_date: String,
    pub scorecard_url: String,
    pub reporting_period: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub fy2024_overpayment_amount_millions: f64,
    pub fy2024_overpayment_rate_percent: f64,
    pub sample_period_note: String,
    pub primary_root_cause_amount_millions: f64,
    pub root_cause_control_scope: String,
    pub root_cause_data_access_issue: String,
    pub mitigation_strategy: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityScorecardProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("scorecard_url", &self.scorecard_url)?;
        validate_required("reporting_period", &self.reporting_period)?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("sample_period_note", &self.sample_period_note)?;
        validate_required("root_cause_control_scope", &self.root_cause_control_scope)?;
        validate_required(
            "root_cause_data_access_issue",
            &self.root_cause_data_access_issue,
        )?;
        validate_required("mitigation_strategy", &self.mitigation_strategy)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_scorecard_probe" {
            return Err(format!(
                "payment integrity scorecard probe record_family must be payment_integrity_scorecard_probe, got {}",
                self.record_family
            ));
        }
        if self.source_id != "SRC-OMB-PAYMENTACCURACY" {
            return Err(format!(
                "payment integrity scorecard probe source_id must be SRC-OMB-PAYMENTACCURACY, got {}",
                self.source_id
            ));
        }
        if self.reporting_period != "Q4 2025" {
            return Err(format!(
                "payment integrity scorecard probe reporting_period must be Q4 2025, got {}",
                self.reporting_period
            ));
        }
        if self.fy2024_overpayment_amount_millions < 0.0
            || self.primary_root_cause_amount_millions < 0.0
        {
            return Err("payment integrity scorecard amounts must be non-negative".to_string());
        }
        if self.fy2024_overpayment_rate_percent < 0.0
            || self.fy2024_overpayment_rate_percent > 100.0
        {
            return Err("payment integrity scorecard overpayment rate must be 0..100".to_string());
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity scorecard probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity scorecard probe public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityProgramReviewGateRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_scorecard_record_id: String,
    pub source_readiness_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub reporting_period: String,
    pub fy2024_overpayment_amount_millions: f64,
    pub fy2024_overpayment_rate_percent: f64,
    pub methodology_status: String,
    pub access_floor_status: String,
    pub corrective_action_status: String,
    pub confidence_limit_status: String,
    pub claim_boundary_status: String,
    pub required_next_evidence: Vec<String>,
    pub review_gate_status: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityProgramReviewGateRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_scorecard_record_id",
            &self.source_scorecard_record_id,
        )?;
        validate_required(
            "source_readiness_record_id",
            &self.source_readiness_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("reporting_period", &self.reporting_period)?;
        validate_required("methodology_status", &self.methodology_status)?;
        validate_required("access_floor_status", &self.access_floor_status)?;
        validate_required("corrective_action_status", &self.corrective_action_status)?;
        validate_required("confidence_limit_status", &self.confidence_limit_status)?;
        validate_required("claim_boundary_status", &self.claim_boundary_status)?;
        validate_required("review_gate_status", &self.review_gate_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_program_review_gate" {
            return Err(format!(
                "payment integrity program review gate record_family must be payment_integrity_program_review_gate, got {}",
                self.record_family
            ));
        }
        if self.reporting_period != "Q4 2025" {
            return Err(format!(
                "payment integrity program review gate reporting_period must be Q4 2025, got {}",
                self.reporting_period
            ));
        }
        if self.fy2024_overpayment_amount_millions < 0.0
            || self.fy2024_overpayment_rate_percent < 0.0
            || self.fy2024_overpayment_rate_percent > 100.0
        {
            return Err(
                "payment integrity program review gate amounts/rates must be non-negative and rates 0..100"
                    .to_string(),
            );
        }
        if self.required_next_evidence.len() < 4 {
            return Err(
                "payment integrity program review gate must list methodology, access, corrective-action, and confidence evidence needs"
                    .to_string(),
            );
        }
        let evidence_text = self.required_next_evidence.join(" ").to_ascii_lowercase();
        for required in ["methodology", "access", "corrective", "confidence"] {
            if !evidence_text.contains(required) {
                return Err(format!(
                    "payment integrity program review gate required_next_evidence must include {required}"
                ));
            }
        }
        if self.review_gate_status != "blocked_before_savings_score" {
            return Err(format!(
                "payment integrity program review gate review_gate_status must be blocked_before_savings_score, got {}",
                self.review_gate_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity program review gates must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity program review gate public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityProgramReviewTaskRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_program_gate_record_id: String,
    pub source_scorecard_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub evidence_family: String,
    pub extraction_task: String,
    pub target_source_or_system: String,
    pub completion_gate: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityProgramReviewTaskRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_program_gate_record_id",
            &self.source_program_gate_record_id,
        )?;
        validate_required(
            "source_scorecard_record_id",
            &self.source_scorecard_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("evidence_family", &self.evidence_family)?;
        validate_required("extraction_task", &self.extraction_task)?;
        validate_required("target_source_or_system", &self.target_source_or_system)?;
        validate_required("completion_gate", &self.completion_gate)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_program_review_task" {
            return Err(format!(
                "payment integrity program review task record_family must be payment_integrity_program_review_task, got {}",
                self.record_family
            ));
        }
        if ![
            "methodology",
            "access_floor",
            "corrective_action",
            "confidence_limits",
        ]
        .contains(&self.evidence_family.as_str())
        {
            return Err(format!(
                "payment integrity program review task evidence_family is unsupported: {}",
                self.evidence_family
            ));
        }
        if self.completion_gate != "required_before_savings_score" {
            return Err(format!(
                "payment integrity program review task completion_gate must be required_before_savings_score, got {}",
                self.completion_gate
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity program review tasks must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity program review task public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityProgramReviewStatusRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_program_gate_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub total_required_task_count: u8,
    pub completed_task_count: u8,
    pub blocked_task_count: u8,
    pub blocker_summary: String,
    pub next_priority_task_family: String,
    pub next_priority_reason: String,
    pub review_status: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityProgramReviewStatusRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_program_gate_record_id",
            &self.source_program_gate_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("blocker_summary", &self.blocker_summary)?;
        validate_required("next_priority_task_family", &self.next_priority_task_family)?;
        validate_required("next_priority_reason", &self.next_priority_reason)?;
        validate_required("review_status", &self.review_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_program_review_status" {
            return Err(format!(
                "payment integrity program review status record_family must be payment_integrity_program_review_status, got {}",
                self.record_family
            ));
        }
        if self.total_required_task_count != 4 {
            return Err(format!(
                "payment integrity program review status total_required_task_count must be 4, got {}",
                self.total_required_task_count
            ));
        }
        if self.completed_task_count + self.blocked_task_count != self.total_required_task_count {
            return Err(
                "payment integrity program review status completed + blocked tasks must equal total"
                    .to_string(),
            );
        }
        if ![
            "methodology",
            "access_floor",
            "corrective_action",
            "confidence_limits",
        ]
        .contains(&self.next_priority_task_family.as_str())
        {
            return Err(format!(
                "payment integrity program review status next_priority_task_family is unsupported: {}",
                self.next_priority_task_family
            ));
        }
        if self.review_status != "blocked_before_savings_score" {
            return Err(format!(
                "payment integrity program review status review_status must be blocked_before_savings_score, got {}",
                self.review_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity program review status must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity program review status public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyPlanRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_program_status_record_id: String,
    pub source_methodology_task_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub required_methodology_fields: Vec<String>,
    pub source_discovery_targets: Vec<String>,
    pub extraction_priority: u8,
    pub methodology_completion_rule: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyPlanRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_program_status_record_id",
            &self.source_program_status_record_id,
        )?;
        validate_required(
            "source_methodology_task_record_id",
            &self.source_methodology_task_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required(
            "methodology_completion_rule",
            &self.methodology_completion_rule,
        )?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_plan" {
            return Err(format!(
                "payment integrity methodology plan record_family must be payment_integrity_methodology_plan, got {}",
                self.record_family
            ));
        }
        if self.extraction_priority == 0 {
            return Err(
                "payment integrity methodology plan extraction_priority must be positive"
                    .to_string(),
            );
        }
        if self.required_methodology_fields.len() < 6 {
            return Err(
                "payment integrity methodology plan must list at least six required methodology fields"
                    .to_string(),
            );
        }
        let fields = self
            .required_methodology_fields
            .join(" ")
            .to_ascii_lowercase();
        for required in [
            "sample",
            "universe",
            "method",
            "exclusion",
            "period",
            "payment",
        ] {
            if !fields.contains(required) {
                return Err(format!(
                    "payment integrity methodology plan required fields must include {required}"
                ));
            }
        }
        if self.source_discovery_targets.len() < 2 {
            return Err(
                "payment integrity methodology plan must list at least two source discovery targets"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology plans must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology plan public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFieldRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_plan_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub field_status: String,
    pub required_source_target: String,
    pub completion_rule: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFieldRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_plan_record_id",
            &self.source_methodology_plan_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("field_status", &self.field_status)?;
        validate_required("required_source_target", &self.required_source_target)?;
        validate_required("completion_rule", &self.completion_rule)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_field" {
            return Err(format!(
                "payment integrity methodology field record_family must be payment_integrity_methodology_field, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.field_status.as_str(),
            "open_source_needed" | "open_reframing_and_source_needed"
        ) {
            return Err(format!(
                "payment integrity methodology field status is unsupported: {}",
                self.field_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology fields must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology field public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologySourceTargetRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_plan_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target: String,
    pub target_priority: u8,
    pub target_status: String,
    pub target_use: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologySourceTargetRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_plan_record_id",
            &self.source_methodology_plan_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("source_target", &self.source_target)?;
        validate_required("target_status", &self.target_status)?;
        validate_required("target_use", &self.target_use)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_source_target" {
            return Err(format!(
                "payment integrity methodology source target record_family must be payment_integrity_methodology_source_target, got {}",
                self.record_family
            ));
        }
        if self.target_priority == 0 {
            return Err(
                "payment integrity methodology source target priority must be positive".to_string(),
            );
        }
        if !matches!(
            self.target_status.as_str(),
            "open_source_needed" | "source_captured"
        ) {
            return Err(format!(
                "payment integrity methodology source target status must be open_source_needed or source_captured, got {}",
                self.target_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology source targets must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology source target public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyQueryRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_target_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub query_text: String,
    pub query_scope: String,
    pub capture_rule: String,
    pub query_status: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyQueryRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_target_record_id",
            &self.source_methodology_target_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("query_text", &self.query_text)?;
        validate_required("query_scope", &self.query_scope)?;
        validate_required("capture_rule", &self.capture_rule)?;
        validate_required("query_status", &self.query_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_query" {
            return Err(format!(
                "payment integrity methodology query record_family must be payment_integrity_methodology_query, got {}",
                self.record_family
            ));
        }
        if !matches!(self.query_status.as_str(), "open_not_executed" | "executed") {
            return Err(format!(
                "payment integrity methodology query status must be open_not_executed or executed, got {}",
                self.query_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology queries must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology query public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyQueryRunRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_query_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub run_status: String,
    pub planned_query_text: String,
    pub result_capture_status: String,
    pub required_capture_fields: Vec<String>,
    pub next_run_rule: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyQueryRunRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_query_record_id",
            &self.source_methodology_query_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("run_status", &self.run_status)?;
        validate_required("planned_query_text", &self.planned_query_text)?;
        validate_required("result_capture_status", &self.result_capture_status)?;
        validate_required("next_run_rule", &self.next_run_rule)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_query_run" {
            return Err(format!(
                "payment integrity methodology query run record_family must be payment_integrity_methodology_query_run, got {}",
                self.record_family
            ));
        }
        let valid_run_capture_pair = matches!(
            (
                self.run_status.as_str(),
                self.result_capture_status.as_str()
            ),
            ("pending_not_run", "no_result_captured") | ("executed", "methodology_result_captured")
        );
        if !valid_run_capture_pair {
            return Err(format!(
                "payment integrity methodology query run status/capture pair is invalid: {}/{}",
                self.run_status, self.result_capture_status
            ));
        }
        if self.required_capture_fields.len() < 4 {
            return Err(
                "payment integrity methodology query run must list at least four required capture fields"
                    .to_string(),
            );
        }
        let capture_fields = self.required_capture_fields.join(" ").to_ascii_lowercase();
        for required in ["url", "observed", "title", "field"] {
            if !capture_fields.contains(required) {
                return Err(format!(
                    "payment integrity methodology query run required capture fields must include {required}"
                ));
            }
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology query runs must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology query run public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyResultRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_query_run_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub observed_date: String,
    pub source_url: String,
    pub source_title: String,
    pub reporting_period: String,
    pub captured_methodology_text: String,
    pub captured_field_scope: Vec<String>,
    pub field_closure_allowed: bool,
    pub result_status: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyResultRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_query_run_record_id",
            &self.source_methodology_query_run_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_url", &self.source_url)?;
        validate_required("source_title", &self.source_title)?;
        validate_required("reporting_period", &self.reporting_period)?;
        validate_required("captured_methodology_text", &self.captured_methodology_text)?;
        validate_required("result_status", &self.result_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_result" {
            return Err(format!(
                "payment integrity methodology result record_family must be payment_integrity_methodology_result, got {}",
                self.record_family
            ));
        }
        if self.captured_field_scope.is_empty() {
            return Err(
                "payment integrity methodology result must list captured field scope".to_string(),
            );
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology result must not close fields until reviewed"
                    .to_string(),
            );
        }
        if self.result_status != "source_captured_review_needed" {
            return Err(format!(
                "payment integrity methodology result status must be source_captured_review_needed, got {}",
                self.result_status
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology results must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology result public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyResultReviewReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_result_record_ids: Vec<String>,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_capture_count: u8,
    pub review_readiness_status: String,
    pub next_field_review_count: u8,
    pub next_methodology_fields: Vec<String>,
    pub next_action: String,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyResultReviewReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required_vec(
            "source_methodology_result_record_ids",
            &self.source_methodology_result_record_ids,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("review_readiness_status", &self.review_readiness_status)?;
        validate_required("next_action", &self.next_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_result_review_readiness" {
            return Err(format!(
                "payment integrity methodology result review readiness record_family must be payment_integrity_methodology_result_review_readiness, got {}",
                self.record_family
            ));
        }
        if self.source_capture_count as usize != self.source_methodology_result_record_ids.len() {
            return Err(
                "payment integrity methodology result review readiness source count must match source IDs"
                    .to_string(),
            );
        }
        if self.review_readiness_status != "ready_for_field_review_queue" {
            return Err(format!(
                "payment integrity methodology result review readiness status must be ready_for_field_review_queue, got {}",
                self.review_readiness_status
            ));
        }
        if self.next_field_review_count as usize != self.next_methodology_fields.len() {
            return Err(
                "payment integrity methodology result review readiness field count must match next fields"
                    .to_string(),
            );
        }
        if self.field_closure_allowed || self.public_claim_allowed || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology result review readiness must block field closure, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology result review readiness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFieldReviewRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_result_record_id: String,
    pub source_methodology_field_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub evidence_status: String,
    pub reviewed_source_scope: String,
    pub review_note: String,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFieldReviewRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_result_record_id",
            &self.source_methodology_result_record_id,
        )?;
        validate_required(
            "source_methodology_field_record_id",
            &self.source_methodology_field_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("evidence_status", &self.evidence_status)?;
        validate_required("reviewed_source_scope", &self.reviewed_source_scope)?;
        validate_required("review_note", &self.review_note)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_field_review" {
            return Err(format!(
                "payment integrity methodology field review record_family must be payment_integrity_methodology_field_review, got {}",
                self.record_family
            ));
        }
        if ![
            "partial_support_review_needed",
            "closure_support_review_needed",
            "not_supported_by_result",
            "field_reframing_review_needed",
        ]
        .contains(&self.evidence_status.as_str())
        {
            return Err(format!(
                "payment integrity methodology field review evidence_status is unsupported: {}",
                self.evidence_status
            ));
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology field review must not close fields".to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology field reviews must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology field review public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyGapFollowupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_field_review_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub gap_class: String,
    pub followup_priority: u8,
    pub source_target: String,
    pub next_action: String,
    pub completion_evidence_required: Vec<String>,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyGapFollowupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_field_review_record_id",
            &self.source_methodology_field_review_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("gap_class", &self.gap_class)?;
        validate_required("source_target", &self.source_target)?;
        validate_required("next_action", &self.next_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_gap_followup" {
            return Err(format!(
                "payment integrity methodology gap followup record_family must be payment_integrity_methodology_gap_followup, got {}",
                self.record_family
            ));
        }
        if ![
            "unsupported_field_source_needed",
            "partial_support_citation_needed",
            "closure_support_captured_review_needed",
            "field_reframing_and_detail_source_needed",
        ]
        .contains(&self.gap_class.as_str())
        {
            return Err(format!(
                "payment integrity methodology gap followup gap_class is unsupported: {}",
                self.gap_class
            ));
        }
        if !(1..=8).contains(&self.followup_priority) {
            return Err(format!(
                "payment integrity methodology gap followup priority must be 1-8, got {}",
                self.followup_priority
            ));
        }
        if self.completion_evidence_required.is_empty() {
            return Err(
                "payment integrity methodology gap followup must list completion evidence"
                    .to_string(),
            );
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology gap followup must not close fields".to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology gap followups must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology gap followup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyGapSourceCaptureRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_gap_followup_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub observed_date: String,
    pub source_url: String,
    pub source_title: String,
    pub source_publisher: String,
    pub captured_source_scope: String,
    pub captured_methodology_summary: String,
    pub support_status: String,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyGapSourceCaptureRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_gap_followup_record_id",
            &self.source_methodology_gap_followup_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_url", &self.source_url)?;
        validate_required("source_title", &self.source_title)?;
        validate_required("source_publisher", &self.source_publisher)?;
        validate_required("captured_source_scope", &self.captured_source_scope)?;
        validate_required(
            "captured_methodology_summary",
            &self.captured_methodology_summary,
        )?;
        validate_required("support_status", &self.support_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_gap_source_capture" {
            return Err(format!(
                "payment integrity methodology gap source capture record_family must be payment_integrity_methodology_gap_source_capture, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.support_status.as_str(),
            "partial_support_review_needed"
                | "closure_support_review_needed"
                | "field_reframing_review_needed"
        ) {
            return Err(format!(
                "payment integrity methodology gap source capture support_status must be partial_support_review_needed or closure_support_review_needed, got {}",
                self.support_status
            ));
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology gap source capture must not close fields"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology gap source captures must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology gap source capture public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologySourceCaptureRollupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_gap_followup_record_id: String,
    pub source_methodology_gap_source_capture_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub capture_coverage_status: String,
    pub remaining_review_need: String,
    pub reviewer_action: String,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologySourceCaptureRollupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_gap_followup_record_id",
            &self.source_methodology_gap_followup_record_id,
        )?;
        validate_required(
            "source_methodology_gap_source_capture_record_id",
            &self.source_methodology_gap_source_capture_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("capture_coverage_status", &self.capture_coverage_status)?;
        validate_required("remaining_review_need", &self.remaining_review_need)?;
        validate_required("reviewer_action", &self.reviewer_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_source_capture_rollup" {
            return Err(format!(
                "payment integrity methodology source capture rollup record_family must be payment_integrity_methodology_source_capture_rollup, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.capture_coverage_status.as_str(),
            "source_captured_review_needed" | "field_reframing_supported_full_treatment_open"
        ) {
            return Err(format!(
                "payment integrity methodology source capture rollup status must be source_captured_review_needed, got {}",
                self.capture_coverage_status
            ));
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology source capture rollup must not close fields"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology source capture rollups must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology source capture rollup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyClosureReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_source_capture_rollup_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub closure_readiness_status: String,
    pub readiness_reason: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyClosureReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_source_capture_rollup_record_id",
            &self.source_methodology_source_capture_rollup_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("closure_readiness_status", &self.closure_readiness_status)?;
        validate_required("readiness_reason", &self.readiness_reason)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_closure_readiness" {
            return Err(format!(
                "payment integrity methodology closure readiness record_family must be payment_integrity_methodology_closure_readiness, got {}",
                self.record_family
            ));
        }
        if !["closure_review_candidate", "additional_source_needed"]
            .contains(&self.closure_readiness_status.as_str())
        {
            return Err(format!(
                "payment integrity methodology closure readiness status is unsupported: {}",
                self.closure_readiness_status
            ));
        }
        if self.field_closure_allowed {
            return Err(
                "payment integrity methodology closure readiness must not close fields".to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology closure readiness rows must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology closure readiness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyClosureDecisionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_closure_readiness_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub decision_status: String,
    pub field_closed: bool,
    pub decision_basis: String,
    pub closure_scope: String,
    pub residual_limitations: Vec<String>,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyClosureDecisionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_closure_readiness_record_id",
            &self.source_methodology_closure_readiness_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("decision_status", &self.decision_status)?;
        validate_required("decision_basis", &self.decision_basis)?;
        validate_required("closure_scope", &self.closure_scope)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_closure_decision" {
            return Err(format!(
                "payment integrity methodology closure decision record_family must be payment_integrity_methodology_closure_decision, got {}",
                self.record_family
            ));
        }
        if self.decision_status != "field_closed_internal_only" {
            return Err(format!(
                "payment integrity methodology closure decision status must be field_closed_internal_only, got {}",
                self.decision_status
            ));
        }
        if !self.field_closed {
            return Err(
                "payment integrity methodology closure decision must set field_closed true"
                    .to_string(),
            );
        }
        if self.residual_limitations.is_empty() {
            return Err(
                "payment integrity methodology closure decision must list residual limitations"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology closure decisions must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology closure decision public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyResidualSourceGapRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_closure_readiness_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub methodology_field: String,
    pub residual_gap_class: String,
    pub source_need: String,
    pub next_query_text: String,
    pub closure_blocked_reason: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyResidualSourceGapRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_closure_readiness_record_id",
            &self.source_methodology_closure_readiness_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("methodology_field", &self.methodology_field)?;
        validate_required("residual_gap_class", &self.residual_gap_class)?;
        validate_required("source_need", &self.source_need)?;
        validate_required("next_query_text", &self.next_query_text)?;
        validate_required("closure_blocked_reason", &self.closure_blocked_reason)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_residual_source_gap" {
            return Err(format!(
                "payment integrity methodology residual source gap record_family must be payment_integrity_methodology_residual_source_gap, got {}",
                self.record_family
            ));
        }
        if ![
            "detail_source_needed",
            "current_year_source_needed",
            "reviewer_determination_needed",
            "field_reframing_and_detail_source_needed",
        ]
        .contains(&self.residual_gap_class.as_str())
        {
            return Err(format!(
                "payment integrity methodology residual source gap class is unsupported: {}",
                self.residual_gap_class
            ));
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology residual source gaps must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology residual source gap public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyClosureCoverageRecord {
    pub record_id: String,
    pub record_family: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_methodology_closure_decision_record_id: String,
    pub total_methodology_fields: u8,
    pub closed_field_count: u8,
    pub open_field_count: u8,
    pub closed_fields: Vec<String>,
    pub open_fields: Vec<String>,
    pub coverage_status: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyClosureCoverageRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required(
            "source_methodology_closure_decision_record_id",
            &self.source_methodology_closure_decision_record_id,
        )?;
        validate_required("coverage_status", &self.coverage_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_closure_coverage" {
            return Err(format!(
                "payment integrity methodology closure coverage record_family must be payment_integrity_methodology_closure_coverage, got {}",
                self.record_family
            ));
        }
        if self.total_methodology_fields != 8 {
            return Err(format!(
                "payment integrity methodology closure coverage must have 8 total fields, got {}",
                self.total_methodology_fields
            ));
        }
        if self.closed_field_count as usize != self.closed_fields.len()
            || self.open_field_count as usize != self.open_fields.len()
            || self.closed_field_count + self.open_field_count != self.total_methodology_fields
        {
            return Err(
                "payment integrity methodology closure coverage field counts are inconsistent"
                    .to_string(),
            );
        }
        if self.coverage_status != "partial_methodology_closure" {
            return Err(format!(
                "payment integrity methodology closure coverage status must be partial_methodology_closure, got {}",
                self.coverage_status
            ));
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology closure coverage must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology closure coverage public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyScoringGateRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_closure_coverage_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub gate_status: String,
    pub gate_reason: String,
    pub blockers: Vec<String>,
    pub next_milestone: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyScoringGateRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_closure_coverage_record_id",
            &self.source_methodology_closure_coverage_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("gate_status", &self.gate_status)?;
        validate_required("gate_reason", &self.gate_reason)?;
        validate_required("next_milestone", &self.next_milestone)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_scoring_gate" {
            return Err(format!(
                "payment integrity methodology scoring gate record_family must be payment_integrity_methodology_scoring_gate, got {}",
                self.record_family
            ));
        }
        if self.gate_status != "blocked_methodology_incomplete" {
            return Err(format!(
                "payment integrity methodology scoring gate status must be blocked_methodology_incomplete, got {}",
                self.gate_status
            ));
        }
        if self.blockers.is_empty() {
            return Err(
                "payment integrity methodology scoring gate must list blockers".to_string(),
            );
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology scoring gate must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology scoring gate public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyProgramRollupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_scoring_gate_record_id: String,
    pub source_methodology_closure_coverage_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub total_methodology_fields: u8,
    pub closed_field_count: u8,
    pub open_field_count: u8,
    pub scoring_gate_status: String,
    pub next_open_methodology_fields: Vec<String>,
    pub next_action: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyProgramRollupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_scoring_gate_record_id",
            &self.source_methodology_scoring_gate_record_id,
        )?;
        validate_required(
            "source_methodology_closure_coverage_record_id",
            &self.source_methodology_closure_coverage_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("scoring_gate_status", &self.scoring_gate_status)?;
        validate_required("next_action", &self.next_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_program_rollup" {
            return Err(format!(
                "payment integrity methodology program rollup record_family must be payment_integrity_methodology_program_rollup, got {}",
                self.record_family
            ));
        }
        if self.total_methodology_fields != 8 {
            return Err(format!(
                "payment integrity methodology program rollup must have 8 total fields, got {}",
                self.total_methodology_fields
            ));
        }
        if self.open_field_count as usize != self.next_open_methodology_fields.len()
            || self.closed_field_count + self.open_field_count != self.total_methodology_fields
        {
            return Err(
                "payment integrity methodology program rollup field counts are inconsistent"
                    .to_string(),
            );
        }
        if self.scoring_gate_status != "blocked_methodology_incomplete" {
            return Err(format!(
                "payment integrity methodology program rollup scoring_gate_status must be blocked_methodology_incomplete, got {}",
                self.scoring_gate_status
            ));
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology program rollup must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology program rollup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyOpenProgramStatusRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_methodology_plan_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub closure_path_status: String,
    pub total_methodology_fields: u8,
    pub closed_field_count: u8,
    pub open_field_count: u8,
    pub closure_decision_count: u8,
    pub residual_source_gap_count: u8,
    pub blocker_summary: String,
    pub next_priority: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyOpenProgramStatusRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_methodology_plan_record_id",
            &self.source_methodology_plan_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("closure_path_status", &self.closure_path_status)?;
        validate_required("blocker_summary", &self.blocker_summary)?;
        validate_required("next_priority", &self.next_priority)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_open_program_status" {
            return Err(format!(
                "payment integrity methodology open program status record_family must be payment_integrity_methodology_open_program_status, got {}",
                self.record_family
            ));
        }
        if self.total_methodology_fields != 8 {
            return Err(format!(
                "payment integrity methodology open program status must have 8 total fields, got {}",
                self.total_methodology_fields
            ));
        }
        if self.closed_field_count + self.open_field_count != self.total_methodology_fields {
            return Err(
                "payment integrity methodology open program status field counts are inconsistent"
                    .to_string(),
            );
        }
        if self.closure_decision_count != self.closed_field_count {
            return Err(
                "payment integrity methodology open program status closure decisions must match closed field count"
                    .to_string(),
            );
        }
        if self.residual_source_gap_count != self.open_field_count {
            return Err(
                "payment integrity methodology open program status residual gaps must match open field count"
                    .to_string(),
            );
        }
        match self.closure_path_status.as_str() {
            "closure_coverage_available" => {
                if self.closed_field_count == 0 {
                    return Err(
                        "closure_coverage_available status requires at least one closed field"
                            .to_string(),
                    );
                }
            }
            "fully_open_no_closure_decision" => {
                if self.closed_field_count != 0 {
                    return Err(
                        "fully_open_no_closure_decision status requires zero closed fields"
                            .to_string(),
                    );
                }
            }
            _ => {
                return Err(format!(
                    "payment integrity methodology open program status closure_path_status is unsupported: {}",
                    self.closure_path_status
                ));
            }
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology open program status must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology open program status public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyResidualGapPriorityRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_open_program_status_record_id: String,
    pub source_residual_source_gap_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub selected_methodology_field: String,
    pub priority_reason: String,
    pub next_query_text: String,
    pub resolution_rule: String,
    pub blocked_claims_note: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyResidualGapPriorityRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_open_program_status_record_id",
            &self.source_open_program_status_record_id,
        )?;
        validate_required(
            "source_residual_source_gap_record_id",
            &self.source_residual_source_gap_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required(
            "selected_methodology_field",
            &self.selected_methodology_field,
        )?;
        validate_required("priority_reason", &self.priority_reason)?;
        validate_required("next_query_text", &self.next_query_text)?;
        validate_required("resolution_rule", &self.resolution_rule)?;
        validate_required("blocked_claims_note", &self.blocked_claims_note)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_residual_gap_priority" {
            return Err(format!(
                "payment integrity methodology residual gap priority record_family must be payment_integrity_methodology_residual_gap_priority, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology residual gap priority rank must be positive"
                    .to_string(),
            );
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology residual gap priority must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology residual gap priority public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }
        let blocked_note = self.blocked_claims_note.to_ascii_lowercase();
        if !(blocked_note.contains("scor")
            && blocked_note.contains("savings")
            && blocked_note.contains("waste"))
        {
            return Err(
                "payment integrity methodology residual gap priority blocked_claims_note must block scoring, savings, and waste claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyPrioritySourceWorkRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_residual_gap_priority_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub selected_methodology_field: String,
    pub observed_date: String,
    pub source_work_status: String,
    pub official_source_urls: Vec<String>,
    pub source_summary: String,
    pub resolution_effect: String,
    pub remaining_blocker: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyPrioritySourceWorkRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_residual_gap_priority_record_id",
            &self.source_residual_gap_priority_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required(
            "selected_methodology_field",
            &self.selected_methodology_field,
        )?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_work_status", &self.source_work_status)?;
        validate_required("source_summary", &self.source_summary)?;
        validate_required("resolution_effect", &self.resolution_effect)?;
        validate_required("remaining_blocker", &self.remaining_blocker)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_priority_source_work" {
            return Err(format!(
                "payment integrity methodology priority source work record_family must be payment_integrity_methodology_priority_source_work, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology priority source work rank must be positive"
                    .to_string(),
            );
        }
        match self.source_work_status.as_str() {
            "reviewer_resolution_ready"
            | "source_captured_review_needed"
            | "boundary_source_captured_review_needed"
            | "partial_recovery_process_support_review_needed"
            | "resolved_by_subsequent_same_period_source" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology priority source work status is unsupported: {}",
                    self.source_work_status
                ));
            }
        }
        if self.official_source_urls.is_empty() {
            return Err(
                "payment integrity methodology priority source work requires at least one official source URL"
                    .to_string(),
            );
        }
        if !self
            .official_source_urls
            .iter()
            .all(|url| url.contains(".gov") || url.contains("paymentaccuracy.gov"))
        {
            return Err(
                "payment integrity methodology priority source work source URLs must be official .gov or PaymentAccuracy sources"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology priority source work must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology priority source work public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyPriorityReviewerActionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_priority_source_work_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub selected_methodology_field: String,
    pub reviewer_action_status: String,
    pub reviewer_action: String,
    pub field_reframing_allowed: bool,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub next_required_artifact: String,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyPriorityReviewerActionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_priority_source_work_record_id",
            &self.source_priority_source_work_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required(
            "selected_methodology_field",
            &self.selected_methodology_field,
        )?;
        validate_required("reviewer_action_status", &self.reviewer_action_status)?;
        validate_required("reviewer_action", &self.reviewer_action)?;
        validate_required("next_required_artifact", &self.next_required_artifact)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_priority_reviewer_action" {
            return Err(format!(
                "payment integrity methodology priority reviewer action record_family must be payment_integrity_methodology_priority_reviewer_action, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology priority reviewer action rank must be positive"
                    .to_string(),
            );
        }
        match self.reviewer_action_status.as_str() {
            "field_reframing_approved_internal_only" => {
                if !self.field_reframing_allowed {
                    return Err(
                        "field_reframing_approved_internal_only requires field_reframing_allowed"
                            .to_string(),
                    );
                }
            }
            "additional_source_work_required" => {
                if self.field_reframing_allowed {
                    return Err(
                        "additional_source_work_required must not allow field reframing"
                            .to_string(),
                    );
                }
            }
            _ => {
                return Err(format!(
                    "payment integrity methodology priority reviewer action status is unsupported: {}",
                    self.reviewer_action_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology priority reviewer action must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology priority reviewer action public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFieldUpdateRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_priority_reviewer_action_record_id: String,
    pub source_methodology_field_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub old_methodology_field: String,
    pub revised_methodology_field: String,
    pub old_required_source_target: String,
    pub revised_required_source_target: String,
    pub old_completion_rule: String,
    pub revised_completion_rule: String,
    pub update_status: String,
    pub update_scope: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFieldUpdateRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_priority_reviewer_action_record_id",
            &self.source_priority_reviewer_action_record_id,
        )?;
        validate_required(
            "source_methodology_field_record_id",
            &self.source_methodology_field_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("old_methodology_field", &self.old_methodology_field)?;
        validate_required("revised_methodology_field", &self.revised_methodology_field)?;
        validate_required(
            "old_required_source_target",
            &self.old_required_source_target,
        )?;
        validate_required(
            "revised_required_source_target",
            &self.revised_required_source_target,
        )?;
        validate_required("old_completion_rule", &self.old_completion_rule)?;
        validate_required("revised_completion_rule", &self.revised_completion_rule)?;
        validate_required("update_status", &self.update_status)?;
        validate_required("update_scope", &self.update_scope)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_field_update" {
            return Err(format!(
                "payment integrity methodology field update record_family must be payment_integrity_methodology_field_update, got {}",
                self.record_family
            ));
        }
        if self.update_status != "field_reframed_internal_only" {
            return Err(format!(
                "payment integrity methodology field update status must be field_reframed_internal_only, got {}",
                self.update_status
            ));
        }
        if self.old_methodology_field == self.revised_methodology_field {
            return Err(
                "payment integrity methodology field update must change the methodology field"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology field update must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology field update public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupSourceQueryRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_priority_reviewer_action_record_id: String,
    pub source_field_update_record_id: Option<String>,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub query_objective: String,
    pub query_text: String,
    pub source_scope: String,
    pub capture_rule: String,
    pub success_rule: String,
    pub query_status: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupSourceQueryRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_priority_reviewer_action_record_id",
            &self.source_priority_reviewer_action_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("query_objective", &self.query_objective)?;
        validate_required("query_text", &self.query_text)?;
        validate_required("source_scope", &self.source_scope)?;
        validate_required("capture_rule", &self.capture_rule)?;
        validate_required("success_rule", &self.success_rule)?;
        validate_required("query_status", &self.query_status)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_source_query" {
            return Err(format!(
                "payment integrity methodology followup source query record_family must be payment_integrity_methodology_followup_source_query, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup source query rank must be positive"
                    .to_string(),
            );
        }
        if self.query_status != "open_not_executed" {
            return Err(format!(
                "payment integrity methodology followup source query status must be open_not_executed, got {}",
                self.query_status
            ));
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology followup source query must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup source query public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupSourceQueryRunRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_source_query_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub run_status: String,
    pub planned_query_text: String,
    pub result_capture_status: String,
    pub required_capture_fields: Vec<String>,
    pub next_run_rule: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupSourceQueryRunRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_source_query_record_id",
            &self.source_followup_source_query_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("run_status", &self.run_status)?;
        validate_required("planned_query_text", &self.planned_query_text)?;
        validate_required("result_capture_status", &self.result_capture_status)?;
        validate_required("next_run_rule", &self.next_run_rule)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_source_query_run" {
            return Err(format!(
                "payment integrity methodology followup source query run record_family must be payment_integrity_methodology_followup_source_query_run, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup source query run rank must be positive"
                    .to_string(),
            );
        }
        if self.run_status != "pending_not_run" {
            return Err(format!(
                "payment integrity methodology followup source query run status must be pending_not_run, got {}",
                self.run_status
            ));
        }
        if self.result_capture_status != "no_result_captured" {
            return Err(format!(
                "payment integrity methodology followup source query run result_capture_status must be no_result_captured, got {}",
                self.result_capture_status
            ));
        }
        if self.required_capture_fields.is_empty() {
            return Err(
                "payment integrity methodology followup source query run requires capture fields"
                    .to_string(),
            );
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity methodology followup source query run must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup source query run public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupSourceCaptureRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_source_query_run_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub observed_date: String,
    pub source_url: String,
    pub source_title: String,
    pub captured_source_scope: String,
    pub captured_boundary_summary: String,
    pub recoverability_boundary_status: String,
    pub closure_effect: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupSourceCaptureRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_source_query_run_record_id",
            &self.source_followup_source_query_run_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_url", &self.source_url)?;
        validate_required("source_title", &self.source_title)?;
        validate_required("captured_source_scope", &self.captured_source_scope)?;
        validate_required("captured_boundary_summary", &self.captured_boundary_summary)?;
        validate_required(
            "recoverability_boundary_status",
            &self.recoverability_boundary_status,
        )?;
        validate_required("closure_effect", &self.closure_effect)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_source_capture" {
            return Err(format!(
                "payment integrity methodology followup source capture record_family must be payment_integrity_methodology_followup_source_capture, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup source capture rank must be positive"
                    .to_string(),
            );
        }
        match self.recoverability_boundary_status.as_str() {
            "partial_recovery_process_support_review_needed"
            | "boundary_block_support_review_needed"
            | "recoverability_mapping_partial_review_needed" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology followup source capture recoverability status is unsupported: {}",
                    self.recoverability_boundary_status
                ));
            }
        }
        if !(self.source_url.contains(".gov") || self.source_url.contains("paymentaccuracy.gov")) {
            return Err(
                "payment integrity methodology followup source capture source_url must be official .gov or PaymentAccuracy source"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology followup source capture must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup source capture public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_source_capture_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub capture_rollup_status: String,
    pub boundary_finding: String,
    pub remaining_review_need: String,
    pub reviewer_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_source_capture_record_id",
            &self.source_followup_source_capture_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("capture_rollup_status", &self.capture_rollup_status)?;
        validate_required("boundary_finding", &self.boundary_finding)?;
        validate_required("remaining_review_need", &self.remaining_review_need)?;
        validate_required("reviewer_action", &self.reviewer_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_source_capture_rollup" {
            return Err(format!(
                "payment integrity methodology followup source capture rollup record_family must be payment_integrity_methodology_followup_source_capture_rollup, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup source capture rollup rank must be positive"
                    .to_string(),
            );
        }
        match self.capture_rollup_status.as_str() {
            "reviewer_boundary_decision_needed" | "additional_positive_basis_needed" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology followup source capture rollup status is unsupported: {}",
                    self.capture_rollup_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology followup source capture rollup must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup source capture rollup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_source_capture_rollup_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub boundary_decision_status: String,
    pub boundary_decision: String,
    pub scoring_implication: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_source_capture_rollup_record_id",
            &self.source_followup_source_capture_rollup_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("boundary_decision_status", &self.boundary_decision_status)?;
        validate_required("boundary_decision", &self.boundary_decision)?;
        validate_required("scoring_implication", &self.scoring_implication)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_boundary_decision" {
            return Err(format!(
                "payment integrity methodology followup boundary decision record_family must be payment_integrity_methodology_followup_boundary_decision, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup boundary decision rank must be positive"
                    .to_string(),
            );
        }
        match self.boundary_decision_status.as_str() {
            "narrow_boundary_supported_internal_only"
            | "claim_guard_confirmed_internal_only"
            | "additional_positive_basis_required" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology followup boundary decision status is unsupported: {}",
                    self.boundary_decision_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology followup boundary decision must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup boundary decision public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_boundary_decision_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub boundary_readiness_status: String,
    pub readiness_scope: String,
    pub readiness_reason: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_boundary_decision_record_id",
            &self.source_followup_boundary_decision_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("boundary_readiness_status", &self.boundary_readiness_status)?;
        validate_required("readiness_scope", &self.readiness_scope)?;
        validate_required("readiness_reason", &self.readiness_reason)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_followup_boundary_readiness" {
            return Err(format!(
                "payment integrity methodology followup boundary readiness record_family must be payment_integrity_methodology_followup_boundary_readiness, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology followup boundary readiness rank must be positive"
                    .to_string(),
            );
        }
        match self.boundary_readiness_status.as_str() {
            "narrow_internal_readiness_candidate" | "additional_positive_basis_needed" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology followup boundary readiness status is unsupported: {}",
                    self.boundary_readiness_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology followup boundary readiness must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology followup boundary readiness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyNarrowClosureCandidateRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_followup_boundary_readiness_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub candidate_scope: String,
    pub candidate_basis: String,
    pub excluded_scoring_basis: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyNarrowClosureCandidateRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_followup_boundary_readiness_record_id",
            &self.source_followup_boundary_readiness_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("candidate_scope", &self.candidate_scope)?;
        validate_required("candidate_basis", &self.candidate_basis)?;
        validate_required("excluded_scoring_basis", &self.excluded_scoring_basis)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_narrow_closure_candidate" {
            return Err(format!(
                "payment integrity methodology narrow closure candidate record_family must be payment_integrity_methodology_narrow_closure_candidate, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology narrow closure candidate rank must be positive"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology narrow closure candidate must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology narrow closure candidate public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyNarrowClosureDecisionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_narrow_closure_candidate_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub priority_rank: u8,
    pub narrow_decision_status: String,
    pub closed_component: String,
    pub decision_basis: String,
    pub excluded_scope: String,
    pub residual_open_need: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyNarrowClosureDecisionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_narrow_closure_candidate_record_id",
            &self.source_narrow_closure_candidate_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("narrow_decision_status", &self.narrow_decision_status)?;
        validate_required("closed_component", &self.closed_component)?;
        validate_required("decision_basis", &self.decision_basis)?;
        validate_required("excluded_scope", &self.excluded_scope)?;
        validate_required("residual_open_need", &self.residual_open_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_narrow_closure_decision" {
            return Err(format!(
                "payment integrity methodology narrow closure decision record_family must be payment_integrity_methodology_narrow_closure_decision, got {}",
                self.record_family
            ));
        }
        if self.priority_rank == 0 {
            return Err(
                "payment integrity methodology narrow closure decision rank must be positive"
                    .to_string(),
            );
        }
        if self.narrow_decision_status != "component_closed_internal_only" {
            return Err(format!(
                "payment integrity methodology narrow closure decision status must be component_closed_internal_only, got {}",
                self.narrow_decision_status
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology narrow closure decision must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology narrow closure decision public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyOpenProgramComponentProgressRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_open_program_status_record_id: String,
    pub source_narrow_closure_decision_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub component_progress_status: String,
    pub total_methodology_fields: u8,
    pub closed_field_count_after_component_decision: u8,
    pub open_field_count_after_component_decision: u8,
    pub narrow_component_decision_count: u8,
    pub component_progress_summary: String,
    pub unchanged_field_count_reason: String,
    pub next_gate_condition: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyOpenProgramComponentProgressRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_open_program_status_record_id",
            &self.source_open_program_status_record_id,
        )?;
        validate_required(
            "source_narrow_closure_decision_record_id",
            &self.source_narrow_closure_decision_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("component_progress_status", &self.component_progress_status)?;
        validate_required(
            "component_progress_summary",
            &self.component_progress_summary,
        )?;
        validate_required(
            "unchanged_field_count_reason",
            &self.unchanged_field_count_reason,
        )?;
        validate_required("next_gate_condition", &self.next_gate_condition)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_open_program_component_progress" {
            return Err(format!(
                "payment integrity methodology open program component progress record_family must be payment_integrity_methodology_open_program_component_progress, got {}",
                self.record_family
            ));
        }
        if !matches!(
            self.component_progress_status.as_str(),
            "narrow_component_recorded_no_field_closure"
                | "narrow_sample_design_component_added_full_field_counts_unchanged"
                | "full_field_closure_added_after_narrow_component"
        ) {
            return Err(format!(
                "payment integrity methodology open program component progress status is unsupported: {}",
                self.component_progress_status
            ));
        }
        if self.total_methodology_fields != 8 {
            return Err(format!(
                "payment integrity methodology open program component progress must have 8 total fields, got {}",
                self.total_methodology_fields
            ));
        }
        if self.closed_field_count_after_component_decision
            + self.open_field_count_after_component_decision
            != self.total_methodology_fields
        {
            return Err(
                "payment integrity methodology open program component progress field counts are inconsistent"
                    .to_string(),
            );
        }
        if self.narrow_component_decision_count == 0 {
            return Err(
                "payment integrity methodology open program component progress must record at least one narrow component decision"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology open program component progress must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology open program component progress public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateRequirementRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_progress_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub gate_status: String,
    pub required_positive_evidence: String,
    pub blocked_translation: String,
    pub next_source_target: String,
    pub next_decision_type: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateRequirementRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_progress_record_id",
            &self.source_component_progress_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("gate_status", &self.gate_status)?;
        validate_required(
            "required_positive_evidence",
            &self.required_positive_evidence,
        )?;
        validate_required("blocked_translation", &self.blocked_translation)?;
        validate_required("next_source_target", &self.next_source_target)?;
        validate_required("next_decision_type", &self.next_decision_type)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_requirement" {
            return Err(format!(
                "payment integrity methodology component gate requirement record_family must be payment_integrity_methodology_component_gate_requirement, got {}",
                self.record_family
            ));
        }
        let valid_requirement_state = matches!(
            (self.gate_status.as_str(), self.next_decision_type.as_str()),
            (
                "positive_evidence_required_before_field_closure",
                "full_field_closure_review"
            ) | (
                "narrow_component_evidence_required_full_field_remains_open",
                "narrow_component_closure_review_only"
            )
        );
        if !valid_requirement_state {
            return Err(format!(
                "payment integrity methodology component gate requirement status and next_decision_type are unsupported: {} / {}",
                self.gate_status, self.next_decision_type
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate requirement must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate requirement public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateSourceTargetRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_requirement_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub source_target_name: String,
    pub source_target_scope: String,
    pub evidence_to_extract: Vec<String>,
    pub negative_evidence_rule: String,
    pub next_artifact_family: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateSourceTargetRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_requirement_record_id",
            &self.source_component_gate_requirement_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("source_target_name", &self.source_target_name)?;
        validate_required("source_target_scope", &self.source_target_scope)?;
        validate_required("negative_evidence_rule", &self.negative_evidence_rule)?;
        validate_required("next_artifact_family", &self.next_artifact_family)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_source_target" {
            return Err(format!(
                "payment integrity methodology component gate source target record_family must be payment_integrity_methodology_component_gate_source_target, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate source target priority must be positive"
                    .to_string(),
            );
        }
        if self.evidence_to_extract.is_empty() {
            return Err(
                "payment integrity methodology component gate source target must list evidence to extract"
                    .to_string(),
            );
        }
        if self.next_artifact_family != "payment_integrity_methodology_component_gate_source_query"
        {
            return Err(format!(
                "payment integrity methodology component gate source target next_artifact_family must be payment_integrity_methodology_component_gate_source_query, got {}",
                self.next_artifact_family
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate source target must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate source target public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateSourceQueryRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_source_target_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub query_text: String,
    pub query_scope: String,
    pub expected_evidence: Vec<String>,
    pub insufficient_result_rule: String,
    pub next_artifact_family: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateSourceQueryRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_source_target_record_id",
            &self.source_component_gate_source_target_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("query_text", &self.query_text)?;
        validate_required("query_scope", &self.query_scope)?;
        validate_required("insufficient_result_rule", &self.insufficient_result_rule)?;
        validate_required("next_artifact_family", &self.next_artifact_family)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_source_query" {
            return Err(format!(
                "payment integrity methodology component gate source query record_family must be payment_integrity_methodology_component_gate_source_query, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate source query priority must be positive"
                    .to_string(),
            );
        }
        if self.expected_evidence.is_empty() {
            return Err(
                "payment integrity methodology component gate source query must list expected evidence"
                    .to_string(),
            );
        }
        if self.next_artifact_family
            != "payment_integrity_methodology_component_gate_source_query_run"
        {
            return Err(format!(
                "payment integrity methodology component gate source query next_artifact_family must be payment_integrity_methodology_component_gate_source_query_run, got {}",
                self.next_artifact_family
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate source query must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate source query public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_source_query_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub run_status: String,
    pub planned_query_text: String,
    pub result_capture_status: String,
    pub required_capture_fields: Vec<String>,
    pub next_run_rule: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_source_query_record_id",
            &self.source_component_gate_source_query_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("run_status", &self.run_status)?;
        validate_required("planned_query_text", &self.planned_query_text)?;
        validate_required("result_capture_status", &self.result_capture_status)?;
        validate_required("next_run_rule", &self.next_run_rule)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_source_query_run" {
            return Err(format!(
                "payment integrity methodology component gate source query run record_family must be payment_integrity_methodology_component_gate_source_query_run, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate source query run priority must be positive"
                    .to_string(),
            );
        }
        let valid_run_state = matches!(
            (
                self.run_status.as_str(),
                self.result_capture_status.as_str()
            ),
            ("pending_not_run", "no_result_captured")
                | ("executed", "component_gate_source_captured")
        );
        if !valid_run_state {
            return Err(format!(
                "payment integrity methodology component gate source query run status pair is unsupported: {} / {}",
                self.run_status, self.result_capture_status
            ));
        }
        if self.required_capture_fields.is_empty() {
            return Err(
                "payment integrity methodology component gate source query run requires capture fields"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate source query run must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate source query run public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateSourceCaptureRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_source_query_run_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub observed_date: String,
    pub source_url: String,
    pub source_title: String,
    pub captured_source_scope: String,
    pub captured_gate_summary: String,
    pub component_gate_status: String,
    pub next_review_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateSourceCaptureRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_source_query_run_record_id",
            &self.source_component_gate_source_query_run_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_url", &self.source_url)?;
        validate_required("source_title", &self.source_title)?;
        validate_required("captured_source_scope", &self.captured_source_scope)?;
        validate_required("captured_gate_summary", &self.captured_gate_summary)?;
        validate_required("component_gate_status", &self.component_gate_status)?;
        validate_required("next_review_action", &self.next_review_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_source_capture" {
            return Err(format!(
                "payment integrity methodology component gate source capture record_family must be payment_integrity_methodology_component_gate_source_capture, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate source capture priority must be positive"
                    .to_string(),
            );
        }
        match self.component_gate_status.as_str() {
            "partial_positive_basis_review_needed"
            | "context_only_no_positive_amount_basis"
            | "category_split_partial_review_needed"
            | "narrow_sample_design_governance_component_supported" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology component gate source capture status is unsupported: {}",
                    self.component_gate_status
                ));
            }
        }
        if !(self.source_url.contains(".gov") || self.source_url.contains("paymentaccuracy.gov")) {
            return Err(
                "payment integrity methodology component gate source capture source_url must be official .gov or PaymentAccuracy source"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate source capture must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate source capture public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_source_capture_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub capture_rollup_status: String,
    pub gate_finding: String,
    pub remaining_review_need: String,
    pub reviewer_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_source_capture_record_id",
            &self.source_component_gate_source_capture_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("capture_rollup_status", &self.capture_rollup_status)?;
        validate_required("gate_finding", &self.gate_finding)?;
        validate_required("remaining_review_need", &self.remaining_review_need)?;
        validate_required("reviewer_action", &self.reviewer_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family
            != "payment_integrity_methodology_component_gate_source_capture_rollup"
        {
            return Err(format!(
                "payment integrity methodology component gate source capture rollup record_family must be payment_integrity_methodology_component_gate_source_capture_rollup, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate source capture rollup priority must be positive"
                    .to_string(),
            );
        }
        match self.capture_rollup_status.as_str() {
            "reviewer_gate_decision_needed"
            | "additional_positive_basis_needed"
            | "narrow_component_review_ready_full_field_open" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology component gate source capture rollup status is unsupported: {}",
                    self.capture_rollup_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate source capture rollup must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate source capture rollup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_source_capture_rollup_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub boundary_decision_status: String,
    pub boundary_decision: String,
    pub scoring_implication: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_source_capture_rollup_record_id",
            &self.source_component_gate_source_capture_rollup_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("boundary_decision_status", &self.boundary_decision_status)?;
        validate_required("boundary_decision", &self.boundary_decision)?;
        validate_required("scoring_implication", &self.scoring_implication)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_boundary_decision" {
            return Err(format!(
                "payment integrity methodology component gate boundary decision record_family must be payment_integrity_methodology_component_gate_boundary_decision, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate boundary decision priority must be positive"
                    .to_string(),
            );
        }
        match self.boundary_decision_status.as_str() {
            "narrow_process_boundary_supported_internal_only"
            | "additional_positive_basis_required"
            | "narrow_component_only_full_field_blocked" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology component gate boundary decision status is unsupported: {}",
                    self.boundary_decision_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate boundary decision must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate boundary decision public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_boundary_decision_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub boundary_readiness_status: String,
    pub readiness_scope: String,
    pub readiness_reason: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_boundary_decision_record_id",
            &self.source_component_gate_boundary_decision_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("boundary_readiness_status", &self.boundary_readiness_status)?;
        validate_required("readiness_scope", &self.readiness_scope)?;
        validate_required("readiness_reason", &self.readiness_reason)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_boundary_readiness" {
            return Err(format!(
                "payment integrity methodology component gate boundary readiness record_family must be payment_integrity_methodology_component_gate_boundary_readiness, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate boundary readiness priority must be positive"
                    .to_string(),
            );
        }
        match self.boundary_readiness_status.as_str() {
            "narrow_internal_readiness_candidate"
            | "additional_positive_basis_needed"
            | "narrow_component_candidate_ready_full_field_open" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology component gate boundary readiness status is unsupported: {}",
                    self.boundary_readiness_status
                ));
            }
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate boundary readiness must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate boundary readiness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_boundary_readiness_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub candidate_status: String,
    pub candidate_scope: String,
    pub candidate_basis: String,
    pub excluded_scoring_basis: String,
    pub next_required_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_boundary_readiness_record_id",
            &self.source_component_gate_boundary_readiness_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("candidate_status", &self.candidate_status)?;
        validate_required("candidate_scope", &self.candidate_scope)?;
        validate_required("candidate_basis", &self.candidate_basis)?;
        validate_required("excluded_scoring_basis", &self.excluded_scoring_basis)?;
        validate_required("next_required_action", &self.next_required_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_narrow_candidate" {
            return Err(format!(
                "payment integrity methodology component gate narrow candidate record_family must be payment_integrity_methodology_component_gate_narrow_candidate, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate narrow candidate priority must be positive"
                    .to_string(),
            );
        }
        if self.candidate_status != "narrow_component_candidate_internal_only" {
            return Err(format!(
                "payment integrity methodology component gate narrow candidate status is unsupported: {}",
                self.candidate_status
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate narrow candidate must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate narrow candidate public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_narrow_candidate_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub narrow_decision_status: String,
    pub closed_component: String,
    pub decision_basis: String,
    pub excluded_scope: String,
    pub residual_open_need: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_narrow_candidate_record_id",
            &self.source_component_gate_narrow_candidate_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("narrow_decision_status", &self.narrow_decision_status)?;
        validate_required("closed_component", &self.closed_component)?;
        validate_required("decision_basis", &self.decision_basis)?;
        validate_required("excluded_scope", &self.excluded_scope)?;
        validate_required("residual_open_need", &self.residual_open_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_narrow_decision" {
            return Err(format!(
                "payment integrity methodology component gate narrow decision record_family must be payment_integrity_methodology_component_gate_narrow_decision, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate narrow decision priority must be positive"
                    .to_string(),
            );
        }
        if self.narrow_decision_status != "component_closed_internal_only" {
            return Err(format!(
                "payment integrity methodology component gate narrow decision status must be component_closed_internal_only, got {}",
                self.narrow_decision_status
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate narrow decision must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate narrow decision public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_open_program_status_record_id: String,
    pub source_component_gate_narrow_decision_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub component_progress_status: String,
    pub total_methodology_fields: u8,
    pub closed_field_count_after_component_decision: u8,
    pub open_field_count_after_component_decision: u8,
    pub component_gate_decision_count: u8,
    pub component_progress_summary: String,
    pub unchanged_field_count_reason: String,
    pub next_gate_condition: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_open_program_status_record_id",
            &self.source_open_program_status_record_id,
        )?;
        validate_required(
            "source_component_gate_narrow_decision_record_id",
            &self.source_component_gate_narrow_decision_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("component_progress_status", &self.component_progress_status)?;
        validate_required(
            "component_progress_summary",
            &self.component_progress_summary,
        )?;
        validate_required(
            "unchanged_field_count_reason",
            &self.unchanged_field_count_reason,
        )?;
        validate_required("next_gate_condition", &self.next_gate_condition)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_progress" {
            return Err(format!(
                "payment integrity methodology component gate progress record_family must be payment_integrity_methodology_component_gate_progress, got {}",
                self.record_family
            ));
        }
        if self.component_progress_status != "component_gate_progress_recorded_no_field_closure" {
            return Err(format!(
                "payment integrity methodology component gate progress status must be component_gate_progress_recorded_no_field_closure, got {}",
                self.component_progress_status
            ));
        }
        if self.total_methodology_fields != 8 {
            return Err(format!(
                "payment integrity methodology component gate progress must have 8 total fields, got {}",
                self.total_methodology_fields
            ));
        }
        if self.closed_field_count_after_component_decision
            + self.open_field_count_after_component_decision
            != self.total_methodology_fields
        {
            return Err(
                "payment integrity methodology component gate progress field counts are inconsistent"
                    .to_string(),
            );
        }
        if self.component_gate_decision_count == 0 {
            return Err(
                "payment integrity methodology component gate progress must record at least one component gate decision"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressRequirementRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_progress_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub gate_status: String,
    pub required_positive_evidence: String,
    pub blocked_translation: String,
    pub next_source_target: String,
    pub next_decision_type: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressRequirementRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_progress_record_id",
            &self.source_component_gate_progress_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("gate_status", &self.gate_status)?;
        validate_required(
            "required_positive_evidence",
            &self.required_positive_evidence,
        )?;
        validate_required("blocked_translation", &self.blocked_translation)?;
        validate_required("next_source_target", &self.next_source_target)?;
        validate_required("next_decision_type", &self.next_decision_type)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_methodology_component_gate_progress_requirement"
        {
            return Err(format!(
                "payment integrity methodology component gate progress requirement record_family must be payment_integrity_methodology_component_gate_progress_requirement, got {}",
                self.record_family
            ));
        }
        if self.gate_status != "positive_evidence_required_before_field_closure" {
            return Err(format!(
                "payment integrity methodology component gate progress requirement status must be positive_evidence_required_before_field_closure, got {}",
                self.gate_status
            ));
        }
        if self.next_decision_type != "full_field_closure_review" {
            return Err(format!(
                "payment integrity methodology component gate progress requirement next_decision_type must be full_field_closure_review, got {}",
                self.next_decision_type
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress requirement must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress requirement public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_progress_requirement_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub source_target_name: String,
    pub source_target_scope: String,
    pub evidence_to_extract: Vec<String>,
    pub negative_evidence_rule: String,
    pub next_artifact_family: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_progress_requirement_record_id",
            &self.source_component_gate_progress_requirement_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("source_target_name", &self.source_target_name)?;
        validate_required("source_target_scope", &self.source_target_scope)?;
        validate_required("negative_evidence_rule", &self.negative_evidence_rule)?;
        validate_required("next_artifact_family", &self.next_artifact_family)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family
            != "payment_integrity_methodology_component_gate_progress_source_target"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source target record_family must be payment_integrity_methodology_component_gate_progress_source_target, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate progress source target priority must be positive"
                    .to_string(),
            );
        }
        if self.evidence_to_extract.is_empty() {
            return Err(
                "payment integrity methodology component gate progress source target must list evidence to extract"
                    .to_string(),
            );
        }
        if self.next_artifact_family
            != "payment_integrity_methodology_component_gate_progress_source_query"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source target next_artifact_family must be payment_integrity_methodology_component_gate_progress_source_query, got {}",
                self.next_artifact_family
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress source target must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress source target public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_progress_source_target_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub query_text: String,
    pub query_scope: String,
    pub expected_evidence: Vec<String>,
    pub insufficient_result_rule: String,
    pub next_artifact_family: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_progress_source_target_record_id",
            &self.source_component_gate_progress_source_target_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("query_text", &self.query_text)?;
        validate_required("query_scope", &self.query_scope)?;
        validate_required("insufficient_result_rule", &self.insufficient_result_rule)?;
        validate_required("next_artifact_family", &self.next_artifact_family)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family
            != "payment_integrity_methodology_component_gate_progress_source_query"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source query record_family must be payment_integrity_methodology_component_gate_progress_source_query, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate progress source query priority must be positive"
                    .to_string(),
            );
        }
        if self.expected_evidence.is_empty() {
            return Err(
                "payment integrity methodology component gate progress source query must list expected evidence"
                    .to_string(),
            );
        }
        if self.next_artifact_family
            != "payment_integrity_methodology_component_gate_progress_source_query_run"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source query next_artifact_family must be payment_integrity_methodology_component_gate_progress_source_query_run, got {}",
                self.next_artifact_family
            ));
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress source query must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress source query public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_progress_source_query_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub run_status: String,
    pub planned_query_text: String,
    pub result_capture_status: String,
    pub required_capture_fields: Vec<String>,
    pub next_run_rule: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_progress_source_query_record_id",
            &self.source_component_gate_progress_source_query_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("run_status", &self.run_status)?;
        validate_required("planned_query_text", &self.planned_query_text)?;
        validate_required("result_capture_status", &self.result_capture_status)?;
        validate_required("next_run_rule", &self.next_run_rule)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family
            != "payment_integrity_methodology_component_gate_progress_source_query_run"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source query run record_family must be payment_integrity_methodology_component_gate_progress_source_query_run, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate progress source query run priority must be positive"
                    .to_string(),
            );
        }
        if self.run_status != "pending_not_run" {
            return Err(format!(
                "payment integrity methodology component gate progress source query run status must be pending_not_run, got {}",
                self.run_status
            ));
        }
        if self.result_capture_status != "no_result_captured" {
            return Err(format!(
                "payment integrity methodology component gate progress source query run result_capture_status must be no_result_captured, got {}",
                self.result_capture_status
            ));
        }
        if self.required_capture_fields.is_empty() {
            return Err(
                "payment integrity methodology component gate progress source query run requires capture fields"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress source query run must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress source query run public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityMethodologyComponentGateProgressSourceCaptureRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_component_gate_progress_source_query_run_record_id: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub source_target_priority: u8,
    pub observed_date: String,
    pub source_url: String,
    pub source_title: String,
    pub captured_source_scope: String,
    pub captured_gate_summary: String,
    pub component_gate_status: String,
    pub next_review_action: String,
    pub field_closure_allowed: bool,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityMethodologyComponentGateProgressSourceCaptureRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_component_gate_progress_source_query_run_record_id",
            &self.source_component_gate_progress_source_query_run_record_id,
        )?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("source_url", &self.source_url)?;
        validate_required("source_title", &self.source_title)?;
        validate_required("captured_source_scope", &self.captured_source_scope)?;
        validate_required("captured_gate_summary", &self.captured_gate_summary)?;
        validate_required("component_gate_status", &self.component_gate_status)?;
        validate_required("next_review_action", &self.next_review_action)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family
            != "payment_integrity_methodology_component_gate_progress_source_capture"
        {
            return Err(format!(
                "payment integrity methodology component gate progress source capture record_family must be payment_integrity_methodology_component_gate_progress_source_capture, got {}",
                self.record_family
            ));
        }
        if self.source_target_priority == 0 {
            return Err(
                "payment integrity methodology component gate progress source capture priority must be positive"
                    .to_string(),
            );
        }
        match self.component_gate_status.as_str() {
            "partial_positive_basis_review_needed"
            | "category_split_partial_review_needed"
            | "context_only_no_positive_amount_basis" => {}
            _ => {
                return Err(format!(
                    "payment integrity methodology component gate progress source capture status is unsupported: {}",
                    self.component_gate_status
                ));
            }
        }
        if !(self.source_url.contains(".gov") || self.source_url.contains("paymentaccuracy.gov")) {
            return Err(
                "payment integrity methodology component gate progress source capture source_url must be official .gov or PaymentAccuracy source"
                    .to_string(),
            );
        }
        if self.field_closure_allowed
            || self.scoring_allowed
            || self.public_claim_allowed
            || self.savings_estimate_allowed
        {
            return Err(
                "payment integrity methodology component gate progress source capture must block field closure, scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity methodology component gate progress source capture public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityNextProgramSelectionRecord {
    pub record_id: String,
    pub record_family: String,
    pub selected_program_key: String,
    pub agency_code: String,
    pub program_or_activity: String,
    pub selection_status: String,
    pub selection_reason: String,
    pub official_source_urls: Vec<String>,
    pub starting_methodology_fields: Vec<String>,
    pub next_artifact_family: String,
    pub scoring_allowed: bool,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityNextProgramSelectionRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("selected_program_key", &self.selected_program_key)?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("program_or_activity", &self.program_or_activity)?;
        validate_required("selection_status", &self.selection_status)?;
        validate_required("selection_reason", &self.selection_reason)?;
        validate_required("next_artifact_family", &self.next_artifact_family)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_next_program_selection" {
            return Err(format!(
                "payment integrity next program selection record_family must be payment_integrity_next_program_selection, got {}",
                self.record_family
            ));
        }
        if self.selection_status != "selected_for_methodology_planning" {
            return Err(format!(
                "payment integrity next program selection status must be selected_for_methodology_planning, got {}",
                self.selection_status
            ));
        }
        if self.official_source_urls.len() < 2 {
            return Err(
                "payment integrity next program selection must list at least two official source URLs"
                    .to_string(),
            );
        }
        if !self
            .official_source_urls
            .iter()
            .all(|url| url.starts_with("https://"))
        {
            return Err(
                "payment integrity next program selection source URLs must be https URLs"
                    .to_string(),
            );
        }
        if self.starting_methodology_fields.is_empty() {
            return Err(
                "payment integrity next program selection must list starting methodology fields"
                    .to_string(),
            );
        }
        if self.next_artifact_family != "payment_integrity_methodology_plan" {
            return Err(format!(
                "payment integrity next program selection next_artifact_family must be payment_integrity_methodology_plan, got {}",
                self.next_artifact_family
            ));
        }
        if self.scoring_allowed || self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "payment integrity next program selection must block scoring, public claims, and savings estimates"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "payment integrity next program selection public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PaymentIntegrityClaimsTimelinessProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub observed_date: String,
    pub page_url: String,
    pub agency_code: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub metric_unit: String,
    pub comparison_operator: String,
    pub metric_period: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl PaymentIntegrityClaimsTimelinessProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("page_url", &self.page_url)?;
        validate_required("agency_code", &self.agency_code)?;
        validate_required("metric_name", &self.metric_name)?;
        validate_required("metric_unit", &self.metric_unit)?;
        validate_required("comparison_operator", &self.comparison_operator)?;
        validate_required("metric_period", &self.metric_period)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "payment_integrity_claims_timeliness_probe" {
            return Err(format!(
                "payment integrity claims timeliness probe record_family must be payment_integrity_claims_timeliness_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:payment-integrity:claims-timeliness:v1"
        {
            return Err(
                "claims timeliness probes must point to the claims-timeliness evidence queue row"
                    .to_string(),
            );
        }
        if !matches!(
            self.source_id.as_str(),
            "SRC-SSA-PERFORMANCE" | "SRC-VA-CLAIMS-DATA"
        ) {
            return Err(format!(
                "unsupported claims timeliness source_id {}",
                self.source_id
            ));
        }
        if !matches!(
            self.comparison_operator.as_str(),
            "reported_value" | "less_than" | "improvement"
        ) {
            return Err(format!(
                "unsupported claims timeliness comparison_operator {}",
                self.comparison_operator
            ));
        }
        if self.metric_value < 0.0 {
            return Err("claims timeliness metric_value must be non-negative".to_string());
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "claims timeliness probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "claims timeliness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DebtMaturityRiskTreasuryProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub query_date: String,
    pub api_url: String,
    pub record_date: String,
    pub row_kind: String,
    pub security_type: String,
    pub security_description: String,
    pub debt_held_public_amount: Option<f64>,
    pub intragovernmental_holdings_amount: Option<f64>,
    pub total_public_debt_outstanding_amount: Option<f64>,
    pub average_interest_rate_percent: Option<f64>,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DebtMaturityRiskTreasuryProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("query_date", &self.query_date)?;
        validate_required("api_url", &self.api_url)?;
        validate_required("record_date", &self.record_date)?;
        validate_required("row_kind", &self.row_kind)?;
        validate_required("security_type", &self.security_type)?;
        validate_required("security_description", &self.security_description)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "debt_maturity_risk_treasury_probe" {
            return Err(format!(
                "debt maturity risk probe record_family must be debt_maturity_risk_treasury_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:debt-interest:maturity-risk:v1"
        {
            return Err(
                "debt maturity risk probes must point to the maturity-risk evidence queue row"
                    .to_string(),
            );
        }
        match self.source_id.as_str() {
            "SRC-TREASURY-DEBT-PENNY" => {
                if self.row_kind != "debt_stock" {
                    return Err("Debt to the Penny rows must use row_kind debt_stock".to_string());
                }
                validate_positive_option(
                    "debt_held_public_amount",
                    self.debt_held_public_amount,
                    "debt-stock probe rows",
                )?;
                validate_positive_option(
                    "intragovernmental_holdings_amount",
                    self.intragovernmental_holdings_amount,
                    "debt-stock probe rows",
                )?;
                validate_positive_option(
                    "total_public_debt_outstanding_amount",
                    self.total_public_debt_outstanding_amount,
                    "debt-stock probe rows",
                )?;
                if self.average_interest_rate_percent.is_some() {
                    return Err(
                        "debt-stock probe rows must not publish average interest rates".to_string(),
                    );
                }
            }
            "SRC-TREASURY-AVG-INTEREST" => {
                if self.row_kind != "average_interest_rate" {
                    return Err(
                        "Average Interest Rates rows must use row_kind average_interest_rate"
                            .to_string(),
                    );
                }
                validate_positive_option(
                    "average_interest_rate_percent",
                    self.average_interest_rate_percent,
                    "average-rate probe rows",
                )?;
                if self.debt_held_public_amount.is_some()
                    || self.intragovernmental_holdings_amount.is_some()
                    || self.total_public_debt_outstanding_amount.is_some()
                {
                    return Err(
                        "average-rate probe rows must not publish debt-stock amounts".to_string(),
                    );
                }
            }
            _ => {
                return Err(format!(
                    "unsupported debt maturity risk source_id {}",
                    self.source_id
                ));
            }
        }

        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "debt maturity risk probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "debt maturity risk public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DebtPrimaryBalanceFiscalProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub fiscal_year: u16,
    pub source_ids: Vec<String>,
    pub total_receipts_millions: f64,
    pub total_outlays_millions: f64,
    pub deficit_gap_millions: f64,
    pub gross_treasury_interest_outlays_millions: f64,
    pub primary_deficit_proxy_millions: f64,
    pub borrowed_share_percent_of_outlays: f64,
    pub income_tax_coverage_percent_of_outlays: f64,
    pub basis_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DebtPrimaryBalanceFiscalProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required_vec("source_ids", &self.source_ids)?;
        validate_required("basis_note", &self.basis_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "debt_primary_balance_fiscal_probe" {
            return Err(format!(
                "debt primary balance probe record_family must be debt_primary_balance_fiscal_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:debt-interest:primary-balance:v1"
        {
            return Err(
                "debt primary balance probes must point to the primary-balance evidence queue row"
                    .to_string(),
            );
        }
        if self.fiscal_year != 2025 {
            return Err(format!(
                "debt primary balance first-pass probe only covers FY2025, got {}",
                self.fiscal_year
            ));
        }
        for (field, value) in [
            ("total_receipts_millions", self.total_receipts_millions),
            ("total_outlays_millions", self.total_outlays_millions),
            ("deficit_gap_millions", self.deficit_gap_millions),
            (
                "gross_treasury_interest_outlays_millions",
                self.gross_treasury_interest_outlays_millions,
            ),
            (
                "primary_deficit_proxy_millions",
                self.primary_deficit_proxy_millions,
            ),
            (
                "borrowed_share_percent_of_outlays",
                self.borrowed_share_percent_of_outlays,
            ),
            (
                "income_tax_coverage_percent_of_outlays",
                self.income_tax_coverage_percent_of_outlays,
            ),
        ] {
            if value < 0.0 {
                return Err(format!("{field} must be non-negative"));
            }
        }
        if self.total_outlays_millions <= self.total_receipts_millions {
            return Err("FY2025 first-pass probe expects a deficit year".to_string());
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "debt primary balance probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "debt primary balance public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DisasterDeclarationProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub query_date: String,
    pub api_url: String,
    pub disaster_number: u32,
    pub declaration_date: String,
    pub incident_type: String,
    pub state: String,
    pub designated_area: String,
    pub declaration_title: String,
    pub ih_program_declared: bool,
    pub ia_program_declared: bool,
    pub pa_program_declared: bool,
    pub hm_program_declared: bool,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DisasterDeclarationProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("query_date", &self.query_date)?;
        validate_required("api_url", &self.api_url)?;
        validate_required("declaration_date", &self.declaration_date)?;
        validate_required("incident_type", &self.incident_type)?;
        validate_required("state", &self.state)?;
        validate_required("designated_area", &self.designated_area)?;
        validate_required("declaration_title", &self.declaration_title)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "disaster_declaration_probe" {
            return Err(format!(
                "disaster declaration probe record_family must be disaster_declaration_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:disaster-exposure:supplemental-tracking:v1"
        {
            return Err(
                "disaster declaration probes must point to the supplemental-tracking evidence queue row"
                    .to_string(),
            );
        }
        if self.source_id != "SRC-FEMA-DISASTER-DECLARATIONS" {
            return Err(format!(
                "disaster declaration probe source_id must be SRC-FEMA-DISASTER-DECLARATIONS, got {}",
                self.source_id
            ));
        }
        if self.disaster_number == 0 {
            return Err("disaster declaration probe disaster_number must be positive".to_string());
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "disaster declaration probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "disaster declaration public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DisasterMitigationProjectProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub query_date: String,
    pub api_url: String,
    pub project_identifier: String,
    pub program_area: String,
    pub program_fy: u16,
    pub state: String,
    pub county: String,
    pub disaster_number: Option<u32>,
    pub project_type: String,
    pub status: String,
    pub recipient: String,
    pub subrecipient: String,
    pub data_source: String,
    pub date_approved: Option<String>,
    pub date_closed: Option<String>,
    pub project_amount: Option<f64>,
    pub federal_share_obligated: Option<f64>,
    pub cost_share_percentage: Option<f64>,
    pub benefit_cost_ratio: Option<f64>,
    pub net_value_benefits: Option<f64>,
    pub number_of_properties: Option<u32>,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DisasterMitigationProjectProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("query_date", &self.query_date)?;
        validate_required("api_url", &self.api_url)?;
        validate_required("project_identifier", &self.project_identifier)?;
        validate_required("program_area", &self.program_area)?;
        validate_required("state", &self.state)?;
        validate_required("county", &self.county)?;
        validate_required("project_type", &self.project_type)?;
        validate_required("status", &self.status)?;
        validate_required("recipient", &self.recipient)?;
        validate_required("subrecipient", &self.subrecipient)?;
        validate_required("data_source", &self.data_source)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "disaster_mitigation_project_probe" {
            return Err(format!(
                "disaster mitigation project probe record_family must be disaster_mitigation_project_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:disaster-exposure:mitigation:v1"
        {
            return Err(
                "disaster mitigation project probes must point to the mitigation evidence queue row"
                    .to_string(),
            );
        }
        if self.source_id != "SRC-FEMA-HMA-PROJECTS" {
            return Err(format!(
                "disaster mitigation project probe source_id must be SRC-FEMA-HMA-PROJECTS, got {}",
                self.source_id
            ));
        }
        if self.program_fy == 0 {
            return Err(
                "disaster mitigation project probe program_fy must be positive".to_string(),
            );
        }
        for (field, value) in [
            ("project_amount", self.project_amount),
            ("federal_share_obligated", self.federal_share_obligated),
            ("cost_share_percentage", self.cost_share_percentage),
            ("benefit_cost_ratio", self.benefit_cost_ratio),
            ("net_value_benefits", self.net_value_benefits),
        ] {
            if let Some(value) = value {
                if value < 0.0 {
                    return Err(format!(
                        "disaster mitigation project probe {field} must be non-negative"
                    ));
                }
            }
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "disaster mitigation project probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "disaster mitigation project public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DefenseAuditControlProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub observed_date: String,
    pub report_url: String,
    pub report_number: String,
    pub fiscal_year: u16,
    pub finding_type: String,
    pub finding_identifier: String,
    pub finding_title: String,
    pub audit_opinion: Option<String>,
    pub material_weakness_count: Option<u32>,
    pub significant_deficiency_count: Option<u32>,
    pub noncompliance_count: Option<u32>,
    pub reported_amount_usd: Option<f64>,
    pub reported_amount_basis: Option<String>,
    pub affected_area: String,
    pub control_signal: String,
    pub recommendation_signal: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DefenseAuditControlProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("report_url", &self.report_url)?;
        validate_required("report_number", &self.report_number)?;
        validate_required("finding_type", &self.finding_type)?;
        validate_required("finding_identifier", &self.finding_identifier)?;
        validate_required("finding_title", &self.finding_title)?;
        validate_required("affected_area", &self.affected_area)?;
        validate_required("control_signal", &self.control_signal)?;
        validate_required("recommendation_signal", &self.recommendation_signal)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "defense_audit_control_probe" {
            return Err(format!(
                "defense audit control probe record_family must be defense_audit_control_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:defense:audit-control-closure:v1"
        {
            return Err(
                "defense audit control probes must point to the audit-control evidence queue row"
                    .to_string(),
            );
        }
        if self.source_id != "SRC-DODIG-FY2025-AUDIT" {
            return Err(format!(
                "defense audit control probe source_id must be SRC-DODIG-FY2025-AUDIT, got {}",
                self.source_id
            ));
        }
        if self.fiscal_year == 0 {
            return Err("defense audit control probe fiscal_year must be positive".to_string());
        }
        if let Some(amount) = self.reported_amount_usd {
            if amount < 0.0 {
                return Err(
                    "defense audit control probe reported_amount_usd must be non-negative"
                        .to_string(),
                );
            }
            if self
                .reported_amount_basis
                .as_deref()
                .unwrap_or("")
                .is_empty()
            {
                return Err(
                    "defense audit control probe reported_amount_basis is required when amount exists"
                        .to_string(),
                );
            }
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "defense audit control probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "defense audit control public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DefenseProcurementControlProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_id: String,
    pub observed_date: String,
    pub report_url: String,
    pub report_number: String,
    pub report_year: u16,
    pub program_or_portfolio: String,
    pub service_or_scope: String,
    pub acquisition_pathway: Option<String>,
    pub signal_type: String,
    pub signal_title: String,
    pub reported_amount_usd: Option<f64>,
    pub reported_amount_basis: Option<String>,
    pub reported_percent: Option<f64>,
    pub reported_months: Option<f64>,
    pub reviewed_program_count: Option<u32>,
    pub control_signal: String,
    pub recommendation_signal: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl DefenseProcurementControlProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_id", &self.source_id)?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("report_url", &self.report_url)?;
        validate_required("report_number", &self.report_number)?;
        validate_required("program_or_portfolio", &self.program_or_portfolio)?;
        validate_required("service_or_scope", &self.service_or_scope)?;
        validate_required("signal_type", &self.signal_type)?;
        validate_required("signal_title", &self.signal_title)?;
        validate_required("control_signal", &self.control_signal)?;
        validate_required("recommendation_signal", &self.recommendation_signal)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "defense_procurement_control_probe" {
            return Err(format!(
                "defense procurement control probe record_family must be defense_procurement_control_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:defense:procurement-control:v1"
        {
            return Err(
                "defense procurement control probes must point to the procurement-control evidence queue row"
                    .to_string(),
            );
        }
        if self.source_id != "SRC-GAO-WEAPON-SYSTEMS-2025" {
            return Err(format!(
                "defense procurement control probe source_id must be SRC-GAO-WEAPON-SYSTEMS-2025, got {}",
                self.source_id
            ));
        }
        if self.report_year == 0 {
            return Err(
                "defense procurement control probe report_year must be positive".to_string(),
            );
        }
        if let Some(amount) = self.reported_amount_usd {
            if amount < 0.0 {
                return Err(
                    "defense procurement control probe reported_amount_usd must be non-negative"
                        .to_string(),
                );
            }
            if self
                .reported_amount_basis
                .as_deref()
                .unwrap_or("")
                .is_empty()
            {
                return Err(
                    "defense procurement control probe reported_amount_basis is required when amount exists"
                        .to_string(),
                );
            }
        }
        for (field, value) in [
            ("reported_percent", self.reported_percent),
            ("reported_months", self.reported_months),
        ] {
            if let Some(value) = value {
                if value < 0.0 {
                    return Err(format!(
                        "defense procurement control probe {field} must be non-negative"
                    ));
                }
            }
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "defense procurement control probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "defense procurement control public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HealthPriceDisciplineProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_ids: Vec<String>,
    pub observed_date: String,
    pub program_part: String,
    pub service_or_drug_category: String,
    pub fiscal_or_calendar_year: String,
    pub price_or_expenditure_basis: String,
    pub benchmark_or_comparison: String,
    pub metric_value: Option<f64>,
    pub metric_unit: Option<String>,
    pub denominator_value: Option<f64>,
    pub denominator_unit: Option<String>,
    pub computed_value_usd: Option<f64>,
    pub quality_or_access_measure: String,
    pub source_record_ids: Vec<String>,
    pub readiness_status: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl HealthPriceDisciplineProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("program_part", &self.program_part)?;
        validate_required("service_or_drug_category", &self.service_or_drug_category)?;
        validate_required("fiscal_or_calendar_year", &self.fiscal_or_calendar_year)?;
        validate_required(
            "price_or_expenditure_basis",
            &self.price_or_expenditure_basis,
        )?;
        validate_required("benchmark_or_comparison", &self.benchmark_or_comparison)?;
        validate_required("quality_or_access_measure", &self.quality_or_access_measure)?;
        validate_required("readiness_status", &self.readiness_status)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "health_price_discipline_probe" {
            return Err(format!(
                "health price discipline probe record_family must be health_price_discipline_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:health-medicare:price-discipline:v1"
        {
            return Err(
                "health price discipline probes must point to the price-discipline evidence queue row"
                    .to_string(),
            );
        }
        if self.source_ids.is_empty() {
            return Err("health price discipline probes need at least one source_id".to_string());
        }
        for source_id in &self.source_ids {
            validate_required("source_id", source_id)?;
        }
        for (field, value) in [
            ("metric_value", self.metric_value),
            ("denominator_value", self.denominator_value),
            ("computed_value_usd", self.computed_value_usd),
        ] {
            if let Some(value) = value {
                if value < 0.0 {
                    return Err(format!(
                        "health price discipline probe {field} must be non-negative"
                    ));
                }
            }
        }
        if self.metric_value.is_some() && self.metric_unit.as_deref().unwrap_or("").is_empty() {
            return Err(
                "health price discipline probe metric_unit is required when metric_value exists"
                    .to_string(),
            );
        }
        if self.denominator_value.is_some()
            && self.denominator_unit.as_deref().unwrap_or("").is_empty()
        {
            return Err(
                "health price discipline probe denominator_unit is required when denominator_value exists"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "health price discipline probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "health price discipline public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HealthAdminSimplificationProbeRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_ids: Vec<String>,
    pub observed_date: String,
    pub program_part: String,
    pub workflow_step: String,
    pub period: String,
    pub administrative_cost_or_cycle_time_basis: String,
    pub claim_or_case_count: Option<f64>,
    pub claim_or_case_count_unit: Option<String>,
    pub metric_value: Option<f64>,
    pub metric_unit: Option<String>,
    pub access_or_integrity_floor: String,
    pub source_record_ids: Vec<String>,
    pub readiness_status: String,
    pub source_scope_note: String,
    pub next_extract_need: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl HealthAdminSimplificationProbeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("observed_date", &self.observed_date)?;
        validate_required("program_part", &self.program_part)?;
        validate_required("workflow_step", &self.workflow_step)?;
        validate_required("period", &self.period)?;
        validate_required(
            "administrative_cost_or_cycle_time_basis",
            &self.administrative_cost_or_cycle_time_basis,
        )?;
        validate_required("access_or_integrity_floor", &self.access_or_integrity_floor)?;
        validate_required("readiness_status", &self.readiness_status)?;
        validate_required("source_scope_note", &self.source_scope_note)?;
        validate_required("next_extract_need", &self.next_extract_need)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "health_admin_simplification_probe" {
            return Err(format!(
                "health admin simplification probe record_family must be health_admin_simplification_probe, got {}",
                self.record_family
            ));
        }
        if self.source_evidence_queue_record_id
            != "cost-down-evidence-queue:health-medicare:administrative-simplification:v1"
        {
            return Err(
                "health admin simplification probes must point to the administrative-simplification evidence queue row"
                    .to_string(),
            );
        }
        if self.source_ids.is_empty() {
            return Err(
                "health admin simplification probes need at least one source_id".to_string(),
            );
        }
        for source_id in &self.source_ids {
            validate_required("source_id", source_id)?;
        }
        for (field, value) in [
            ("claim_or_case_count", self.claim_or_case_count),
            ("metric_value", self.metric_value),
        ] {
            if let Some(value) = value {
                if value < 0.0 {
                    return Err(format!(
                        "health admin simplification probe {field} must be non-negative"
                    ));
                }
            }
        }
        if self.claim_or_case_count.is_some()
            && self
                .claim_or_case_count_unit
                .as_deref()
                .unwrap_or("")
                .is_empty()
        {
            return Err(
                "health admin simplification probe claim_or_case_count_unit is required when count exists"
                    .to_string(),
            );
        }
        if self.metric_value.is_some() && self.metric_unit.as_deref().unwrap_or("").is_empty() {
            return Err(
                "health admin simplification probe metric_unit is required when metric_value exists"
                    .to_string(),
            );
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "health admin simplification probes must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "health admin simplification public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CostDownFirstPassRollupRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_evidence_queue_record_id: String,
    pub source_backlog_record_id: String,
    pub source_pressure_record_id: String,
    pub lane_id: String,
    pub lever_id: String,
    pub first_pass_artifacts: Vec<String>,
    pub first_pass_row_count: u32,
    pub signal_status: String,
    pub strongest_current_signal: String,
    pub scoring_blockers: Vec<String>,
    pub next_scoring_step: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl CostDownFirstPassRollupRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("source_backlog_record_id", &self.source_backlog_record_id)?;
        validate_required("source_pressure_record_id", &self.source_pressure_record_id)?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("lever_id", &self.lever_id)?;
        validate_required_vec("first_pass_artifacts", &self.first_pass_artifacts)?;
        validate_required("signal_status", &self.signal_status)?;
        validate_required("strongest_current_signal", &self.strongest_current_signal)?;
        validate_required_vec("scoring_blockers", &self.scoring_blockers)?;
        validate_required("next_scoring_step", &self.next_scoring_step)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "cost_down_first_pass_rollup" {
            return Err(format!(
                "cost-down first-pass rollup record_family must be cost_down_first_pass_rollup, got {}",
                self.record_family
            ));
        }
        if self.first_pass_row_count == 0 {
            return Err("cost-down first-pass rollup row count must be positive".to_string());
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "cost-down first-pass rollup must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "cost-down first-pass rollup public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CostDownScoringReadinessRecord {
    pub record_id: String,
    pub record_family: String,
    pub source_rollup_record_id: String,
    pub source_evidence_queue_record_id: String,
    pub lane_id: String,
    pub lever_id: String,
    pub prioritization_rank: u8,
    pub readiness_tier: String,
    pub evidence_maturity_score: u8,
    pub scale_pressure_score: u8,
    pub scoring_complexity_score: u8,
    pub priority_rationale: String,
    pub immediate_next_artifact: String,
    pub public_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
    pub public_use_rule: String,
}

impl CostDownScoringReadinessRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("record_family", &self.record_family)?;
        validate_required("source_rollup_record_id", &self.source_rollup_record_id)?;
        validate_required(
            "source_evidence_queue_record_id",
            &self.source_evidence_queue_record_id,
        )?;
        validate_required("lane_id", &self.lane_id)?;
        validate_required("lever_id", &self.lever_id)?;
        validate_required("readiness_tier", &self.readiness_tier)?;
        validate_required("priority_rationale", &self.priority_rationale)?;
        validate_required("immediate_next_artifact", &self.immediate_next_artifact)?;
        validate_required("public_use_rule", &self.public_use_rule)?;

        if self.record_family != "cost_down_scoring_readiness" {
            return Err(format!(
                "cost-down scoring readiness record_family must be cost_down_scoring_readiness, got {}",
                self.record_family
            ));
        }
        if self.prioritization_rank == 0 {
            return Err("cost-down scoring readiness rank must be positive".to_string());
        }
        for (field, value) in [
            ("evidence_maturity_score", self.evidence_maturity_score),
            ("scale_pressure_score", self.scale_pressure_score),
            ("scoring_complexity_score", self.scoring_complexity_score),
        ] {
            if !(1..=5).contains(&value) {
                return Err(format!(
                    "cost-down scoring readiness {field} must be between 1 and 5"
                ));
            }
        }
        if self.public_claim_allowed || self.savings_estimate_allowed {
            return Err(
                "cost-down scoring readiness must keep public claims and savings estimates blocked"
                    .to_string(),
            );
        }
        let public_rule = self.public_use_rule.to_ascii_lowercase();
        if !(public_rule.contains("not")
            && public_rule.contains("savings estimate")
            && public_rule.contains("not a finding"))
        {
            return Err(
                "cost-down scoring readiness public_use_rule must block savings and finding claims"
                    .to_string(),
            );
        }

        Ok(())
    }
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
                PUBLIC_CLAIM_ALLOWED_LABEL
            } else {
                PUBLIC_CLAIM_BLOCKED_LABEL
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
pub const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_RECORD_FAMILY: &str =
    "external_accountability_claim_intake";
pub const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_SCHEMA_VERSION: &str = "v1";
pub const EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_USE_RULE: &str = "Internal quarantine use only: record that an attributed claim was published, preserve source and amount semantics, and request corroborating or counterevidence; do not present the underlying allegation as fact or infer fraud, waste, debt, collectibility, recovery, prevention, performance, or savings.";
pub const PUBLIC_CLAIM_ALLOWED_LABEL: &str = "Public claim allowed.";
pub const PUBLIC_CLAIM_BLOCKED_LABEL: &str = "Public claim blocked.";

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
        if self.public_claim_allowed && self.claim_gate != PUBLIC_CLAIM_ALLOWED_LABEL {
            return Err(
                "performance demand checklist record allowed claim has wrong claim_gate"
                    .to_string(),
            );
        }
        if !self.public_claim_allowed && self.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
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

impl PerformanceDemandResponseLogClass {
    pub fn all_classes() -> &'static [Self] {
        &[
            Self::NotYetReceived,
            Self::CompleteEvidenceResponse,
            Self::PartialEvidenceResponse,
            Self::ProcessOnlyResponse,
            Self::NoEvidenceResponse,
        ]
    }

    pub fn rubric_classes() -> &'static [Self] {
        &[
            Self::CompleteEvidenceResponse,
            Self::PartialEvidenceResponse,
            Self::ProcessOnlyResponse,
            Self::NoEvidenceResponse,
        ]
    }

    pub fn wire_value(&self) -> &'static str {
        match self {
            Self::NotYetReceived => "not-yet-received",
            Self::CompleteEvidenceResponse => "complete-evidence-response",
            Self::PartialEvidenceResponse => "partial-evidence-response",
            Self::ProcessOnlyResponse => "process-only-response",
            Self::NoEvidenceResponse => "no-evidence-response",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::NotYetReceived => "Not yet received",
            Self::CompleteEvidenceResponse => "Complete evidence response",
            Self::PartialEvidenceResponse => "Partial evidence response",
            Self::ProcessOnlyResponse => "Process-only response",
            Self::NoEvidenceResponse => "No evidence response",
        }
    }

    pub fn rubric_meaning(&self) -> &'static str {
        match self {
            Self::NotYetReceived => "No reply has been logged in TAXLANE.",
            Self::CompleteEvidenceResponse => {
                "Provides source record/version, reviewed performance evidence or official finding, role-approved wording, and public-claim basis."
            }
            Self::PartialEvidenceResponse => {
                "Provides some requested evidence but leaves at least one required item missing or unclear."
            }
            Self::ProcessOnlyResponse => {
                "Explains process, office ownership, or future work but does not provide the requested evidence."
            }
            Self::NoEvidenceResponse => {
                "Declines, ignores, or cannot identify the requested evidence."
            }
        }
    }

    pub fn rubric_next_action(&self) -> &'static str {
        match self {
            Self::NotYetReceived => PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION,
            Self::CompleteEvidenceResponse => "Route to role review before any public claim.",
            Self::PartialEvidenceResponse => "Ask a narrower follow-up for the missing item.",
            Self::ProcessOnlyResponse => "Keep claim gate blocked and request the evidence source.",
            Self::NoEvidenceResponse => {
                "Keep claim gate blocked; do not infer misconduct or poor performance."
            }
        }
    }

    pub fn requires_evidence_received(&self) -> bool {
        matches!(
            self,
            Self::CompleteEvidenceResponse | Self::PartialEvidenceResponse
        )
    }

    pub fn forbids_evidence_received(&self) -> bool {
        matches!(
            self,
            Self::NotYetReceived | Self::ProcessOnlyResponse | Self::NoEvidenceResponse
        )
    }
}

impl From<&PerformanceDemandResponseClass> for PerformanceDemandResponseLogClass {
    fn from(response_class: &PerformanceDemandResponseClass) -> Self {
        match response_class {
            PerformanceDemandResponseClass::CompleteEvidenceResponse => {
                Self::CompleteEvidenceResponse
            }
            PerformanceDemandResponseClass::PartialEvidenceResponse => {
                Self::PartialEvidenceResponse
            }
            PerformanceDemandResponseClass::ProcessOnlyResponse => Self::ProcessOnlyResponse,
            PerformanceDemandResponseClass::NoEvidenceResponse => Self::NoEvidenceResponse,
        }
    }
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
    pub fn apply_intake(
        &self,
        intake: &PerformanceDemandResponseIntakeRecord,
    ) -> Result<Self, String> {
        self.validate()?;
        intake.validate()?;

        if self.record_id != intake.record_id {
            return Err("response intake record_id does not match response log record".to_string());
        }

        let response_class = PerformanceDemandResponseLogClass::from(&intake.response_class);
        let updated = Self {
            record_id: self.record_id.clone(),
            lane_id: self.lane_id.clone(),
            program_or_account_id: self.program_or_account_id.clone(),
            response_class,
            evidence_received: intake.evidence_received.clone(),
            missing_evidence: intake.missing_evidence.clone(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: PerformanceDemandResponseLogClass::from(&intake.response_class)
                .rubric_next_action()
                .to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };
        updated.validate()?;
        Ok(updated)
    }

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
        if self.claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err(
                "performance demand response log record must preserve blocked claim_gate"
                    .to_string(),
            );
        }
        if self.response_class.requires_evidence_received() && self.evidence_received.is_empty() {
            return Err(
                "complete and partial evidence response log records require evidence_received"
                    .to_string(),
            );
        }
        if self.response_class.forbids_evidence_received() && !self.evidence_received.is_empty() {
            return Err(
                "not-yet-received, process-only, and no-evidence response log records must not list evidence_received"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseStatus {
    pub artifact: String,
    pub total_rows: usize,
    pub not_yet_received: usize,
    pub public_claim_allowed: usize,
    pub public_claim_blocked: usize,
    pub use_rule: String,
}

impl PerformanceDemandResponseStatus {
    pub fn from_response_log_records(
        artifact: &str,
        records: &[PerformanceDemandResponseLogRecord],
    ) -> Result<Self, String> {
        validate_required("artifact", artifact)?;
        if records.is_empty() {
            return Err("performance demand response status needs response rows".to_string());
        }
        for record in records {
            record.validate()?;
        }

        let total_rows = records.len();
        let not_yet_received = records
            .iter()
            .filter(|record| {
                record.response_class == PerformanceDemandResponseLogClass::NotYetReceived
            })
            .count();
        let public_claim_allowed = records
            .iter()
            .filter(|record| record.public_claim_allowed)
            .count();
        let public_claim_blocked = total_rows.saturating_sub(public_claim_allowed);

        Ok(Self {
            artifact: artifact.to_string(),
            total_rows,
            not_yet_received,
            public_claim_allowed,
            public_claim_blocked,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_required("artifact", &self.artifact)?;
        validate_required("use_rule", &self.use_rule)?;
        if self.use_rule != PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE {
            return Err("performance demand response status has unexpected use_rule".to_string());
        }
        if self.total_rows == 0 {
            return Err("performance demand response status needs response rows".to_string());
        }
        if self.public_claim_allowed + self.public_claim_blocked != self.total_rows {
            return Err(
                "performance demand response status claim counts do not sum to total_rows"
                    .to_string(),
            );
        }
        if self.not_yet_received > self.total_rows {
            return Err(
                "performance demand response status not_yet_received exceeds total_rows"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseBundleArtifact {
    pub artifact: String,
    pub role: String,
    pub kind: String,
    pub row_count: String,
    pub sha256: String,
    pub consumer_use: String,
}

impl PerformanceDemandResponseBundleArtifact {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("bundle artifact", &self.artifact)?;
        validate_required("bundle artifact role", &self.role)?;
        validate_required("bundle artifact kind", &self.kind)?;
        validate_required("bundle artifact row_count", &self.row_count)?;
        validate_required("bundle artifact sha256", &self.sha256)?;
        validate_required("bundle artifact consumer_use", &self.consumer_use)?;
        if self.artifact.contains('\\')
            || self.artifact.starts_with('/')
            || self.artifact.contains("..")
        {
            return Err(format!(
                "bundle artifact must be repo-relative with forward slashes: {}",
                self.artifact
            ));
        }
        if !matches!(self.kind.as_str(), "jsonl" | "json" | "markdown") {
            return Err(format!(
                "bundle artifact {} has unsupported kind {}",
                self.artifact, self.kind
            ));
        }
        if self.sha256.len() != 64 || !self.sha256.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(format!(
                "bundle artifact {} has invalid sha256 {}",
                self.artifact, self.sha256
            ));
        }
        if self.kind == "jsonl" {
            self.row_count.parse::<usize>().map_err(|_| {
                format!(
                    "bundle artifact {} JSONL row_count must be a number",
                    self.artifact
                )
            })?;
        } else if self.row_count != "n/a" {
            return Err(format!(
                "bundle artifact {} non-JSONL row_count must be n/a",
                self.artifact
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseBundleManifest {
    pub artifact: String,
    pub bundle_kind: String,
    pub total_rows: usize,
    pub updated_rows: usize,
    pub public_claim_allowed: usize,
    pub public_claim_blocked: usize,
    pub artifacts: Vec<PerformanceDemandResponseBundleArtifact>,
    pub boundary: String,
    pub use_rule: String,
}

impl PerformanceDemandResponseBundleManifest {
    pub fn from_status(
        artifact: &str,
        status: &PerformanceDemandResponseStatus,
        artifacts: Vec<PerformanceDemandResponseBundleArtifact>,
    ) -> Result<Self, String> {
        status.validate()?;
        let manifest = Self {
            artifact: artifact.to_string(),
            bundle_kind: "applied-response-importer-fixture".to_string(),
            total_rows: status.total_rows,
            updated_rows: status.total_rows.saturating_sub(status.not_yet_received),
            public_claim_allowed: status.public_claim_allowed,
            public_claim_blocked: status.public_claim_blocked,
            artifacts,
            boundary: "Importer fixture only; not canonical response status, public-claim eligibility, misconduct finding, performance finding, or proof of reform benefits.".to_string(),
            use_rule: status.use_rule.clone(),
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_required("bundle manifest artifact", &self.artifact)?;
        validate_required("bundle manifest kind", &self.bundle_kind)?;
        validate_required("bundle manifest boundary", &self.boundary)?;
        validate_required("bundle manifest use_rule", &self.use_rule)?;
        if self.bundle_kind != "applied-response-importer-fixture" {
            return Err(
                "performance demand response bundle manifest has unexpected bundle_kind"
                    .to_string(),
            );
        }
        if self.use_rule != PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE {
            return Err(
                "performance demand response bundle manifest has unexpected use_rule".to_string(),
            );
        }
        if self.total_rows == 0 {
            return Err(
                "performance demand response bundle manifest needs response rows".to_string(),
            );
        }
        if self.updated_rows > self.total_rows {
            return Err(
                "performance demand response bundle manifest updated_rows exceeds total_rows"
                    .to_string(),
            );
        }
        if self.public_claim_allowed != 0 {
            return Err("applied fixture bundle must not allow public claims".to_string());
        }
        if self.public_claim_allowed + self.public_claim_blocked != self.total_rows {
            return Err(
                "performance demand response bundle manifest claim counts do not sum to total_rows"
                    .to_string(),
            );
        }
        if self.artifacts.is_empty() {
            return Err("performance demand response bundle manifest needs artifacts".to_string());
        }
        for artifact in &self.artifacts {
            artifact.validate()?;
        }
        for required in [
            "data/derived/accountability_evidence/performance-demand-response-intake.example.jsonl",
            "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl",
            "data/derived/accountability_evidence/performance-demand-response-status.applied-example.json",
            "data/derived/accountability_evidence/performance-demand-response-dashboard.applied-example.md",
            "data/derived/accountability_evidence/performance-demand-response-handoff.applied-example.md",
            "data/derived/accountability_evidence/performance-demand-response-applied-example.schema.md",
            "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.md",
            "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.jsonl",
            "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.schema.md",
        ] {
            if !self
                .artifacts
                .iter()
                .any(|artifact| artifact.artifact == required)
            {
                return Err(format!(
                    "performance demand response bundle manifest missing {required}"
                ));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PerformanceDemandResponseDeltaRow {
    pub record_id: String,
    pub before_response_class: PerformanceDemandResponseLogClass,
    pub after_response_class: PerformanceDemandResponseLogClass,
    pub before_evidence_received_count: usize,
    pub after_evidence_received_count: usize,
    pub missing_evidence_changed: bool,
    pub next_action_changed: bool,
    pub before_claim_gate: String,
    pub after_claim_gate: String,
}

impl PerformanceDemandResponseDeltaRow {
    pub fn from_response_log_records(
        before_rows: &[PerformanceDemandResponseLogRecord],
        after_rows: &[PerformanceDemandResponseLogRecord],
    ) -> Result<Vec<Self>, String> {
        if before_rows.is_empty() || after_rows.is_empty() {
            return Err("performance demand response delta needs response rows".to_string());
        }

        let before_by_id = response_log_records_by_id(before_rows, "before")?;
        let after_by_id = response_log_records_by_id(after_rows, "after")?;
        if before_by_id.len() != after_by_id.len() {
            return Err("performance demand response delta row counts do not match".to_string());
        }

        let mut rows = Vec::new();
        for (record_id, after) in &after_by_id {
            let before = before_by_id.get(record_id).ok_or_else(|| {
                format!("performance demand response delta missing before row: {record_id}")
            })?;
            if before == after {
                continue;
            }

            let delta = Self {
                record_id: record_id.to_string(),
                before_response_class: before.response_class.clone(),
                after_response_class: after.response_class.clone(),
                before_evidence_received_count: before.evidence_received.len(),
                after_evidence_received_count: after.evidence_received.len(),
                missing_evidence_changed: before.missing_evidence != after.missing_evidence,
                next_action_changed: before.next_action != after.next_action,
                before_claim_gate: before.claim_gate.clone(),
                after_claim_gate: after.claim_gate.clone(),
            };
            delta.validate()?;
            rows.push(delta);
        }

        Ok(rows)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("before_claim_gate", &self.before_claim_gate)?;
        validate_required("after_claim_gate", &self.after_claim_gate)?;
        if self.before_claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err(
                "performance demand response delta before row must keep blocked claim_gate"
                    .to_string(),
            );
        }
        if self.after_claim_gate != PUBLIC_CLAIM_BLOCKED_LABEL {
            return Err(
                "performance demand response delta after row must keep blocked claim_gate"
                    .to_string(),
            );
        }

        Ok(())
    }
}

fn response_log_records_by_id<'a>(
    records: &'a [PerformanceDemandResponseLogRecord],
    label: &str,
) -> Result<BTreeMap<&'a str, &'a PerformanceDemandResponseLogRecord>, String> {
    let mut by_id = BTreeMap::new();
    for record in records {
        record.validate()?;
        if by_id.insert(record.record_id.as_str(), record).is_some() {
            return Err(format!(
                "performance demand response delta has duplicate {label} row: {}",
                record.record_id
            ));
        }
    }
    Ok(by_id)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerformanceDemandResponseClass {
    CompleteEvidenceResponse,
    PartialEvidenceResponse,
    ProcessOnlyResponse,
    NoEvidenceResponse,
}

impl PerformanceDemandResponseClass {
    pub fn all_classes() -> &'static [Self] {
        &[
            Self::CompleteEvidenceResponse,
            Self::PartialEvidenceResponse,
            Self::ProcessOnlyResponse,
            Self::NoEvidenceResponse,
        ]
    }

    pub fn wire_value(&self) -> &'static str {
        match self {
            Self::CompleteEvidenceResponse => "complete-evidence-response",
            Self::PartialEvidenceResponse => "partial-evidence-response",
            Self::ProcessOnlyResponse => "process-only-response",
            Self::NoEvidenceResponse => "no-evidence-response",
        }
    }

    pub fn intake_meaning(&self) -> &'static str {
        match self {
            Self::CompleteEvidenceResponse => {
                "All requested evidence and claim basis were provided, pending role review."
            }
            Self::PartialEvidenceResponse => {
                "At least one requested evidence item remains missing or unclear."
            }
            Self::ProcessOnlyResponse => {
                "The reply explains process but does not provide requested evidence."
            }
            Self::NoEvidenceResponse => {
                "The reply declines, ignores, or cannot identify requested evidence."
            }
        }
    }

    pub fn requires_evidence_received(&self) -> bool {
        matches!(
            self,
            Self::CompleteEvidenceResponse | Self::PartialEvidenceResponse
        )
    }

    pub fn forbids_evidence_received(&self) -> bool {
        matches!(self, Self::ProcessOnlyResponse | Self::NoEvidenceResponse)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimantType {
    JournalistOrCommentator,
    Witness,
    Whistleblower,
    Beneficiary,
    VendorOrRecipient,
    AgencyOfficial,
    InspectorGeneral,
    LawEnforcement,
    Court,
    ElectedOfficial,
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimPublicationKind {
    OriginalVideo,
    SocialPost,
    Article,
    Interview,
    WrittenTestimony,
    HearingVideo,
    AgencyRelease,
    AuditReport,
    CourtRecord,
    Dataset,
    ResponseLetter,
    Correction,
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimEvidenceRelation {
    ClaimOrigin,
    SupportsClaimAtom,
    CorroboratesPart,
    ContradictsPart,
    SuppliesContext,
    OfficialResponse,
    Correction,
    Supersession,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimCustodyStatus {
    UrlObservedNotCaptured,
    CapturedHashVerified,
    OfficialCopyCaptured,
    Unavailable,
    Superseded,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimType {
    DirectObservation,
    IdentityOrAffiliation,
    SiteOrServiceOperation,
    EligibilityOrEnrollment,
    PaymentOrBilling,
    AwardOrContract,
    DuplicateOrOverlap,
    DataQuality,
    ControlFailure,
    AggregateImproperPaymentAllegation,
    AggregateFraudAllegation,
    PerformanceAllegation,
    DebtAllegation,
    RecoveryAssertion,
    PreventionAssertion,
    SavingsAssertion,
    OfficialResponse,
    CorrectionOrRetraction,
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimAmountSemantic {
    SourceStatedTotal,
    ProgramOutlays,
    AwardOrContractCeiling,
    PaidAmount,
    BilledAmount,
    QuestionedCost,
    StatisticalImproperPaymentEstimate,
    UnknownPaymentStatus,
    AllegedFraudExposure,
    ChargedLoss,
    CourtConfirmedFraud,
    SettlementAmount,
    IdentifiedOverpayment,
    EstablishedDebt,
    CollectibleAmount,
    RecoveredCash,
    RestitutionOrdered,
    RestitutionPaid,
    PreventedLossEstimate,
    SourceStatedSavingsTotal,
    GrossSavingsEstimate,
    ControlCost,
    OffsetOrLeakage,
    NetSavingsEstimate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimStatus {
    IntakeUnverified,
    SourceCustodied,
    EvidenceMappingInProgress,
    AttributedClaimSupported,
    PartiallyCorroborated,
    IndependentlyCorroborated,
    Contested,
    UnableToVerify,
    Corrected,
    Retracted,
    Superseded,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimLegalOrAdministrativeStatus {
    NoneEstablished,
    AgencyReviewReported,
    AuditOpened,
    ReferredForReview,
    InvestigationReported,
    CivilComplaintFiled,
    CriminalChargeFiled,
    OfficialFinding,
    SettlementNoAdmission,
    SettlementWithAdmission,
    PleaEntered,
    Adjudicated,
    Dismissed,
    Overturned,
    ClosedWithoutFinding,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalClaimReviewStatus {
    Draft,
    SourceReviewed,
    AccountabilityReviewed,
    RoleReviewed,
    Superseded,
    Retired,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimAmountStatus {
    PublisherAllegation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimAggregationMethod {
    Undisclosed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimAmountDerivation {
    SourceStatedLowerBound,
    SourceStatedExact,
    SourceStatedRange,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimSummability {
    NotSummable,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimResponseRequestStatus {
    NotRecorded,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalClaimResponsePosition {
    NoneRecorded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimant {
    pub display_name: String,
    pub claimant_type: ExternalClaimantType,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimPublication {
    pub publication_kind: ExternalClaimPublicationKind,
    pub publisher: String,
    pub published_date: Option<String>,
    pub observed_date: String,
    pub source_id: String,
    pub source_url: String,
    pub custody_path: Option<String>,
    pub sha256: Option<String>,
    pub custody_status: ExternalClaimCustodyStatus,
    pub evidence_relation: ExternalClaimEvidenceRelation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimAtom {
    pub claim_type: ExternalClaimType,
    pub neutral_paraphrase: String,
    pub exact_text_verified: bool,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub geography: String,
    pub coverage_period: String,
    pub basis_disclosed: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimAmountAssertion {
    pub value: f64,
    pub lower_bound: Option<f64>,
    pub upper_bound: Option<f64>,
    pub currency: String,
    pub unit: String,
    pub amount_semantic: ExternalClaimAmountSemantic,
    pub amount_status: ExternalClaimAmountStatus,
    pub aggregation_method: ExternalClaimAggregationMethod,
    pub population_or_universe: String,
    pub period: String,
    pub derivation: ExternalClaimAmountDerivation,
    pub overlap_group: String,
    pub overlap_established: bool,
    pub summability: ExternalClaimSummability,
    pub lineage_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimOfficialResponse {
    pub request_status: ExternalClaimResponseRequestStatus,
    pub requested_at: Option<String>,
    pub respondent: Option<String>,
    pub response_source_ids: Vec<String>,
    pub position: ExternalClaimResponsePosition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimGates {
    pub attributed_claim_reporting_allowed: bool,
    pub underlying_factual_claim_allowed: bool,
    pub misconduct_signal_claim_allowed: bool,
    pub official_finding_claim_allowed: bool,
    pub performance_claim_allowed: bool,
    pub fraud_claim_allowed: bool,
    pub waste_claim_allowed: bool,
    pub debt_claim_allowed: bool,
    pub collectibility_claim_allowed: bool,
    pub recovery_claim_allowed: bool,
    pub prevention_claim_allowed: bool,
    pub savings_estimate_allowed: bool,
}

impl ExternalAccountabilityClaimGates {
    pub fn all_false(&self) -> bool {
        !self.attributed_claim_reporting_allowed
            && !self.underlying_factual_claim_allowed
            && !self.misconduct_signal_claim_allowed
            && !self.official_finding_claim_allowed
            && !self.performance_claim_allowed
            && !self.fraud_claim_allowed
            && !self.waste_claim_allowed
            && !self.debt_claim_allowed
            && !self.collectibility_claim_allowed
            && !self.recovery_claim_allowed
            && !self.prevention_claim_allowed
            && !self.savings_estimate_allowed
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalAccountabilityClaimIntakeRecord {
    pub record_id: String,
    pub record_family: String,
    pub schema_version: String,
    pub claim_group_id: String,
    pub atom_order: u32,
    pub lane_id: Option<String>,
    pub program_or_account_id: Option<String>,
    pub subject_ids: Vec<String>,
    pub claimant: ExternalAccountabilityClaimant,
    pub publications: Vec<ExternalAccountabilityClaimPublication>,
    pub claim_atom: ExternalAccountabilityClaimAtom,
    pub amount_assertion: ExternalAccountabilityClaimAmountAssertion,
    pub corroborating_source_ids: Vec<String>,
    pub counterevidence_source_ids: Vec<String>,
    pub official_response: ExternalAccountabilityClaimOfficialResponse,
    pub claim_status: ExternalClaimStatus,
    pub legal_or_administrative_status: ExternalClaimLegalOrAdministrativeStatus,
    pub review_status: ExternalClaimReviewStatus,
    pub status_as_of: String,
    pub supersedes_record_id: Option<String>,
    pub correction_or_retraction_note: Option<String>,
    pub comparison_basis: String,
    pub due_process_caveat: String,
    pub use_rule: String,
    pub claim_gates: ExternalAccountabilityClaimGates,
}

impl ExternalAccountabilityClaimIntakeRecord {
    pub fn validate(&self) -> Result<(), String> {
        validate_required("record_id", &self.record_id)?;
        validate_required("claim_group_id", &self.claim_group_id)?;
        validate_required("claimant.display_name", &self.claimant.display_name)?;
        validate_required(
            "claim_atom.neutral_paraphrase",
            &self.claim_atom.neutral_paraphrase,
        )?;
        validate_required("claim_atom.subject", &self.claim_atom.subject)?;
        validate_required("claim_atom.predicate", &self.claim_atom.predicate)?;
        validate_required("claim_atom.object", &self.claim_atom.object)?;
        validate_required("claim_atom.geography", &self.claim_atom.geography)?;
        validate_required(
            "claim_atom.coverage_period",
            &self.claim_atom.coverage_period,
        )?;
        validate_required(
            "amount_assertion.population_or_universe",
            &self.amount_assertion.population_or_universe,
        )?;
        validate_required("amount_assertion.period", &self.amount_assertion.period)?;
        validate_required(
            "amount_assertion.overlap_group",
            &self.amount_assertion.overlap_group,
        )?;
        validate_required("comparison_basis", &self.comparison_basis)?;
        validate_required("due_process_caveat", &self.due_process_caveat)?;
        validate_required("use_rule", &self.use_rule)?;
        validate_iso_date("status_as_of", &self.status_as_of)?;
        let status_as_of = parse_iso_date("status_as_of", &self.status_as_of)?;

        if self.record_family != EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_RECORD_FAMILY {
            return Err("external claim intake has unexpected record_family".to_string());
        }
        if self.schema_version != EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_SCHEMA_VERSION {
            return Err("external claim intake has unexpected schema_version".to_string());
        }
        if self.use_rule != EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_USE_RULE {
            return Err("external claim intake has unexpected use_rule".to_string());
        }
        if self.atom_order != 1 {
            return Err(
                "external claim intake rows must contain one amount atom at atom_order 1"
                    .to_string(),
            );
        }
        if self.publications.is_empty() {
            return Err("external claim intake requires at least one publication".to_string());
        }

        let mut source_ids = BTreeSet::new();
        let mut claim_origin_count = 0usize;
        let mut has_url_observed = false;
        let mut all_claim_origins_captured = true;
        for publication in &self.publications {
            validate_required("publication.publisher", &publication.publisher)?;
            validate_required("publication.source_id", &publication.source_id)?;
            validate_required("publication.source_url", &publication.source_url)?;
            validate_required("publication.observed_date", &publication.observed_date)?;
            let observed_date =
                parse_iso_date("publication.observed_date", &publication.observed_date)?;
            if observed_date > status_as_of {
                return Err(
                    "publication observed_date must not be later than status_as_of".to_string(),
                );
            }
            if let Some(date) = &publication.published_date {
                let published_interval =
                    parse_iso_date_precision("publication.published_date", date)?;
                if published_interval.0 > observed_date {
                    return Err(
                        "publication published_date must not be later than observed_date"
                            .to_string(),
                    );
                }
            }
            if !publication.source_url.starts_with("https://") {
                return Err("external claim publication source_url must use https".to_string());
            }
            if !source_ids.insert(publication.source_id.as_str()) {
                return Err(
                    "external claim publication source IDs must be unique per row".to_string(),
                );
            }
            if publication.evidence_relation == ExternalClaimEvidenceRelation::ClaimOrigin {
                claim_origin_count += 1;
                all_claim_origins_captured &= matches!(
                    publication.custody_status,
                    ExternalClaimCustodyStatus::CapturedHashVerified
                        | ExternalClaimCustodyStatus::OfficialCopyCaptured
                );
            } else if publication.evidence_relation
                != ExternalClaimEvidenceRelation::SuppliesContext
            {
                return Err(
                    "unverified intake publications may only originate a claim or supply context"
                        .to_string(),
                );
            }
            match publication.custody_status {
                ExternalClaimCustodyStatus::UrlObservedNotCaptured => {
                    has_url_observed = true;
                    if publication.custody_path.is_some() || publication.sha256.is_some() {
                        return Err(
                            "URL-observed publication must not claim custody path or SHA-256"
                                .to_string(),
                        );
                    }
                }
                ExternalClaimCustodyStatus::CapturedHashVerified
                | ExternalClaimCustodyStatus::OfficialCopyCaptured => {
                    let path = publication
                        .custody_path
                        .as_deref()
                        .ok_or_else(|| "captured publication requires custody_path".to_string())?;
                    validate_required("publication.custody_path", path)?;
                    let sha256 = publication
                        .sha256
                        .as_deref()
                        .ok_or_else(|| "captured publication requires sha256".to_string())?;
                    if !is_lowercase_sha256(sha256) {
                        return Err(
                            "captured publication sha256 must be 64 lowercase hex characters"
                                .to_string(),
                        );
                    }
                }
                ExternalClaimCustodyStatus::Unavailable
                | ExternalClaimCustodyStatus::Superseded => {
                    if publication.custody_path.is_some() || publication.sha256.is_some() {
                        return Err(
                            "uncaptured publication status must not claim custody metadata"
                                .to_string(),
                        );
                    }
                }
            }
        }
        if claim_origin_count != 1 {
            return Err(
                "external claim intake requires exactly one claim_origin publication".to_string(),
            );
        }
        if has_url_observed && self.claim_atom.exact_text_verified {
            return Err("URL-observed claim text cannot be exact_text_verified".to_string());
        }

        let amount = &self.amount_assertion;
        if !amount.value.is_finite() || amount.value <= 0.0 {
            return Err("external claim amount value must be finite and positive".to_string());
        }
        match amount.derivation {
            ExternalClaimAmountDerivation::SourceStatedLowerBound => {
                if amount.lower_bound != Some(amount.value) || amount.upper_bound.is_some() {
                    return Err("external claim lower-bound derivation requires lower_bound equal to value and no upper_bound".to_string());
                }
            }
            ExternalClaimAmountDerivation::SourceStatedExact => {
                if amount.lower_bound.is_some() || amount.upper_bound.is_some() {
                    return Err("external claim exact derivation must not carry bounds".to_string());
                }
            }
            ExternalClaimAmountDerivation::SourceStatedRange => {
                let lower = amount.lower_bound.ok_or_else(|| {
                    "external claim range derivation requires lower_bound".to_string()
                })?;
                let upper = amount.upper_bound.ok_or_else(|| {
                    "external claim range derivation requires upper_bound".to_string()
                })?;
                if !lower.is_finite()
                    || !upper.is_finite()
                    || lower <= 0.0
                    || lower >= upper
                    || amount.value < lower
                    || amount.value > upper
                {
                    return Err("external claim range derivation requires positive ordered bounds containing value".to_string());
                }
            }
        }
        if amount.currency != "USD" || !matches!(amount.unit.as_str(), "millions" | "billions") {
            return Err("external claim amount requires USD and millions or billions".to_string());
        }
        if amount.overlap_established {
            return Err("external claim intake overlap_established must remain false".to_string());
        }
        if amount.summability != ExternalClaimSummability::NotSummable {
            return Err("external claim intake must remain not_summable".to_string());
        }
        if !amount.lineage_ids.is_empty() {
            return Err("external claim intake lineage_ids must remain empty".to_string());
        }
        if self.claim_atom.basis_disclosed {
            return Err("external claim intake basis_disclosed must remain false".to_string());
        }
        if !self.subject_ids.is_empty()
            || !self.corroborating_source_ids.is_empty()
            || !self.counterevidence_source_ids.is_empty()
        {
            return Err("external claim quarantine must not claim subjects, corroboration, or counterevidence".to_string());
        }
        if self.official_response.requested_at.is_some()
            || !self.official_response.response_source_ids.is_empty()
        {
            return Err("external claim official response must remain not recorded".to_string());
        }
        if self.legal_or_administrative_status
            != ExternalClaimLegalOrAdministrativeStatus::NoneEstablished
        {
            return Err(
                "external claim intake must keep legal status none_established".to_string(),
            );
        }
        match self.claim_status {
            ExternalClaimStatus::IntakeUnverified => {
                if self.review_status != ExternalClaimReviewStatus::Draft
                    || self.claim_atom.exact_text_verified
                    || self.official_response.respondent.is_some()
                    || all_claim_origins_captured
                {
                    return Err("unverified external claim must remain draft, exact-text unverified, uncaptured at its origin, and without a named respondent".to_string());
                }
            }
            ExternalClaimStatus::AttributedClaimSupported => {
                let respondent = self
                    .official_response
                    .respondent
                    .as_deref()
                    .ok_or_else(|| {
                        "supported attributed claim requires a named respondent".to_string()
                    })?;
                validate_required("official_response.respondent", respondent)?;
                if !all_claim_origins_captured
                    || !self.claim_atom.exact_text_verified
                    || !matches!(
                        self.review_status,
                        ExternalClaimReviewStatus::SourceReviewed
                            | ExternalClaimReviewStatus::AccountabilityReviewed
                            | ExternalClaimReviewStatus::RoleReviewed
                    )
                {
                    return Err("attributed_claim_supported requires captured claim origin, exact-text verification, and source-or-higher review".to_string());
                }
            }
            _ => {
                return Err(
                    "external claim status is not enabled for the current quarantine slice"
                        .to_string(),
                );
            }
        }
        if self.supersedes_record_id.is_some() || self.correction_or_retraction_note.is_some() {
            return Err(
                "initial external claim intake cannot claim correction or supersession".to_string(),
            );
        }
        if !self.claim_gates.all_false() {
            return Err("external claim intake requires all twelve claim gates false".to_string());
        }
        let caveat = self.due_process_caveat.to_ascii_lowercase();
        let attribution_boundary = match self.claim_status {
            ExternalClaimStatus::IntakeUnverified => caveat.contains("records an attributed"),
            ExternalClaimStatus::AttributedClaimSupported => {
                caveat.contains("custody proves only the attributed assertion")
                    && caveat.contains("no response is recorded")
            }
            _ => false,
        };
        if !attribution_boundary
            || !(caveat.contains("does not establish") || caveat.contains("do not establish"))
        {
            return Err(
                "external claim intake requires attributed-claim due-process wording".to_string(),
            );
        }
        if self.publications.iter().any(|publication| {
            publication.publication_kind == ExternalClaimPublicationKind::WrittenTestimony
                && publication.publisher.contains("House")
        }) && (!(self.comparison_basis.contains("not an official finding")
            || self.comparison_basis.contains("official context only")
            || self.comparison_basis.contains("supports only that"))
            || self.claim_gates.official_finding_claim_allowed)
        {
            return Err(
                "House-hosted testimony must remain context, not an official finding".to_string(),
            );
        }

        Ok(())
    }
}

fn is_lowercase_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn parse_iso_date_precision(
    label: &str,
    value: &str,
) -> Result<((u16, u8, u8), (u16, u8, u8)), String> {
    match value.len() {
        4 => {
            let year = parse_date_number(label, value, "year")?;
            Ok(((year, 1, 1), (year, 12, 31)))
        }
        7 if value.as_bytes()[4] == b'-' => {
            let year = parse_date_number(label, &value[..4], "year")?;
            let month = parse_date_number::<u8>(label, &value[5..], "month")?;
            let last_day = days_in_month(year, month)
                .ok_or_else(|| format!("{label} contains an invalid month"))?;
            Ok(((year, month, 1), (year, month, last_day)))
        }
        10 => {
            let date = parse_iso_date(label, value)?;
            Ok((date, date))
        }
        _ => Err(format!("{label} must use YYYY, YYYY-MM, or YYYY-MM-DD")),
    }
}

fn parse_iso_date(label: &str, value: &str) -> Result<(u16, u8, u8), String> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(format!("{label} must use YYYY-MM-DD"));
    }
    let year = parse_date_number(label, &value[..4], "year")?;
    let month = parse_date_number::<u8>(label, &value[5..7], "month")?;
    let day = parse_date_number::<u8>(label, &value[8..], "day")?;
    let last_day =
        days_in_month(year, month).ok_or_else(|| format!("{label} contains an invalid month"))?;
    if day == 0 || day > last_day {
        return Err(format!("{label} contains an invalid day"));
    }
    Ok((year, month, day))
}

fn parse_date_number<T>(label: &str, value: &str, component: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("{label} contains an invalid {component}"));
    }
    value
        .parse::<T>()
        .map_err(|_| format!("{label} contains an invalid {component}"))
}

fn days_in_month(year: u16, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => Some(29),
        2 => Some(28),
        _ => None,
    }
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
        if self.response_class.requires_evidence_received() && self.evidence_received.is_empty() {
            return Err(
                "complete and partial evidence response intake records require evidence_received"
                    .to_string(),
            );
        }
        if self.response_class.forbids_evidence_received() && !self.evidence_received.is_empty() {
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

fn validate_required_vec(label: &str, values: &[String]) -> Result<(), String> {
    if values.is_empty() {
        return Err(format!("{label} must not be empty"));
    }
    for value in values {
        validate_required(label, value)?;
    }
    Ok(())
}

fn validate_positive_option(label: &str, value: Option<f64>, context: &str) -> Result<(), String> {
    match value {
        Some(value) if value > 0.0 => Ok(()),
        Some(_) => Err(format!("{context} {label} must be positive")),
        None => Err(format!("{context} {label} must be present")),
    }
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn contains_any_case_insensitive(haystack: &str, needles: &[&str]) -> bool {
    needles
        .iter()
        .any(|needle| contains_case_insensitive(haystack, needle))
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

    fn external_claim_intake_fixture() -> ExternalAccountabilityClaimIntakeRecord {
        ExternalAccountabilityClaimIntakeRecord {
            record_id: "external-claim:test:amount:01".to_string(),
            record_family: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_RECORD_FAMILY.to_string(),
            schema_version: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_SCHEMA_VERSION.to_string(),
            claim_group_id: "external-claim-group:test".to_string(),
            atom_order: 1,
            lane_id: Some("health".to_string()),
            program_or_account_id: None,
            subject_ids: Vec::new(),
            claimant: ExternalAccountabilityClaimant {
                display_name: "Example Claimant".to_string(),
                claimant_type: ExternalClaimantType::JournalistOrCommentator,
            },
            publications: vec![ExternalAccountabilityClaimPublication {
                publication_kind: ExternalClaimPublicationKind::SocialPost,
                publisher: "Example Claimant".to_string(),
                published_date: Some("2026-07-10".to_string()),
                observed_date: "2026-07-14".to_string(),
                source_id: "SRC-EXAMPLE-CLAIM".to_string(),
                source_url: "https://example.test/claim".to_string(),
                custody_path: None,
                sha256: None,
                custody_status: ExternalClaimCustodyStatus::UrlObservedNotCaptured,
                evidence_relation: ExternalClaimEvidenceRelation::ClaimOrigin,
            }],
            claim_atom: ExternalAccountabilityClaimAtom {
                claim_type: ExternalClaimType::AggregateFraudAllegation,
                neutral_paraphrase: "Example Claimant alleges more than $10 million in fraud."
                    .to_string(),
                exact_text_verified: false,
                subject: "Example activity".to_string(),
                predicate: "is alleged to involve fraud".to_string(),
                object: "public-program payments".to_string(),
                geography: "Example jurisdiction".to_string(),
                coverage_period: "source_defined_undetermined".to_string(),
                basis_disclosed: false,
            },
            amount_assertion: ExternalAccountabilityClaimAmountAssertion {
                value: 10.0,
                lower_bound: Some(10.0),
                upper_bound: None,
                currency: "USD".to_string(),
                unit: "millions".to_string(),
                amount_semantic: ExternalClaimAmountSemantic::AllegedFraudExposure,
                amount_status: ExternalClaimAmountStatus::PublisherAllegation,
                aggregation_method: ExternalClaimAggregationMethod::Undisclosed,
                population_or_universe: "undisclosed".to_string(),
                period: "undisclosed".to_string(),
                derivation: ExternalClaimAmountDerivation::SourceStatedLowerBound,
                overlap_group: "example-undetermined".to_string(),
                overlap_established: false,
                summability: ExternalClaimSummability::NotSummable,
                lineage_ids: Vec::new(),
            },
            corroborating_source_ids: Vec::new(),
            counterevidence_source_ids: Vec::new(),
            official_response: ExternalAccountabilityClaimOfficialResponse {
                request_status: ExternalClaimResponseRequestStatus::NotRecorded,
                requested_at: None,
                respondent: None,
                response_source_ids: Vec::new(),
                position: ExternalClaimResponsePosition::NoneRecorded,
            },
            claim_status: ExternalClaimStatus::IntakeUnverified,
            legal_or_administrative_status:
                ExternalClaimLegalOrAdministrativeStatus::NoneEstablished,
            review_status: ExternalClaimReviewStatus::Draft,
            status_as_of: "2026-07-14".to_string(),
            supersedes_record_id: None,
            correction_or_retraction_note: None,
            comparison_basis: "Publisher allegation; basis is not reconciled.".to_string(),
            due_process_caveat: "This quarantine row records an attributed allegation and does not establish misconduct, fraud, debt, recovery, or savings.".to_string(),
            use_rule: EXTERNAL_ACCOUNTABILITY_CLAIM_INTAKE_USE_RULE.to_string(),
            claim_gates: ExternalAccountabilityClaimGates {
                attributed_claim_reporting_allowed: false,
                underlying_factual_claim_allowed: false,
                misconduct_signal_claim_allowed: false,
                official_finding_claim_allowed: false,
                performance_claim_allowed: false,
                fraud_claim_allowed: false,
                waste_claim_allowed: false,
                debt_claim_allowed: false,
                collectibility_claim_allowed: false,
                recovery_claim_allowed: false,
                prevention_claim_allowed: false,
                savings_estimate_allowed: false,
            },
        }
    }

    fn captured_attributed_claim_fixture() -> ExternalAccountabilityClaimIntakeRecord {
        let mut record = external_claim_intake_fixture();
        record.claimant.claimant_type = ExternalClaimantType::Witness;
        record.publications[0].custody_status = ExternalClaimCustodyStatus::OfficialCopyCaptured;
        record.publications[0].custody_path = Some("data/raw/example/testimony.pdf".to_string());
        record.publications[0].sha256 =
            Some("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string());
        record.claim_atom.exact_text_verified = true;
        record.claim_atom.coverage_period = "source_defined_undetermined".to_string();
        record.amount_assertion.period = "source_defined_undetermined".to_string();
        record.claim_status = ExternalClaimStatus::AttributedClaimSupported;
        record.review_status = ExternalClaimReviewStatus::SourceReviewed;
        record.official_response.respondent = Some("Named Organization".to_string());
        record.due_process_caveat = "Custody proves only the attributed assertion; no response is recorded, and the row does not establish an underlying payment or wrongdoing.".to_string();
        record
    }

    fn captured_attributed_claim_with_context_fixture() -> ExternalAccountabilityClaimIntakeRecord {
        let mut record = captured_attributed_claim_fixture();
        record
            .publications
            .push(ExternalAccountabilityClaimPublication {
                publication_kind: ExternalClaimPublicationKind::Dataset,
                publisher: "Independent Official Publisher".to_string(),
                published_date: Some("2026-04-22".to_string()),
                observed_date: "2026-07-14".to_string(),
                source_id: "SRC-OFFICIAL-CONTEXT".to_string(),
                source_url: "https://example.test/official-context.pdf".to_string(),
                custody_path: Some("data/raw/example/official-context.pdf".to_string()),
                sha256: Some(
                    "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789".to_string(),
                ),
                custody_status: ExternalClaimCustodyStatus::OfficialCopyCaptured,
                evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
            });
        record
    }

    fn breadth_benchmark_fixture() -> BreadthBenchmarkRecord {
        BreadthBenchmarkRecord {
            record_id: "breadth:test".to_string(),
            record_family: BREADTH_BENCHMARK_RECORD_FAMILY.to_string(),
            lane_id: "health".to_string(),
            metric_label: "Health spending share".to_string(),
            depth_tier: "tier_1_full".to_string(),
            coverage_status: "full_comparison".to_string(),
            current_value: Some(17.2),
            current_unit: "percent_gdp".to_string(),
            current_period: "CY2024".to_string(),
            current_basis: "matched OECD basis".to_string(),
            benchmark_low: Some(9.3),
            benchmark_high: Some(9.3),
            benchmark_unit: "percent_gdp".to_string(),
            benchmark_period: "CY2024".to_string(),
            benchmark_type: "oecd_average".to_string(),
            gap_direction: "above_benchmark".to_string(),
            comparability_grade: "A".to_string(),
            source_ids: vec!["SRC-OECD-HEALTH-2025".to_string()],
            efficiency_gap_status: "observed_comparison_not_causal".to_string(),
            improper_payment_amount_millions: None,
            improper_payment_rate_percent: None,
            improper_payment_scope: "none_attached".to_string(),
            fraud_amount_millions: None,
            fraud_status: "not_measured_not_inferred".to_string(),
            recoverable_savings_millions: None,
            savings_status: "blocked_not_scored".to_string(),
            next_depth_need: "Separate price and utilization effects.".to_string(),
        }
    }

    #[test]
    fn validates_breadth_benchmark_boundary() {
        assert_eq!(breadth_benchmark_fixture().validate(), Ok(()));
    }

    #[test]
    fn blocks_world_comparison_fraud_assumption() {
        let mut row = breadth_benchmark_fixture();
        row.fraud_amount_millions = Some(100.0);
        row.fraud_status = "inferred_from_peer_gap".to_string();
        assert!(row.validate().is_err());
    }

    #[test]
    fn blocks_benchmark_unit_mismatch() {
        let mut row = breadth_benchmark_fixture();
        row.benchmark_unit = "percent_federal_outlays".to_string();
        assert!(row.validate().is_err());
    }

    #[test]
    fn validates_headline_basis_boundary() {
        let row = HeadlineBasisRecord {
            record_id: "headline:interest:net".to_string(),
            record_family: HEADLINE_BASIS_RECORD_FAMILY.to_string(),
            comparison_group: "interest".to_string(),
            measure_label: "Net interest".to_string(),
            value: 970065.0,
            unit: "millions_usd".to_string(),
            period: "FY2025".to_string(),
            government_scope: "us_federal".to_string(),
            accounting_scope: "OMB function 900".to_string(),
            source_ids: vec!["SRC-OMB-HIST-3-2-FY2027".to_string()],
            headline_use: "canonical".to_string(),
            substitution_status: "not_interchangeable".to_string(),
            cannot_substitute_for: vec!["headline:interest:gross".to_string()],
            interpretation: "Use for the additive budget lane.".to_string(),
        };
        assert_eq!(row.validate(), Ok(()));
    }

    #[test]
    fn blocks_interchangeable_headline_claim() {
        let mut row = HeadlineBasisRecord {
            record_id: "headline:defense:function".to_string(),
            record_family: HEADLINE_BASIS_RECORD_FAMILY.to_string(),
            comparison_group: "defense".to_string(),
            measure_label: "National Defense".to_string(),
            value: 916140.0,
            unit: "millions_usd".to_string(),
            period: "FY2025".to_string(),
            government_scope: "us_federal".to_string(),
            accounting_scope: "OMB function 050".to_string(),
            source_ids: vec!["SRC-OMB-HIST-3-2-FY2027".to_string()],
            headline_use: "canonical".to_string(),
            substitution_status: "not_interchangeable".to_string(),
            cannot_substitute_for: vec!["headline:defense:dod".to_string()],
            interpretation: "Use for the whole defense lane.".to_string(),
        };
        row.substitution_status = "interchangeable".to_string();
        assert!(row.validate().is_err());
    }

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
    fn blocks_partial_response_log_without_evidence() {
        let record = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::PartialEvidenceResponse,
            evidence_received: Vec::new(),
            missing_evidence: "Role review remains missing.".to_string(),
            claim_gate: "Public claim blocked.".to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn applies_valid_response_intake_to_response_log() {
        let log_record = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };
        let intake = PerformanceDemandResponseIntakeRecord {
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

        let updated = log_record.apply_intake(&intake).unwrap();

        assert_eq!(
            updated.response_class,
            PerformanceDemandResponseLogClass::PartialEvidenceResponse
        );
        assert_eq!(updated.evidence_received, vec!["audit memo URL"]);
        assert_eq!(
            updated.next_action,
            "Ask a narrower follow-up for the missing item."
        );
        assert_eq!(updated.public_claim_allowed, false);
        assert_eq!(updated.validate(), Ok(()));
    }

    #[test]
    fn blocks_response_intake_record_id_mismatch() {
        let log_record = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };
        let intake = PerformanceDemandResponseIntakeRecord {
            record_id: "accountability-evidence:other".to_string(),
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

        assert!(log_record.apply_intake(&intake).is_err());
    }

    #[test]
    fn exposes_response_rubric_classes() {
        assert_eq!(
            PerformanceDemandResponseLogClass::all_classes()
                .iter()
                .map(PerformanceDemandResponseLogClass::wire_value)
                .collect::<Vec<_>>(),
            vec![
                "not-yet-received",
                "complete-evidence-response",
                "partial-evidence-response",
                "process-only-response",
                "no-evidence-response",
            ]
        );

        let classes = PerformanceDemandResponseLogClass::rubric_classes();

        assert_eq!(
            classes,
            &[
                PerformanceDemandResponseLogClass::CompleteEvidenceResponse,
                PerformanceDemandResponseLogClass::PartialEvidenceResponse,
                PerformanceDemandResponseLogClass::ProcessOnlyResponse,
                PerformanceDemandResponseLogClass::NoEvidenceResponse,
            ]
        );
        assert_eq!(classes[0].label(), "Complete evidence response");
        assert_eq!(
            classes[0].rubric_next_action(),
            "Route to role review before any public claim."
        );
    }

    #[test]
    fn builds_response_status_from_log_records() {
        let records = vec![PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: "Public claim blocked.".to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        }];

        let status = PerformanceDemandResponseStatus::from_response_log_records(
            "data/derived/accountability_evidence/performance-demand-response-log.jsonl",
            &records,
        )
        .unwrap();

        assert_eq!(
            status,
            PerformanceDemandResponseStatus {
                artifact:
                    "data/derived/accountability_evidence/performance-demand-response-log.jsonl"
                        .to_string(),
                total_rows: 1,
                not_yet_received: 1,
                public_claim_allowed: 0,
                public_claim_blocked: 1,
                use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
            }
        );
        status.validate().unwrap();
    }

    #[test]
    fn blocks_response_status_count_mismatch() {
        let status = PerformanceDemandResponseStatus {
            artifact: "data/derived/accountability_evidence/performance-demand-response-log.jsonl"
                .to_string(),
            total_rows: 2,
            not_yet_received: 1,
            public_claim_allowed: 1,
            public_claim_blocked: 0,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(status.validate().is_err());
    }

    fn bundle_manifest_fixture_artifacts() -> Vec<PerformanceDemandResponseBundleArtifact> {
        [
            (
                "data/derived/accountability_evidence/performance-demand-response-intake.example.jsonl",
                "Source-custodied intake fixture row.",
                "jsonl",
                "1",
                "Exercise importer parsing and record-id matching.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl",
                "Response-log rows after applying example intake.",
                "jsonl",
                "2",
                "Inspect typed applied rows without changing canonical response status.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-status.applied-example.json",
                "Compact applied response counts.",
                "json",
                "n/a",
                "Feed fixture counts into UI/API tests without recomputing rows.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-dashboard.applied-example.md",
                "Human-readable applied response counts.",
                "markdown",
                "n/a",
                "Scan importer behavior without opening JSON.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-handoff.applied-example.md",
                "Task routing for the applied fixture set.",
                "markdown",
                "n/a",
                "Choose the right applied artifact by implementation task.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-applied-example.schema.md",
                "Fixture artifact contract.",
                "markdown",
                "n/a",
                "Confirm roles and guardrails for applied importer artifacts.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.md",
                "Human-readable changed fields.",
                "markdown",
                "n/a",
                "Inspect row-level changes after applying example intake.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.jsonl",
                "Machine-readable changed fields.",
                "jsonl",
                "1",
                "Feed delta rows into UI/API diff consumers.",
            ),
            (
                "data/derived/accountability_evidence/performance-demand-response-delta.applied-example.schema.md",
                "Delta row field contract.",
                "markdown",
                "n/a",
                "Confirm field meanings and blocked-claim guardrails.",
            ),
        ]
        .into_iter()
        .map(
            |(artifact, role, kind, row_count, consumer_use)| {
                PerformanceDemandResponseBundleArtifact {
                artifact: artifact.to_string(),
                role: role.to_string(),
                kind: kind.to_string(),
                row_count: row_count.to_string(),
                sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
                consumer_use: consumer_use.to_string(),
                }
            },
        )
        .collect()
    }

    #[test]
    fn validates_response_bundle_manifest() {
        let status = PerformanceDemandResponseStatus {
            artifact: "data/derived/accountability_evidence/performance-demand-response-log.applied-example.jsonl".to_string(),
            total_rows: 2,
            not_yet_received: 1,
            public_claim_allowed: 0,
            public_claim_blocked: 2,
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        let manifest = PerformanceDemandResponseBundleManifest::from_status(
            "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json",
            &status,
            bundle_manifest_fixture_artifacts(),
        )
        .expect("valid bundle manifest");

        assert_eq!(manifest.updated_rows, 1);
        assert_eq!(manifest.validate(), Ok(()));
    }

    #[test]
    fn blocks_response_bundle_manifest_public_claim_bypass() {
        let mut manifest = PerformanceDemandResponseBundleManifest {
            artifact: "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json".to_string(),
            bundle_kind: "applied-response-importer-fixture".to_string(),
            total_rows: 2,
            updated_rows: 1,
            public_claim_allowed: 1,
            public_claim_blocked: 1,
            artifacts: bundle_manifest_fixture_artifacts(),
            boundary: "Importer fixture only; not canonical response status, public-claim eligibility, misconduct finding, performance finding, or proof of reform benefits.".to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(manifest.validate().is_err());
        manifest.public_claim_allowed = 0;
        manifest.public_claim_blocked = 2;
        assert_eq!(manifest.validate(), Ok(()));
    }

    #[test]
    fn blocks_response_bundle_manifest_missing_artifact() {
        let mut artifacts = bundle_manifest_fixture_artifacts();
        artifacts.pop();
        let manifest = PerformanceDemandResponseBundleManifest {
            artifact: "data/derived/accountability_evidence/performance-demand-response-bundle.applied-example.json".to_string(),
            bundle_kind: "applied-response-importer-fixture".to_string(),
            total_rows: 2,
            updated_rows: 1,
            public_claim_allowed: 0,
            public_claim_blocked: 2,
            artifacts,
            boundary: "Importer fixture only; not canonical response status, public-claim eligibility, misconduct finding, performance finding, or proof of reform benefits.".to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(manifest.validate().is_err());
    }

    #[test]
    fn builds_response_delta_rows_from_log_records() {
        let before = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };
        let after = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::PartialEvidenceResponse,
            evidence_received: vec!["audit memo URL".to_string()],
            missing_evidence: "Role-approved public wording remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: "Ask a narrower follow-up for the missing item.".to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        let delta =
            PerformanceDemandResponseDeltaRow::from_response_log_records(&[before], &[after])
                .unwrap();

        assert_eq!(
            delta,
            vec![PerformanceDemandResponseDeltaRow {
                record_id: "accountability-evidence:test".to_string(),
                before_response_class: PerformanceDemandResponseLogClass::NotYetReceived,
                after_response_class: PerformanceDemandResponseLogClass::PartialEvidenceResponse,
                before_evidence_received_count: 0,
                after_evidence_received_count: 1,
                missing_evidence_changed: true,
                next_action_changed: true,
                before_claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
                after_claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            }]
        );
    }

    #[test]
    fn blocks_response_delta_mismatched_rows() {
        let before = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:test".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::NotYetReceived,
            evidence_received: Vec::new(),
            missing_evidence: "Requested evidence remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: PERFORMANCE_DEMAND_RESPONSE_LOG_NEXT_ACTION.to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };
        let after = PerformanceDemandResponseLogRecord {
            record_id: "accountability-evidence:other".to_string(),
            lane_id: Some("health".to_string()),
            program_or_account_id: Some("omb-function-550".to_string()),
            response_class: PerformanceDemandResponseLogClass::PartialEvidenceResponse,
            evidence_received: vec!["audit memo URL".to_string()],
            missing_evidence: "Role-approved public wording remains missing.".to_string(),
            claim_gate: PUBLIC_CLAIM_BLOCKED_LABEL.to_string(),
            public_claim_allowed: false,
            next_action: "Ask a narrower follow-up for the missing item.".to_string(),
            use_rule: PERFORMANCE_DEMAND_RESPONSE_LOG_USE_RULE.to_string(),
        };

        assert!(
            PerformanceDemandResponseDeltaRow::from_response_log_records(&[before], &[after])
                .is_err()
        );
    }

    #[test]
    fn validates_external_claim_intake_record() {
        assert_eq!(external_claim_intake_fixture().validate(), Ok(()));
    }

    #[test]
    fn validates_captured_attributed_claim_record() {
        assert_eq!(captured_attributed_claim_fixture().validate(), Ok(()));
    }

    #[test]
    fn validates_captured_official_context_without_corroboration() {
        let record = captured_attributed_claim_with_context_fixture();
        assert_eq!(
            record.claim_status,
            ExternalClaimStatus::AttributedClaimSupported
        );
        assert_eq!(
            record.legal_or_administrative_status,
            ExternalClaimLegalOrAdministrativeStatus::NoneEstablished
        );
        assert!(record.corroborating_source_ids.is_empty());
        assert!(record.counterevidence_source_ids.is_empty());
        assert!(record.official_response.response_source_ids.is_empty());
        assert!(record.claim_gates.all_false());
        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_context_source_misclassified_as_corroboration() {
        let mut record = captured_attributed_claim_with_context_fixture();
        record.corroborating_source_ids = vec!["SRC-OFFICIAL-CONTEXT".to_string()];
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_captured_attributed_claim_without_path_or_hash() {
        let mut record = captured_attributed_claim_fixture();
        record.publications[0].custody_path = None;
        assert!(record.validate().is_err());

        let mut record = captured_attributed_claim_fixture();
        record.publications[0].sha256 = None;
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_attributed_claim_without_origin_verification_or_review() {
        let mut record = captured_attributed_claim_fixture();
        record.publications[0].custody_status = ExternalClaimCustodyStatus::UrlObservedNotCaptured;
        record.publications[0].custody_path = None;
        record.publications[0].sha256 = None;
        assert!(record.validate().is_err());

        let mut record = captured_attributed_claim_fixture();
        record.claim_atom.exact_text_verified = false;
        assert!(record.validate().is_err());

        let mut record = captured_attributed_claim_fixture();
        record.review_status = ExternalClaimReviewStatus::Draft;
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_attributed_claim_without_named_respondent_state() {
        let mut record = captured_attributed_claim_fixture();
        record.official_response.respondent = None;
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_external_claim_gate_bypass() {
        let mut record = external_claim_intake_fixture();
        record.claim_gates.fraud_claim_allowed = true;
        assert!(record.validate().is_err());

        let mut record = captured_attributed_claim_fixture();
        record.claim_gates.attributed_claim_reporting_allowed = true;
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_exact_external_claim_verification_without_custody() {
        let mut record = external_claim_intake_fixture();
        record.claim_atom.exact_text_verified = true;
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_external_claim_invalid_date() {
        let mut record = external_claim_intake_fixture();
        record.status_as_of = "July 14, 2026".to_string();
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_external_claim_invalid_calendar_month() {
        let mut record = external_claim_intake_fixture();
        record.publications[0].published_date = Some("2026-13".to_string());
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_external_claim_date_chronology() {
        let mut record = external_claim_intake_fixture();
        record.publications[0].published_date = Some("2026-07-15".to_string());
        assert!(record.validate().is_err());

        let mut record = external_claim_intake_fixture();
        record.publications[0].observed_date = "2026-07-15".to_string();
        assert!(record.validate().is_err());
    }

    #[test]
    fn blocks_external_claim_invalid_amount_summability_and_overlap() {
        let mut record = external_claim_intake_fixture();
        record.amount_assertion.lower_bound = Some(9.0);
        assert!(record.validate().is_err());

        let mut record = external_claim_intake_fixture();
        record.amount_assertion.overlap_established = true;
        assert!(record.validate().is_err());

        let mut record = external_claim_intake_fixture();
        record.amount_assertion.lineage_ids = vec!["unsupported-lineage".to_string()];
        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_external_claim_exact_and_range_amount_shapes() {
        let mut exact = external_claim_intake_fixture();
        exact.amount_assertion.derivation = ExternalClaimAmountDerivation::SourceStatedExact;
        exact.amount_assertion.lower_bound = None;
        assert_eq!(exact.validate(), Ok(()));

        let mut range = external_claim_intake_fixture();
        range.amount_assertion.derivation = ExternalClaimAmountDerivation::SourceStatedRange;
        range.amount_assertion.lower_bound = Some(5.0);
        range.amount_assertion.upper_bound = Some(15.0);
        assert_eq!(range.validate(), Ok(()));
    }

    #[test]
    fn blocks_official_hosting_to_finding_bypass() {
        let mut record = external_claim_intake_fixture();
        record
            .publications
            .push(ExternalAccountabilityClaimPublication {
                publication_kind: ExternalClaimPublicationKind::WrittenTestimony,
                publisher: "U.S. House Committee on Example Affairs".to_string(),
                published_date: None,
                observed_date: "2026-07-14".to_string(),
                source_id: "SRC-HOUSE-EXAMPLE".to_string(),
                source_url: "https://example.test/house-testimony.pdf".to_string(),
                custody_path: None,
                sha256: None,
                custody_status: ExternalClaimCustodyStatus::UrlObservedNotCaptured,
                evidence_relation: ExternalClaimEvidenceRelation::SuppliesContext,
            });
        record.legal_or_administrative_status =
            ExternalClaimLegalOrAdministrativeStatus::OfficialFinding;
        record.claim_gates.official_finding_claim_allowed = true;
        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_response_intake_record_guardrails() {
        assert_eq!(
            PerformanceDemandResponseClass::all_classes()
                .iter()
                .map(PerformanceDemandResponseClass::wire_value)
                .collect::<Vec<_>>(),
            vec![
                "complete-evidence-response",
                "partial-evidence-response",
                "process-only-response",
                "no-evidence-response",
            ]
        );

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
    fn blocks_partial_response_intake_without_evidence() {
        let record = PerformanceDemandResponseIntakeRecord {
            record_id: "accountability-evidence:test".to_string(),
            reply_source_id: "SRC-REPLY-TEST".to_string(),
            reply_received_date: "2026-06-23".to_string(),
            sender_or_office: "Example Office".to_string(),
            response_class: PerformanceDemandResponseClass::PartialEvidenceResponse,
            evidence_received: Vec::new(),
            missing_evidence: "Role-approved public wording remains missing.".to_string(),
            role_review_needed: true,
            public_claim_allowed: false,
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

    #[test]
    fn validates_per_unit_ready_record_boundary() {
        let record = PerUnitDisplayReadinessRecord {
            record_id:
                "per-unit-ready:medicare-part-b-government-contribution-per-part-b-enrollee:cy2025"
                    .to_string(),
            record_family: "per_unit_display_readiness".to_string(),
            display_status: "ready_same_source_year_basis".to_string(),
            lane_id: "medicare-smi-part-b".to_string(),
            public_label: "Medicare Part B government contribution per Part B enrollee".to_string(),
            numerator_label: "CY2025 Part B government contribution".to_string(),
            numerator_value: 422_200_000_000.0,
            numerator_unit: "usd".to_string(),
            denominator_id: "medicare_part_b_enrollment".to_string(),
            denominator_value: Some(63_448_000.0),
            denominator_unit: "people".to_string(),
            computed_value_usd: Some(6654.27),
            year: "CY2025".to_string(),
            year_basis: "calendar_year".to_string(),
            source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            source_record_ids: vec![
                "medicare-part-financing:part-b:cy2025:cms-trustees-2026".to_string(),
                "denominator-value:medicare-part-b-enrollment:cy2025:cms-trustees-2026".to_string(),
            ],
            public_use_rule:
                "Allowed as CY2025 source-basis context; not an individual liability calculation."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_per_unit_blocked_record_with_amount() {
        let record = PerUnitDisplayReadinessRecord {
            record_id: "per-unit-blocked:medicare-hi-payroll-per-covered-worker".to_string(),
            record_family: "per_unit_display_readiness".to_string(),
            display_status: "blocked_missing_denominator".to_string(),
            lane_id: "medicare-hi".to_string(),
            public_label: "Medicare HI payroll financing per HI covered worker".to_string(),
            numerator_label: "HI payroll tax income".to_string(),
            numerator_value: 403_200_000_000.0,
            numerator_unit: "usd".to_string(),
            denominator_id: "medicare_hi_covered_workers".to_string(),
            denominator_value: Some(185_000_000.0),
            denominator_unit: "people".to_string(),
            computed_value_usd: Some(2179.46),
            year: "CY2025".to_string(),
            year_basis: "calendar_year".to_string(),
            source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            source_record_ids: vec![
                "medicare-part-financing:hi:cy2025:cms-trustees-2026".to_string(),
            ],
            public_use_rule: "Blocked until extracted. Do not substitute OASDI covered workers."
                .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_per_unit_receipt_card_boundary() {
        let record = PerUnitReceiptCardRecord {
            record_id: "per-unit-card:defense-outlays-per-resident".to_string(),
            record_family: "per_unit_receipt_cards".to_string(),
            source_readiness_record_id:
                "per-unit-ready:defense-outlays-per-resident:fy2025-omb-over-cy2025-census"
                    .to_string(),
            card_status: "illustrative_cross_basis".to_string(),
            lane_id: "national-defense".to_string(),
            headline: "Defense-Military FY2025 outlays equal about $2,541 per CY2025 resident as a civic-cost illustration.".to_string(),
            amount_usd: Some(2540.86),
            basis_label:
                "FY2025 OMB Defense-Military subfunction outlays divided by Census resident population"
                    .to_string(),
            visible_caveat:
                "Cross-basis illustration only. It is not equal tax liability, personal benefit, or legal dedication of income-tax dollars."
                    .to_string(),
            allowed_public_use:
                "Can appear as broad civic burden context with the basis visible.".to_string(),
            blocked_public_use:
                "Do not call it a personalized receipt or what each resident paid.".to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_per_unit_card_cross_basis_without_caveat() {
        let record = PerUnitReceiptCardRecord {
            record_id: "per-unit-card:gross-interest-outlays-per-resident".to_string(),
            record_family: "per_unit_receipt_cards".to_string(),
            source_readiness_record_id:
                "per-unit-ready:gross-interest-outlays-per-resident:fy2025-omb-over-cy2025-census"
                    .to_string(),
            card_status: "illustrative_cross_basis".to_string(),
            lane_id: "net-interest".to_string(),
            headline: "Gross Treasury interest FY2025 outlays equal about $3,557 per resident."
                .to_string(),
            amount_usd: Some(3556.66),
            basis_label: "FY2025 OMB over CY2025 Census population".to_string(),
            visible_caveat: "Cross-basis illustration only.".to_string(),
            allowed_public_use: "Can appear as broad civic debt-service context.".to_string(),
            blocked_public_use: "Do not call it a current program benefit.".to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_efficiency_pressure_boundary() {
        let record = EfficiencyPressureRecord {
            record_id: "efficiency-pressure:defense-fy2025".to_string(),
            record_family: "efficiency_pressure".to_string(),
            fiscal_year: 2025,
            surface: "Defense readiness and procurement efficiency".to_string(),
            related_spend_categories: vec!["spendcat-fy2025-005".to_string()],
            pressure_basis: vec![
                "large outlay share".to_string(),
                "strategic role caveat".to_string(),
            ],
            pressure_level: "high".to_string(),
            not_a_finding: true,
            cost_down_levers: vec![
                "procurement discipline".to_string(),
                "audit control closure".to_string(),
                "readiness-per-dollar measurement".to_string(),
            ],
            outcome_floor:
                "Savings must preserve readiness, alliance obligations, and service-member commitments."
                    .to_string(),
            evidence_needed: vec![
                "DOD budget justification".to_string(),
                "GAO weapon-system reports".to_string(),
                "readiness performance records".to_string(),
            ],
            public_claim_status: "blocked_question_surface_only".to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_efficiency_pressure_finding_claim() {
        let record = EfficiencyPressureRecord {
            record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            record_family: "efficiency_pressure".to_string(),
            fiscal_year: 2025,
            surface: "Health waste finding".to_string(),
            related_spend_categories: vec!["spendcat-fy2025-003".to_string()],
            pressure_basis: vec!["large outlay share".to_string(), "proves waste".to_string()],
            pressure_level: "highest".to_string(),
            not_a_finding: true,
            cost_down_levers: vec![
                "price discipline".to_string(),
                "drug pricing".to_string(),
                "administrative simplification".to_string(),
            ],
            outcome_floor: "Coverage must be preserved.".to_string(),
            evidence_needed: vec![
                "CMS source".to_string(),
                "CBO source".to_string(),
                "GAO source".to_string(),
            ],
            public_claim_status: "blocked_question_surface_only".to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_cost_down_backlog_boundary() {
        let record = CostDownBacklogRecord {
            record_id: "cost-down:health-medicare:price-discipline".to_string(),
            record_family: "cost_down_backlog".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            lever_id: "price-discipline".to_string(),
            lever_label: "Provider and procedure price discipline".to_string(),
            lever_type: "price_discipline".to_string(),
            action_question:
                "Which high-volume prices are above benchmark after quality and access controls?"
                    .to_string(),
            required_evidence: vec![
                "CMS price and utilization source".to_string(),
                "quality and access floor source".to_string(),
            ],
            measurement_metric: "price index versus benchmark with access and outcome floor"
                .to_string(),
            outcome_floor: "Coverage, access, and health outcomes must be preserved or improved."
                .to_string(),
            time_horizon: "medium_term".to_string(),
            estimated_savings_usd: None,
            savings_claim_status: "blocked_no_estimate".to_string(),
            public_use_rule:
                "Use as a work item only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_cost_down_backlog_savings_estimate() {
        let record = CostDownBacklogRecord {
            record_id: "cost-down:defense:procurement-control".to_string(),
            record_family: "cost_down_backlog".to_string(),
            source_pressure_record_id: "efficiency-pressure:defense-fy2025".to_string(),
            lane_id: "national-defense".to_string(),
            lever_id: "procurement-control".to_string(),
            lever_label: "Procurement control closure".to_string(),
            lever_type: "procurement_control".to_string(),
            action_question:
                "Which acquisition controls have reviewed evidence of avoidable cost growth?"
                    .to_string(),
            required_evidence: vec![
                "GAO weapon-system source".to_string(),
                "DOD acquisition baseline source".to_string(),
            ],
            measurement_metric: "cost growth against baseline with readiness floor".to_string(),
            outcome_floor: "Readiness and strategy commitments must remain preserved.".to_string(),
            time_horizon: "medium_term".to_string(),
            estimated_savings_usd: Some(1_000_000_000.0),
            savings_claim_status: "blocked_no_estimate".to_string(),
            public_use_rule:
                "Use as a work item only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_cost_down_source_packet_boundary() {
        let record = CostDownSourcePacketRecord {
            record_id: "cost-down-source-packet:health-medicare:price-discipline:v1".to_string(),
            record_family: "cost_down_source_packet".to_string(),
            source_backlog_record_id: "cost-down:health-medicare:price-discipline".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            packet_status: "reviewed_source_packet_no_savings_estimate".to_string(),
            source_ids: vec![
                "SRC-OECD-HEALTH-2025".to_string(),
                "SRC-JAMA-PAPANICOLAS-2018".to_string(),
                "SRC-CBO-LTBO".to_string(),
            ],
            evidence_summary: vec![
                "OECD benchmark supports high-level health cost pressure.".to_string(),
                "Peer-reviewed literature supports price and administration as drivers."
                    .to_string(),
            ],
            metric_candidates: vec![
                "government/compulsory health spend as percent of GDP".to_string(),
                "price and administration driver indicators".to_string(),
            ],
            outcome_floor_checks: vec![
                "coverage preserved or improved".to_string(),
                "access and outcome floor preserved".to_string(),
            ],
            missing_before_estimate: vec![
                "program-specific CMS price/utilization extraction".to_string(),
                "reviewed scoring method".to_string(),
            ],
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a source packet only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_cost_down_source_packet_claim_bypass() {
        let record = CostDownSourcePacketRecord {
            record_id: "cost-down-source-packet:health-medicare:price-discipline:v1".to_string(),
            record_family: "cost_down_source_packet".to_string(),
            source_backlog_record_id: "cost-down:health-medicare:price-discipline".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            packet_status: "reviewed_source_packet_no_savings_estimate".to_string(),
            source_ids: vec![
                "SRC-OECD-HEALTH-2025".to_string(),
                "SRC-JAMA-PAPANICOLAS-2018".to_string(),
                "SRC-CBO-LTBO".to_string(),
            ],
            evidence_summary: vec![
                "OECD benchmark supports high-level health cost pressure.".to_string(),
                "Peer-reviewed literature supports price and administration as drivers."
                    .to_string(),
            ],
            metric_candidates: vec![
                "government/compulsory health spend as percent of GDP".to_string(),
                "price and administration driver indicators".to_string(),
            ],
            outcome_floor_checks: vec![
                "coverage preserved or improved".to_string(),
                "access and outcome floor preserved".to_string(),
            ],
            missing_before_estimate: vec![
                "program-specific CMS price/utilization extraction".to_string(),
                "reviewed scoring method".to_string(),
            ],
            public_claim_allowed: true,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a source packet only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_cost_down_evidence_queue_boundary() {
        let record = CostDownEvidenceQueueRecord {
            record_id: "cost-down-evidence-queue:health-medicare:price-discipline:v1"
                .to_string(),
            record_family: "cost_down_evidence_queue".to_string(),
            source_packet_record_id: "cost-down-source-packet:health-medicare:price-discipline:v1"
                .to_string(),
            source_backlog_record_id: "cost-down:health-medicare:price-discipline".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            extraction_priority: "first_pass".to_string(),
            primary_source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            extract_question: "Which Medicare services have price and outcome data ready for a controlled comparison?".to_string(),
            first_extract: "CMS service-level price, utilization, and quality extract.".to_string(),
            extract_grain: "program-service-year".to_string(),
            query_lock_fields: vec![
                "source_id".to_string(),
                "observed_date".to_string(),
                "fiscal_or_calendar_year".to_string(),
            ],
            output_artifact_candidate:
                "data/derived/efficiency_pressure/extracts/health_price_first_pass.jsonl"
                    .to_string(),
            scoring_blockers: vec![
                "case-mix method".to_string(),
                "quality and access floor".to_string(),
            ],
            outcome_floor: "Coverage, access, and health outcomes must remain preserved."
                .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as an extraction queue row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_cost_down_evidence_queue_claim_bypass() {
        let record = CostDownEvidenceQueueRecord {
            record_id: "cost-down-evidence-queue:health-medicare:price-discipline:v1"
                .to_string(),
            record_family: "cost_down_evidence_queue".to_string(),
            source_packet_record_id: "cost-down-source-packet:health-medicare:price-discipline:v1"
                .to_string(),
            source_backlog_record_id: "cost-down:health-medicare:price-discipline".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            extraction_priority: "first_pass".to_string(),
            primary_source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            extract_question: "Which Medicare services have price and outcome data ready for a controlled comparison?".to_string(),
            first_extract: "CMS service-level price, utilization, and quality extract.".to_string(),
            extract_grain: "program-service-year".to_string(),
            query_lock_fields: vec![
                "source_id".to_string(),
                "observed_date".to_string(),
                "fiscal_or_calendar_year".to_string(),
            ],
            output_artifact_candidate:
                "data/derived/efficiency_pressure/extracts/health_price_first_pass.jsonl"
                    .to_string(),
            scoring_blockers: vec![
                "case-mix method".to_string(),
                "quality and access floor".to_string(),
            ],
            outcome_floor: "Coverage, access, and health outcomes must remain preserved."
                .to_string(),
            public_claim_allowed: true,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as an extraction queue row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_payment_integrity_portal_probe_boundary() {
        let record = PaymentIntegrityPortalProbeRecord {
            record_id: "payment-integrity-portal-probe:omb-paymentaccuracy:fps:2026-06-30"
                .to_string(),
            record_family: "payment_integrity_portal_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1".to_string(),
            source_id: "SRC-OMB-PAYMENTACCURACY".to_string(),
            observed_date: "2026-06-30".to_string(),
            page_url: "https://www.paymentaccuracy.gov/".to_string(),
            row_kind: "homepage_highest_performing_agency".to_string(),
            agency_code: "FPS".to_string(),
            agency_name: "Federal Permitting Improvement Steering Council".to_string(),
            high_priority_program_count: 0,
            improper_payment_percentage: 0.0,
            source_scope_note:
                "Homepage agency trend row only; not program-level improper-payment extraction."
                    .to_string(),
            next_extract_need:
                "Download program-year PaymentAccuracy data with methodology and root-cause fields."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a portal probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_payment_integrity_portal_probe_claim_bypass() {
        let record = PaymentIntegrityPortalProbeRecord {
            record_id: "payment-integrity-portal-probe:omb-paymentaccuracy:fps:2026-06-30"
                .to_string(),
            record_family: "payment_integrity_portal_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1".to_string(),
            source_id: "SRC-OMB-PAYMENTACCURACY".to_string(),
            observed_date: "2026-06-30".to_string(),
            page_url: "https://www.paymentaccuracy.gov/".to_string(),
            row_kind: "homepage_highest_performing_agency".to_string(),
            agency_code: "FPS".to_string(),
            agency_name: "Federal Permitting Improvement Steering Council".to_string(),
            high_priority_program_count: 0,
            improper_payment_percentage: 0.0,
            source_scope_note:
                "Homepage agency trend row only; not program-level improper-payment extraction."
                    .to_string(),
            next_extract_need:
                "Download program-year PaymentAccuracy data with methodology and root-cause fields."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: true,
            public_use_rule:
                "Use as a portal probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_payment_integrity_scorecard_probe_boundary() {
        let record = PaymentIntegrityScorecardProbeRecord {
            record_id: "payment-integrity-scorecard-probe:omb-paymentaccuracy:cms-part-d:q4-2025"
                .to_string(),
            record_family: "payment_integrity_scorecard_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1"
                    .to_string(),
            source_id: "SRC-OMB-PAYMENTACCURACY".to_string(),
            observed_date: "2026-06-30".to_string(),
            scorecard_url: "https://paymentaccuracy.gov/assets/scorecards/Q4%202025/Centers%20for%20Medicare%20%26%20Medicaid%20Services%20%28CMS%29%20-%20Medicare%20Prescription%20Drug%20Benefit%20%28Part%20D%29.pdf".to_string(),
            reporting_period: "Q4 2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            fy2024_overpayment_amount_millions: 3_053.0,
            fy2024_overpayment_rate_percent: 1.02,
            sample_period_note: "FY2024 overpayments; sample period and methodology remain source-specific.".to_string(),
            primary_root_cause_amount_millions: 2_403.0,
            root_cause_control_scope: "Administrative or process error made by other party".to_string(),
            root_cause_data_access_issue: "State data".to_string(),
            mitigation_strategy: "Engage states and stakeholders to improve Medicare Part D data access and reporting.".to_string(),
            source_scope_note: "Scorecard row only; not a savings estimate or finding.".to_string(),
            next_extract_need: "Download full program-year data with root-cause and methodology fields.".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a scorecard probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_payment_integrity_scorecard_probe_claim_bypass() {
        let record = PaymentIntegrityScorecardProbeRecord {
            record_id: "payment-integrity-scorecard-probe:omb-paymentaccuracy:cms-part-d:q4-2025"
                .to_string(),
            record_family: "payment_integrity_scorecard_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1"
                    .to_string(),
            source_id: "SRC-OMB-PAYMENTACCURACY".to_string(),
            observed_date: "2026-06-30".to_string(),
            scorecard_url: "https://paymentaccuracy.gov/assets/scorecards/Q4%202025/Centers%20for%20Medicare%20%26%20Medicaid%20Services%20%28CMS%29%20-%20Medicare%20Prescription%20Drug%20Benefit%20%28Part%20D%29.pdf".to_string(),
            reporting_period: "Q4 2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            fy2024_overpayment_amount_millions: 3_053.0,
            fy2024_overpayment_rate_percent: 1.02,
            sample_period_note: "FY2024 overpayments; sample period and methodology remain source-specific.".to_string(),
            primary_root_cause_amount_millions: 2_403.0,
            root_cause_control_scope: "Administrative or process error made by other party".to_string(),
            root_cause_data_access_issue: "State data".to_string(),
            mitigation_strategy: "Engage states and stakeholders to improve Medicare Part D data access and reporting.".to_string(),
            source_scope_note: "Scorecard row only; not a savings estimate or finding.".to_string(),
            next_extract_need: "Download full program-year data with root-cause and methodology fields.".to_string(),
            public_claim_allowed: true,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a scorecard probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_err());
    }

    #[test]
    fn validates_payment_integrity_program_review_gate_boundary() {
        let record = PaymentIntegrityProgramReviewGateRecord {
            record_id: "payment-integrity-program-review-gate:cms-part-d:q4-2025".to_string(),
            record_family: "payment_integrity_program_review_gate".to_string(),
            source_scorecard_record_id:
                "payment-integrity-scorecard-probe:omb-paymentaccuracy:cms-part-d:q4-2025"
                    .to_string(),
            source_readiness_record_id:
                "cost-down-scoring-readiness:payment-integrity:eligibility-accuracy:v1"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            reporting_period: "Q4 2025".to_string(),
            fy2024_overpayment_amount_millions: 3053.0,
            fy2024_overpayment_rate_percent: 1.02,
            methodology_status: "scorecard_probe_only_methodology_needed".to_string(),
            access_floor_status: "beneficiary_access_floor_needed".to_string(),
            corrective_action_status: "corrective_action_detail_needed".to_string(),
            confidence_limit_status: "confidence_limit_extract_needed".to_string(),
            claim_boundary_status: "blocked_before_public_claim".to_string(),
            required_next_evidence: vec![
                "methodology and sample design".to_string(),
                "beneficiary access, denial, appeal, reversal, and timeliness floor".to_string(),
                "corrective-action owner, milestone, and status".to_string(),
                "confidence limits and uncertainty fields".to_string(),
            ],
            review_gate_status: "blocked_before_savings_score".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a program-review gate only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_program_review_task_boundary() {
        let record = PaymentIntegrityProgramReviewTaskRecord {
            record_id: "payment-integrity-program-review-task:cms-part-d:methodology:q4-2025"
                .to_string(),
            record_family: "payment_integrity_program_review_task".to_string(),
            source_program_gate_record_id:
                "payment-integrity-program-review-gate:cms-part-d:q4-2025".to_string(),
            source_scorecard_record_id:
                "payment-integrity-scorecard-probe:omb-paymentaccuracy:cms-part-d:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            evidence_family: "methodology".to_string(),
            extraction_task:
                "Extract sample design, payment universe, estimation method, and exclusion rules."
                    .to_string(),
            target_source_or_system: "PaymentAccuracy scorecard and agency payment-integrity materials"
                .to_string(),
            completion_gate: "required_before_savings_score".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a program-review task only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_program_review_status_boundary() {
        let record = PaymentIntegrityProgramReviewStatusRecord {
            record_id: "payment-integrity-program-review-status:cms-part-d:q4-2025".to_string(),
            record_family: "payment_integrity_program_review_status".to_string(),
            source_program_gate_record_id:
                "payment-integrity-program-review-gate:cms-part-d:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            total_required_task_count: 4,
            completed_task_count: 0,
            blocked_task_count: 4,
            blocker_summary:
                "Methodology, access floor, corrective action, and uncertainty evidence remain open."
                    .to_string(),
            next_priority_task_family: "methodology".to_string(),
            next_priority_reason:
                "Methodology defines the payment universe before any control effect can be scored."
                    .to_string(),
            review_status: "blocked_before_savings_score".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a program-review status only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_plan_boundary() {
        let record = PaymentIntegrityMethodologyPlanRecord {
            record_id: "payment-integrity-methodology-plan:cms-part-d:q4-2025".to_string(),
            record_family: "payment_integrity_methodology_plan".to_string(),
            source_program_status_record_id:
                "payment-integrity-program-review-status:cms-part-d:q4-2025".to_string(),
            source_methodology_task_record_id:
                "payment-integrity-program-review-task:cms-part-d:methodology:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            required_methodology_fields: vec![
                "sample design".to_string(),
                "payment universe".to_string(),
                "estimation method".to_string(),
                "exclusion rules".to_string(),
                "sample period".to_string(),
                "payment type split".to_string(),
            ],
            source_discovery_targets: vec![
                "PaymentAccuracy scorecard methodology appendix".to_string(),
                "CMS improper-payment methodology documentation".to_string(),
            ],
            extraction_priority: 1,
            methodology_completion_rule:
                "Complete only when each required field is source-cited for the same reporting period."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology plan only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_field_boundary() {
        let record = PaymentIntegrityMethodologyFieldRecord {
            record_id: "payment-integrity-methodology-field:cms-part-d:sample-design:q4-2025"
                .to_string(),
            record_family: "payment_integrity_methodology_field".to_string(),
            source_methodology_plan_record_id:
                "payment-integrity-methodology-plan:cms-part-d:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample design".to_string(),
            field_status: "open_source_needed".to_string(),
            required_source_target: "PaymentAccuracy scorecard methodology appendix".to_string(),
            completion_rule: "Capture source citation, reporting period, and field text."
                .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology field checklist row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_source_target_boundary() {
        let record = PaymentIntegrityMethodologySourceTargetRecord {
            record_id: "payment-integrity-methodology-source-target:cms-part-d:paymentaccuracy-methodology:q4-2025".to_string(),
            record_family: "payment_integrity_methodology_source_target".to_string(),
            source_methodology_plan_record_id:
                "payment-integrity-methodology-plan:cms-part-d:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            source_target: "PaymentAccuracy scorecard methodology appendix".to_string(),
            target_priority: 1,
            target_status: "open_source_needed".to_string(),
            target_use: "Find sample design, payment universe, estimate basis, and uncertainty fields."
                .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology source-target row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_query_boundary() {
        let record = PaymentIntegrityMethodologyQueryRecord {
            record_id:
                "payment-integrity-methodology-query:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_query".to_string(),
            source_methodology_target_record_id:
                "payment-integrity-methodology-source-target:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            query_text: "PaymentAccuracy Q4 2025 Medicare Part D methodology appendix".to_string(),
            query_scope: "official PaymentAccuracy, OMB, HHS, and CMS sources".to_string(),
            capture_rule: "Capture canonical URL, observed date, reporting period, and methodology fields."
                .to_string(),
            query_status: "open_not_executed".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology query row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_query_run_boundary() {
        let record = PaymentIntegrityMethodologyQueryRunRecord {
            record_id:
                "payment-integrity-methodology-query-run:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_query_run".to_string(),
            source_methodology_query_record_id:
                "payment-integrity-methodology-query:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            run_status: "pending_not_run".to_string(),
            planned_query_text: "PaymentAccuracy Q4 2025 Medicare Part D methodology appendix"
                .to_string(),
            result_capture_status: "no_result_captured".to_string(),
            required_capture_fields: vec![
                "canonical URL".to_string(),
                "observed date".to_string(),
                "source title".to_string(),
                "methodology field text".to_string(),
            ],
            next_run_rule:
                "Run query against official sources and create a source extract only after source text is captured."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology query-run row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_result_boundary() {
        let record = PaymentIntegrityMethodologyResultRecord {
            record_id:
                "payment-integrity-methodology-result:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_result".to_string(),
            source_methodology_query_run_record_id:
                "payment-integrity-methodology-query-run:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            observed_date: "2026-07-01".to_string(),
            source_url: "https://paymentaccuracy.gov/assets/scorecards/Q4%202025/Centers%20for%20Medicare%20%26%20Medicaid%20Services%20%28CMS%29%20-%20Medicare%20Prescription%20Drug%20Benefit%20%28Part%20D%29.pdf".to_string(),
            source_title: "Payment Integrity Scorecard: Medicare Prescription Drug Benefit (Part D), Q4 2025".to_string(),
            reporting_period: "Q4 2025".to_string(),
            captured_methodology_text:
                "Scorecard reports FY2024 overpayment amount and notes the estimate is based on a 1/2022-12/2022 sampling timeframe."
                    .to_string(),
            captured_field_scope: vec![
                "reporting period".to_string(),
                "sample period".to_string(),
                "overpayment amount".to_string(),
            ],
            field_closure_allowed: false,
            result_status: "source_captured_review_needed".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology result row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_result_review_readiness_boundary() {
        let record = PaymentIntegrityMethodologyResultReviewReadinessRecord {
            record_id:
                "payment-integrity-methodology-result-review-readiness:va-pltss:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_result_review_readiness".to_string(),
            source_methodology_result_record_ids: vec![
                "payment-integrity-methodology-result:va-pltss:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
                "payment-integrity-methodology-result:va-pltss:afr-section-iii:q4-2025"
                    .to_string(),
            ],
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)".to_string(),
            source_capture_count: 2,
            review_readiness_status: "ready_for_field_review_queue".to_string(),
            next_field_review_count: 2,
            next_methodology_fields: vec![
                "sample design".to_string(),
                "reviewed-claim universe".to_string(),
            ],
            next_action: "Create field-review rows before closure.".to_string(),
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology result review-readiness row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_field_review_boundary() {
        let record = PaymentIntegrityMethodologyFieldReviewRecord {
            record_id:
                "payment-integrity-methodology-field-review:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_field_review".to_string(),
            source_methodology_result_record_id:
                "payment-integrity-methodology-result:cms-part-d:paymentaccuracy-methodology:q4-2025"
                    .to_string(),
            source_methodology_field_record_id:
                "payment-integrity-methodology-field:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample period".to_string(),
            evidence_status: "partial_support_review_needed".to_string(),
            reviewed_source_scope: "PaymentAccuracy Q4 2025 scorecard PDF".to_string(),
            review_note:
                "Scorecard states a sampling timeframe, but field closure still needs citation review."
                    .to_string(),
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology field-review row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_gap_followup_boundary() {
        let record = PaymentIntegrityMethodologyGapFollowupRecord {
            record_id:
                "payment-integrity-methodology-gap-followup:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_gap_followup".to_string(),
            source_methodology_field_review_record_id:
                "payment-integrity-methodology-field-review:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample design".to_string(),
            gap_class: "unsupported_field_source_needed".to_string(),
            followup_priority: 1,
            source_target: "CMS Part D improper-payment methodology documentation".to_string(),
            next_action: "Locate source text describing sample design.".to_string(),
            completion_evidence_required: vec![
                "source URL".to_string(),
                "observed date".to_string(),
                "field-specific quoted or summarized source text".to_string(),
            ],
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology gap-followup row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_gap_source_capture_boundary() {
        let record = PaymentIntegrityMethodologyGapSourceCaptureRecord {
            record_id:
                "payment-integrity-methodology-gap-source-capture:cms-part-d:sample-design:cms-fy2024-fact-sheet"
                    .to_string(),
            record_family: "payment_integrity_methodology_gap_source_capture".to_string(),
            source_methodology_gap_followup_record_id:
                "payment-integrity-methodology-gap-followup:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample design".to_string(),
            observed_date: "2026-07-01".to_string(),
            source_url:
                "https://www.cms.gov/newsroom/fact-sheets/fiscal-year-2024-improper-payments-fact-sheet"
                    .to_string(),
            source_title: "Fiscal Year 2024 Improper Payments Fact Sheet".to_string(),
            source_publisher: "Centers for Medicare & Medicaid Services".to_string(),
            captured_source_scope: "Medicare Part D improper payment measurements".to_string(),
            captured_methodology_summary:
                "CMS states that Part D IPM reviews a statistically valid stratified random sample of PDEs."
                    .to_string(),
            support_status: "partial_support_review_needed".to_string(),
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology gap source-capture row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_source_capture_rollup_boundary() {
        let record = PaymentIntegrityMethodologySourceCaptureRollupRecord {
            record_id:
                "payment-integrity-methodology-source-capture-rollup:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_source_capture_rollup".to_string(),
            source_methodology_gap_followup_record_id:
                "payment-integrity-methodology-gap-followup:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            source_methodology_gap_source_capture_record_id:
                "payment-integrity-methodology-gap-source-capture:cms-part-d:sample-design:cms-fy2024-fact-sheet"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample design".to_string(),
            capture_coverage_status: "source_captured_review_needed".to_string(),
            remaining_review_need:
                "Reviewer must decide whether the captured sample-design source text is sufficient for field closure."
                    .to_string(),
            reviewer_action:
                "Compare capture against checklist completion rule and either create closure decision or a new source gap."
                    .to_string(),
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology source-capture rollup row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_closure_readiness_boundary() {
        let record = PaymentIntegrityMethodologyClosureReadinessRecord {
            record_id:
                "payment-integrity-methodology-closure-readiness:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_closure_readiness".to_string(),
            source_methodology_source_capture_rollup_record_id:
                "payment-integrity-methodology-source-capture-rollup:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample period".to_string(),
            closure_readiness_status: "closure_review_candidate".to_string(),
            readiness_reason:
                "Captured scorecard text directly states the sampling timeframe, but reviewer closure is still required."
                    .to_string(),
            next_required_action:
                "Reviewer must compare captured text to completion rule and issue a separate closure decision."
                    .to_string(),
            field_closure_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology closure-readiness row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_closure_decision_boundary() {
        let record = PaymentIntegrityMethodologyClosureDecisionRecord {
            record_id:
                "payment-integrity-methodology-closure-decision:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_closure_decision".to_string(),
            source_methodology_closure_readiness_record_id:
                "payment-integrity-methodology-closure-readiness:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample period".to_string(),
            decision_status: "field_closed_internal_only".to_string(),
            field_closed: true,
            decision_basis:
                "Source capture directly states the FY2024 estimate sampling timeframe."
                    .to_string(),
            closure_scope: "Close sample-period field only for Part D methodology review."
                .to_string(),
            residual_limitations: vec![
                "No other methodology fields are closed by this decision.".to_string(),
                "No savings estimate or waste finding is allowed.".to_string(),
            ],
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as an internal methodology closure decision only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_residual_source_gap_boundary() {
        let record = PaymentIntegrityMethodologyResidualSourceGapRecord {
            record_id:
                "payment-integrity-methodology-residual-source-gap:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_residual_source_gap".to_string(),
            source_methodology_closure_readiness_record_id:
                "payment-integrity-methodology-closure-readiness:cms-part-d:sample-design:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            methodology_field: "sample design".to_string(),
            residual_gap_class: "detail_source_needed".to_string(),
            source_need: "Sample size and selection method text".to_string(),
            next_query_text: "CMS Part D IPM sample size selection method FY2024".to_string(),
            closure_blocked_reason:
                "Captured source supports sampling type but not enough detail for closure."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology residual source-gap row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_closure_coverage_boundary() {
        let record = PaymentIntegrityMethodologyClosureCoverageRecord {
            record_id: "payment-integrity-methodology-closure-coverage:cms-part-d:q4-2025"
                .to_string(),
            record_family: "payment_integrity_methodology_closure_coverage".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            source_methodology_closure_decision_record_id:
                "payment-integrity-methodology-closure-decision:cms-part-d:sample-period:q4-2025"
                    .to_string(),
            total_methodology_fields: 8,
            closed_field_count: 1,
            open_field_count: 7,
            closed_fields: vec!["sample period".to_string()],
            open_fields: vec![
                "sample design".to_string(),
                "payment universe".to_string(),
                "estimation method".to_string(),
                "exclusion rules".to_string(),
                "payment type split".to_string(),
                "state-data dependency treatment".to_string(),
                "overpayment versus recoverable amount basis".to_string(),
            ],
            coverage_status: "partial_methodology_closure".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology closure-coverage row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_scoring_gate_boundary() {
        let record = PaymentIntegrityMethodologyScoringGateRecord {
            record_id: "payment-integrity-methodology-scoring-gate:cms-part-d:q4-2025"
                .to_string(),
            record_family: "payment_integrity_methodology_scoring_gate".to_string(),
            source_methodology_closure_coverage_record_id:
                "payment-integrity-methodology-closure-coverage:cms-part-d:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            gate_status: "blocked_methodology_incomplete".to_string(),
            gate_reason: "Only one of eight methodology fields is internally closed.".to_string(),
            blockers: vec!["7 open methodology fields".to_string()],
            next_milestone: "Close residual source gaps before scoring.".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology scoring-gate row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_program_rollup_boundary() {
        let record = PaymentIntegrityMethodologyProgramRollupRecord {
            record_id: "payment-integrity-methodology-program-rollup:cms-part-d:q4-2025"
                .to_string(),
            record_family: "payment_integrity_methodology_program_rollup".to_string(),
            source_methodology_scoring_gate_record_id:
                "payment-integrity-methodology-scoring-gate:cms-part-d:q4-2025".to_string(),
            source_methodology_closure_coverage_record_id:
                "payment-integrity-methodology-closure-coverage:cms-part-d:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            total_methodology_fields: 8,
            closed_field_count: 1,
            open_field_count: 7,
            scoring_gate_status: "blocked_methodology_incomplete".to_string(),
            next_open_methodology_fields: vec![
                "sample design".to_string(),
                "payment universe".to_string(),
                "estimation method".to_string(),
                "exclusion rules".to_string(),
                "payment type split".to_string(),
                "state-data dependency treatment".to_string(),
                "overpayment versus recoverable amount basis".to_string(),
            ],
            next_action: "Resolve residual source gaps before scoring.".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology program rollup only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_open_program_status_boundary() {
        let record = PaymentIntegrityMethodologyOpenProgramStatusRecord {
            record_id: "payment-integrity-methodology-open-program-status:va-pltss:q4-2025"
                .to_string(),
            record_family: "payment_integrity_methodology_open_program_status".to_string(),
            source_methodology_plan_record_id:
                "payment-integrity-methodology-plan:va-pltss:q4-2025".to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)".to_string(),
            closure_path_status: "fully_open_no_closure_decision".to_string(),
            total_methodology_fields: 8,
            closed_field_count: 0,
            open_field_count: 8,
            closure_decision_count: 0,
            residual_source_gap_count: 8,
            blocker_summary: "All eight methodology fields remain open.".to_string(),
            next_priority: "Resolve residual source gaps before any scoring gate.".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology open-program status row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_residual_gap_priority_boundary() {
        let record = PaymentIntegrityMethodologyResidualGapPriorityRecord {
            record_id: "payment-integrity-methodology-residual-gap-priority:va-pltss:documentation-defect-versus-recoverable-overpayment-basis:q4-2025".to_string(),
            record_family: "payment_integrity_methodology_residual_gap_priority".to_string(),
            source_open_program_status_record_id:
                "payment-integrity-methodology-open-program-status:va-pltss:q4-2025".to_string(),
            source_residual_source_gap_record_id: "payment-integrity-methodology-residual-source-gap:va-pltss:documentation-defect-versus-recoverable-overpayment-basis:q4-2025".to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)".to_string(),
            priority_rank: 2,
            selected_methodology_field:
                "documentation defect versus recoverable overpayment basis".to_string(),
            priority_reason:
                "Resolve recoverability before treating documentation defects as savings."
                    .to_string(),
            next_query_text: "site:department.va.gov PLTSS recoverable overpayment documentation defect collections improper unknown payments".to_string(),
            resolution_rule:
                "Close only with an official source distinguishing documentation defects from recoverable overpayments."
                    .to_string(),
            blocked_claims_note:
                "Until resolved, block scoring, savings estimates, and waste claims for this program."
                    .to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology residual-gap priority row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_priority_source_work_boundary() {
        let record = PaymentIntegrityMethodologyPrioritySourceWorkRecord {
            record_id: "payment-integrity-methodology-priority-source-work:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025".to_string(),
            record_family: "payment_integrity_methodology_priority_source_work".to_string(),
            source_residual_gap_priority_record_id: "payment-integrity-methodology-residual-gap-priority:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicaid".to_string(),
            priority_rank: 3,
            selected_methodology_field: "improper payment versus fraud/waste basis".to_string(),
            observed_date: "2026-07-01".to_string(),
            source_work_status: "boundary_source_captured_review_needed".to_string(),
            official_source_urls: vec![
                "https://www.cms.gov/files/document/cms-financial-report-fiscal-year-2024.pdf"
                    .to_string(),
            ],
            source_summary:
                "CMS source language supports improper-payment boundary review for Medicaid."
                    .to_string(),
            resolution_effect:
                "Use as partial boundary support, not as field closure or scoring support."
                    .to_string(),
            remaining_blocker:
                "Reviewer still needs a closure decision before any scoring or savings claim."
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology priority source-work row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_priority_reviewer_action_boundary() {
        let record = PaymentIntegrityMethodologyPriorityReviewerActionRecord {
            record_id: "payment-integrity-methodology-priority-reviewer-action:usda-federal-crop-insurance:agency-process-error-definition:q4-2025".to_string(),
            record_family: "payment_integrity_methodology_priority_reviewer_action".to_string(),
            source_priority_source_work_record_id: "payment-integrity-methodology-priority-source-work:usda-federal-crop-insurance:agency-process-error-definition:q4-2025".to_string(),
            agency_code: "USDA".to_string(),
            program_or_activity: "Federal Crop Insurance Program".to_string(),
            priority_rank: 1,
            selected_methodology_field: "agency-process-error definition".to_string(),
            reviewer_action_status: "field_reframing_approved_internal_only".to_string(),
            reviewer_action:
                "Reframe the checklist field to current scorecard root-cause language."
                    .to_string(),
            field_reframing_allowed: true,
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            next_required_artifact: "payment_integrity_methodology_field_update".to_string(),
            public_use_rule:
                "Use as a methodology priority reviewer-action row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_field_update_boundary() {
        let record = PaymentIntegrityMethodologyFieldUpdateRecord {
            record_id:
                "payment-integrity-methodology-field-update:usda-federal-crop-insurance:agency-process-error-definition:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_field_update".to_string(),
            source_priority_reviewer_action_record_id: "payment-integrity-methodology-priority-reviewer-action:usda-federal-crop-insurance:agency-process-error-definition:q4-2025".to_string(),
            source_methodology_field_record_id:
                "payment-integrity-methodology-field:usda-federal-crop-insurance:agency-process-error-definition:q4-2025"
                    .to_string(),
            agency_code: "USDA".to_string(),
            program_or_activity: "Federal Crop Insurance Program".to_string(),
            old_methodology_field: "agency-process-error definition".to_string(),
            revised_methodology_field:
                "data-access outside-agency-control root-cause definition".to_string(),
            old_required_source_target:
                "USDA/RMA Federal Crop Insurance improper-payment methodology or quality-control documentation"
                    .to_string(),
            revised_required_source_target:
                "PaymentAccuracy FCIC scorecard root-cause table and USDA/RMA support for data-access/outside-agency-control treatment"
                    .to_string(),
            old_completion_rule:
                "Capture source citation and how federal-agency process error is defined and distinguished from other root causes."
                    .to_string(),
            revised_completion_rule:
                "Capture source citation and how failure or inability to access data or information is classified and distinguished from recoverable overpayments."
                    .to_string(),
            update_status: "field_reframed_internal_only".to_string(),
            update_scope:
                "Repair the methodology checklist field label and source target only.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology field-update row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_source_query_boundary() {
        let record = PaymentIntegrityMethodologyFollowupSourceQueryRecord {
            record_id:
                "payment-integrity-methodology-followup-source-query:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_source_query".to_string(),
            source_priority_reviewer_action_record_id:
                "payment-integrity-methodology-priority-reviewer-action:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            source_field_update_record_id: None,
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            priority_rank: 4,
            query_objective:
                "Find official recoverable or collectible basis for reported overpayments."
                    .to_string(),
            query_text:
                "site:cms.gov Part D IPM recoverable collectible overpayment FY2024 PDE audit"
                    .to_string(),
            source_scope: "official CMS, HHS, and PaymentAccuracy sources".to_string(),
            capture_rule:
                "Capture source URL, observed date, recoverable amount basis, and limitation language."
                    .to_string(),
            success_rule:
                "Success requires official language separating reported overpayment estimates from collectible recoveries."
                    .to_string(),
            query_status: "open_not_executed".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up source-query row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_source_query_run_boundary() {
        let record = PaymentIntegrityMethodologyFollowupSourceQueryRunRecord {
            record_id:
                "payment-integrity-methodology-followup-source-query-run:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_source_query_run".to_string(),
            source_followup_source_query_record_id:
                "payment-integrity-methodology-followup-source-query:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            priority_rank: 4,
            run_status: "pending_not_run".to_string(),
            planned_query_text:
                "site:cms.gov Part D IPM recoverable collectible overpayment FY2024 PDE audit closeout recovery estimate"
                    .to_string(),
            result_capture_status: "no_result_captured".to_string(),
            required_capture_fields: vec![
                "canonical URL".to_string(),
                "observed date".to_string(),
                "recoverable amount basis".to_string(),
            ],
            next_run_rule:
                "Run against official CMS, HHS, OMB, and PaymentAccuracy sources only."
                    .to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up source-query-run row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_source_capture_boundary() {
        let record = PaymentIntegrityMethodologyFollowupSourceCaptureRecord {
            record_id:
                "payment-integrity-methodology-followup-source-capture:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_source_capture".to_string(),
            source_followup_source_query_run_record_id:
                "payment-integrity-methodology-followup-source-query-run:cms-part-d:overpayment-versus-recoverable-amount-basis:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicare Prescription Drug Benefit (Part D)".to_string(),
            priority_rank: 4,
            observed_date: "2026-07-01".to_string(),
            source_url: "https://paymentaccuracy.gov/".to_string(),
            source_title: "Payment Integrity Scorecard".to_string(),
            captured_source_scope: "Q4 2025 scorecard recovery text".to_string(),
            captured_boundary_summary:
                "The source supports audit-specific recovery process language only.".to_string(),
            recoverability_boundary_status:
                "partial_recovery_process_support_review_needed".to_string(),
            closure_effect: "Keep field open pending reviewer assessment.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up source-capture row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_source_capture_rollup_boundary() {
        let record = PaymentIntegrityMethodologyFollowupSourceCaptureRollupRecord {
            record_id:
                "payment-integrity-methodology-followup-source-capture-rollup:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_source_capture_rollup"
                .to_string(),
            source_followup_source_capture_record_id:
                "payment-integrity-methodology-followup-source-capture:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicaid".to_string(),
            priority_rank: 3,
            capture_rollup_status: "additional_positive_basis_needed".to_string(),
            boundary_finding:
                "Source blocks direct translation of PERM improper-payment dollars to savings."
                    .to_string(),
            remaining_review_need:
                "Find positive recoverable amount basis before scoring.".to_string(),
            reviewer_action:
                "Keep field open and queue additional source work if scoring is needed."
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up source-capture rollup row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_boundary_decision_boundary() {
        let record = PaymentIntegrityMethodologyFollowupBoundaryDecisionRecord {
            record_id:
                "payment-integrity-methodology-followup-boundary-decision:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_boundary_decision".to_string(),
            source_followup_source_capture_rollup_record_id:
                "payment-integrity-methodology-followup-source-capture-rollup:cms-medicaid:improper-payment-versus-fraud-waste-basis:q4-2025"
                    .to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicaid".to_string(),
            priority_rank: 3,
            boundary_decision_status: "claim_guard_confirmed_internal_only".to_string(),
            boundary_decision:
                "Treat the source as a claim guard against scoring PERM dollars as savings."
                    .to_string(),
            scoring_implication: "No scoring allowed from this capture.".to_string(),
            next_required_action: "Find positive recoverable basis before scoring.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up boundary-decision row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_followup_boundary_readiness_boundary() {
        let record = PaymentIntegrityMethodologyFollowupBoundaryReadinessRecord {
            record_id:
                "payment-integrity-methodology-followup-boundary-readiness:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_followup_boundary_readiness".to_string(),
            source_followup_boundary_decision_record_id:
                "payment-integrity-methodology-followup-boundary-decision:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            agency_code: "USDA".to_string(),
            program_or_activity: "Federal Crop Insurance Program".to_string(),
            priority_rank: 1,
            boundary_readiness_status: "narrow_internal_readiness_candidate".to_string(),
            readiness_scope: "Root-cause field framing only.".to_string(),
            readiness_reason:
                "Boundary decision supports field framing but not collectible savings.".to_string(),
            next_required_action:
                "Create a narrow closure-readiness candidate only for field framing.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology follow-up boundary-readiness row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_narrow_closure_candidate_boundary() {
        let record = PaymentIntegrityMethodologyNarrowClosureCandidateRecord {
            record_id:
                "payment-integrity-methodology-narrow-closure-candidate:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_narrow_closure_candidate".to_string(),
            source_followup_boundary_readiness_record_id:
                "payment-integrity-methodology-followup-boundary-readiness:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            agency_code: "USDA".to_string(),
            program_or_activity: "Federal Crop Insurance Program".to_string(),
            priority_rank: 1,
            candidate_scope: "Root-cause field framing only.".to_string(),
            candidate_basis: "Boundary-readiness row supports internal framing.".to_string(),
            excluded_scoring_basis:
                "Does not support collectible savings or recoverable-dollar scoring.".to_string(),
            next_required_action:
                "Reviewer may create a narrow internal closure decision for field framing only."
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology narrow closure-candidate row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_narrow_closure_decision_boundary() {
        let record = PaymentIntegrityMethodologyNarrowClosureDecisionRecord {
            record_id:
                "payment-integrity-methodology-narrow-closure-decision:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_narrow_closure_decision".to_string(),
            source_narrow_closure_candidate_record_id:
                "payment-integrity-methodology-narrow-closure-candidate:usda-federal-crop-insurance:data-access-outside-agency-control-root-cause-definition:q4-2025"
                    .to_string(),
            agency_code: "USDA".to_string(),
            program_or_activity: "Federal Crop Insurance Program".to_string(),
            priority_rank: 1,
            narrow_decision_status: "component_closed_internal_only".to_string(),
            closed_component: "Root-cause field framing only.".to_string(),
            decision_basis: "Source supports current root-cause wording.".to_string(),
            excluded_scope: "Recoverable savings basis remains open.".to_string(),
            residual_open_need: "Collectible amount basis.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology narrow closure-decision row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_open_program_component_progress_boundary() {
        let record = PaymentIntegrityMethodologyOpenProgramComponentProgressRecord {
            record_id:
                "payment-integrity-methodology-open-program-component-progress:va-pltss:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_open_program_component_progress"
                .to_string(),
            source_open_program_status_record_id:
                "payment-integrity-methodology-open-program-status:va-pltss:q4-2025"
                    .to_string(),
            source_narrow_closure_decision_record_id:
                "payment-integrity-methodology-narrow-closure-decision:va-pltss:documentation-defect-versus-recoverable-overpayment-basis:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            component_progress_status: "narrow_component_recorded_no_field_closure".to_string(),
            total_methodology_fields: 8,
            closed_field_count_after_component_decision: 0,
            open_field_count_after_component_decision: 8,
            narrow_component_decision_count: 1,
            component_progress_summary:
                "Record a narrow reporting-split component without changing field counts."
                    .to_string(),
            unchanged_field_count_reason:
                "Recoverable amount basis and estimator support remain outside scope.".to_string(),
            next_gate_condition:
                "Create a full field-closure decision before any scoring gate.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology open-program component-progress row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_requirement_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateRequirementRecord {
            record_id:
                "payment-integrity-methodology-component-gate-requirement:va-pltss:recoverable-incorrect-amount-subset:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_requirement".to_string(),
            source_component_progress_record_id:
                "payment-integrity-methodology-open-program-component-progress:va-pltss:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            gate_status: "positive_evidence_required_before_field_closure".to_string(),
            required_positive_evidence:
                "Recoverable incorrect-amount subset must be supported by source text."
                    .to_string(),
            blocked_translation:
                "Do not translate overpayment reporting into a waste-reduction score.".to_string(),
            next_source_target: "VA PLTSS bills-of-collection or AFR material.".to_string(),
            next_decision_type: "full_field_closure_review".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate-requirement row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_source_target_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateSourceTargetRecord {
            record_id:
                "payment-integrity-methodology-component-gate-source-target:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_source_target"
                .to_string(),
            source_component_gate_requirement_record_id:
                "payment-integrity-methodology-component-gate-requirement:va-pltss:recoverable-incorrect-amount-subset:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            source_target_name: "VA bills-of-collection materials".to_string(),
            source_target_scope:
                "Find recoverable incorrect-amount overpayment treatment.".to_string(),
            evidence_to_extract: vec![
                "source URL".to_string(),
                "bill-of-collection basis".to_string(),
            ],
            negative_evidence_rule:
                "Keep scoring blocked if recoverable incorrect-amount subset is absent."
                    .to_string(),
            next_artifact_family:
                "payment_integrity_methodology_component_gate_source_query".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate source-target row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_source_query_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateSourceQueryRecord {
            record_id:
                "payment-integrity-methodology-component-gate-source-query:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_source_query".to_string(),
            source_component_gate_source_target_record_id:
                "payment-integrity-methodology-component-gate-source-target:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            query_text: "site:department.va.gov PLTSS bills of collection overpayment"
                .to_string(),
            query_scope: "Find recoverable incorrect-amount overpayment support.".to_string(),
            expected_evidence: vec![
                "official URL".to_string(),
                "bill-of-collection basis".to_string(),
            ],
            insufficient_result_rule:
                "Keep scoring blocked if recoverable subset support is absent.".to_string(),
            next_artifact_family:
                "payment_integrity_methodology_component_gate_source_query_run".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate source-query row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_source_query_run_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateSourceQueryRunRecord {
            record_id:
                "payment-integrity-methodology-component-gate-source-query-run:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_source_query_run"
                .to_string(),
            source_component_gate_source_query_record_id:
                "payment-integrity-methodology-component-gate-source-query:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            run_status: "pending_not_run".to_string(),
            planned_query_text: "site:department.va.gov PLTSS bills of collection overpayment"
                .to_string(),
            result_capture_status: "no_result_captured".to_string(),
            required_capture_fields: vec![
                "official URL".to_string(),
                "bill-of-collection basis".to_string(),
            ],
            next_run_rule: "Create a source capture only after official evidence is found."
                .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate source-query-run row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_source_capture_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateSourceCaptureRecord {
            record_id:
                "payment-integrity-methodology-component-gate-source-capture:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_source_capture"
                .to_string(),
            source_component_gate_source_query_run_record_id:
                "payment-integrity-methodology-component-gate-source-query-run:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            observed_date: "2026-07-02".to_string(),
            source_url:
                "https://department.va.gov/wp-content/uploads/2026/01/2025-Section-III-Other-Information.pdf"
                    .to_string(),
            source_title: "VA FY 2025 Agency Financial Report, Section III".to_string(),
            captured_source_scope: "PLTSS corrective-action text".to_string(),
            captured_gate_summary:
                "Source supports bills-of-collection process but not a quantified subset."
                    .to_string(),
            component_gate_status: "partial_positive_basis_review_needed".to_string(),
            next_review_action: "Keep scoring blocked pending recoverable-dollar support."
                .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate source-capture row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_source_capture_rollup_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateSourceCaptureRollupRecord {
            record_id:
                "payment-integrity-methodology-component-gate-source-capture-rollup:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_source_capture_rollup"
                .to_string(),
            source_component_gate_source_capture_record_id:
                "payment-integrity-methodology-component-gate-source-capture:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            capture_rollup_status: "reviewer_gate_decision_needed".to_string(),
            gate_finding:
                "Process support exists but quantified recoverable subset remains missing."
                    .to_string(),
            remaining_review_need:
                "Reviewer must decide whether process support is sufficient for internal gate handling."
                    .to_string(),
            reviewer_action: "Create a boundary decision; keep scoring blocked.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate source-capture rollup row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_boundary_decision_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateBoundaryDecisionRecord {
            record_id:
                "payment-integrity-methodology-component-gate-boundary-decision:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_boundary_decision"
                .to_string(),
            source_component_gate_source_capture_rollup_record_id:
                "payment-integrity-methodology-component-gate-source-capture-rollup:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            boundary_decision_status: "narrow_process_boundary_supported_internal_only"
                .to_string(),
            boundary_decision:
                "Support only the bills-of-collection process boundary, not scoring."
                    .to_string(),
            scoring_implication: "No scoring is allowed without quantified subset support."
                .to_string(),
            next_required_action: "Prepare readiness only for narrow process boundary."
                .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate boundary-decision row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_boundary_readiness_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateBoundaryReadinessRecord {
            record_id:
                "payment-integrity-methodology-component-gate-boundary-readiness:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_boundary_readiness"
                .to_string(),
            source_component_gate_boundary_decision_record_id:
                "payment-integrity-methodology-component-gate-boundary-decision:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            boundary_readiness_status: "narrow_internal_readiness_candidate".to_string(),
            readiness_scope: "Bills-of-collection process boundary only.".to_string(),
            readiness_reason:
                "Decision supports process boundary but not recoverable-dollar scoring."
                    .to_string(),
            next_required_action: "Prepare narrow component candidate only.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate boundary-readiness row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_narrow_candidate_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateNarrowCandidateRecord {
            record_id:
                "payment-integrity-methodology-component-gate-narrow-candidate:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_narrow_candidate"
                .to_string(),
            source_component_gate_boundary_readiness_record_id:
                "payment-integrity-methodology-component-gate-boundary-readiness:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            candidate_status: "narrow_component_candidate_internal_only".to_string(),
            candidate_scope: "Bills-of-collection process boundary only.".to_string(),
            candidate_basis:
                "Boundary readiness supports process boundary but not recoverable-dollar scoring."
                    .to_string(),
            excluded_scoring_basis:
                "Does not quantify a recoverable subset or support savings estimates."
                    .to_string(),
            next_required_action: "Prepare narrow component decision only.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate narrow candidate only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_narrow_decision_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateNarrowDecisionRecord {
            record_id:
                "payment-integrity-methodology-component-gate-narrow-decision:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_narrow_decision"
                .to_string(),
            source_component_gate_narrow_candidate_record_id:
                "payment-integrity-methodology-component-gate-narrow-candidate:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            narrow_decision_status: "component_closed_internal_only".to_string(),
            closed_component: "Bills-of-collection process boundary only.".to_string(),
            decision_basis:
                "Candidate supports process boundary but not recoverable-dollar scoring."
                    .to_string(),
            excluded_scope:
                "Does not close recoverable subset, collectible amount, or scoring gate."
                    .to_string(),
            residual_open_need:
                "Recoverable incorrect-amount subset remains open before any waste-reduction score."
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate narrow decision only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_progress".to_string(),
            source_open_program_status_record_id:
                "payment-integrity-methodology-open-program-status:va-pltss:q4-2025"
                    .to_string(),
            source_component_gate_narrow_decision_record_id:
                "payment-integrity-methodology-component-gate-narrow-decision:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            component_progress_status: "component_gate_progress_recorded_no_field_closure"
                .to_string(),
            total_methodology_fields: 8,
            closed_field_count_after_component_decision: 0,
            open_field_count_after_component_decision: 8,
            component_gate_decision_count: 1,
            component_progress_summary:
                "Bills-of-collection process boundary recorded as internal component progress."
                    .to_string(),
            unchanged_field_count_reason:
                "No recoverable-dollar basis or field closure was established.".to_string(),
            next_gate_condition:
                "Resolve recoverable incorrect-amount subset before scoring.".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_requirement_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressRequirementRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress-requirement:va-pltss:recoverable-incorrect-amount-subset:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_progress_requirement"
                .to_string(),
            source_component_gate_progress_record_id:
                "payment-integrity-methodology-component-gate-progress:va-pltss:bills-of-collection:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            gate_status: "positive_evidence_required_before_field_closure".to_string(),
            required_positive_evidence:
                "Source text must quantify recoverable incorrect-amount PLTSS overpayments."
                    .to_string(),
            blocked_translation:
                "Do not translate bills-of-collection process progress into savings."
                    .to_string(),
            next_source_target: "VA PLTSS bills-of-collection recoverability material."
                .to_string(),
            next_decision_type: "full_field_closure_review".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress requirement row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_source_target_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressSourceTargetRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress-source-target:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_progress_source_target"
                .to_string(),
            source_component_gate_progress_requirement_record_id:
                "payment-integrity-methodology-component-gate-progress-requirement:va-pltss:recoverable-incorrect-amount-subset:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            source_target_name: "VA PLTSS bills-of-collection debt materials".to_string(),
            source_target_scope:
                "Find collectible-dollar basis for incorrect-amount PLTSS reviews.".to_string(),
            evidence_to_extract: vec![
                "source URL".to_string(),
                "recoverable overpayment amount".to_string(),
            ],
            negative_evidence_rule:
                "If the source only describes process without dollars, keep scoring blocked."
                    .to_string(),
            next_artifact_family:
                "payment_integrity_methodology_component_gate_progress_source_query".to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress source-target row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_source_query_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressSourceQueryRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress-source-query:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_progress_source_query"
                .to_string(),
            source_component_gate_progress_source_target_record_id:
                "payment-integrity-methodology-component-gate-progress-source-target:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            query_text:
                "site:department.va.gov OR site:va.gov PLTSS bills of collection recoverable overpayment debt"
                    .to_string(),
            query_scope: "Search official VA materials for collectible PLTSS overpayment dollars."
                .to_string(),
            expected_evidence: vec![
                "official URL".to_string(),
                "recoverable overpayment amount".to_string(),
            ],
            insufficient_result_rule:
                "If results only describe process without dollars, keep scoring blocked."
                    .to_string(),
            next_artifact_family:
                "payment_integrity_methodology_component_gate_progress_source_query_run"
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress source-query row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_source_query_run_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressSourceQueryRunRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress-source-query-run:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            record_family: "payment_integrity_methodology_component_gate_progress_source_query_run"
                .to_string(),
            source_component_gate_progress_source_query_record_id:
                "payment-integrity-methodology-component-gate-progress-source-query:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            run_status: "pending_not_run".to_string(),
            planned_query_text:
                "site:department.va.gov OR site:va.gov PLTSS bills of collection recoverable overpayment debt"
                    .to_string(),
            result_capture_status: "no_result_captured".to_string(),
            required_capture_fields: vec![
                "official URL".to_string(),
                "recoverable overpayment amount".to_string(),
            ],
            next_run_rule:
                "Run against official VA sources; create a capture only if collectible-dollar evidence appears."
                    .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress source-query-run row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_methodology_component_gate_progress_source_capture_boundary() {
        let record = PaymentIntegrityMethodologyComponentGateProgressSourceCaptureRecord {
            record_id:
                "payment-integrity-methodology-component-gate-progress-source-capture:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            record_family:
                "payment_integrity_methodology_component_gate_progress_source_capture"
                    .to_string(),
            source_component_gate_progress_source_query_run_record_id:
                "payment-integrity-methodology-component-gate-progress-source-query-run:va-pltss:bills-of-collection-debt:q4-2025"
                    .to_string(),
            agency_code: "VA".to_string(),
            program_or_activity: "Purchased Long Term Services and Supports (PLTSS)"
                .to_string(),
            source_target_priority: 1,
            observed_date: "2026-07-02".to_string(),
            source_url: "https://department.va.gov/wp-content/uploads/2026/01/2025-Section-III-Other-Information.pdf"
                .to_string(),
            source_title: "Department of Veterans Affairs FY 2025 Agency Financial Report, Section III Other Information"
                .to_string(),
            captured_source_scope: "PLTSS post-payment review bills-of-collection process."
                .to_string(),
            captured_gate_summary:
                "Supports process context but not quantified recoverable dollars.".to_string(),
            component_gate_status: "partial_positive_basis_review_needed".to_string(),
            next_review_action: "Keep scoring blocked until recoverable dollars are sourced."
                .to_string(),
            field_closure_allowed: false,
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a methodology component gate progress source-capture row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_next_program_selection_boundary() {
        let record = PaymentIntegrityNextProgramSelectionRecord {
            record_id: "payment-integrity-next-program-selection:cms-medicaid:q4-2025".to_string(),
            record_family: "payment_integrity_next_program_selection".to_string(),
            selected_program_key: "cms-medicaid".to_string(),
            agency_code: "HHS".to_string(),
            program_or_activity: "Medicaid".to_string(),
            selection_status: "selected_for_methodology_planning".to_string(),
            selection_reason:
                "Medicaid is the next high-outlay payment-integrity branch after Part D."
                    .to_string(),
            official_source_urls: vec![
                "https://paymentaccuracy.gov/assets/scorecards/Q4%202025/Centers%20for%20Medicare%20%26%20Medicaid%20Services%20%28CMS%29%20-%20Medicaid.pdf"
                    .to_string(),
                "https://www.cms.gov/data-research/monitoring-programs/improper-payment-measurement-programs"
                    .to_string(),
            ],
            starting_methodology_fields: vec![
                "sample design".to_string(),
                "payment universe".to_string(),
                "estimation method".to_string(),
            ],
            next_artifact_family: "payment_integrity_methodology_plan".to_string(),
            scoring_allowed: false,
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a next-program selection row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert!(record.validate().is_ok());
    }

    #[test]
    fn validates_payment_integrity_claims_timeliness_probe_boundary() {
        let record = PaymentIntegrityClaimsTimelinessProbeRecord {
            record_id:
                "payment-integrity-claims-timeliness-probe:ssa:initial-disability-delta:2026-06-30"
                    .to_string(),
            record_family: "payment_integrity_claims_timeliness_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:claims-timeliness:v1".to_string(),
            source_id: "SRC-SSA-PERFORMANCE".to_string(),
            observed_date: "2026-06-30".to_string(),
            page_url: "https://www.ssa.gov/ssa-performance".to_string(),
            agency_code: "SSA".to_string(),
            metric_name: "Initial disability processing time improvement versus May 2025"
                .to_string(),
            metric_value: 42.0,
            metric_unit: "days".to_string(),
            comparison_operator: "improvement".to_string(),
            metric_period: "current page observed 2026-06-30".to_string(),
            source_scope_note:
                "Public performance-page probe only; not a query-locked monthly CSV extract."
                    .to_string(),
            next_extract_need:
                "Retrieve monthly processing-time series and pair with accuracy, appeal, and access metrics."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a claims-timeliness probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_debt_maturity_risk_treasury_probe_boundary() {
        let record = DebtMaturityRiskTreasuryProbeRecord {
            record_id: "debt-maturity-risk-treasury-probe:debt-to-penny:2026-06-29"
                .to_string(),
            record_family: "debt_maturity_risk_treasury_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:debt-interest:maturity-risk:v1".to_string(),
            source_id: "SRC-TREASURY-DEBT-PENNY".to_string(),
            query_date: "2026-06-30".to_string(),
            api_url: "https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v2/accounting/od/debt_to_penny?sort=-record_date&page[size]=5".to_string(),
            record_date: "2026-06-29".to_string(),
            row_kind: "debt_stock".to_string(),
            security_type: "all".to_string(),
            security_description: "Total public debt outstanding".to_string(),
            debt_held_public_amount: Some(31_621_329_805_348.19),
            intragovernmental_holdings_amount: Some(7_724_010_982_621.53),
            total_public_debt_outstanding_amount: Some(39_345_340_787_969.72),
            average_interest_rate_percent: None,
            source_scope_note: "Daily Treasury debt-stock row; not a maturity distribution or debt-management scenario.".to_string(),
            next_extract_need: "Pair with maturity distribution, refinancing exposure, and CBO interest-rate assumptions.".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a Treasury rate-risk probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_debt_primary_balance_fiscal_probe_boundary() {
        let record = DebtPrimaryBalanceFiscalProbeRecord {
            record_id: "debt-primary-balance-fiscal-probe:fy2025".to_string(),
            record_family: "debt_primary_balance_fiscal_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:debt-interest:primary-balance:v1".to_string(),
            fiscal_year: 2025,
            source_ids: vec![
                "SRC-OMB-HIST-1-1-FY2027".to_string(),
                "SRC-OMB-HIST-3-2-FY2027".to_string(),
            ],
            total_receipts_millions: 5_236_421.0,
            total_outlays_millions: 7_011_105.0,
            deficit_gap_millions: 1_774_684.0,
            gross_treasury_interest_outlays_millions: 1_215_611.0,
            primary_deficit_proxy_millions: 559_073.0,
            borrowed_share_percent_of_outlays: 25.312472142,
            income_tax_coverage_percent_of_outlays: 37.883386428,
            basis_note:
                "Primary-deficit proxy subtracts OMB Table 3.2 gross Treasury-interest subfunction outlays from the FY2025 deficit gap."
                    .to_string(),
            next_extract_need:
                "Replace proxy with CBO/OMB primary-deficit series and policy scenario assumptions."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as fiscal-balance context only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_disaster_declaration_probe_boundary() {
        let record = DisasterDeclarationProbeRecord {
            record_id: "disaster-declaration-probe:fema:5642:co:custer:2026-06-30"
                .to_string(),
            record_family: "disaster_declaration_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:disaster-exposure:supplemental-tracking:v1"
                    .to_string(),
            source_id: "SRC-FEMA-DISASTER-DECLARATIONS".to_string(),
            query_date: "2026-06-30".to_string(),
            api_url: "https://www.fema.gov/api/open/v2/DisasterDeclarationsSummaries".to_string(),
            disaster_number: 5642,
            declaration_date: "2026-06-29T00:00:00Z".to_string(),
            incident_type: "Fire".to_string(),
            state: "CO".to_string(),
            designated_area: "Custer (County)".to_string(),
            declaration_title: "ASPEN ACRES FIRE".to_string(),
            ih_program_declared: false,
            ia_program_declared: false,
            pa_program_declared: true,
            hm_program_declared: false,
            source_scope_note: "Declaration-area row only; not an outlay, damage, or waste estimate.".to_string(),
            next_extract_need: "Link declaration to account, obligation, award, and outlay records.".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a disaster declaration probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_disaster_mitigation_project_probe_boundary() {
        let record = DisasterMitigationProjectProbeRecord {
            record_id: "disaster-mitigation-project-probe:fema-hma:dr-4781-0063:2026-06-30"
                .to_string(),
            record_family: "disaster_mitigation_project_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:disaster-exposure:mitigation:v1".to_string(),
            source_id: "SRC-FEMA-HMA-PROJECTS".to_string(),
            query_date: "2026-06-30".to_string(),
            api_url: "https://www.fema.gov/api/open/v4/HazardMitigationAssistanceProjects"
                .to_string(),
            project_identifier: "DR-4781-0063".to_string(),
            program_area: "HMGP".to_string(),
            program_fy: 2024,
            state: "Texas".to_string(),
            county: "Hardin".to_string(),
            disaster_number: Some(4781),
            project_type: "205.8: Retrofitting Public Structures - Wind".to_string(),
            status: "Pending".to_string(),
            recipient: "Statewide".to_string(),
            subrecipient: "Hardin (County)".to_string(),
            data_source: "HMGP".to_string(),
            date_approved: None,
            date_closed: None,
            project_amount: Some(3_823_400.0),
            federal_share_obligated: None,
            cost_share_percentage: Some(0.75),
            benefit_cost_ratio: Some(9.39),
            net_value_benefits: Some(35_890_154.0),
            number_of_properties: Some(0),
            source_scope_note:
                "FEMA HMA project row only; not an avoided-loss or savings estimate."
                    .to_string(),
            next_extract_need:
                "Attach benefit-cost method, hazard/geography crosswalk, and event-to-account bridge."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a disaster mitigation project probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_defense_audit_control_probe_boundary() {
        let record = DefenseAuditControlProbeRecord {
            record_id: "defense-audit-control-probe:dodig-fy2025:summary:2026-06-30"
                .to_string(),
            record_family: "defense_audit_control_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:defense:audit-control-closure:v1".to_string(),
            source_id: "SRC-DODIG-FY2025-AUDIT".to_string(),
            observed_date: "2026-06-30".to_string(),
            report_url: "https://media.defense.gov/2025/Dec/19/2003847587/-1/-1/1/DODIG-2026-032.PDF"
                .to_string(),
            report_number: "DODIG-2026-032".to_string(),
            fiscal_year: 2025,
            finding_type: "audit_result_summary".to_string(),
            finding_identifier: "agency-wide-fy2025".to_string(),
            finding_title: "FY2025 DoD agency-wide financial statement audit result"
                .to_string(),
            audit_opinion: Some("disclaimer_of_opinion".to_string()),
            material_weakness_count: Some(26),
            significant_deficiency_count: Some(2),
            noncompliance_count: Some(5),
            reported_amount_usd: Some(4_600_000_000_000.0),
            reported_amount_basis: Some("assets assessed by independent auditors".to_string()),
            affected_area: "agency-wide financial statements".to_string(),
            control_signal:
                "Audit report summarizes material weaknesses, significant deficiencies, and noncompliance instances."
                    .to_string(),
            recommendation_signal:
                "Use as inventory for closure tracking and corrective-action extraction."
                    .to_string(),
            source_scope_note:
                "Audit-control context only; not a waste, fraud, readiness, or savings finding."
                    .to_string(),
            next_extract_need:
                "Attach finding-level corrective-action status, repeat-finding age, and mission floor."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a defense audit control probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_defense_procurement_control_probe_boundary() {
        let record = DefenseProcurementControlProbeRecord {
            record_id:
                "defense-procurement-control-probe:gao-2025:mdap-cost-growth:2026-06-30"
                    .to_string(),
            record_family: "defense_procurement_control_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:defense:procurement-control:v1".to_string(),
            source_id: "SRC-GAO-WEAPON-SYSTEMS-2025".to_string(),
            observed_date: "2026-06-30".to_string(),
            report_url: "https://www.gao.gov/products/gao-25-107569".to_string(),
            report_number: "GAO-25-107569".to_string(),
            report_year: 2025,
            program_or_portfolio: "Major Defense Acquisition Programs".to_string(),
            service_or_scope: "DOD portfolio".to_string(),
            acquisition_pathway: Some("MDAP".to_string()),
            signal_type: "portfolio_cost_growth".to_string(),
            signal_title: "MDAP portfolio cost growth".to_string(),
            reported_amount_usd: Some(49_300_000_000.0),
            reported_amount_basis: Some("GAO-reported 2024 portfolio cost growth".to_string()),
            reported_percent: None,
            reported_months: None,
            reviewed_program_count: None,
            control_signal:
                "GAO reports portfolio cost growth that needs program-level baseline review."
                    .to_string(),
            recommendation_signal:
                "Extract program-level baseline, current estimate, schedule, and readiness floor."
                    .to_string(),
            source_scope_note:
                "Procurement-control context only; not a readiness, waste, or savings finding."
                    .to_string(),
            next_extract_need:
                "Attach acquisition baseline, current estimate, schedule variance, and strategy floor."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a defense procurement control probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_health_price_discipline_probe_boundary() {
        let record = HealthPriceDisciplineProbeRecord {
            record_id: "health-price-discipline-probe:medicare-part-b:gov-support-per-enrollee:cy2025"
                .to_string(),
            record_family: "health_price_discipline_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:health-medicare:price-discipline:v1".to_string(),
            source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            observed_date: "2026-06-30".to_string(),
            program_part: "Part B".to_string(),
            service_or_drug_category: "government contribution per Part B enrollee".to_string(),
            fiscal_or_calendar_year: "CY2025".to_string(),
            price_or_expenditure_basis: "Trustees Part B government contribution divided by Part B enrollment".to_string(),
            benchmark_or_comparison: "same-source per-enrollee anchor; not a service-price benchmark".to_string(),
            metric_value: Some(422_200_000_000.0),
            metric_unit: Some("usd".to_string()),
            denominator_value: Some(63_448_000.0),
            denominator_unit: Some("people".to_string()),
            computed_value_usd: Some(6_654.27),
            quality_or_access_measure: "not yet attached".to_string(),
            source_record_ids: vec![
                "medicare-part-financing:part-b:cy2025:cms-trustees-2026".to_string(),
                "denominator-value:medicare-part-b-enrollment:cy2025:cms-trustees-2026".to_string(),
            ],
            readiness_status: "anchor_ready_service_price_blocked".to_string(),
            source_scope_note:
                "Per-enrollee financing anchor only; not a provider price or savings estimate."
                    .to_string(),
            next_extract_need:
                "Attach service-level price/utilization data, case-mix controls, and quality floor."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a health price-discipline probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_health_admin_simplification_probe_boundary() {
        let record = HealthAdminSimplificationProbeRecord {
            record_id: "health-admin-simplification-probe:medicare:claims-workflow-gap:2026-06-30"
                .to_string(),
            record_family: "health_admin_simplification_probe".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:health-medicare:administrative-simplification:v1"
                    .to_string(),
            source_ids: vec!["SRC-CMS-MEDICARE-TRUSTEES-2026".to_string()],
            observed_date: "2026-06-30".to_string(),
            program_part: "Part A / Part B / Part D".to_string(),
            workflow_step: "claims workflow source inventory gap".to_string(),
            period: "not-yet-query-locked".to_string(),
            administrative_cost_or_cycle_time_basis:
                "blocked until CMS/HHS workflow volume and cost sources are selected".to_string(),
            claim_or_case_count: None,
            claim_or_case_count_unit: None,
            metric_value: None,
            metric_unit: None,
            access_or_integrity_floor:
                "blocked until access, due-process, payment-accuracy, and service-level floors are attached"
                    .to_string(),
            source_record_ids: vec![
                "cost-down-evidence-queue:health-medicare:administrative-simplification:v1"
                    .to_string(),
            ],
            readiness_status: "blocked_missing_workflow_extract".to_string(),
            source_scope_note:
                "Explicit blocker row; not an administrative savings estimate.".to_string(),
            next_extract_need:
                "Choose CMS/HHS claims, denial, appeal, authorization, and rework sources."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a health administrative simplification probe only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_cost_down_first_pass_rollup_boundary() {
        let record = CostDownFirstPassRollupRecord {
            record_id: "cost-down-first-pass-rollup:health-medicare:price-discipline:v1"
                .to_string(),
            record_family: "cost_down_first_pass_rollup".to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:health-medicare:price-discipline:v1".to_string(),
            source_backlog_record_id: "cost-down:health-medicare:price-discipline".to_string(),
            source_pressure_record_id: "efficiency-pressure:health-medicare-fy2025".to_string(),
            lane_id: "health-medicare".to_string(),
            lever_id: "price-discipline".to_string(),
            first_pass_artifacts: vec![
                "data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.jsonl"
                    .to_string(),
            ],
            first_pass_row_count: 6,
            signal_status: "context_ready_scoring_blocked".to_string(),
            strongest_current_signal:
                "OECD benchmark and Medicare per-enrollee anchors are ready; service-level prices are not."
                    .to_string(),
            scoring_blockers: vec![
                "service-level price/utilization extract".to_string(),
                "quality and access floor".to_string(),
                "reviewed scoring method".to_string(),
            ],
            next_scoring_step:
                "Choose CMS/HHS service categories and query-lock price/utilization fields."
                    .to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a first-pass rollup only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_cost_down_scoring_readiness_boundary() {
        let record = CostDownScoringReadinessRecord {
            record_id: "cost-down-scoring-readiness:payment-integrity:eligibility-accuracy:v1"
                .to_string(),
            record_family: "cost_down_scoring_readiness".to_string(),
            source_rollup_record_id:
                "cost-down-first-pass-rollup:payment-integrity:eligibility-accuracy:v1"
                    .to_string(),
            source_evidence_queue_record_id:
                "cost-down-evidence-queue:payment-integrity:eligibility-accuracy:v1"
                    .to_string(),
            lane_id: "payment-integrity-administration".to_string(),
            lever_id: "eligibility-accuracy".to_string(),
            prioritization_rank: 1,
            readiness_tier: "near_term_program_review".to_string(),
            evidence_maturity_score: 4,
            scale_pressure_score: 4,
            scoring_complexity_score: 3,
            priority_rationale:
                "Program scorecards expose rates, dollars, and root causes, but access floors remain missing."
                    .to_string(),
            immediate_next_artifact:
                "program-level methodology and corrective-action extract".to_string(),
            public_claim_allowed: false,
            savings_estimate_allowed: false,
            public_use_rule:
                "Use as a scoring-readiness row only; not a savings estimate and not a finding of waste."
                    .to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn validates_spend_category_map_record_boundary() {
        let record = SpendCategoryMapRecord {
            model_id: SPEND_CATEGORY_MAP_MODEL_ID.to_string(),
            record_id: "spendcat-fy2025-001".to_string(),
            fiscal_year: 2025,
            rank: 1,
            source_level: "omb_subfunction".to_string(),
            source_id: "SRC-OMB-HIST-3-2-FY2027".to_string(),
            function_code: "650".to_string(),
            function_label: "Social Security".to_string(),
            subfunction_code: "651".to_string(),
            subfunction_label: "Social security".to_string(),
            subfunction_outlays_millions: 1_580_673.0,
            share_of_total_outlays_percent: 22.545276387,
            modeled_income_tax_allocation_millions: 598_812.460748,
            allocation_method: "proportional_outlay_share".to_string(),
            legal_allocation_status: "modeled_not_legal_dedication".to_string(),
            funding_caveat: "OMB subfunction row; not taxpayer-dollar tracing.".to_string(),
            next_source_need: "SSA trustees and OMB mandatory-program tables.".to_string(),
            accountability_status: "question_surface_only".to_string(),
        };

        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn blocks_spend_category_public_claim_bypass() {
        let record = SpendCategoryMapRecord {
            model_id: SPEND_CATEGORY_MAP_MODEL_ID.to_string(),
            record_id: "spendcat-fy2025-001".to_string(),
            fiscal_year: 2025,
            rank: 1,
            source_level: "omb_subfunction".to_string(),
            source_id: "SRC-OMB-HIST-3-2-FY2027".to_string(),
            function_code: "650".to_string(),
            function_label: "Social Security".to_string(),
            subfunction_code: "651".to_string(),
            subfunction_label: "Social security".to_string(),
            subfunction_outlays_millions: 1_580_673.0,
            share_of_total_outlays_percent: 22.545276387,
            modeled_income_tax_allocation_millions: 598_812.460748,
            allocation_method: "proportional_outlay_share".to_string(),
            legal_allocation_status: "modeled_not_legal_dedication".to_string(),
            funding_caveat: "OMB subfunction row; not taxpayer-dollar tracing.".to_string(),
            next_source_need: "SSA trustees and OMB mandatory-program tables.".to_string(),
            accountability_status: "public_claim_allowed".to_string(),
        };

        assert!(record.validate().is_err());
    }
}
