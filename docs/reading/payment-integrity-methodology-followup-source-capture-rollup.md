# Payment Integrity Methodology Follow-Up Source Capture Rollup

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_followup_source_capture_rollup_q4_2025.jsonl`

This packet rolls up follow-up recoverability source captures into reviewer
actions.

## Rollup Results

| Rank | Program | Status |
|---:|---|---|
| 1 | USDA Federal Crop Insurance Program | Reviewer boundary decision needed. |
| 2 | VA PLTSS | Reviewer boundary decision needed. |
| 3 | Medicaid | Additional positive recoverable-basis source needed. |
| 4 | Medicare Part D | Reviewer boundary decision needed. |

Medicaid is the clearest blocker: the captured CMS source supports claim
guarding, not scoring. USDA, VA PLTSS, and Part D have partial recovery-process
or mapping support, but still need reviewer decisions before any field can move
toward closure-readiness.

For PLTSS, current policy now closes the agencywide definition question. The
remaining need is program-specific cause-to-category, bill, dispute, and
certified-collection lineage.

## Boundary

These rows do not close fields, do not score programs, do not estimate savings,
do not identify waste, do not identify fraud, and do not claim recoverable
amounts.
