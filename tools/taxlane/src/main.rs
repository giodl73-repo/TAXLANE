use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use roxmltree::Document;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use zip::ZipArchive;

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
const SOURCE_PROFILE_PATH: &str = "data/derived/income_tax_outlay_model/source-profile.md";
const OBSERVED_DATE: &str = "2026-06-21";
const MODEL_ID: &str = "individual-income-tax-proportional-outlays-v1";
const TABLE_1_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx";
const TABLE_2_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx";
const TABLE_2_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx";
const TABLE_3_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx";
const RECEIPT_SHARE_JSONL_PATH: &str =
    "data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-2-FY2027.2026-06-21.draft.jsonl";
const RECEIPT_SHARE_PROFILE_PATH: &str = "data/extracted/receipt_source/table-2-2-profile.md";
const SOURCE_IDS: &[&str] = &[
    "SRC-OMB-HIST-1-1-FY2027",
    "SRC-OMB-HIST-2-1-FY2027",
    "SRC-OMB-HIST-3-1-FY2027",
];

const BROAD_CATEGORIES: &[(&str, &str, i64)] = &[
    ("national-defense", "National Defense", 4),
    ("human-resources", "Human resources", 5),
    ("physical-resources", "Physical resources", 14),
    ("net-interest", "Net interest", 22),
    ("other-functions", "Other functions", 25),
    (
        "undistributed-offsetting-receipts",
        "Undistributed offsetting receipts",
        32,
    ),
];

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
            if area == "income-tax-outlay" && command == "model" && flag == "--check" =>
        {
            run_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "model" => run_model_write(),
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
        [area, command, flag]
            if area == "receipt-source" && command == "table-2-2" && flag == "--check" =>
        {
            run_table_2_2_check()
        }
        [area, command] if area == "receipt-source" && command == "table-2-2" => {
            run_table_2_2_write()
        }
        _ => {
            eprintln!(
                "usage: taxlane-tools income-tax-outlay <validate|model [--check]|summary [--check]|export [--check]|manifest [--check]>\n       taxlane-tools receipt-source table-2-2 [--check]"
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

    if let Err(err) = build_annual_model(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
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

fn run_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_annual_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
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

fn run_table_2_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_2_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_receipt_share_table_2_2(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn repo_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|err| format!("failed to get current directory: {err}"))
}

fn parse_json(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))?;
    Ok(())
}

fn build_receipt_share_table_2_2(root: &Path, check_only: bool) -> Result<(), String> {
    let rows = build_receipt_share_rows(root)?;
    validate_receipt_share_rows(&rows)?;
    let jsonl = receipt_share_jsonl(&rows);
    let markdown = receipt_share_profile_markdown(&rows)?;

    if check_only {
        compare_text(
            root,
            RECEIPT_SHARE_JSONL_PATH,
            &jsonl,
            "Table 2.2 receipt share JSONL",
        )?;
        compare_text(
            root,
            RECEIPT_SHARE_PROFILE_PATH,
            &markdown,
            "Table 2.2 receipt share profile",
        )?;
    } else {
        fs::write(root.join(RECEIPT_SHARE_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_JSONL_PATH}: {err}"))?;
        fs::write(root.join(RECEIPT_SHARE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {RECEIPT_SHARE_PROFILE_PATH}: {err}"))?;
    }

    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    println!(
        "validated {} Table 2.2 receipt share rows for {}-{}",
        rows.len(),
        first_year,
        last_year
    );
    Ok(())
}

#[derive(Clone, Copy)]
struct ReceiptShareCategory {
    column: &'static str,
    receipt_category: &'static str,
    source_receipt_label: &'static str,
    allocation_status: &'static str,
    notes: &'static str,
}

const RECEIPT_SHARE_CATEGORIES: &[ReceiptShareCategory] = &[
    ReceiptShareCategory {
        column: "B",
        receipt_category: "individual-income-taxes",
        source_receipt_label: "Individual Income Taxes",
        allocation_status: "general_receipt",
        notes: "Share of total receipts; AP13 general-fund concept supports ordinary individual income taxes as general receipts absent a cited legal dedication.",
    },
    ReceiptShareCategory {
        column: "C",
        receipt_category: "corporation-income-taxes",
        source_receipt_label: "Corporation Income Taxes",
        allocation_status: "unknown",
        notes: "Share of total receipts; allocation label remains unknown pending source-specific review.",
    },
    ReceiptShareCategory {
        column: "D",
        receipt_category: "social-insurance-and-retirement-receipts",
        source_receipt_label: "Social Insurance and Retirement Receipts Total",
        allocation_status: "unknown",
        notes: "Share of total receipts; social-insurance receipts remain separate from individual income taxes and require subcomponent review for allocation.",
    },
    ReceiptShareCategory {
        column: "G",
        receipt_category: "excise-taxes",
        source_receipt_label: "Excise Taxes",
        allocation_status: "unknown",
        notes: "Share of total receipts; excise taxes can include general and dedicated treatment.",
    },
    ReceiptShareCategory {
        column: "H",
        receipt_category: "other-receipts",
        source_receipt_label: "Other",
        allocation_status: "unknown",
        notes: "Share of total receipts; other receipts are heterogeneous and need source-specific review.",
    },
    ReceiptShareCategory {
        column: "I",
        receipt_category: "total-receipts",
        source_receipt_label: "Total Receipts",
        allocation_status: "mixed",
        notes: "Total receipts combine categories with different budget treatment and are not a legal allocation category.",
    },
];

#[derive(Clone)]
struct ReceiptShareRow {
    fiscal_year: i64,
    source_row: i64,
    source_column: &'static str,
    receipt_category: &'static str,
    source_receipt_label: &'static str,
    percent: f64,
    actual_or_projection: &'static str,
    allocation_status: &'static str,
    notes: &'static str,
}

fn build_receipt_share_rows(root: &Path) -> Result<Vec<ReceiptShareRow>, String> {
    let sheet = read_sheet(&root.join(TABLE_2_2_PATH))?;
    let mut rows = Vec::new();

    for (row_num, cells) in &sheet {
        let Some(year_label) = table_2_2_year_label(cells.get("A")) else {
            continue;
        };
        let Some((year, actual_or_projection)) = parse_table_2_2_year(&year_label) else {
            continue;
        };

        for category in RECEIPT_SHARE_CATEGORIES {
            let Some(percent) = number_cell(cells.get(category.column)) else {
                return Err(format!(
                    "Table 2.2 row {row_num} missing percent in column {}",
                    category.column
                ));
            };
            rows.push(ReceiptShareRow {
                fiscal_year: year,
                source_row: *row_num,
                source_column: category.column,
                receipt_category: category.receipt_category,
                source_receipt_label: category.source_receipt_label,
                percent: round6(percent),
                actual_or_projection,
                allocation_status: category.allocation_status,
                notes: category.notes,
            });
        }
    }

    rows.sort_by_key(|row| {
        (
            row.fiscal_year,
            receipt_share_sort_key(row.receipt_category),
        )
    });
    Ok(rows)
}

fn table_2_2_year_label(value: Option<&CellValue>) -> Option<String> {
    text_cell(value).or_else(|| int_cell(value).map(|year| year.to_string()))
}

fn parse_table_2_2_year(label: &str) -> Option<(i64, &'static str)> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    if let Some(year) = trimmed.strip_suffix(" estimate") {
        return year.parse::<i64>().ok().map(|year| (year, "estimate"));
    }
    trimmed.parse::<i64>().ok().map(|year| {
        let status = if year <= 2025 { "actual" } else { "estimate" };
        (year, status)
    })
}

fn receipt_share_sort_key(category: &str) -> usize {
    RECEIPT_SHARE_CATEGORIES
        .iter()
        .position(|candidate| candidate.receipt_category == category)
        .unwrap_or(usize::MAX)
}

fn validate_receipt_share_rows(rows: &[ReceiptShareRow]) -> Result<(), String> {
    if rows.len() != 588 {
        return Err(format!(
            "expected 588 Table 2.2 receipt share rows, found {}",
            rows.len()
        ));
    }

    let mut by_year: BTreeMap<i64, Vec<&ReceiptShareRow>> = BTreeMap::new();
    for row in rows {
        if !(0.0..=100.0).contains(&row.percent) {
            return Err(format!(
                "{} {} percent out of range: {}",
                row.fiscal_year, row.receipt_category, row.percent
            ));
        }
        by_year.entry(row.fiscal_year).or_default().push(row);
    }

    for (year, year_rows) in by_year {
        if year_rows.len() != RECEIPT_SHARE_CATEGORIES.len() {
            return Err(format!(
                "{year}: expected {} share rows, found {}",
                RECEIPT_SHARE_CATEGORIES.len(),
                year_rows.len()
            ));
        }
        let category_sum: f64 = year_rows
            .iter()
            .filter(|row| row.receipt_category != "total-receipts")
            .map(|row| row.percent)
            .sum();
        if (category_sum - 100.0).abs() > 0.25 {
            return Err(format!(
                "{year}: receipt-source shares sum to {category_sum}"
            ));
        }
        let total = year_rows
            .iter()
            .find(|row| row.receipt_category == "total-receipts")
            .map(|row| row.percent)
            .ok_or_else(|| format!("{year}: missing total receipts share"))?;
        if (total - 100.0).abs() > 0.000001 {
            return Err(format!("{year}: total receipts share is {total}"));
        }
    }
    Ok(())
}

fn receipt_share_jsonl(rows: &[ReceiptShareRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let mut source_ids = vec!["SRC-OMB-HIST-2-2-FY2027"];
        if row.receipt_category == "individual-income-taxes" {
            source_ids.push("SRC-OMB-AP-13-FUNDS-FY2027");
        }
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"receipt_source\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":\"OMB Historical Table 2.2 FY2027\",\"source_row_ref\":{},\"receipt_category\":{},\"source_receipt_label\":{},\"measure\":\"share_of_total\",\"amount\":null,\"percent\":{},\"amount_units\":\"percent\",\"actual_or_projection\":{},\"fund_group_link\":null,\"allocation_status\":{},\"status\":\"draft\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "receipt:{}:{}:share-of-total",
                row.fiscal_year, row.receipt_category
            )),
            row.fiscal_year,
            source_ids
                .iter()
                .map(|source| json_string(source))
                .collect::<Vec<_>>()
                .join(","),
            json_string(&format!(
                "Table!A{}:{}{}; column {} {}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.source_column,
                row.source_receipt_label
            )),
            json_string(row.receipt_category),
            json_string(row.source_receipt_label),
            decimal_string(row.percent, 6),
            json_string(row.actual_or_projection),
            json_string(row.allocation_status),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn receipt_share_profile_markdown(rows: &[ReceiptShareRow]) -> Result<String, String> {
    let first_year = rows
        .first()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let last_year = rows
        .last()
        .ok_or_else(|| "no Table 2.2 rows".to_string())?
        .fiscal_year;
    let year_count = rows.len() / RECEIPT_SHARE_CATEGORIES.len();
    let estimate_count = rows
        .iter()
        .filter(|row| row.actual_or_projection == "estimate")
        .map(|row| row.fiscal_year)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let sample_years = [1934, 1940, 1980, 2000, 2025, 2031];
    let mut by_year: BTreeMap<i64, BTreeMap<&str, f64>> = BTreeMap::new();
    for row in rows {
        by_year
            .entry(row.fiscal_year)
            .or_default()
            .insert(row.receipt_category, row.percent);
    }

    let mut lines = vec![
        "# OMB Table 2.2 Receipt Share Profile".to_string(),
        String::new(),
        "## Source".to_string(),
        String::new(),
        "- Source ID: `SRC-OMB-HIST-2-2-FY2027`".to_string(),
        "- Raw artifact: `data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx`"
            .to_string(),
        "- Table title: `Table 2.2 - PERCENTAGE COMPOSITION OF RECEIPTS BY SOURCE: 1934 - 2031`"
            .to_string(),
        String::new(),
        "## Coverage".to_string(),
        String::new(),
        format!("- Fiscal years emitted: {first_year}-{last_year}"),
        format!("- Year count: {year_count}"),
        format!("- Estimate years: {estimate_count}"),
        format!("- Record count: {}", rows.len()),
        String::new(),
        "## Extracted Columns".to_string(),
        String::new(),
        "| Column | Receipt category | Source label |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for category in RECEIPT_SHARE_CATEGORIES {
        lines.push(format!(
            "| {} | `{}` | {} |",
            category.column, category.receipt_category, category.source_receipt_label
        ));
    }
    lines.extend([
        String::new(),
        "## Sample Shares".to_string(),
        String::new(),
        "Percentages are OMB-reported shares of total receipts.".to_string(),
        String::new(),
        "| Fiscal year | Individual income | Corporation income | Social insurance | Excise | Other | Total receipts |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for year in sample_years {
        let categories = by_year
            .get(&year)
            .ok_or_else(|| format!("missing sample year {year}"))?;
        lines.push(format!(
            "| {year} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% |",
            categories["individual-income-taxes"],
            categories["corporation-income-taxes"],
            categories["social-insurance-and-retirement-receipts"],
            categories["excise-taxes"],
            categories["other-receipts"],
            categories["total-receipts"],
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Keep Table 2.2 percentage rows separate from Table 2.1 amount rows.".to_string(),
        "- Skip the transition-quarter `TQ` row because it is not a fiscal year.".to_string(),
        "- Preserve estimate years as `actual_or_projection = \"estimate\"`.".to_string(),
        "- Treat total receipts as `mixed` because it combines categories with different budget treatment.".to_string(),
        "- Keep non-individual receipt allocation labels as `unknown` pending narrower review.".to_string(),
        String::new(),
    ]);
    Ok(lines.join("\n"))
}

fn build_annual_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_annual_records(root)?;
    let jsonl = annual_model_jsonl(&records);
    let markdown = source_profile_markdown(&profile);

    if check_only {
        compare_text(root, ANNUAL_JSONL_PATH, &jsonl, "annual model JSONL")?;
        compare_text(root, SOURCE_PROFILE_PATH, &markdown, "source profile")?;
    } else {
        fs::write(root.join(ANNUAL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {ANNUAL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SOURCE_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {SOURCE_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} rows for {}-{}",
        profile.record_count, profile.first_year, profile.last_year
    );
    Ok(())
}

#[derive(Clone)]
enum CellValue {
    Number(f64),
    Text(String),
}

#[derive(Clone)]
struct Table11Row {
    row: i64,
    total_receipts: f64,
    total_outlays: f64,
    surplus_or_deficit: f64,
}

#[derive(Clone)]
struct Table21Row {
    row: i64,
    individual_income_tax: f64,
}

#[derive(Clone)]
struct AnnualRecord {
    fiscal_year: i64,
    category_key: &'static str,
    category_label: &'static str,
    table_11_row: i64,
    table_21_row: i64,
    table_31_row: i64,
    category_outlays_amount: f64,
    total_outlays_amount: f64,
    category_total_outlays_amount: f64,
    individual_income_tax_receipts_amount: f64,
    outlay_share_percent: f64,
    allocation_share_percent: f64,
    modeled_income_tax_allocation_amount: f64,
    total_receipts_amount: f64,
    surplus_or_deficit_amount: f64,
    deficit_gap_amount: f64,
    borrowed_share_percent_of_outlays: f64,
    income_tax_coverage_percent_of_outlays: f64,
    category_total_reconciliation_difference_amount: f64,
}

struct AnnualCheck {
    year: i64,
    table_1_1_outlays: f64,
    table_3_1_outlays: f64,
    category_total: f64,
    income_tax: f64,
    modeled_sum: f64,
    deficit_gap: f64,
}

struct AnnualProfile {
    year_count: usize,
    first_year: i64,
    last_year: i64,
    record_count: usize,
    annual_checks: Vec<AnnualCheck>,
}

fn build_annual_records(root: &Path) -> Result<(Vec<AnnualRecord>, AnnualProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let (years_31, t31) = parse_table_3_1(&read_sheet(&root.join(TABLE_3_1_PATH))?)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut errors = Vec::new();
    let mut annual_checks = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let Some(total_outlays_31) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        if (table_11.total_outlays - total_outlays_31).abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }

        let category_total: f64 = BROAD_CATEGORIES
            .iter()
            .map(|(key, _, _)| {
                t31.get(*key)
                    .and_then(|values| values.get(year))
                    .copied()
                    .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        let category_total_difference = category_total - total_outlays_31;
        if (category_total - total_outlays_31).abs() > 2.0 {
            errors.push(format!(
                "{year}: category total {category_total} does not reconcile to Table 3.1 total {total_outlays_31}"
            ));
        }

        let income_tax = table_21.individual_income_tax;
        let total_receipts = table_11.total_receipts;
        let surplus_or_deficit = table_11.surplus_or_deficit;
        let deficit_gap = (total_outlays_31 - total_receipts).max(0.0);
        let borrowed_share = deficit_gap / total_outlays_31 * 100.0;
        let income_tax_coverage = income_tax / total_outlays_31 * 100.0;
        let mut modeled_sum = 0.0;

        for (key, label, table_row) in BROAD_CATEGORIES {
            let category_outlays = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            let outlay_share = category_outlays / total_outlays_31 * 100.0;
            let allocation_share = category_outlays / category_total * 100.0;
            let modeled_amount = income_tax * category_outlays / category_total;
            modeled_sum += modeled_amount;
            records.push(AnnualRecord {
                fiscal_year: *year,
                category_key: key,
                category_label: label,
                table_11_row: table_11.row,
                table_21_row: table_21.row,
                table_31_row: *table_row,
                category_outlays_amount: round6(category_outlays),
                total_outlays_amount: round6(total_outlays_31),
                category_total_outlays_amount: round6(category_total),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(outlay_share),
                allocation_share_percent: round9(allocation_share),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
                total_receipts_amount: round6(total_receipts),
                surplus_or_deficit_amount: round6(surplus_or_deficit),
                deficit_gap_amount: round6(deficit_gap),
                borrowed_share_percent_of_outlays: round9(borrowed_share),
                income_tax_coverage_percent_of_outlays: round9(income_tax_coverage),
                category_total_reconciliation_difference_amount: round6(category_total_difference),
            });
        }

        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: modeled allocation sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        annual_checks.push(AnnualCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_outlays: total_outlays_31,
            category_total,
            income_tax,
            modeled_sum,
            deficit_gap,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years.first().ok_or_else(|| "no annual years".to_string())?;
    let last_year = *years.last().ok_or_else(|| "no annual years".to_string())?;
    let profile = AnnualProfile {
        year_count: years.len(),
        first_year,
        last_year,
        record_count: records.len(),
        annual_checks,
    };
    Ok((records, profile))
}

fn read_sheet(path: &Path) -> Result<BTreeMap<i64, BTreeMap<String, CellValue>>, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    let mut archive =
        ZipArchive::new(file).map_err(|err| format!("failed to read XLSX {:?}: {err}", path))?;
    let shared = read_shared_strings(&mut archive)?;
    let mut sheet_xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .map_err(|err| format!("failed to read sheet1.xml from {:?}: {err}", path))?
        .read_to_string(&mut sheet_xml)
        .map_err(|err| format!("failed to decode sheet1.xml from {:?}: {err}", path))?;
    let doc = Document::parse(&sheet_xml)
        .map_err(|err| format!("failed to parse sheet1.xml from {:?}: {err}", path))?;
    let mut rows = BTreeMap::new();
    for row in doc.descendants().filter(|node| node.has_tag_name("row")) {
        let row_num = row
            .attribute("r")
            .and_then(|value| value.parse::<i64>().ok())
            .ok_or_else(|| format!("sheet row without numeric r in {:?}", path))?;
        let mut cells = BTreeMap::new();
        for cell in row.children().filter(|node| node.has_tag_name("c")) {
            let Some(reference) = cell.attribute("r") else {
                continue;
            };
            let column = cell_column(reference);
            if column.is_empty() {
                continue;
            }
            let cell_type = cell.attribute("t");
            let raw = cell
                .children()
                .find(|node| node.has_tag_name("v"))
                .and_then(|node| node.text());
            let value = match (cell_type, raw) {
                (Some("s"), Some(raw)) => shared
                    .get(raw.parse::<usize>().map_err(|err| {
                        format!("invalid shared string index {raw:?} in {:?}: {err}", path)
                    })?)
                    .cloned(),
                (Some("inlineStr"), _) => Some(
                    cell.descendants()
                        .filter(|node| node.has_tag_name("t"))
                        .filter_map(|node| node.text())
                        .collect::<String>(),
                ),
                (_, Some(raw)) => Some(raw.to_string()),
                _ => None,
            };
            if let Some(value) = value.and_then(|value| cell_value(&value)) {
                cells.insert(column, value);
            }
        }
        rows.insert(row_num, cells);
    }
    Ok(rows)
}

fn read_shared_strings<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<Vec<String>, String> {
    let Ok(mut file) = archive.by_name("xl/sharedStrings.xml") else {
        return Ok(Vec::new());
    };
    let mut xml = String::new();
    file.read_to_string(&mut xml)
        .map_err(|err| format!("failed to decode sharedStrings.xml: {err}"))?;
    let doc =
        Document::parse(&xml).map_err(|err| format!("failed to parse sharedStrings.xml: {err}"))?;
    let strings = doc
        .descendants()
        .filter(|node| node.has_tag_name("si"))
        .map(|si| {
            si.descendants()
                .filter(|node| node.has_tag_name("t"))
                .filter_map(|node| node.text())
                .collect::<String>()
        })
        .collect();
    Ok(strings)
}

fn cell_column(reference: &str) -> String {
    reference
        .chars()
        .take_while(|char| char.is_ascii_alphabetic())
        .collect()
}

fn cell_value(raw: &str) -> Option<CellValue> {
    let value = raw.trim();
    if value.is_empty() || value == ".........." {
        return None;
    }
    if value == "-*" {
        return Some(CellValue::Number(0.0));
    }
    match value.parse::<f64>() {
        Ok(number) => Some(CellValue::Number(number)),
        Err(_) => Some(CellValue::Text(value.to_string())),
    }
}

fn parse_table_1_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table11Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let Some(year) = int_cell(cells.get("A")) else {
            continue;
        };
        let (Some(receipts), Some(outlays), Some(surplus_or_deficit)) = (
            number_cell(cells.get("B")),
            number_cell(cells.get("C")),
            number_cell(cells.get("D")),
        ) else {
            continue;
        };
        output.insert(
            year,
            Table11Row {
                row: *row_num,
                total_receipts: receipts,
                total_outlays: outlays,
                surplus_or_deficit,
            },
        );
    }
    output
}

fn parse_table_2_1(rows: &BTreeMap<i64, BTreeMap<String, CellValue>>) -> BTreeMap<i64, Table21Row> {
    let mut output = BTreeMap::new();
    for (row_num, cells) in rows {
        let (Some(year), Some(amount)) = (int_cell(cells.get("A")), number_cell(cells.get("B")))
        else {
            continue;
        };
        output.insert(
            year,
            Table21Row {
                row: *row_num,
                individual_income_tax: amount,
            },
        );
    }
    output
}

fn parse_table_3_1(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(Vec<i64>, BTreeMap<String, BTreeMap<i64, f64>>), String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut years_by_col = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            years_by_col.insert(column.clone(), year);
        }
    }

    let mut categories = BTreeMap::new();
    let mut table_rows: Vec<(&str, &str, i64)> = BROAD_CATEGORIES.to_vec();
    table_rows.push(("total-federal-outlays", "Total, Federal outlays", 35));
    for (key, label, row_num) in table_rows {
        let cells = rows
            .get(&row_num)
            .ok_or_else(|| format!("missing Table 3.1 row {row_num}"))?;
        if text_cell(cells.get("A")).as_deref() != Some(label) {
            return Err(format!(
                "Unexpected Table 3.1 row {row_num}: {:?}",
                text_cell(cells.get("A"))
            ));
        }
        let mut values = BTreeMap::new();
        for (column, year) in &years_by_col {
            if let Some(value) = number_cell(cells.get(column)) {
                values.insert(*year, value);
            }
        }
        categories.insert(key.to_string(), values);
    }
    let mut years = years_by_col.values().copied().collect::<Vec<_>>();
    years.sort_unstable();
    Ok((years, categories))
}

fn int_cell(value: Option<&CellValue>) -> Option<i64> {
    match value {
        Some(CellValue::Number(number)) if number.fract() == 0.0 => Some(*number as i64),
        _ => None,
    }
}

fn number_cell(value: Option<&CellValue>) -> Option<f64> {
    match value {
        Some(CellValue::Number(number)) => Some(*number),
        _ => None,
    }
}

fn text_cell(value: Option<&CellValue>) -> Option<String> {
    match value {
        Some(CellValue::Text(text)) => Some(text.clone()),
        _ => None,
    }
}

fn annual_model_jsonl(records: &[AnnualRecord]) -> String {
    let mut lines = Vec::new();
    for record in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table_refs\":{{\"fiscal_spine\":{},\"tax_receipts\":{},\"outlay_category\":{},\"outlay_total\":\"OMB Historical Table 3.1 FY2027 row 35\"}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"category_key\":{},\"category_label\":{},\"category_outlays_amount\":{},\"total_outlays_amount\":{},\"category_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"total_receipts_amount\":{},\"surplus_or_deficit_amount\":{},\"deficit_gap_amount\":{},\"borrowed_share_percent_of_outlays\":{},\"income_tax_coverage_percent_of_outlays\":{},\"category_total_reconciliation_difference_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by broad Table 3.1 outlay share, normalized over displayed broad-category rows to handle source rounding; not legal dedication or program tracing.\"}}",
            json_string(&format!("income-tax-outlay-model:{}:{}", record.fiscal_year, record.category_key)),
            json_string(MODEL_ID),
            record.fiscal_year,
            SOURCE_IDS.iter().map(|source| json_string(source)).collect::<Vec<_>>().join(","),
            json_string(&format!("OMB Historical Table 1.1 FY2027 row {}", record.table_11_row)),
            json_string(&format!("OMB Historical Table 2.1 FY2027 row {}, column B", record.table_21_row)),
            json_string(&format!("OMB Historical Table 3.1 FY2027 row {}", record.table_31_row)),
            json_string(record.category_key),
            json_string(record.category_label),
            decimal_string(record.category_outlays_amount, 6),
            decimal_string(record.total_outlays_amount, 6),
            decimal_string(record.category_total_outlays_amount, 6),
            decimal_string(record.individual_income_tax_receipts_amount, 6),
            decimal_string(record.outlay_share_percent, 9),
            decimal_string(record.allocation_share_percent, 9),
            decimal_string(record.modeled_income_tax_allocation_amount, 6),
            decimal_string(record.total_receipts_amount, 6),
            decimal_string(record.surplus_or_deficit_amount, 6),
            annual_deficit_gap_string(record.deficit_gap_amount),
            decimal_string(record.borrowed_share_percent_of_outlays, 9),
            decimal_string(record.income_tax_coverage_percent_of_outlays, 9),
            decimal_string(record.category_total_reconciliation_difference_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

fn source_profile_markdown(profile: &AnnualProfile) -> String {
    let sample_years = [1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{MODEL_ID}`"),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Source Roles".to_string(),
        String::new(),
        "| Source ID | Use |".to_string(),
        "|---|---|".to_string(),
        "| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit. |"
            .to_string(),
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipts. |".to_string(),
        "| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays. |"
            .to_string(),
        String::new(),
        "## Broad Categories".to_string(),
        String::new(),
        "| Category key | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "All amounts are in millions of dollars. `Modeled sum` is the sum of".to_string(),
        "the six category allocation rows for the fiscal year.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 outlays | Category total | Income tax receipts | Modeled sum | Deficit gap |".to_string(),
        "|---:|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for row in profile
        .annual_checks
        .iter()
        .filter(|row| sample_years.contains(&row.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.year,
            comma_number(row.table_1_1_outlays, 0),
            comma_number(row.table_3_1_outlays, 0),
            comma_number(row.category_total, 0),
            comma_number(row.income_tax, 0),
            comma_number(row.modeled_sum, 3),
            comma_number(row.deficit_gap, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "These rows allocate individual income-tax receipts by reported outlay".to_string(),
        "share, normalized over the displayed broad-category rows when source".to_string(),
        "rounding creates a small difference from the displayed total. They do".to_string(),
        "not claim that income-tax dollars were legally dedicated to the listed".to_string(),
        "outlay categories.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
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
        return Err(format!("stale {label}"));
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
    row.insert(field.to_string(), compact_decimal(value));
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
        compact_decimal(number)
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

fn compact_decimal(value: f64) -> String {
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

fn comma_number(value: f64, decimals: usize) -> String {
    let text = format!("{value:.decimals$}");
    let (sign, unsigned) = text
        .strip_prefix('-')
        .map_or(("", text.as_str()), |rest| ("-", rest));
    let (integer, fraction) = unsigned
        .split_once('.')
        .map_or((unsigned, None), |(integer, fraction)| {
            (integer, Some(fraction))
        });
    let mut grouped = String::new();
    for (index, char) in integer.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(char);
    }
    let integer = grouped.chars().rev().collect::<String>();
    match fraction {
        Some(fraction) => format!("{sign}{integer}.{fraction}"),
        None => format!("{sign}{integer}"),
    }
}

fn annual_deficit_gap_string(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else {
        decimal_string(value, 6)
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
        "1. `cargo run -p taxlane-tools -- income-tax-outlay model`".to_string(),
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
