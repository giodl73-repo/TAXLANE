# FY2025 fund-group reconciliation fixture schema

Schema for `fund_group_fy2025_reconciliation_fixture.v1.draft.json`.

Required invariants:

- `record_id = fund-group-fy2025-reconciliation-fixture:v1`.
- The source must be the already-local OMB Appendix Chapter 13 funds PDF.
- Raw byte count and SHA-256 must be recomputed from the local file.
- Values use tenths of billions as published in the source tables, not invented
  precision.
- Table 13-1 receipt components must sum to the published unified receipt total.
- Table 13-1 outlay components must expose the public rounding residual line.
- Deficit must also be recorded as positive financing need.
- Table 13-3 fund-balance arithmetic must reconcile.
- The fixture is aggregate federal-fund/trust-fund context only; it is not
  transportation-specific trust-fund values.
- Missing transportation outputs remain `null`.
- Only `aggregate_fund_group_fixture_published` may be `true`; all rate, savings,
  solver, target-cost, waste, fraud, department-cut, technology-savings, and
  balanced-budget booleans remain `false`.
