# Pulse 01

## Objective

Make applied response bundle artifacts verifiable by consumers.

## Changes

- Added `row_count` and `sha256` fields to bundle artifact entries.
- Extended core bundle artifact validation for checksum shape and row-count
  rules.
- Updated generated bundle JSON, schema documentation, review evidence, VTRACE
  rows, and manifest.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
