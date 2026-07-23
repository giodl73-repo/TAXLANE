# Pulse 199: Income-security/family food hardship and nutrition capture gap

## Intent

Record the open USDA food hardship and nutrition source-custody gate without
populating unsupported food-security, SNAP, material-hardship, benefit-package,
solver, or rate values.

## Changes

- Added a machine-readable capture-gap record for the food
  hardship/nutrition item in the income-security/family closure queue.
- Named candidate USDA ERS food-security and USDA FNS SNAP source surfaces.
- Added a reader packet, schema note, and validator/test coverage.

## Non-goals

This pulse does not capture USDA raw source custody, populate ERS food-security
context, populate FNS SNAP context, complete a nutrition-program boundary,
populate material-hardship floor values, populate food-security floor values,
publish benefit-package context, design a benefit package, publish a take-up
model, publish pass/fail findings, select target cost, publish federal effect,
publish gross savings, publish net savings, populate solver input, calculate
rates, publish a public rate card, publish a department-cut instruction, claim
technology savings, or make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
