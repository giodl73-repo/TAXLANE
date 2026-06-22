# Pulse 01: Artifact Manifest

## Goal

Add a generated manifest for the modeled income-tax outlay artifact chain.

## Changes

- Add `build_manifest.py`.
- Add generated `MANIFEST.md`.
- Link the manifest from the model README.

## Validation

- `python data/derived/income_tax_outlay_model/build_manifest.py --check`
- `git diff --check`

## Status

Done.
