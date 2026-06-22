# Wave: Decade Income-Tax Outlay Summary

## Goal

Summarize the yearly individual income-tax outlay model by decade so TAXLANE can
describe long-run category changes without replacing the annual source rows.

## Thesis

The annual model is the source of truth for fiscal-year views. A decade summary
is useful for interpretation only if it is derived mechanically from those rows,
uses cumulative income-tax receipts as the denominator, and keeps deficit
context visible.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Decade rollup | done | Generated decade category percentages, cumulative modeled amounts, and deficit context. |
| 02 | Interpretation note | done | Recorded what the decade rollup shows and how to phrase it. |

## Success Criteria

- Decade category percentages equal cumulative modeled category allocations
  divided by cumulative individual income-tax receipts.
- The 2020s are labeled as partial because the model currently covers
  2020-2025.
- Deficit gap and borrowed-share context remain visible for every decade.
- The summary is labeled as derived from the annual modeled allocation rows.

## Non-Goals

- Do not replace the annual model with decade averages.
- Do not include 2026-2031 projections.
- Do not make legal-dedication or reform-design claims.

## Validation

Run:

```powershell
python data/derived/income_tax_outlay_model/build_decade_summary.py --check
git diff --check
```
