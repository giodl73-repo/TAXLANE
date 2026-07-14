# Payment Integrity Depth Card

Machine record:
`data/derived/breadth_benchmark_matrix/payment_integrity_depth_card.fy2024.v1.draft.json`.

> Draft review surface. The annual extraction and underlying probes remain
> blocked from unrestricted public claims pending source and role review.

The official FY2024 PaymentAccuracy workbook reconciles GAO's rounded **$161.5B
improper-and-unknown payment headline** across 68 reported programs:

| Evidence class | Amount | Share of combined headline |
|---|---:|---:|
| Overpayments | $135.184B | 83.68% |
| Underpayments | $7.864B | 4.87% |
| Technically improper payments | $5.923B | 3.67% |
| Unknown payments | $12.569B | 7.78% |
| **Combined** | **$161.540B** | **100.00%** |

Overpayments, underpayments, and technically improper payments reconcile to
$148.971B classified improper payments. Adding $12.569B unknown payments
reconciles to $161.540B. Covered outlays were $4.072T; the workbook does not
represent the full federal payment universe, and program measurement periods
vary.

## Fraud and recovery remain parallel evidence

The workbook also has 54 FY2024 confirmed-fraud rows, defined narrowly as cases
confirmed by a court, and 59 agency recovery rows. Neither table is established
as a disjoint, same-period subset of the estimated improper-payment headline.
TAXLANE therefore does not subtract either from $161.540B or call the remainder
recoverable savings.

## VA source reconciliation

The annual workbook reports VA Purchased Long Term Services and Supports as
$218.30M overpaid, $6.41M underpaid, $432.46M technically improper, and $102.93M
unknown. Source review found that the official Q4 2025 scorecard also reports
$218.30M in FY2024 overpayments at 3.88%, based on an October 2022 through
September 2023 sample. The prior $2.502B probe came from a non-resolving URL and
is corrected; it was not an official-source disagreement.

```text
improper payments
!= overpayments
!= confirmed fraud
!= recoverable dollars
!= collected recoveries
!= net savings
```

Next depth is role review of the extraction and corrected probe, plus matched
program/period links for fraud, recovery, control cost, appeals, access effects,
and prospective prevention.

The first recovery-link attempt is now documented in the
[VA PLTSS recovery bridge](va-pltss-recovery-bridge.md). It remains blocked
because projected errors, established debts, and collections do not share a
matched cohort, period, or transaction lineage.

## Federal crop insurance reconciliation

The [Federal Crop Insurance payment-integrity bridge](federal-crop-insurance-payment-integrity-bridge.md)
reconciles the FY2024 annual row to the Q4 2025 scorecard and RMA review-period
sources. Covered outlays were $23,867.31M; the annual row reports $573.93M in
overpayments, $5.43M in underpayments, and zero technically improper or unknown
payments. The scorecard's $574M and 2.40% are rounded presentations of the
annual overpayment result.

The July 2021 through June 2022 measurement window and complete annual payment-
type split are closed internally. The
[FY2024 root-cause definition bridge](federal-crop-insurance-root-cause-definition-bridge.md)
adds a third internal-only closure: USDA defines failure to access information
as an administrative or calculation error after available information was used,
and inability to access information mainly as participant certification error. The
[FY2024 payment-universe bridge](federal-crop-insurance-payment-universe-bridge.md)
adds a fourth: the official FCIC/RMA financial-statements report discloses premium
subsidy, Administrative and Operating expense, and indemnities across high-,
medium-, and low-payment AIP tiers. Four methodology fields remain open: sample
design, estimation method, exclusion rules, and recoverable-savings basis. The
[FY2024 sample-design component bridge](federal-crop-insurance-sample-design-component-bridge.md)
adds a narrow internal component without changing those counts: 326 RY2022
policies, AIP-aware selection and tiering, statistical-validity language, and
independent audit review. Frame construction, allocation, probabilities,
randomization, replacement, nonresponse, weights, estimator, and variance remain
open. Compliance is not public reproducibility. The separate
[FY2020 historical sampling-method bridge](federal-crop-insurance-historical-sampling-method-bridge.md)
records simple-random policy selection, all three named payment categories, and
statistically valid rate and dollar estimates for RY2018. Because that evidence
is historical unaudited Other Information, it does not establish continuity to
FY2024, close a current field, or change the four-closed/four-open aggregate.

The [public methodology evidence-ceiling bridge](federal-crop-insurance-public-methodology-evidence-ceiling.md)
adds the governing access boundary. OMB requires point and confidence-interval
estimates but directs the S&EMP and checklist to secure MAX. The FY2025 report
repeats the public categories, tiers, statistical-validity language, and a
3.29-percent actual rate without exposing the FCIC estimator or exclusions.
Zero fields close; the aggregate remains four closed and four open. No reported
amount or root-cause
category is treated as identified debt, collected recovery, fraud, waste,
preventable loss, or net savings. The USDA-wide Do Not Pay figures that follow
the FCIC section are excluded. The financial statements' Other Information on
printed pages 60-61 is unaudited; its apparent $579.93M overpayment typo is not
used in place of the official annual workbook's $573.93M.

The [recovery-lineage boundary bridge](federal-crop-insurance-recovery-lineage-boundary-bridge.md)
follows the 326 sampled policies through no-finding closures, Initial and Final
Findings, review completion, and rate reporting. It also establishes that
general compliance findings and criminal recoveries are separate amount
classes, not recoveries of the statistical estimate. This narrow component
closes no full field; debt, appeals, collections, prevention, and control cost
remain unreconciled.

The [appeal and collectibility governance bridge](federal-crop-insurance-appeal-collectibility-governance-bridge.md)
adds the post-Finding state machine: CARS receipt, a 45-day evidence-backed
dispute window, final administrative review, possible correction and payment,
repayment discretion, and setoff with preserved appeal rights. It closes a
narrow governance component only. A finding is not automatically final
collectible debt, and a payment requirement is not automatically cash collected.
