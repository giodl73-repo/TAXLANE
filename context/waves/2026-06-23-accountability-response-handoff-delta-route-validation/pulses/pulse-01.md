# Pulse 01

## Objective

Harden validation for applied response handoff delta routes.

## Changes

- Added CLI checks requiring delta Markdown, JSONL, and schema routes in the
  applied response handoff.
- Updated VTRACE rows, wave notes, review evidence, and manifest.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
