# TRIBUNAL H.R. 1702 candidate intake disposition schema

## Product question

Does TRIBUNAL's federal district-court capacity envelope establish an
admissible JUS savings candidate or change Taxlane's rate result?

## Producer input and fiscal path

`producer_input` records the exact commit, replay command, pack hash,
fourteen-section result, legislative status, readiness, and denied authorities.
`official_fiscal_path` preserves judgeship capacity, direct compensation,
appropriated operations, annual timing, and CBO's sub-$0.5M FY2025 notation.

## Disposition

`taxlane_assessment` registers a new JUS identity while keeping it separate
from the existing selected-DOJ-grant reduction. Capacity investment is not a
spending reduction and cannot close the earlier candidate's beneficiary and
distribution floors.

## Invariants

- The exact TRIBUNAL pack replays with all fourteen sections.
- `111 + 283 = 394` million dollars.
- FY2026 combines to $8M and FY2035 combines to $71M.
- Capacity is not classified as an outcome or savings result.
- Enactment, outcomes, rights floors, delivery, overlap, and admission remain held.
- JUS does not reopen and no rate is recomputed.

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/tribunal_hr1702_candidate_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
cargo test --workspace
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run -p taxlane-tools -- income-tax-outlay manifest --check
git diff --check
```
