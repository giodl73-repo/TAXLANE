pub const SHARE_PPM: i128 = 1_000_000;
pub const MINUTES_PER_HOUR: i128 = 60;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissedExamStressInput {
    pub missed_exam_claims: i128,
    pub notice_reach_ppm: i128,
    pub accommodation_offer_ppm_of_reached: i128,
    pub reschedule_acceptance_ppm_of_offered: i128,
    pub exam_completion_ppm_of_accepted: i128,
    pub existing_evidence_sufficiency_ppm_of_reviewed: i128,
    pub followup_ppm_of_unresolved: i128,
    pub accommodation_minutes_per_offered_claim: i128,
    pub evidence_review_minutes_per_claim: i128,
    pub productive_hours_per_fte_year: i128,
    pub annual_compensation_usd_micros_per_fte: i128,
    pub annual_it_and_support_cost_usd_micros: i128,
    pub claimant_hours_per_completed_exam_micros: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissedExamStressOutput {
    pub notice_reached_claims_micros: i128,
    pub accommodation_offered_claims_micros: i128,
    pub reschedule_accepted_claims_micros: i128,
    pub completed_exam_claims_micros: i128,
    pub existing_evidence_review_claims_micros: i128,
    pub existing_evidence_sufficient_claims_micros: i128,
    pub unresolved_claims_micros: i128,
    pub procedurally_reviewable_claims_micros: i128,
    pub followup_claims_stress_micros: i128,
    pub accommodation_work_minutes_micros: i128,
    pub evidence_review_work_minutes_micros: i128,
    pub total_work_hours_micros: i128,
    pub required_fte_micros: i128,
    pub staff_cost_usd_micros: i128,
    pub total_modeled_cost_usd_micros: i128,
    pub claimant_burden_hours_micros: i128,
    pub modeled_cost_per_procedurally_reviewable_claim_usd_micros: Option<i128>,
}

fn validate_share(value: i128, field: &str) -> Result<(), String> {
    if !(0..=SHARE_PPM).contains(&value) {
        return Err(format!("{field} must be between zero and one million ppm"));
    }
    Ok(())
}

fn checked_fraction(value: i128, share_ppm: i128) -> Result<i128, String> {
    value
        .checked_mul(share_ppm)
        .ok_or("VET claim-share multiplication overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("VET claim-share division failed".to_string())
}

fn scale_units_to_micros(per_unit: i128, count_micros: i128) -> Result<i128, String> {
    per_unit
        .checked_mul(count_micros)
        .ok_or("VET workload scaling overflow".to_string())
}

fn scale_money(per_unit_usd_micros: i128, units_micros: i128) -> Result<i128, String> {
    per_unit_usd_micros
        .checked_mul(units_micros)
        .ok_or("VET money scaling overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("VET money scaling division failed".to_string())
}

fn validate(input: &MissedExamStressInput) -> Result<(), String> {
    let counts = [
        input.missed_exam_claims,
        input.accommodation_minutes_per_offered_claim,
        input.evidence_review_minutes_per_claim,
        input.productive_hours_per_fte_year,
        input.annual_compensation_usd_micros_per_fte,
        input.annual_it_and_support_cost_usd_micros,
        input.claimant_hours_per_completed_exam_micros,
    ];
    if counts.iter().any(|value| *value < 0) {
        return Err("VET missed-exam inputs cannot be negative".to_string());
    }
    if input.productive_hours_per_fte_year == 0 {
        return Err("VET productive hours must be positive".to_string());
    }
    validate_share(input.notice_reach_ppm, "notice reach")?;
    validate_share(
        input.accommodation_offer_ppm_of_reached,
        "accommodation offer",
    )?;
    validate_share(
        input.reschedule_acceptance_ppm_of_offered,
        "reschedule acceptance",
    )?;
    validate_share(input.exam_completion_ppm_of_accepted, "exam completion")?;
    validate_share(
        input.existing_evidence_sufficiency_ppm_of_reviewed,
        "existing evidence sufficiency",
    )?;
    validate_share(input.followup_ppm_of_unresolved, "unresolved follow-up")?;
    Ok(())
}

pub fn run_missed_exam_stress(
    input: &MissedExamStressInput,
) -> Result<MissedExamStressOutput, String> {
    validate(input)?;
    let cohort = input
        .missed_exam_claims
        .checked_mul(SHARE_PPM)
        .ok_or("VET cohort scaling overflow".to_string())?;
    let reached = checked_fraction(cohort, input.notice_reach_ppm)?;
    let offered = checked_fraction(reached, input.accommodation_offer_ppm_of_reached)?;
    let accepted = checked_fraction(offered, input.reschedule_acceptance_ppm_of_offered)?;
    let completed = checked_fraction(accepted, input.exam_completion_ppm_of_accepted)?;
    let reviewed = cohort
        .checked_sub(completed)
        .ok_or("VET reviewed-claim subtraction failed".to_string())?;
    let sufficient = checked_fraction(
        reviewed,
        input.existing_evidence_sufficiency_ppm_of_reviewed,
    )?;
    let unresolved = reviewed
        .checked_sub(sufficient)
        .ok_or("VET unresolved-claim subtraction failed".to_string())?;
    let procedurally_reviewable = completed
        .checked_add(sufficient)
        .ok_or("VET reviewable-claim addition overflow".to_string())?;
    let followup = checked_fraction(unresolved, input.followup_ppm_of_unresolved)?;

    let accommodation_minutes =
        scale_units_to_micros(input.accommodation_minutes_per_offered_claim, offered)?;
    let review_minutes = scale_units_to_micros(input.evidence_review_minutes_per_claim, reviewed)?;
    let total_minutes = accommodation_minutes
        .checked_add(review_minutes)
        .ok_or("VET total-work addition overflow".to_string())?;
    let total_hours = total_minutes / MINUTES_PER_HOUR;
    let productive_hours_micros = input
        .productive_hours_per_fte_year
        .checked_mul(SHARE_PPM)
        .ok_or("VET productive-hour scaling overflow".to_string())?;
    let required_fte = total_hours
        .checked_mul(SHARE_PPM)
        .ok_or("VET FTE numerator overflow".to_string())?
        .checked_div(productive_hours_micros)
        .ok_or("VET FTE division failed".to_string())?;
    let staff_cost = scale_money(input.annual_compensation_usd_micros_per_fte, required_fte)?;
    let total_cost = staff_cost
        .checked_add(input.annual_it_and_support_cost_usd_micros)
        .ok_or("VET total-cost addition overflow".to_string())?;
    let claimant_burden = input
        .claimant_hours_per_completed_exam_micros
        .checked_mul(completed)
        .ok_or("VET claimant-burden scaling overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("VET claimant-burden division failed".to_string())?;
    let unit_cost = if procedurally_reviewable == 0 {
        None
    } else {
        Some(
            total_cost
                .checked_mul(SHARE_PPM)
                .ok_or("VET unit-cost numerator overflow".to_string())?
                .checked_div(procedurally_reviewable)
                .ok_or("VET unit-cost division failed".to_string())?,
        )
    };
    Ok(MissedExamStressOutput {
        notice_reached_claims_micros: reached,
        accommodation_offered_claims_micros: offered,
        reschedule_accepted_claims_micros: accepted,
        completed_exam_claims_micros: completed,
        existing_evidence_review_claims_micros: reviewed,
        existing_evidence_sufficient_claims_micros: sufficient,
        unresolved_claims_micros: unresolved,
        procedurally_reviewable_claims_micros: procedurally_reviewable,
        followup_claims_stress_micros: followup,
        accommodation_work_minutes_micros: accommodation_minutes,
        evidence_review_work_minutes_micros: review_minutes,
        total_work_hours_micros: total_hours,
        required_fte_micros: required_fte,
        staff_cost_usd_micros: staff_cost,
        total_modeled_cost_usd_micros: total_cost,
        claimant_burden_hours_micros: claimant_burden,
        modeled_cost_per_procedurally_reviewable_claim_usd_micros: unit_cost,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized_case() -> MissedExamStressInput {
        MissedExamStressInput {
            missed_exam_claims: 1_000,
            notice_reach_ppm: 900_000,
            accommodation_offer_ppm_of_reached: 800_000,
            reschedule_acceptance_ppm_of_offered: 750_000,
            exam_completion_ppm_of_accepted: 800_000,
            existing_evidence_sufficiency_ppm_of_reviewed: 250_000,
            followup_ppm_of_unresolved: 500_000,
            accommodation_minutes_per_offered_claim: 30,
            evidence_review_minutes_per_claim: 90,
            productive_hours_per_fte_year: 1_500,
            annual_compensation_usd_micros_per_fte: 180_000_000_000,
            annual_it_and_support_cost_usd_micros: 100_000_000_000,
            claimant_hours_per_completed_exam_micros: 4_000_000,
        }
    }

    #[test]
    fn normalized_funnel_preserves_claims_and_workload() {
        let result = run_missed_exam_stress(&normalized_case()).unwrap();
        assert_eq!(result.notice_reached_claims_micros, 900_000_000);
        assert_eq!(result.accommodation_offered_claims_micros, 720_000_000);
        assert_eq!(result.reschedule_accepted_claims_micros, 540_000_000);
        assert_eq!(result.completed_exam_claims_micros, 432_000_000);
        assert_eq!(result.existing_evidence_review_claims_micros, 568_000_000);
        assert_eq!(
            result.existing_evidence_sufficient_claims_micros,
            142_000_000
        );
        assert_eq!(result.unresolved_claims_micros, 426_000_000);
        assert_eq!(result.procedurally_reviewable_claims_micros, 574_000_000);
    }

    #[test]
    fn workforce_and_cost_identity_replays() {
        let result = run_missed_exam_stress(&normalized_case()).unwrap();
        assert_eq!(result.total_work_hours_micros, 1_212_000_000);
        assert_eq!(result.required_fte_micros, 808_000);
        assert_eq!(result.staff_cost_usd_micros, 145_440_000_000);
        assert_eq!(result.total_modeled_cost_usd_micros, 245_440_000_000);
        assert_eq!(result.claimant_burden_hours_micros, 1_728_000_000);
    }

    #[test]
    fn no_notice_or_accommodation_still_requires_evidence_review() {
        let mut input = normalized_case();
        input.notice_reach_ppm = 0;
        let result = run_missed_exam_stress(&input).unwrap();
        assert_eq!(result.completed_exam_claims_micros, 0);
        assert_eq!(result.existing_evidence_review_claims_micros, 1_000_000_000);
        assert_eq!(result.procedurally_reviewable_claims_micros, 250_000_000);
    }

    #[test]
    fn reviewable_does_not_mean_granted_or_appeal_avoided() {
        let result = run_missed_exam_stress(&normalized_case()).unwrap();
        assert_eq!(result.followup_claims_stress_micros, 213_000_000);
        assert!(
            result.procedurally_reviewable_claims_micros > result.followup_claims_stress_micros
        );
    }

    #[test]
    fn rejects_invalid_shares_and_zero_productive_hours() {
        let mut input = normalized_case();
        input.notice_reach_ppm = SHARE_PPM + 1;
        assert!(run_missed_exam_stress(&input).is_err());
        input = normalized_case();
        input.productive_hours_per_fte_year = 0;
        assert!(run_missed_exam_stress(&input).is_err());
    }
}
