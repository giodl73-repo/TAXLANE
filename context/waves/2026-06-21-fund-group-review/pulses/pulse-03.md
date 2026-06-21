# Pulse 03: Fund-Group Source Review

## Goal

Verify OMB Table 1.4 checksum, row anchors, labels, and extracted values.

## Changes

- Add a Source Custodian review note for the first fund-group slice.
- Update the fund-group review wave status.

## Validation

- `git diff --check`
- Recomputed raw artifact SHA-256.
- Parsed draft JSONL.
- Verified every row anchor and extracted amount against `hist01z4_fy2027.xlsx`.

## Status

Done.
