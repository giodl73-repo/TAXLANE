# Pulse 92 — Transportation Pilot Floor Indicator Contract

## Scope

Create the floor indicator contract for the selected transportation asset
maintenance and safety pilot.

## Artifacts

- `data/derived/breadth_benchmark_matrix/transportation_pilot_floor_indicator_contract.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/transportation_pilot_floor_indicator_contract.schema.md`
- `docs/reading/transportation-pilot-floor-indicator-contract.md`

## Boundary

This pulse creates a floor indicator contract, not floor thresholds or floor
pass findings. It does not capture source bytes, close custody, complete a
baseline path, run a simulator, set target costs, calculate rates, publish
public cards, estimate savings, find waste or fraud, instruct department cuts,
claim technology savings, create modernization or stress paths, produce solver
results, or make a balanced-budget claim.

## Acceptance coverage

- Requires every lower-cost scenario to pass all floor families before target
  cost use.
- Names access/coverage, quality/safety, equity/distribution,
  adequacy/resilience, delivery-feasibility, and transportation
  asset-condition floor families.
- Keeps threshold values and observed values null.
- Keeps floor pass flags false.
- Keeps indicator records empty until source custody and threshold decisions
  exist.
- Requires source custody, period, unit, perimeter, missingness, and
  federal/state/local translation fields for future indicator records.
- Leaves blocked gates false, outputs null, and public claim booleans false.
