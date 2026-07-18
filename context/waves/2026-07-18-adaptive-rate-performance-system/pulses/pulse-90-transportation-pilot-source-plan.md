# Pulse 90 — Transportation Pilot Source Plan

## Scope

Create the source-plan contract for the selected transportation asset
maintenance and safety pilot.

## Artifacts

- `data/derived/breadth_benchmark_matrix/transportation_pilot_source_plan.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/transportation_pilot_source_plan.schema.md`
- `docs/reading/transportation-pilot-source-plan.md`

## Boundary

This pulse names source families and custody requirements only. It does not
capture source bytes, close custody, create the baseline path, set floor
thresholds, create modernization or stress paths, run a simulator, set target
costs, calculate rates, publish a public card, estimate savings, find waste or
fraud, instruct department cuts, claim technology savings, produce solver
results, or make a balanced-budget claim.

## Acceptance coverage

- Names official source families for federal outlays, trust funds, DOT/FHWA,
  NHTSA/BTS, Census state/local finance, GAO/OIG controls, and ITF/OECD context.
- Requires retrieval metadata, raw bytes, byte count, SHA-256, local raw path,
  matched period/unit/perimeter, and missingness disclosure.
- Keeps trust funds, explicit transfers, credited offsets, and state/local/
  private/user-financed context separate.
- Leaves planned floor families unthresholded with null values and false pass
  flags.
- Keeps all downstream outputs null and all public claim booleans false.
