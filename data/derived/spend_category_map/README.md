# Spend Category Map

## Purpose

This derived family turns the existing TAXLANE FY2025 outlay-subfunction model
into a spend-side map: large federal spending categories, their share of total
outlays, their modeled income-tax allocation, and the next source needed for a
deeper agency/program question.

It is a question-routing layer. It is not legal dedication, taxpayer-dollar
tracing, recipient-level spending, or a performance finding.

## Model ID

`spend-category-map-v1`

## Inputs

| Source ID | Role |
|---|---|
| `SRC-OMB-HIST-1-1-FY2027` | Total FY2025 outlays, receipts, and deficit context. |
| `SRC-OMB-HIST-2-1-FY2027` | Individual income-tax receipt amount used in modeled allocation. |
| `SRC-OMB-HIST-3-2-FY2027` | FY2025 function/subfunction outlays. |

## Artifacts

| Artifact | Role |
|---|---|
| `spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl` | Top FY2025 OMB subfunction spend-category rows. |
| `spend_category_map.schema.md` | Field contract for the JSONL rows. |
| `spend-category-dashboard.md` | Human-readable summary generated from the spend-category JSONL. |
| `accountability-question-handoff.md` | Safe follow-up questions and source needs by top spend category. |

## Method

Rows are selected from
`data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv`.

Each row keeps the OMB function/subfunction identity and adds source-routing
metadata. The money fields are copied from the existing subfunction model:

```text
outlay_share_percent = subfunction_outlays / FY2025 total_outlays * 100
modeled_income_tax_allocation = individual_income_tax_receipts
                                * subfunction_outlays / FY2025 total_outlays
```

For FY2025, the emitted Table 3.2 subfunction total reconciles exactly to OMB
total outlays in the current extraction, so the source model's allocation-share
denominator and the public "share of total outlays" wording are numerically
identical.

## Caveats

- OMB subfunctions are broad categories.
- Some subfunctions are larger than a single agency or program.
- USAspending obligation/award records are not final outlays without a source
  bridge.
- Spend size is not evidence of fraud, waste, abuse, duplication, or bad
  performance.

## Validation

```powershell
cargo run -p taxlane-tools -- income-tax-outlay validate
git diff --check
```
