# Pulse 01: Accountability Evidence Schema

## Goal

Add the accountability evidence model and close VTRACE WP-TAX-002.

## Changes

- Add `docs/data/accountability-evidence-schema.md`.
- Add `docs/research/2026-06-23-accountability-evidence-boundary.md`.
- Add `reviews/2026-06-23-accountability-evidence-role-review.md`.
- Update the data dictionary and data index.
- Update VTRACE trace, verification, validation, evidence, and work-package status for WP-TAX-002.
- Include new artifacts in generated manifest coverage.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`
- `cargo fmt --check`
- `git diff --check`

## Status

Done.
