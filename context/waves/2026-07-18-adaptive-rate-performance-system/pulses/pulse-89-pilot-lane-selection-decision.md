# Pulse 89 — Pilot Lane Selection Decision

## Scope

Select the first pilot lane for scaffold work after role review.

## Artifacts

- `data/derived/breadth_benchmark_matrix/pilot_lane_selection_decision.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/pilot_lane_selection_decision.schema.md`
- `docs/reading/pilot-lane-selection-decision.md`
- `reviews/2026-07-18-pilot-lane-selection-decision-role-review.md`

## Decision

Select transportation asset maintenance and safety under the
`transportation-infrastructure` lane for scaffold work only.

## Boundary

This pulse does not run a simulator, set target costs, calculate rates, publish
public rate cards, propose taxes, estimate savings, find waste or fraud,
instruct department cuts, claim technology savings, set outcome-floor
thresholds, produce solver results, or make a balanced-budget claim.

## Acceptance coverage

- Selects only a bounded pilot for source and scaffold work.
- Defers disaster reserve operations and claims-processing modernization without
  rejecting them.
- Preserves excluded first pilots.
- Keeps simulator readiness, public-claim allowance, and all public claim
  booleans false except the internal scaffold-selection flag.
