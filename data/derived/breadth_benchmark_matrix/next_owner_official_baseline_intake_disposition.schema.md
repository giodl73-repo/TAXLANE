# Next-owner official baseline intake disposition schema

## Product question

Can Taxlane use the official aggregate LIFELINE, COVENANT, TRIBUNAL, and ENVOY
held packs as lane baselines without treating them as fiscal candidates?

## Input custody

Each `inputs[]` row records the producing repository, exact commit, replay
command, deterministic pack hash, Taxlane track, official source identity,
candidate state, and producer authority. Producer outputs remain producer-owned
and are not rewritten into Taxlane evidence.

## Contract assessment

`contract_assessment` records the fourteen required LaneEvidencePack sections,
the candidate-free baseline class, and the adapter decision. Section
compatibility is not fiscal admissibility. A null `candidate_id` is valid only
for held baseline intake and cannot enter candidate admission.

## Lane disposition

Each `lane_dispositions[]` row names the useful baseline finding, the missing
candidate evidence, the existing reopening result, and the unchanged fiscal
disposition. Official aggregate custody alone cannot create a policy mechanism,
annual scored cash path, service-floor result, implementation plan, net effect,
or overlap allocation.

## Invariants

The record passes only when:

- all four exact producer commits replay;
- every pack has all fourteen contract sections;
- every pack is official aggregate, candidate-free, held, and not admission-ready;
- savings, allocation, rate-change, and release authority remain false;
- no existing track reopening condition is triggered;
- admitted FY2026 primary reduction remains zero; and
- the $813.727 billion target and analytical schedules remain unchanged.

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/next_owner_official_baseline_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
cargo test --workspace
cargo run -p taxlane-tools -- income-tax-outlay validate
cargo run -p taxlane-tools -- income-tax-outlay manifest --check
git diff --check
```
