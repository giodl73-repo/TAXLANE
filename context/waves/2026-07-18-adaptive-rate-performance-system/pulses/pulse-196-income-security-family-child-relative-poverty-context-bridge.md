# Pulse 196: Income-security/family child relative poverty context bridge

## Intent

Close the narrow piece of the income-security/family child-poverty source gate
that can be closed from already captured source custody: international child
relative-poverty context from the OECD IDD panel.

## Changes

- Added a machine-readable bridge from the existing OECD age-relative-poverty
  country panel into the income-security/family lane.
- Preserved the distinction between international relative-poverty context and
  the still-open Census domestic child poverty and income-unit source gate.
- Added a reader packet, schema note, and validator/test coverage.

## Non-goals

This pulse does not capture Census domestic child poverty custody, populate
child-poverty floor values, populate material-hardship floor values, design a
benefit package, publish a take-up model, complete federal/state/local
translation, publish pass/fail findings, select target cost, publish federal
effect, publish gross savings, publish net savings, populate solver input,
calculate rates, publish a public rate card, publish a department-cut
instruction, claim technology savings, or make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
