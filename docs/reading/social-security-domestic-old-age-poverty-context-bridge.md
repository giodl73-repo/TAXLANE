# Social Security Domestic Old-Age Poverty Context Bridge

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_domestic_old_age_poverty_context_bridge.v1.draft.json`

This packet reuses existing Census P60-287 raw custody for Social Security
old-age poverty context. It adds domestic 65+ context alongside the OECD
international old-age poverty bridge, but it does not choose a Taxlane floor
threshold.

The Census workbooks used here are:

| Table | Raw file | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| Table A-3 | `tableA3_hist_pov_by_all_and_age.xlsx` | 57,388 | `a72a881ce64b1d32bacaa35a43a291fb75119503a793195c252d560c253b0ed2` |
| Table B-2 | `tableB-2.xlsx` | 43,484 | `8cdb688380c543c1bd3bc47e2124ec6872511eff8c03c8340b1adacdbd1525fe` |
| Table B-7 | `tableB-7.xlsx` | 14,272 | `ceea883550e7453b3002d90afb4caa7b52612cb9fb24846fe1df424468ca46f7` |
| Income-to-Poverty Ratios | `Income-to-Poverty-Ratios.xlsx` | 14,948 | `fb5b9c60b02cef2acc49d1674271839623082fad4ab00395d42d4949de00938f` |

The 2024 domestic context values carried forward are:

| Measure | 65+ value |
| --- | ---: |
| Official poverty, Table A-3 | 6.108 million, 9.9 percent |
| SPM poverty, Table B-2 | 9.223 million, 15.0 percent |
| Social Security SPM element effect, Table B-7 | -20.100 million |
| Official income below 125 percent of poverty | 8.703 million, 14.2 percent |
| Official income below 150 percent of poverty | 11.640 million, 18.9 percent |
| Official income below 200 percent of poverty | 17.290 million, 28.1 percent |

This is Census domestic 65-plus old-age poverty context only. It is not old-age
poverty measure selection, not income-unit boundary review, not threshold
rationale, not old-age poverty floor values, not policy values, not stress
values, not pass/fail findings, not solver input, not rate calculation, not a
public rate card, not gross savings, not net savings, and not a balanced-budget
claim.

Compact validator phrase: Census domestic 65-plus old-age poverty context only.
Compact validator phrase: not a balanced-budget claim.
