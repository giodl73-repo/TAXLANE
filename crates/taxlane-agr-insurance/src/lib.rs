pub const SHARE_PPM: i128 = 1_000_000;
pub const BASIS_POINTS_PER_ONE: i128 = 10_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitivityInput {
    pub horizon_years: u8,
    pub annual_retained_premium_musd_micros: i128,
    pub baseline_return_bps: i128,
    pub target_return_bps: i128,
    pub participation_ppm: i128,
    pub paused_market_ppm: i128,
    pub administration_cost_ppm_of_gross: i128,
    pub stabilization_cost_ppm_of_gross: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnualSensitivity {
    pub year: u8,
    pub phase_in_ppm: i128,
    pub gross_reduction_musd_micros: i128,
    pub administration_cost_musd_micros: i128,
    pub stabilization_cost_musd_micros: i128,
    pub net_reduction_musd_micros: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitivityOutput {
    pub annual: Vec<AnnualSensitivity>,
    pub gross_reduction_musd_micros: i128,
    pub administration_cost_musd_micros: i128,
    pub stabilization_cost_musd_micros: i128,
    pub net_reduction_musd_micros: i128,
}

fn checked_fraction(value: i128, numerator: i128, denominator: i128) -> Result<i128, String> {
    value
        .checked_mul(numerator)
        .ok_or("AGR sensitivity multiplication overflow".to_string())?
        .checked_div(denominator)
        .ok_or("AGR sensitivity division failed".to_string())
}

fn validate_share(value: i128, field: &str) -> Result<(), String> {
    if !(0..=SHARE_PPM).contains(&value) {
        return Err(format!("{field} must be between zero and one million ppm"));
    }
    Ok(())
}

pub fn phase_in_ppm(year: u8) -> i128 {
    match year {
        1 => 250_000,
        2 => 500_000,
        _ => SHARE_PPM,
    }
}

pub fn run_sensitivity(input: &SensitivityInput) -> Result<SensitivityOutput, String> {
    if input.horizon_years == 0 || input.horizon_years > 30 {
        return Err("AGR sensitivity horizon must be between 1 and 30 years".to_string());
    }
    if input.annual_retained_premium_musd_micros < 0
        || input.baseline_return_bps < 0
        || input.target_return_bps < 0
    {
        return Err("AGR sensitivity monetary and return inputs must be nonnegative".to_string());
    }
    validate_share(input.participation_ppm, "participation")?;
    validate_share(input.paused_market_ppm, "paused market")?;
    validate_share(
        input.administration_cost_ppm_of_gross,
        "administration cost",
    )?;
    validate_share(input.stabilization_cost_ppm_of_gross, "stabilization cost")?;
    if input.administration_cost_ppm_of_gross + input.stabilization_cost_ppm_of_gross > SHARE_PPM {
        return Err("AGR sensitivity cost shares cannot exceed gross reduction".to_string());
    }

    let return_gap_bps = (input.baseline_return_bps - input.target_return_bps).max(0);
    let full_market_gross = checked_fraction(
        input.annual_retained_premium_musd_micros,
        return_gap_bps,
        BASIS_POINTS_PER_ONE,
    )?;
    let participating_gross =
        checked_fraction(full_market_gross, input.participation_ppm, SHARE_PPM)?;
    let active_gross = checked_fraction(
        participating_gross,
        SHARE_PPM - input.paused_market_ppm,
        SHARE_PPM,
    )?;

    let mut annual = Vec::with_capacity(input.horizon_years.into());
    let mut gross_total = 0_i128;
    let mut administration_total = 0_i128;
    let mut stabilization_total = 0_i128;
    let mut net_total = 0_i128;
    for year in 1..=input.horizon_years {
        let phase = phase_in_ppm(year);
        let gross = checked_fraction(active_gross, phase, SHARE_PPM)?;
        let administration =
            checked_fraction(gross, input.administration_cost_ppm_of_gross, SHARE_PPM)?;
        let stabilization =
            checked_fraction(gross, input.stabilization_cost_ppm_of_gross, SHARE_PPM)?;
        let net = gross
            .checked_sub(administration)
            .and_then(|value| value.checked_sub(stabilization))
            .ok_or("AGR sensitivity net subtraction overflow".to_string())?;
        gross_total += gross;
        administration_total += administration;
        stabilization_total += stabilization;
        net_total += net;
        annual.push(AnnualSensitivity {
            year,
            phase_in_ppm: phase,
            gross_reduction_musd_micros: gross,
            administration_cost_musd_micros: administration,
            stabilization_cost_musd_micros: stabilization,
            net_reduction_musd_micros: net,
        });
    }
    Ok(SensitivityOutput {
        annual,
        gross_reduction_musd_micros: gross_total,
        administration_cost_musd_micros: administration_total,
        stabilization_cost_musd_micros: stabilization_total,
        net_reduction_musd_micros: net_total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case() -> SensitivityInput {
        SensitivityInput {
            horizon_years: 10,
            annual_retained_premium_musd_micros: 1_000_000_000,
            baseline_return_bps: 1_680,
            target_return_bps: 1_200,
            participation_ppm: SHARE_PPM,
            paused_market_ppm: 0,
            administration_cost_ppm_of_gross: 50_000,
            stabilization_cost_ppm_of_gross: 0,
        }
    }

    #[test]
    fn normalized_historical_gap_replays_exactly() {
        let result = run_sensitivity(&case()).unwrap();
        assert_eq!(result.annual.len(), 10);
        assert_eq!(result.gross_reduction_musd_micros, 420_000_000);
        assert_eq!(result.administration_cost_musd_micros, 21_000_000);
        assert_eq!(result.net_reduction_musd_micros, 399_000_000);
    }

    #[test]
    fn market_stress_and_pause_reduce_the_envelope() {
        let mut input = case();
        input.participation_ppm = 900_000;
        input.paused_market_ppm = 200_000;
        input.stabilization_cost_ppm_of_gross = 100_000;
        let result = run_sensitivity(&input).unwrap();
        assert_eq!(result.gross_reduction_musd_micros, 302_400_000);
        assert_eq!(result.administration_cost_musd_micros, 15_120_000);
        assert_eq!(result.stabilization_cost_musd_micros, 30_240_000);
        assert_eq!(result.net_reduction_musd_micros, 257_040_000);

        input.paused_market_ppm = SHARE_PPM;
        assert_eq!(
            run_sensitivity(&input).unwrap().net_reduction_musd_micros,
            0
        );
    }

    #[test]
    fn no_positive_return_gap_produces_no_reduction() {
        let mut input = case();
        input.baseline_return_bps = 1_020;
        assert_eq!(
            run_sensitivity(&input).unwrap().gross_reduction_musd_micros,
            0
        );
    }

    #[test]
    fn rejects_invalid_shares_and_horizon() {
        let mut input = case();
        input.horizon_years = 0;
        assert!(run_sensitivity(&input).is_err());
        input.horizon_years = 10;
        input.paused_market_ppm = SHARE_PPM + 1;
        assert!(run_sensitivity(&input).is_err());
    }
}
