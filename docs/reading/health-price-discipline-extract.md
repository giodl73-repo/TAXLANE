# Health Price Discipline Extract

## Purpose

This packet records the first health/Medicare price-discipline probe.

Machine rows:
`data/derived/efficiency_pressure/extracts/health_price_discipline_first_pass.jsonl`.

## Extracted Probe

The first pass locks 6 rows: an OECD high-level health-spending benchmark, a
JAMA literature context row, three CY2025 Medicare Trustees per-enrollee anchors
for HI/Part A, Part B, and Part D, and one explicit blocker for missing
service-level CMS/HHS price and utilization extraction.

## Boundary

These rows identify where price discipline could be investigated next. They are
not service-price findings, waste findings, or savings estimates. Before scoring
anything, the next extract must attach service/drug-level price and utilization
data, a benchmark and case-mix method, and quality/access floors.
