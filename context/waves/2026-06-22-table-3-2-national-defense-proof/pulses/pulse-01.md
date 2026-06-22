# Pulse 01: National Defense Proof Slice

## Goal

Emit a reconciled Table 3.2 proof slice for National Defense actual-year
function and subfunction outlays.

## Changes

- Add Rust `outlay-function table-3-2-national-defense` subcommand.
- Emit National Defense records for fiscal years 1962-2025.
- Add `table-3-2-national-defense-profile.md` with reconciliation samples.
- Update outlay-function README current outputs.

## Validation

- `cargo run -p taxlane-tools -- outlay-function table-3-2-national-defense --check`
- `cargo run -p taxlane-tools -- outlay-function table-3-1 --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
