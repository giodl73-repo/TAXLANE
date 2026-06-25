# Wave: Operating-System Sourcing + Validation

## Goal

Close the last pressure-test P3 (inline sourcing in the public packets) and harden
the data layer so the program-lane reform rules self-enforce in the Rust tooling.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Inline source IDs in public packets | done | Added Sources footers to the three public reading packets, mapping each figure-cluster to ledger source IDs; verified all 14 cited SRC IDs exist in the ledger. |
| 02 | Wire records into Rust validation | done | Added `validate_program_lane_records` to taxlane-tools: enforces the `proposed_reform` allocation gate, ledger-backed `source_ids`, and 100% share invariants across all program-lane record families (85 records). Negative-tested. |

## Validation

```powershell
git diff --check
cargo fmt -p taxlane-tools --check
cargo test -p taxlane-tools
cargo run -p taxlane-tools -- income-tax-outlay validate
```

## Status

Complete.
