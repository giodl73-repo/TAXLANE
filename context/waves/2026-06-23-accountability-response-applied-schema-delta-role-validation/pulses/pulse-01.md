# Pulse 01

## Objective

Harden validation for applied fixture schema delta roles.

## Changes

- Added CLI checks requiring delta Markdown, JSONL, and schema roles in the
  applied fixture schema.
- Updated VTRACE rows, wave notes, review evidence, and manifest.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
