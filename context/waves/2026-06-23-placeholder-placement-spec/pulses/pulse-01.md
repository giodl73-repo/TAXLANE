# Pulse 01: Static Placement Spec

## Goal

Add static placement rules for a future placeholder receipt display.

## Changes

- Add `docs/design/placeholder-receipt-placement-spec.md`.
- Add `docs/design/README.md`.
- Link the placement spec from the display packet, chart handoff, and role
  review.
- Include the placement spec in generated manifest coverage.
- Update wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay manifest`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `git diff --check`

## Status

Done.
