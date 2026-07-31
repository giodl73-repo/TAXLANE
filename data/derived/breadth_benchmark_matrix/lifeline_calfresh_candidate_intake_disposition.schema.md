# LIFELINE CalFresh candidate intake disposition schema

## Product question

Does LIFELINE's California CalFresh current-law implementation envelope
establish an admissible ISF savings candidate or change Taxlane's rate result?

## Producer input

`producer_input` records the exact repository commit, replay command, pack
hash, fourteen-section contract result, candidate identity, legal/budget
status, readiness flags, and denied authorities. Taxlane preserves the
producer's distinction between enacted federal law and proposed state funding.

## Fiscal path

`official_fiscal_path` keeps gross implementation administration, the
caseload-related administration offset, net administration, funding incidence,
affected people, and projected lost benefits separate. Neither caseload
contraction nor benefit loss is classified as delivery efficiency.

## Disposition

`taxlane_assessment` records a new ISF identity but classifies it as a
current-law implementation stress envelope, not an alternative reduction. It
does not replace or close the existing school-meal candidate. Level 2 gates
must close before any admission decision can be reconsidered.

## Invariants

- The exact LIFELINE pack replays and exposes all fourteen sections.
- Administration arithmetic is `86800 + (-78600) = 8200` thousand dollars.
- Federal, state, and county net shares sum to the $8.2 million net cost.
- The 302,300 affected people and $758 million benefit loss stay outside the
  administration-savings ledger.
- Outcomes, floors, delivery, overlap, admission, and savings remain held.
- ISF does not reopen and no rate is recomputed.

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/lifeline_calfresh_candidate_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
cargo test --workspace
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run -p taxlane-tools -- income-tax-outlay manifest --check
git diff --check
```
