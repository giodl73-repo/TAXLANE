# Payment Integrity Methodology Field Reviews

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_reviews_q4_2025.jsonl`

This packet maps captured methodology results back to methodology checklist
fields. It now covers the eight Medicare Part D fields, eight Medicaid fields,
eight VA PLTSS fields, and eight USDA Federal Crop Insurance fields.

## Current Review

The Part D review finds partial support for sample period, state-data dependency
treatment, and overpayment-versus-recoverable-amount basis. It does not find
support in the captured scorecard result for sample design, payment universe,
estimation method, exclusion rules, or payment type split.

The Medicaid review finds partial support for payment universe, sample period,
payment type split, state rotation/weighting treatment, and improper-payment
versus fraud/waste basis. Sample design, estimation method, and exclusion rules
remain unsupported by the captured result set.

The VA PLTSS review finds partial support for seven fields: sample design,
reviewed-claim universe, estimation method, sample period, payment type split,
documentation standard, and documentation-defect versus recoverable-overpayment
basis. Exclusion rules remain unsupported by the captured result set. No VA
field is closure-ready yet.

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
