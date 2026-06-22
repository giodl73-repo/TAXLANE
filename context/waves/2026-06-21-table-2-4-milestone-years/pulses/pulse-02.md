# Pulse 02: 1957 and 1966 Social-Insurance Rows

## Goal

Extract social-insurance milestone rows for 1957 and 1966 from OMB Table 2.4.

## Changes

- Add draft JSONL rows for 1957 and 1966 social-insurance subcomponents.
- Preserve off-budget, trust-fund, federal-fund, and total labels.
- Add reconciliation notes against Table 2.1 parent social-insurance rows.
- Mark this pulse complete in the wave.

## Validation

- Parse every JSONL row.
- Reconcile 1957 and 1966 social-insurance totals to Table 2.1 parent rows.
- `git diff --check`

## Status

Done.
