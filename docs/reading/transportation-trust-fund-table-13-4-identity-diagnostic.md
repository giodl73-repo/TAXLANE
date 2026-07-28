# Transportation Trust Fund Table 13-4 Identity Diagnostic

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_trust_fund_table_13_4_identity_diagnostic.v1.draft.json`

This packet checks the internal arithmetic of the OMB Analytical Perspectives
Table 13-4 transportation fund rows already captured in the context path.
It covers the Highway Trust Fund and Airport and Airway Trust Fund for
FY2025-FY2031.

Diagnostic result:

- 14 fund-year rows checked.
- 0 rows have a delta above the 0.1 billion rounding tolerance.
- Maximum absolute delta is 0.1 billion.
- Highway Trust Fund balance turns negative in FY2028.
- Airport and Airway Trust Fund balance remains positive through FY2031.

This improves transportation trust-fund source reconciliation inside Table
13-4 only. It does not add FY2032-FY2035 values and does not reconcile the
trust-fund rows to Function 400 net outlays, explicit general-fund transfers,
credited offsetting collections, or a solver perimeter.

OMB Table 13-4 transportation trust-fund rows internally reconcile within
rounding tolerance for FY2025-FY2031, but this is diagnostic context only. This
is not FY2032-FY2035 values, not a complete transportation trust-fund path, not
explicit general-fund transfers, not credited offsetting collections, not
Function 400 mapping, not solver input, not a solver run, not a rate
calculation, not a public rate card, not a savings estimate, and not a
balanced-budget claim. This is not a balanced-budget claim.
