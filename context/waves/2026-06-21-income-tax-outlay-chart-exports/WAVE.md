# Wave: Income-Tax Outlay Chart Exports

## Goal

Create chart-ready CSV exports from the annual and decade modeled allocation
JSONL files.

## Thesis

The canonical model rows should remain JSONL, but reader-facing charts and quick
analysis need a wide CSV view with one row per fiscal year or decade. Exported
views must preserve model labels, deficit context, and partial-decade caveats.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Chart-ready CSV exports | done | Generated annual and decade wide CSV files from canonical JSONL rows. |
| 02 | Export usage note | done | Documented intended chart uses and wording guardrails. |

## Success Criteria

- Annual CSV has one row per fiscal year, 1940-2025.
- Decade CSV has one row per decade bucket, with the 2020s marked partial.
- Category percentage columns sum to 100 percent within rounding tolerance.
- Borrowed-share and income-tax coverage context remain present.

## Non-Goals

- Do not introduce a new allocation method.
- Do not build a UI in this wave.
- Do not include projected years.

## Validation

Run:

```powershell
python data/derived/income_tax_outlay_model/export_chart_views.py --check
git diff --check
```
