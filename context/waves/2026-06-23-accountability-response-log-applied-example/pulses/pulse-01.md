# Pulse 01

## Objective

Add applied response-log example rows that show the typed output of the
intake-to-log handoff.

## Changes

- Added `performance-demand-response-log.applied-example.jsonl`.
- Added CLI generation and validation for applied response-log example rows.
- Updated artifact map, README, manifest, VTRACE rows, and review evidence.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
