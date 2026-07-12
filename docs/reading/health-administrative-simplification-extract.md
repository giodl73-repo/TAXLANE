# Health Administrative Simplification Extract

## Purpose

This packet records the first health/Medicare administrative-simplification
probe.

Machine rows:
`data/derived/efficiency_pressure/extracts/health_admin_simplification_first_pass.jsonl`.

## Extracted Probe

The first pass locks 6 rows: JAMA administrative-cost literature context, OECD
health-cost pressure context, Medicare HI/Part A, Part B, and Part D trust-fund
context rows where administrative costs are not yet isolated, and an explicit
blocker for missing CMS/HHS workflow-volume extraction.

## Boundary

These rows identify where administrative simplification needs deeper source
work. They are not administrative-cost findings, workflow findings, waste
findings, or savings estimates. The next extract must attach workflow volumes,
cycle times, denial/appeal/rework/authorization counts, administrative cost, and
access/payment-integrity floors.
