# Pulse 121 — Current-law FY2025 17-row ledger custody

## Scope

Create the first data-bearing current-law custody packet for the full FY2025 17-row federal ledger.

## Added

- `data/derived/breadth_benchmark_matrix/current_law_fy2025_17_row_ledger_custody.v1.draft.json`
- `data/derived/breadth_benchmark_matrix/current_law_fy2025_17_row_ledger_custody.schema.md`
- `docs/reading/current-law-fy2025-17-row-ledger-custody.md`
- Rust validator and focused regression test

## Boundary

This pulse uses existing local OMB Historical Table custody. No external request was submitted and no agency or person was contacted.

The packet publishes FY2025 baseline-year current-law ledger values only. It does not publish a ten-year path, general-fund path, trust-fund path, health component path, net-interest/debt path, solver input, target cost, rate, public rate card, tax proposal, savings estimate, waste finding, fraud finding, department-cut instruction, technology-savings claim, or balanced-budget claim.

## Reconciliation

- Ledger row count: 17
- Positive rows: 15
- Negative fiscal reconciliation rows: 2
- FY2025 total outlays: `$7,011.105B`
- Rounding residual: `$0`
