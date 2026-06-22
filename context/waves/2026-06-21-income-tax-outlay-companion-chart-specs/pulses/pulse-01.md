# Pulse 01: Companion Line Charts

## Goal

Add line chart specs for financing context values.

## Changes

- Add annual financing context line spec.
- Add decade financing context line spec.
- Update chart README.

## Validation

- `python -m json.tool docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json`
- `python -m json.tool docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json`
- `git diff --check`

## Status

Done.
