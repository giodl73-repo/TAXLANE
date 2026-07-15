# Payment Integrity Methodology Field Reviews

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl`

This packet maps captured methodology results back to methodology checklist
fields. It now covers the eight Medicare Part D fields, eight Medicaid fields,
eight VA PLTSS fields, and eight USDA Federal Crop Insurance fields.

## Current Review

The Part D review closes sample period, payment type split, and sponsor
documentation dependency treatment internally. The
FY2024 annual row exactly reconciles $3,052.65 million overpayments, $522.44
million underpayments, and zero technical or unknown payments to $3,575.09
million improper payments. Sample design, payment universe, estimation method,
exclusion rules, and recoverable-amount basis remain open.

The payment-universe review now closes two components internally. Same-period
sources identify sampled reconciliation PDE records as the measurement object,
corrected-versus-reported GDC as the error basis, and $96,521.39 million in
row-828 outlays as the exact value rounding to CMS's $96.52 billion denominator.
Appendix A separately establishes that after a reconciliation PDE is adjusted,
reconciliation-PDE-aligned documentation remains required and linked adjustment
documentation is additionally required. The cutoff and final reconciliation
target are prior context. The full field remains open because inclusion,
exclusion, denominator, weight, estimator and payment effects, complete payment
streams, and the GDC-to-federal-outlay mapping are not disclosed.

The overpayment-versus-recoverable review also closes one current-process
component internally. The Q4 2025 scorecard documents issued national-audit
closeout notices requiring deletion of audit-determined improper PDE records
and calls that recovery; planned DME and Tepezza notices remain distinct. The
later process has no cohort or amount linkage to the FY2024/CY2022 estimate, so
the full recoverable-amount basis remains open.

The captured scorecard corrects that dependency review: Part D's source issue
is sponsor drug, pricing, and documentation data, not state data. The CY2022
guide and FAQ supply the same-period fail, correction, final-review, retention,
and successor-sponsor treatment needed for the narrow internal closure.

For sample design, same-period sources now support PDEs as the sampled unit, a
statistically valid stratified-random selection description, sponsor-contract
routing, record identifiers, the reconciliation-PDE cutoff, and OMB sampling-
and-estimation-plan compliance. The full field remains open because the
national sample size, frame and coverage, stratum definitions, allocation,
inclusion probabilities, within-stratum selection implementation, replacement
and nonresponse rules, weights, and linkage to the beneficiary simulation are
not published.

For estimation method, web extraction verifies the official HHS FY2024 APR
process text at printed page 88—PDE-level GDC error assignment followed by
representative-beneficiary simulation—but official PDF custody is blocked by
Akamai HTTP 403. Captured findings and annual row 828 now close the published
confidence-interval and margin-of-error output component. Row 828 does not
define the 0.42 value's units, formula, or relationship to the findings bounds.
Formula, weights, PDE-to-beneficiary aggregation and sample linkage, simulation,
record treatment, variance and confidence-limit construction, margin-of-error
definition, same-period 5% confirmation, and rounding remain open.

For exclusion rules, one narrow component now closes internally. CY2022 timely
incomplete or problematic files remain in review; a Missing Documentation Form
does not replace valid evidence and leaves the PDE failed while unresolved; and
missing or invalid evidence may be cured before the final deadline. FY2024
findings include invalid or missing documentation in the reported category.
Full exclusion rules remain open for taxonomy, current counts, decision stages,
submission-state distinctions, post-deadline and appeal treatment, replacement,
weights, and estimator effects. FY2020's 27 exclusions are comparison-only.

The Medicaid review finds partial support for payment universe, sample period,
payment type split, state rotation/weighting treatment, and improper-payment
versus fraud/waste basis. Sample design, estimation method, and exclusion rules
remain unsupported by the captured result set.

The VA PLTSS review now identifies a complete same-period FY2024 payment-type
composition, with later-cycle FY2025 AFR corroboration. Sample design,
reviewed-claim universe, estimation method, exclusion rules, documentation
standard, and documentation-defect versus recoverable-overpayment basis remain
open.

The USDA Federal Crop Insurance review finds partial support for sample design,
payment universe, estimation method, sample period, and recoverable-savings
basis. Exclusion rules and payment-type split remain unsupported by the captured
result set. The current scorecard root-cause text does not support the older
agency-process-error field framing; it needs reviewer resolution against the
current data-access/outside-agency-control wording. No USDA field is
closure-ready yet.

## Boundary

These rows are review notes only. They do not close methodology fields, do not
estimate savings, and do not make a waste finding.
