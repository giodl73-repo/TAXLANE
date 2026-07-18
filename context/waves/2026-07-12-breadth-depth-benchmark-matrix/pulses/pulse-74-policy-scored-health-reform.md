# Pulse 74 — Policy-scored central health reform and stress

## Decision

Create the health policy-score gate for `central_reform` and `stress` without
inventing a federal score from private-insurance sensitivity arithmetic.

## Added

- `health_policy_scored_reform_path.v1.draft.json`.
- Corresponding schema.
- Public reader.
- Validator checks that central and stress federal cash-flow fields remain null
  and solver-ineligible.

## Boundary

The record does not choose a federal policy instrument, score savings, publish a
target cost, publish a federal budget effect, or map the aggressive
private-insurance payment sensitivity to fiscal stress.

`stress` is reserved for the same selected policy under adverse realization:
weaker payment effect, higher utilization, higher implementation cost, access
remediation, weaker receipts, and higher interest rates where relevant.

## Next Gate

Select and score a specific federal health policy instrument with segmentation,
phase-in, behavior, transition/admin/enforcement costs, incidence, provenance,
and all outcome floors passed.
