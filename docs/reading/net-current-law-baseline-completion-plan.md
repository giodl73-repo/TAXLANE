# NET current-law baseline completion plan

Machine record:
`data/derived/breadth_benchmark_matrix/net_current_law_baseline_completion_plan.v1.draft.json`.

## Current usable spine

The custodied OMB PBD path provides seven net-interest context rows: FY2025
actual and FY2026–FY2031 projections. It is a source target for reconciliation,
not an endogenous debt model.

## Completion sequence

1. Extend or explicitly bound the horizon beyond FY2031.
2. Add source-custodied annual debt stock.
3. Add maturity-bucket stock and rollover rules.
4. Add effective-rate paths by bucket and a stress path.
5. Define within-year new-borrowing timing.
6. Reconcile gross interest, interest receipts, and net interest.
7. Add explicit other-financing rows.
8. Build a zero-policy-change regression fixture.
9. Apply one admitted primary-balance shock and verify subsequent debt and
   interest move through the formula.

Until all inputs reconcile, NET remains an accounting guardrail rather than a
source of direct savings. The five candidate assessments produce no admitted
primary-balance shock, so they trigger no interest recomputation.

