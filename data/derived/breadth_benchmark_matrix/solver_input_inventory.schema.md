# Solver input inventory schema

Schema for `solver_input_inventory.v1.draft.json`.

Required invariants:

- `record_id = solver-input-inventory:v1`.
- Every required solver input must be represented exactly once.
- Every row must have `ready: false` and `value: null`.
- Current artifacts may be named only as partial/context coverage, not as solver
  inputs ready for execution.
- Missing evidence must be explicit for every row.
- Next bounded actions may identify capture/model tasks only; they cannot open
  rates, savings, target costs, or solver outputs.
- Only `solver_input_inventory_published` may be `true`; all rate, savings,
  target-cost, waste, fraud, department-cut, technology-savings, and
  balanced-budget booleans must remain `false`.
