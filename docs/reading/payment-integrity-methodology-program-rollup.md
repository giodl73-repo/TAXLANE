# Payment Integrity Methodology Program Rollup

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_program_rollup_q4_2025.jsonl`

This packet summarizes the current methodology gate status for the covered
payment-integrity programs before moving to another program branch.

## Current Status

Medicare Part D, Medicaid, and VA PLTSS are still blocked from scoring.

| Program | Closed fields | Open fields | Gate |
|---|---:|---:|---|
| Medicare Prescription Drug Benefit (Part D) | 3 | 5 | blocked_methodology_incomplete |
| Medicaid | 1 | 7 | blocked_methodology_incomplete |
| VA Purchased Long Term Services and Supports | 2 | 6 | blocked_methodology_incomplete |

The open fields are still source-work items, not findings. They identify where
the methodology chain needs more official documentation before TaxLane can
estimate recoverable amounts, savings opportunities, waste, or fraud.

For Part D, the five open fields are sample design, payment universe,
estimation method, exclusion rules, and overpayment versus recoverable amount
basis.

## Boundary

These rows are internal status rollups. They are not savings estimates, waste
findings, fraud claims, recoverable-dollar claims, or public claims.
