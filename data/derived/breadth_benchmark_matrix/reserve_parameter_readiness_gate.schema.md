# Reserve parameter readiness gate schema

`reserve_parameter_readiness_gate.v1.draft.json` is a gate record for the
reserve parameters required by `reserve_rule_contract.v1.draft.json`.

Required invariants:

- `record_family` is `reserve_parameter_readiness_gate`.
- `pulse` is `103`.
- the reserve-rule contract, solver-input inventory, target-cost contract,
  balance guardrail, and rate-adjustment operating model paths are explicit;
- every `required_parameter_decisions` row has `required: true`,
  `ready: false`, `current_value: null`, and at least one blocker;
- accounting invariants preserve null missingness, separate trust funds,
  endogenous net interest, explicit emergency overrides, future-year payback,
  and a public rounding residual line;
- every `gate_status` value is `false`;
- only `reserve_parameter_readiness_gate_published` may be `true`;
- no reserve parameter, solver, rate, savings, waste, fraud, technology,
  department-cut, tax-proposal, public-rate-card, target-cost, or
  balanced-budget claim may be published from this gate.
