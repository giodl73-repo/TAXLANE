# Receipt Source Extraction

This directory contains `receipt_source` records extracted from OMB Historical
Tables and reviewed budget-concept sources.

## Initial Sources

| Source ID | Raw artifact | Use |
|---|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx` | Fiscal spine and reconciliation. |
| `SRC-OMB-HIST-2-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-2-1-FY2027/2026-06-21/hist02z1_fy2027.xlsx` | Receipt amount records. |
| `SRC-OMB-HIST-2-2-FY2027` | `data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx` | Receipt share records. |
| `SRC-OMB-HIST-2-4-FY2027` | `data/raw/omb/SRC-OMB-HIST-2-4-FY2027/2026-06-21/hist02z4_fy2027.xlsx` | Social-insurance and excise subcomponent detail. |

## Planned Output

Use JSONL records named by source, observed date, and review state:

```text
receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.draft.jsonl
receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.source-reviewed.jsonl
receipt_source.SRC-OMB-HIST-2-2-FY2027.2026-06-21.draft.jsonl
receipt_source.SRC-OMB-HIST-2-4-FY2027.2026-06-21.draft.jsonl
```

Rows must follow `docs/data/receipts-funds-schema.md`.

## Review Gates

1. Preserve fiscal-year basis.
2. Preserve exact OMB row labels.
3. Keep amount and percentage rows separate.
4. Reconcile Table 2.1 total receipts to Table 1.1 within displayed precision.
5. Keep social-insurance receipts separate from individual income taxes.

## Current Output

- `receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.source-reviewed.jsonl`
- `first-slice-reconciliation.md`
- `allocation-review.md`
- `table-2-4-profile.md`
