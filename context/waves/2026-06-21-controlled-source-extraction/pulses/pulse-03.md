# Pulse 03: OMB Receipts First-Source Capture

## Goal

Capture OMB receipt and fund spreadsheet metadata and prepare receipt/fund draft
extraction workspaces.

## Changes

- Capture OMB FY2027 Historical Tables 1.1, 1.4, 2.1, 2.2, and 2.4 under
  `data/raw/omb/`.
- Add source metadata and SHA-256 checksums in `data/metadata/`.
- Add draft extraction workspaces for `receipt_source` and `fund_group`.
- Update the controlled source extraction wave status.

## Validation

- `git diff --check`

## Status

Done.
