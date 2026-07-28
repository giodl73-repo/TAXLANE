# Transportation Trust Fund Treasury MTS FY2025 Anchor Context

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_trust_fund_treasury_mts_fy2025_anchor_context.v1.draft.json`

Raw source files:

- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv`
- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv`

This packet captures Treasury Fiscal Data Monthly Treasury Statement Tables 4
and 5 for final FY2025 record date `2025-09-30`.

Extracted receipt anchors:

- MTS Table 4 line 44, `Airport and Airway Trust Fund`: receipts of
  `22651.06130492` million dollars.
- MTS Table 4 line 45, `Highway Trust Fund`: receipts of `44294.72985973`
  million dollars.

Extracted outlay context:

- MTS Table 5 line 436, `Total--Airport and Airway Trust Fund`: outlays of
  `19679.24588718` million dollars.
- MTS Table 5 line 444, `Total--Federal Highway Administration`: outlays of
  `65032.57729402` million dollars. This is an agency total, not a Highway Trust Fund total.

Two negative MTS Table 5 transportation trust-fund rows are recorded as
observed rows but are not separately netted into a trust-fund income/outgo
bridge.

This is fiscal-year anchor context only. It is not transportation trust-fund income/outgo reconciliation, not fund-balance reconciliation, not an explicit transfer schedule, not credited offsetting collections, not Function 400 mapping, not solver input, not a solver run, not a rate calculation, not a public rate card, not a savings estimate, and not a balanced-budget claim.
