# Pulse 01: VTRACE First Slice

## Goal

Add TAXLANE's first VTRACE package and make the Rust crate adaptation path
explicit.

## Changes

- Add `docs/vtrace/` with mission, requirements, specification baseline, trace,
  work packages, verification, validation, evidence, and review.
- Define future work packages for accountability/fraud-waste evidence modeling
  and Rust domain crate separation.
- Include VTRACE artifacts in generated manifest coverage.
- Update wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`
- `git diff --check`

## Status

Done.
