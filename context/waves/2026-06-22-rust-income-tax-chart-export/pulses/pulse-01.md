# Pulse 01: Rust Chart CSV Exporter

## Goal

Move chart-ready CSV export from Python to Rust.

## Changes

- Add Rust `income-tax-outlay export` subcommand.
- Remove Python `export_chart_views.py`.
- Update model README and manifest regeneration order.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay export --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
