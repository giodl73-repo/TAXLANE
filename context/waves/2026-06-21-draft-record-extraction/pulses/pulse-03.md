# Pulse 03: OMB Receipts First Draft Rows

## Goal

Extract a narrow receipt-source slice and reconcile total receipts to OMB
Historical Table 1.1.

## Changes

- Add 1934-1936 Table 2.1 receipt-source draft JSONL records.
- Add first-slice reconciliation notes against Table 1.1.
- Keep fund allocation status unknown pending budget-concept review.
- Update the draft record extraction wave status.

## Validation

- `git diff --check`
- Parse draft JSONL records as JSON.
- Verify Table 2.1 total receipts equal Table 1.1 total receipts for 1934-1936.

## Status

Done.
