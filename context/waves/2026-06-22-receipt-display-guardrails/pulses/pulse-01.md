# Pulse 01: Receipt Display Guardrails

## Goal

Encode the placeholder receipt role-review blockers directly in the receipt
artifact and reader packet.

## Changes

- Add display guardrail fields to the canonical placeholder receipt JSON.
- Add offset-row wording to the reader packet.
- Add Social Security and Medicare dedicated-financing caveats to the reader
  packet.
- Update wave status.

## Validation

- `git diff --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`

## Status

Done.
