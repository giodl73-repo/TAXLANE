# OMB Receipt Share Table 2.2 FY2025-FY2031 Context

Machine record:
`data/derived/breadth_benchmark_matrix/omb_receipt_share_table_2_2_fy2025_2031_context.v1.draft.json`.

This packet extracts official OMB Historical Table 2.2 FY2025-FY2031
receipt-source share context from the locally custodied FY2027 workbook.
FY2025 is actual. FY2026-FY2031 are OMB estimates. The source unit is percent of total receipts.

The packet covers individual income taxes, corporation income taxes, social
insurance and retirement receipts, excise taxes, other receipts, and total
receipts. Component shares can sum to 99.9 or 100.1 because the source table is
rounded to one decimal place; the validation tolerance is 0.25 percentage
points.

## Boundary

OMB Historical Table 2.2 receipt shares are receipt-composition context only.
They are not receipt amounts, not legal/economic receipt bases, not assigned bases,
not incidence or distribution models, not rate bridges, not solver inputs, not
tax proposals, and not balanced-budget claims.

The values reduce revenue-solvency context gaps, but matched receipt bases,
behavior, incidence, distribution, administration burden, current-law solver
yields, reform yields, rates, public rate cards, and solver rows remain blocked.
