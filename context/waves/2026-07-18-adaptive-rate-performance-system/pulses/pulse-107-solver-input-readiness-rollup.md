# Pulse 107 — Solver input readiness rollup

Pulse 107 adds
`data/derived/breadth_benchmark_matrix/solver_input_readiness_rollup.v1.draft.json`.

The rollup connects the solver-input inventory to the reserve, net-interest,
assigned receipt-base, and distribution placeholder artifacts produced in
Pulses 102–106.

It deliberately does not mark any input ready. Every row remains `ready: false`
with `value: null`, and all solver, target-cost, rate, public-card,
tax-proposal, savings, waste, fraud, technology, department-cut, and
balanced-budget claims remain blocked.
