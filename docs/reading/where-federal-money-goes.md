# Where Federal Money Goes

## Current-law view

For FY2025, OMB records:

| Measure | Amount / share |
|---|---:|
| Total federal outlays | $7.011T |
| Total federal receipts | $5.236T |
| Deficit gap | $1.775T |
| Borrowed share of outlays | 25.31% |
| Individual income-tax coverage of outlays | 37.88% |

This packet describes where outlays go. It does not claim that individual
income-tax dollars are legally dedicated to these categories. When it shows an
income-tax amount, that amount is a modeled proportional allocation from the
existing TAXLANE outlay model.

## Largest FY2025 outlay subfunctions

| Rank | Subfunction | Outlays | Share of total outlays | Modeled income-tax allocation |
|---:|---|---:|---:|---:|
| 1 | Social Security | $1.581T | 22.55% | $598.812B |
| 2 | Interest on Treasury debt securities, gross | $1.216T | 17.34% | $460.515B |
| 3 | Medicare | $996.718B | 14.22% | $377.591B |
| 4 | Health care services | $922.659B | 13.16% | $349.534B |
| 5 | Department of Defense-Military | $868.426B | 12.39% | $328.989B |
| 6 | Other income security | $235.321B | 3.36% | $89.148B |
| 7 | Income security for veterans | $200.631B | 2.86% | $76.006B |
| 8 | Federal employee retirement and disability | $190.234B | 2.71% | $72.067B |
| 9 | Food and nutrition assistance | $149.631B | 2.13% | $56.685B |
| 10 | Hospital and medical care for veterans | $148.835B | 2.12% | $56.384B |
| 11 | Ground transportation | $100.827B | 1.44% | $38.197B |
| 12 | Housing assistance | $77.989B | 1.11% | $29.545B |
| 13 | Elementary, secondary, and vocational education | $68.364B | 0.98% | $25.899B |
| 14 | Disaster relief and insurance | $62.768B | 0.90% | $23.779B |
| 15 | Health research and training | $50.204B | 0.72% | $19.019B |

## What stands out

The spending side is concentrated. The top five subfunctions alone are Social
Security, gross Treasury interest, Medicare, health care services, and
DOD-Military. Together they explain most of the federal outlay picture before
smaller program areas enter the conversation.

The largest non-benefit line is debt interest. That means past borrowing is not
just a financing detail; it is now one of the largest current outlay categories.

Health appears twice near the top: Medicare and broader health care services.
This matches TAXLANE's separate research finding that the United States has a
health cost problem more than a simple coverage-level problem.

Defense remains a major lane, but it is smaller than the combined retirement,
health, and debt-interest structure. A spending-side map should therefore avoid
treating any single category as the whole budget story.

## Safe accountability questions

Large spend categories justify public questions, but size alone is not evidence
of fraud, waste, abuse, or poor performance.

Safe next questions include:

- What outcome or legal obligation is this category meant to satisfy?
- Which agency, account, or program activity owns the largest pieces?
- Is the category mostly direct benefits, provider payments, payroll, grants,
  procurement, interest, or transfers?
- Which share is mandatory under current law versus annually appropriated?
- What public performance metric would show whether the spending is working?
- Which source can answer the question: OMB function/subfunction tables, agency
  budget justifications, Treasury, CBO, GAO, inspectors general, or USAspending?

Use `accountability-public-brief.md` for the current evidence guardrails before
turning any spend-size observation into a performance claim.

The draft spend-category records in
`data/derived/spend_category_map/spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl`
turn the top categories into a source-routing surface. Each row records the OMB
subfunction evidence level, the modeled allocation caveat, and the next source
needed before asking agency, program, or performance questions. Use
`data/derived/spend_category_map/spend-category-dashboard.md` for a compact
human-readable scan of those rows.

For cost-down work, use
`docs/research/2026-06-28-efficiency-pressure-framework.md` and
`data/derived/efficiency_pressure/efficiency_pressure.fy2025.v1.draft.jsonl`.
Those records identify where to apply pressure over time while keeping waste
claims blocked until reviewed evidence exists.
Then use `docs/reading/cost-down-backlog.md` and
`data/derived/efficiency_pressure/cost_down_backlog.fy2025.v1.draft.jsonl` for
the specific levers, evidence requirements, metrics, and outcome floors.

## What this packet cannot prove

- It cannot trace a specific taxpayer's payment to a specific program.
- It cannot show legal dedication unless a source says the receipt is dedicated.
- It cannot show recipient-level outlays; OMB subfunctions are broader than that.
- It cannot classify waste, fraud, abuse, duplication, or effectiveness.
- It cannot substitute obligations or awards for outlays without a source bridge.

## Sources

Derived from:

- `data/derived/income_tax_outlay_model/income_tax_outlay_model.omb-fy2027.2026-06-21.annual-wide.csv`
- `data/derived/income_tax_outlay_subfunction_model/income_tax_outlay_subfunction_model.omb-fy2027.2026-06-21.fy2025-top-subfunctions.csv`
- `data/derived/spend_category_map/spend_category_map.fy2025.omb-fy2027-v1.draft.jsonl`
- `data/derived/spend_category_map/spend-category-dashboard.md`
