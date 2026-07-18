# Pulse 99 — FY2025 fund-group reconciliation fixture

Pulse 99 adds
`data/derived/breadth_benchmark_matrix/fund_group_fy2025_reconciliation_fixture.v1.draft.json`.

It captures aggregate FY2025 federal-fund and trust-fund accounting context from
the already-local OMB Appendix Chapter 13 funds PDF. The fixture records values
in tenths of billions as published, including the explicit -$0.1B public
rounding residual required to reconcile outlay components to the $7,011.1B
published unified outlay total.

This is aggregate context only. It does not create transportation-specific
trust-fund values, does not complete transportation reconciliation, does not run
the simulator, and does not open target-cost, rate, savings, waste/fraud,
department-cut, technology-savings, solver, or balanced-budget claims.
