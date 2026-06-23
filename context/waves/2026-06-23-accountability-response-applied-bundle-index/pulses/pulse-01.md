# Pulse 01

## Objective

Make the full applied response fixture bundle discoverable from one artifact.

## Changes

- Added a generated applied response bundle index.
- Added CLI validation for bundle staleness, README discovery, and required
  applied fixture artifact membership.
- Updated artifact-map routing, VTRACE rows, review evidence, and manifest.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
