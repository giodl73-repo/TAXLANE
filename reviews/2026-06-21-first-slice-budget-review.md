# First-Slice Budget Review

## Scope

This Budget Accountant review covers the first extracted slices:

| Family | Draft records | Rows |
|---|---|---:|
| `rates_timeline` | `data/extracted/rates_timeline/SRC-IRS-SOI-HT23/rates_timeline.SRC-IRS-SOI-HT23.2026-06-21.draft.jsonl` | 12 |
| `receipt_source` | `data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.draft.jsonl` | 18 |
| `outlay_function` | `data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl` | 21 |

## Budget Accountant Findings

| Check | Result |
|---|---|
| Rates use tax-year basis | Pass: all rates rows use `year_basis = "tax_year"`. |
| Receipts use fiscal-year basis | Pass: all receipt rows use `year_basis = "fiscal_year"`. |
| Outlays use fiscal-year basis | Pass: all outlay rows use `year_basis = "fiscal_year"`. |
| Amount rows have null percentages | Pass: receipt and outlay amount rows have `amount` values and null `percent`. |
| Rates summary rows are caveated | Pass: IRS rows state they are lowest/highest bracket summary rows, not complete statutory schedules. |
| Receipt categories remain separate | Pass: individual income taxes, corporation income taxes, and social-insurance receipts are separate categories. |
| Receipt allocation not overclaimed | Pass: receipt rows keep `allocation_status = "unknown"`. |
| Net interest remains visible | Pass: net interest appears as its own outlay function. |
| Offsetting remains visible | Pass: undistributed offsetting receipts are negative and explicitly labeled. |
| Totals reconcile | Pass: receipt and outlay reconciliation notes show zero displayed difference to Table 1.1 for the reviewed years. |
| Public allocation blocked | Pass: no reviewed record claims taxpayer-dollar tracing or legal dedication. |

## Validation Evidence

The review script parsed all first-slice JSONL files and checked budget
semantics.

Result:

```text
{'rates_timeline.SRC-IRS-SOI-HT23.2026-06-21.draft.jsonl': 12, 'receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.draft.jsonl': 18, 'outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl': 21}
budget_semantics ok
```

## Decision

The reviewed first slices are eligible for `budget-reviewed` status for the
specific rows covered by the source reviews and this budget review.

This does not approve a taxpayer-facing receipt, lane allocation, legal
dedication claim, or full-table extraction.

## Remaining Blocks

- Full IRS rates extraction needs footnote parsing.
- Receipt allocation needs fund-group and budget-concept review.
- Outlay lane use needs function/subfunction crosswalk review.
- Public taxpayer allocation needs allocation method and deficit treatment.
