# Pulse 02: Fund Concept First Draft Records

## Goal

Extract the first AP13 budget-concept records needed to interpret fund-group
rows without turning accounting concepts into legal-allocation claims.

## Changes

- Add draft `budget_concept` JSONL records for:
  - Federal funds group.
  - General fund.
  - Special funds.
  - Revolving funds.
  - Trust funds.
  - Federal trust-fund public caveat.
- Add `budget_concept` to the data dictionary family map.
- Keep source anchors at PDF page and printed-page level.

## Validation

- Parse every JSONL record.
- `git diff --check`

## Status

Done.
