# Pulse 01: Subfunction Reconciliation Review

## Goal

Record the reconciliation status of generated subfunction model outputs.

## Changes

- Add `data/derived/income_tax_outlay_subfunction_model/reconciliation-review.md`.
- Include the review in manifest coverage.
- Keep public-use caveats visible in the review decision.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
