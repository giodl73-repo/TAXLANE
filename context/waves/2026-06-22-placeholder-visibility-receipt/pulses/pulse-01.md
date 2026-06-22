# Pulse 01: Placeholder FY2025 Receipt

## Goal

Add a draft visibility receipt scenario for a placeholder `$1,000` ordinary
individual income-tax payment.

## Changes

- Add `data/derived/taxpayer_receipt_model/` with a README and canonical draft
  JSON scenario.
- Add `docs/reading/placeholder-visibility-receipt.md`.
- Update derived-data and wave indexes.

## Validation

- `git diff --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`

## Status

Done.
