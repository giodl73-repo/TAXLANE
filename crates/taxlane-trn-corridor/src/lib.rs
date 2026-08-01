//! Mechanical lifecycle-price stresses for a tolled freight connector.
//!
//! This crate deliberately does not estimate demand, financing returns, toll
//! elasticity, benefits, or government savings. It turns an explicitly chosen
//! scenario into transparent traffic, cost, and price identities.

const PPM: i128 = 1_000_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FreightConnectorStressInput {
    pub design_daily_vehicles: i128,
    pub design_daily_trucks: i128,
    pub utilization_ppm: i128,
    pub operating_days: i128,
    pub total_capital_usd_micros: i128,
    pub public_capital_usd_micros: i128,
    pub recovery_years: i128,
    pub annual_om_ppm_of_total_capital: i128,
    pub annual_public_lease_payment_usd_micros: i128,
    pub truck_price_weight: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FreightConnectorStressResult {
    pub private_capital_usd_micros: i128,
    pub scenario_annual_vehicles_micros: i128,
    pub scenario_annual_trucks_micros: i128,
    pub scenario_annual_other_vehicles_micros: i128,
    pub straight_line_annual_private_capital_usd_micros: i128,
    pub annual_om_stress_usd_micros: i128,
    pub annual_revenue_requirement_usd_micros: i128,
    pub weighted_annual_vehicle_units_micros: i128,
    pub other_vehicle_equivalent_price_usd_micros: i128,
    pub truck_equivalent_price_usd_micros: i128,
    pub public_capital_share_ppm: i128,
}

pub fn run_freight_connector_stress(
    input: &FreightConnectorStressInput,
) -> Result<FreightConnectorStressResult, String> {
    if input.design_daily_vehicles <= 0
        || input.design_daily_trucks < 0
        || input.design_daily_trucks > input.design_daily_vehicles
        || !(1..=PPM).contains(&input.utilization_ppm)
        || input.operating_days <= 0
        || input.total_capital_usd_micros <= 0
        || input.public_capital_usd_micros < 0
        || input.public_capital_usd_micros > input.total_capital_usd_micros
        || input.recovery_years <= 0
        || input.annual_om_ppm_of_total_capital < 0
        || input.annual_public_lease_payment_usd_micros < 0
        || input.truck_price_weight <= 0
    {
        return Err("invalid freight connector stress input".to_string());
    }

    let private_capital = input.total_capital_usd_micros - input.public_capital_usd_micros;
    let annual_design_vehicles = input.design_daily_vehicles * input.operating_days;
    let annual_design_trucks = input.design_daily_trucks * input.operating_days;
    let annual_design_other = annual_design_vehicles - annual_design_trucks;
    let scenario_vehicles = annual_design_vehicles * input.utilization_ppm;
    let scenario_trucks = annual_design_trucks * input.utilization_ppm;
    let scenario_other = annual_design_other * input.utilization_ppm;
    let annual_private_capital = private_capital / input.recovery_years;
    let annual_om = input.total_capital_usd_micros * input.annual_om_ppm_of_total_capital / PPM;
    let annual_requirement =
        annual_private_capital + annual_om + input.annual_public_lease_payment_usd_micros;
    let weighted_units = scenario_other + scenario_trucks * input.truck_price_weight;
    if weighted_units <= 0 {
        return Err("freight connector stress produced zero weighted traffic".to_string());
    }
    let other_price = annual_requirement * PPM / weighted_units;
    let truck_price = other_price * input.truck_price_weight;

    Ok(FreightConnectorStressResult {
        private_capital_usd_micros: private_capital,
        scenario_annual_vehicles_micros: scenario_vehicles,
        scenario_annual_trucks_micros: scenario_trucks,
        scenario_annual_other_vehicles_micros: scenario_other,
        straight_line_annual_private_capital_usd_micros: annual_private_capital,
        annual_om_stress_usd_micros: annual_om,
        annual_revenue_requirement_usd_micros: annual_requirement,
        weighted_annual_vehicle_units_micros: weighted_units,
        other_vehicle_equivalent_price_usd_micros: other_price,
        truck_equivalent_price_usd_micros: truck_price,
        public_capital_share_ppm: input.public_capital_usd_micros * PPM
            / input.total_capital_usd_micros,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> FreightConnectorStressInput {
        FreightConnectorStressInput {
            design_daily_vehicles: 11_000,
            design_daily_trucks: 6_600,
            utilization_ppm: 700_000,
            operating_days: 365,
            total_capital_usd_micros: 190_000_000_000_000,
            public_capital_usd_micros: 21_000_000_000_000,
            recovery_years: 30,
            annual_om_ppm_of_total_capital: 20_000,
            annual_public_lease_payment_usd_micros: 100_000_000_000,
            truck_price_weight: 3,
        }
    }

    #[test]
    fn replays_mechanical_traffic_cost_and_weighted_price() {
        let result = run_freight_connector_stress(&fixture()).unwrap();
        assert_eq!(result.private_capital_usd_micros, 169_000_000_000_000);
        assert_eq!(result.scenario_annual_vehicles_micros, 2_810_500_000_000);
        assert_eq!(result.scenario_annual_trucks_micros, 1_686_300_000_000);
        assert_eq!(result.annual_om_stress_usd_micros, 3_800_000_000_000);
        assert_eq!(
            result.annual_revenue_requirement_usd_micros,
            9_533_333_333_333
        );
        assert_eq!(result.public_capital_share_ppm, 110_526);
        assert_eq!(result.other_vehicle_equivalent_price_usd_micros, 1_541_837);
        assert_eq!(result.truck_equivalent_price_usd_micros, 4_625_511);
    }

    #[test]
    fn rejects_impossible_truck_count() {
        let mut input = fixture();
        input.design_daily_trucks = 11_001;
        assert!(run_freight_connector_stress(&input).is_err());
    }
}
