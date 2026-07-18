# Net-interest formula contract

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_formula_contract.v1.draft.json`

Pulse 104 defines the formula boundary for endogenous net interest. It does not publish a debt path, net-interest path, solver run, rate calculation, or balanced-budget result.

Required but still null:

- baseline debt stock;
- baseline net interest;
- maturity bucket schedule;
- effective rate path by bucket;
- new borrowing timing rule;
- interest receipts treatment;
- explicit other financing series;
- primary-balance feedback test fixture.

Net interest is endogenous. Net interest cannot be cut directly. After any primary-balance change, the solver must recompute deficit, debt, maturity-bucket debt stock, and subsequent net interest.

The regression test contract is named `primary_balance_change_recomputes_debt_and_interest`, but its fixture path is still null until the official debt, maturity, and rate inputs exist.

This is a net-interest formula contract, not a net-interest path, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
