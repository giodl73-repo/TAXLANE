# Pulse 104 — Net-interest formula contract

Pulse 104 adds
`data/derived/breadth_benchmark_matrix/net_interest_formula_contract.v1.draft.json`.

The contract defines the endogenous net-interest formula boundary required
before the deterministic solver can publish debt, interest, rate, or
balanced-budget outputs. It records primary balance, deficit, debt[t],
net-interest[t], and the required iteration rule.

No net-interest path is calculated. Debt, maturity, rate, new-borrowing timing,
interest-receipt, other-financing, and feedback-test fixture values remain null.
All solver, rate, target-cost, savings, waste, fraud, technology,
department-cut, tax-proposal, public-rate-card, and balanced-budget claims stay
blocked.
