# Medicare Part D Published Uncertainty-Output Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_published_uncertainty_output_bridge.fy2024.v1.draft.json`.

## Published output component

The checksum-verified FY2024 findings publish the CY2022 Part D IPM gross
improper-payment result and its uncertainty output on printed and PDF file page
1, Table 1. The rounded gross estimate is **$3.58 billion** at **3.70%**. Its
published 95% confidence interval is **$3.19 billion to $4.01 billion**, with
rate bounds of **3.31% to 4.15%**.

PaymentAccuracy annual workbook row 828 separately reports **$3,575.09
million** improper payments, an improper-payment rate of **0.037039355**, the
confidence label **`95% to <100%`**, and a margin-of-error value of **0.42**.

Together these sources close one internal component: the FY2024 published
confidence-interval and annual margin-of-error outputs are identified and
preserved at their publisher-reported precision.

## Margin-of-error boundary

Row 828 does not disclose a unit or formula for 0.42. This bridge does not call
it percentage points, percent, dollars, a relative measure, or any other unit.
The row also does not explain how that field relates to the findings' rounded
dollar and rate bounds. No reconciliation is forced.

The 95% bounds accompany the gross improper-payment result. The findings do not
publish a separate confidence interval for the $2.53 billion net estimate, so
the gross interval is not transferred to the net result.

## Remaining estimation-method gaps

Full estimation method remains open for official HHS FY2024 APR binary custody;
the estimator formula; weights, strata, projection, and aggregation;
PDE-to-beneficiary sample linkage; benefit-parameter simulation; record and
exclusion treatment; the variance estimator and finite-population method;
confidence-limit construction; margin-of-error definition and formula;
unrounded values and rounding rules; and same-period beneficiary-sample and
extrapolation confirmation.

Published bounds are statistical uncertainty around the gross improper-payment
result, not a range of debt, collectible dollars, recovery, prevention, or
savings. Rounded values cannot reconstruct estimator mechanics. OMB-compliant
and statistically valid remain publisher attestations rather than reproducible
methodology.

One component closes internally, but the full `estimation method` field remains
open. Medicare Part D stays three fields closed and five open. Every public,
field-closure, scoring, fraud, waste, debt, collectibility, recovery, prevention,
and savings gate remains false.
