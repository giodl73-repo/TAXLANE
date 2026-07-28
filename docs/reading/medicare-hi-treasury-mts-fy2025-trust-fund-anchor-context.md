# Medicare HI Treasury MTS FY2025 Trust Fund Anchor Context

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_hi_treasury_mts_fy2025_trust_fund_anchor_context.v1.draft.json`

Raw source files:

- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv`
- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv`

This packet captures Treasury Fiscal Data Monthly Treasury Statement Tables 4
and 5 for final FY2025 record date `2025-09-30`.

Extracted FY2025 anchors:

- MTS Table 4 line 27, `Total -- Federal Hospital Insurance Trust Fund`:
  receipts of `395350.35946967` million dollars.
- MTS Table 5 line 221, `Total--Federal Hospital Insurance Trust Fund`:
  outlays of `444832.69985451` million dollars.

Those Treasury MTS anchors align with the existing rounded OMB FY2025 HI
receipt and outlay anchors, differing by `0.35946967` million and `0.69985451`
million respectively. They do not remove the calendar-year versus fiscal-year
boundary in CMS Trustees Table II.E1.

Two negative MTS Table 5 Federal Hospital Insurance Trust Fund rows are recorded
as observed rows but are not separately netted into a solver bridge. The total
HI outlay line remains the FY2025 anchor for this context packet.

This is fiscal-year anchor context only. It is not a calendar-to-fiscal conversion, not a FY2025-FY2035 Medicare HI fiscal-year path, not an income-category crosswalk, not matched solver yield, not solver input, not a solver run, not a rate calculation, not a public rate card, not a savings estimate, and not a balanced-budget claim.
