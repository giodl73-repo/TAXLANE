# Medicare HI CY2025-CY2035 Current-Law Context Path

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_hi_cy2025_2035_current_law_context_path.v1.draft.json`

Pulse 207 adds a source-custodied CMS Trustees context path for the Medicare
Hospital Insurance Trust Fund. The values come from the 2026 Medicare Trustees
Report, Table II.E1, `Estimated Operations of the HI Trust Fund under
Intermediate Assumptions, Calendar Years 2025-2035`.

What is now usable:

- CY2025 actual HI income, expenditures, net change, fund balance, and asset
  ratio context;
- CY2026-CY2035 intermediate-assumption calendar-year rows;
- explicit post-depletion handling for 2033 and later, including the Trustees'
  hypothetical-after-depletion boundary.

What remains blocked:

- fiscal-year HI path values for OMB solver use;
- OMB/CMS receipt-row bridge and matched solver yield;
- explicit transfer schedule and fund reconciliation;
- health floor thresholds, observed values, policy values, stress values, and
  pass/fail findings;
- solver inputs, rates, public rate cards, savings, technology-savings claims,
  and balanced-budget claims.

This is real Medicare HI current-law source progress, but it is calendar-year
context, not a fiscal-year solver input.
