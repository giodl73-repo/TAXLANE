# Outlay Program Extraction

This directory is reserved for draft `outlay_program` records extracted from
OMB agency and program historical tables.

## Initial Sources

| Source ID | Raw artifact | Use |
|---|---|---|
| `SRC-OMB-HIST-4-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-4-1-FY2027/2026-06-21/hist04z1_fy2027.xlsx` | Agency-level spending context. |
| `SRC-OMB-HIST-8-5-FY2027` | `data/raw/omb/SRC-OMB-HIST-8-5-FY2027/2026-06-21/hist08z5_fy2027.xlsx` | Mandatory and related program records. |
| `SRC-OMB-HIST-8-7-FY2027` | `data/raw/omb/SRC-OMB-HIST-8-7-FY2027/2026-06-21/hist08z7_fy2027.xlsx` | Discretionary program records. |
| `SRC-OMB-HIST-11-3-FY2027` | `data/raw/omb/SRC-OMB-HIST-11-3-FY2027/2026-06-21/hist11z3_fy2027.xlsx` | Payments-for-individuals program context. |

## Planned Output

Use JSONL draft records named by source and observed date:

```text
outlay_program.SRC-OMB-HIST-8-5-FY2027.2026-06-21.draft.jsonl
outlay_program.SRC-OMB-HIST-8-7-FY2027.2026-06-21.draft.jsonl
outlay_program.SRC-OMB-HIST-11-3-FY2027.2026-06-21.draft.jsonl
```

Rows must follow `docs/data/outlays-lanes-schema.md`.

## Review Gates

1. Preserve exact OMB program and agency labels.
2. Keep agency, function, and program views separate.
3. Distinguish mandatory, discretionary, net interest, offsetting, and mixed
   records.
4. Do not treat program records as complete government-wide totals without
   reconciliation.
