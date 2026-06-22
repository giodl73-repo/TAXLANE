# Pulse 01: Subfunction Chart README

## Goal

Place subfunction chart handoff rules beside the chart specs.

## Changes

- Add a chart-set README under `docs/charts/income-tax-outlay-subfunction-model/`.
- Link the chart-set README from the top-level chart catalog.
- Include the chart-set README in manifest coverage.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
