# Pulse 100 — Solver accounting readiness gate

Pulse 100 adds
`data/derived/breadth_benchmark_matrix/solver_accounting_readiness_gate.v1.draft.json`.

It prevents the Pulse 99 aggregate fund-group fixture from being misused as a
solver input. The fixture can test rounding, deficit sign, and aggregate
trust-fund balance arithmetic only.

The deterministic solver remains blocked until lane/fund annual paths exist for
OASDI, Medicare HI, transportation trust, general fund, reserves, explicit
interfund transfers, credited offsetting collections, assigned receipt bases,
and endogenous net interest.

No solver run, target cost, rate, savings, waste/fraud, department-cut,
technology-savings, or balanced-budget claim is opened.
