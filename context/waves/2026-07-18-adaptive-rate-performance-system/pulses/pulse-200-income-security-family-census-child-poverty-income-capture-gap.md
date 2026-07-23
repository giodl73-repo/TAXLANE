# Pulse 200: Income-security/family Census child poverty and income capture gap

## Intent

Record the open Census domestic child poverty and income source-custody gate
without populating unsupported poverty, SPM, income-unit, floor, solver, or rate
values.

## Changes

- Added a machine-readable capture-gap record for the child poverty/income item
  in the income-security/family closure queue.
- Named candidate Census poverty, SPM, and CPS ASEC source surfaces.
- Added a reader packet, schema note, and validator/test coverage.

## Non-goals

This pulse does not capture Census raw source custody, populate official child
poverty context, populate SPM child poverty context, populate deep poverty or
near-poverty context, define the income-unit perimeter, populate child-poverty
floor values, publish pass/fail findings, select target cost, publish federal
effect, publish gross savings, publish net savings, populate solver input,
calculate rates, publish a public rate card, publish a department-cut
instruction, claim technology savings, or make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
