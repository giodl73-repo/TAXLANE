# Pulse 01: Full Table 3.2 Actual-Year Extraction

## Goal

Emit draft `outlay_function` records for Table 3.2 actual-year function,
subfunction, and total-outlays rows.

## Changes

- Add Rust `outlay-function table-3-2` subcommand.
- Emit Table 3.2 draft JSONL records for fiscal years 1962-2025.
- Add `table-3-2-profile.md` with source coverage and reconciliation samples.
- Update outlay-function README current outputs.

## Validation

- `cargo run -p taxlane-tools -- outlay-function table-3-2 --check`
- `cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense --check`
- `cargo run -p taxlane-tools -- outlay-function table-3-1 --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
