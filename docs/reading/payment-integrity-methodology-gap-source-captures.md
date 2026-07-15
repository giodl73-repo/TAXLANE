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
period evidence. The seventh capture combines that scorecard with the CY2022
submission guide and FAQ for sponsor-documentation dependency treatment. The
eighth capture uses the scorecard for
overpayment-versus-recoverable-amount basis evidence.

For sample design, CMS states that Part D IPM reviews a statistically valid
stratified random sample of Prescription Drug Events and uses prescription
record data plus supporting documentation from Part D plan sponsors. The
CY2022 guide further establishes PDE-level selection, sponsor-contract routing,
CMS record identifiers, and the reconciliation-PDE cutoff. These components do
not disclose the national sample size, frame and coverage, stratum definitions,
allocation, inclusion probabilities, within-stratum selection implementation,
replacement and nonresponse rules, weights, or linkage to the beneficiary
simulation, so sample design remains open.

For payment universe, the checksum-verified guide and findings identify sampled
reconciliation PDE records and corrected-versus-reported GDC, while the annual
workbook's row 828 reconciles $92,946.30 million proper plus $3,575.09 million
improper to $96,521.39 million outlays. CMS publishes the rounded denominator as
$96.52 billion. This closes only the measurement-object and
published-denominator component. Appendix A closes a separate component: after
a sampled reconciliation PDE is adjusted, reconciliation-PDE-aligned documents
remain required and linked adjustment documents are additionally required. The
cutoff and final reconciliation target are prior context, not new closures.
Included and excluded streams, adjustment and payment effects, denominator,
weights, estimator treatment, and the mapping from combined plan-beneficiary
GDC liability to federal outlays remain open.

For estimation method, web extraction verifies the official HHS FY2024 APR
passage at printed page 88 assigning each audited PDE a gross drug cost error
and describing representative-beneficiary simulation. HHS blocked official PDF
custody with Akamai HTTP 403, so that process capture remains custody-blocked.
Separately, checksum-custodied findings and annual row 828 close the published
confidence-interval and margin-of-error output component. Row 828 does not
define the 0.42 value's units or formula, and no reconciliation to the findings
bounds is forced. Formula, weights, PDE-to-beneficiary linkage, simulation,
record treatment, variance and confidence-limit construction, margin-of-error
definition, same-period 5% confirmation, and rounding remain open.

For exclusion rules, the CY2022 guide and FAQ establish a narrow current-cycle
component: timely incomplete or problematic files remain in review; a Missing
Documentation Form does not substitute for evidence and leaves the PDE failed
while unresolved; and missing or invalid evidence may be cured before the final
deadline. FY2024 findings include invalid or missing documentation in the 2.70%
reported category. CMS's FY2020 findings separately excluded 27 of 4,526 PDEs
when requested evidence could not be obtained, but that CY2018 treatment is
historical comparison only. Full exclusion rules remain open for taxonomy,
counts, decision stages, submission-state distinctions, post-deadline and appeal
treatment, replacement and weights, estimator effects, and historical
continuity.

For payment type split, the exact annual row reports $3,052.65 million in
overpayments, $522.44 million in underpayments, and zero technically improper
or unknown payments, reconciling to $3,575.09 million improper payments. CMS's
FY2024 findings corroborate the rounded values and category definitions. This
closes payment type split internally but does not establish debt, collectibility,
recovery, fraud, waste, or savings.

For sample period, the PaymentAccuracy Q4 2025 scorecard states that the FY2024
estimate is based on a sampling timeframe starting 1/2022 and ending 12/2022.
This supports sample-period review but still needs reviewer confirmation before
field closure.

For the inherited state-data dependency field, the checksum-verified scorecard
reports $3,053 million at 3.16% and classifies the full rounded amount as outside
agency control because of failure to access data or information needed. The
underlying issues are Part D sponsor drug, pricing, and documentation inputs—not
state data. The CY2022 guide and FAQ establish missing-document failure,
approved correction, final review, retention, and successor-sponsor treatment,
supporting narrow internal closure under the sponsor-documentation label.

For overpayment-versus-recoverable-amount basis, the checksum-verified Q4 2025
scorecard reports issued Adcirca, Revatio, and Cialis audit closeout notices
requiring deletion of PDE records determined improper, which the publisher
describes as resulting in recovery. It separately labels DME and Tepezza notices
as planned. This closes one current-process component, but the later process has
no amount or cohort linkage to the FY2024/CY2022 estimate and does not establish
the collectible or recoverable amount basis.

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

The VA PLTSS captures use the PaymentAccuracy annual workbook and Q4 2025
scorecard, VA FY2025 AFR Section III, and VA Financial Policy Chapter 03. The
same-period annual row completely reconciles overpayment, underpayment,
technically improper, and unknown amounts. The FY2025 AFR corroborates that
taxonomy for a separate later cycle.

For VA PLTSS, sample period and payment-type split are now closure-supported.
The captures do not close sample design, reviewed-claim universe, estimator,
exclusion rules, documentation standard, or documentation-defect-to-
recoverable-overpayment basis.

The USDA Federal Crop Insurance captures use the PaymentAccuracy FCIC Q4 2025
scorecard, RMA COM-25-001, and the RMA compliance memorandum archive. They
provide partial support for sample design, payment universe, estimation method,
sample period, and recoverable-savings basis; source-location context for
exclusion rules and payment-type split; and a root-cause mismatch for the older
agency-process-error field. All eight USDA fields remain open.

## Boundary

These captures give partial support for methodology fields but do not close
them. They do not estimate savings and do not make a waste finding.
