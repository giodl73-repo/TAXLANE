# Pulse 02: Fund-Group First Draft Rows

## Goal

Extract the first fund-group receipt/outlay/surplus-deficit draft rows and
reconcile totals.

## Changes

- Add 1934 Table 1.4 fund-group draft JSONL records.
- Add first-slice reconciliation notes against Table 1.1.
- Keep legal dedication and appropriation status unknown.
- Update the fund-group review wave status.

## Validation

- `git diff --check`
- Parse draft JSONL records as JSON.
- Verify Table 1.4 total receipts, outlays, and surplus/deficit equal Table 1.1
  for 1934.

## Status

Done.
