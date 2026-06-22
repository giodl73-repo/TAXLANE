# Income-Tax Outlay Subfunction Model Charts

Parent catalog: `docs/charts/README.md`

## Purpose

These chart specs visualize modeled allocations of ordinary individual
income-tax receipts by OMB Table 3.2 subfunction outlay share.

They are drilldown specs only. They do not change the model, add source rows,
claim legal dedication of income-tax receipts, or trace dollars to programs.

## Specs

| Spec | Data | Use |
|---|---|---|
| `fy2025-top-subfunctions.vl.json` | FY2025 top-subfunction CSV | Show the largest modeled subfunction allocations in the latest actual year. |
| `selected-subfunction-trends.vl.json` | Annual long CSV, fiscal 1962-2025 | Compare selected major subfunctions over time after broad context is visible. |
| `decade-top-subfunctions.vl.json` | Decade long CSV | Show the largest modeled subfunction allocations by decade. |

## Public Handoff

Use these specs only after the broad modeled-outlay packet or chart context is
visible. Public displays should link or sit near
`docs/reading/modeled-income-tax-outlays.md` and
`docs/reading/modeled-income-tax-subfunction-outlays.md`.

## Wording Rule

Use "modeled allocation by OMB Table 3.2 subfunction outlay share." Do not use
"income taxes paid for this program" or "where income taxes went."

## Partial-Decade Rule

For `decade-top-subfunctions.vl.json`, label the 1960s as FY1962-FY1969 and the
2020s as FY2020-FY2025. Do not compare those partial buckets as if they contain
ten fiscal years.
