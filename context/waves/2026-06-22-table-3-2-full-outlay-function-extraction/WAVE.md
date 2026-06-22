# Wave: Table 3.2 Full Outlay Function Extraction

## Goal

Extract the actual-year OMB Historical Table 3.2 function and subfunction rows
into draft `outlay_function` JSONL records.

## Thesis

The National Defense proof slice established the Table 3.2 hierarchy and
rounding behavior. The next complete slice should generalize that parser across
all Table 3.2 functions while preserving source labels, excluding component
rows that are not OMB subfunctions, and reconciling annual totals to Table 3.1.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Full Table 3.2 actual-year extraction | done | Added Rust extraction for FY1962-FY2025 Table 3.2 subfunction, parent-total, and total-outlays records. |

## Success Criteria

- `cargo run -p taxlane-tools -- outlay-function table-3-2` writes draft JSONL
  and profile outputs.
- `cargo run -p taxlane-tools -- outlay-function table-3-2 --check` detects
  stale outputs.
- Table 3.2 total outlays reconcile to Table 3.1 total outlays.
- Emitted function totals reconcile to emitted subfunctions within
  displayed-source rounding.
- TQ and FY2026-FY2031 estimates are excluded.

## Non-Goals

- Do not emit lower component rows without OMB subfunction codes.
- Do not emit parenthetical on/off-budget split rows.
- Do not apply public-lane allocations.

## Validation

Run:

```powershell
cargo run -p taxlane-tools -- outlay-function table-3-2 --check
cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense --check
cargo run -p taxlane-tools -- outlay-function table-3-1 --check
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo fmt --check
cargo test
git diff --check
```
