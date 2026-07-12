# Payment Integrity Methodology Field Updates

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_field_updates_q4_2025.jsonl`

This packet records internal methodology checklist repairs approved by priority
reviewer actions.

## Current Update

| Program | Old field | Revised field |
|---|---|---|
| USDA Federal Crop Insurance Program | agency-process-error definition | data-access outside-agency-control root-cause definition |

The USDA update repairs a stale field label, source target, and completion rule
so the checklist matches the current FCIC scorecard root-cause framing. It does
not close the field. The revised field still needs source work separating
data-access/outside-agency-control causes from recoverable or collectible
overpayments before any scoring.

## Boundary

These rows do not close fields, do not score programs, do not estimate savings,
do not identify waste, do not identify fraud, and do not claim recoverable
amounts.
