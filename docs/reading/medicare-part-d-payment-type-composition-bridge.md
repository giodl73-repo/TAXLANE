# Medicare Part D Payment-Type Composition Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_part_d_payment_type_composition_bridge.fy2024.v1.draft.json`.

## Result

The official PaymentAccuracy FY2024 annual row publishes a complete Part D
composition for the January–December 2022 sample period. It reports $3,052.65
million overpayments, $522.44 million underpayments, zero technically improper
payments, and zero unknown payments. The components reconcile exactly to
$3,575.09 million improper payments, and improper plus unknown remains
$3,575.09 million.

The zero unknown-payment amount is an explicit reported value, not missing
source treatment. This closes the Part D `payment type split` internally.
Part D moves from one closed and seven open fields to two closed and six open.

## Claim firewall

The $3,052.65 million overpayment component is a statistical estimate, not an
identified-debt, collectible, recovered, or collected amount. The composition
does not close sample design, payment universe, estimator, exclusions,
sponsor-data treatment, or overpayment-versus-recoverable-amount basis.

No score or public claim about debt, collectibility, recovery, fraud, waste, or
savings is allowed.
