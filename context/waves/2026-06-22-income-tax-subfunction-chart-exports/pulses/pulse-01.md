# Pulse 01: Subfunction CSV Exports

## Goal

Emit chart-ready CSV views from the canonical subfunction allocation JSONL.

## Changes

- Add Rust `income-tax-outlay subfunction-export` subcommand.
- Emit a full annual-long CSV view.
- Emit a FY2025 top-subfunction ranked CSV view.
- Include subfunction export checks in aggregate validation.
- Include subfunction model artifacts in the generated manifest.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check`
- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-export --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
