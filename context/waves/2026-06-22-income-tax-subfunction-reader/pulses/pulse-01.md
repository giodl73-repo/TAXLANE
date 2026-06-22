# Pulse 01: Subfunction Reader Packet

## Goal

Add a public drilldown packet for the modeled subfunction allocation views.

## Changes

- Add `docs/reading/modeled-income-tax-subfunction-outlays.md`.
- Add `reviews/2026-06-22-subfunction-reader-role-review.md`.
- Add the reader packet to `docs/reading/README.md`.
- Include the packet and review in the generated manifest.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
