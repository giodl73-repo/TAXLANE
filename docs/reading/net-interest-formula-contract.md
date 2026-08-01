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
- incremental interest-receipt treatment; and
- a reduced-form primary-balance feedback fixture.

Required but still null for the full-stock model:

- complete maturity bucket schedule. Exact existing-stock runoff and public-
  holder bounds are ready, but future issuance and rollover are not;

Bucket-specific rates and stress remain a completion-step gap even though the
matching-vintage aggregate CBO rate path is ready for bounded incremental
feedback.

The [compatibility audit](net-current-law-baseline-compatibility-audit.md)
records why the CBO average-rate path is compatible with reduced-form policy
deltas while Treasury's latest-month maturity and rate contexts cannot yet
substitute for a full-stock bucket path.

Net interest is endogenous. Net interest cannot be cut directly. After any primary-balance change, the model must recompute deficit, debt, and subsequent net interest; the full-stock mode must additionally recompute maturity-bucket debt stock.

The regression test contract is named `primary_balance_change_recomputes_debt_and_interest`. Its fixture now points to the [CBO average-rate feedback model](net-interest-cbo-average-rate-feedback.md), which proves zero-input stability and later-year debt-service movement for a mechanical nonzero shock. It is reduced-form; maturity-aware feedback remains blocked.

This is a net-interest formula contract, not a net-interest path, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
