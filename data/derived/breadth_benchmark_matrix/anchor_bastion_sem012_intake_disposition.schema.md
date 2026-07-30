# ANCHOR/BASTION SEM-012 intake disposition schema

## Product question

Can Taxlane use the held ANCHOR retirement/disability and BASTION defense
evidence-pack candidates to change the OAS or DEF fiscal dispositions or the
current analytical rate schedule?

## Record identity

- `record_id`
- `record_family`
- `version`
- `status`
- `as_of_date`

## Required input custody

Each `inputs[]` member records the producing repository, exact commit,
replay command, schema, lane identifier, source digest, scenario version,
candidate state, emission state, authority boundary, and unresolved holds.
Inputs remain producer-owned and are not silently rewritten into Taxlane
evidence.

## Compatibility assessment

`shared_contract_assessment` separates:

- common candidate fields that both producers already supply;
- lane-specific evidence that must remain typed rather than flattened;
- Taxlane admission fields absent from both candidate schemas; and
- the disposition of the proposed shared contract.

Semantic compatibility does not mean fiscal admissibility.

## Lane disposition

Each `lane_dispositions[]` member must name:

- the Taxlane track and accounting rail;
- useful evidence carried by the candidate;
- unresolved Taxlane admission gates;
- whether the existing track reopening condition was triggered;
- admitted FY2026 primary reduction; and
- whether the current track disposition changed.

An input with `admission_state=held`, `emitted=false`, synthetic source
custody, or missing current-law cash effects cannot produce an admitted fiscal
effect.

## Portfolio invariants

The record passes only when:

- both producer authority objects deny Taxlane admission, allocation, rate
  change, and release;
- no held producer pack is treated as emitted;
- OAS remains a dedicated-solvency overlay;
- DEF remains at reviewed zero admission unless every existing gate closes;
- admitted FY2026 primary reduction remains zero;
- PAY remains non-additive and NET remains endogenous;
- no rate recomputation is claimed without a changed admitted input; and
- the existing analytical schedules remain unchanged when the fiscal input is
  unchanged.

## Validation

```powershell
Get-Content data/derived/breadth_benchmark_matrix/anchor_bastion_sem012_intake_disposition.v1.draft.json | ConvertFrom-Json | Out-Null
git diff --check
cargo test --workspace
```

