# Pulse 01: Decade Rollup

## Goal

Generate decade-level category percentages from the annual draft model rows.

## Changes

- Add a decade summary generator.
- Add machine-readable decade JSONL.
- Add a Markdown decade summary table.

## Validation

- `python data/derived/income_tax_outlay_model/build_decade_summary.py --check`
- JSONL parse and per-decade percentage sum check.
- `git diff --check`

## Status

Done.
