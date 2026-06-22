# Pulse 01: Rust Manifest Generator

## Goal

Move income-tax outlay manifest generation from Python to Rust.

## Changes

- Add Rust manifest build/check subcommand.
- Remove Python manifest and Python validation runners.
- Update model README.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
