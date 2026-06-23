# Pulse 01: Static Receipt Display Specs

## Goal

Create static Vega-Lite handoff specs for the placeholder visibility receipt.

## Changes

- Add `docs/charts/taxpayer-receipt-model/README.md`.
- Add `placeholder-lane-bars.vl.json`.
- Add `placeholder-financing-context.vl.json`.
- Add both specs to tool validation.
- Update the chart catalog and wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `git diff --check`

## Status

Done.
