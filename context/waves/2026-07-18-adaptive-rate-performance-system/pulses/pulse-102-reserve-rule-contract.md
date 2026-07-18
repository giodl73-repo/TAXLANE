# Pulse 102 — Reserve rule contract

Pulse 102 adds
`data/derived/breadth_benchmark_matrix/reserve_rule_contract.v1.draft.json`.

It defines the reserve fields and guardrails required before any deterministic
solver run may use reserve accounting. No numeric reserve cap, contribution,
withdrawal, emergency override, or payback schedule is selected.

All reserve parameters remain null, and solver/rate/savings/balanced-budget
claims remain blocked.
