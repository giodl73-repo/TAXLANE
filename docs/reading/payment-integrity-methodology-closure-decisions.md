# Payment Integrity Methodology Closure Decisions

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_closure_decisions_q4_2025.jsonl`

This packet records internal methodology closure decisions.

## Current Decision

The Medicare Part D sample-period field is closed internally. The decision is
narrow: the PaymentAccuracy Q4 2025 scorecard states that the FY2024 estimate is
based on a sampling timeframe starting 1/2022 and ending 12/2022.

The Medicare Part D payment-type split is also closed internally. The exact
FY2024 annual row reports $3,052.65 million overpayments, $522.44 million
underpayments, and zero technical or unknown payments, which reconciles to the
$3,575.09 million improper-payment estimate. The CMS findings PDF corroborates
the rounded categories; none of these amounts establishes debt or recovery.

The Medicare Part D sponsor-documentation dependency treatment field is also
closed internally. The CY2022 guide and FAQ show that missing documentation
leaves a sampled PDE in fail status, while approved resets and timely
resubmissions can cure the record before final review. CMS's FY2024 findings
place documentation errors in a 2.70% statistical overpayment component. A
failed PDE is a measurement status, not proof of debt, fraud, or recovery.

The Medicaid sample-period field is also closed internally. The decision is
narrow: the PaymentAccuracy Q4 2025 Medicaid scorecard states that the FY2024
estimate is based on a sampling timeframe starting 7/2022 and ending 6/2023.

The VA PLTSS sample-period field is now closed internally. The scorecard and
annual workbook state that the FY2024 estimate uses an October 2022 through
September 2023 sample, and VA OIG independently reviewed the sampling plans,
estimates, and margins of error.

The VA PLTSS payment-type split is also closed internally. The same-period
FY2024 annual row completely reconciles overpayment, underpayment, technically
improper, and unknown amounts. The FY2025 AFR corroborates the taxonomy for a
later cycle but is not blended with the FY2024 estimate.

There are ten internal closure decisions across the four programs. No other
Part D, Medicaid, or VA PLTSS methodology field is closed.

## Boundary

This is an internal field-level methodology decision. It does not estimate
savings, does not make a waste finding, and does not create a public claim.
