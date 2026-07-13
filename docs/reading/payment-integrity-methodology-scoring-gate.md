# Payment Integrity Methodology Scoring Gate

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_scoring_gate_q4_2025.jsonl`

This packet records whether the Medicare Part D, Medicaid, and VA PLTSS
payment-integrity methodology records are ready for scoring.

## Current Gate

Scoring is blocked because methodology closure is incomplete. For each covered
program, only one of eight fields is internally closed.

Part D blockers:

- 7 open methodology fields
- recoverable or collectible amount basis is not established
- estimator formula, weights, and uncertainty treatment are not established
- included and excluded payment universe is not established

Medicaid blockers:

- 7 open methodology fields
- estimator formula, weights, variance, and confidence-limit treatment is not established
- included and excluded payment universe is not established
- state-cycle weighting and national aggregation mechanics are not established
- recoverable or collectible amount basis is not established

VA PLTSS blockers:

- 7 open methodology fields
- sample frame, sample size, and selection method are not published
- reviewed-claim universe, exclusions, estimator formula, and weights remain open
- projected errors are not linked to established receivables or collections
- veteran access, appeal, and provider-burden floors remain open

## Boundary

These rows block scoring. They are not savings estimates, waste findings, fraud
claim, or public claim.
