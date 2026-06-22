# Pulse 01: Deficit Context Guardrail

## Goal

Make deficit and income-tax coverage context required for public subfunction
chart use.

## Changes

- Add `docs/research/2026-06-22-subfunction-deficit-context-note.md`.
- Add a financing-context section to the subfunction reader packet.
- Add a deficit-context addendum to the subfunction reader role review.
- Record this wave in `context/waves/PHASES.md`.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
