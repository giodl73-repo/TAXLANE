# Pulse 01: Vega-Lite Chart Specs

## Goal

Add portable chart specifications for the modeled allocation CSV exports.

## Changes

- Add annual stacked area Vega-Lite spec.
- Add decade stacked bar Vega-Lite spec.
- Add chart README with usage and wording guardrails.

## Validation

- `python -m json.tool docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json`
- `python -m json.tool docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json`
- `git diff --check`

## Status

Done.
