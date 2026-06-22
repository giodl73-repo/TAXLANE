# Income-Tax Outlay Chart Export Note

## Purpose

TAXLANE now has chart-ready CSV views for the annual and decade modeled
income-tax allocation datasets.

The CSVs are convenience exports. The canonical records remain the JSONL files:

- `income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl`
- `income_tax_outlay_model.omb-fy2027.2026-06-21.decade-summary.jsonl`

## Export Files

| File | Row grain | Use |
|---|---|---|
| `income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv` | One row per fiscal year, 1940-2025 | Time-series charts and year picker views. |
| `income_tax_outlay_model.omb-fy2027.2026-06-21.decade-wide.csv` | One row per decade bucket | Long-run comparison and narrative summary. |

## Recommended Charts

1. Stacked area chart of annual category percentages, 1940-2025.
2. Decade stacked bar chart using the decade-wide CSV.
3. Line chart for borrowed share of outlays beside, not inside, the modeled tax
   allocation stack.
4. Line chart for individual income-tax coverage of total outlays.

## Wording Guardrails

Use chart titles such as:

- "Modeled allocation of individual income-tax receipts by outlay share"
- "If income-tax receipts were allocated by broad spending shares"

Avoid chart titles such as:

- "Where income taxes went"
- "What income taxes funded"

Those titles imply legal or program tracing that this model does not support.
