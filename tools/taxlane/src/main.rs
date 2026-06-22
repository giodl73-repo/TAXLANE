use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MODEL_CHECKS: &[&[&str]] = &[&[
    "python",
    "data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py",
    "--check",
]];

const CHART_SPECS: &[&str] = &[
    "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
    "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
    "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
];

const MANIFEST_PATH: &str = "data/derived/income_tax_outlay_model/MANIFEST.md";
const ANNUAL_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl";
const DECADE_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl";
const ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv";
const DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv";
const DECADE_MD_PATH: &str = "data/derived/income_tax_outlay_model/decade-summary.md";

const ANNUAL_HEADERS: &[&str] = &[
    "fiscal_year",
    "coverage_note",
    "individual_income_tax_receipts_millions",
    "total_outlays_millions",
    "total_receipts_millions",
    "deficit_gap_millions",
    "borrowed_share_percent_of_outlays",
    "income_tax_coverage_percent_of_outlays",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
    "national_defense_percent",
    "human_resources_percent",
    "physical_resources_percent",
    "net_interest_percent",
    "other_functions_percent",
    "offsetting_receipts_percent",
    "category_percent_sum",
];

const DECADE_HEADERS: &[&str] = &[
    "decade",
    "start_fiscal_year",
    "end_fiscal_year",
    "year_count",
    "coverage_note",
    "cumulative_individual_income_tax_receipts_millions",
    "cumulative_total_outlays_millions",
    "cumulative_total_receipts_millions",
    "cumulative_deficit_gap_millions",
    "borrowed_share_percent_of_outlays",
    "income_tax_coverage_percent_of_outlays",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
    "national_defense_percent",
    "human_resources_percent",
    "physical_resources_percent",
    "net_interest_percent",
    "other_functions_percent",
    "offsetting_receipts_percent",
    "category_percent_sum",
];

const CATEGORY_FIELDS: &[(&str, &str)] = &[
    ("national-defense", "national_defense_percent"),
    ("human-resources", "human_resources_percent"),
    ("physical-resources", "physical_resources_percent"),
    ("net-interest", "net_interest_percent"),
    ("other-functions", "other_functions_percent"),
    (
        "undistributed-offsetting-receipts",
        "offsetting_receipts_percent",
    ),
];

#[derive(Clone, Copy)]
struct Artifact {
    path: &'static str,
    role: &'static str,
    grain: &'static str,
    kind: &'static str,
    canonical: &'static str,
}

