# Pulse 05: Source-Version Ledger

## Goal

Create a reproducibility ledger for official income-tax and federal-budget
sources before extracting rates, receipts, outlays, or accounting definitions.

## Changes

- Add `docs/sources/source-version-ledger.md`.
- Record official source IDs for constitutional authority, the Revenue Act of
  1913, IRS historical rate tables, OMB historical receipts/outlays tables, OMB
  budget-accounting chapters, Treasury Fiscal Data, USAspending, and candidate
  CBO cross-checks.
- Define required source fields and extraction rules.
- Mark CBO as candidate-only until accessible verification is available.

## Validation

- `git diff --check`

## Status

Done.
