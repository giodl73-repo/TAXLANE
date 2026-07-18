# Pulse 72 — Freeze Accounting And Rate Definitions

## Decision

Freeze the accounting signs, fiscal identities, reserve treatment, and rate
denominator names before building the integrated solver.

## Added

- A machine-readable fiscal accounting and rate-definition record.
- A schema note and public reader for the frozen definitions.
- A link from the target-cost contract to the accounting definition record.
- Validator checks that preserve the two denominator names and prohibit calling
  post-dedicated-receipt residual shares a "share of every tax dollar."

## Boundary

This pulse does not build a solver, select rates, score federal savings, or open
any balanced-budget claim. Reserve numeric parameters, current-law paths,
dedicated receipt bases, residual general-fund requirements, endogenous net
interest, and distributional effects remain blockers.

## Frozen Definitions

- all-receipt funding share = gross program cost / total funded federal cost;
- residual general-fund requirement share = residual general-fund need / total
  residual general-fund need.

A value calculated after subtracting dedicated receipts is not "share of every
tax dollar."
