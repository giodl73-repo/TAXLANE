# COVENANT H.R. 2137 candidate intake disposition schema

## Product question

Does COVENANT's first official candidate pack close the existing H.R. 2137 VET
admission gaps or change Taxlane's rate result?

## Producer input

`producer_input` records the exact repository commit, replay command, pack hash,
fourteen-section contract result, candidate identity, legislative status,
readiness flags, and denied authorities. Taxlane does not rewrite producer
evidence or inherit domain authority.

## Fiscal path

`official_fiscal_path` keeps gross claims/service costs, the pension/Medicaid
interaction, direct outlays, appropriation outlays, and the combined score
separate. The signed pension/Medicaid interaction is already included in direct
outlays and cannot be relabeled as claims efficiency or added twice.

## Disposition

`taxlane_assessment` distinguishes new producer corroboration from a new
candidate identity. `level_2_requirements` lists the evidence still required
for admission. `portfolio_result` must retain zero admitted FY2026 reduction,
the $813.727B target, and the current schedules unless all applicable gates pass.

## Invariants

- The exact COVENANT pack replays and exposes all fourteen sections.
- Its candidate identity matches Taxlane's existing H.R. 2137 candidate.
- The ten-year arithmetic is `173 + (-145) = 28` and `-108 + 136 = 28`.
- The reported bill is not treated as enacted or appropriated.
- Outcomes, floors, delivery, admission, and savings remain false or null.
- VET does not reopen and no rate is recomputed.

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/covenant_hr2137_candidate_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
cargo test --workspace
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run -p taxlane-tools -- income-tax-outlay manifest --check
git diff --check
```
