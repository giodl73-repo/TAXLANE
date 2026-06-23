# Pulse 01: Placement Spec Role Review

## Goal

Add a role review for the static placeholder receipt placement spec.

## Changes

- Add `reviews/2026-06-23-placeholder-placement-spec-role-review.md`.
- Link the review from `docs/design/`.
- Include the review in generated manifest coverage.
- Update wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `git diff --check`

## Status

Done.
