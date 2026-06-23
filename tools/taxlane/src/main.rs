use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use roxmltree::Document;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use taxlane_core::{
    AccountabilityEvidenceRecord, ArtifactMetadata, PerformanceDemandChecklistRecord,
};
use zip::ZipArchive;

const CHART_SPECS: &[&str] = &[
    "docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json",
    "docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json",
    "docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json",
    "docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json",
    "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json",
    "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json",
];

const MANIFEST_PATH: &str = "data/derived/income_tax_outlay_model/MANIFEST.md";
const ANNUAL_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl";
const DECADE_JSONL_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl";
const ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv";
const DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv";
const DECADE_MD_PATH: &str = "data/derived/income_tax_outlay_model/decade-summary.md";
const SOURCE_PROFILE_PATH: &str = "data/derived/income_tax_outlay_model/source-profile.md";
const SUBFUNCTION_MODEL_JSONL_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl";
const SUBFUNCTION_ANNUAL_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv";
const SUBFUNCTION_DECADE_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv";
const SUBFUNCTION_FY2025_TOP_CSV_PATH: &str = "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv";
const SUBFUNCTION_MODEL_PROFILE_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/source-profile.md";
const SUBFUNCTION_MODEL_README_PATH: &str =
    "data/derived/income_tax_outlay_subfunction_model/README.md";
const PLACEHOLDER_RECEIPT_JSON_PATH: &str = "data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json";
const PLACEHOLDER_RECEIPT_LANE_BARS_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json";
const PLACEHOLDER_RECEIPT_FINANCING_CONTEXT_SPEC_PATH: &str =
    "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json";
const ACCOUNTABILITY_EVIDENCE_JSONL_PATH: &str = "data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl";
const ACCOUNTABILITY_READINESS_REPORT_PATH: &str =
    "data/derived/accountability_evidence/readiness-report.md";
const ACCOUNTABILITY_ACTION_QUEUE_PATH: &str =
    "data/derived/accountability_evidence/action-queue.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-packet.md";
const ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH: &str =
    "data/derived/accountability_evidence/accountability-work-items.jsonl";
const ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH: &str =
    "data/derived/accountability_evidence/claim-guard-report.md";
const ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH: &str =
    "data/derived/accountability_evidence/public-questions.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.jsonl";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-claim-gates.json";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-dashboard.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-brief.md";
const ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH: &str =
    "data/derived/accountability_evidence/performance-demand-checklist.schema.md";
const ACCOUNTABILITY_ARTIFACT_MAP_PATH: &str =
    "data/derived/accountability_evidence/artifact-map.md";
const ACCOUNTABILITY_PUBLIC_BRIEF_PATH: &str = "docs/reading/accountability-public-brief.md";
const README_PATH: &str = "README.md";
const READING_INDEX_PATH: &str = "docs/reading/README.md";
const SOURCE_VERSION_LEDGER_PATH: &str = "docs/sources/source-version-ledger.md";
const OBSERVED_DATE: &str = "2026-06-21";
const MODEL_ID: &str = "individual-income-tax-proportional-outlays-v1";
const SUBFUNCTION_MODEL_ID: &str = "individual-income-tax-proportional-subfunction-outlays-v1";
const TABLE_1_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx";
const TABLE_2_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx";
const TABLE_2_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx";
const TABLE_3_1_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx";
const TABLE_3_2_PATH: &str = "data/raw/omb/SRC-OMB-HIST-3-2-FY2027/2026-06-21/hist03z2_fy2027.xlsx";
const RECEIPT_SHARE_JSONL_PATH: &str =
    "data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-2-FY2027.2026-06-21.draft.jsonl";
const RECEIPT_SHARE_PROFILE_PATH: &str = "data/extracted/receipt_source/table-2-2-profile.md";
const OUTLAY_FUNCTION_3_1_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl";
const OUTLAY_FUNCTION_3_1_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-1-profile.md";
const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH: &str = "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.national-defense.draft.jsonl";
const OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-national-defense-profile.md";
const OUTLAY_FUNCTION_3_2_JSONL_PATH: &str =
    "data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.draft.jsonl";
const OUTLAY_FUNCTION_3_2_PROFILE_PATH: &str =
    "data/extracted/outlay_function/table-3-2-profile.md";
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

const SUBFUNCTION_ANNUAL_HEADERS: &[&str] = &[
    "fiscal_year",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "individual_income_tax_receipts_millions",
    "total_outlays_millions",
    "subfunction_outlays_millions",
    "modeled_income_tax_allocation_millions",
    "allocation_share_percent",
    "outlay_share_percent",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
];

const SUBFUNCTION_TOP_HEADERS: &[&str] = &[
    "rank",
    "fiscal_year",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "modeled_income_tax_allocation_millions",
    "allocation_share_percent",
    "subfunction_outlays_millions",
    "allocation_method",
    "legal_allocation_status",
];

const SUBFUNCTION_DECADE_HEADERS: &[&str] = &[
    "decade",
    "start_fiscal_year",
    "end_fiscal_year",
    "year_count",
    "coverage_note",
    "function_code",
    "function_label",
    "subfunction_code",
    "subfunction_label",
    "cumulative_individual_income_tax_receipts_millions",
    "cumulative_subfunction_outlays_millions",
    "cumulative_modeled_income_tax_allocation_millions",
    "decade_allocation_share_percent",
    "allocation_method",
    "legal_allocation_status",
    "actual_or_projection",
];

#[derive(Clone, Copy)]
struct Artifact {
    path: &'static str,
    role: &'static str,
    grain: &'static str,
    kind: &'static str,
    canonical: &'static str,
}

impl Artifact {
    fn metadata(&self) -> ArtifactMetadata<'_> {
        ArtifactMetadata {
            path: self.path,
            role: self.role,
            grain: self.grain,
            kind: self.kind,
            canonical: self.canonical,
        }
    }
}

