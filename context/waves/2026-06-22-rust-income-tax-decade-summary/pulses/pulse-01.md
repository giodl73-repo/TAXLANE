# Pulse 01: Rust Decade Summary Generator

## Goal

Move decade summary generation from Python to Rust.

## Changes

- Add Rust `income-tax-outlay summary` subcommand.
- Remove Python `build_decade_summary.py`.
- Update model README and manifest regeneration order.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay summary --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
