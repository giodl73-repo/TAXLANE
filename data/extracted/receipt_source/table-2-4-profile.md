# OMB Table 2.4 Profile

## Source

- Source ID: `SRC-OMB-HIST-2-4-FY2027`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-2-4-FY2027/2026-06-21/hist02z4_fy2027.xlsx`
- SHA-256:
  `21D071576D5627A18C3F62DE86BFC7FAECED1A68265F2DB87B4F737B2773C5BD`
- Table title:
  `Table 2.4 - COMPOSITION OF SOCIAL INSURANCE AND RETIREMENT RECEIPTS AND OF EXCISE TAXES: 1940 - 2031`

## Extraction Note

`openpyxl` reports the workbook as one populated row because the workbook lacks
normal dimension metadata. The worksheet XML contains the table rows and should
be parsed directly or with a reader that ignores the workbook dimension.

## Layout

| XML row | Meaning |
|---:|---|
| 1 | Table title. |
| 2 | Units: millions of dollars. |
| 3 | Fiscal-year header, beginning with 1940 in column B. |
| 4-24 | Social insurance and retirement receipts. |
| 25-54 | Excise taxes. |
| 55 | `*` marker definition: $500 thousand or less. |
| 56 | Footnote 1: on-budget and off-budget. |
| 57 | Note: unless otherwise noted, receipts shown are trust funds and on-budget. |

## Social Insurance Row Groups

| Row | Source label | 1940 amount |
|---:|---|---:|
| 7 | Old-age and survivors insurance, Federal funds | 54 |
| 8 | Old-age and survivors insurance, Trust funds (Off-Budget) | 550 |
| 12 | Railroad retirement/pension fund, Federal funds | 1 |
| 13 | Railroad retirement/pension fund, Trust funds | 120 |
| 15 | Employment and general retirement total | 725 |
| 17 | Unemployment insurance, Federal funds | 111 |
| 18 | Unemployment insurance, Trust funds | 904 |
| 19 | Unemployment insurance total | 1,015 |
| 21 | Federal employees retirement - employee share | 44 |
| 22 | Non-Federal employees retirement | 1 |
| 23 | Other retirement total | 45 |
| 24 | Total, Social Insurance and Retirement Receipts (1) | 1,785 |

## Excise Row Groups

| Row | Source label | 1940 amount |
|---:|---|---:|
| 27 | Federal funds, Alcohol | 623 |
| 28 | Federal funds, Tobacco | 606 |
| 37 | Federal funds, Other | 748 |
| 38 | Federal funds total | 1,977 |
| 53 | Trust funds total | Missing marker |
| 54 | Total, Excise Taxes | 1,977 |

## First-Year Reconciliation Targets

For fiscal year 1940:

| Parent category | Table 2.4 total | Table 2.1 parent | Difference |
|---|---:|---:|---:|
| Social insurance and retirement receipts | 1,785 | 1,785 | 0 |
| Excise taxes | 1,977 | 1,977 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Use fiscal year as the year basis.
- Preserve OMB row labels exactly in `source_receipt_label`.
- Preserve fund-group labels where the table provides them.
- Treat `..........` as a missing/not-applicable marker, not zero.
- Treat `*` and `-*` as source markers for $500 thousand or less, not ordinary
  numeric values.
- Use Table 2.4 only after parent Table 2.1 categories exist.
