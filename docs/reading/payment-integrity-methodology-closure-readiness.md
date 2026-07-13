# Payment Integrity Methodology Closure Readiness

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_readiness_q4_2025.jsonl`

This packet triages the Medicare Part D, Medicaid, VA PLTSS, and USDA Federal
Crop Insurance methodology source-capture rollups into reviewer next steps.

## Current Status

Three fields are closure-review candidates:

- Part D sample period
- Medicaid sample period
- VA PLTSS sample period

Part D still has seven fields that need narrower source work before closure
review:

- sample design
- payment universe
- estimation method
- exclusion rules
- payment type split
- state-data dependency treatment
- overpayment versus recoverable amount basis

Medicaid also has seven fields that need narrower source work before closure
review:

- sample design
- payment universe
- estimation method
- exclusion rules
- payment type split
- state rotation and weighting treatment
- improper payment versus fraud/waste basis

VA PLTSS has seven fields that need narrower source work before closure review:

- sample design
- reviewed-claim universe
- estimation method
- exclusion rules
- payment type split
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

Closure decisions are built for the Part D, Medicaid, and VA PLTSS sample-period
fields. USDA Federal Crop Insurance has no closure decision.

## Boundary

These rows are readiness triage only. They do not close methodology fields, do
not estimate savings, and do not make a waste finding.
