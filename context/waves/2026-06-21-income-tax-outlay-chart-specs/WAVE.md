# Wave: Income-Tax Outlay Chart Specs

## Goal

Add portable chart specifications for the modeled income-tax outlay allocation
CSV exports.

## Thesis

The model now has canonical JSONL, chart-ready CSVs, and a reader packet. The
next handoff is declarative chart specs that preserve modeled-allocation wording
and keep borrowing context outside the tax allocation stack.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Vega-Lite chart specs | done | Added annual and decade chart specs plus chart usage notes. |

## Success Criteria

- Chart specs read from the generated CSV exports.
- Annual chart shows modeled category percentages as a stacked area.
- Decade chart shows modeled category percentages as a stacked bar.
- Borrowed share remains a separate chart layer or companion view, not a tax
  category.
- Spec titles use modeled-allocation language.

## Non-Goals

- Do not build or host a UI.
- Do not add new model rows or alter model math.
- Do not imply legal tracing or program financing.

## Validation

Run:

```powershell
python -m json.tool docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json
python -m json.tool docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json
git diff --check
```
