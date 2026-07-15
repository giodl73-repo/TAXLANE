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

Medicaid has one internally closed sample-period field, while Part D has closed
sample period, payment type split, and sponsor documentation dependency
treatment. Part D sample design also has supported components but remains open
for sample size, frame, strata and allocation, probabilities, selection
implementation, unit-treatment rules, weights, and beneficiary-simulation
linkage. The selected recoverability blocker still prevents public scoring
because improper-payment and overpayment figures are not automatically fraud,
waste, recoverable amounts, or collectible savings.

For Part D, the current audit-closeout PDE-deletion process component is now
preserved, but rank 4 is unchanged. The next query targets same-cohort national-
audit, PDE-deletion, debt, and collection lineage for FY2024/CY2022; the Q4 2025
process alone supplies no amount or cohort linkage.

Part D estimation method likewise remains open. Its web-verified APR process
text lacks official custody, and the captured findings do not provide formula,
weights, aggregation, simulation mechanics, PDE-to-beneficiary sample linkage,
record treatment, variance, same-period 5% confirmation, or reconciliation.
These residuals do not displace recoverability as the selected scoring blocker.

Part D exclusion rules also remains open despite one closed component. The
current-cycle missing-document review, failed-status, and cure boundary is
resolved; taxonomy, counts, decision stages, submission-state distinctions,
post-deadline and appeal treatment, replacement and weights, estimator effects,
and historical continuity are not. FY2020's 27 exclusions remain comparison-
only, and this component does not displace recoverability as the selected
scoring blocker.

## Boundary

These rows do not close methodology fields, do not estimate savings, do not
identify waste, do not identify fraud, and do not claim recoverable amounts.
