# Pulse 87 — Deterministic Annual Update Simulator

## Scope

Create the contract for a narrow deterministic annual-update simulator.

## Artifacts

- `data/derived/breadth_benchmark_matrix/deterministic_annual_update_simulator_contract.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/deterministic_annual_update_simulator_contract.schema.md`
- `docs/reading/deterministic-annual-update-simulator-contract.md`

## Boundary

This pulse does not run a simulator, perform optimization, select a pilot lane,
publish a public rate card, calculate rates, score savings, issue department
cuts, or make a balanced-budget claim.

## Acceptance coverage

- Requires baseline, modernization, and stress paths.
- Prohibits optimization.
- Treats net interest and fund effects according to the solver contract.
- Keeps lower-rate recognition blocked unless all floors pass.
