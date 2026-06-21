# Pulse 02: 1940 Subcomponent Draft Rows

## Goal

Extract the first fiscal year 1940 social-insurance and excise subcomponent rows
from OMB Table 2.4.

## Changes

- Add draft JSONL subcomponent rows for 1940 social-insurance and excise
  composition.
- Add schema fields for receipt subcomponent extraction.
- Record first-year reconciliation against Table 2.1 parent categories.
- Mark this pulse complete in the wave.

## Validation

- Parse every JSONL row.
- Reconcile 1940 social-insurance and excise totals to Table 2.1 parent rows.
- `git diff --check`

## Status

Done.