const ARTIFACTS: &[Artifact] = &[
    Artifact {
        path: "README.md",
        role: "Repository overview",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
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
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl",
        role: "Canonical annual subfunction model rows",
        grain: "fiscal year by Table 3.2 subfunction",
        kind: "jsonl",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv",
        role: "Chart-ready annual subfunction long view",
        grain: "fiscal year by Table 3.2 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv",
        role: "Chart-ready decade subfunction long view",
        grain: "decade by Table 3.2 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv",
        role: "Chart-ready FY2025 top subfunction view",
        grain: "ranked FY2025 subfunction",
        kind: "csv",
        canonical: "no",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/README.md",
        role: "Subfunction model method and schema note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/source-profile.md",
        role: "Subfunction source coverage and reconciliation sample",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/income_tax_outlay_subfunction_model/reconciliation-review.md",
        role: "Subfunction generated-row reconciliation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
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
        path: "data/derived/README.md",
        role: "Derived data index",
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
        path: "docs/reading/modeled-income-tax-subfunction-outlays.md",
        role: "Reader-facing subfunction drilldown packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-22-subfunction-reader-role-review.md",
        role: "Subfunction reader role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-22-subfunction-deficit-context-note.md",
        role: "Subfunction deficit context note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/README.md",
        role: "Data documentation index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/dictionary.md",
        role: "Data dictionary",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/data/accountability-evidence-schema.md",
        role: "Accountability evidence schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/research/2026-06-23-accountability-evidence-boundary.md",
        role: "Accountability evidence boundary note",
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
        path: "docs/charts/income-tax-outlay-subfunction-model/README.md",
        role: "Subfunction chart set handoff note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-model/README.md",
        role: "Broad chart set handoff note",
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
        path: "docs/charts/income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json",
        role: "FY2025 top subfunction allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json",
        role: "Selected subfunction trend chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json",
        role: "Decade top subfunction allocation chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "data/derived/taxpayer_receipt_model/taxpayer_receipt_model.placeholder-1000.fy2025.omb-fy2027-v1.draft.json",
        role: "Placeholder visibility receipt scenario",
        grain: "scenario",
        kind: "json",
        canonical: "yes",
    },
    Artifact {
        path: "data/derived/accountability_evidence/accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl",
        role: "Draft accountability evidence records",
        grain: "evidence record",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/README.md",
        role: "Accountability evidence dataset method note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/readiness-report.md",
        role: "Accountability evidence readiness report",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/action-queue.md",
        role: "Accountability evidence action queue",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-packet.md",
        role: "Accountability performance demand packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/accountability-work-items.jsonl",
        role: "Accountability machine-readable work items",
        grain: "work item",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/claim-guard-report.md",
        role: "Accountability claim guard report",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/public-questions.md",
        role: "Accountability public-safe questions",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.md",
        role: "Accountability performance demand checklist",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.jsonl",
        role: "Accountability performance demand checklist rows",
        grain: "demand checklist row",
        kind: "jsonl",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-claim-gates.json",
        role: "Accountability performance demand claim gates",
        grain: "claim gate summary",
        kind: "json",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-dashboard.md",
        role: "Accountability performance demand dashboard",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-brief.md",
        role: "Accountability performance demand brief",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/performance-demand-checklist.schema.md",
        role: "Accountability performance demand checklist schema",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "data/derived/accountability_evidence/artifact-map.md",
        role: "Accountability artifact map",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/accountability-public-brief.md",
        role: "Reader-facing accountability brief",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/README.md",
        role: "Reading packet index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/placeholder-visibility-receipt.md",
        role: "Placeholder receipt reader packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/reading/placeholder-receipt-display-packet.md",
        role: "Placeholder receipt static display packet",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-display-packet-role-review.md",
        role: "Placeholder receipt display packet role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/placeholder-receipt-placement-spec.md",
        role: "Placeholder receipt static placement spec",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/README.md",
        role: "Design handoff index",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-placement-spec-role-review.md",
        role: "Placeholder receipt placement spec role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/design/placeholder-receipt-mockup-review-checklist.md",
        role: "Placeholder receipt mockup review checklist",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-placeholder-mockup-checklist-role-review.md",
        role: "Placeholder receipt mockup checklist role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-role-review.md",
        role: "Accountability evidence role review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-source-custody-review.md",
        role: "Accountability evidence source-custody review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-validator-hardening-review.md",
        role: "Accountability validator hardening review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-readiness-classification-review.md",
        role: "Accountability readiness classification review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-readiness-report-review.md",
        role: "Accountability readiness report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-evidence-only-record-review.md",
        role: "Accountability evidence-only record review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-next-action-report-review.md",
        role: "Accountability next-action report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-action-queue-review.md",
        role: "Accountability action queue review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-packet-review.md",
        role: "Accountability performance demand packet review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-core-workflow-review.md",
        role: "Accountability core workflow review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-work-items-review.md",
        role: "Accountability work items review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-claim-guard-report-review.md",
        role: "Accountability claim guard report review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-public-questions-review.md",
        role: "Accountability public questions review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-public-brief-review.md",
        role: "Accountability public brief review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-brief-discovery-review.md",
        role: "Accountability brief discovery review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-artifact-map-review.md",
        role: "Accountability artifact map review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-checklist-review.md",
        role: "Accountability performance demand checklist review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-checklist-jsonl-review.md",
        role: "Accountability performance demand checklist JSONL review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-core-contract-review.md",
        role: "Accountability demand checklist core contract review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-jsonl-read-validation-review.md",
        role: "Accountability demand checklist JSONL read validation review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-demand-checklist-schema-review.md",
        role: "Accountability demand checklist schema review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-claim-gates-review.md",
        role: "Accountability performance demand claim gates review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-accountability-performance-demand-dashboard-review.md",
        role: "Accountability performance demand dashboard review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "reviews/2026-06-23-rust-core-crate-architecture-review.md",
        role: "Rust core crate architecture review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/MISSION.md",
        role: "VTRACE mission",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/REQUIREMENTS.md",
        role: "VTRACE requirements",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/SPECIFICATION_BASELINE.md",
        role: "VTRACE specification baseline",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/TRACE.md",
        role: "VTRACE trace matrix",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/WORK_PACKAGES.md",
        role: "VTRACE work packages",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/VERIFICATION.md",
        role: "VTRACE verification plan",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/VALIDATION.md",
        role: "VTRACE validation scenarios",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/EVIDENCE.md",
        role: "VTRACE evidence ledger",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/vtrace/REVIEW.md",
        role: "VTRACE adoption review",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/README.md",
        role: "Taxpayer receipt chart set handoff note",
        grain: "documentation",
        kind: "markdown",
        canonical: "supporting",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/placeholder-lane-bars.vl.json",
        role: "Placeholder receipt lane bar chart spec",
        grain: "visualization spec",
        kind: "json",
        canonical: "view",
    },
    Artifact {
        path: "docs/charts/taxpayer-receipt-model/placeholder-financing-context.vl.json",
        role: "Placeholder receipt financing context chart spec",
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
        path: "crates/taxlane-core/Cargo.toml",
        role: "Rust Taxlane core crate manifest",
        grain: "tooling",
        kind: "toml",
        canonical: "supporting",
    },
    Artifact {
        path: "crates/taxlane-core/src/lib.rs",
        role: "Rust Taxlane core domain library",
        grain: "library",
        kind: "rust",
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
            if area == "income-tax-outlay"
                && command == "subfunction-model"
                && flag == "--check" =>
        {
            run_subfunction_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-model" => {
            run_subfunction_model_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay"
                && command == "subfunction-export"
                && flag == "--check" =>
        {
            run_subfunction_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-export" => {
            run_subfunction_export_write()
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
        [area, command, flag]
            if area == "receipt-source" && command == "table-2-2" && flag == "--check" =>
        {
            run_table_2_2_check()
        }
        [area, command] if area == "receipt-source" && command == "table-2-2" => {
            run_table_2_2_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-1" && flag == "--check" =>
        {
            run_table_3_1_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-1" => {
            run_table_3_1_write()
        }
        [area, command, flag]
            if area == "outlay-function"
                && command == "table-3-2-national-defense"
                && flag == "--check" =>
        {
            run_table_3_2_national_defense_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2-national-defense" => {
            run_table_3_2_national_defense_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-2" && flag == "--check" =>
        {
            run_table_3_2_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2" => {
            run_table_3_2_write()
        }
        _ => {
            eprintln!(
                "usage: taxlane-tools income-tax-outlay <validate|model [--check]|subfunction-model [--check]|subfunction-export [--check]|summary [--check]|export [--check]|manifest [--check]>\n       taxlane-tools receipt-source table-2-2 [--check]\n       taxlane-tools outlay-function table-3-1 [--check]\n       taxlane-tools outlay-function table-3-2-national-defense [--check]\n       taxlane-tools outlay-function table-3-2 [--check]"
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

    if let Err(err) = export_subfunction_chart_views(&root, true) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_manifest(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = validate_accountability_evidence_records(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_readiness_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_action_queue(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_packet(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_work_items(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_claim_guard_report(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_questions(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_public_brief_discovery(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_artifact_map(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_checklist_jsonl(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_claim_gates(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_dashboard(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
    }

    if let Err(err) = check_accountability_performance_demand_brief(&root) {
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

    if let Err(err) = validate_placeholder_receipt_chart_sync(&root) {
        eprintln!("{err}");
        return ExitCode::from(1);
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

fn run_subfunction_model_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_model_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_subfunction_model(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_export_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_subfunction_export_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match export_subfunction_chart_views(&root, false) {
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

fn run_table_3_1_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_1_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_1(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_national_defense_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_national_defense_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2_national_defense(&root, false) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_check() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, true) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run_table_3_2_write() -> ExitCode {
    let root = match repo_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    match build_outlay_function_table_3_2(&root, false) {
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

fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let file = File::open(path).map_err(|err| format!("failed to open {:?}: {err}", path))?;
    serde_json::from_reader::<_, serde_json::Value>(file)
        .map_err(|err| format!("failed to parse {:?}: {err}", path))
}

fn validate_placeholder_receipt_chart_sync(root: &Path) -> Result<(), String> {
    let receipt = read_json(&root.join(PLACEHOLDER_RECEIPT_JSON_PATH))?;
    let lane_spec = read_json(&root.join(PLACEHOLDER_RECEIPT_LANE_BARS_SPEC_PATH))?;
    let context_spec = read_json(&root.join(PLACEHOLDER_RECEIPT_FINANCING_CONTEXT_SPEC_PATH))?;

    let receipt_lanes = receipt
        .get("lane_allocations")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder receipt missing lane_allocations".to_string())?;
    let chart_lanes = lane_spec
        .pointer("/data/values")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder lane chart missing data.values".to_string())?;

    if receipt_lanes.len() != chart_lanes.len() {
        return Err(format!(
            "placeholder lane chart has {} rows but receipt has {} rows",
            chart_lanes.len(),
            receipt_lanes.len()
        ));
    }

    let mut chart_by_label = BTreeMap::new();
    for row in chart_lanes {
        let lane = row
            .get("lane")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder lane chart row missing lane".to_string())?;
        chart_by_label.insert(lane.to_string(), row);
    }

    for lane in receipt_lanes {
        let label = lane
            .get("public_label")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder receipt lane missing public_label".to_string())?;
        let chart = chart_by_label
            .get(label)
            .ok_or_else(|| format!("placeholder lane chart missing lane {label}"))?;
        assert_number_close(
            chart,
            "amount",
            number_field(lane, "placeholder_allocation_amount_rounded_usd")?,
            0.000001,
            &format!("placeholder lane chart amount for {label}"),
        )?;
        assert_number_close(
            chart,
            "share",
            number_field(lane, "allocation_share_percent")?,
            0.000001,
            &format!("placeholder lane chart share for {label}"),
        )?;
        let expected_treatment = chart_treatment_for_lane(lane)?;
        let actual_treatment = chart
            .get("treatment")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("placeholder lane chart missing treatment for {label}"))?;
        if actual_treatment != expected_treatment {
            return Err(format!(
                "placeholder lane chart treatment for {label}: expected {expected_treatment:?}, found {actual_treatment:?}"
            ));
        }
    }

    let context_rows = context_spec
        .pointer("/data/values")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "placeholder financing context chart missing data.values".to_string())?;
    let mut context_by_measure = BTreeMap::new();
    for row in context_rows {
        let measure = row
            .get("measure")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "placeholder financing context row missing measure".to_string())?;
        context_by_measure.insert(measure.to_string(), row);
    }
    let borrowed = context_by_measure
        .get("Borrowed share of outlays")
        .ok_or_else(|| "placeholder financing context missing borrowed share".to_string())?;
    assert_number_close(
        borrowed,
        "percent",
        number_field(&receipt, "borrowed_share_percent_of_outlays")?,
        0.000001,
        "placeholder financing context borrowed share",
    )?;
    let coverage = context_by_measure
        .get("Individual income-tax coverage of outlays")
        .ok_or_else(|| "placeholder financing context missing income-tax coverage".to_string())?;
    assert_number_close(
        coverage,
        "percent",
        number_field(&receipt, "income_tax_coverage_percent_of_outlays")?,
        0.000001,
        "placeholder financing context income-tax coverage",
    )?;

    println!("validated placeholder receipt chart sync");
    Ok(())
}

fn chart_treatment_for_lane(lane: &serde_json::Value) -> Result<&'static str, String> {
    match string_field(lane, "display_treatment")?.as_str() {
        "modeled_lane" => Ok("Modeled lane"),
        "dedicated_financing_caveat_required" => Ok("Dedicated-financing caveat"),
        "display_separately" => match string_field(lane, "spending_control")?.as_str() {
            "net-interest" => Ok("Financing cost"),
            "offsetting" => Ok("Offset"),
            other => Err(format!(
                "unknown display_separately spending_control {other:?}"
            )),
        },
        "negative_or_offset_sensitive_lane" => Ok("Offset-sensitive adjustment"),
        other => Err(format!("unknown display_treatment {other:?}")),
    }
}

fn assert_number_close(
    row: &serde_json::Value,
    field: &str,
    expected: f64,
    tolerance: f64,
    label: &str,
) -> Result<(), String> {
    let actual = number_field(row, field)?;
    if (actual - expected).abs() > tolerance {
        return Err(format!("{label}: expected {expected}, found {actual}"));
    }
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

fn build_outlay_function_table_3_1(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_outlay_function_3_1_rows(root)?;
    validate_outlay_function_3_1_rows(&rows, &profile)?;
    let jsonl = outlay_function_3_1_jsonl(&rows);
    let markdown = outlay_function_3_1_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_JSONL_PATH,
            &jsonl,
            "Table 3.1 outlay function JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_1_PROFILE_PATH,
            &markdown,
            "Table 3.1 outlay function profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_1_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_1_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_1_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.1 outlay function rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_outlay_function_table_3_2_national_defense(
    root: &Path,
    check_only: bool,
) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_national_defense_rows(root)?;
    validate_table_3_2_national_defense_rows(&rows, &profile)?;
    let jsonl = table_3_2_national_defense_jsonl(&rows);
    let markdown = table_3_2_national_defense_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH,
            &jsonl,
            "Table 3.2 National Defense JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH,
            &markdown,
            "Table 3.2 National Defense profile",
        )?;
    } else {
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH),
            jsonl,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_JSONL_PATH}: {err}")
        })?;
        fs::write(
            root.join(OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH),
            markdown,
        )
        .map_err(|err| {
            format!("failed to write {OUTLAY_FUNCTION_3_2_NATIONAL_DEFENSE_PROFILE_PATH}: {err}")
        })?;
    }

    println!(
        "validated {} Table 3.2 National Defense rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_outlay_function_table_3_2(root: &Path, check_only: bool) -> Result<(), String> {
    let (rows, profile) = build_table_3_2_rows(root)?;
    validate_table_3_2_rows(&profile)?;
    let jsonl = table_3_2_jsonl(&rows);
    let markdown = table_3_2_profile_markdown(&profile);

    if check_only {
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_JSONL_PATH,
            &jsonl,
            "Table 3.2 JSONL",
        )?;
        compare_text(
            root,
            OUTLAY_FUNCTION_3_2_PROFILE_PATH,
            &markdown,
            "Table 3.2 profile",
        )?;
    } else {
        fs::write(root.join(OUTLAY_FUNCTION_3_2_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_JSONL_PATH}: {err}"))?;
        fs::write(root.join(OUTLAY_FUNCTION_3_2_PROFILE_PATH), markdown)
            .map_err(|err| format!("failed to write {OUTLAY_FUNCTION_3_2_PROFILE_PATH}: {err}"))?;
    }

    println!(
        "validated {} Table 3.2 rows for {}-{}",
        rows.len(),
        profile.first_year,
        profile.last_year
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

#[derive(Clone)]
struct OutlayFunctionRow {
    fiscal_year: i64,
    source_column: String,
    function_code: String,
    function_label: String,
    source_row: i64,
    amount: f64,
    actual_or_projection: &'static str,
    offsetting_treatment: &'static str,
    notes: &'static str,
    include_table_1_1_source: bool,
    table_1_1_row: Option<i64>,
}

struct OutlayFunctionCheck {
    year: i64,
    table_1_1_outlays: f64,
    table_3_1_total: f64,
    broad_category_total: f64,
    total_difference: f64,
    broad_category_difference: f64,
}

struct OutlayFunctionProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    checks: Vec<OutlayFunctionCheck>,
}

#[derive(Clone, Copy)]
struct Table32NationalDefenseLine {
    source_row: i64,
    subfunction_code: Option<&'static str>,
    subfunction_label: Option<&'static str>,
    source_label: &'static str,
    notes: &'static str,
}

const TABLE_3_2_NATIONAL_DEFENSE_LINES: &[Table32NationalDefenseLine] = &[
    Table32NationalDefenseLine {
        source_row: 13,
        subfunction_code: Some("051"),
        subfunction_label: Some("Department of Defense-Military"),
        source_label: "051 Subtotal, Department of Defense-Military",
        notes: "Subfunction total; lower component rows under 051 are not emitted in this proof slice.",
    },
    Table32NationalDefenseLine {
        source_row: 14,
        subfunction_code: Some("053"),
        subfunction_label: Some("Atomic energy defense activities"),
        source_label: "053 Atomic energy defense activities",
        notes: "National Defense subfunction total from Table 3.2.",
    },
    Table32NationalDefenseLine {
        source_row: 15,
        subfunction_code: Some("054"),
        subfunction_label: Some("Defense-related activities"),
        source_label: "054 Defense-related activities",
        notes: "National Defense subfunction total from Table 3.2.",
    },
    Table32NationalDefenseLine {
        source_row: 16,
        subfunction_code: None,
        subfunction_label: None,
        source_label: "Total, National Defense",
        notes: "Parent function total reconciled to OMB Historical Table 3.1 National Defense.",
    },
];

#[derive(Clone)]
struct Table32OutlayFunctionRow {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: &'static str,
    function_label: &'static str,
    subfunction_code: Option<&'static str>,
    subfunction_label: Option<&'static str>,
    source_label: &'static str,
    amount: f64,
    notes: &'static str,
}

struct Table32NationalDefenseCheck {
    year: i64,
    table_3_1_national_defense: f64,
    table_3_2_national_defense: f64,
    subfunction_total: f64,
    table_3_1_difference: f64,
    subfunction_difference: f64,
}

struct Table32NationalDefenseProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    checks: Vec<Table32NationalDefenseCheck>,
}

#[derive(Clone)]
enum Table32LineKind {
    Subfunction,
    FunctionTotal,
    GrandTotal,
}

#[derive(Clone)]
struct Table32Line {
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: Option<String>,
    subfunction_label: Option<String>,
    source_label: String,
    kind: Table32LineKind,
}

#[derive(Clone)]
struct Table32Row {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: Option<String>,
    subfunction_label: Option<String>,
    source_label: String,
    amount: f64,
    kind: Table32LineKind,
}

struct Table32FunctionCheck {
    year: i64,
    function_code: String,
    function_label: String,
    function_total: f64,
    subfunction_total: f64,
    difference: f64,
}

struct Table32GrandCheck {
    year: i64,
    table_3_1_total_outlays: f64,
    table_3_2_total_outlays: f64,
    function_total_sum: f64,
    table_3_1_difference: f64,
    function_total_difference: f64,
}

struct Table32Profile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    line_count: usize,
    subfunction_line_count: usize,
    function_total_line_count: usize,
    function_count: usize,
    grand_checks: Vec<Table32GrandCheck>,
    function_checks: Vec<Table32FunctionCheck>,
}

#[derive(Clone)]
struct SubfunctionModelRow {
    fiscal_year: i64,
    source_column: String,
    source_row: i64,
    function_code: String,
    function_label: String,
    subfunction_code: String,
    subfunction_label: String,
    subfunction_outlays_amount: f64,
    subfunction_total_outlays_amount: f64,
    total_outlays_amount: f64,
    individual_income_tax_receipts_amount: f64,
    outlay_share_percent: f64,
    allocation_share_percent: f64,
    modeled_income_tax_allocation_amount: f64,
}

struct SubfunctionModelCheck {
    year: i64,
    table_3_2_total_outlays: f64,
    subfunction_total: f64,
    individual_income_tax: f64,
    modeled_sum: f64,
    subfunction_total_difference: f64,
}

struct SubfunctionModelProfile {
    first_year: i64,
    last_year: i64,
    year_count: usize,
    record_count: usize,
    subfunction_count: usize,
    checks: Vec<SubfunctionModelCheck>,
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

fn build_outlay_function_3_1_rows(
    root: &Path,
) -> Result<(Vec<OutlayFunctionRow>, OutlayFunctionProfile), String> {
    let t11 = parse_table_1_1(&read_sheet(&root.join(TABLE_1_1_PATH))?);
    let sheet = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (years_31, t31) = parse_table_3_1(&sheet)?;
    let columns_by_year = table_3_1_year_columns(&sheet)?;
    let years: Vec<i64> = years_31
        .into_iter()
        .filter(|year| (1940..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(table_11) = t11.get(year) else {
            errors.push(format!("{year}: missing Table 1.1 row"));
            continue;
        };
        let Some(source_column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.1 source column"));
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

        let broad_category_total: f64 = BROAD_CATEGORIES
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
        let total_difference = total_outlays_31 - table_11.total_outlays;
        let broad_category_difference = broad_category_total - total_outlays_31;
        if total_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.1 total {total_outlays_31} does not reconcile to Table 1.1 total {}",
                table_11.total_outlays
            ));
        }
        if broad_category_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.1 broad category total {broad_category_total} does not reconcile to total {total_outlays_31}"
            ));
        }

        for (key, label, source_row) in BROAD_CATEGORIES {
            let amount = t31
                .get(*key)
                .and_then(|values| values.get(year))
                .copied()
                .ok_or_else(|| format!("{year}: missing Table 3.1 category {key}"))?;
            rows.push(OutlayFunctionRow {
                fiscal_year: *year,
                source_column: source_column.clone(),
                function_code: (*key).to_string(),
                function_label: (*label).to_string(),
                source_row: *source_row,
                amount: round6(amount),
                actual_or_projection: "actual",
                offsetting_treatment: if *key == "undistributed-offsetting-receipts" {
                    "undistributed-offsetting-receipts"
                } else {
                    "net"
                },
                notes: outlay_function_notes(key),
                include_table_1_1_source: false,
                table_1_1_row: None,
            });
        }
        rows.push(OutlayFunctionRow {
            fiscal_year: *year,
            source_column: source_column.clone(),
            function_code: "total-federal-outlays".to_string(),
            function_label: "Total, Federal outlays".to_string(),
            source_row: 35,
            amount: round6(total_outlays_31),
            actual_or_projection: "actual",
            offsetting_treatment: "net",
            notes: "Total federal outlays reconciled to OMB Historical Table 1.1 total outlays within displayed precision.",
            include_table_1_1_source: true,
            table_1_1_row: Some(table_11.row),
        });

        checks.push(OutlayFunctionCheck {
            year: *year,
            table_1_1_outlays: table_11.total_outlays,
            table_3_1_total: total_outlays_31,
            broad_category_total,
            total_difference,
            broad_category_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.1 years".to_string())?;
    let profile = OutlayFunctionProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

fn validate_outlay_function_3_1_rows(
    rows: &[OutlayFunctionRow],
    profile: &OutlayFunctionProfile,
) -> Result<(), String> {
    let expected_rows = profile.year_count * (BROAD_CATEGORIES.len() + 1);
    if rows.len() != expected_rows {
        return Err(format!(
            "expected {expected_rows} Table 3.1 outlay function rows, found {}",
            rows.len()
        ));
    }
    for check in &profile.checks {
        if check.total_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.1/Table 1.1 total difference {}",
                check.year, check.total_difference
            ));
        }
        if check.broad_category_difference.abs() > 2.0 {
            return Err(format!(
                "{}: broad category total difference {}",
                check.year, check.broad_category_difference
            ));
        }
    }
    Ok(())
}

fn outlay_function_notes(key: &str) -> &'static str {
    match key {
        "net-interest" => "Net interest is kept visible as its own outlay function.",
        "undistributed-offsetting-receipts" => {
            "Undistributed offsetting receipts are kept visible and negative as reported by OMB."
        }
        _ => "Broad Table 3.1 outlay function; no lane allocation applied yet.",
    }
}

fn outlay_function_3_1_jsonl(rows: &[OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let source_ids = if row.include_table_1_1_source {
            "\"SRC-OMB-HIST-3-1-FY2027\",\"SRC-OMB-HIST-1-1-FY2027\""
        } else {
            "\"SRC-OMB-HIST-3-1-FY2027\""
        };
        let reconciliation = row.table_1_1_row.map_or_else(String::new, |table_1_1_row| {
            format!("; reconciled to Table 1.1 row {table_1_1_row}")
        });
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[{}],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":null,\"subfunction_label\":null,\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":{},\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:total:outlays",
                row.fiscal_year, row.function_code
            )),
            row.fiscal_year,
            source_ids,
            json_string("OMB Historical Table 3.1 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}{}",
                row.source_row,
                row.source_column,
                row.source_row,
                row.function_label,
                reconciliation
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_amount(row.amount),
            json_string(row.actual_or_projection),
            json_string(row.offsetting_treatment),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn outlay_function_3_1_profile_markdown(profile: &OutlayFunctionProfile) -> String {
    let sample_years = [1940, 1950, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.1 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-1-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; FY2026-FY2031 are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | OMB label | Table 3.1 row |".to_string(),
        "|---|---|---:|".to_string(),
    ];
    for (key, label, row_num) in BROAD_CATEGORIES {
        lines.push(format!("| `{key}` | {label} | {row_num} |"));
    }
    lines.push("| `total-federal-outlays` | Total, Federal outlays | 35 |".to_string());
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Broad category total is the sum of the six visible Table 3.1 rows above.".to_string(),
        String::new(),
        "| Fiscal year | Table 1.1 outlays | Table 3.1 total | Broad category total | Table total diff | Broad category diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_1_1_outlays, 0),
            comma_number(check.table_3_1_total, 0),
            comma_number(check.broad_category_total, 0),
            comma_number(check.total_difference, 0),
            comma_number(check.broad_category_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Net interest is extracted as its own visible outlay function.".to_string(),
        "- Undistributed offsetting receipts are extracted as negative amounts with `offsetting_treatment = \"undistributed-offsetting-receipts\"`.".to_string(),
        "- Function codes are TAXLANE slugs because Table 3.1 uses labels, not OMB numeric function codes.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_table_3_2_national_defense_rows(
    root: &Path,
) -> Result<(Vec<Table32OutlayFunctionRow>, Table32NationalDefenseProfile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    validate_table_3_2_national_defense_labels(&sheet_32)?;

    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();
    let mut rows = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_national_defense) = t31
            .get("national-defense")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 National Defense"));
            continue;
        };

        let mut subfunction_total = 0.0;
        let mut parent_total = None;
        for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
            let amount = table_3_2_number(&sheet_32, line.source_row, column)?;
            if line.subfunction_code.is_some() {
                subfunction_total += amount;
            } else {
                parent_total = Some(amount);
            }
            rows.push(Table32OutlayFunctionRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: "050",
                function_label: "National Defense",
                subfunction_code: line.subfunction_code,
                subfunction_label: line.subfunction_label,
                source_label: line.source_label,
                amount: round6(amount),
                notes: line.notes,
            });
        }

        let Some(table_3_2_national_defense) = parent_total else {
            errors.push(format!("{year}: missing Table 3.2 National Defense total"));
            continue;
        };
        let table_3_1_difference = table_3_2_national_defense - table_3_1_national_defense;
        let subfunction_difference = subfunction_total - table_3_2_national_defense;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense {table_3_2_national_defense} does not reconcile to Table 3.1 {table_3_1_national_defense}"
            ));
        }
        if subfunction_difference.abs() > 2.0 {
            errors.push(format!(
                "{year}: Table 3.2 National Defense subfunctions {subfunction_total} do not reconcile to total {table_3_2_national_defense}"
            ));
        }
        checks.push(Table32NationalDefenseCheck {
            year: *year,
            table_3_1_national_defense,
            table_3_2_national_defense,
            subfunction_total,
            table_3_1_difference,
            subfunction_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 National Defense years".to_string())?;
    let profile = Table32NationalDefenseProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        checks,
    };
    Ok((rows, profile))
}

fn validate_table_3_2_national_defense_rows(
    rows: &[Table32OutlayFunctionRow],
    profile: &Table32NationalDefenseProfile,
) -> Result<(), String> {
    let expected_rows = profile.year_count * TABLE_3_2_NATIONAL_DEFENSE_LINES.len();
    if rows.len() != expected_rows {
        return Err(format!(
            "expected {expected_rows} Table 3.2 National Defense rows, found {}",
            rows.len()
        ));
    }
    for check in &profile.checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 National Defense difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.subfunction_difference.abs() > 2.0 {
            return Err(format!(
                "{}: National Defense subfunction difference {}",
                check.year, check.subfunction_difference
            ));
        }
    }
    Ok(())
}

fn validate_table_3_2_national_defense_labels(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<(), String> {
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        let label = sheet
            .get(&line.source_row)
            .and_then(|row| text_cell(row.get("A")))
            .ok_or_else(|| format!("missing Table 3.2 row {} label", line.source_row))?;
        if label != line.source_label {
            return Err(format!(
                "Unexpected Table 3.2 row {}: {label:?}",
                line.source_row
            ));
        }
    }
    Ok(())
}

fn table_3_2_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Result<f64, String> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
        .ok_or_else(|| format!("missing Table 3.2 amount at {column}{row_num}"))
}

fn table_3_2_national_defense_jsonl(rows: &[Table32OutlayFunctionRow]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":\"net\",\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(row.function_code),
            json_string(row.function_label),
            json_option_string(row.subfunction_code),
            json_option_string(row.subfunction_label),
            json_amount(row.amount),
            json_string(OBSERVED_DATE),
            json_string(row.notes),
        ));
    }
    lines.join("\n") + "\n"
}

fn table_3_2_national_defense_profile_markdown(profile: &Table32NationalDefenseProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 National Defense Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Extracted Rows".to_string(),
        String::new(),
        "| Function code | Subfunction code | Source label | Table 3.2 row |".to_string(),
        "|---|---|---|---:|".to_string(),
    ];
    for line in TABLE_3_2_NATIONAL_DEFENSE_LINES {
        lines.push(format!(
            "| `050` | {} | {} | {} |",
            line.subfunction_code
                .map(|code| format!("`{code}`"))
                .unwrap_or_else(|| "`null`".to_string()),
            line.source_label,
            line.source_row
        ));
    }
    lines.extend([
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Subfunction total is rows 13, 14, and 15.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 National Defense | Table 3.2 National Defense | Subfunction total | Table 3.1 diff | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ]);
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_national_defense, 0),
            comma_number(check.table_3_2_national_defense, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.subfunction_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- This is a proof slice for function `050 National Defense`, not the full Table 3.2 extraction.".to_string(),
        "- Rows 6-12 are lower component rows inside subfunction `051`; this proof emits row 13 as the subfunction total instead.".to_string(),
        "- Parent total row 16 is emitted with `subfunction_code = null` so it can reconcile to Table 3.1.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_table_3_2_rows(root: &Path) -> Result<(Vec<Table32Row>, Table32Profile), String> {
    let sheet_31 = read_sheet(&root.join(TABLE_3_1_PATH))?;
    let (_, t31) = parse_table_3_1(&sheet_31)?;
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut grand_checks = Vec::new();
    let mut function_checks = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_3_1_total_outlays) = t31
            .get("total-federal-outlays")
            .and_then(|values| values.get(year))
            .copied()
        else {
            errors.push(format!("{year}: missing Table 3.1 total outlays"));
            continue;
        };

        let mut subfunction_totals: BTreeMap<String, f64> = BTreeMap::new();
        let mut explicit_function_totals: BTreeMap<String, (String, f64)> = BTreeMap::new();
        let mut table_3_2_total_outlays = None;

        for line in &lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            match line.kind {
                Table32LineKind::Subfunction => {
                    *subfunction_totals
                        .entry(line.function_code.clone())
                        .or_insert(0.0) += amount;
                }
                Table32LineKind::FunctionTotal => {
                    explicit_function_totals.insert(
                        line.function_code.clone(),
                        (line.function_label.clone(), amount),
                    );
                }
                Table32LineKind::GrandTotal => {
                    table_3_2_total_outlays = Some(amount);
                }
            }
            rows.push(Table32Row {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line.subfunction_code.clone(),
                subfunction_label: line.subfunction_label.clone(),
                source_label: line.source_label.clone(),
                amount: round6(amount),
                kind: line.kind.clone(),
            });
        }

        let Some(table_3_2_total_outlays) = table_3_2_total_outlays else {
            errors.push(format!("{year}: missing Table 3.2 total outlays"));
            continue;
        };
        let mut function_total_sum = 0.0;
        for (function_code, subfunction_total) in &subfunction_totals {
            if let Some((function_label, function_total)) =
                explicit_function_totals.get(function_code)
            {
                let difference = subfunction_total - function_total;
                if difference.abs() > 2.0 {
                    errors.push(format!(
                        "{year}: Table 3.2 function {function_code} subfunctions {subfunction_total} do not reconcile to total {function_total}"
                    ));
                }
                function_total_sum += function_total;
                function_checks.push(Table32FunctionCheck {
                    year: *year,
                    function_code: function_code.clone(),
                    function_label: function_label.clone(),
                    function_total: *function_total,
                    subfunction_total: *subfunction_total,
                    difference,
                });
            } else {
                function_total_sum += subfunction_total;
            }
        }
        let table_3_1_difference = table_3_2_total_outlays - table_3_1_total_outlays;
        let function_total_difference = function_total_sum - table_3_2_total_outlays;
        if table_3_1_difference.abs() > 0.5 {
            errors.push(format!(
                "{year}: Table 3.2 total {table_3_2_total_outlays} does not reconcile to Table 3.1 total {table_3_1_total_outlays}"
            ));
        }
        if function_total_difference.abs() > 5.0 {
            errors.push(format!(
                "{year}: Table 3.2 function totals {function_total_sum} do not reconcile to total outlays {table_3_2_total_outlays}"
            ));
        }
        grand_checks.push(Table32GrandCheck {
            year: *year,
            table_3_1_total_outlays,
            table_3_2_total_outlays,
            function_total_sum,
            table_3_1_difference,
            function_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no Table 3.2 years".to_string())?;
    let subfunction_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .count();
    let function_total_line_count = lines
        .iter()
        .filter(|line| matches!(line.kind, Table32LineKind::FunctionTotal))
        .count();
    let function_count = lines
        .iter()
        .filter(|line| !matches!(line.kind, Table32LineKind::GrandTotal))
        .map(|line| line.function_code.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let profile = Table32Profile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: rows.len(),
        line_count: lines.len(),
        subfunction_line_count,
        function_total_line_count,
        function_count,
        grand_checks,
        function_checks,
    };
    Ok((rows, profile))
}

fn parse_table_3_2_lines(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<Vec<Table32Line>, String> {
    let mut lines = Vec::new();
    let mut current_function: Option<(String, String)> = None;
    for (row_num, cells) in sheet {
        if *row_num < 4 {
            continue;
        }
        let Some(label) = text_cell(cells.get("A")) else {
            continue;
        };
        if let Some((code, function_label)) = parse_table_3_2_function_header(&label) {
            if is_table_3_2_function_code(&code) {
                current_function = Some((code, function_label));
            }
            continue;
        }
        if label.starts_with('(')
            || label == "On-budget unless otherwise stated"
            || label == "N/A = Not available"
        {
            continue;
        }
        if label == "Total outlays" {
            lines.push(Table32Line {
                source_row: *row_num,
                function_code: "total-federal-outlays".to_string(),
                function_label: "Total outlays".to_string(),
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::GrandTotal,
            });
            continue;
        }
        if let Some(total_label) = label.strip_prefix("Total, ") {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!("Table 3.2 row {row_num} total without function"));
            };
            if total_label != function_label {
                return Err(format!(
                    "Table 3.2 row {row_num} total {total_label:?} does not match current function {function_label:?}"
                ));
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: None,
                subfunction_label: None,
                source_label: label,
                kind: Table32LineKind::FunctionTotal,
            });
            continue;
        }
        if let Some((subfunction_code, mut subfunction_label)) = parse_table_3_2_coded_label(&label)
        {
            let Some((function_code, function_label)) = current_function.clone() else {
                return Err(format!(
                    "Table 3.2 row {row_num} subfunction without function"
                ));
            };
            if let Some(subtotal_label) = subfunction_label.strip_prefix("Subtotal, ") {
                subfunction_label = subtotal_label.to_string();
            }
            lines.push(Table32Line {
                source_row: *row_num,
                function_code,
                function_label,
                subfunction_code: Some(subfunction_code),
                subfunction_label: Some(subfunction_label),
                source_label: label,
                kind: Table32LineKind::Subfunction,
            });
        }
    }
    Ok(lines)
}

