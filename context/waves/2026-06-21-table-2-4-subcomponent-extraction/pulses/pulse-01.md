# Pulse 01: Table 2.4 Workbook Profile

## Goal

Profile OMB Historical Table 2.4 before extracting receipt subcomponent rows.

## Changes

- Add a Table 2.4 workbook profile to the receipt-source extraction workspace.
- Record raw workbook checksum, XML row layout, row groups, and first-year
  reconciliation targets.
- Mark this wave active in `PHASES.md`.

## Validation

- Confirm SHA-256 against metadata.
- `git diff --check`

## Status

Done.
