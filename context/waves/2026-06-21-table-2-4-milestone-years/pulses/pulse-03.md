# Pulse 03: 1957 and 1983 Excise Trust Rows

## Goal

Extract excise-tax milestone rows for 1957 and 1983 from OMB Table 2.4.

## Changes

- Add draft JSONL rows for 1957 and 1983 excise subcomponents.
- Preserve federal-funds and trust-funds groupings.
- Add reconciliation notes against Table 2.1 parent excise rows.
- Mark this pulse complete in the wave.

## Validation

- Parse every milestone JSONL row.
- Reconcile 1957 and 1983 excise totals to Table 2.1 parent rows.
- `git diff --check`

## Status

Done.
