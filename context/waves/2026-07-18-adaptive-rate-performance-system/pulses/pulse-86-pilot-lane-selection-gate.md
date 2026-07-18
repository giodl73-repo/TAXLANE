# Pulse 86 — Pilot Lane Selection Gate

## Scope

Create the role-review gate for selecting the first deterministic simulator
pilot.

## Artifacts

- `data/derived/breadth_benchmark_matrix/pilot_lane_selection_gate.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/pilot_lane_selection_gate.schema.md`
- `docs/reading/pilot-lane-selection-gate.md`

## Boundary

This pulse does not choose the final pilot lane. It does not publish a public
rate card, statutory rate, effective rate, tax proposal, savings estimate,
waste finding, fraud finding, department cut, technology-savings claim, solver
result, or balanced-budget claim.

## Acceptance coverage

- Names pilot-selection criteria.
- Lists recommended initial candidates without selecting them.
- Excludes Social Security, Medicare, broad health, veterans commitments, and
  immediate normative distribution choices as first pilots.
- Blocks final pilot selection until role review clears normative and source
  conflicts.
