# Pulse 01: Validation Heading Cleanup

## Goal

Make the broad model README validation sections distinct.

## Changes

- Rename the final command-only validation heading.
- Refresh manifest coverage for the README hash.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
