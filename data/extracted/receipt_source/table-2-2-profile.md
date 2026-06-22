# OMB Table 2.2 Receipt Share Profile

## Source

- Source ID: `SRC-OMB-HIST-2-2-FY2027`
- Raw artifact: `data/raw/omb/SRC-OMB-HIST-2-2-FY2027/2026-06-21/hist02z2_fy2027.xlsx`
- Table title: `Table 2.2 - PERCENTAGE COMPOSITION OF RECEIPTS BY SOURCE: 1934 - 2031`

## Coverage

- Fiscal years emitted: 1934-2031
- Year count: 98
- Estimate years: 6
- Record count: 588

## Extracted Columns

| Column | Receipt category | Source label |
|---|---|---|
| B | `individual-income-taxes` | Individual Income Taxes |
| C | `corporation-income-taxes` | Corporation Income Taxes |
| D | `social-insurance-and-retirement-receipts` | Social Insurance and Retirement Receipts Total |
| G | `excise-taxes` | Excise Taxes |
| H | `other-receipts` | Other |
| I | `total-receipts` | Total Receipts |

## Sample Shares

Percentages are OMB-reported shares of total receipts.

| Fiscal year | Individual income | Corporation income | Social insurance | Excise | Other | Total receipts |
|---:|---:|---:|---:|---:|---:|---:|
| 1934 | 14.2% | 12.3% | 1.0% | 45.8% | 26.7% | 100.0% |
| 1940 | 13.6% | 18.3% | 27.3% | 30.2% | 10.7% | 100.0% |
| 1980 | 47.2% | 12.5% | 30.5% | 4.7% | 5.1% | 100.0% |
| 2000 | 49.6% | 10.2% | 32.2% | 3.4% | 4.5% | 100.0% |
| 2025 | 50.7% | 8.6% | 33.4% | 2.0% | 5.2% | 100.0% |
| 2031 | 50.3% | 7.2% | 31.7% | 1.5% | 9.4% | 100.0% |

## Extraction Decisions

- Keep Table 2.2 percentage rows separate from Table 2.1 amount rows.
- Skip the transition-quarter `TQ` row because it is not a fiscal year.
- Preserve estimate years as `actual_or_projection = "estimate"`.
- Treat total receipts as `mixed` because it combines categories with different budget treatment.
- Keep non-individual receipt allocation labels as `unknown` pending narrower review.
