# Net-interest formula contract

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_formula_contract.v1.draft.json`

Pulse 104 defines the formula boundary for endogenous net interest. It does not publish an admitted debt path, net-interest path, solver run, rate calculation, or balanced-budget result.

Now admitted from the hash-custodied CORE-G CBO spine:

- baseline debt stock;
- baseline net interest;
- explicit other financing series;
- new borrowing timing rule, using a midpoint central rail and early/late
  sensitivity bounds;
- matching-vintage CBO FY2025–FY2035 average-interest-rate path;
- incremental interest-receipt treatment;
- a reduced-form primary-balance feedback fixture; and
- an inclusive trailing-12-month MSPD term mix with deterministic monthly
  refinancing for marginal policy-debt deltas.

All eight formula inputs are now ready for marginal incremental feedback. The
[empirical rollover convention](net-interest-mspd-empirical-rollover-convention.md)
supplies the maturity input in that scope. It does not supply the future total
gross issuance needed to reproduce CBO's entire debt stock.

Bucket-specific rates and stress remain a completion-step gap even though the
matching-vintage aggregate CBO rate path is ready for bounded incremental
feedback.

The [compatibility audit](net-current-law-baseline-compatibility-audit.md)
records why the CBO average-rate path is compatible with reduced-form policy
deltas while the empirical mix and latest-month Treasury contexts cannot
substitute for a full-stock bucket path.

Net interest is endogenous. Net interest cannot be cut directly. After any primary-balance change, the model must recompute deficit, debt, and subsequent net interest; the full-stock mode must additionally recompute maturity-bucket debt stock.

The regression test contract is named `primary_balance_change_recomputes_debt_and_interest`. Its fixture points to the [CBO average-rate feedback model](net-interest-cbo-average-rate-feedback.md), which proves zero-input stability and later-year debt-service movement for a mechanical nonzero shock. The separate empirical fixture proves marginal maturity-aware rollover. Full-stock feedback and bucket-rate stress remain blocked.

This is a net-interest formula contract, not a net-interest path, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
