# Medicare HI receipt-base reconciliation schema

This schema describes `medicare_hi_receipt_base_reconciliation` records.

Required fields:

- Identity fields and paths to the target-cost contract, official source capture
  record, receipt-base reconciliation gap, dedicated-receipt anchors, and
  rate-publication readiness rollup.
- Source-custody status booleans.
- `source_context` with CMS Medicare HI taxable payroll, CMS HI payroll-tax
  yield context, OMB Hospital Insurance receipt anchor, and a diagnostic
  source-yield-to-payroll ratio.
- `reconciliation` with difference formulas and explicit non-interchangeability
  status.
- `remaining_gates`, `blocked_outputs`, `public_warning_phrases`, and
  `claim_booleans`.

The diagnostic ratio is not a statutory rate, effective rate, public rate card,
solver input, tax proposal, savings estimate, or balanced-budget claim.
