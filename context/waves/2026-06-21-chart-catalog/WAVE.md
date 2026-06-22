# Wave: Chart Catalog

## Goal

Add a top-level chart catalog so TAXLANE visualization specs are discoverable.

## Thesis

The chart specs are useful only if future readers and UI work can find them,
understand their data dependencies, and preserve the model caveats. A chart
catalog should document that without changing data or chart math.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Chart catalog | done | Added top-level chart index and cross-linked the income-tax outlay chart folder. |

## Success Criteria

- Chart catalog lists available specs and their source CSVs.
- Catalog states chart reading order.
- Catalog preserves modeled-allocation wording and deficit-context guidance.

## Non-Goals

- Do not add or modify chart specs.
- Do not build a UI.
- Do not alter model data.

## Validation

Run:

```powershell
python -m json.tool docs/charts/income-tax-outlay-model/annual-stacked-area.vl.json
python -m json.tool docs/charts/income-tax-outlay-model/decade-stacked-bar.vl.json
python -m json.tool docs/charts/income-tax-outlay-model/annual-financing-context-lines.vl.json
python -m json.tool docs/charts/income-tax-outlay-model/decade-financing-context-lines.vl.json
git diff --check
```
