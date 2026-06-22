# Income-Tax Outlay Subfunction Model Reconciliation Review

## Scope

Review of draft `income_tax_outlay_subfunction_model` records and chart-ready
exports generated from the FY2027 OMB Historical Tables.

## Output Reviewed

- `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.draft.jsonl`
- `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.annual-long.csv`
- `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv`
- `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv`
- 4,691 annual subfunction rows
- Fiscal years 1962-2025
- 521 decade rollup rows
- 25 FY2025 ranked rows

## Checks

| Check | Result |
|---|---|
| JSONL parses successfully | Pass |
| Annual CSV row count matches canonical annual model rows | Pass |
| Annual modeled allocations sum back to individual income-tax receipts for each fiscal year | Pass |
| Maximum annual allocation residual after CSV rounding is below one dollar in source units | Pass |
| Decade rollup shares sum to 100 percent for each decade bucket | Pass |
| FY2025 ranked export contains the expected 25 top-subfunction rows | Pass |
| Every annual row is labeled `modeled_not_legal_dedication` | Pass |

## Coverage Notes

The actual-year subfunction model starts in FY1962 because that is the Table 3.2
subfunction overlap with the individual income-tax receipt source. The decade
rollup therefore has a partial 1960s bucket, and the current actual-year model
has a partial 2020s bucket ending in FY2025.

The source profile and chart-ready views should be read at their own grain. The
profile records extraction coverage and sample reconciliation; the chart-ready
annual export records emitted modeled rows that passed allocation checks.

## Review Decision

The draft subfunction rows and chart-ready exports are fit for internal
analysis and reader-facing drilldown examples that use modeled-allocation
wording.

They should not be described as source-reviewed legal allocation records,
program financing, taxpayer-dollar tracing, or statutory dedication.
