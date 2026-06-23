# Pulse 01

## Objective

Make applied response delta comparison reusable outside the CLI.

## Changes

- Added core delta row construction and validation.
- Added core tests for changed rows and mismatched row sets.
- Updated CLI delta rendering to consume the core type.
- Updated manifest, VTRACE rows, wave notes, and review evidence.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
