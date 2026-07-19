# Medicare HI perimeter-bridge requirements schema

This schema describes `medicare_hi_perimeter_bridge_requirements` records.

Required fields:

- Identity fields and paths to the target-cost contract, Medicare HI receipt-base
  reconciliation, receipt-base reconciliation gap, and rate-publication
  readiness rollup.
- Source-custody status booleans.
- `bridge_scope` with trust-fund separation, CMS/OMB context values, unreconciled
  difference, and diagnostic-ratio publication boundary.
- `required_bridge_components`, each with a question, evidence requirements,
  null value, false readiness, and incomplete status.
- `summary`, `blocked_outputs`, `public_warning_phrases`, and `claim_booleans`.

The record defines bridge requirements only. It must not publish a completed
perimeter bridge, assigned base, statutory rate, effective rate, solver input,
public rate card, tax proposal, savings estimate, waste/fraud finding,
technology-savings claim, or balanced-budget claim.
