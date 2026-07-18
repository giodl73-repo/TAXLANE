# Reserve rule contract

Machine record:
`data/derived/breadth_benchmark_matrix/reserve_rule_contract.v1.draft.json`

Pulse 102 defines the reserve fields required before the deterministic solver
can run. It does not choose numeric reserve parameters.

Required but still null:

- reserve contribution;
- reserve withdrawal;
- reserve balance;
- reserve cap;
- emergency override rule;
- cyclical shortfall draw rule;
- surplus routing rule;
- future-year payback rule;
- public rounding residual line.

The contract keeps the over-the-cycle reserve rule: a cyclical shortfall draws
the reserve before any rate increase or benefit cut. Surpluses route to the
reserve, then debt, before a discretionary rate cut.

cyclical shortfall draws the reserve before any rate increase or benefit cut.

This is a reserve rule contract, not reserve parameters, not a solver run, not target-cost selection, not rate calculation, not a public rate card, not a tax proposal, not a savings estimate, not a waste finding, not a fraud finding, not a department-cut instruction, not a technology-savings claim, and not a balanced-budget claim.
