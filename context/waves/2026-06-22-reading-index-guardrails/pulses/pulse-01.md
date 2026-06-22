# Pulse 01: Reading Order Guardrail

## Goal

Update public reader entry points for the current modeled-allocation packet set.

## Changes

- Add reading order to `docs/reading/README.md`.
- Add public-use wording guardrails for allocation packets.
- Update root README reader guidance to include the broad and subfunction
  modeled-allocation packets.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
