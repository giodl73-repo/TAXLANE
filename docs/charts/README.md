# TAXLANE Chart Catalog

## Purpose

This catalog lists chart specifications that visualize TAXLANE data artifacts.
The specs are handoff assets for analysis and future UI work. They do not
change the underlying data or model methods.

## Available Chart Sets

| Chart set | Purpose | Data artifacts |
|---|---|---|
| `income-tax-outlay-model/` | Visualize the modeled allocation of ordinary individual income-tax receipts by broad outlay share, plus financing context. | Annual and decade wide CSV exports from `data/derived/income_tax_outlay_model/`. |
| `income-tax-outlay-subfunction-model/` | Prepare Table 3.2 subfunction allocation views for analysis and future UI work. | Annual long, decade long, and FY2025 top-subfunction CSV exports from `data/derived/income_tax_outlay_subfunction_model/`. |

## Income-Tax Outlay Model Specs

| Spec | Data grain | Recommended role |
|---|---|---|
| `income-tax-outlay-model/annual-stacked-area.vl.json` | Fiscal year, 1940-2025 | Primary annual modeled allocation chart. |
| `income-tax-outlay-model/decade-stacked-bar.vl.json` | Decade bucket | Compact long-run comparison. |
| `income-tax-outlay-model/annual-financing-context-lines.vl.json` | Fiscal year, 1940-2025 | Companion financing context for the annual chart. |
| `income-tax-outlay-model/decade-financing-context-lines.vl.json` | Decade bucket | Companion financing context for the decade chart. |

## Income-Tax Subfunction Model Specs

| Spec | Data grain | Recommended role |
|---|---|---|
| `income-tax-outlay-subfunction-model/fy2025-top-subfunctions.vl.json` | FY2025 ranked subfunction | Compact drilldown into the largest modeled subfunction allocations. |
| `income-tax-outlay-subfunction-model/selected-subfunction-trends.vl.json` | Fiscal year by selected subfunction, 1962-2025 | Trend comparison for major subfunctions after broad-model context is visible. |
| `income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json` | Decade by ranked subfunction | Compact long-run drilldown for the largest modeled subfunction allocations in each decade. |

## Reading Order

1. Start with the allocation chart: annual stacked area or decade stacked bar.
2. Read the financing context chart next: borrowed share and income-tax coverage
   explain how much of total outlays was not covered by income-tax receipts.
3. Use the subfunction exports only for drilldown or ranked detail after the
   broad model context is visible.
4. Use the broad reader packet for public wording:
   `docs/reading/modeled-income-tax-outlays.md`.
5. Use the subfunction reader packet for any Table 3.2 drilldown wording:
   `docs/reading/modeled-income-tax-subfunction-outlays.md`.

## Wording Rule

Use "modeled allocation" or "if allocated by broad outlay share." Do not use
"where income taxes went" or "what income taxes funded" because those phrases
imply legal tracing that the model does not support.

For subfunction charts, use "if allocated by OMB Table 3.2 subfunction outlay
shares." Do not describe a subfunction label as a program receipt or a legal
funding destination.

## Deficit Context Rule

Borrowed share is financing context. It should appear beside the allocation
chart, not as a stacked income-tax allocation category.

Subfunction charts inherit this rule. A public display of a subfunction chart
needs nearby broad-model financing context or a direct link back to the broad
modeled-outlay packet.

## Partial-Decade Rule

The subfunction decade chart includes partial buckets for the 1960s
(FY1962-FY1969) and 2020s (FY2020-FY2025). Label those buckets as partial and
do not compare them as if each contains ten fiscal years.
