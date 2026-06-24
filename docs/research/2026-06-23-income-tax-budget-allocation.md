# Income-Tax-as-Budget Allocation

## Decision supported

This realizes the "spend only what we collect, no more" principle for the
individual income tax specifically: treat current income-tax collection
($2,656,044M FY2025) as a **fixed budget envelope** and allocate it across the
general-fund lanes it is responsible for, as percentages summing to 100%.

Data record:
`data/derived/program_lane_rate_model/income_tax_budget_allocation.fy2025.v1.draft.jsonl`.

## Framing

- **The budget = what we collect.** Income tax is $2,656,044M. Under a hard
  no-borrowing rule, that is the ceiling for the lanes income tax funds.
- **Social Security is carved out** to its own payroll tax (it is dedicated, not
  income-tax funded) — per the Budget Accountant and Public Goods Steward panel
  lenses.
- **Basis**: neutral proportional allocation across the non-Social-Security
  general-fund lanes (each lane's share of that $5,430,432M cost). This is a
  starting point; priority weights can override it.

## What fits, and what does not

The honest result: the general-fund lanes cost **$5,430,432M**, but income tax
collects only **$2,656,044M** — so income tax covers **48.9%** of them. The other
**$2,774,388M** is exactly the amount that today is met by other taxes
(corporate, excise, customs) plus **borrowing**. Under "spend what we collect,"
that borrowed portion is forbidden and must close by cutting cost or collecting
more — the decision the eight-role panel debated
(`reviews/2026-06-23-fair-rate-panel.md`).

| Lane | % of income-tax budget | Income-tax $ (M) | Full cost (M) | Not covered by income tax (M) |
|---|---:|---:|---:|---:|
| Medicare | 18.35% | 487,498 | 996,718 | 509,220 |
| Health (non-Medicare) | 18.02% | 478,593 | 978,511 | 499,918 |
| Debt Service (net interest) | 17.86% | 474,462 | 970,065 | 495,603 |
| National Defense | 16.87% | 448,087 | 916,140 | 468,053 |
| Income Security & Family | 12.92% | 343,160 | 701,609 | 358,450 |
| Veterans | 6.95% | 184,472 | 377,163 | 192,691 |
| Transportation | 2.68% | 71,076 | 145,320 | 74,244 |
| Constitutional Govt & Justice | 2.28% | 60,498 | 123,692 | 63,194 |
| Nat. Resources/Energy/Environ. | 2.04% | 54,094 | 110,599 | 56,505 |
| Community/Disaster/Regional | 1.52% | 40,289 | 82,374 | 42,085 |
| Education, Work & Social Services | 1.33% | 35,236 | 72,042 | 36,806 |
| Agriculture & Food | 0.87% | 23,206 | 47,447 | 24,240 |
| International Affairs | 0.83% | 22,093 | 45,171 | 23,078 |
| Science, Space & Civic Capacity | 0.77% | 20,526 | 41,966 | 21,440 |
| Commerce/Housing Credit | -0.52% | -13,759 | -28,131 | -14,372 |
| Undistributed Offsetting Receipts | -2.77% | -73,490 | -150,254 | -76,764 |

Percentages sum to 100% of the income-tax budget.

## Caveats

- Proportional is a neutral baseline, **not** a recommendation; the panel argues
  for priority-weighting (fund net interest senior; protect the social floor;
  bend health cost; ring-fence payroll lanes).
- Medicare and Transportation have partial dedicated funding (HI payroll, fuel
  excise), so their true income-tax draw is smaller than shown; refine after the
  receipts-by-source split (wave Pulse 02).
- `allocation_method = proposed_reform`; not legal dedication.

## Adopt now

- Use as the "where your income tax goes, and what it cannot cover" view.
- Pair with the panel review when setting priority weights off the proportional
  baseline.
