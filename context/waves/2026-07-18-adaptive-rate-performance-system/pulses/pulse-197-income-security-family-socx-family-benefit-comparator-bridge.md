# Pulse 197: Income-security/family SOCX family-benefit comparator bridge

## Intent

Close the narrow piece of the income-security/family international comparator
source gate that can be closed from already captured source custody: OECD SOCX
public family-benefit total, cash, and in-kind service context.

## Changes

- Added a machine-readable bridge from the existing SOCX old-age/family country
  panel into the income-security/family lane.
- Preserved the distinction between SOCX family-benefit context and the
  still-open broader comparator lineage for tax credits, childcare
  participation, ESSPROS, ILO, missing countries, and child-outcome linkage.
- Added a reader packet, schema note, and validator/test coverage.

## Non-goals

This pulse does not complete international comparator lineage, populate
tax-credit composition, populate childcare participation context, add ESSPROS or
ILO context, link child outcomes, select target cost, publish federal effect,
publish gross savings, publish net savings, populate solver input, calculate
rates, publish a public rate card, publish a department-cut instruction, claim
technology savings, or make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
