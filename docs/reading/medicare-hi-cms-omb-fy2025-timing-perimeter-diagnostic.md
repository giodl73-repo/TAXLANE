# Medicare HI CMS OMB FY2025 Timing Perimeter Diagnostic

Machine record:
`data/derived/breadth_benchmark_matrix/medicare_hi_cms_omb_fy2025_timing_perimeter_diagnostic.v1.draft.json`

This packet compares CMS Trustees CY2025 HI context with OMB FY2025 receipt and
outlay anchors. It is a timing/perimeter diagnostic only.

Diagnostic comparisons:

- CMS CY2025 HI total income: `462400` million.
- OMB FY2025 Hospital Insurance receipt anchor: `395350` million.
- CMS income minus OMB receipt anchor: `67050` million.
- CMS CY2025 HI total expenditures: `444200` million.
- OMB FY2025 HI outlay anchor: `444832` million.
- CMS expenditures minus OMB outlay anchor: `-632` million.

The expenditure comparison is close, but it still does not prove a fiscal-year bridge.
The income comparison is materially different and requires an explicit
income-category, timing, and perimeter bridge before any solver or rate use.

Medicare HI CMS/OMB FY2025 timing-perimeter comparison is diagnostic context
only. It is not a calendar-to-fiscal conversion, not a fiscal-year HI path, not
an OMB/CMS receipt-row bridge, not a matched solver yield, not solver input, not
a solver run, not a rate calculation, not a public rate card, not a savings
estimate, and not a balanced-budget claim. This is not a balanced-budget claim.
