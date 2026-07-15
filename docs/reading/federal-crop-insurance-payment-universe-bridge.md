# Federal Crop Insurance Payment-Universe Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/federal_crop_insurance_payment_universe_bridge.fy2024.v1.draft.json`.

This bridge uses FCIC/RMA's FY2024 financial report and the existing COM-23-001
capture to close one additional methodology field internally: `payment
universe`.

## What The Sources Add

On printed page 18, FCIC/RMA says its improper-payment rate considers all
categories of payments and names them: premium subsidy, Administrative and
Operating (A&O) expense, and indemnities. It also says payments are separated
by Approved Insurance Provider into high, medium, and low tiers, after which an
annual statistically valid sample is drawn and reviewed.

COM-23-001 complements that universe description by requiring all applicable
policyholder and AIP documentation for the 326 statistically selected policies
to be submitted through the Compliance Activities Results System.

Printed pages 60-61 repeat the FY2024 outlays, improper-payment estimate, and
2.43% rate and state that payment-integrity methodologies did not change. The
source locations are PDF file page 29 and pages 71-72 respectively; some viewers
display the zero-based indices as 28 and 70-71.

## Closure Decision

The explicit statement that all payment categories are considered, together
with the enumeration of the three categories, is sufficient to close the
payment-universe field for internal methodology tracking. FCIC now has four
closed fields and four open fields. The remaining open fields are sample
design, estimation method, exclusion rules, and recoverable-savings basis.

The high/medium/low AIP tiers are useful sample-design evidence, but they do not
disclose frame construction, tier allocation, selection probabilities,
randomization details, weights, projection method, or variance method. The
sources also do not disclose policy-level or review exclusions. Those fields
remain open.

## Unaudited Information And Numeric Conflict

The report labels printed pages 60-61 as Other Information (Unaudited), and
printed page 18 is part of Management's Discussion and Analysis (Unaudited).
The FY2024 payment-integrity table appears to print `$579.93M` as overpayments.
That conflicts with the authoritative PaymentAccuracy annual workbook value of
`$573.93M` and appears to be a typographical error. The bridge does not use the
report's `$579.93M` cell for any numeric reconciliation or claim.

## Claim Firewall

This field closure defines the included payment categories. It does not
establish identified debt, collectibility, collections, recovery, prevention,
fraud, control cost, or savings. Program scoring and all public, fraud,
recovery, prevention, and savings claims remain blocked.
