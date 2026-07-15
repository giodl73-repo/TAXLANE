# Federal Crop Insurance Payment-Integrity Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json`.

This bridge reconciles the official FY2024 PaymentAccuracy annual row, the Q4
2025 FCIC scorecard, and RMA's source for the underlying FY2024 policy review.
It initially closed two methodology fields internally. The linked
[USDA AFR root-cause extension](federal-crop-insurance-root-cause-definition-bridge.md)
supports a third internal-only closure, and the linked
[FCIC/RMA payment-universe extension](federal-crop-insurance-payment-universe-bridge.md)
supports a fourth while preserving every claim gate.
The [sample-design component extension](federal-crop-insurance-sample-design-component-bridge.md)
then records narrow FY2024 governance evidence without changing the four-closed,
four-open field count.

## Reconciled FY2024 Result

The annual workbook reports $23,867.31M in covered outlays and $579.36M in
improper payments, a 2.4274% rate. The payment-type composition is $573.93M in
overpayments, $5.43M in underpayments, and zero technically improper or unknown
payments. All reported overpayments are classified as outside agency control.

The Q4 2025 scorecard rounds the same overpayment result to $574M and 2.40%.
Its displayed root-cause categories are $467M for failure to access needed data
or information and $107M for inability to access it. Those rounded categories
sum to the rounded scorecard total, not to the annual workbook's centimillion
precision.

## Review And Period Match

RMA COM-23-001 says the FY2024 reporting-period review used a statistically
valid sample of 326 policies from the 2022 reinsurance year, selected with
regard to the Approved Insurance Provider servicing each policy. The 2022
Standard Reinsurance Agreement defines that year as
July 2021 through June 2022. Those dates match both PaymentAccuracy sources
exactly.

This evidence closes `sample period` and `payment type split` internally. The
FY2024 USDA AFR separately defines the failure-to-access and inability-to-
access categories, closing `data-access outside-agency-control root-cause
definition` internally. Sample design remains open because the public sources
do not disclose the sample frame, strata, weights, projection estimator, or
variance method. The official FCIC/RMA FY2024 financial-statements report closes payment
universe internally by disclosing all three payment categories—premium subsidy,
Administrative and Operating expense, and indemnities—and high-, medium-, and
low-payment AIP tiers. Estimation method, exclusion rules, and recoverable-
savings basis also remain open.

USDA OIG/KPMG independently reviewed the FY2024 program samples and underlying
sampling methodologies. Combined with the same-period disclosures, that supports
only a narrow component: 326 RY2022 policies, AIP-aware selection and tiering,
and an annual statistical-validity designation. The audit's printed-page-23
recommendation for more detailed S&EMP review reinforces the boundary:
compliance is not public reproducibility. Frame construction, allocation,
selection probabilities, randomization, replacement, nonresponse, weights,
estimator, and variance remain undisclosed.

## Claim Firewall

The scorecard says AIPs are notified through CARS and that overpayments are
tracked, collected, and verified through RMA accounting systems.
It does not publish a collected or collectible amount or connect collections
to the FY2024 statistical estimate.

The USDA-wide Do Not Pay section immediately following the FCIC discussion is
not program-specific and is excluded from the bridge.

The financial statements' Other Information on printed pages 60-61 is
unaudited and corroborative only. Its apparent $579.93M overpayment typo is
excluded in favor of the official annual workbook's $573.93M.

Improper payments are not automatically fraud, waste, identified debt,
recoveries, preventable loss, or net savings. No score is allowed until the
remaining methodology, debt-lineage, appeal, collection, prevention, and
control-cost evidence is available.
