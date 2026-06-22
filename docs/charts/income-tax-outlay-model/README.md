# Income-Tax Outlay Model Charts

Parent catalog: `docs/charts/README.md`

## Purpose

These chart specs visualize the modeled allocation of ordinary individual
income-tax receipts by broad OMB outlay share.

They are visualization specs only. They do not change the model, add source
rows, or claim legal dedication of income-tax receipts.

## Specs

| Spec | Data | Use |
|---|---|---|
| `annual-stacked-area.vl.json` | Annual wide CSV, fiscal 1940-2025 | Show year-by-year change in modeled category shares. |
| `decade-stacked-bar.vl.json` | Decade wide CSV | Show long-run category mix by decade. |
| `annual-financing-context-lines.vl.json` | Annual wide CSV, fiscal 1940-2025 | Show borrowed share of outlays and income-tax coverage of outlays by year. |
| `decade-financing-context-lines.vl.json` | Decade wide CSV | Show borrowed share and income-tax coverage by decade. |

## Companion Context

The allocation specs include borrowed-share and income-tax coverage values in
tooltips. The companion line specs show those values directly. They should not
be stacked into the tax allocation categories.

## Title Rule

Use "modeled allocation" in chart titles. Avoid titles such as "where income
taxes went" or "what income taxes funded" because those imply legal or program
tracing.
