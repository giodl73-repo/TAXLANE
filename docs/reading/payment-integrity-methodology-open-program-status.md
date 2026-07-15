# Payment Integrity Methodology Open Program Status

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_methodology_open_program_status_q4_2025.jsonl`

This packet summarizes all four methodology branches without requiring a
closure-coverage or scoring-gate row for programs that have no closure decisions
yet.

## Current Status

| Program | Closed fields | Open fields | Status |
|---|---:|---:|---|
| Medicare Prescription Drug Benefit (Part D) | 3 | 5 | closure coverage available |
| Medicaid | 1 | 7 | closure coverage available |
| VA PLTSS | 2 | 6 | closure coverage available |
| USDA Federal Crop Insurance Program | 4 | 4 | closure coverage available |

All four programs have closure coverage. Part D has closed sample period,
payment type split, and sponsor documentation dependency treatment. PLTSS has
closed sample period and payment type split; both programs' recoverability
fields remain open. FCIC has four internal closures and four residual gaps.

Part D also has a narrow payment-universe component closure for the PDE/GDC
measurement object and published denominator reconciliation. Full payment
universe stays open pending the complete included/excluded stream taxonomy,
combined plan-beneficiary liability to federal-outlay mapping, and denominator
construction rules. The program remains three closed and five open with three
closure decisions, five residual gaps, and every scoring and claim gate false.

Part D now also has a reconciliation-PDE adjustment-documentation component.
After an adjustment, reconciliation-PDE-aligned documentation remains required
and linked adjustment documentation is additionally required. The cutoff and
final reconciliation target are prior context, and no inclusion, exclusion,
denominator, weight, estimator, or payment effect is disclosed. Counts,
decisions, residual gaps, and every gate remain unchanged.

Part D also has a current audit-closeout PDE-deletion recovery-process component.
That later Q4 2025 process has no amount or cohort linkage to the FY2024/CY2022
estimate, so the recoverable-amount field remains open for debt, appeal,
settlement, collectibility, collection, write-off, liability allocation, and
control-cost lineage. Counts, decisions, open status, and all gates are unchanged.

Part D also has a published confidence-interval and margin-of-error output
component closure. The official findings and row 828 preserve the reported
outputs, but the 0.42 margin-of-error field has no disclosed units or formula
and is not forced to reconcile to the findings bounds. APR custody and the full
estimator remain open. The program therefore remains three closed and five open
with three closure decisions, five residual gaps, and every scoring and claim
gate false.

## Boundary

These rows are internal status summaries only. They do not close fields, do not
estimate savings, do not identify waste, do not identify fraud, and do not claim
recoverable amounts.
