# Pulse 91 — Transportation Pilot Baseline Path Contract

## Scope

Create the current-law baseline path contract for the selected transportation
asset-maintenance and safety pilot.

## Artifacts

- `data/derived/breadth_benchmark_matrix/transportation_pilot_baseline_path_contract.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/transportation_pilot_baseline_path_contract.schema.md`
- `docs/reading/transportation-pilot-baseline-path-contract.md`

## Boundary

This pulse creates a baseline contract, not a completed baseline path. It does
not capture source bytes, close custody, run a simulator, set target costs,
calculate rates, publish public cards, estimate savings, find waste or fraud,
instruct department cuts, claim technology savings, create modernization or
stress paths, set floor thresholds, produce solver results, or make a
balanced-budget claim.

## Acceptance coverage

- Requires FY2025-FY2035 current-law annual rows with zero reform deltas.
- Reuses the existing FY2025 transportation depth-card anchor and verifies the
  component sum.
- Requires gross outlays, implementation/admin outlays, credited offsets,
  dedicated receipts, explicit general-fund transfers, reserve contribution,
  net cash requirement, fund balance change, source metadata, byte count, and
  SHA-256 in future annual rows.
- Keeps trust funds separate and requires explicit transfers and offsets.
- Leaves baseline rows empty, blocked gates false, outputs null, and public
  claim booleans false.
