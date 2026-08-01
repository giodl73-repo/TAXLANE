# NET current-law baseline completion plan

Machine record:
`data/derived/breadth_benchmark_matrix/net_current_law_baseline_completion_plan.v1.draft.json`.

## Current usable spine

The custodied OMB PBD path provides seven net-interest context rows. The
existing CORE-G CBO spine additionally provides a hash-custodied FY2025–FY2035
horizon, annual debt held by the public, annual net interest, explicit other
financing and timing, and a deterministic zero-policy topline reconciliation.
The [compatibility audit](net-current-law-baseline-compatibility-audit.md)
therefore marks seven of nine completion steps ready. The explicit borrowing-
timing convention, reduced-form feedback fixture, and source-replayed empirical
marginal rollover are usable without treating them as a full-stock debt model.

## Completion sequence

1. Extend or explicitly bound the horizon beyond FY2031.
2. Add source-custodied annual debt stock.
3. Add maturity-bucket stock and rollover rules. **Ready for marginal
   feedback:** existing-stock runoff and public-holder intervals are paired
   with an inclusive trailing-12-month issuance mix and deterministic monthly
   rollover. Future total issuance remains blocked for full-stock forecasting.
4. Add effective-rate paths by bucket and a stress path.
   **Partial:** matching-vintage CBO aggregate average rates support reduced-
   form feedback; bucket mapping and stress remain blocked.
5. Define within-year new-borrowing timing. **Ready:** midpoint central rail,
   early/late boundaries, closed-form interest financing, and display rounding.
6. Reconcile gross interest, interest receipts, and net interest. **Partial:**
   OMB reconciles, but matching CBO-vintage component values remain blocked.
7. Add explicit other-financing rows.
8. Build a zero-policy-change regression fixture.
9. Apply a primary-balance shock and verify subsequent debt and interest move
   through the formula. **Ready for reduced-form testing:** zero and mechanical
   nonzero fixtures pass; policy output still requires an admitted shock.

NET remains an endogenous accounting result rather than a source of direct
savings. The five candidate assessments produce no admitted primary-balance
shock, so the working fiscal result still triggers no interest recomputation.
