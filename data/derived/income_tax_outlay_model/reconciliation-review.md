# Income-Tax Outlay Model Reconciliation Review

## Scope

Review of draft `income_tax_outlay_model` records generated from the FY2027 OMB
Historical Tables.

## Output Reviewed

- `income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl`
- 516 rows
- Fiscal years 1940-2025
- Six broad Table 3.1 categories per fiscal year

## Checks

| Check | Result |
|---|---|
| JSONL parses successfully | Pass |
| Row count equals 86 years times 6 categories | Pass |
| Table 3.1 total outlays reconcile to Table 1.1 total outlays | Pass |
| Broad-category sums reconcile within displayed-million source rounding | Pass |
| Modeled allocations sum to individual income-tax receipts for each year | Pass |
| Every row carries deficit gap and borrowed-share context | Pass |
| Every row is labeled `modeled_not_legal_dedication` | Pass |

## Rounding Treatment

Table 3.1 broad category rows are displayed in millions. In a small number of
years, the six displayed category rows differ from displayed total outlays by
one or two million dollars. The model keeps the displayed total outlays for
deficit context and normalizes the allocation shares over the displayed
broad-category sum so the modeled allocation equals individual income-tax
receipts for each year.

## Review Decision

The draft rows are fit for internal analysis and reader-facing examples that
use modeled-allocation wording. They should not be described as source-reviewed
legal allocation records or as program-level tracing.
