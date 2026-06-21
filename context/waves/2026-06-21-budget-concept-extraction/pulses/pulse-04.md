# Pulse 04: Fund Rows Budget Interpretation

## Goal

Apply reviewed AP13 budget concepts to the first 1934 fund-group slice without
overclaiming legal dedication or treating Federal Funds as the General Fund.

## Changes

- Promote the first 1934 fund-group JSONL file to `source-reviewed`.
- Set concept-backed interpretation fields:
  - `total`: `mixed` dedication and appropriation.
  - `federal-funds`: `mixed` dedication and appropriation.
  - `trust-funds`: `dedicated` and unknown appropriation.
  - `interfund-transactions`: still `unknown`.
- Add a budget-interpretation note for downstream use.

## Validation

- Parse every source-reviewed JSONL record.
- Recheck 1934 totals against Table 1.1 values already recorded in
  `first-slice-reconciliation.md`.
- `git diff --check`

## Status

Done.
