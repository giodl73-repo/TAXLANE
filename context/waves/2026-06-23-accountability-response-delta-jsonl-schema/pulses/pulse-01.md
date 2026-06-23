# Pulse 01

## Objective

Add schema documentation for applied response delta JSONL rows.

## Changes

- Added `performance-demand-response-delta.applied-example.schema.md`.
- Added CLI generation and validation for the delta JSONL schema.
- Updated README, artifact map, applied fixture schema, manifest, VTRACE rows,
  and review evidence.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
