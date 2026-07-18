# Pulse 75 — Integrated fiscal solver

## Decision

Build the narrow deterministic fiscal-solver scaffold before optimization.

## Added

- `integrated_fiscal_solver.v1.draft.json`.
- Corresponding schema.
- Public reader.
- Validator checks for FY2025 17-row reconciliation, denominator separation,
  null blocked outputs, separate funds, debt identity, and primary-change
  interest feedback.

## Boundary

The solver does not optimize, publish balanced rates, score target reform paths,
or make a balanced-budget claim. Assigned-base rates, reserves, fund balances,
distributional effects, and macro feedback remain null or blocked.

Net interest is endogenous in the regression fixture and is not cut directly.

## Next Gate

Load reconciled target paths, trust-fund balance paths, reserve rules, assigned
receipt bases, distribution, macro feedback, and interaction scoring.
