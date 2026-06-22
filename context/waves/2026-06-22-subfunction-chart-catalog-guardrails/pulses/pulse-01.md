# Pulse 01: Catalog Guardrails

## Goal

Update the chart catalog so subfunction chart handoffs use the current reader
guardrails.

## Changes

- Add the subfunction reader packet to chart reading order.
- Add a Table 3.2 subfunction wording rule.
- Require broad financing context for public subfunction chart use.
- Add a partial-decade rule for the subfunction decade chart.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
