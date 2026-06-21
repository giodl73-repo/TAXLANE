# Pulse 01: Individual Income Receipt Allocation

## Goal

Apply the reviewed AP13 general-fund concept to the first individual
income-tax receipt rows without changing numeric receipt values or implying
program-level allocation.

## Changes

- Promote the first Table 2.1 receipt-source JSONL to `source-reviewed`.
- Add AP13 as concept support for individual income-tax rows.
- Set individual income-tax `allocation_status` to `general_receipt`.
- Set total receipt rows to `mixed`.
- Keep other receipt categories at `unknown`.
- Add an allocation-review note.
- Mark this wave active in `PHASES.md`.

## Validation

- Parse every source-reviewed JSONL record.
- Recheck 1934-1936 total receipts against `first-slice-reconciliation.md`.
- `git diff --check`

## Status

Done.
