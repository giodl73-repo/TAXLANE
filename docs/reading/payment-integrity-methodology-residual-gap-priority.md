# Payment Integrity Methodology Residual Gap Priority

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_residual_gap_priority_q4_2025.jsonl`

This packet selects one next residual source gap for each open methodology
program branch. It is an action queue for source work, not a scoring queue.

## Priority Queue

| Rank | Program | Selected blocker |
|---:|---|---|
| 1 | USDA Federal Crop Insurance Program | agency-process-error definition |
| 2 | VA PLTSS | documentation defect versus recoverable overpayment basis |
| 3 | Medicaid | improper payment versus fraud/waste basis |
| 4 | Medicare Part D | overpayment versus recoverable amount basis |

The first two rows protect branches with no closure decisions. USDA is first
because its current scorecard root-cause wording conflicts with the older field
framing. VA PLTSS is second because documentation defects require a
recoverability boundary before any waste or savings interpretation.

Medicaid and Part D already have one internally closed sample-period field, but
their selected blockers still prevent public scoring because improper-payment
and overpayment figures are not automatically fraud, waste, recoverable amounts,
or collectible savings.

## Boundary

These rows do not close methodology fields, do not estimate savings, do not
identify waste, do not identify fraud, and do not claim recoverable amounts.
