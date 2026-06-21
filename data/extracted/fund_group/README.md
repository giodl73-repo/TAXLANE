# Fund Group Extraction

This directory is reserved for draft `fund_group` records extracted from OMB
Historical Table 1.4 and related budget concept sources.

## Initial Sources

| Source ID | Raw artifact | Use |
|---|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx` | Fiscal spine and reconciliation. |
| `SRC-OMB-HIST-1-4-FY2027` | `data/raw/omb/SRC-OMB-HIST-1-4-FY2027/2026-06-21/hist01z4_fy2027.xlsx` | Fund-group receipt, outlay, and deficit records. |

## Planned Output

Use JSONL draft records named by source and observed date:

```text
fund_group.SRC-OMB-HIST-1-4-FY2027.2026-06-21.draft.jsonl
```

Rows must follow `docs/data/receipts-funds-schema.md`.

## Review Gates

1. Preserve fiscal-year basis.
2. Preserve exact OMB row labels.
3. Reconcile fund-group totals to Table 1.1 within displayed precision.
4. Do not label a fund group legally dedicated without budget-concept support.
5. Do not treat trust fund as private trust ownership.
