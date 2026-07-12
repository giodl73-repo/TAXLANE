# Payment Integrity Methodology Open Program Status

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.jsonl`

This packet summarizes all four methodology branches without requiring a
closure-coverage or scoring-gate row for programs that have no closure decisions
yet.

## Current Status

| Program | Closed fields | Open fields | Status |
|---|---:|---:|---|
| Medicare Prescription Drug Benefit (Part D) | 1 | 7 | closure coverage available |
| Medicaid | 1 | 7 | closure coverage available |
| VA PLTSS | 0 | 8 | fully open |
| USDA Federal Crop Insurance Program | 0 | 8 | fully open |

VA PLTSS and USDA Federal Crop Insurance are not eligible for the existing
closure-coverage/scoring-gate rollup yet because they have no full-field
internal closure decisions. Later component-progress rows may record narrow
internal decisions, but those do not change the field counts above. USDA also
has a root-cause framing mismatch: the current FCIC scorecard uses
data-access/outside-agency-control wording, not the older agency-process-error
field framing.

## Boundary

These rows are internal status summaries only. They do not close fields, do not
estimate savings, do not identify waste, do not identify fraud, and do not claim
recoverable amounts.