const ARTIFACTS: &[Artifact] = &[
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl",
        role: "Canonical annual model rows",
        grain: "fiscal year by broad category",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl",
        role: "Canonical decade summary rows",
        grain: "decade by broad category",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv",
        role: "Chart-ready annual wide view",
        grain: "fiscal year",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv",
        role: "Chart-ready decade wide view",
        grain: "decade",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/README.md",
        role: "Model method and schema note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/source-profile.md",
        role: "Source coverage and reconciliation sample",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/reconciliation-review.md",
        role: "Generated-row reconciliation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_model/decade-summary.md",
        role: "Human-readable decade summary",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/modeled-income-tax-outlays.md",
        role: "Reader-facing packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/README.md",
        role: "Chart catalog",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
        role: "Annual allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
        role: "Decade allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
        role: "Annual financing context chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
        role: "Decade financing context chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "Cargo.toml",
        role: "Rust workspace manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "Cargo.lock",
        role: "Rust dependency lockfile",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "tools/taxlane/Cargo.toml",
        role: "Rust Taxlane tools crate manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "tools/taxlane/src/main.rs",
        role: "Rust validation and manifest command implementation",
        grain: "script",
        kind: "rust",
        canonical: "supporting",
    },
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [area, command] if area == "income-tax-outlay" && command == "validate" => {
            run_income_tax_outlay_validation()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "summary" && flag == "--check" =>
        {
            run_summary_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "summary" => {
            run_summary_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "export" && flag == "--check" =>
        {
            run_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "export" => run_export_write(),
        [area, command, flag]
            if area == "income-tax-outlay" && command == "manifest" && flag == "--check" =>
        {
            run_manifest_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "manifest" => {
            run_manifest_write()
        }
        _ => {
            eprintln!(
                "usage: taxlane-tools income-tax-outlay <validate|summary [--check]|export [--check]|manifest [--check]>"
            );
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

    if let Err(err) = build_decade_summary(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = export_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_manifest(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
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

fn run_summary_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_summary_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_decade_summary(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_chart_views(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_manifest_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match check_manifest(&root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_manifest_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_manifest(&root).and_then(|manifest| {
        fs::write(root.join(MANIFEST_PATH), manifest)
            .map_err(|err| format!("failed to write {MANIFEST_PATH}: {err}"))
    }) {
        Ok(()) => {
            println!("wrote {MANIFEST_PATH}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
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

fn build_decade_summary(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_decade_summary_rows(root)?;
    validate_decade_summary_rows(&rows)?;
    let jsonl = decade_summary_jsonl(&rows);
    let markdown = decade_summary_markdown(&rows)?;

    if check_only {
        compare_text(root, DECADE_JSONL_PATH, &jsonl, "decade JSONL")?;
        compare_text(root, DECADE_MD_PATH, &markdown, "decade Markdown")?;
    } else {
        fs::write(root.join(DECADE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {DECADE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(DECADE_MD_PATH), markdown)
            .map_err(|err| format!("failed to write {DECADE_MD_PATH}: {err}"))?;
    }
    println!("validated {} decade summary rows", rows.len());
    Ok(())
}

#[derive(Clone)]
struct DecadeSummaryRow {
    decade: String,
    start_fiscal_year: i64,
    end_fiscal_year: i64,
    year_count: usize,
    coverage_note: &'static str,
    category_key: String,
    category_label: String,
    cumulative_modeled_income_tax_allocation_amount: f64,
    cumulative_individual_income_tax_receipts_amount: f64,
    category_percent_of_decade_income_tax: f64,
    cumulative_total_outlays_amount: f64,
    cumulative_total_receipts_amount: f64,
    cumulative_deficit_gap_amount: f64,
    borrowed_share_percent_of_outlays: f64,
    income_tax_coverage_percent_of_outlays: f64,
}

fn build_decade_summary_rows(root: &Path) -> Result<Vec<DecadeSummaryRow>, String> {
    let annual_rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut by_decade: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
    for row in annual_rows {
        let year = int_field(&row, "fiscal_year")?;
        by_decade.entry(decade_label(year)).or_default().push(row);
    }

    let mut output = Vec::new();
    for (decade, decade_rows) in by_decade {
        let mut years: Vec<i64> = decade_rows
            .iter()
            .map(|row| int_field(row, "fiscal_year"))
            .collect::<Result<Vec<_>, _>>()?;
        years.sort_unstable();
        years.dedup();

        for year in &years {
            let count = decade_rows
                .iter()
                .filter(|row| int_field(row, "fiscal_year").ok() == Some(*year))
                .count();
            if count != CATEGORY_FIELDS.len() {
                return Err(format!(
                    "{decade}: expected six category rows for fiscal year {year}, found {count}"
                ));
            }
        }

        let anchors: Vec<&serde_json::Value> = decade_rows
            .iter()
            .filter(|row| {
                string_field(row, "category_key").ok().as_deref() == Some("national-defense")
            })
            .collect();
        let income_tax_total = sum_field(&anchors, "individual_income_tax_receipts_amount")?;
        let total_outlays = sum_field(&anchors, "total_outlays_amount")?;
        let total_receipts = sum_field(&anchors, "total_receipts_amount")?;
        let deficit_gap = sum_field(&anchors, "deficit_gap_amount")?;
        let borrowed_share = if total_outlays == 0.0 {
            0.0
        } else {
            deficit_gap / total_outlays * 100.0
        };
        let income_tax_coverage = if total_outlays == 0.0 {
            0.0
        } else {
            income_tax_total / total_outlays * 100.0
        };

        let mut percent_sum = 0.0;
        for (category_key, _) in CATEGORY_FIELDS {
            let category_rows: Vec<&serde_json::Value> = decade_rows
                .iter()
                .filter(|row| {
                    string_field(row, "category_key").ok().as_deref() == Some(*category_key)
                })
                .collect();
            if category_rows.len() != years.len() {
                return Err(format!("{decade}: missing {category_key} rows"));
            }
            let modeled_total = sum_field(&category_rows, "modeled_income_tax_allocation_amount")?;
            let category_percent = modeled_total / income_tax_total * 100.0;
            percent_sum += category_percent;
            output.push(DecadeSummaryRow {
                decade: decade.clone(),
                start_fiscal_year: *years.first().ok_or_else(|| format!("{decade}: no years"))?,
                end_fiscal_year: *years.last().ok_or_else(|| format!("{decade}: no years"))?,
                year_count: years.len(),
                coverage_note: if years.len() < 10 {
                    "partial_decade"
                } else {
                    "full_decade"
                },
                category_key: (*category_key).to_string(),
                category_label: string_field(category_rows[0], "category_label")?,
                cumulative_modeled_income_tax_allocation_amount: round6(modeled_total),
                cumulative_individual_income_tax_receipts_amount: round6(income_tax_total),
                category_percent_of_decade_income_tax: round9(category_percent),
                cumulative_total_outlays_amount: round6(total_outlays),
                cumulative_total_receipts_amount: round6(total_receipts),
                cumulative_deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
            });
        }
        if (percent_sum - 100.0).abs() > 0.00001 {
            return Err(format!(
                "{decade}: category percentages sum to {percent_sum}"
            ));
        }
    }
    Ok(output)
}

fn validate_decade_summary_rows(rows: &[DecadeSummaryRow]) -> Result<(), String> {
    if rows.len() != 54 {
        return Err(format!(
            "expected 54 decade summary rows, found {}",
            rows.len()
        ));
    }
    let mut by_decade: BTreeMap<&str, usize> = BTreeMap::new();
    for row in rows {
        *by_decade.entry(&row.decade).or_default() += 1;
    }
    for (decade, count) in by_decade {
        if count != CATEGORY_FIELDS.len() {
            return Err(format!(
                "{decade}: expected six category rows, found {count}"
            ));
        }
    }
    Ok(())
}

fn decade_summary_jsonl(rows: &[DecadeSummaryRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model_decade_summary\",\"source_record_family\":\"income_tax_outlay_model\",\"model_id\":\"individual-income-tax-proportional-outlays-v1\",\"decade\":{},\"start_fiscal_year\":{},\"end_fiscal_year\":{},\"year_count\":{},\"coverage_note\":{},\"category_key\":{},\"category_label\":{},\"cumulative_modeled_income_tax_allocation_amount\":{},\"cumulative_individual_income_tax_receipts_amount\":{},\"category_percent_of_decade_income_tax\":{},\"cumulative_total_outlays_amount\":{},\"cumulative_total_receipts_amount\":{},\"cumulative_deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"notes\":\"Decade summary derived from annual modeled allocation rows; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}:decade-summary", row.decade, row.category_key)),
            json_string(&row.decade),
            row.start_fiscal_year,
            row.end_fiscal_year,
            row.year_count,
            json_string(row.coverage_note),
            json_string(&row.category_key),
            json_string(&row.category_label),
            decimal_string(row.cumulative_modeled_income_tax_allocation_amount, 6),
            decimal_string(row.cumulative_individual_income_tax_receipts_amount, 6),
            decimal_string(row.category_percent_of_decade_income_tax, 9),
            decimal_string(row.cumulative_total_outlays_amount, 6),
            decimal_string(row.cumulative_total_receipts_amount, 6),
            decimal_string(row.cumulative_deficit_gap_amount, 6),
            decimal_string(row.borrowed_share_percent_of_outlays, 9),
            decimal_string(row.income_tax_coverage_percent_of_outlays, 9),
        ));
    }
    lines.join("\n") + "\n"
}

fn decade_summary_markdown(rows: &[DecadeSummaryRow]) -> Result<String, String> {
    let mut by_decade: BTreeMap<&str, BTreeMap<&str, &DecadeSummaryRow>> = BTreeMap::new();
    for row in rows {
        by_decade
            .entry(&row.decade)
            .or_default()
            .insert(&row.category_key, row);
    }

    let mut lines = vec![
        "# Decade Summary: Modeled Income-Tax Outlay Allocation".to_string(),
        String::new(),
        "This table summarizes the annual draft model by decade. Category".to_string(),
        "percentages equal cumulative modeled category allocations divided by".to_string(),
        "cumulative individual income-tax receipts for the years in that decade.".to_string(),
        "The 2020s are partial because the current actual-year model ends in 2025.".to_string(),
        String::new(),
        "These are modeled allocations, not legal destinations for income-tax".to_string(),
        "receipts.".to_string(),
        String::new(),
        "| Decade | Years | National defense | Human resources | Physical resources | Net interest | Other functions | Offsetting receipts | Borrowed share of outlays | Income-tax coverage of outlays |".to_string(),
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ];

    for (decade, categories) in by_decade {
        let first = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let values: Vec<f64> = CATEGORY_FIELDS
            .iter()
            .map(|(category, _)| {
                categories
                    .get(category)
                    .map(|row| row.category_percent_of_decade_income_tax)
                    .ok_or_else(|| format!("{decade}: missing {category} row"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        lines.push(format!(
            "| {} | {}-{} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            decade,
            first.start_fiscal_year,
            first.end_fiscal_year,
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            first.borrowed_share_percent_of_outlays,
            first.income_tax_coverage_percent_of_outlays
        ));
    }
    Ok(lines.join("\n") + "\n")
}

fn export_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_annual_csv_rows(root)?;
    let decade = build_decade_csv_rows(root)?;
    validate_csv_rows(&annual, "annual", 86)?;
    validate_csv_rows(&decade, "decade", 9)?;

    if check_only {
        compare_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        compare_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    } else {
        write_csv(root, ANNUAL_CSV_PATH, ANNUAL_HEADERS, &annual)?;
        write_csv(root, DECADE_CSV_PATH, DECADE_HEADERS, &decade)?;
    }

    println!(
        "validated {} annual rows and {} decade rows",
        annual.len(),
        decade.len()
    );
    Ok(())
}

fn build_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(ANNUAL_JSONL_PATH))?;
    let mut grouped: BTreeMap<i64, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let year = int_field(&row, "fiscal_year")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(year).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (year, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{year}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("fiscal_year".to_string(), year.to_string());
        row.insert("coverage_note".to_string(), "full_year".to_string());
        insert_json_number(
            &mut row,
            "individual_income_tax_receipts_millions",
            anchor,
            "individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "total_outlays_millions",
            anchor,
            "total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "total_receipts_millions",
            anchor,
            "total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "deficit_gap_millions",
            anchor,
            "deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("{year}: missing {category_key} row"))?;
            let percent = number_field(category, "allocation_share_percent")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

fn build_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(DECADE_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<String, serde_json::Value>> = BTreeMap::new();
    for row in rows {
        let decade = string_field(&row, "decade")?;
        let category = string_field(&row, "category_key")?;
        grouped.entry(decade).or_default().insert(category, row);
    }

    let mut output = Vec::new();
    for (decade, categories) in grouped {
        let anchor = categories
            .get("national-defense")
            .ok_or_else(|| format!("{decade}: missing national-defense row"))?;
        let mut row = BTreeMap::new();
        row.insert("decade".to_string(), decade);
        row.insert(
            "start_fiscal_year".to_string(),
            int_field(anchor, "start_fiscal_year")?.to_string(),
        );
        row.insert(
            "end_fiscal_year".to_string(),
            int_field(anchor, "end_fiscal_year")?.to_string(),
        );
        row.insert(
            "year_count".to_string(),
            int_field(anchor, "year_count")?.to_string(),
        );
        row.insert(
            "coverage_note".to_string(),
            string_field(anchor, "coverage_note")?,
        );
        insert_json_number(
            &mut row,
            "cumulative_individual_income_tax_receipts_millions",
            anchor,
            "cumulative_individual_income_tax_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_outlays_millions",
            anchor,
            "cumulative_total_outlays_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_total_receipts_millions",
            anchor,
            "cumulative_total_receipts_amount",
        );
        insert_json_number(
            &mut row,
            "cumulative_deficit_gap_millions",
            anchor,
            "cumulative_deficit_gap_amount",
        );
        insert_number(
            &mut row,
            "borrowed_share_percent_of_outlays",
            number_field(anchor, "borrowed_share_percent_of_outlays")?,
        );
        insert_number(
            &mut row,
            "income_tax_coverage_percent_of_outlays",
            number_field(anchor, "income_tax_coverage_percent_of_outlays")?,
        );
        row.insert(
            "allocation_method".to_string(),
            string_field(anchor, "allocation_method")?,
        );
        row.insert(
            "legal_allocation_status".to_string(),
            string_field(anchor, "legal_allocation_status")?,
        );
        row.insert(
            "actual_or_projection".to_string(),
            string_field(anchor, "actual_or_projection")?,
        );

        let mut percent_sum = 0.0;
        for (category_key, field_name) in CATEGORY_FIELDS {
            let category = categories
                .get(*category_key)
                .ok_or_else(|| format!("missing {category_key} row"))?;
            let percent = number_field(category, "category_percent_of_decade_income_tax")?;
            insert_number(&mut row, field_name, percent);
            percent_sum += percent;
        }
        insert_number(&mut row, "category_percent_sum", round6(percent_sum));
        output.push(row);
    }
    Ok(output)
}

fn validate_csv_rows(
    rows: &[BTreeMap<String, String>],
    label: &str,
    expected_count: usize,
) -> Result<(), String> {
    if rows.len() != expected_count {
        return Err(format!(
            "{label}: expected {expected_count} rows, found {}",
            rows.len()
        ));
    }
    for row in rows {
        let percent_sum = row
            .get("category_percent_sum")
            .ok_or_else(|| format!("{label}: missing category_percent_sum"))?
            .parse::<f64>()
            .map_err(|err| format!("{label}: invalid category_percent_sum: {err}"))?;
        if (percent_sum - 100.0).abs() > 0.00001 {
            return Err(format!("{label}: percent sum {percent_sum} for {row:?}"));
        }
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!("{label}: missing modeled legal status for {row:?}"));
        }
        if row.get("actual_or_projection").map(String::as_str) != Some("actual") {
            return Err(format!("{label}: unexpected projection status for {row:?}"));
        }
    }
    Ok(())
}

fn write_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let text = csv_text(headers, rows)?;
    fs::write(root.join(relative_path), text)
        .map_err(|err| format!("failed to write {relative_path}: {err}"))
}

fn compare_csv(
    root: &Path,
    relative_path: &str,
    headers: &[&str],
    rows: &[BTreeMap<String, String>],
) -> Result<(), String> {
    let expected = normalize_newlines(&csv_text(headers, rows)?);
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale CSV export: run `cargo run -p taxlane-tools -- income-tax-outlay export`"
        ));
    }
    Ok(())
}

fn compare_text(
    root: &Path,
    relative_path: &str,
    expected: &str,
    label: &str,
) -> Result<(), String> {
    let current = fs::read_to_string(root.join(relative_path))
        .map_err(|err| format!("failed to read {relative_path}: {err}"))?;
    if normalize_newlines(&current) != normalize_newlines(expected) {
        return Err(format!(
            "stale {label}: run `cargo run -p taxlane-tools -- income-tax-outlay summary`"
        ));
    }
    Ok(())
}

fn csv_text(headers: &[&str], rows: &[BTreeMap<String, String>]) -> Result<String, String> {
    if rows.is_empty() {
        return Err("no CSV rows".to_string());
    }
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer
        .write_record(headers.iter().copied())
        .map_err(|err| format!("failed to write CSV header: {err}"))?;
    for row in rows {
        let values: Vec<&str> = headers
            .iter()
            .map(|header| row.get(*header).map(String::as_str).unwrap_or(""))
            .collect();
        writer
            .write_record(values)
            .map_err(|err| format!("failed to write CSV row: {err}"))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|err| format!("failed to finish CSV: {err}"))?;
    String::from_utf8(bytes).map_err(|err| format!("invalid UTF-8 CSV: {err}"))
}

fn read_jsonl(path: PathBuf) -> Result<Vec<serde_json::Value>, String> {
    let content =
        fs::read_to_string(&path).map_err(|err| format!("failed to read {:?}: {err}", path))?;
    content
        .lines()
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))
        })
        .collect()
}

fn int_field(row: &serde_json::Value, field: &str) -> Result<i64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_i64)
        .ok_or_else(|| format!("missing integer field {field}"))
}

fn number_field(row: &serde_json::Value, field: &str) -> Result<f64, String> {
    row.get(field)
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| format!("missing number field {field}"))
}

fn string_field(row: &serde_json::Value, field: &str) -> Result<String, String> {
    row.get(field)
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("missing string field {field}"))
}

fn insert_number(row: &mut BTreeMap<String, String>, field: &str, value: f64) {
    row.insert(field.to_string(), python_float(value));
}

fn insert_json_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    source: &serde_json::Value,
    source_field: &str,
) {
    row.insert(field.to_string(), json_number_string(source, source_field));
}

fn json_number_string(row: &serde_json::Value, field: &str) -> String {
    let value = row
        .get(field)
        .unwrap_or_else(|| panic!("missing number field {field}"));
    if let Some(number) = value.as_i64() {
        number.to_string()
    } else if let Some(number) = value.as_f64() {
        python_float(number)
    } else {
        panic!("missing number field {field}")
    }
}

fn round6(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

fn round9(value: f64) -> f64 {
    (value * 1_000_000_000.0).round() / 1_000_000_000.0
}

fn python_float(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        let text = format!("{value:.12}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn decimal_string(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" {
        "0.0".to_string()
    } else if trimmed.contains('.') {
        trimmed.to_string()
    } else {
        format!("{trimmed}.0")
    }
}

fn decade_label(year: i64) -> String {
    let start = year - year % 10;
    format!("{start}s")
}

fn sum_field(rows: &[&serde_json::Value], field: &str) -> Result<f64, String> {
    rows.iter().map(|row| number_field(row, field)).sum()
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("serializing string should not fail")
}

fn check_manifest(root: &Path) -> Result<(), String> {
    let expected = normalize_newlines(&build_manifest(root)?);
    let current = fs::read_to_string(root.join(MANIFEST_PATH))
        .map_err(|err| format!("failed to read {MANIFEST_PATH}: {err}"))?;
    if normalize_newlines(&current) != expected {
        return Err(format!(
            "stale manifest: run `cargo run -p taxlane-tools -- income-tax-outlay manifest`"
        ));
    }
    println!("validated income-tax outlay artifact manifest");
    Ok(())
}

fn build_manifest(root: &Path) -> Result<String, String> {
    let mut rows = Vec::new();
    for artifact in ARTIFACTS {
        let path = root.join(artifact.path);
        if !path.exists() {
            return Err(format!("missing artifact: {}", artifact.path));
        }
        rows.push((
            artifact,
            count_rows(&path, artifact.kind)?,
            sha256_file(&path)?,
        ));
    }

    let mut lines = vec![
        "# Income-Tax Outlay Model Artifact Manifest".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This manifest records the artifact chain for the modeled allocation of".to_string(),
        "ordinary individual income-tax receipts by broad OMB outlay share.".to_string(),
        String::new(),
        "The annual and decade JSONL files are canonical model outputs. CSV files,".to_string(),
        "Markdown notes, and chart specs are derived or supporting views.".to_string(),
        String::new(),
        "## Model".to_string(),
        String::new(),
        "- Model ID: `individual-income-tax-proportional-outlays-v1`".to_string(),
        "- Coverage: fiscal years 1940-2025 for annual actual-year rows".to_string(),
        "- Projection treatment: FY2026-FY2031 excluded".to_string(),
        "- Legal status: modeled allocation, not legal dedication".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Path | Role | Grain | Rows | Canonical | SHA-256 |".to_string(),
        "|---|---|---|---:|---|---|".to_string(),
    ];

    for (artifact, rows, sha) in rows {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} | `{}` |",
            artifact.path, artifact.role, artifact.grain, rows, artifact.canonical, sha
        ));
    }

    lines.extend([
        String::new(),
        "## Regeneration Order".to_string(),
        String::new(),
        "1. `build_income_tax_outlay_model.py`".to_string(),
        "2. `cargo run -p taxlane-tools -- income-tax-outlay summary`".to_string(),
        "3. `cargo run -p taxlane-tools -- income-tax-outlay export`".to_string(),
        "4. `cargo run -p taxlane-tools -- income-tax-outlay manifest`".to_string(),
        String::new(),
        "Run validation after regeneration:".to_string(),
        String::new(),
        "```powershell".to_string(),
        "cargo run -p taxlane-tools -- income-tax-outlay validate".to_string(),
        "```".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn count_rows(path: &Path, kind: &str) -> Result<String, String> {
    match kind {
        "jsonl" => {
            let content = fs::read_to_string(path)
                .map_err(|err| format!("failed to read {:?}: {err}", path))?;
            let mut count = 0usize;
            for line in content.lines() {
                serde_json::from_str::<serde_json::Value>(line)
                    .map_err(|err| format!("failed to parse JSONL {:?}: {err}", path))?;
                count += 1;
            }
            Ok(count.to_string())
        }
        "csv" => {
            let mut reader = csv::Reader::from_path(path)
                .map_err(|err| format!("failed to read CSV {:?}: {err}", path))?;
            let count = reader.records().count();
            Ok(count.to_string())
        }
        _ => Ok("n/a".to_string()),
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024 * 64];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|err| format!("failed to read {:?}: {err}", path))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}
