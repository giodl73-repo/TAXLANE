# Wave: Table 3.1 Outlay Function Expansion

## Goal

Expand the draft OMB Historical Table 3.1 outlay-function extraction from the
initial 1940-1942 slice to all actual fiscal years.

## Thesis

The budget-history base needs a complete annual broad-function extract before
Table 3.2 subfunctions are joined. Table 3.1 provides a stable broad-function
spine that can be reconciled directly to Table 1.1 total outlays.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Table 3.1 actual-year extraction | done | Added Rust extraction for FY1940-FY2025 broad outlay functions and profile documentation. |

## Success Criteria

- `cargo run -p taxlane-tools -- outlay-function table-3-1` writes draft JSONL
  and profile outputs.
- `cargo run -p taxlane-tools -- outlay-function table-3-1 --check` detects
  stale outputs.
- Records preserve the six visible Table 3.1 broad rows plus total federal
  outlays.
- Total federal outlays reconcile to OMB Historical Table 1.1.
- FY2026-FY2031 estimates are excluded from this actual-year draft.

## Non-Goals

- Do not extract Table 3.2 subfunctions in this wave.
- Do not apply public-lane allocations.
- Do not make reform claims from the draft rows.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- outlay-function table-3-1 --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
