# Payment Integrity Methodology Gap Source Captures

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_gap_source_captures_q4_2025.jsonl`

This packet records official-source captures produced from the methodology
gap-followup queue.

## Current Captures

The first two captures use the CMS Fiscal Year 2024 Improper Payments Fact Sheet
for Medicare Part D methodology gaps. The third capture uses the CMS Medicare
Part D IPM Program Background page. The fourth capture uses a CMS FY2020 Part D
IPM findings PDF as historical exclusion-rule evidence. The fifth capture uses
the CMS FY2024 Part D IPM findings PDF for payment-type split evidence. The
sixth capture uses the PaymentAccuracy Q4 2025 Part D scorecard for sample
period evidence. The seventh capture uses the same scorecard for state-data
dependency evidence. The eighth capture uses the scorecard for
overpayment-versus-recoverable-amount basis evidence.

For sample design, CMS states that Part D IPM reviews a statistically valid
stratified random sample of Prescription Drug Events and uses prescription
record data plus supporting documentation from Part D plan sponsors.

For payment universe, CMS states that Part D IPM primarily analyzes Prescription
Drug Events, which are CMS-defined summary extracts that include prescription
transaction details. This partially supports the universe question but does not
fully define included and excluded payments.

For estimation method, CMS states that validation findings are applied to a
randomly selected 5% sample of the Part D beneficiary population and extrapolated
onto payments of remaining Part D beneficiaries to determine the gross payment
error amount and national Part D IPM. This partially supports the estimator
question but still needs reviewer confirmation before field closure.

For exclusion rules, CMS's FY2020 findings state that 27 sampled PDEs were
excluded from the Part D IPM calculation because requested supporting
documentation could not be obtained. This is evidence that exclusion rules exist,
but it is historical and does not close the FY2024/Q4 2025 exclusion-rule field.

For payment type split, CMS's FY2024 findings define overpayment and
underpayment logic using corrected gross drug cost compared with the PDE record.
They also identify FY2024 overpayment categories and the FY2024 underpayment
category. This supports overpayment and underpayment split review but does not
establish unknown-payment treatment.

For sample period, the PaymentAccuracy Q4 2025 scorecard states that the FY2024
estimate is based on a sampling timeframe starting 1/2022 and ending 12/2022.
This supports sample-period review but still needs reviewer confirmation before
field closure.

For state-data dependency treatment, the PaymentAccuracy Q4 2025 scorecard
classifies the FY2024 Part D overpayment root cause as failure to access data or
information needed and describes data/support validation barriers. This supports
state-data dependency review but does not establish the full data-dependency
treatment used in the estimate.

For overpayment-versus-recoverable-amount basis, the PaymentAccuracy Q4 2025
scorecard reports the FY2024 overpayment amount and rate and includes
corrective-action and recovery-plan context. This supports distinguishing the
reported overpayment estimate from recovery work, but it does not establish the
collectible or recoverable amount basis.

The Medicaid captures use the PaymentAccuracy Medicaid Q4 2025 scorecard, CMS
PERM program page, CMS PERM manual, and CMS FY2025 Improper Payments Fact Sheet.
They provide partial support for all eight Medicaid methodology gaps.

For Medicaid sample design, the PERM manual supports sample-design review but
still needs cycle-specific sample detail before closure.

For Medicaid payment universe and payment-type split, the CMS PERM page
identifies Fee-For-Service, managed care, and eligibility components, but does
not fully close included/excluded payment definitions, overlap treatment, or
component weighting.

For Medicaid estimation method and exclusion rules, the PERM manual provides
methodology structure and component-combination context, but the exact
cycle-specific estimator, uncertainty, and exclusion rules remain review needs.

For Medicaid sample period, the PaymentAccuracy scorecard gives the sampling
timeframe starting 7/2022 and ending 6/2023, pending reconciliation with PERM
cycle documentation.

For Medicaid state rotation/weighting, the CMS FY2025 fact sheet supports the
three-cycle, approximately 17-states-per-cycle review structure, but not full
weighting mechanics.

For Medicaid improper-payment versus fraud/waste basis, CMS source language
supports that PERM improper-payment rates are not fraud rates and that
insufficient documentation is generally not indicative of fraud or abuse. It
does not establish recoverable savings.

The VA PLTSS captures use the PaymentAccuracy PLTSS Q4 2025 scorecard, VA
FY2025 AFR Section III, and VA Financial Policy Chapter 03. They provide partial
support for seven VA methodology fields and source-location context for
exclusion rules, but all eight VA fields remain open.

For VA PLTSS, the current captures support statistical sampling, FY2024 tested
payment context, proper/improper/unknown reporting, payment-appropriateness test
attributes, documentation-related causes, and recovery context. They do not
close PLTSS-specific sample design, reviewed-claim universe, estimator,
exclusion rules, exact sample dates, payment-type split, documentation standard,
or recoverable-overpayment basis.

The USDA Federal Crop Insurance captures use the PaymentAccuracy FCIC Q4 2025
scorecard, RMA COM-25-001, and the RMA compliance memorandum archive. They
provide partial support for sample design, payment universe, estimation method,
sample period, and recoverable-savings basis; source-location context for
exclusion rules and payment-type split; and a root-cause mismatch for the older
agency-process-error field. All eight USDA fields remain open.

## Boundary

These captures give partial support for methodology fields but do not close
them. They do not estimate savings and do not make a waste finding.
