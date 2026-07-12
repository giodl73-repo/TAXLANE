# Per-Unit Receipt Cards

## Purpose

This packet turns the denominator readiness layer into public-facing card copy.
It is not a taxpayer bill, not a calculator, and not legal dedication of income
taxes. Every card carries its source basis.

Machine rows: `data/derived/denominator_requirements/per_unit_receipt_cards.v1.draft.jsonl`.

## Source-Basis Context Cards

These cards use numerator and denominator values from the same source-year basis.

| Card | Amount | Source basis |
|---|---:|---|
| Medicare Part A spent about this much per Part A enrollee | $6,428 | CY2025 Trustees expenditures / CY2025 Part A enrollment |
| Medicare Part B government support per Part B enrollee | $6,654 | CY2025 Trustees government contribution / CY2025 Part B enrollment |
| Medicare Part D government support per Part D enrollee | $2,623 | CY2025 Trustees government contribution / CY2025 Part D enrollment |

Use these as Medicare financing context. Do not call Part B or Part D
payroll-funded, and do not describe any card as what a person personally paid or
received.

## Civic Illustration Cards

These cards intentionally combine FY2025 OMB outlays with CY2025 Census
denominators. That makes them useful as civic scale, not as receipts.

| Card | Amount | Source basis |
|---|---:|---|
| Defense-Military FY2025 outlays per CY2025 resident | $2,541 | FY2025 OMB Table 3.2 / Census July 1, 2025 resident population |
| Gross Treasury interest FY2025 outlays per CY2025 resident | $3,557 | FY2025 OMB Table 3.2 / Census July 1, 2025 resident population |

Use the phrase "as a civic-cost illustration" near these values. Do not say
each resident paid that amount, received that amount, or legally had income-tax
dollars dedicated to the lane.

## Blocked Card

| Card | Why blocked |
|---|---|
| Medicare HI payroll financing per HI covered worker | The HI payroll numerator is sourced, but the Medicare-HI-specific covered-worker denominator is not extracted. OASDI covered workers are not a safe substitute. |

## Public Copy Rule

Every per-unit card needs these labels:

| Label | Required wording |
|---|---|
| Status | Source-basis context, civic illustration, or blocked. |
| Basis | Name the numerator year/source and denominator year/source. |
| Caveat | Say what the number is not. |

Without those labels, do not publish the number.
