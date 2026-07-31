# COVENANT candidate intake

## Outcome

Replay COVENANT's first official H.R. 2137 candidate pack, reconcile it with
Taxlane's existing VET candidate, and determine whether its new producer-owned
evidence closes the track-reopening gate.

## Result

- exact producer commit and pack hash custodied;
- all fourteen Taxlane sections present;
- existing H.R. 2137 identity matched, with no duplicate fiscal effect;
- $173M gross service cost and -$145M pension/Medicaid interaction kept
  separate, producing a $28M combined cost;
- Level 2 outcome, floor, incidence, implementation, appropriation, and
  delivery evidence remains incomplete;
- zero FY2026 reduction admitted; and
- target and rate schedules unchanged.

Fixed point: `pass_with_risk`, with zero open P1/P2 findings. Review:
[intake-role-review.md](reviews/intake-role-review.md).

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/covenant_hr2137_candidate_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
cargo test --workspace
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run -p taxlane-tools -- income-tax-outlay manifest --check
git diff --check
```
