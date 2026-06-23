# Pulse 01: Core Crate Boundary

## Goal

Add `taxlane-core` and close VTRACE WP-TAX-003.

## Changes

- Add `crates/taxlane-core` as a workspace crate.
- Add typed artifact metadata validation and accountability evidence record validation.
- Add tests for accountability evidence boundary behavior.
- Wire `taxlane-tools` manifest validation through `taxlane-core`.
- Add architecture review and VTRACE closeout updates.

## Validation

- `cargo test`
- `cargo fmt --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`
- `git diff --check`

## Status

Done.
