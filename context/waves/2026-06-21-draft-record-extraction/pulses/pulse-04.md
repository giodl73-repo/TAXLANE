# Pulse 04: OMB Outlays First Draft Rows

## Goal

Extract a narrow outlay-function slice and identify net-interest and offsetting
rows.

## Changes

- Add 1940-1942 Table 3.1 outlay-function draft JSONL records.
- Add first-slice reconciliation notes against Table 1.1.
- Keep net interest and undistributed offsetting receipts visible.
- Update the draft record extraction wave status.

## Validation

- `git diff --check`
- Parse draft JSONL records as JSON.
- Verify Table 3.1 total outlays equal Table 1.1 total outlays for 1940-1942.

## Status

Done.
