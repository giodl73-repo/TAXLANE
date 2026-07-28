# CORE-H Shared Accounting Substrate

Machine record:
`data/derived/breadth_benchmark_matrix/core_h_shared_accounting_substrate.v1.draft.json`.

CORE-H is complete. The shared Rust core now implements checked-integer annual
accounting for named funds, reserves, federal deficits and debt, and endogenous
interest. Transfers, credited offsets, other income, balance adjustments,
financing/timing, and rounding remain visible fields.

TRN-A directly shaped the interfaces: fund ledgers stay separate, source
bridges stay outside arithmetic, null years are not coerced, reserve balances
do not merge into trust funds, reserve overdraw is rejected, and interest
rounding exposes an exact remainder.

This allows TRN-B to start against completed TRN-A and CORE-H dependencies. It
does not select the transportation source bridge, choose reserve parameters,
populate missing years, run a solver, publish a rate, estimate savings, or
claim a balanced budget.
