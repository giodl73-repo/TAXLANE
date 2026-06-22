# Pulse 01: Model Contract and Source Profile

## Goal

Define the yearly individual income-tax outlay allocation model before relying
on generated rows.

## Changes

- Mark this wave active in `PHASES.md`.
- Add a derived model README with method, fields, and public wording guardrails.
- Add a repeatable generator for draft fiscal-year records and reconciliation.

## Validation

- `python data/derived/income_tax_outlay_model/build_income_tax_outlay_model.py --check`
- `git diff --check`

## Status

Done.
