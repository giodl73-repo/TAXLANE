# Reserve rule contract schema

Schema for `reserve_rule_contract.v1.draft.json`.

Required invariants:

- `record_id = reserve-rule-contract:v1`.
- Contract paths must point to the balance guardrail, rate operating model,
  program-lane target-cost contract, and solver input inventory.
- Required reserve fields must exist and have `initial_value: null`.
- Reserve rules must require over-the-cycle reserve accounting, explicit
  emergency overrides, future-year payback, and public rounding residuals.
- All parameter placeholders remain `null`.
- `solver_ready` remains `false`.
- Only `reserve_rule_contract_published` may be `true`; all parameter, rate,
  savings, solver, waste, fraud, department-cut, technology-savings, and
  balanced-budget booleans must remain `false`.
