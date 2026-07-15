# VA PLTSS Payment-Type Composition Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/va_pltss_payment_type_composition_bridge.fy2025.v1.draft.json`.

## Result

The same-period PaymentAccuracy FY2024 annual row already provides a complete
PLTSS composition: $218.30 million overpayments, $6.41 million underpayments,
$432.46 million technically improper payments, and $102.93 million unknown
payments. The first three reconcile to $657.17 million improper payments, and
improper plus unknown reconciles to $760.10 million. This is the primary basis
for closing the FY2024 `payment type split` field internally.

VA's FY2025 Agency Financial Report independently publishes a complete PLTSS
payment-category table for a later cycle testing payments made in FY2024. Of
$7,330.11 million in outlays, VA reports
$6,982.69 million proper, $301.58 million improper, and $45.84 million unknown.
The improper amount separates into $77.08 million overpayments and $224.50
million non-monetary loss; the latter separates into $1.82 million underpayments
and $222.68 million technically improper payments. VA reports $347.41 million,
or 4.74 percent, improper plus unknown and notes that totals may vary because of
rounding.

The later table corroborates that VA continues to distinguish the same payment
categories. PLTSS now has two closed fields and six open fields.

## Period boundary

The FY2025 AFR says VA tested and reported payments made in FY2024. This is not
the earlier FY2024 annual/scorecard estimate, whose sample ran from October
2022 through September 2023 and whose amounts differ. The field closure uses
the earlier same-period annual row; the two compositions must not be combined
or presented as a same-cohort reconciliation.

## Recoverability firewall

VA separately says post-payment reviews may result in additional payments or
bills of collection for claims paid in the incorrect amount. It does not map
the table's estimated categories or individual documentation defects to bills,
appeals, write-offs, collections, or recoverable savings. Technically improper
and unknown payments are not recoverable debt by inference. The $77.08 million overpayment estimate
is not an established-debt or collection total.

The documentation-standard and documentation-defect-versus-recoverable-
overpayment fields remain open. No recovery rate, savings estimate, fraud,
waste, debt, collectibility, or public performance claim is allowed.
