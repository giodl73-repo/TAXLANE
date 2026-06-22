# Wave: Table 3.2 National Defense Proof

## Goal

Extract a reconciled proof slice from OMB Historical Table 3.2 for National
Defense function and subfunction outlays.

## Thesis

Table 3.2 has a deeper hierarchy than Table 3.1. A narrow National Defense
proof slice lets the extractor prove function/subfunction boundaries,
actual-year handling, and parent reconciliation before expanding to every
function in the workbook.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | National Defense proof slice | done | Added Rust extraction for FY1962-FY2025 National Defense subfunction totals and profile documentation. |

## Success Criteria

- `cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense`
  writes draft JSONL and profile outputs.
- `cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense --check`
  detects stale outputs.
- Rows 13, 14, and 15 reconcile to row 16 within displayed-source rounding.
- Row 16 reconciles to Table 3.1 National Defense.
- TQ and FY2026-FY2031 estimates are excluded.

## Non-Goals

- Do not extract all Table 3.2 functions in this wave.
- Do not emit lower component rows under subfunction `051`.
- Do not apply public-lane allocations.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense --check
cargo run -p taxlane-tools -- outlay-function table-3-1 --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
