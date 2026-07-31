# LIFELINE CalFresh Level 2 baseline disposition schema

## Product question

Does LIFELINE's current CalFresh access and rights snapshot complete the
California H.R. 1 candidate's Level 2 gates or change Taxlane's fiscal result?

## Contract

`producer_input` preserves the exact commit, command, hash, fourteen-section
contract, readiness, and denied authority. `current_system_floor_snapshot`
preserves reporting dates and keeps operations, errors, churn, hearings, and
candidate outcomes separate.

## Invariants

- May operations and February errors predate June 1 implementation.
- `43,910 / 60,469` truncates to 7,261 basis points without proving wrongdoing.
- 2020 Q4 churn is stale for the 2026 candidate.
- Hearing flows are not a single cohort or candidate-coded outcomes.
- Lost benefits and caseload contraction are not administrative efficiency.
- No savings, reopening, target change, or rate recomputation occurs.
