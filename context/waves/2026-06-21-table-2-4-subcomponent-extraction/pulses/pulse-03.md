# Pulse 03: 1940 Subcomponent Source Review

## Goal

Verify the 1940 Table 2.4 subcomponent rows against the captured workbook XML
and promote them to source-reviewed.

## Changes

- Rename the 1940 Table 2.4 JSONL from draft to source-reviewed.
- Promote row statuses to `source-reviewed`.
- Add source-review notes for anchors, totals, and limits.
- Mark this pulse complete in the wave.

## Validation

- Parse every source-reviewed JSONL row.
- Reconcile 1940 social-insurance and excise totals to Table 2.1 parent rows.
- `git diff --check`

## Status

Done.
