# Income-Tax Outlay Model Charts

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

## Companion Context

The specs include borrowed-share and income-tax coverage values in tooltips.
Those values should also be shown as separate companion charts or annotations in
any final UI. They should not be stacked into the tax allocation categories.

## Title Rule

Use "modeled allocation" in chart titles. Avoid titles such as "where income
taxes went" or "what income taxes funded" because those imply legal or program
tracing.
