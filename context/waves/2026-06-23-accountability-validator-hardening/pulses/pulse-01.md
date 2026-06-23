# Pulse 01: Core Claim Guardrails

## Goal

Add Rust guardrails for possible-misconduct evidence signals and public accusation wording.

## Changes

- Reject draft `possible_fraud`, `possible_waste`, and `possible_abuse` records.
- Reject accusation-style public wording without `official_finding` or `adjudicated` status.
- Add unit tests for rejected and allowed accountability signal paths.
- Update schema notes, VTRACE rows, and role review.

## Validation

- `cargo test`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`
- `git diff --check`

## Status

Done.
