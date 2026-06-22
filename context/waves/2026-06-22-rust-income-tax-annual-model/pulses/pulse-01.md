# Pulse 01: Rust Annual Model Builder

## Goal

Move annual income-tax outlay model generation from Python to Rust.

## Changes

- Add Rust `income-tax-outlay model` subcommand.
- Parse OMB XLSX workbook internals in Rust.
- Remove Python `build_income_tax_outlay_model.py`.
- Update model README and manifest regeneration order.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay model --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
