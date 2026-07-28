# SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025 Metadata

Publisher: Bureau of the Fiscal Service, U.S. Department of the Treasury

Dataset: Monthly Treasury Statement, Tables 4 and 5

Retrieved: 2026-07-24

Table 4 URL:
`https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v1/accounting/mts/mts_table_4?filter=record_date:eq:2025-09-30&page[size]=10000&format=csv`

Table 5 URL:
`https://api.fiscaldata.treasury.gov/services/api/fiscal_service/v1/accounting/mts/mts_table_5?filter=record_date:eq:2025-09-30&page[size]=10000&format=csv`

Local artifacts:

- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_4_fy2025_final.csv`
- `data/raw/treasury/SRC-TREASURY-FISCALDATA-MTS-TABLE-4-5-FY2025/2026-07-24/mts_table_5_fy2025_final.csv`

Byte counts:

- Table 4: `15442`
- Table 5: `203342`

SHA-256:

- Table 4: `f82fdcae4b28e3a9a66dfeb20726d1a81d900ca5eabc3559741882e9258fb204`
- Table 5: `fb1646d18d9cc05a217b3b6ac084fd006e0bf01fa26c8ee8815b881579cea66a`

Rows:

- Table 4: `57`
- Table 5: `811`

Custody boundary: local raw custody exists for the final FY2025 MTS Table 4 and
Table 5 record date. Current derived packets extract Federal Hospital Insurance
Trust Fund receipt/outlay anchors and transportation trust-fund receipt/outlay
context only. This is not a calendar-to-fiscal conversion, not a FY2026-FY2035
Medicare HI path, not transportation income/outgo reconciliation, not Function
400 mapping, not a solver input, not a rate calculation, not a savings estimate,
and not a balanced-budget claim.
