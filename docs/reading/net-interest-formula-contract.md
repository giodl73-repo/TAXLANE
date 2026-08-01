# Net-interest formula contract

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_formula_contract.v1.draft.json`

Pulse 104 defines the formula boundary for endogenous net interest. It does not publish a debt path, net-interest path, solver run, rate calculation, or balanced-budget result.

Now admitted from the hash-custodied CORE-G CBO spine:

- baseline debt stock;
- baseline net interest;
- explicit other financing series.

Required but still null:

- maturity bucket schedule;
- effective rate path by bucket;
- new borrowing timing rule;
- interest receipts treatment;
- primary-balance feedback test fixture.

The [compatibility audit](net-current-law-baseline-compatibility-audit.md)
records why the aggregate annual inputs are compatible while Treasury's
latest-month maturity and rate contexts cannot yet substitute for bucket paths.

Net interest is endogenous. Net interest cannot be cut directly. After any primary-balance change, the solver must recompute deficit, debt, maturity-bucket debt stock, and subsequent net interest.

The regression test contract is named `primary_balance_change_recomputes_debt_and_interest`, but its fixture path is still null until the maturity and bucket-rate inputs exist. The separate zero-policy topline replay is ready; it is not a primary-balance feedback test.

This is a net-interest formula contract, not a net-interest path, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
