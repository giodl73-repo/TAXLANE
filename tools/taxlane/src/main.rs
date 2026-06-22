use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const MODEL_CHECKS: &[&[&str]] = &[
    &[
        "python",
        "data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py",
        "--check",
    ],
    &[
        "python",
        "data/derived/income_tax_outlay_model/build_decade_summary.py",
        "--check",
    ],
    &[
        "python",
        "data/derived/income_tax_outlay_model/export_chart_views.py",
        "--check",
    ],
    &[
        "python",
        "data/derived/income_tax_outlay_model/build_manifest.py",
        "--check",
    ],
];

const CHART_SPECS: &[&str] = &[
    "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
    "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
    "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [area, command] if area == "income-tax-outlay" && command == "validate" => {
            run_income_tax_outlay_validation()
        }
        _ => {
            eprintln!("usage: taxlane-tools income-tax-outlay validate");
            ExitCode::from(2)
        }
    }
}

fn run_income_tax_outlay_validation() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };

    for check in MODEL_CHECKS {
        if let Err(err) = run_check(&root, check) {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    }

    for spec in CHART_SPECS {
        if let Err(err) = parse_json(&root.join(spec)) {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
        println!("validated JSON spec: {spec}");
    }

    println!(
        "validated income-tax outlay model checks and {} chart specs",
        CHART_SPECS.len()
    );
    ExitCode::SUCCESS
}

fn repo_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|err| format!("failed to get current directory: {err}"))
}

fn run_check(root: &Path, check: &[&str]) -> Result<(), String> {
    let Some((program, args)) = check.split_first() else {
        return Err("empty validation command".to_string());
    };
    println!("+ {} {}", program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|err| format!("failed to run {program}: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "validation command failed with status {status}: {program}"
        ))
    }
}

fn parse_json(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))?;
    Ok(())
}
