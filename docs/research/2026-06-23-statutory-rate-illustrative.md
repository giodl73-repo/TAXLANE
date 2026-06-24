# Statutory Lane Rates — Illustrative "% of Income"

## Decision supported

Delivers the rate framing the owner asked for from the start — a per-lane rate
expressed as a percentage of income — using the IRS SOI aggregate income base.
It is labeled **illustrative** because it maps a tax-year base (TY2022) onto the
fiscal-year lane structure (FY2025); a single-year statutory rate needs matched
years (deferred).

Data record:
`data/derived/program_lane_rate_model/program_lane_statutory_rate.ty2022-illustrative.draft.jsonl`.

## The income base (IRS SOI, TY2022, complete report)

| Base | TY2022 ($M) |
|---|---:|
| Aggregate AGI (less deficit), all returns | 14,833,957 |
| Aggregate taxable income | 11,714,186 |
| Total income tax after credits | 2,098,923 |
| Returns filed / taxable returns | 161.3M / 110.6M |

**Today's individual income tax is ~14.2% of AGI / ~17.9% of taxable income** in
aggregate (after credits). That is the rate being split across lanes below.

## Illustrative current rate per lane (general-fund share)

Each general-fund lane's share of the income tax, expressed as a percent of AGI
and of taxable income. Social Security is excluded (funded by payroll). Shares
of AGI sum to the current 14.15%.

| Lane | % of income tax | **% of AGI** | % of taxable income |
|---|---:|---:|---:|
| Medicare | 18.35% | **2.60%** | 3.29% |
| Health (non-Medicare) | 18.02% | **2.55%** | 3.23% |
| Debt Service (net interest) | 17.86% | **2.53%** | 3.20% |
| National Defense | 16.87% | **2.39%** | 3.02% |
| Income Security & Family | 12.92% | **1.83%** | 2.32% |
| Veterans | 6.95% | **0.98%** | 1.24% |
| Transportation | 2.68% | **0.38%** | 0.48% |
| Constitutional Govt & Justice | 2.28% | **0.32%** | 0.41% |
| Nat. Resources/Energy/Environ. | 2.04% | **0.29%** | 0.37% |
| Community/Disaster/Regional | 1.52% | **0.22%** | 0.27% |
| Education, Work & Social Services | 1.33% | **0.19%** | 0.24% |
| Agriculture & Food | 0.87% | **0.12%** | 0.16% |
| International Affairs | 0.83% | **0.12%** | 0.15% |
| Science, Space & Civic Capacity | 0.77% | **0.11%** | 0.14% |
| Commerce/Housing Credit | -0.52% | -0.07% | -0.09% |
| Undistributed Offsetting Receipts | -2.77% | -0.39% | -0.50% |

Read as: of the ~14.2% of AGI Americans pay in income tax today, about 2.6 points
go to Medicare's general-fund share, 2.5 to non-Medicare health, 2.5 to interest
on past debt, 2.4 to defense, and so on.

## The decisive balanced-budget finding

To fund **all** the non-Social-Security general-fund lanes ($5,430,432M) from the
**income tax alone** would require **36.6% of AGI — 2.6× today's 14.2%.** That is
politically and economically infeasible on the income tax by itself.

This is the quantified proof of the panel's conclusion: a balanced "spend what we
collect" system **cannot** run on the income tax alone. It requires the
all-receipts base (income + payroll + corporate + excise) **and** the
spending-discipline side (the health cost-per-outcome target above all), landing
near the panel's ~$6.0-6.3T with tax-to-GDP rising only partway toward OECD.

## Caveats (Source Custodian conditions)

- **Cross-year map**: TY2022 base × FY2025 lane structure. Directional, not a
  filing-year rate. A matched-year build needs FY2025 SOI (not yet published) or
  FY-converted lane costs.
- AGI vs taxable income are different denominators; both shown.
- These are **aggregate** effective rates, not marginal bracket rates and not an
  individual's rate. No individual liability is implied.
- `allocation_method = proposed_reform`; no legal dedication.

## Adopt now

- Use the %-of-AGI column as the public "what each lane costs you in income tax"
  figure, with the illustrative label.
- Use the 36.6%/2.6× finding as the core argument for the all-receipts +
  discipline blend.

## Defer

- A matched-year statutory rate and any marginal-bracket schedule.
- Distributional (who-pays) analysis across the income distribution.
