pub const SHARE_PPM: i128 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AwardRule {
    pub maximum_usd: i64,
    pub minimum_usd: i64,
    pub sai_ineligibility_threshold_usd: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicantCase {
    pub maximum_eligible: bool,
    pub minimum_eligible: bool,
    pub sai: i64,
    pub threshold_exempt: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubstitutionInput {
    pub grant_reduction_usd: i64,
    pub loan_share_ppm: i128,
    pub institution_aid_share_ppm: i128,
    pub household_work_or_transfer_share_ppm: i128,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubstitutionOutput {
    pub loan_usd_micros: i128,
    pub institution_aid_usd_micros: i128,
    pub household_work_or_transfer_usd_micros: i128,
    pub unmet_need_usd_micros: i128,
}

pub const CURRENT_2026_27: AwardRule = AwardRule {
    maximum_usd: 7_395,
    minimum_usd: 740,
    sai_ineligibility_threshold_usd: 14_790,
};

pub const ADD_ON_ELIMINATION: AwardRule = AwardRule {
    maximum_usd: 6_335,
    minimum_usd: 635,
    sai_ineligibility_threshold_usd: 12_670,
};

fn round_to_nearest_five(value: i64) -> i64 {
    ((value + 2) / 5) * 5
}

pub fn scheduled_award_usd(rule: AwardRule, case: ApplicantCase) -> Result<i64, String> {
    if rule.maximum_usd <= 0
        || rule.minimum_usd <= 0
        || rule.minimum_usd > rule.maximum_usd
        || rule.sai_ineligibility_threshold_usd <= 0
    {
        return Err("Pell award rule values must be positive and ordered".to_string());
    }
    if case.maximum_eligible {
        return Ok(rule.maximum_usd);
    }
    if !case.threshold_exempt && case.sai >= rule.sai_ineligibility_threshold_usd {
        return Ok(0);
    }
    let sai_award = round_to_nearest_five(rule.maximum_usd - case.sai.max(0));
    if sai_award >= rule.minimum_usd {
        return Ok(sai_award.min(rule.maximum_usd));
    }
    Ok(if case.minimum_eligible {
        rule.minimum_usd
    } else {
        0
    })
}

pub fn award_reduction_usd(case: ApplicantCase) -> Result<i64, String> {
    let current = scheduled_award_usd(CURRENT_2026_27, case)?;
    let proposal = scheduled_award_usd(ADD_ON_ELIMINATION, case)?;
    current
        .checked_sub(proposal)
        .ok_or("Pell proposal increased an award unexpectedly".to_string())
}

pub fn linear_intensity_exposure_usd_micros(
    scheduled_reduction_usd: i64,
    enrollment_intensity_ppm: i128,
) -> Result<i128, String> {
    if scheduled_reduction_usd < 0 || !(0..=SHARE_PPM).contains(&enrollment_intensity_ppm) {
        return Err("Pell reduction and enrollment intensity are out of range".to_string());
    }
    i128::from(scheduled_reduction_usd)
        .checked_mul(enrollment_intensity_ppm)
        .ok_or("Pell intensity multiplication overflow".to_string())
}

pub fn allocate_substitution(input: SubstitutionInput) -> Result<SubstitutionOutput, String> {
    if input.grant_reduction_usd < 0 {
        return Err("Pell grant reduction cannot be negative".to_string());
    }
    let shares = [
        input.loan_share_ppm,
        input.institution_aid_share_ppm,
        input.household_work_or_transfer_share_ppm,
    ];
    if shares.iter().any(|share| !(0..=SHARE_PPM).contains(share))
        || shares.iter().sum::<i128>() > SHARE_PPM
    {
        return Err("Pell substitution shares must be valid and sum to at most one".to_string());
    }
    let reduction = i128::from(input.grant_reduction_usd) * SHARE_PPM;
    let allocate = |share: i128| reduction * share / SHARE_PPM;
    let loan = allocate(input.loan_share_ppm);
    let institution = allocate(input.institution_aid_share_ppm);
    let household = allocate(input.household_work_or_transfer_share_ppm);
    Ok(SubstitutionOutput {
        loan_usd_micros: loan,
        institution_aid_usd_micros: institution,
        household_work_or_transfer_usd_micros: household,
        unmet_need_usd_micros: reduction - loan - institution - household,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(sai: i64, minimum_eligible: bool) -> ApplicantCase {
        ApplicantCase {
            maximum_eligible: false,
            minimum_eligible,
            sai,
            threshold_exempt: false,
        }
    }

    #[test]
    fn maximum_and_sai_calculated_cases_lose_the_add_on() {
        let maximum = ApplicantCase {
            maximum_eligible: true,
            ..case(0, false)
        };
        assert_eq!(award_reduction_usd(maximum).unwrap(), 1_060);
        assert_eq!(award_reduction_usd(case(1_000, false)).unwrap(), 1_060);
        assert_eq!(award_reduction_usd(case(5_500, false)).unwrap(), 1_060);
    }

    #[test]
    fn minimum_and_threshold_rules_create_nonuniform_exposure() {
        assert_eq!(award_reduction_usd(case(6_000, true)).unwrap(), 760);
        assert_eq!(award_reduction_usd(case(6_000, false)).unwrap(), 1_395);
        assert_eq!(award_reduction_usd(case(7_000, true)).unwrap(), 105);
        assert_eq!(award_reduction_usd(case(13_000, true)).unwrap(), 740);
    }

    #[test]
    fn special_rule_preserves_threshold_exception() {
        let mut special = case(13_000, true);
        special.threshold_exempt = true;
        assert_eq!(award_reduction_usd(special).unwrap(), 105);
    }

    #[test]
    fn intensity_is_linear_exposure_not_a_payment_schedule() {
        assert_eq!(
            linear_intensity_exposure_usd_micros(1_060, 250_000).unwrap(),
            265_000_000
        );
        assert_eq!(
            linear_intensity_exposure_usd_micros(1_060, 750_000).unwrap(),
            795_000_000
        );
        assert!(linear_intensity_exposure_usd_micros(1_060, SHARE_PPM + 1).is_err());
    }

    #[test]
    fn substitution_stress_preserves_the_household_gap_identity() {
        let output = allocate_substitution(SubstitutionInput {
            grant_reduction_usd: 1_060,
            loan_share_ppm: 500_000,
            institution_aid_share_ppm: 250_000,
            household_work_or_transfer_share_ppm: 0,
        })
        .unwrap();
        assert_eq!(output.loan_usd_micros, 530_000_000);
        assert_eq!(output.institution_aid_usd_micros, 265_000_000);
        assert_eq!(output.unmet_need_usd_micros, 265_000_000);
    }
}
