# Pulse 101 — Solver input inventory

Pulse 101 adds
`data/derived/breadth_benchmark_matrix/solver_input_inventory.v1.draft.json`.

It converts the solver accounting gate into an actionable input inventory. Every
required solver input remains `ready: false` and `value: null`, with the missing
evidence named explicitly.

No solver run, target cost, rate, savings, waste/fraud, department-cut,
technology-savings, or balanced-budget claim is opened.
