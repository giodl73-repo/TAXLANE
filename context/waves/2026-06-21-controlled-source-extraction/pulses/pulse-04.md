# Pulse 04: OMB Outlays First-Source Capture

## Goal

Capture OMB outlay spreadsheet metadata and prepare outlay and lane draft
extraction workspaces.

## Changes

- Capture OMB FY2027 Historical Tables 3.1, 3.2, 4.1, 8.5, 8.7, and 11.3 under
  `data/raw/omb/`.
- Add source metadata and SHA-256 checksums in `data/metadata/`.
- Add draft extraction workspaces for `outlay_function`, `outlay_program`, and
  `lane_crosswalk`.
- Update the controlled source extraction wave status.

## Validation

- `git diff --check`

## Status

Done.
