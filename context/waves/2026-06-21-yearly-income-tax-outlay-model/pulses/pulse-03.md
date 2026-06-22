# Pulse 03: Reconciliation Review

## Goal

Review the generated annual model rows against the three OMB source tables.

## Changes

- Verify year coverage and row counts.
- Confirm Table 3.1 total outlays reconcile to Table 1.1.
- Confirm modeled category allocations sum to individual income-tax receipts.
- Add a reconciliation review note.

## Validation

- `python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check`
- Independent JSONL parse and annual aggregation check.
- `git diff --check`

## Status

Done.
