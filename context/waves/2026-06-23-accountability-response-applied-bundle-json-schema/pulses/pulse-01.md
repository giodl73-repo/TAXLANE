# Pulse 01

## Objective

Make the applied response bundle JSON contract inspectable.

## Changes

- Added `performance-demand-response-bundle.applied-example.schema.md`.
- Added CLI validation for generated schema text, README discovery, and field
  coverage.
- Updated artifact map, applied fixture schema, VTRACE rows, review evidence,
  and manifest.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
