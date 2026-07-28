# Transportation Trust Fund Cross-Source Reconciliation Status

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_trust_fund_cross_source_reconciliation_status.v1.draft.json`

This packet rolls up the transportation trust-fund reconciliation state across
the OMB Table 13-4 FY2025-FY2031 context, the Table 13-4 identity diagnostic,
Treasury FY2025 MTS anchor context, and CBO FY2032-FY2035 balance-only context.

Current usable findings:

- OMB Table 13-4 internally reconciles for 14 fund-year rows across
  FY2025-FY2031 within 0.1 billion dollars of rounding tolerance.
- Highway Trust Fund balance turns negative in FY2028 in OMB Table 13-4.
- Airport and Airway Trust Fund balance remains positive through FY2031 in OMB
  Table 13-4.
- CBO provides FY2032-FY2035 balance-only context for the Highway Trust Fund
  and Airport and Airway Trust Fund.
- Treasury MTS FY2025 anchor context is present, and federal funds are not
  general fund.

The blocking issue is the FY2031 OMB/CBO overlap. OMB Table 13-4 has Airport
and Airway at 43.0 billion dollars and Highway at -122.4 billion dollars in
FY2031. CBO has Airport and Airway at 34.699 billion dollars and Highway at 0.0
billion dollars for the same fiscal year. That mismatch blocks stitching CBO
FY2032-FY2035 balances onto OMB Table 13-4 as a continuous trust-fund path.

This packet is not an OMB/CBO stitched path, not FY2032-FY2035 income/outgo
rows, not a complete FY2025-FY2035 transportation trust-fund path, not explicit
general-fund transfers, not credited offsetting collections, not Function 400
mapping, not fund-balance reconciliation, not solver input, not a rate
calculation, not a public rate card, not a savings estimate, not a
technology-savings claim, and not a balanced-budget claim.

Compact validator phrase: not a balanced-budget claim.