fn parse_table_3_2_function_header(label: &str) -> Option<(String, String)> {
    let label = label.strip_suffix(':')?;
    parse_table_3_2_coded_label(label)
}

fn is_table_3_2_function_code(code: &str) -> bool {
    matches!(
        code,
        "050"
            | "150"
            | "250"
            | "270"
            | "300"
            | "350"
            | "370"
            | "400"
            | "450"
            | "500"
            | "550"
            | "570"
            | "600"
            | "650"
            | "700"
            | "750"
            | "800"
            | "900"
            | "920"
            | "950"
    )
}

fn parse_table_3_2_coded_label(label: &str) -> Option<(String, String)> {
    let (code, rest) = label.split_once(' ')?;
    if code.len() == 3 && code.chars().all(|char| char.is_ascii_digit()) {
        Some((code.to_string(), rest.trim().to_string()))
    } else {
        None
    }
}

fn table_3_2_optional_number(
    sheet: &BTreeMap<i64, BTreeMap<String, CellValue>>,
    row_num: i64,
    column: &str,
) -> Option<f64> {
    sheet
        .get(&row_num)
        .and_then(|row| number_cell(row.get(column)))
}

fn validate_table_3_2_rows(profile: &Table32Profile) -> Result<(), String> {
    for check in &profile.grand_checks {
        if check.table_3_1_difference.abs() > 0.5 {
            return Err(format!(
                "{}: Table 3.2/Table 3.1 total difference {}",
                check.year, check.table_3_1_difference
            ));
        }
        if check.function_total_difference.abs() > 5.0 {
            return Err(format!(
                "{}: Table 3.2 function total difference {}",
                check.year, check.function_total_difference
            ));
        }
    }
    for check in &profile.function_checks {
        if check.difference.abs() > 2.0 {
            return Err(format!(
                "{} {}: Table 3.2 function difference {}",
                check.year, check.function_code, check.difference
            ));
        }
    }
    Ok(())
}

