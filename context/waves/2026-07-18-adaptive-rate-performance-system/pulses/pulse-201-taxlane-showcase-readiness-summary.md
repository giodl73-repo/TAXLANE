# Pulse 201: Taxlane showcase readiness summary

## Intent

Refresh Taxlane's showable status after the income-security/family Pulses 194
through 200 so the repo has one concise demo path without implying solver,
rate, savings, or balanced-budget readiness.

## Changes

- Updated the income-security/family source-capture status rollup from the
  older all-open Pulse 192 snapshot to the post-Pulse-200 state.
- Marked two narrow contexts as ready: FY2025 federal account-perimeter source
  custody and OECD family-benefit comparator context.
- Kept four capture gaps documented and open: CBO baseline/take-up, Census child
  poverty/income, HHS/ACF childcare/family services, and USDA food/nutrition.
- Added a Taxlane showcase readiness summary and reader packet.
- Added validator/test coverage for the updated rollup and showcase summary.

## Non-goals

This pulse does not complete source capture, build a benefit package model,
publish take-up estimates, select floor thresholds, populate pass/fail findings,
complete federal/state/local translation, populate solver inputs, calculate
rates, publish savings, publish a public rate card, publish a department-cut
instruction, claim technology savings, or make a balanced-budget claim.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run -p taxlane-tools -- income-tax-outlay manifest --check`
- `cargo test`
