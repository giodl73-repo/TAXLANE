# Pulse 01: Decade Subfunction CSV Export

## Goal

Generate a decade-level subfunction allocation CSV from the canonical annual
subfunction model rows.

## Changes

- Add `income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.decade-long.csv`.
- Extend `income-tax-outlay subfunction-export` and `--check`.
- Validate each decade's subfunction shares sum to 100 percent.
- Add manifest and README coverage.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
