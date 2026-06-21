# OMB Fund Group First Slice Reconciliation

## Source

- Fund-group source: `SRC-OMB-HIST-1-4-FY2027`
- Fiscal spine: `SRC-OMB-HIST-1-1-FY2027`
- Schema: `docs/data/receipts-funds-schema.md`

## Slice

The first draft fund-group extraction covers fiscal year 1934 from OMB
Historical Table 1.4:

- Receipts by total, federal funds, trust funds, and interfund transactions.
- Outlays by total, federal funds, trust funds, and interfund transactions.
- Surplus/deficit by total, federal funds, and trust funds.

All records are `draft-extracted`.

## Reconciliation

| Fiscal year | Measure | Table 1.4 total | Table 1.1 total | Difference |
|---:|---|---:|---:|---:|
| 1934 | Receipts | 2,955 | 2,955 | 0 |
| 1934 | Outlays | 6,541 | 6,541 | 0 |
| 1934 | Surplus/deficit | -3,586 | -3,586 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- `interfund-transactions` is preserved as its own source column.
- `legal_dedication` remains `unknown` for every row.
- `appropriation_required` remains `unknown` for every row.
- `budget_concept_source_id` points to `SRC-OMB-AP-13-FUNDS-FY2027` as the
  concept source needed before interpretation.
- No receipt-source category is linked to a fund group in this pulse.
