use taxlane_scenario::{Baseline, Scenario, compare};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("Provide a baseline JSON path")?;
    let baseline: Baseline = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let result = compare(&baseline, &Scenario::default())?;
    println!(
        "{} lanes; FY{}; baseline deficit {} million dollars",
        baseline.lanes.len(),
        result.fiscal_year,
        result.deficit_musd
    );
    Ok(())
}
