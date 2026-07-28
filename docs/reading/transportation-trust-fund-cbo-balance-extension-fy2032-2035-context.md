# Transportation Trust Fund CBO Balance Extension Context

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_trust_fund_cbo_balance_extension_fy2032_2035_context.v1.draft.json`.

This packet captures unambiguous CBO February 2026 open-data balance rows for
the Highway Trust Fund and Airport and Airway Trust Fund in FY2032-FY2035. It
uses the already-custodied CBO trust-fund CSV and only reads the balance
variables `tf_bal_airport` and `tf_bal_highway`.

The FY2032-FY2035 CBO balance context shows Airport and Airway Trust Fund
balances of 38.411, 42.533, 47.123, and 52.210 billion dollars. The CBO Highway
Trust Fund balance is 0.0 billion dollars in each of FY2032-FY2035.

The FY2031 overlap with OMB Table 13-4 exposes source-perimeter and balance
treatment differences: OMB Table 13-4 has Airport and Airway at 43.0 and
Highway at -122.4 billion dollars, while CBO has Airport and Airway at 34.699
and Highway at 0.0 billion dollars.

## Boundary

This is CBO FY2032-FY2035 transportation trust-fund balance extension context
only. It is not OMB Table 13-4 FY2032-FY2035 income/outgo rows, not a complete
FY2025-FY2035 transportation trust-fund path, not trust-fund income/outgo
reconciliation, not explicit general-fund transfers, not credited offsetting
collections, not Function 400 mapping, not solver input, not a rate calculation,
not a public rate card, not a savings estimate, and not a balanced-budget claim.

The CBO surplus variables for airport and highway have duplicate date/variable rows in the raw CSV, so this packet excludes those surplus rows from income/outgo or solver use.
