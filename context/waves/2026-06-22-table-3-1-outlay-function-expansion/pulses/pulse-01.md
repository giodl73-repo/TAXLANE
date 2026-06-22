# Pulse 01: Table 3.1 Actual-Year Extraction

## Goal

Emit draft outlay-function records for all actual-year OMB Historical Table 3.1
broad outlay functions.

## Changes

- Add Rust `outlay-function table-3-1` subcommand.
- Emit Table 3.1 draft JSONL records for fiscal years 1940-2025.
- Add `table-3-1-profile.md` with source coverage and reconciliation samples.
- Update outlay-function README current outputs and mark the first-slice note as
  historical.

## Validation

- `cargo run -p taxlane-tools -- outlay-function table-3-1 --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
