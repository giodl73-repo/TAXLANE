# Pulse 01: Mockup Review Checklist

## Goal

Add a static review checklist for any future placeholder receipt mockup.

## Changes

- Add `docs/design/placeholder-receipt-mockup-review-checklist.md`.
- Link the checklist from the design index and placement-spec review.
- Include the checklist in generated manifest coverage.
- Update wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `git diff --check`

## Status

Done.
