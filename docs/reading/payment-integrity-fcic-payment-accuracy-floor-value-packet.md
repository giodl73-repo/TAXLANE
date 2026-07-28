# Payment Integrity FCIC Payment Accuracy Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/payment_integrity_fcic_payment_accuracy_floor_value_packet.v1.draft.json`

Source bridge:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json`

This is a Wave D floor-value packet for the non-additive payment-integrity
overlay. It converts the source-reviewed Federal Crop Insurance Corporation
payment-accuracy baseline into a draft threshold rationale and baseline value,
but it does not pass or fail any policy scenario.

Draft threshold rule: a payment-integrity control cannot pass this FCIC
payment-accuracy floor if reviewed policy and stress evidence show the FCIC
payment accuracy rate falling below the FY2024 annual-workbook baseline.

Selected baseline and threshold:

| Field | Value |
| --- | ---: |
| FCIC payment accuracy threshold floor | 97.5725794 percent |
| FCIC payment accuracy baseline | 97.5725794 percent |
| Covered outlays | $23,867.31 million |
| Properly paid amount | $23,287.95 million |
| Improper payment rate | 2.4274206 percent |
| Improper payment amount | $579.36 million |
| Overpayment rate | 2.4046698 percent |
| Underpayment rate | 0.0227508 percent |
| RMA FY2024 sample policy count | 326 policies |

This baseline is component-specific and statistical. It is not a transaction
debt list, not a false-positive rate, not due-process evidence, not
causal-prevention lineage, not same-cohort collection lineage, not a fraud
finding, not a waste finding, and not a savings estimate.

This is not role-reviewed final threshold selection, not policy values, not
stress values, not pass/fail evidence, not lower-cost scenario admissibility,
not solver input, not rate calculation, not gross savings, not net savings, and
not a balanced-budget claim.

Compact validator phrase: draft no-regression FCIC payment-accuracy floor threshold.
Compact validator phrase: not a false-positive model.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a balanced-budget claim.
