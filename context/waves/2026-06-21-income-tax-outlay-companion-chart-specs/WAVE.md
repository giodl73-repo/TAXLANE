# Wave: Income-Tax Outlay Companion Chart Specs

## Goal

Add companion chart specifications for borrowed-share and income-tax coverage
context.

## Thesis

The allocation charts show modeled tax shares. Financing context should be
visible beside those charts, not stacked into the allocation categories. Separate
line charts keep that distinction clear.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Companion line charts | done | Added annual and decade line specs for borrowed share and income-tax coverage. |

## Success Criteria

- Companion specs read from the generated annual and decade CSV exports.
- Borrowed share and income-tax coverage are shown as separate measures.
- Titles avoid implying that borrowing is an income-tax category.
- Existing allocation chart specs remain unchanged.

## Non-Goals

- Do not build a UI.
- Do not modify model data or export math.
- Do not merge borrowing into the allocation stack.

## Validation

Run:

```powershell
python -m json.tool docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json
python -m json.tool docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json
git diff --check
```
