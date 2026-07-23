# Pulse 198: Income-security/family childcare and family-service capture gap

## Intent

Record the open HHS/ACF childcare and family-service source-custody gate without
populating unsupported childcare access, TANF, delivery-feasibility, solver, or
rate values.

## Changes

- Added a machine-readable capture-gap record for the childcare/family-service
  item in the income-security/family closure queue.
- Named candidate ACF CCDF and TANF source surfaces.
- Added a reader packet, schema note, and validator/test coverage.

## Non-goals

This pulse does not capture HHS/ACF raw source custody, populate CCDF or TANF
context, populate family-service context, populate childcare-access floor
values, publish work/care transition context, publish delivery-feasibility
context, design a benefit package, publish a take-up model, publish pass/fail
findings, select target cost, publish federal effect, publish gross savings,
publish net savings, populate solver input, calculate rates, publish a public
rate card, publish a department-cut instruction, claim technology savings, or
make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
