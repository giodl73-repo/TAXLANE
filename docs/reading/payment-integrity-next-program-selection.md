# Payment Integrity Next Program Selection

Machine rows:
`data/derived/efficiency_pressure/extracts/payment_integrity_next_program_selection_q4_2025.jsonl`

This packet records payment-integrity branch handoffs after methodology gates
are built.

## Current Selection

The current selected branch is USDA Federal Crop Insurance Program. It is the
remaining PaymentAccuracy scorecard branch already scaffolded with methodology
fields, source targets, queries, and pending query-run rows.

Current Federal Crop Insurance starting fields:

- sample design
- payment universe
- estimation method
- exclusion rules
- sample period
- payment type split
- agency-process-error definition
- recoverable savings basis

## Prior Handoffs

VA Purchased Long Term Services and Supports (PLTSS) remains in the file as the
prior selected branch. Its source-review chain is built, but closure remains
blocked because the captured official sources do not close the program-specific
methodology fields.

VA PLTSS starting fields:

- sample design
- reviewed-claim universe
- estimation method
- exclusion rules
- sample period
- payment type split
- documentation standard
- documentation defect versus recoverable overpayment basis

The Medicaid/PERM row remains as the prior selected branch. It points to
official PaymentAccuracy and CMS source pages and now links to a methodology
plan, field checklist, source captures, closure coverage, and scoring gate. The
gate still blocks scoring because seven methodology fields remain open.

## Boundary

These are branch-selection rows only. They are not savings estimates, waste
findings, fraud claims, recoverable-dollar claims, or public claims.
