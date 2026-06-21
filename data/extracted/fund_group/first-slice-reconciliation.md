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

All records are `source-reviewed`.

## Reconciliation

| Fiscal year | Measure | Table 1.4 total | Table 1.1 total | Difference |
|---:|---|---:|---:|---:|
| 1934 | Receipts | 2,955 | 2,955 | 0 |
| 1934 | Outlays | 6,541 | 6,541 | 0 |
| 1934 | Surplus/deficit | -3,586 | -3,586 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- `interfund-transactions` is preserved as its own source column.
- `legal_dedication` uses reviewed AP13 concept support: `mixed` for total and
  federal-funds rows, `dedicated` for trust-funds rows, and `unknown` for
  interfund transactions.
- `appropriation_required` is `mixed` for total and federal-funds rows and
  remains `unknown` for trust-funds and interfund-transaction rows.
- `budget_concept_source_id` points to `SRC-OMB-AP-13-FUNDS-FY2027` as the
  concept source used for interpretation.
- No receipt-source category is linked to a fund group in this pulse.
