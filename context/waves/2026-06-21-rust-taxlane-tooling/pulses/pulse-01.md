# Pulse 01: Rust Validation Runner

## Goal

Add the first Rust Taxlane tooling command for income-tax outlay validation.

## Changes

- Add root Cargo workspace.
- Add `tools/taxlane` Rust CLI crate.
- Update income-tax outlay model docs to prefer the Rust validation command.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
