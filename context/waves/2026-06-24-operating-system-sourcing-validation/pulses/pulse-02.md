# Pulse 02: Wire Records into Rust Validation

## Goal
Make the program-lane reform rules self-enforce instead of being documented only.

## Changes
- Added `validate_program_lane_records` to `tools/taxlane/src/main.rs`, wired into `run_income_tax_outlay_validation`. For every JSONL record under `data/derived/program_lane_rate_model/` it enforces:
  - non-empty `record_id`, `record_family`, `status`;
  - `allocation_method` must contain `proposed_reform` (the reform gate — blocks any record claiming legal dedication);
  - any `source_ids` must exist in the source-version ledger;
  - the receipt-share model and the income-tax-budget allocation each reconcile to 100% (+/-0.1).
- Hardened `read_jsonl` to skip blank lines.
- Regenerated `MANIFEST.md` (README + main.rs hash updates).

## Validation
- `cargo fmt --check` clean; `cargo test` ok; `income-tax-outlay validate` exits 0 with "validated 85 program-lane records".
- Negative test: a `legal_dedication` record fails with a clear message; passes once removed.

## Status
Done.
