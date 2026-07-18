# Health Current-Law Fiscal Path

Machine record:
`data/derived/breadth_benchmark_matrix/health_fiscal_scenario_path.v1.draft.json`.

Schema:
`data/derived/breadth_benchmark_matrix/health_fiscal_scenario_path.schema.md`.

This is a current-law path, not a reform score. It covers FY2025-FY2036 and
keeps Medicare HI, Medicare SMI and other Medicare, and non-Medicare health
general-fund spending separate.

OMB Table 3.2 supplies non-Medicare health values through FY2031. The same table
also supplies combined Medicare context through FY2031, but that combined value
is not used as either a Medicare HI or SMI component. HI must remain a separate
fund.

FY2032-FY2036 component values remain null because no local official annual
health component source covering those years was captured in this branch.
No interpolation is used.

The FY2025 fixture reconciliation is:

| Item | Millions |
|---|---:|
| Medicare total | 996,718 |
| Non-Medicare health | 978,511 |
| Combined health and Medicare | 1,975,229 |
| Medicare payroll financing fixture | 395,350 |
| Medicare excise financing fixture | 3,434 |
| Medicare residual general requirement fixture | 597,934 |

All central-reform and stress federal cash-flow fields remain solver-ineligible.
