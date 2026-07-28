# Treasury MTS Table 8 Federal Fund FY2025 Context

Machine record:
`data/derived/breadth_benchmark_matrix/treasury_mts_table_8_federal_fund_fy2025_context.v1.draft.json`

Raw source:
`data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-8-FY2025/2026-07-24/mts_table_8_fy2025_final.csv`

This packet captures Treasury Fiscal Data Monthly Treasury Statement Table 8 for
the final FY2025 record date, `2025-09-30`.

Key FY2025 Table 8 values:

- Total Federal Fund Receipts and Outlays receipts:
  `3411392764386.33` dollars.
- Total Federal Fund Receipts and Outlays outlays:
  `5282735020404.42` dollars.
- Net Budget Receipts & Outlays receipts: `5234616386315.43` dollars.
- Net Budget Receipts & Outlays outlays: `7009973667049.30` dollars.

Compared with OMB Historical Table 1.4, the Treasury MTS federal-fund total is
lower by `2104.23561367` million dollars for receipts and lower by
`1766.97959558` million dollars for outlays. That source/timing/perimeter
difference is why this diagnostic does not unlock a general-fund path.

Federal funds are broader than the general fund. Treasury MTS Table 8 federal
fund context is not a general-fund annual path, not a forward projection, not an
explicit transfer schedule, not solver input, not a solver run, not a rate
calculation, not a public rate card, not a savings estimate, and not a balanced-budget claim.
