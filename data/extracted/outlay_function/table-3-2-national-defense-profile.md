# Table 3.2 National Defense Profile

## Source Coverage

- Outlay source: `SRC-OMB-HIST-3-2-FY2027`
- Reconciliation source: `SRC-OMB-HIST-3-1-FY2027`
- Fiscal years emitted: 1962-2025
- Year count: 64
- Record count: 256
- Actual/projection treatment: actual years only; TQ and FY2026-FY2031 estimates are excluded.

## Extracted Rows

| Function code | Subfunction code | Source label | Table 3.2 row |
|---|---|---|---:|
| `050` | `051` | 051 Subtotal, Department of Defense-Military | 13 |
| `050` | `053` | 053 Atomic energy defense activities | 14 |
| `050` | `054` | 054 Defense-related activities | 15 |
| `050` | `null` | Total, National Defense | 16 |

## Reconciliation Sample

Amounts are in millions of dollars. Subfunction total is rows 13, 14, and 15.

| Fiscal year | Table 3.1 National Defense | Table 3.2 National Defense | Subfunction total | Table 3.1 diff | Subfunction diff |
|---:|---:|---:|---:|---:|---:|
| 1962 | 52,345 | 52,345 | 52,345 | 0 | 0 |
| 1970 | 81,692 | 81,692 | 81,692 | 0 | 0 |
| 1980 | 133,995 | 133,995 | 133,996 | 0 | 1 |
| 2000 | 294,363 | 294,363 | 294,363 | 0 | 0 |
| 2025 | 916,140 | 916,140 | 916,140 | 0 | 0 |

## Extraction Decisions

- This is a proof slice for function `050 National Defense`, not the full Table 3.2 extraction.
- Rows 6-12 are lower component rows inside subfunction `051`; this proof emits row 13 as the subfunction total instead.
- Parent total row 16 is emitted with `subfunction_code = null` so it can reconcile to Table 3.1.
- No public lane allocation should use these draft rows.
