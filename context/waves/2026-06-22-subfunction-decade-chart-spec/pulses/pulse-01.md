# Pulse 01: Decade Top-Subfunction Chart Spec

## Goal

Add a chart specification for the subfunction decade rollup.

## Changes

- Add `docs/charts/income-tax-outlay-subfunction-model/decade-top-subfunctions.vl.json`.
- Include the new chart spec in Rust chart validation.
- Include the new chart spec in the generated manifest.
- Update the chart catalog.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
