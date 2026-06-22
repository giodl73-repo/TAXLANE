# Pulse 01: Subfunction Chart Specs

## Goal

Add first chart specifications for the subfunction allocation CSV exports.

## Changes

- Add FY2025 top-subfunction Vega-Lite bar chart spec.
- Add selected-subfunction annual trend Vega-Lite line chart spec.
- Include both specs in Rust chart-spec validation.
- Include both specs in the generated artifact manifest.
- Update the chart catalog.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
