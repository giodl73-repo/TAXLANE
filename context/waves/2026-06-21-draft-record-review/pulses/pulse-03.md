# Pulse 03: OMB Outlays Source Review

## Goal

Verify OMB Table 3.1 checksum, row anchors, labels, and 1940-1942 extracted
values.

## Changes

- Add a Source Custodian review note for the first OMB outlay-function slice.
- Update the draft record review wave status.

## Validation

- `git diff --check`
- Recomputed raw artifact SHA-256.
- Parsed draft JSONL.
- Verified every row anchor, source label, and extracted amount against
  `hist03z1_fy2027.xlsx`.

## Status

Done.
