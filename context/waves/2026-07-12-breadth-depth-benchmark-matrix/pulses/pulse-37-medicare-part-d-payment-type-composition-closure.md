# Pulse 37: Medicare Part D Payment-Type Composition Closure

## Objective

Resolve the strongest remaining same-period payment-integrity methodology field
without conflating estimated overpayments with recoverability or savings.

## Evidence

The official PaymentAccuracy FY2024 annual workbook row for Medicare Part D,
source row 828, reports $3,052.65 million overpayments, $522.44 million
underpayments, zero technically improper payments, and zero unknown payments.
Those categories reconcile exactly to $3,575.09 million improper payments for
the January through December 2022 sample period.

## Decision

Close the Part D `payment type split` field internally. The annual row directly
answers the prior unknown-payment question with an explicit zero rather than a
missing value. Part D moves to two closed and six open methodology fields.

## Boundary and next action

Do not interpret the $3,052.65 million overpayment estimate as identified debt,
collectible dollars, recovery, collections, waste, or savings. Sample design,
payment universe, estimation method, exclusions, sponsor-data dependency, and
overpayment-versus-recoverable-amount basis remain open.

The next useful Part D evidence is a same-period source defining the included
and excluded Prescription Drug Event universe or the estimator, weights, and
uncertainty treatment. Keep all scoring and public claim gates blocked.