fn table_3_2_jsonl(rows: &[Table32Row]) -> String {
    let mut lines = Vec::new();
    for row in rows {
        let subfunction_id = row.subfunction_code.as_deref().unwrap_or("total");
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"outlay_function\",\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table\":{},\"source_row_ref\":{},\"superfunction\":null,\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"measure\":\"outlays\",\"amount\":{},\"percent\":null,\"amount_units\":\"millions_usd\",\"actual_or_projection\":\"actual\",\"offsetting_treatment\":{},\"status\":\"draft-extracted\",\"observed_date\":{},\"notes\":{}}}",
            json_string(&format!(
                "outlay-function:{}:{}:{}:outlays",
                row.fiscal_year, row.function_code, subfunction_id
            )),
            row.fiscal_year,
            json_string("OMB Historical Table 3.2 FY2027"),
            json_string(&format!(
                "Table!A{}:{}{}; {}",
                row.source_row, row.source_column, row.source_row, row.source_label
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_owned_option_string(row.subfunction_code.as_ref()),
            json_owned_option_string(row.subfunction_label.as_ref()),
            json_amount(row.amount),
            json_string(table_3_2_offsetting_treatment(row)),
            json_string(OBSERVED_DATE),
            json_string(table_3_2_notes(row)),
        ));
    }
    lines.join("\n") + "\n"
}

fn table_3_2_offsetting_treatment(row: &Table32Row) -> &'static str {
    if row.function_code == "950" {
        "undistributed-offsetting-receipts"
    } else if row.subfunction_code.as_deref() == Some("809") {
        "offsetting-receipts"
    } else {
        "net"
    }
}

fn table_3_2_notes(row: &Table32Row) -> &'static str {
    match row.kind {
        Table32LineKind::Subfunction => {
            "Table 3.2 subfunction row; lower component rows and parenthetical on/off-budget splits are not emitted."
        }
        Table32LineKind::FunctionTotal => {
            "Table 3.2 parent function total used for subfunction reconciliation."
        }
        Table32LineKind::GrandTotal => {
            "Table 3.2 total outlays reconciled to OMB Historical Table 3.1 total outlays."
        }
    }
}

