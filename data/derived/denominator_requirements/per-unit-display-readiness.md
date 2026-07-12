# Per-Unit Display Readiness

## Purpose

This dashboard translates the denominator work into a public-display boundary:
which per-unit claims are ready, which are illustrative cross-basis claims, and
which remain blocked.

Machine rows: `data/derived/denominator_requirements/per_unit_display_readiness.v1.draft.jsonl`.

## Ready On Same Source-Year Basis

| Display | Computed value | Basis |
|---|---:|---|
| Medicare HI / Part A expenditures per Part A enrollee | $6,428.36 | CY2025 Trustees expenditures / CY2025 Part A enrollment |
| Medicare Part B government contribution per Part B enrollee | $6,654.27 | CY2025 Trustees government contribution / CY2025 Part B enrollment |
| Medicare Part D government contribution per Part D enrollee | $2,622.61 | CY2025 Trustees government contribution / CY2025 Part D enrollment |

These can be used as Medicare financing-context examples, not as FY2025 OMB
outlay receipts.

## Illustrative Cross-Basis Only

| Display | Computed value | Basis |
|---|---:|---|
| Defense-Military FY2025 outlays per CY2025 resident | $2,540.86 | FY2025 OMB Table 3.2 / Census July 1 2025 resident population |
| Gross Treasury interest FY2025 outlays per CY2025 resident | $3,556.66 | FY2025 OMB Table 3.2 / Census July 1 2025 resident population |

These are broad civic-cost illustrations. They are not equal tax liabilities,
legal dedication, or personal benefit matching.

## Still Blocked

| Display | Blocker |
|---|---|
| Medicare HI payroll financing per HI covered worker | Need a Medicare HI covered-worker denominator. Do not substitute OASDI covered workers. |

## Rule

The display status must travel with the number:

- `ready_same_source_year_basis` can appear as source-basis context.
- `illustrative_cross_basis` must visibly name both the fiscal-year numerator
  and calendar-year denominator.
- `blocked_missing_denominator` cannot publish a computed value.
