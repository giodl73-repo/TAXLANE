# Pulse 01: Subfunction Family Dictionary Entry

## Goal

Align the data dictionary with the generated subfunction model.

## Changes

- Add `income_tax_outlay_subfunction_model` to the record family map.
- Define required emitted fields.
- Add drilldown and legal-allocation guardrails.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