fn table_3_2_profile_markdown(profile: &Table32Profile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Table 3.2 Outlay Function Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        "- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Record count: {}", profile.record_count),
        format!("- Source lines emitted: {}", profile.line_count),
        format!("- Function count: {}", profile.function_count),
        format!("- Subfunction lines: {}", profile.subfunction_line_count),
        format!(
            "- Explicit function-total lines: {}",
            profile.function_total_line_count
        ),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. Function total sum uses explicit parent totals when Table 3.2 provides them, otherwise the emitted subfunction total.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.1 total outlays | Table 3.2 total outlays | Function total sum | Table 3.1 diff | Function total diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .grand_checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_1_total_outlays, 0),
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.function_total_sum, 0),
            comma_number(check.table_3_1_difference, 0),
            comma_number(check.function_total_difference, 0),
        ));
    }
    if let Some(check) = profile.function_checks.iter().max_by(|left, right| {
        left.difference
            .abs()
            .partial_cmp(&right.difference.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        lines.extend([
            String::new(),
            "## Function Reconciliation Note".to_string(),
            String::new(),
            format!(
                "Largest displayed-source function subtotal difference: FY{} `{}` {} has subfunction total {} versus parent total {}, difference {}.",
                check.year,
                check.function_code,
                check.function_label,
                comma_number(check.subfunction_total, 0),
                comma_number(check.function_total, 0),
                comma_number(check.difference, 0),
            ),
        ]);
    }
    lines.extend([
        String::new(),
        "## Extraction Decisions".to_string(),
        String::new(),
        "- Emit three-digit coded subfunction rows and explicit parent `Total, ...` rows.".to_string(),
        "- Emit `Total outlays` as a grand-total record for annual reconciliation.".to_string(),
        "- Skip lower component rows without OMB subfunction codes, including parenthetical on/off-budget splits.".to_string(),
        "- Keep TQ and FY2026-FY2031 estimate columns out of this actual-year draft.".to_string(),
        "- No public lane allocation should use these draft rows.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn build_subfunction_model(root: &Path, check_only: bool) -> Result<(), String> {
    let (records, profile) = build_subfunction_model_records(root)?;
    validate_subfunction_model_records(&records, &profile)?;
    let jsonl = subfunction_model_jsonl(&records);
    let profile_markdown = subfunction_model_profile_markdown(&profile);
    let readme = subfunction_model_readme_markdown();

    if check_only {
        compare_text(
            root,
            SUBFUNCTION_MODEL_JSONL_PATH,
            &jsonl,
            "subfunction model JSONL",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_PROFILE_PATH,
            &profile_markdown,
            "subfunction model profile",
        )?;
        compare_text(
            root,
            SUBFUNCTION_MODEL_README_PATH,
            &readme,
            "subfunction model README",
        )?;
    } else {
        fs::create_dir_all(root.join("data/derived/income_tax_outlay_subfunction_model"))
            .map_err(|err| format!("failed to create subfunction model directory: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_JSONL_PATH), jsonl)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_JSONL_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_PROFILE_PATH), profile_markdown)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_PROFILE_PATH}: {err}"))?;
        fs::write(root.join(SUBFUNCTION_MODEL_README_PATH), readme)
            .map_err(|err| format!("failed to write {SUBFUNCTION_MODEL_README_PATH}: {err}"))?;
    }

    println!(
        "validated {} subfunction model rows for {}-{}",
        records.len(),
        profile.first_year,
        profile.last_year
    );
    Ok(())
}

fn build_subfunction_model_records(
    root: &Path,
) -> Result<(Vec<SubfunctionModelRow>, SubfunctionModelProfile), String> {
    let t21 = parse_table_2_1(&read_sheet(&root.join(TABLE_2_1_PATH))?);
    let sheet_32 = read_sheet(&root.join(TABLE_3_2_PATH))?;
    let columns_by_year = table_3_2_year_columns(&sheet_32)?;
    let lines = parse_table_3_2_lines(&sheet_32)?;
    let subfunction_lines: Vec<Table32Line> = lines
        .into_iter()
        .filter(|line| matches!(line.kind, Table32LineKind::Subfunction))
        .collect();
    let years: Vec<i64> = columns_by_year
        .keys()
        .copied()
        .filter(|year| (1962..=2025).contains(year))
        .collect();

    let mut records = Vec::new();
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for year in &years {
        let Some(column) = columns_by_year.get(year) else {
            errors.push(format!("{year}: missing Table 3.2 source column"));
            continue;
        };
        let Some(table_21) = t21.get(year) else {
            errors.push(format!("{year}: missing Table 2.1 row"));
            continue;
        };
        let total_outlays = table_3_2_optional_number(&sheet_32, 140, column)
            .ok_or_else(|| format!("{year}: missing Table 3.2 total outlays"))?;
        let mut subfunction_total = 0.0;
        let mut year_values = Vec::new();
        for line in &subfunction_lines {
            let Some(amount) = table_3_2_optional_number(&sheet_32, line.source_row, column) else {
                continue;
            };
            subfunction_total += amount;
            year_values.push((line, amount));
        }
        let subfunction_total_difference = subfunction_total - total_outlays;
        if subfunction_total_difference.abs() > 10.0 {
            errors.push(format!(
                "{year}: Table 3.2 subfunction total {subfunction_total} does not reconcile to total outlays {total_outlays}"
            ));
        }
        let income_tax = table_21.individual_income_tax;
        let mut modeled_sum = 0.0;
        for (line, amount) in year_values {
            let modeled_amount = income_tax * amount / subfunction_total;
            modeled_sum += modeled_amount;
            records.push(SubfunctionModelRow {
                fiscal_year: *year,
                source_column: column.clone(),
                source_row: line.source_row,
                function_code: line.function_code.clone(),
                function_label: line.function_label.clone(),
                subfunction_code: line
                    .subfunction_code
                    .clone()
                    .ok_or_else(|| "missing subfunction code".to_string())?,
                subfunction_label: line
                    .subfunction_label
                    .clone()
                    .ok_or_else(|| "missing subfunction label".to_string())?,
                subfunction_outlays_amount: round6(amount),
                subfunction_total_outlays_amount: round6(subfunction_total),
                total_outlays_amount: round6(total_outlays),
                individual_income_tax_receipts_amount: round6(income_tax),
                outlay_share_percent: round9(amount / total_outlays * 100.0),
                allocation_share_percent: round9(amount / subfunction_total * 100.0),
                modeled_income_tax_allocation_amount: round6(modeled_amount),
            });
        }
        if (modeled_sum - income_tax).abs() > 0.0005 {
            errors.push(format!(
                "{year}: subfunction modeled sum {modeled_sum} does not match individual income-tax receipts {income_tax}"
            ));
        }
        checks.push(SubfunctionModelCheck {
            year: *year,
            table_3_2_total_outlays: total_outlays,
            subfunction_total,
            individual_income_tax: income_tax,
            modeled_sum,
            subfunction_total_difference,
        });
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    let first_year = *years
        .first()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let last_year = *years
        .last()
        .ok_or_else(|| "no subfunction model years".to_string())?;
    let subfunction_count = subfunction_lines.len();
    let profile = SubfunctionModelProfile {
        first_year,
        last_year,
        year_count: years.len(),
        record_count: records.len(),
        subfunction_count,
        checks,
    };
    Ok((records, profile))
}

fn validate_subfunction_model_records(
    records: &[SubfunctionModelRow],
    profile: &SubfunctionModelProfile,
) -> Result<(), String> {
    if records.is_empty() {
        return Err("no subfunction model rows".to_string());
    }
    for check in &profile.checks {
        if check.subfunction_total_difference.abs() > 10.0 {
            return Err(format!(
                "{}: subfunction total difference {}",
                check.year, check.subfunction_total_difference
            ));
        }
        if (check.modeled_sum - check.individual_income_tax).abs() > 0.0005 {
            return Err(format!(
                "{}: modeled sum {} does not equal income tax {}",
                check.year, check.modeled_sum, check.individual_income_tax
            ));
        }
    }
    Ok(())
}

fn subfunction_model_jsonl(records: &[SubfunctionModelRow]) -> String {
    let mut lines = Vec::new();
    for row in records {
        lines.push(format!(
            "{{\"record_id\":{},\"record_family\":\"income_tax_outlay_subfunction_model\",\"model_id\":{},\"fiscal_year\":{},\"year_basis\":\"fiscal_year\",\"source_ids\":[\"SRC-OMB-HIST-2-1-FY2027\",\"SRC-OMB-HIST-3-2-FY2027\"],\"source_table_refs\":{{\"tax_receipts\":\"OMB Historical Table 2.1 FY2027\",\"outlay_subfunction\":{}}},\"tax_source\":\"individual-income-taxes\",\"allocation_method\":\"proportional_outlay_share\",\"legal_allocation_status\":\"modeled_not_legal_dedication\",\"function_code\":{},\"function_label\":{},\"subfunction_code\":{},\"subfunction_label\":{},\"subfunction_outlays_amount\":{},\"total_outlays_amount\":{},\"subfunction_total_outlays_amount\":{},\"individual_income_tax_receipts_amount\":{},\"outlay_share_percent\":{},\"allocation_share_percent\":{},\"modeled_income_tax_allocation_amount\":{},\"actual_or_projection\":\"actual\",\"status\":\"draft\",\"observed_date\":{},\"notes\":\"Modeled allocation of ordinary individual income-tax receipts by Table 3.2 subfunction outlay share; not legal dedication or program tracing.\"}}",
            json_string(&format!(
                "income-tax-outlay-subfunction-model:{}:{}:{}",
                row.fiscal_year, row.function_code, row.subfunction_code
            )),
            json_string(SUBFUNCTION_MODEL_ID),
            row.fiscal_year,
            json_string(&format!(
                "OMB Historical Table 3.2 FY2027 row {}, column {}",
                row.source_row, row.source_column
            )),
            json_string(&row.function_code),
            json_string(&row.function_label),
            json_string(&row.subfunction_code),
            json_string(&row.subfunction_label),
            json_amount(row.subfunction_outlays_amount),
            json_amount(row.total_outlays_amount),
            json_amount(row.subfunction_total_outlays_amount),
            json_amount(row.individual_income_tax_receipts_amount),
            decimal_string(row.outlay_share_percent, 9),
            decimal_string(row.allocation_share_percent, 9),
            decimal_string(row.modeled_income_tax_allocation_amount, 6),
            json_string(OBSERVED_DATE),
        ));
    }
    lines.join("\n") + "\n"
}

fn subfunction_model_profile_markdown(profile: &SubfunctionModelProfile) -> String {
    let sample_years = [1962, 1970, 1980, 2000, 2025];
    let mut lines = vec![
        "# Income-Tax Outlay Subfunction Model Source Profile".to_string(),
        String::new(),
        "## Source Coverage".to_string(),
        String::new(),
        format!("- Model ID: `{SUBFUNCTION_MODEL_ID}`"),
        "- Tax receipt source: `SRC-OMB-HIST-2-1-FY2027`".to_string(),
        "- Outlay source: `SRC-OMB-HIST-3-2-FY2027`".to_string(),
        format!(
            "- Fiscal years emitted: {}-{}",
            profile.first_year, profile.last_year
        ),
        format!("- Year count: {}", profile.year_count),
        format!("- Subfunction count: {}", profile.subfunction_count),
        format!("- Record count: {}", profile.record_count),
        "- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.".to_string(),
        String::new(),
        "## Reconciliation Sample".to_string(),
        String::new(),
        "Amounts are in millions of dollars. `Subfunction total` is the denominator used for modeled allocation.".to_string(),
        String::new(),
        "| Fiscal year | Table 3.2 total outlays | Subfunction total | Income tax receipts | Modeled sum | Subfunction diff |".to_string(),
        "|---:|---:|---:|---:|---:|---:|".to_string(),
    ];
    for check in profile
        .checks
        .iter()
        .filter(|check| sample_years.contains(&check.year))
    {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            check.year,
            comma_number(check.table_3_2_total_outlays, 0),
            comma_number(check.subfunction_total, 0),
            comma_number(check.individual_income_tax, 0),
            comma_number(check.modeled_sum, 3),
            comma_number(check.subfunction_total_difference, 0),
        ));
    }
    lines.extend([
        String::new(),
        "## Model Caveat".to_string(),
        String::new(),
        "This is a visibility model. It allocates ordinary individual income-tax receipts by reported Table 3.2 subfunction outlay shares. It is not a legal dedication, appropriation rule, or program-financing claim.".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn subfunction_model_readme_markdown() -> String {
    [
        "# Individual Income-Tax Outlay Subfunction Model",
        "",
        "## Purpose",
        "",
        "This derived model estimates, by fiscal year and OMB Table 3.2 subfunction, how ordinary individual income-tax receipts would be allocated if allocated in proportion to that year's reported subfunction outlays.",
        "",
        "This is a visibility model. It is not a legal dedication, appropriation rule, or program-financing claim.",
        "",
        "## Model ID",
        "",
        "`individual-income-tax-proportional-subfunction-outlays-v1`",
        "",
        "## Inputs",
        "",
        "| Source ID | Role |",
        "|---|---|",
        "| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount by fiscal year. |",
        "| `SRC-OMB-HIST-3-2-FY2027` | Function and subfunction outlays by fiscal year. |",
        "",
        "## Coverage",
        "",
        "The first draft model covers fiscal years 1962-2025, the overlap between Table 3.2 actual-year subfunction rows and Table 2.1 individual income-tax receipt rows.",
        "",
        "## Artifacts",
        "",
        "| Artifact | Role |",
        "|---|---|",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl` | Canonical annual modeled allocation rows by Table 3.2 subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv` | Chart-ready long CSV view with one row per fiscal year and subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv` | Chart-ready decade rollup by subfunction. |",
        "| `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv` | Chart-ready FY2025 ranked view for the largest modeled subfunction allocations. |",
        "",
        "## Method",
        "",
        "For each fiscal year and emitted Table 3.2 subfunction:",
        "",
        "```text",
        "outlay_share_percent = subfunction_outlays / total_federal_outlays * 100",
        "allocation_share_percent = subfunction_outlays / sum_of_subfunction_outlays * 100",
        "modeled_income_tax_allocation = individual_income_tax_receipts",
        "                                * subfunction_outlays",
        "                                / sum_of_subfunction_outlays",
        "```",
        "",
        "The allocation denominator uses the emitted subfunction rows so modeled rows sum back to individual income-tax receipts. Small differences from displayed total outlays are source rounding.",
        "",
        "## Decade Rollup Caveat",
        "",
        "The decade-long CSV sums modeled allocation dollars within each decade and then calculates each subfunction's share of that decade total. It is not an average of annual percentages or annual ranks.",
        "",
        "The 1960s bucket is partial because subfunction actual-year coverage starts in FY1962. The 2020s bucket is partial because the actual-year model currently ends in FY2025.",
        "",
        "## Regeneration",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export",
        "cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check",
        "```",
        "",
        "## Validation Command",
        "",
        "```powershell",
        "cargo run -p taxlane-tools -- income-tax-outlay validate",
        "```",
        "",
    ]
    .join("\n")
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

fn table_3_1_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&2)
        .ok_or_else(|| "missing Table 3.1 header row 2".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        if let Some(year) = int_cell(Some(value)) {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

fn table_3_2_year_columns(
    rows: &BTreeMap<i64, BTreeMap<String, CellValue>>,
) -> Result<BTreeMap<i64, String>, String> {
    let header = rows
        .get(&3)
        .ok_or_else(|| "missing Table 3.2 header row 3".to_string())?;
    let mut columns = BTreeMap::new();
    for (column, value) in header {
        let year = match value {
            CellValue::Number(number) if number.fract() == 0.0 => Some(*number as i64),
            CellValue::Text(text) => parse_table_3_2_year(text),
            _ => None,
        };
        if let Some(year) = year {
            columns.insert(year, column.clone());
        }
    }
    Ok(columns)
}

fn parse_table_3_2_year(label: &str) -> Option<i64> {
    let trimmed = label.trim();
    if trimmed == "TQ" {
        return None;
    }
    trimmed
        .strip_suffix(" estimate")
        .unwrap_or(trimmed)
        .parse::<i64>()
        .ok()
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

fn export_subfunction_chart_views(root: &Path, check_only: bool) -> Result<(), String> {
    let annual = build_subfunction_annual_csv_rows(root)?;
    let decade = build_subfunction_decade_csv_rows(root)?;
    let top = build_subfunction_fy2025_top_csv_rows(root, 25)?;
    validate_subfunction_csv_rows(&annual, "subfunction annual", 4691)?;
    validate_subfunction_decade_csv_rows(&decade)?;
    validate_subfunction_csv_rows(&top, "subfunction FY2025 top", 25)?;

    if check_only {
        compare_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        compare_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    } else {
        write_csv(
            root,
            SUBFUNCTION_ANNUAL_CSV_PATH,
            SUBFUNCTION_ANNUAL_HEADERS,
            &annual,
        )?;
        write_csv(
            root,
            SUBFUNCTION_DECADE_CSV_PATH,
            SUBFUNCTION_DECADE_HEADERS,
            &decade,
        )?;
        write_csv(
            root,
            SUBFUNCTION_FY2025_TOP_CSV_PATH,
            SUBFUNCTION_TOP_HEADERS,
            &top,
        )?;
    }

    println!(
        "validated {} subfunction annual rows, {} decade rows, and {} FY2025 top rows",
        annual.len(),
        decade.len(),
        top.len()
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

fn build_subfunction_annual_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    rows.iter().map(subfunction_annual_csv_row).collect()
}

fn build_subfunction_decade_csv_rows(root: &Path) -> Result<Vec<BTreeMap<String, String>>, String> {
    let rows = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?;
    let mut grouped: BTreeMap<String, BTreeMap<(String, String), SubfunctionDecadeRollup>> =
        BTreeMap::new();
    let mut decade_receipts: BTreeMap<String, BTreeMap<i64, f64>> = BTreeMap::new();

    for row in &rows {
        let year = int_field(row, "fiscal_year")?;
        let decade = decade_label(year);
        let income_tax = number_field(row, "individual_income_tax_receipts_amount")?;
        decade_receipts
            .entry(decade.clone())
            .or_default()
            .entry(year)
            .or_insert(income_tax);

        let function_code = string_field(row, "function_code")?;
        let subfunction_code = string_field(row, "subfunction_code")?;
        let rollup = grouped
            .entry(decade)
            .or_default()
            .entry((function_code, subfunction_code))
            .or_insert_with(|| SubfunctionDecadeRollup {
                function_code: string_field(row, "function_code").unwrap_or_default(),
                function_label: string_field(row, "function_label").unwrap_or_default(),
                subfunction_code: string_field(row, "subfunction_code").unwrap_or_default(),
                subfunction_label: string_field(row, "subfunction_label").unwrap_or_default(),
                subfunction_outlays: 0.0,
                modeled_allocation: 0.0,
            });
        rollup.subfunction_outlays += number_field(row, "subfunction_outlays_amount")?;
        rollup.modeled_allocation += number_field(row, "modeled_income_tax_allocation_amount")?;
    }

    let mut output = Vec::new();
    for (decade, mut subfunctions) in grouped {
        let receipts_by_year = decade_receipts
            .get(&decade)
            .ok_or_else(|| format!("{decade}: missing receipt denominator"))?;
        let start_year = *receipts_by_year
            .keys()
            .next()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let end_year = *receipts_by_year
            .keys()
            .next_back()
            .ok_or_else(|| format!("{decade}: no years"))?;
        let year_count = receipts_by_year.len();
        let income_tax: f64 = receipts_by_year.values().sum();
        let coverage_note = if year_count == 10 {
            "full_decade"
        } else {
            "partial_decade"
        };

        let mut rows: Vec<SubfunctionDecadeRollup> =
            subfunctions.values_mut().map(|row| row.clone()).collect();
        rows.sort_by(|left, right| {
            right
                .modeled_allocation
                .partial_cmp(&left.modeled_allocation)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.subfunction_label.cmp(&right.subfunction_label))
        });

        for row in rows {
            let mut output_row = BTreeMap::new();
            output_row.insert("decade".to_string(), decade.clone());
            output_row.insert("start_fiscal_year".to_string(), start_year.to_string());
            output_row.insert("end_fiscal_year".to_string(), end_year.to_string());
            output_row.insert("year_count".to_string(), year_count.to_string());
            output_row.insert("coverage_note".to_string(), coverage_note.to_string());
            output_row.insert("function_code".to_string(), row.function_code);
            output_row.insert("function_label".to_string(), row.function_label);
            output_row.insert("subfunction_code".to_string(), row.subfunction_code);
            output_row.insert("subfunction_label".to_string(), row.subfunction_label);
            insert_rounded_number(
                &mut output_row,
                "cumulative_individual_income_tax_receipts_millions",
                income_tax,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_subfunction_outlays_millions",
                row.subfunction_outlays,
                6,
            );
            insert_rounded_number(
                &mut output_row,
                "cumulative_modeled_income_tax_allocation_millions",
                row.modeled_allocation,
                6,
            );
            insert_number(
                &mut output_row,
                "decade_allocation_share_percent",
                round9(row.modeled_allocation / income_tax * 100.0),
            );
            output_row.insert(
                "allocation_method".to_string(),
                "proportional_outlay_share".to_string(),
            );
            output_row.insert(
                "legal_allocation_status".to_string(),
                "modeled_not_legal_dedication".to_string(),
            );
            output_row.insert("actual_or_projection".to_string(), "actual".to_string());
            output.push(output_row);
        }
    }
    Ok(output)
}

fn build_subfunction_fy2025_top_csv_rows(
    root: &Path,
    count: usize,
) -> Result<Vec<BTreeMap<String, String>>, String> {
    let mut rows: Vec<serde_json::Value> = read_jsonl(root.join(SUBFUNCTION_MODEL_JSONL_PATH))?
        .into_iter()
        .filter(|row| int_field(row, "fiscal_year") == Ok(2025))
        .collect();
    rows.sort_by(|left, right| {
        number_field(right, "modeled_income_tax_allocation_amount")
            .unwrap_or(0.0)
            .partial_cmp(&number_field(left, "modeled_income_tax_allocation_amount").unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    rows.iter()
        .take(count)
        .enumerate()
        .map(|(index, row)| subfunction_top_csv_row(index + 1, row))
        .collect()
}

#[derive(Clone)]
struct SubfunctionDecadeRollup {
    function_code: String,
    function_label: String,
    subfunction_code: String,
    subfunction_label: String,
    subfunction_outlays: f64,
    modeled_allocation: f64,
}

fn subfunction_annual_csv_row(row: &serde_json::Value) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_json_number(
        &mut output,
        "individual_income_tax_receipts_millions",
        row,
        "individual_income_tax_receipts_amount",
    );
    insert_json_number(
        &mut output,
        "total_outlays_millions",
        row,
        "total_outlays_amount",
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_number(
        &mut output,
        "outlay_share_percent",
        number_field(row, "outlay_share_percent")?,
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
    output.insert(
        "actual_or_projection".to_string(),
        string_field(row, "actual_or_projection")?,
    );
    Ok(output)
}

fn subfunction_top_csv_row(
    rank: usize,
    row: &serde_json::Value,
) -> Result<BTreeMap<String, String>, String> {
    let mut output = BTreeMap::new();
    output.insert("rank".to_string(), rank.to_string());
    output.insert(
        "fiscal_year".to_string(),
        int_field(row, "fiscal_year")?.to_string(),
    );
    output.insert(
        "function_code".to_string(),
        string_field(row, "function_code")?,
    );
    output.insert(
        "function_label".to_string(),
        string_field(row, "function_label")?,
    );
    output.insert(
        "subfunction_code".to_string(),
        string_field(row, "subfunction_code")?,
    );
    output.insert(
        "subfunction_label".to_string(),
        string_field(row, "subfunction_label")?,
    );
    insert_rounded_number(
        &mut output,
        "modeled_income_tax_allocation_millions",
        number_field(row, "modeled_income_tax_allocation_amount")?,
        6,
    );
    insert_number(
        &mut output,
        "allocation_share_percent",
        number_field(row, "allocation_share_percent")?,
    );
    insert_json_number(
        &mut output,
        "subfunction_outlays_millions",
        row,
        "subfunction_outlays_amount",
    );
    output.insert(
        "allocation_method".to_string(),
        string_field(row, "allocation_method")?,
    );
    output.insert(
        "legal_allocation_status".to_string(),
        string_field(row, "legal_allocation_status")?,
    );
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

fn validate_subfunction_csv_rows(
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
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!("{label}: missing modeled legal status for {row:?}"));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!("{label}: unexpected allocation method for {row:?}"));
        }
    }
    Ok(())
}

fn validate_subfunction_decade_csv_rows(rows: &[BTreeMap<String, String>]) -> Result<(), String> {
    if rows.is_empty() {
        return Err("subfunction decade: no rows".to_string());
    }
    let mut percent_sums: BTreeMap<String, f64> = BTreeMap::new();
    for row in rows {
        if row.get("legal_allocation_status").map(String::as_str)
            != Some("modeled_not_legal_dedication")
        {
            return Err(format!(
                "subfunction decade: missing modeled legal status for {row:?}"
            ));
        }
        if row.get("allocation_method").map(String::as_str) != Some("proportional_outlay_share") {
            return Err(format!(
                "subfunction decade: unexpected allocation method for {row:?}"
            ));
        }
        let decade = row
            .get("decade")
            .ok_or_else(|| "subfunction decade: missing decade".to_string())?;
        let percent = row
            .get("decade_allocation_share_percent")
            .ok_or_else(|| "subfunction decade: missing percent".to_string())?
            .parse::<f64>()
            .map_err(|err| format!("subfunction decade: invalid percent: {err}"))?;
        *percent_sums.entry(decade.to_string()).or_default() += percent;
    }
    for (decade, percent_sum) in percent_sums {
        if (percent_sum - 100.0).abs() > 0.0001 {
            return Err(format!(
                "subfunction decade: {decade} percent sum {percent_sum}"
            ));
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

fn validate_accountability_evidence_records(root: &Path) -> Result<(), String> {
    let source_ledger = fs::read_to_string(root.join(SOURCE_VERSION_LEDGER_PATH))
        .map_err(|err| format!("failed to read {SOURCE_VERSION_LEDGER_PATH}: {err}"))?;
    let records = read_accountability_evidence_records(root)?;
    if records.is_empty() {
        return Err("accountability evidence: no records".to_string());
    }

    for record in records {
        record
            .validate()
            .map_err(|err| format!("{}: {err}", record.record_id))?;
        for source_id in &record.source_ids {
            let ledger_token = format!("`{source_id}`");
            if !source_ledger.contains(&ledger_token) {
                return Err(format!(
                    "{}: source_id {source_id} is missing from {SOURCE_VERSION_LEDGER_PATH}",
                    record.record_id
                ));
            }
        }
    }

    println!("validated accountability evidence records");
    Ok(())
}

fn check_accountability_readiness_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_readiness_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_READINESS_REPORT_PATH,
        &expected,
        "accountability readiness report",
    )?;
    println!("validated accountability readiness report");
    Ok(())
}

fn check_accountability_action_queue(root: &Path) -> Result<(), String> {
    let expected = build_accountability_action_queue(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_ACTION_QUEUE_PATH,
        &expected,
        "accountability action queue",
    )?;
    println!("validated accountability action queue");
    Ok(())
}

fn check_accountability_performance_demand_packet(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_packet(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_PACKET_PATH,
        &expected,
        "accountability performance demand packet",
    )?;
    println!("validated accountability performance demand packet");
    Ok(())
}

fn check_accountability_work_items(root: &Path) -> Result<(), String> {
    let expected = build_accountability_work_items_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_WORK_ITEMS_JSONL_PATH,
        &expected,
        "accountability work items",
    )?;
    println!("validated accountability work items");
    Ok(())
}

fn check_accountability_claim_guard_report(root: &Path) -> Result<(), String> {
    let expected = build_accountability_claim_guard_report(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_CLAIM_GUARD_REPORT_PATH,
        &expected,
        "accountability claim guard report",
    )?;
    println!("validated accountability claim guard report");
    Ok(())
}

fn check_accountability_public_questions(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_questions(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_QUESTIONS_PATH,
        &expected,
        "accountability public questions",
    )?;
    println!("validated accountability public questions");
    Ok(())
}

fn check_accountability_public_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_public_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PUBLIC_BRIEF_PATH,
        &expected,
        "accountability public brief",
    )?;
    println!("validated accountability public brief");
    Ok(())
}

fn check_accountability_public_brief_discovery(root: &Path) -> Result<(), String> {
    let root_readme = fs::read_to_string(root.join(README_PATH))
        .map_err(|err| format!("failed to read {README_PATH}: {err}"))?;
    if !root_readme.contains(ACCOUNTABILITY_PUBLIC_BRIEF_PATH) {
        return Err(format!(
            "{README_PATH} must link {ACCOUNTABILITY_PUBLIC_BRIEF_PATH}"
        ));
    }

    let reading_index = fs::read_to_string(root.join(READING_INDEX_PATH))
        .map_err(|err| format!("failed to read {READING_INDEX_PATH}: {err}"))?;
    if !reading_index.contains("accountability-public-brief.md") {
        return Err(format!(
            "{READING_INDEX_PATH} must link accountability-public-brief.md"
        ));
    }

    println!("validated accountability public brief discovery");
    Ok(())
}

fn check_accountability_artifact_map(root: &Path) -> Result<(), String> {
    let expected = build_accountability_artifact_map();
    compare_text(
        root,
        ACCOUNTABILITY_ARTIFACT_MAP_PATH,
        &expected,
        "accountability artifact map",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("artifact-map.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link artifact-map.md".to_string(),
        );
    }

    let artifact_map = fs::read_to_string(root.join(ACCOUNTABILITY_ARTIFACT_MAP_PATH))
        .map_err(|err| format!("failed to read {ACCOUNTABILITY_ARTIFACT_MAP_PATH}: {err}"))?;
    for required in [
        "performance-demand-dashboard.md",
        "performance-demand-claim-gates.json",
        "performance-demand-checklist.jsonl",
        "performance-demand-checklist.schema.md",
    ] {
        if !artifact_map.contains(required) {
            return Err(format!(
                "{ACCOUNTABILITY_ARTIFACT_MAP_PATH} must route {required}"
            ));
        }
    }

    println!("validated accountability artifact map");
    Ok(())
}

fn check_accountability_performance_demand_checklist(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_PATH,
        &expected,
        "accountability performance demand checklist",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand checklist");
    Ok(())
}

fn check_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_checklist_jsonl(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH,
        &expected,
        "accountability performance demand checklist JSONL",
    )?;

    let rows: Vec<PerformanceDemandChecklistRecord> =
        read_jsonl(root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH))?
            .into_iter()
            .map(|row| {
                serde_json::from_value(row).map_err(|err| {
                    format!("accountability performance demand checklist JSONL: {err}")
                })
            })
            .collect::<Result<_, _>>()?;
    if rows.is_empty() {
        return Err("accountability performance demand checklist JSONL has no rows".to_string());
    }
    let mut expected_rows = read_accountability_evidence_records(root)?;
    expected_rows.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let expected_rows: Vec<PerformanceDemandChecklistRecord> = expected_rows
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    if rows != expected_rows {
        return Err(
            "accountability performance demand checklist JSONL does not match core records"
                .to_string(),
        );
    }
    for row in rows {
        row.validate()?;
        if row.public_claim_allowed {
            return Err(
                "accountability performance demand checklist JSONL unexpectedly allows a public claim"
                    .to_string(),
            );
        }
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-checklist.jsonl") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-checklist.jsonl"
                .to_string(),
        );
    }
    let schema_filename = ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH
        .rsplit('/')
        .next()
        .unwrap_or(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_SCHEMA_PATH);
    if !index.contains(schema_filename) {
        return Err(format!(
            "data/derived/accountability_evidence/README.md must link {schema_filename}"
        ));
    }

    println!("validated accountability performance demand checklist JSONL");
    Ok(())
}

fn check_accountability_performance_demand_claim_gates(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_claim_gates(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH,
        &expected,
        "accountability performance demand claim gates",
    )?;

    let parsed_text = fs::read_to_string(
        root.join(ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH),
    )
    .map_err(|err| {
        format!("failed to read {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let parsed: serde_json::Value = serde_json::from_str(&parsed_text).map_err(|err| {
        format!("failed to parse {ACCOUNTABILITY_PERFORMANCE_DEMAND_CLAIM_GATES_PATH}: {err}")
    })?;
    let total_rows = parsed
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing total_rows".to_string())?;
    let blocked_rows = parsed
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_blocked".to_string())?;
    let allowed_rows = parsed
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "performance demand claim gates missing public_claim_allowed".to_string())?;
    if total_rows != blocked_rows + allowed_rows {
        return Err(
            "performance demand claim gates total does not match allowed plus blocked".to_string(),
        );
    }
    if allowed_rows != 0 {
        return Err("performance demand claim gates unexpectedly allow a public claim".to_string());
    }

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-claim-gates.json") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-claim-gates.json"
                .to_string(),
        );
    }

    println!("validated accountability performance demand claim gates");
    Ok(())
}

fn check_accountability_performance_demand_dashboard(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_dashboard(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_DASHBOARD_PATH,
        &expected,
        "accountability performance demand dashboard",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-dashboard.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-dashboard.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand dashboard");
    Ok(())
}

fn check_accountability_performance_demand_brief(root: &Path) -> Result<(), String> {
    let expected = build_accountability_performance_demand_brief(root)?;
    compare_text(
        root,
        ACCOUNTABILITY_PERFORMANCE_DEMAND_BRIEF_PATH,
        &expected,
        "accountability performance demand brief",
    )?;

    let index = fs::read_to_string(root.join("data/derived/accountability_evidence/README.md"))
        .map_err(|err| {
            format!("failed to read data/derived/accountability_evidence/README.md: {err}")
        })?;
    if !index.contains("performance-demand-brief.md") {
        return Err(
            "data/derived/accountability_evidence/README.md must link performance-demand-brief.md"
                .to_string(),
        );
    }

    println!("validated accountability performance demand brief");
    Ok(())
}

fn build_accountability_readiness_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut lines = vec![
        "# Accountability Evidence Readiness Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This report classifies draft accountability evidence records by public-claim readiness.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Readiness States".to_string(),
        String::new(),
        "| State | Meaning |".to_string(),
        "|---|---|".to_string(),
        "| `EvidenceOnly` | Internal evidence review only; not ready for public claims. |".to_string(),
        "| `NeedsRoleReview` | Source/accountability reviewed and waiting for public wording review. |".to_string(),
        "| `PublicClaimEligible` | Role reviewed with official finding or adjudicated status. |".to_string(),
        String::new(),
        "## Records".to_string(),
        String::new(),
        "| Record ID | Lane | Evidence Kind | Anomaly Class | Allegation Status | Review Status | Readiness | Next Action | Public Summary |".to_string(),
        "|---|---|---|---|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        let readiness = record.public_claim_readiness();
        lines.push(format!(
            "| `{}` | {} | {:?} | {:?} | {:?} | {:?} | `{}` | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.evidence_kind,
            record.anomaly_class,
            record.allegation_status,
            record.review_status,
            readiness.as_str(),
            record.accountability_next_action().replace('|', "\\|"),
            record.public_summary.replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Records marked `EvidenceOnly` or `NeedsRoleReview` must not be presented as public fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_action_queue(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut queue: BTreeMap<&'static str, Vec<AccountabilityEvidenceRecord>> = BTreeMap::new();
    for record in records {
        queue
            .entry(record.accountability_next_action())
            .or_default()
            .push(record);
    }

    let mut lines = vec![
        "# Accountability Evidence Action Queue".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated queue turns draft accountability evidence records into reviewer work."
            .to_string(),
        "It is not a public fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Queue".to_string(),
    ];

    for (action, mut records) in queue {
        records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
        lines.push(String::new());
        lines.push(format!("### {action}"));
        lines.push(String::new());
        lines.push("| Record ID | Lane | Readiness | Public-Use Blocker |".to_string());
        lines.push("|---|---|---|---|".to_string());
        for record in records {
            lines.push(format!(
                "| `{}` | {} | `{}` | {} |",
                record.record_id,
                record.lane_id.as_deref().unwrap_or("n/a"),
                record.public_claim_readiness().as_str(),
                record
                    .accountability_public_use_blocker()
                    .replace('|', "\\|")
            ));
        }
    }

    lines.push(String::new());
    lines.push("## Guardrail".to_string());
    lines.push(String::new());
    lines.push(
        "Queue entries are tasks for evidence review. They are not publishable claims by themselves."
            .to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_packet(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Accountability Performance Demand Packet".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated packet turns accountability evidence blockers into questions people can ask before demanding performance on public money.".to_string(),
        "It explains what TAXLANE can say now, what evidence is still missing, and what claim boundary remains in force.".to_string(),
        String::new(),
        "## Demand Questions".to_string(),
        String::new(),
        "| Record ID | Lane | What TAXLANE Can Say Now | Demand Question | Claim Boundary |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];

    for record in records {
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} |",
            record.record_id,
            record.lane_id.as_deref().unwrap_or("n/a"),
            record.public_summary.replace('|', "\\|"),
            record.accountability_demand_question().replace('|', "\\|"),
            record
                .accountability_public_use_blocker()
                .replace('|', "\\|")
        ));
    }

    lines.push(String::new());
    lines.push("## Public-Use Rule".to_string());
    lines.push(String::new());
    lines.push(
        "Use these rows to request evidence, reviewed wording, or official findings. Do not present them as fraud, waste, abuse, or performance findings.".to_string(),
    );

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_work_items_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let mut lines = Vec::new();
    for record in records {
        lines.push(
            serde_json::to_string(&record.accountability_work_item())
                .map_err(|err| format!("failed to serialize accountability work item: {err}"))?,
        );
    }
    Ok(lines.join("\n") + "\n")
}

fn build_accountability_claim_guard_report(root: &Path) -> Result<String, String> {
    let records = read_accountability_evidence_records(root)?;
    let mut readiness_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut blocker_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut public_claim_allowed = 0usize;

    for record in &records {
        let work_item = record.accountability_work_item();
        *readiness_counts.entry(work_item.readiness).or_default() += 1;
        *blocker_counts
            .entry(work_item.public_use_blocker)
            .or_default() += 1;
        if work_item.public_claim_allowed {
            public_claim_allowed += 1;
        }
    }

    let mut lines = vec![
        "# Accountability Claim Guard Report".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated report summarizes whether accountability evidence records can support public claims.".to_string(),
        "It is a guardrail report, not a fraud, waste, abuse, or performance scorecard.".to_string(),
        String::new(),
        "## Claim Guard Summary".to_string(),
        String::new(),
        format!("- Total records: {}", records.len()),
        format!("- Public claims currently allowed: {public_claim_allowed}"),
        format!(
            "- Public claims currently blocked: {}",
            records.len().saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Readiness Counts".to_string(),
        String::new(),
        "| Readiness | Records |".to_string(),
        "|---|---:|".to_string(),
    ];

    for (readiness, count) in readiness_counts {
        lines.push(format!("| `{readiness}` | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Public-Use Blockers".to_string(),
        String::new(),
        "| Blocker | Records |".to_string(),
        "|---|---:|".to_string(),
    ]);

    for (blocker, count) in blocker_counts {
        let escaped_blocker = blocker.replace('|', "\\|");
        lines.push(format!("| {escaped_blocker} | {count} |"));
    }

    lines.extend([
        String::new(),
        "## Allowed Public Use".to_string(),
        String::new(),
        "Current safe use: ask the demand questions and request the missing reviewed evidence or role-approved wording.".to_string(),
        "Current unsafe use: present these draft records as fraud, waste, abuse, or performance findings.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_public_questions(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Public Accountability Questions".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "These generated questions are safe to ask publicly because they request reviewed evidence or role-approved wording.".to_string(),
        "They are not findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Questions".to_string(),
        String::new(),
        "| Lane | Public-Safe Question | Why This Is Still Blocked |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use these questions to ask for evidence. Do not present the underlying draft records as public claims.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_public_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let total_records = records.len();
    let public_claim_allowed = records
        .iter()
        .filter(|record| record.accountability_work_item().public_claim_allowed)
        .count();

    let mut lines = vec![
        "# Accountability Public Brief".to_string(),
        String::new(),
        "## What TAXLANE Can Say Now".to_string(),
        String::new(),
        "TAXLANE can model how ordinary individual income-tax receipts compare with broad federal outlay categories.".to_string(),
        "That model is a visibility tool, not a legal claim that a taxpayer's dollars are dedicated to a specific program.".to_string(),
        String::new(),
        "TAXLANE can also ask accountability questions about whether spending has reviewed performance evidence.".to_string(),
        "Current accountability records are not public findings of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Current Claim Guard".to_string(),
        String::new(),
        format!("- Accountability records reviewed for public use: {total_records}"),
        format!("- Records currently public-claim eligible: {public_claim_allowed}"),
        format!(
            "- Records still blocked from public claims: {}",
            total_records.saturating_sub(public_claim_allowed)
        ),
        String::new(),
        "## Safe Public Questions".to_string(),
        String::new(),
        "| Lane | Question To Ask | Why It Matters |".to_string(),
        "|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        lines.push(format!(
            "| {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|")
        ));
    }

    lines.extend([
        String::new(),
        "## Use / Avoid".to_string(),
        String::new(),
        "| Use | Avoid |".to_string(),
        "|---|---|".to_string(),
        "| Ask for reviewed performance targets, outcome measures, audit sources, or role-approved wording. | Do not say TAXLANE found fraud, waste, abuse, or poor performance from these draft records. |".to_string(),
        "| Use modeled allocation language when explaining income-tax visibility. | Do not say ordinary income-tax dollars are legally dedicated to the displayed lanes. |".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_artifact_map() -> String {
    let rows = [
        (
            "accountability_evidence.omb-fy2027-v1.2026-06-23.draft.jsonl",
            "Internal evidence reviewers",
            "Validate source-custodied evidence shape.",
            "Do not publish as findings.",
        ),
        (
            "readiness-report.md",
            "Accountability researchers",
            "See readiness and next action per record.",
            "Do not treat readiness as a performance score.",
        ),
        (
            "action-queue.md",
            "Review leads",
            "Work records by next task.",
            "Do not publish queue rows as claims.",
        ),
        (
            "performance-demand-packet.md",
            "Accountability researchers",
            "Ask what evidence, reviewed wording, or official finding is missing.",
            "Do not allege misconduct.",
        ),
        (
            "accountability-work-items.jsonl",
            "Product implementers",
            "Feed future UI/API workflow from structured fields.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "claim-guard-report.md",
            "Review leads",
            "Check allowed versus blocked public claims.",
            "Do not publish findings from blocked records.",
        ),
        (
            "public-questions.md",
            "Citizen readers",
            "Ask safe public questions about performance evidence.",
            "Do not expose raw draft evidence as claims.",
        ),
        (
            "performance-demand-checklist.md",
            "Citizen readers",
            "Demand source, performance, official-finding, wording, and claim-gate evidence.",
            "Do not treat demand rows as findings.",
        ),
        (
            "performance-demand-dashboard.md",
            "Citizen readers",
            "Scan demand-row claim gates before public use.",
            "Do not publish blocked rows as claims.",
        ),
        (
            "performance-demand-brief.md",
            "Citizen readers",
            "Use a compact ask packet for current blocked demand rows.",
            "Do not present the brief as a finding or scorecard.",
        ),
        (
            "performance-demand-checklist.jsonl",
            "Product implementers",
            "Feed demand rows into future UI/API surfaces.",
            "Do not infer public eligibility except from `public_claim_allowed`.",
        ),
        (
            "performance-demand-claim-gates.json",
            "Product implementers",
            "Display allowed versus blocked demand-row counts.",
            "Do not recompute or override claim gates downstream.",
        ),
        (
            "performance-demand-checklist.schema.md",
            "Product implementers",
            "Inspect the demand checklist row contract.",
            "Do not add UI/API fields that weaken the use rule.",
        ),
        (
            "docs/reading/accountability-public-brief.md",
            "Citizen readers",
            "Read the current public handoff.",
            "Do not describe modeled allocation as legal dedication.",
        ),
    ];

    let mut lines = vec![
        "# Accountability Artifact Map".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This map shows which accountability artifact to use for evidence review, performance-demand questions, and public-safe reader handoff.".to_string(),
        "It is not a list of fraud, waste, abuse, or performance findings.".to_string(),
        String::new(),
        "## Use Order".to_string(),
        String::new(),
        "1. Start with the draft JSONL records for source custody.".to_string(),
        "2. Use readiness, queue, demand, work-item, and claim-guard artifacts for internal review workflow.".to_string(),
        "3. Use public questions and the public brief only for outward-facing questions and handoff wording.".to_string(),
        String::new(),
        "## Artifacts".to_string(),
        String::new(),
        "| Artifact | Audience | Use | Avoid |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for (artifact, audience, use_case, avoid) in rows {
        lines.push(format!(
            "| `{artifact}` | {audience} | {use_case} | {avoid} |"
        ));
    }

    lines.extend([
        String::new(),
        "## Public-Use Rule".to_string(),
        String::new(),
        "Public artifacts may ask for performance evidence and official findings. They must not claim fraud, waste, abuse, legal dedication of income taxes, or program performance without reviewed evidence and claim eligibility.".to_string(),
    ]);

    lines.join("\n") + "\n"
}

fn build_accountability_performance_demand_checklist(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Checklist".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated checklist turns TAXLANE accountability blockers into evidence requests a citizen can make before accepting performance or misconduct claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, or poor performance.".to_string(),
        String::new(),
        "## Before Accepting A Claim".to_string(),
        String::new(),
        "- Ask for the source record and source version.".to_string(),
        "- Ask for the reviewed performance target, outcome measure, audit source, or official finding.".to_string(),
        "- Ask whether role review approved the exact public wording.".to_string(),
        "- Ask whether the record is public-claim eligible.".to_string(),
        String::new(),
        "## Record Checklist".to_string(),
        String::new(),
        "| Lane | Demand This Evidence | Do Not Accept Yet | Claim Gate |".to_string(),
        "|---|---|---|---|".to_string(),
    ];

    for record in records {
        let work_item = record.accountability_work_item();
        let claim_gate = if work_item.public_claim_allowed {
            "Public claim allowed."
        } else {
            "Public claim blocked."
        };
        lines.push(format!(
            "| {} | {} | {} | {} |",
            work_item.lane_id.unwrap_or("n/a"),
            work_item.demand_question.replace('|', "\\|"),
            work_item.public_use_blocker.replace('|', "\\|"),
            claim_gate
        ));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this checklist to demand performance evidence and reviewed wording. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_checklist_jsonl(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = Vec::new();
    for record in records {
        let row = record.performance_demand_checklist_row();
        lines.push(
            serde_json::to_string(&row)
                .map_err(|err| format!("failed to serialize demand checklist row: {err}"))?,
        );
    }

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_claim_gates(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));
    let rows: Vec<PerformanceDemandChecklistRecord> = records
        .iter()
        .map(AccountabilityEvidenceRecord::performance_demand_checklist_record)
        .collect();
    let total_rows = rows.len();
    let public_claim_allowed = rows.iter().filter(|row| row.public_claim_allowed).count();
    let public_claim_blocked = total_rows.saturating_sub(public_claim_allowed);
    let mut gate_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &rows {
        *gate_counts.entry(&row.claim_gate).or_default() += 1;
    }
    let claim_gates: Vec<String> = gate_counts
        .into_iter()
        .map(|(claim_gate, rows)| {
            format!(
                "    {{\"claim_gate\":{},\"rows\":{rows}}}",
                json_string(claim_gate)
            )
        })
        .collect();

    Ok(format!(
        concat!(
            "{{\n",
            "  \"artifact\": {},\n",
            "  \"total_rows\": {},\n",
            "  \"public_claim_allowed\": {},\n",
            "  \"public_claim_blocked\": {},\n",
            "  \"claim_gates\": [\n",
            "{}\n",
            "  ],\n",
            "  \"use_rule\": {}\n",
            "}}\n"
        ),
        json_string(ACCOUNTABILITY_PERFORMANCE_DEMAND_CHECKLIST_JSONL_PATH),
        total_rows,
        public_claim_allowed,
        public_claim_blocked,
        claim_gates.join(",\n"),
        json_string(
            "Demand evidence and reviewed wording; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, or poor performance."
        )
    ))
}

fn build_accountability_performance_demand_dashboard(root: &Path) -> Result<String, String> {
    let claim_gates_text = build_accountability_performance_demand_claim_gates(root)?;
    let claim_gates: serde_json::Value =
        serde_json::from_str(&claim_gates_text).map_err(|err| {
            format!("failed to parse generated performance demand claim gates: {err}")
        })?;
    let total_rows = claim_gates
        .get("total_rows")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing total_rows".to_string())?;
    let allowed_rows = claim_gates
        .get("public_claim_allowed")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_allowed".to_string())?;
    let blocked_rows = claim_gates
        .get("public_claim_blocked")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "generated claim gates missing public_claim_blocked".to_string())?;
    let use_rule = claim_gates
        .get("use_rule")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "generated claim gates missing use_rule".to_string())?;

    let mut lines = vec![
        "# Performance Demand Dashboard".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated dashboard summarizes whether performance demand checklist rows can support public claims.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, or poor performance.".to_string(),
        String::new(),
        "## Claim Gate Summary".to_string(),
        String::new(),
        format!("- Demand rows: {total_rows}"),
        format!("- Public claims currently allowed: {allowed_rows}"),
        format!("- Public claims currently blocked: {blocked_rows}"),
        String::new(),
        "## Claim Gates".to_string(),
        String::new(),
        "| Claim Gate | Rows |".to_string(),
        "|---|---:|".to_string(),
    ];

    let gate_rows = claim_gates
        .get("claim_gates")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "generated claim gates missing claim_gates".to_string())?;
    for gate in gate_rows {
        let claim_gate = gate
            .get("claim_gate")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "generated claim gate row missing claim_gate".to_string())?;
        let rows = gate
            .get("rows")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| "generated claim gate row missing rows".to_string())?;
        lines.push(format!("| {} | {rows} |", claim_gate.replace('|', "\\|")));
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        use_rule.to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn build_accountability_performance_demand_brief(root: &Path) -> Result<String, String> {
    let mut records = read_accountability_evidence_records(root)?;
    records.sort_by(|left, right| left.record_id.cmp(&right.record_id));

    let mut lines = vec![
        "# Performance Demand Brief".to_string(),
        String::new(),
        "## Purpose".to_string(),
        String::new(),
        "This generated brief turns blocked performance demand rows into a compact ask packet for citizen readers.".to_string(),
        "It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.".to_string(),
        String::new(),
        "## Current Claim Status".to_string(),
        String::new(),
        "TAXLANE currently has no performance demand rows that are public-claim eligible.".to_string(),
        "Use the rows below to ask for evidence, not to assert wrongdoing or performance failure.".to_string(),
        String::new(),
        "## Ask Packet".to_string(),
    ];

    for record in records {
        let row = record.performance_demand_checklist_record();
        let label = row.lane_id.as_deref().unwrap_or("n/a");
        lines.extend([
            String::new(),
            format!("### {label}"),
            String::new(),
            format!("- Ask: {}", row.demand_question),
            format!("- Do not accept yet: {}", row.do_not_accept_yet),
            format!("- Claim gate: {}", row.claim_gate),
            format!("- Public claim allowed: {}", row.public_claim_allowed),
            "- Required evidence:".to_string(),
        ]);
        for evidence in row.demand_evidence {
            lines.push(format!("  - {evidence}"));
        }
    }

    lines.extend([
        String::new(),
        "## Use Rule".to_string(),
        String::new(),
        "Use this brief to demand source records, reviewed performance evidence, official findings, role-approved wording, and public-claim eligibility. Do not use it to claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.".to_string(),
    ]);

    Ok(lines.join("\n") + "\n")
}

fn read_accountability_evidence_records(
    root: &Path,
) -> Result<Vec<AccountabilityEvidenceRecord>, String> {
    read_jsonl(root.join(ACCOUNTABILITY_EVIDENCE_JSONL_PATH))?
        .into_iter()
        .map(|row| {
            serde_json::from_value(row)
                .map_err(|err| format!("accountability evidence: invalid record shape: {err}"))
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

fn insert_rounded_number(
    row: &mut BTreeMap<String, String>,
    field: &str,
    value: f64,
    decimals: usize,
) {
    row.insert(field.to_string(), rounded_decimal(value, decimals));
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

fn rounded_decimal(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let factor = 10_i128.pow(decimals as u32);
    let scaled = (value * factor as f64).round() as i128;
    let sign = if scaled < 0 { "-" } else { "" };
    let absolute = scaled.abs();
    let integer = absolute / factor;
    let fraction = absolute % factor;

    if decimals == 0 || fraction == 0 {
        return format!("{sign}{integer}");
    }

    let mut fraction_text = format!("{fraction:0decimals$}");
    while fraction_text.ends_with('0') {
        fraction_text.pop();
    }
    format!("{sign}{integer}.{fraction_text}")
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

fn json_amount(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        decimal_string(value, 6)
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

fn json_option_string(value: Option<&str>) -> String {
    value.map_or_else(|| "null".to_string(), json_string)
}

fn json_owned_option_string(value: Option<&String>) -> String {
    value.map_or_else(|| "null".to_string(), |value| json_string(value))
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
    let metadata: Vec<ArtifactMetadata<'_>> = ARTIFACTS.iter().map(Artifact::metadata).collect();
    taxlane_core::validate_artifact_metadata(&metadata)?;

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
        "This manifest records the artifact chain for modeled allocations of".to_string(),
        "ordinary individual income-tax receipts by OMB outlay share.".to_string(),
        String::new(),
        "The annual, decade, and subfunction JSONL files are canonical model".to_string(),
        "outputs. CSV files, Markdown notes, and chart specs are derived or".to_string(),
        "supporting views.".to_string(),
        String::new(),
        "## Model".to_string(),
        String::new(),
        "- Broad model ID: `individual-income-tax-proportional-outlays-v1`".to_string(),
        "- Subfunction model ID: `individual-income-tax-proportional-subfunction-outlays-v1`"
            .to_string(),
        "- Broad coverage: fiscal years 1940-2025 for annual actual-year rows".to_string(),
        "- Subfunction coverage: fiscal years 1962-2025 for Table 3.2 actual-year rows".to_string(),
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
        "4. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model`".to_string(),
        "5. `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export`".to_string(),
        "6. `cargo run -p taxlane-tools -- income-tax-outlay manifest`".to_string(),
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
