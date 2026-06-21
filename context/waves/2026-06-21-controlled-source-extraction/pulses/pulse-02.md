# Pulse 02: IRS Rates First-Source Capture

## Goal

Capture IRS SOI Historical Table 23 source metadata and prepare the first
rates extraction path.

## Changes

- Capture `histab23.xls` under `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/`.
- Add source metadata and checksum in `data/metadata/`.
- Add the draft extraction workspace for `rates_timeline` records.
- Update the controlled source extraction wave status.

## Validation

- `git diff --check`

## Status

Done.
