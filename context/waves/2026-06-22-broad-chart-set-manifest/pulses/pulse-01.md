# Pulse 01: Broad Chart README Manifest Coverage

## Goal

Track the broad chart-set README in the generated artifact manifest.

## Changes

- Add `docs/charts/income-tax-outlay-model/README.md` to manifest artifacts.
- Update the top-level chart catalog to point to local handoff rules for the
  broad chart set.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
