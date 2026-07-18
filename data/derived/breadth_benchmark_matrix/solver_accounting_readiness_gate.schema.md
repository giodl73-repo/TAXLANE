# Solver accounting readiness gate schema

Schema for `solver_accounting_readiness_gate.v1.draft.json`.

Required invariants:

- `record_id = solver-accounting-readiness-gate:v1`.
- The gate may use the FY2025 fund-group fixture only for aggregate accounting
  tests: rounding residual, deficit sign, and fund-balance arithmetic.
- The deterministic solver must remain not ready.
- Transportation solver readiness must remain false.
- Required solver inputs with missing lane/fund annual data must have `ready:
  false` and `value: null`.
- Prohibited uses must include solver run, transportation trust-fund values,
  target cost, rates, savings, waste, fraud, department cut, technology savings,
  and balanced-budget claims.
- Output placeholders remain `null`.
- Only `solver_accounting_readiness_gate_published` may be `true`; all other
  public claim booleans must remain `false`.
