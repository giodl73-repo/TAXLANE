# Pulse 02: Annual Draft Model Rows

## Goal

Generate fiscal-year 1940-2025 draft rows for the broad OMB Table 3.1
categories.

## Changes

- Add `income_tax_outlay_model.omb-fy2027.2026-06-21.draft.jsonl`.
- Add `source-profile.md` with coverage, category anchors, and sample
  reconciliation.
- Keep all rows marked `draft` until source review.

## Validation

- `python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check`
- JSONL parse of generated rows.
- `git diff --check`

## Status

Done.
