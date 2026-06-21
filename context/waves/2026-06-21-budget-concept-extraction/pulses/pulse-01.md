# Pulse 01: AP13 Source Capture

## Goal

Capture OMB Analytical Perspectives Chapter 13 as the fund-concept source and
prepare the concept extraction workspace.

## Changes

- Capture `ap_13_funds_fy2027.pdf` under `data/raw/omb/SRC-OMB-AP-13-FUNDS-FY2027/2026-06-21/`.
- Add source metadata and SHA-256 checksum.
- Add a concept extraction workspace for budget concepts.
- Mark this wave active in `PHASES.md`.

## Validation

- `git diff --check`
- Confirm captured PDF header.
- Confirm `pdftotext` can read the captured PDF.

## Status

Done.
