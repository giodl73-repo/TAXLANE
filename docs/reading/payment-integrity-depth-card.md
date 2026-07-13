# Payment Integrity Depth Card

Machine record:
`data/derived/breadth_benchmark_matrix/payment_integrity_depth_card.fy2024.v1.draft.json`.

> Draft review surface. The underlying scorecard probes remain blocked from
> unrestricted public claims pending source and role review.

The government-wide headline remains **$161.5B in FY2024 reported improper
payments across covered risk-susceptible programs**. It is not the full federal
payment universe, a fraud estimate, or an automatically recoverable amount.

## What the retained probes add

Four Q4 2025 program scorecards report FY2024 overpayment estimates totaling
**$35.499B**, equal to 22.0% of the separate government-wide headline:

| Program probe | Reported overpayment | Rate | Primary root-cause signal |
|---|---:|---:|---|
| Medicaid | $29.370B | 4.81% | Needed provider, eligibility, redetermination, or documentation data unavailable |
| Medicare Part D | $3.053B | 1.02% | Other-party process error and state-data access |
| VA purchased long-term services and supports | $2.502B | 15.54% | Insufficient documentation to determine |
| Federal Crop Insurance | $0.574B | 2.40% | Needed external compliance data unavailable |

The 22.0% figure measures how much of the government-wide headline is touched by
these four retained probes. It is not a representative coverage rate or a
complete decomposition. Source-specific sample periods and methods still apply.

## What is still unknown

Only the four-program overpayment field is populated. This bridge does not yet
contain complete, disjoint amounts for underpayments, unknown payments,
documentation/data errors, confirmed fraud, collectible recoveries, amounts
actually collected, control cost, or preventable future loss. Missing means
unknown—not zero.

```text
improper payments
!= overpayments
!= confirmed fraud
!= recoverable dollars
!= collected recoveries
!= net savings
```

The next depth step is a consistent program-year extraction that preserves
payment type, root cause, adjudication, recovery stage, control cost, appeal and
access effects, and sampling period. Until then, fraud and savings claims remain
blocked.
