# Net Interest PBD FY2025-FY2031 Current-Law Context Path

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_pbd_fy2025_2031_current_law_context_path.v1.draft.json`

Pulse 208 adds local OMB Public Budget Database net-interest context for
FY2025-FY2031. The packet sums 355 workbook rows where `BEA Category` equals
`Net interest`, preserves the workbook net perimeter, and converts thousands of
dollars to millions of dollars.

What is now usable:

- FY2025 actual net-interest context from the local OMB workbook;
- FY2026-FY2031 projected net-interest context from the same source;
- explicit nulls for FY2032-FY2035 because the local workbook does not contain
  those years.

What remains blocked:

- complete FY2025-FY2035 net-interest path;
- debt stock, maturity schedule, and rate path;
- primary-balance feedback fixture;
- direct net-interest cuts;
- solver input, solver run, rates, public rate cards, savings,
  technology-savings claims, and balanced-budget claims.

Net interest is endogenous. This packet is account-level current-law context,
not a policy lever or solver-ready feedback model.
