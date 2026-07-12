# Aligned Contribution Receipt

## Purpose

This packet explains the next TAXLANE receipt rule:

> A visible lane should say what goes in, who benefits, and which denominator is
> being used before it shows a per-person number.

That does not mean every lane is pay-in/pay-out. Defense is a public good. Debt
interest is a financing cost. Health outside Medicare is redistributive support.
Social Security and Medicare HI are closer to contributory social insurance.

## The Rule

Every lane needs four labels before a per-person receipt:

| Label | Meaning |
|---|---|
| Payer base | Who pays or which receipt source funds the lane. |
| Beneficiary base | Who directly benefits, or whether the public benefits collectively. |
| Alignment type | Contributory, premium-plus-general-support, redistributive, public-good, or financing-cost. |
| Denominator | Per resident, taxpayer, worker, beneficiary, enrollee, or household. |

No denominator, no per-person number.

The first sourced denominator values are tax-return denominators from IRS SOI
TY2022: 161.3M individual income tax returns filed and 110.6M taxable returns.
Those support "per filed return" or "per taxable return" illustrations only.
They do not support a generic FY2025 "per person" claim.

## Medicare Correction

Medicare cannot be shown as one clean payroll-funded lane.

| Medicare surface | Correct alignment |
|---|---|
| Medicare HI / Part A | Closest to pay-in/pay-out; payroll-financed social insurance. |
| Medicare SMI / Parts B and D | Premiums plus general revenue support; intentionally subsidized. |
| Medicare total | Mixed; must be split before claiming contribution alignment. |

The current FY2025 allocation file already shows the issue: Medicare outlays are
about $996.7B, while current dedicated HI payroll plus trust-excise receipts in
the model cover only about $398.8B. The remaining need is general financing in
the current TAXLANE allocation model.

There is also a source-boundary check: OMB Table 3.2 reports $996.718B for the
FY2025 Medicare subfunction, while OMB Table 8.5 reports $987.656B for the
Medicare mandatory-program row. That $9.062B difference is a scope boundary, not
a math discrepancy, and Table 8.5 still does not expose the Part A/B/D split.

The 2026 Medicare Trustees Report supplies the financing split on a calendar-year
trust-fund basis. In CY2025, HI reported $462.4B of income and $444.2B of
expenditures. Part B reported $580.5B of income and $584.3B of expenditures,
including $150.3B of premiums and $422.2B of government contributions. Part D
reported $183.317B of income and $181.531B of expenditures, including $14.862B
of premiums, $148.844B of government contributions, and $19.087B of state
payments.

The same Trustees report supplies CY2025 Medicare denominator values: 69.100M
Part A enrollees, 63.448M Part B enrollees, 56.754M Part D enrollees, and
69.289M beneficiaries with HI and/or SMI coverage. These can support
part-specific Medicare examples only on a calendar-year Trustees basis.

Census denominators are also now sourced for broad civic displays: the July 1,
2025 resident population estimate is 341,784,857, and Census HH-1 reports
134.790M households for 2025. These support "per resident" and "per household"
views only; they do not imply equal tax liability or personal benefit matching.

Social Security denominators are now sourced on a rounded CY2025 Trustees basis:
185.0M covered workers, 70.5M OASDI beneficiaries, 62.3M OASI beneficiaries, and
8.2M DI beneficiaries. These support contributory-lane context only when the
wage-base cap and solvency gap remain visible.

The readiness layer now separates three display states: same-source/year-basis
examples that can be shown as source context, cross-basis civic illustrations
that must carry their basis warning, and blocked rows where a denominator is
still missing.

The public receipt-card packet turns those states into reader-facing copy. It
must travel with each number; otherwise the number should stay unpublished.

## How To Read Major Lanes

| Lane | Alignment type | Per-person display should use |
|---|---|---|
| Social Security | Contributory earned benefit | per worker and per beneficiary, with the wage-cap and solvency gap visible |
| Medicare HI / Part A | Contributory earned benefit | per worker and per Part A enrollee/beneficiary |
| Medicare SMI / Parts B/D | Premium plus general support | per enrollee, premium share, and general-revenue support |
| Health outside Medicare | Redistributive transfer / coverage support | per enrollee or beneficiary, with coverage and outcome floors |
| Defense | Public good | per resident or taxpayer civic cost, not personal benefit matching |
| Debt interest | Financing cost | per resident or taxpayer burden of past borrowing |

## What This Changes

The future receipt should not just say:

> You paid X dollars to Medicare.

It should say something closer to:

> Your visible Medicare lane combines a payroll-financed Hospital Insurance
> component with premium/general-revenue Supplementary Medical Insurance support.
> The exact per-worker and per-enrollee displays are blocked until the Part A/B/D
> denominators are sourced.

That is less tidy, but it is more honest.

## Current Artifacts

- `docs/research/2026-06-28-contribution-benefit-alignment.md`
- `data/derived/contribution_alignment/contribution_alignment.fy2025.v1.draft.jsonl`
- `data/derived/contribution_alignment/medicare_source_boundary.fy2025.draft.jsonl`
- `data/derived/contribution_alignment/medicare_part_financing.cy2025.cms-trustees-2026.draft.jsonl`
- `data/derived/denominator_requirements/denominator_requirements.v1.draft.jsonl`
- `data/derived/denominator_requirements/denominator_values.ty2022.irs-soi-1304.draft.jsonl`
- `data/derived/denominator_requirements/denominator_values.cy2025.cms-medicare-trustees-2026.draft.jsonl`
- `data/derived/denominator_requirements/denominator_values.cy2025.census.draft.jsonl`
- `data/derived/denominator_requirements/denominator_values.cy2025.ssa-trustees-2026.draft.jsonl`
- `data/derived/denominator_requirements/per_unit_display_readiness.v1.draft.jsonl`
- `data/derived/denominator_requirements/per-unit-display-readiness.md`
- `data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl`
- `docs/reading/per-unit-receipt-cards.md`
- `docs/research/2026-06-28-denominator-source-ladder.md`
- `docs/research/2026-06-28-medicare-source-boundary.md`
- `docs/research/2026-06-29-medicare-part-financing.md`
- `docs/research/2026-06-29-medicare-denominators.md`
- `docs/research/2026-06-29-civic-denominators.md`
- `docs/research/2026-06-29-social-security-denominators.md`

## Guardrail

This is a design surface. It is not tax advice, an individual liability
calculator, legal dedication of income-tax dollars, or proof that any person gets
back exactly what they paid.
