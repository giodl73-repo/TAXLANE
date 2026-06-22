# Pulse 01: Annual Subfunction Allocation Model

## Goal

Emit draft annual modeled individual income-tax allocation rows by OMB Table 3.2
subfunction.

## Changes

- Add Rust `income-tax-outlay subfunction-model` subcommand.
- Emit FY1962-FY2025 JSONL rows under `data/derived/income_tax_outlay_subfunction_model/`.
- Add source profile and README for the subfunction model.
- Update derived-data model index.

## Validation

- `cargo run -p taxlane-tools -- income-tax-outlay subfunction-model --check`
- `cargo run -p taxlane-tools -- outlay-function table-3-2 --check`
- `cargo run -p taxlane-tools -- income-tax-outlay validate`
- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
