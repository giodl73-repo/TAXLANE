# Pulse 01

## Objective

Add a JSONL handoff for applied response delta rows.

## Changes

- Added `performance-demand-response-delta.applied-example.jsonl`.
- Added CLI generation and validation for typed delta JSONL rows.
- Updated README, artifact map, applied fixture schema, manifest, VTRACE rows,
  and review evidence.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
