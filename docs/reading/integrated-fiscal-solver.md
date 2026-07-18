# Integrated fiscal solver

Machine record:
`data/derived/breadth_benchmark_matrix/integrated_fiscal_solver.v1.draft.json`.

This is a deterministic scaffold, not an optimizer.

It reconciles the FY2025 17-row ledger to `$7,011.105B`, including Commerce and
Housing Credit and Undistributed Offsetting Receipts as negative offset rows.

It exposes two different denominators:

- all-receipt funding share: row cost divided by total funded federal cost;
- residual general-fund requirement share: residual general-fund need divided by
  total residual general-fund need.

A value after subtracting dedicated receipts is not a share of every tax dollar.

The debt path keeps net interest endogenous. A primary-balance improvement must
lower debt and lower subsequent net interest through the average-rate feedback
rule. The regression fixture exists to fail if a primary change leaves later debt
and interest unchanged. In short: primary change leaves later debt and interest unchanged is invalid.

Rates, target reform costs, fund balances, reserves, optimization, distribution,
macro feedback, and balanced-budget claims remain blocked.
