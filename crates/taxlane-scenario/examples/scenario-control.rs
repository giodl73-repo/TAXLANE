use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use taxlane_scenario::{Baseline, Scenario, compare};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    for line in stdin.lock().lines() {
        let input: Value = serde_json::from_str(&line?)?;
        let baseline = input["baseline"]
            .as_str()
            .ok_or("baseline must be raw JSON")?;
        let scenario = input["scenario"]
            .as_str()
            .ok_or("scenario must be raw JSON")?;
        // Test-only native seam mirrors the existing compare_json wrapper.
        let evaluate = || -> Result<String, String> {
            let baseline: Baseline = serde_json::from_str(baseline).map_err(|e| e.to_string())?;
            let scenario: Scenario = serde_json::from_str(scenario).map_err(|e| e.to_string())?;
            serde_json::to_string(&compare(&baseline, &scenario)?).map_err(|e| e.to_string())
        };
        let result = match evaluate() {
            Ok(value) => json!({"Ok": value}),
            Err(message) => json!({"Err": message}),
        };
        writeln!(output, "{result}")?;
    }
    Ok(())
}
