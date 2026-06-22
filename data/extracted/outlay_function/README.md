# Outlay Function Extraction

This directory is reserved for draft `outlay_function` records extracted from
OMB Historical Tables.

## Initial Sources

| Source ID | Raw artifact | Use |
|---|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-1-1-FY2027/2026-06-21/hist01z1_fy2027.xlsx` | Fiscal spine and reconciliation. |
| `SRC-OMB-HIST-3-1-FY2027` | `data/raw/omb/SRC-OMB-HIST-3-1-FY2027/2026-06-21/hist03z1_fy2027.xlsx` | Superfunction and function records. |
| `SRC-OMB-HIST-3-2-FY2027` | `data/raw/omb/SRC-OMB-HIST-3-2-FY2027/2026-06-21/hist03z2_fy2027.xlsx` | Function and subfunction records. |

## Current Output

The current Table 3.1 draft emits all actual fiscal years, 1940-2025:

```text
outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl
table-3-1-profile.md
```

Rows follow `docs/data/outlays-lanes-schema.md`. The Table 3.1 extract keeps
the six broad visible outlay rows plus `Total, Federal outlays`, and excludes
FY2026-FY2031 estimates.

The current Table 3.2 proof slice emits National Defense rows for actual fiscal
years, 1962-2025:

```text
outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.national-defense.draft.jsonl
table-3-2-national-defense-profile.md
```

The proof slice keeps subfunction totals for `051`, `053`, and `054`, plus the
parent `050 National Defense` total. It excludes lower component rows under
`051` until the full Table 3.2 hierarchy is represented.

## Planned Output

Use JSONL draft records named by source and observed date:

```text
outlay_function.SRC-OMB-HIST-3-2-FY2027.2026-06-21.draft.jsonl
```

Rows must follow `docs/data/outlays-lanes-schema.md`.

## Review Gates

1. Preserve fiscal-year basis.
2. Preserve exact OMB function and subfunction labels.
3. Reconcile Table 3.1 total outlays to Table 1.1.
4. Reconcile Table 3.2 subfunction totals to parent functions.
5. Keep net interest and undistributed offsetting receipts visible.
