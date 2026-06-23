# Pulse 01: Placeholder Chart Sync Check

## Goal

Add validation that compares static placeholder receipt chart values against
the canonical placeholder receipt JSON.

## Changes

- Add chart-sync validation to `taxlane-tools income-tax-outlay validate`.
- Check lane labels, rounded amounts, shares, display treatments, borrowed
  share, and income-tax coverage.
- Refresh the artifact manifest after changing the validation tool.
- Update wave status.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo test`
- `git diff --check`

## Status

Done.
