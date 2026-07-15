# Payment Integrity Methodology Closure Readiness

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl`

This packet triages the Medicare Part D, Medicaid, VA PLTSS, and USDA Federal
Crop Insurance methodology source-capture rollups into reviewer next steps.

## Current Status

Ten fields are closure-review candidates across the four programs. For Part D
and VA PLTSS, they include:

- Part D payment type split
- Part D sponsor documentation dependency treatment
- VA PLTSS sample period
- VA PLTSS payment type split

Part D still has five fields that need narrower source work before closure
review:

- sample design
- payment universe
- estimation method
- exclusion rules
- overpayment versus recoverable amount basis

Payment universe has two closed internal components but is not field-ready.
Sampled reconciliation PDE records, corrected-versus-reported GDC, and the
$96,521.39 million row-828 denominator are identified. Appendix A also closes
the two-track documentation treatment after a reconciliation PDE is adjusted,
but not the adjustment's inclusion, exclusion, denominator, weight, estimator,
or payment effect. Closure still requires the complete included/excluded stream
taxonomy and an authoritative bridge from combined plan-beneficiary GDC
liability to federal outlays, including adjustment, phase, overlap, and sample-
to-denominator rules.

Overpayment versus recoverable amount basis also has one closed internal
component but is not field-ready. The current scorecard documents an
audit-closeout PDE-deletion recovery process. Closure still requires same-cohort
amount, debt, appeal, settlement, collectibility, collection, write-off, and
control-cost lineage for the FY2024/CY2022 estimate.

Sample design is component-supported but not closure-ready. The same-period
record supports PDEs as the sampled unit, a statistically valid stratified-
random description, sponsor routing, the reconciliation cutoff, and statistical
governance. Closure still requires the national sample size, frame and coverage,
stratum definitions, allocation, inclusion probabilities, within-stratum
selection implementation, replacement and nonresponse rules, weights, and
linkage to the beneficiary simulation.

Estimation method has one closed published-output component but is not field-
closure-ready. Findings publish 95% dollar and rate bounds, and row 828 reports
its confidence label and 0.42 margin-of-error field without defining units or a
formula. Official APR process text remains without binary custody. Full closure
still requires formula, weights, PDE-to-beneficiary linkage, simulation, record
treatment, variance and confidence-limit construction, margin-of-error
definition, same-period 5% confirmation, and rounding mechanics.

Exclusion rules has one internally closed component but is not field-closure-
ready. Current sources resolve timely problematic-file review, unresolved
missing-document fail treatment, and predeadline cure. Closure still requires
the complete taxonomy, current counts and decision stages, submission-state
distinctions, post-deadline and appeal treatment, replacement and weights,
estimator effects, and continuity from the comparison-only FY2020 exclusions.

The inherited state-data label is resolved as sponsor documentation dependency
treatment. The CY2022 guide and FAQ establish missing-document fail treatment,
correction windows, and final review, while the FY2024 findings establish the
2.70% statistical documentation component.

Medicaid also has seven fields that need narrower source work before closure
review:

- sample design
- payment universe
- estimation method
- exclusion rules
- payment type split
- state rotation and weighting treatment
- improper payment versus fraud/waste basis

VA PLTSS has six fields that need narrower source work before closure review:

- sample design
- reviewed-claim universe
- estimation method
- exclusion rules
- documentation standard
- documentation defect versus recoverable overpayment basis

USDA Federal Crop Insurance has eight fields that need narrower source work
before closure review:

- sample design
- payment universe
- estimation method
- exclusion rules
- sample period
- payment type split
- agency-process-error definition
- recoverable savings basis

The ten closure decisions include the Part D, Medicaid, and VA PLTSS
sample-period fields; the Part D and VA PLTSS payment-type splits; the Part D
sponsor-documentation dependency treatment; and four FCIC fields.

## Boundary

These rows are readiness triage only. They do not close methodology fields, do
not estimate savings, and do not make a waste finding.
