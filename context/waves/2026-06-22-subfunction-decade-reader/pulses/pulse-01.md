# Pulse 01: Decade Reader Guardrails

## Goal

Add reader-facing wording for the subfunction decade rollup.

## Changes

- Add the decade-long CSV to the subfunction reader packet artifact table.
- Add a decade drilldown section with generated top-subfunction shares.
- Label the 1960s and 2020s as partial-decade buckets.
- Extend the role review for decade chart and rollup use.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
