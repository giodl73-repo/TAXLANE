# Wave: Income-Tax Outlay Validation Runner

## Goal

Add a single validation runner for the income-tax outlay model artifact chain.

## Thesis

The model now has multiple dependent outputs. A one-command validation path
reduces the chance that a future change updates one artifact while skipping
another check.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Validation runner | done | Added a runner that executes model, summary, export, manifest, and chart-spec checks. |

## Success Criteria

- Runner checks annual model, decade summary, CSV exports, manifest, and chart
  JSON specs.
- Runner does not rewrite outputs in validation mode.
- README and manifest mention the runner.
- No model math changes.

## Non-Goals

- Do not add new data rows.
- Do not alter checksums except for documentation changed in this wave.
- Do not build a UI.

## Validation

Run:

```powershell
python data/derived/income_tax_outlay_model/validate_all.py
git diff --check
```
