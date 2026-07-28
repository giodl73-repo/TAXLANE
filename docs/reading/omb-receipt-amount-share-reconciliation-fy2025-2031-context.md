# OMB Receipt Amount/Share Reconciliation FY2025-FY2031 Context

Machine record:
`data/derived/breadth_benchmark_matrix/omb_receipt_amount_share_reconciliation_fy2025_2031_context.v1.draft.json`.

This packet cross-checks OMB Historical Table 2.1 FY2025-FY2031 receipt amounts
against OMB Historical Table 2.2 FY2025-FY2031 receipt-source shares. FY2025 is actual.
FY2026-FY2031 are OMB estimates.

The check recomputes each Table 2.1 category amount as a percent of total
receipts and compares it with the one-decimal percentage published in Table 2.2.
Every category-year difference stays within the half-tenth share rounding tolerance.
The largest absolute rounding difference is $3,518 million for
corporation income taxes in FY2031.

## Boundary

This is amount/share consistency context under one-decimal share rounding. It is
not a legal/economic receipt base, not an assigned base, not incidence or
distribution modeling, not a rate bridge, not solver input, not a tax proposal,
and not a balanced-budget claim.

The packet reduces revenue-solvency fiscal receipt reconciliation context gaps,
but matched receipt bases, behavior, incidence, distribution, administration
burden, current-law solver yields, reform yields, rates, public rate cards, and
solver rows remain blocked.
