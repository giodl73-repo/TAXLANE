# Solver input readiness rollup schema

`solver_input_readiness_rollup.v1.draft.json` summarizes the deterministic
solver input state after the reserve, net-interest, receipt-base, and
distribution placeholder contracts.

Required invariants:

- `record_family` is `solver_input_readiness_rollup`.
- `pulse` is `107`.
- linked paths include the solver-input inventory and the Pulse 102–106
  reserve, net-interest, assigned-base, and distribution artifacts;
- the rollup contains exactly the same twelve input IDs as
  `solver_input_inventory.v1.draft.json`;
- every row has `ready: false`, `value: null`, and at least one remaining
  blocker;
- aggregate readiness keeps solver, receipt-base, distribution, reserve,
  net-interest, trust-fund path, target-outlay, and deficit-gap readiness false;
- only `solver_input_readiness_rollup_published` may be true;
- no solver run, target-cost, statutory-rate, effective-rate, public-rate-card,
  tax-proposal, savings, waste, fraud, department-cut, technology, or
  balanced-budget claim may be published from this rollup.
