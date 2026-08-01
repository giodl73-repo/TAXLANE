pub const SHARE_PPM: i128 = 1_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MitigationInput {
    pub eligible_properties: i128,
    pub participation_ppm: i128,
    pub completion_ppm: i128,
    pub unit_intervention_cost_usd_micros: i128,
    pub federal_cost_share_ppm: i128,
    pub delivery_lag_years: u8,
    pub horizon_years: u8,
    pub annual_expected_nfip_claims_usd_micros_per_completed_property: i128,
    pub avoided_claims_ppm: i128,
    pub annual_premium_reduction_usd_micros_per_completed_property: i128,
    pub annual_other_federal_aid_reduction_usd_micros_per_completed_property: i128,
    pub annual_administration_cost_usd_micros_per_completed_property: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnualResult {
    pub year: u8,
    pub benefits_active: bool,
    pub federal_investment_usd_micros: i128,
    pub avoided_nfip_claims_usd_micros: i128,
    pub other_federal_aid_reduction_usd_micros: i128,
    pub premium_revenue_reduction_usd_micros: i128,
    pub administration_cost_usd_micros: i128,
    pub net_federal_effect_usd_micros: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MitigationOutput {
    pub participating_properties_micros: i128,
    pub completed_properties_micros: i128,
    pub active_benefit_years: u8,
    pub annual: Vec<AnnualResult>,
    pub federal_investment_usd_micros: i128,
    pub avoided_nfip_claims_usd_micros: i128,
    pub other_federal_aid_reduction_usd_micros: i128,
    pub premium_revenue_reduction_usd_micros: i128,
    pub administration_cost_usd_micros: i128,
    pub net_federal_effect_usd_micros: i128,
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
        .ok_or("DIS mitigation multiplication overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("DIS mitigation division failed".to_string())
}

fn scale_to_cohort(
    per_property_usd_micros: i128,
    completed_properties_micros: i128,
) -> Result<i128, String> {
    per_property_usd_micros
        .checked_mul(completed_properties_micros)
        .ok_or("DIS cohort multiplication overflow".to_string())?
        .checked_div(SHARE_PPM)
        .ok_or("DIS cohort division failed".to_string())
}

fn validate(input: &MitigationInput) -> Result<(), String> {
    if input.eligible_properties < 0 {
        return Err("DIS eligible properties cannot be negative".to_string());
    }
    if input.horizon_years == 0 || input.horizon_years > 50 {
        return Err("DIS horizon must be between 1 and 50 years".to_string());
    }
    if input.delivery_lag_years >= input.horizon_years {
        return Err("DIS delivery lag must leave at least one active benefit year".to_string());
    }
    validate_share(input.participation_ppm, "participation")?;
    validate_share(input.completion_ppm, "completion")?;
    validate_share(input.federal_cost_share_ppm, "federal cost share")?;
    validate_share(input.avoided_claims_ppm, "avoided claims")?;
    let monetary = [
        input.unit_intervention_cost_usd_micros,
        input.annual_expected_nfip_claims_usd_micros_per_completed_property,
        input.annual_premium_reduction_usd_micros_per_completed_property,
        input.annual_other_federal_aid_reduction_usd_micros_per_completed_property,
        input.annual_administration_cost_usd_micros_per_completed_property,
    ];
    if monetary.iter().any(|value| *value < 0) {
        return Err("DIS monetary inputs cannot be negative".to_string());
    }
    Ok(())
}

pub fn run_mitigation_stress(input: &MitigationInput) -> Result<MitigationOutput, String> {
    validate(input)?;
    let eligible_properties_micros = input
        .eligible_properties
        .checked_mul(SHARE_PPM)
        .ok_or("DIS eligible-property scaling overflow".to_string())?;
    let participating = checked_fraction(eligible_properties_micros, input.participation_ppm)?;
    let completed = checked_fraction(participating, input.completion_ppm)?;
    let cohort_cost = scale_to_cohort(input.unit_intervention_cost_usd_micros, completed)?;
    let investment = checked_fraction(cohort_cost, input.federal_cost_share_ppm)?;
    let expected_claims = scale_to_cohort(
        input.annual_expected_nfip_claims_usd_micros_per_completed_property,
        completed,
    )?;
    let annual_avoided_claims = checked_fraction(expected_claims, input.avoided_claims_ppm)?;
    let annual_aid = scale_to_cohort(
        input.annual_other_federal_aid_reduction_usd_micros_per_completed_property,
        completed,
    )?;
    let annual_premium = scale_to_cohort(
        input.annual_premium_reduction_usd_micros_per_completed_property,
        completed,
    )?;
    let annual_admin = scale_to_cohort(
        input.annual_administration_cost_usd_micros_per_completed_property,
        completed,
    )?;

    let mut annual = Vec::with_capacity(input.horizon_years.into());
    let mut totals = [0_i128; 6];
    for year in 1..=input.horizon_years {
        let active = year > input.delivery_lag_years;
        let row_investment = if year == 1 { investment } else { 0 };
        let (claims, aid, premium, admin) = if active {
            (
                annual_avoided_claims,
                annual_aid,
                annual_premium,
                annual_admin,
            )
        } else {
            (0, 0, 0, 0)
        };
        let net = claims
            .checked_add(aid)
            .and_then(|value| value.checked_sub(premium))
            .and_then(|value| value.checked_sub(admin))
            .and_then(|value| value.checked_sub(row_investment))
            .ok_or("DIS annual net-effect arithmetic overflow".to_string())?;
        for (total, value) in
            totals
                .iter_mut()
                .zip([row_investment, claims, aid, premium, admin, net])
        {
            *total = total
                .checked_add(value)
                .ok_or("DIS cumulative arithmetic overflow".to_string())?;
        }
        annual.push(AnnualResult {
            year,
            benefits_active: active,
            federal_investment_usd_micros: row_investment,
            avoided_nfip_claims_usd_micros: claims,
            other_federal_aid_reduction_usd_micros: aid,
            premium_revenue_reduction_usd_micros: premium,
            administration_cost_usd_micros: admin,
            net_federal_effect_usd_micros: net,
        });
    }
    Ok(MitigationOutput {
        participating_properties_micros: participating,
        completed_properties_micros: completed,
        active_benefit_years: input.horizon_years - input.delivery_lag_years,
        annual,
        federal_investment_usd_micros: totals[0],
        avoided_nfip_claims_usd_micros: totals[1],
        other_federal_aid_reduction_usd_micros: totals[2],
        premium_revenue_reduction_usd_micros: totals[3],
        administration_cost_usd_micros: totals[4],
        net_federal_effect_usd_micros: totals[5],
    })
}

pub fn break_even_annual_expected_claims_usd_micros_per_completed_property(
    input: &MitigationInput,
) -> Result<i128, String> {
    validate(input)?;
    if input.avoided_claims_ppm == 0 {
        return Err("DIS break-even claims require a positive avoided-claims share".to_string());
    }
    let active_years = i128::from(input.horizon_years - input.delivery_lag_years);
    let federal_cost = checked_fraction(
        input.unit_intervention_cost_usd_micros,
        input.federal_cost_share_ppm,
    )?;
    let annual_nonclaim_cost = input
        .annual_premium_reduction_usd_micros_per_completed_property
        .checked_add(input.annual_administration_cost_usd_micros_per_completed_property)
        .and_then(|value| {
            value.checked_sub(
                input.annual_other_federal_aid_reduction_usd_micros_per_completed_property,
            )
        })
        .ok_or("DIS break-even nonclaim arithmetic overflow".to_string())?;
    let required_avoided_claims = federal_cost
        .checked_add(
            annual_nonclaim_cost
                .checked_mul(active_years)
                .ok_or("DIS break-even year scaling overflow".to_string())?,
        )
        .ok_or("DIS break-even cost arithmetic overflow".to_string())?
        .max(0);
    let denominator = active_years
        .checked_mul(input.avoided_claims_ppm)
        .ok_or("DIS break-even denominator overflow".to_string())?;
    let numerator = required_avoided_claims
        .checked_mul(SHARE_PPM)
        .ok_or("DIS break-even numerator overflow".to_string())?;
    let rounded_numerator = numerator
        .checked_add(denominator - 1)
        .ok_or("DIS break-even rounding overflow".to_string())?;
    Ok(rounded_numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized_case() -> MitigationInput {
        MitigationInput {
            eligible_properties: 100,
            participation_ppm: 500_000,
            completion_ppm: 800_000,
            unit_intervention_cost_usd_micros: 100_000_000_000,
            federal_cost_share_ppm: 750_000,
            delivery_lag_years: 3,
            horizon_years: 10,
            annual_expected_nfip_claims_usd_micros_per_completed_property: 10_000_000_000,
            avoided_claims_ppm: 500_000,
            annual_premium_reduction_usd_micros_per_completed_property: 500_000_000,
            annual_other_federal_aid_reduction_usd_micros_per_completed_property: 1_000_000_000,
            annual_administration_cost_usd_micros_per_completed_property: 250_000_000,
        }
    }

    #[test]
    fn normalized_cohort_preserves_the_federal_identity() {
        let result = run_mitigation_stress(&normalized_case()).unwrap();
        assert_eq!(result.participating_properties_micros, 50_000_000);
        assert_eq!(result.completed_properties_micros, 40_000_000);
        assert_eq!(result.active_benefit_years, 7);
        assert_eq!(result.federal_investment_usd_micros, 3_000_000_000_000);
        assert_eq!(result.avoided_nfip_claims_usd_micros, 1_400_000_000_000);
        assert_eq!(
            result.other_federal_aid_reduction_usd_micros,
            280_000_000_000
        );
        assert_eq!(result.premium_revenue_reduction_usd_micros, 140_000_000_000);
        assert_eq!(result.administration_cost_usd_micros, 70_000_000_000);
        assert_eq!(result.net_federal_effect_usd_micros, -1_530_000_000_000);
    }

    #[test]
    fn delivery_lag_defers_benefits_but_not_the_modeled_investment() {
        let result = run_mitigation_stress(&normalized_case()).unwrap();
        assert_eq!(
            result.annual[0].federal_investment_usd_micros,
            3_000_000_000_000
        );
        assert!(!result.annual[2].benefits_active);
        assert_eq!(result.annual[2].avoided_nfip_claims_usd_micros, 0);
        assert!(result.annual[3].benefits_active);
        assert_eq!(
            result.annual[3].avoided_nfip_claims_usd_micros,
            200_000_000_000
        );
    }

    #[test]
    fn break_even_threshold_includes_revenue_aid_and_administration() {
        let threshold =
            break_even_annual_expected_claims_usd_micros_per_completed_property(&normalized_case())
                .unwrap();
        assert_eq!(threshold, 20_928_571_429);
        let mut case = normalized_case();
        case.annual_expected_nfip_claims_usd_micros_per_completed_property = threshold;
        assert!(
            run_mitigation_stress(&case)
                .unwrap()
                .net_federal_effect_usd_micros
                >= 0
        );
    }

    #[test]
    fn zero_participation_has_zero_federal_effect() {
        let mut case = normalized_case();
        case.participation_ppm = 0;
        let result = run_mitigation_stress(&case).unwrap();
        assert_eq!(result.completed_properties_micros, 0);
        assert_eq!(result.net_federal_effect_usd_micros, 0);
    }

    #[test]
    fn rejects_invalid_shares_lag_and_negative_money() {
        let mut case = normalized_case();
        case.completion_ppm = SHARE_PPM + 1;
        assert!(run_mitigation_stress(&case).is_err());
        case = normalized_case();
        case.delivery_lag_years = case.horizon_years;
        assert!(run_mitigation_stress(&case).is_err());
        case = normalized_case();
        case.unit_intervention_cost_usd_micros = -1;
        assert!(run_mitigation_stress(&case).is_err());
    }
}
