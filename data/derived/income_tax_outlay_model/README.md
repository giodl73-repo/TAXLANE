# Individual Income-Tax Outlay Model

## Purpose

This derived model estimates, by fiscal year, how ordinary individual
income-tax receipts would be allocated across broad OMB outlay categories if
allocated in proportion to that year's reported federal outlays.

This is a visibility model. It is not a legal dedication, appropriation rule, or
program-financing claim.

## Model ID

`individual-income-tax-proportional-outlays-v1`

## Inputs

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | Total receipts, total outlays, and surplus/deficit fiscal spine. |
| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount by fiscal year. |
| `SRC-OMB-HIST-3-1-FY2027` | Broad outlay categories and total federal outlays by fiscal year. |

## Coverage

The first draft model covers fiscal years 1940-2025, the overlap between Table
3.1 broad outlay categories and Table 1.1 actual-year totals. OMB years after
2025 are excluded from this first output because they are estimates or
projections in the FY2027 historical tables.

## Method

For each fiscal year and broad Table 3.1 category:

```text
outlay_share_percent = category_outlays / total_federal_outlays * 100
allocation_share_percent = category_outlays / sum_of_broad_category_outlays * 100
modeled_income_tax_allocation = individual_income_tax_receipts
                                * category_outlays
                                / sum_of_broad_category_outlays
```

Amounts are in millions of dollars. Negative outlay categories, such as
undistributed offsetting receipts, remain negative. Because OMB displays Table
3.1 category rows in millions, the six broad categories can differ from the
displayed total by small rounding amounts. The model keeps the displayed total
for deficit context and uses the broad-category sum as the allocation
denominator so category allocations sum back to individual income-tax receipts.

## Required Row Fields

| Field | Meaning |
|---|---|
| `record_id` | Stable model row identifier. |
| `record_family` | Always `income_tax_outlay_model`. |
| `model_id` | Always `individual-income-tax-proportional-outlays-v1`. |
| `fiscal_year` | Federal fiscal year. |
| `year_basis` | Always `fiscal_year`. |
| `source_ids` | Table 1.1, 2.1, and 3.1 source IDs. |
| `tax_source` | Always `individual-income-taxes`. |
| `allocation_method` | Always `proportional_outlay_share`. |
| `legal_allocation_status` | Always `modeled_not_legal_dedication`. |
| `category_key` | Stable broad category key. |
| `category_label` | OMB broad category label. |
| `category_outlays_amount` | Table 3.1 category outlays in millions of dollars. |
| `total_outlays_amount` | Table 3.1 total federal outlays in millions of dollars. |
| `category_total_outlays_amount` | Sum of the six broad Table 3.1 category rows in millions of dollars. |
| `individual_income_tax_receipts_amount` | Table 2.1 individual income-tax receipts in millions of dollars. |
| `outlay_share_percent` | Category outlays divided by displayed total outlays. |
| `allocation_share_percent` | Category outlays divided by the broad-category sum used for allocation. |
| `modeled_income_tax_allocation_amount` | Individual income-tax receipts allocated by category outlay share. |
| `total_receipts_amount` | Table 1.1 total federal receipts in millions of dollars. |
| `surplus_or_deficit_amount` | Table 1.1 surplus or deficit in millions of dollars; deficits are negative. |
| `deficit_gap_amount` | Positive amount by which outlays exceed receipts. |
| `borrowed_share_percent_of_outlays` | Deficit gap divided by total outlays. |
| `income_tax_coverage_percent_of_outlays` | Individual income-tax receipts divided by total outlays. |
| `category_total_reconciliation_difference_amount` | Broad-category sum minus displayed total federal outlays. |
| `actual_or_projection` | `actual` for this first output. |
| `status` | `draft` until source-reviewed. |
| `observed_date` | Source observation date. |
| `notes` | Public caveat and reconciliation caveat. |

## Validation

The generator checks:

1. Every modeled year has Table 1.1, Table 2.1, and Table 3.1 inputs.
2. Table 3.1 total federal outlays reconcile to Table 1.1 total outlays.
3. Broad Table 3.1 category outlays reconcile to Table 3.1 total federal
   outlays within displayed-million rounding tolerance.
4. Modeled category allocations sum back to individual income-tax receipts.
5. No estimate or projection years are emitted.

## Public Wording

Use "modeled allocation" or "if allocated by outlay share." Do not write that
income-tax dollars paid for a category unless a separate legal-dedication source
supports that claim.

## Derived Summaries

`build_decade_summary.py` reads the annual draft JSONL and emits decade-level
summary rows. A decade category percentage is cumulative modeled category
allocation divided by cumulative individual income-tax receipts for the years in
that decade. The 2020s summary is partial because the actual-year model ends in
2025.

## Chart Exports

`export_chart_views.py` reads the annual and decade JSONL files and emits wide
CSV files for charting:

- `income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv`
- `income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv`

The JSONL files remain canonical. CSV files are convenience views for charts,
spreadsheets, and quick inspection.

## Manifest

`build_manifest.py` writes `MANIFEST.md`, which records artifact roles, row
counts, checksums, and regeneration order for the model outputs and supporting
views.

## Validation Runner

Run all model and chart checks with:

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
```

The older Python runner remains available while the generator scripts are being
ported:

```powershell
python data/derived/income_tax_outlay_model/validate_all.py
```
