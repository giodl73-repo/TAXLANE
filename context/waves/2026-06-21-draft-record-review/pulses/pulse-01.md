# Pulse 01: IRS Rates Source Review

## Goal

Verify IRS Table 23 checksum, row anchors, source labels, and extracted
1913-1918 values.

## Changes

- Add a Source Custodian review note for the first IRS rates slice.
- Mark the draft record review wave active.

## Validation

- `git diff --check`
- Recomputed raw artifact SHA-256.
- Parsed draft JSONL.
- Verified every row anchor and extracted value against `histab23.xls`.

## Status

Done.
