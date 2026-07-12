# Payment Integrity Methodology Plans

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_plans_q4_2025.jsonl`

This packet turns the first open task family, methodology, into a source
discovery plan for each selected PaymentAccuracy program. It defines the fields
that must be source-cited before methodology can be marked complete.

## Required Shape

Each program needs the payment universe, sample design, estimation method,
exclusion rules, sample period, and payment-type split. Program-specific fields
then clarify the largest interpretation risk: state-data treatment for Part D,
documentation-defect treatment for VA PLTSS, and agency-process-error treatment
for Federal Crop Insurance. Medicaid adds PERM state rotation/weighting and the
improper-payment versus fraud/waste boundary.

## Boundary

These plans do not close the methodology task yet. They only define what source
evidence must be captured next before any savings score or waste finding can be
considered.
