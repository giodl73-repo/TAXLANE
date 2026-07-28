# OMB Receipt Detail Table 2.4 FY2025-FY2031 Context

Machine record:
`data/derived/breadth_benchmark_matrix/omb_receipt_detail_table_2_4_fy2025_2031_context.v1.draft.json`.

This packet extracts official OMB Historical Table 2.4 FY2025-FY2031 detail for
social insurance and retirement receipts and excise taxes from the locally
custodied FY2027 workbook. FY2025 is actual. FY2026-FY2031 are OMB estimates.
The workbook source unit is millions of dollars.

The packet preserves source-display omissions as null. It validates the official
subtotal rows for employment/general retirement, total social insurance and
retirement receipts, federal-fund excise receipts, trust-fund excise receipts,
and total excise taxes.

## Boundary

OMB Historical Table 2.4 detail is fiscal receipt context only. OAS and DI rows
are not taxable payroll bases. Hospital Insurance rows are not an HI payroll base
or HI income split. Transportation and Airport/Airway excise rows are not statutory user-fee bases.
The record is not legal/economic receipt bases, not assigned bases, not
incidence or distribution models, not rate bridges, not solver inputs, not tax
proposals, and not balanced-budget claims.

The values reduce Social Security, Health/Medicare, Transportation, and Revenue
Solvency source-context gaps, but matched receipt bases, behavior, incidence,
distribution, administration burden, current-law solver yields, reform yields,
rates, public rate cards, and solver rows remain blocked.
