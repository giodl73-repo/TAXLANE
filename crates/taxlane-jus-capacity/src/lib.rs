pub const SHARE_PPM: i128 = 1_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapacityStressInput {
    pub existing_active_judges: i128,
    pub senior_and_magistrate_equivalent_judges_micros: i128,
    pub proposed_authorized_seats: i128,
    pub appointment_fill_ppm: i128,
    pub support_staff_readiness_ppm: i128,
    pub facility_security_technology_readiness_ppm: i128,
    pub productive_capacity_ppm: i128,
    pub weighted_filings: i128,
    pub annual_filings: i128,
    pub annual_terminations_without_candidate: i128,
    pub beginning_pending: i128,
    pub assumed_annual_terminations_per_fully_productive_new_judge: i128,
    pub annual_direct_compensation_usd_micros_per_appointed_judge: i128,
    pub annual_operating_cost_usd_micros_per_staffed_judge: i128,
    pub one_time_readiness_cost_usd_micros_per_authorized_seat: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapacityStressOutput {
    pub appointed_new_judges_micros: i128,
    pub staffed_new_judges_micros: i128,
    pub ready_new_judges_micros: i128,
    pub effective_new_judges_micros: i128,
    pub total_effective_judges_micros: i128,
    pub weighted_filings_per_effective_judge_micros: i128,
    pub additional_terminations_stress_micros: i128,
    pub candidate_terminations_stress_micros: i128,
    pub ending_pending_without_candidate_micros: i128,
    pub ending_pending_with_candidate_stress_micros: i128,
    pub direct_compensation_cost_usd_micros: i128,
    pub operating_cost_usd_micros: i128,
    pub one_time_readiness_cost_usd_micros: i128,
    pub total_modeled_cost_usd_micros: i128,
    pub modeled_cost_per_additional_termination_usd_micros: Option<i128>,
}

fn validate_share(value: i128, field: &str) -> Result<(), String> {
    if !(0..=SHARE_PPM).contains(&value) {
        return Err(format!("{field} must be between zero and one million ppm"));
    }
    Ok(())
}

fn checked_fraction(value: i128, numerator: i128) -> Result<i128, String> {
    value
        .checked_mul(numerator)
        .ok_or("JUS capacity multiplication overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("JUS capacity division failed".to_string())
}

fn scale_count(per_unit: i128, count_micros: i128) -> Result<i128, String> {
    per_unit
        .checked_mul(count_micros)
        .ok_or("JUS count scaling overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("JUS count scaling division failed".to_string())
}

fn scale_count_to_micros(per_unit: i128, count_micros: i128) -> Result<i128, String> {
    per_unit
        .checked_mul(count_micros)
        .ok_or("JUS count-micros scaling overflow".to_string())
}

fn validate(input: &CapacityStressInput) -> Result<(), String> {
    let counts = [
        input.existing_active_judges,
        input.senior_and_magistrate_equivalent_judges_micros,
        input.proposed_authorized_seats,
        input.weighted_filings,
        input.annual_filings,
        input.annual_terminations_without_candidate,
        input.beginning_pending,
        input.assumed_annual_terminations_per_fully_productive_new_judge,
        input.annual_direct_compensation_usd_micros_per_appointed_judge,
        input.annual_operating_cost_usd_micros_per_staffed_judge,
        input.one_time_readiness_cost_usd_micros_per_authorized_seat,
    ];
    if counts.iter().any(|value| *value < 0) {
        return Err("JUS capacity inputs cannot be negative".to_string());
    }
    validate_share(input.appointment_fill_ppm, "appointment fill")?;
    validate_share(input.support_staff_readiness_ppm, "support staff readiness")?;
    validate_share(
        input.facility_security_technology_readiness_ppm,
        "facility, security, and technology readiness",
    )?;
    validate_share(input.productive_capacity_ppm, "productive capacity")?;
    if input.existing_active_judges == 0
        && input.senior_and_magistrate_equivalent_judges_micros == 0
        && input.proposed_authorized_seats == 0
    {
        return Err("JUS stress requires some judicial capacity".to_string());
    }
    Ok(())
}

pub fn run_capacity_stress(input: &CapacityStressInput) -> Result<CapacityStressOutput, String> {
    validate(input)?;
    let proposed_micros = input
        .proposed_authorized_seats
        .checked_mul(SHARE_PPM)
        .ok_or("JUS proposed-seat scaling overflow".to_string())?;
    let appointed = checked_fraction(proposed_micros, input.appointment_fill_ppm)?;
    let staffed = checked_fraction(appointed, input.support_staff_readiness_ppm)?;
    let ready = checked_fraction(staffed, input.facility_security_technology_readiness_ppm)?;
    let effective_new = checked_fraction(ready, input.productive_capacity_ppm)?;
    let existing_micros = input
        .existing_active_judges
        .checked_mul(SHARE_PPM)
        .ok_or("JUS existing-seat scaling overflow".to_string())?;
    let total_effective = existing_micros
        .checked_add(input.senior_and_magistrate_equivalent_judges_micros)
        .and_then(|value| value.checked_add(effective_new))
        .ok_or("JUS effective-capacity addition overflow".to_string())?;
    let weighted_filings_per_effective_judge = input
        .weighted_filings
        .checked_mul(SHARE_PPM)
        .and_then(|value| value.checked_mul(SHARE_PPM))
        .ok_or("JUS weighted-filings scaling overflow".to_string())?
        .checked_div(total_effective)
        .ok_or("JUS weighted-filings division failed".to_string())?;
    let additional_terminations = scale_count_to_micros(
        input.assumed_annual_terminations_per_fully_productive_new_judge,
        effective_new,
    )?;
    let baseline_terminations_micros = input
        .annual_terminations_without_candidate
        .checked_mul(SHARE_PPM)
        .ok_or("JUS baseline termination scaling overflow".to_string())?;
    let candidate_terminations = baseline_terminations_micros
        .checked_add(additional_terminations)
        .ok_or("JUS candidate termination addition overflow".to_string())?;
    let available_matters_micros = input
        .beginning_pending
        .checked_add(input.annual_filings)
        .and_then(|value| value.checked_mul(SHARE_PPM))
        .ok_or("JUS available-matter scaling overflow".to_string())?;
    let ending_without = available_matters_micros
        .checked_sub(baseline_terminations_micros.min(available_matters_micros))
        .ok_or("JUS baseline pending subtraction failed".to_string())?;
    let ending_with = available_matters_micros
        .checked_sub(candidate_terminations.min(available_matters_micros))
        .ok_or("JUS candidate pending subtraction failed".to_string())?;

    let direct_cost = scale_count(
        input.annual_direct_compensation_usd_micros_per_appointed_judge,
        appointed,
    )?;
    let operating_cost = scale_count(
        input.annual_operating_cost_usd_micros_per_staffed_judge,
        staffed,
    )?;
    let readiness_cost = input
        .one_time_readiness_cost_usd_micros_per_authorized_seat
        .checked_mul(input.proposed_authorized_seats)
        .ok_or("JUS readiness-cost scaling overflow".to_string())?;
    let total_cost = direct_cost
        .checked_add(operating_cost)
        .and_then(|value| value.checked_add(readiness_cost))
        .ok_or("JUS total-cost addition overflow".to_string())?;
    let cost_per_termination = if additional_terminations == 0 {
        None
    } else {
        Some(
            total_cost
                .checked_mul(SHARE_PPM)
                .ok_or("JUS unit-cost scaling overflow".to_string())?
                .checked_div(additional_terminations)
                .ok_or("JUS unit-cost division failed".to_string())?,
        )
    };
    Ok(CapacityStressOutput {
        appointed_new_judges_micros: appointed,
        staffed_new_judges_micros: staffed,
        ready_new_judges_micros: ready,
        effective_new_judges_micros: effective_new,
        total_effective_judges_micros: total_effective,
        weighted_filings_per_effective_judge_micros: weighted_filings_per_effective_judge,
        additional_terminations_stress_micros: additional_terminations,
        candidate_terminations_stress_micros: candidate_terminations,
        ending_pending_without_candidate_micros: ending_without,
        ending_pending_with_candidate_stress_micros: ending_with,
        direct_compensation_cost_usd_micros: direct_cost,
        operating_cost_usd_micros: operating_cost,
        one_time_readiness_cost_usd_micros: readiness_cost,
        total_modeled_cost_usd_micros: total_cost,
        modeled_cost_per_additional_termination_usd_micros: cost_per_termination,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial_delivery() -> CapacityStressInput {
        CapacityStressInput {
            existing_active_judges: 100,
            senior_and_magistrate_equivalent_judges_micros: 0,
            proposed_authorized_seats: 10,
            appointment_fill_ppm: 800_000,
            support_staff_readiness_ppm: 900_000,
            facility_security_technology_readiness_ppm: 750_000,
            productive_capacity_ppm: 900_000,
            weighted_filings: 60_000,
            annual_filings: 60_000,
            annual_terminations_without_candidate: 58_000,
            beginning_pending: 20_000,
            assumed_annual_terminations_per_fully_productive_new_judge: 400,
            annual_direct_compensation_usd_micros_per_appointed_judge: 300_000_000_000,
            annual_operating_cost_usd_micros_per_staffed_judge: 700_000_000_000,
            one_time_readiness_cost_usd_micros_per_authorized_seat: 100_000_000_000,
        }
    }

    #[test]
    fn partial_delivery_preserves_every_capacity_gate() {
        let result = run_capacity_stress(&partial_delivery()).unwrap();
        assert_eq!(result.appointed_new_judges_micros, 8_000_000);
        assert_eq!(result.staffed_new_judges_micros, 7_200_000);
        assert_eq!(result.ready_new_judges_micros, 5_400_000);
        assert_eq!(result.effective_new_judges_micros, 4_860_000);
        assert_eq!(result.additional_terminations_stress_micros, 1_944_000_000);
        assert_eq!(
            result.ending_pending_without_candidate_micros,
            22_000_000_000
        );
        assert_eq!(
            result.ending_pending_with_candidate_stress_micros,
            20_056_000_000
        );
    }

    #[test]
    fn authorization_without_appointments_adds_no_operating_capacity() {
        let mut input = partial_delivery();
        input.appointment_fill_ppm = 0;
        let result = run_capacity_stress(&input).unwrap();
        assert_eq!(result.effective_new_judges_micros, 0);
        assert_eq!(result.additional_terminations_stress_micros, 0);
        assert_eq!(
            result.ending_pending_with_candidate_stress_micros,
            22_000_000_000
        );
        assert!(
            result
                .modeled_cost_per_additional_termination_usd_micros
                .is_none()
        );
    }

    #[test]
    fn full_delivery_is_a_mechanical_upper_stress_not_a_forecast() {
        let mut input = partial_delivery();
        input.appointment_fill_ppm = SHARE_PPM;
        input.support_staff_readiness_ppm = SHARE_PPM;
        input.facility_security_technology_readiness_ppm = SHARE_PPM;
        input.productive_capacity_ppm = SHARE_PPM;
        let result = run_capacity_stress(&input).unwrap();
        assert_eq!(result.effective_new_judges_micros, 10_000_000);
        assert_eq!(result.additional_terminations_stress_micros, 4_000_000_000);
        assert_eq!(
            result.ending_pending_with_candidate_stress_micros,
            18_000_000_000
        );
    }

    #[test]
    fn cost_identity_separates_appointment_staffing_and_readiness() {
        let result = run_capacity_stress(&partial_delivery()).unwrap();
        assert_eq!(
            result.direct_compensation_cost_usd_micros,
            2_400_000_000_000
        );
        assert_eq!(result.operating_cost_usd_micros, 5_040_000_000_000);
        assert_eq!(result.one_time_readiness_cost_usd_micros, 1_000_000_000_000);
        assert_eq!(result.total_modeled_cost_usd_micros, 8_440_000_000_000);
    }

    #[test]
    fn rejects_invalid_readiness_and_negative_inputs() {
        let mut input = partial_delivery();
        input.productive_capacity_ppm = SHARE_PPM + 1;
        assert!(run_capacity_stress(&input).is_err());
        input = partial_delivery();
        input.beginning_pending = -1;
        assert!(run_capacity_stress(&input).is_err());
    }
}
