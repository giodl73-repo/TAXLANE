# Payment Integrity Methodology Source Capture Rollup

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_source_capture_rollup_q4_2025.jsonl`

This packet summarizes the Medicare Part D, Medicaid, VA PLTSS, and USDA
Federal Crop Insurance methodology source-capture pass.

## Current Status

All eight Part D methodology gap-followup rows, all eight Medicaid methodology
gap-followup rows, all eight VA PLTSS methodology gap-followup rows, and all
eight USDA Federal Crop Insurance gap-followup rows now have linked
source-capture rows. The rollup converts those captures into reviewer actions:

- decide whether the capture is enough for field closure; or
- queue a narrower source gap for missing details.

Closure-readiness rows are now built for Part D, Medicaid, VA PLTSS, and USDA
Federal Crop Insurance. VA PLTSS and USDA Federal Crop Insurance remain fully
open and need narrower source work before any closure.

## Boundary

These rows do not close methodology fields. They do not estimate savings and do
not make a waste finding.
