# TAXLANE Chart Catalog

## Purpose

This catalog lists chart specifications that visualize TAXLANE data artifacts.
The specs are handoff assets for analysis and future UI work. They do not
change the underlying data or model methods.

## Available Chart Sets

| Chart set | Purpose | Data artifacts |
|---|---|---|
| `income-tax-outlay-model/` | Visualize the modeled allocation of ordinary individual income-tax receipts by broad outlay share, plus financing context. | Annual and decade wide CSV exports from `data/derived/income_tax_outlay_model/`. |
| `income-tax-outlay-subfunction-model/` | Prepare Table 3.2 subfunction allocation views for analysis and future UI work. | Annual long and FY2025 top-subfunction CSV exports from `data/derived/income_tax_outlay_subfunction_model/`. |

## Income-Tax Outlay Model Specs

| Spec | Data grain | Recommended role |
|---|---|---|
| `income-tax-outlay-model/annual-stacked-area.vl.json` | Fiscal year, 1940-2025 | Primary annual modeled allocation chart. |
| `income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade bucket | Compact long-run comparison. |
| `income-tax-outlay-model/annual-financing-context-lines.vl.json` | Fiscal year, 1940-2025 | Companion financing context for the annual chart. |
| `income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade bucket | Companion financing context for the decade chart. |

## Reading Order

1. Start with the allocation chart: annual stacked area or decade stacked bar.
2. Read the financing context chart next: borrowed share and income-tax coverage
   explain how much of total outlays was not covered by income-tax receipts.
3. Use the subfunction exports only for drilldown or ranked detail after the
   broad model context is visible.
4. Use the reader packet for public wording:
   `docs/reading/modeled-income-tax-outlays.md`.

## Wording Rule

Use "modeled allocation" or "if allocated by broad outlay share." Do not use
"where income taxes went" or "what income taxes funded" because those phrases
imply legal tracing that the model does not support.

## Deficit Context Rule

Borrowed share is financing context. It should appear beside the allocation
chart, not as a stacked income-tax allocation category.
