# Wave: Draft Record Extraction

## Goal

Convert captured source artifacts into small, schema-backed draft records that
can be source-reviewed and budget-reviewed before any public allocation model is
published.

## Thesis

TAXLANE should prove extraction quality on narrow slices before scaling to full
tables. Draft records need row anchors, source IDs, year semantics, status
labels, and caveats before they can support public explanations.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | IRS rates first draft rows | done | Extracted 1913-1918 IRS Table 23 lowest/highest regular-tax summary rows as draft JSONL. |
| 02 | IRS rates table profile | pending | Record workbook sheet profile, header rows, footnote handling, and extraction hazards. |
| 03 | OMB receipts first draft rows | pending | Extract a narrow receipt-source slice and reconcile total receipts to Table 1.1. |
| 04 | OMB outlays first draft rows | pending | Extract a narrow outlay-function slice and identify net-interest/offsetting rows. |
| 05 | Draft review checklist | pending | Define source-review and budget-review checklists for promoting draft rows. |

## Success criteria

- Draft records are JSONL and follow the relevant schema.
- Every draft row includes source IDs, source row references, observed date,
  year basis, and status.
- Summary rows are labeled as summary rows, not complete schedules.
- No taxpayer allocation or reform claim is derived from draft rows.
- Validation commands pass.

## Non-goals

- Do not extract full IRS or OMB tables in one pulse.
- Do not mark draft rows reviewed.
- Do not publish taxpayer-facing receipts from draft rows.
- Do not infer legal dedication or allocation from extracted rates.

## Validation

Run:

```powershell
git diff --check
```

For JSONL files, also parse each line as JSON.
