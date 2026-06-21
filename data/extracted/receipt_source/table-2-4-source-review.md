# OMB Table 2.4 1940 Source Review

## Source Custody

- Source ID: `SRC-OMB-HIST-2-4-FY2027`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-2-4-FY2027/2026-06-21/hist02z4_fy2027.xlsx`
- SHA-256:
  `21D071576D5627A18C3F62DE86BFC7FAECED1A68265F2DB87B4F737B2773C5BD`
- Parent source: `SRC-OMB-HIST-2-1-FY2027`

## Review Checks

| Check | Result |
|---|---|
| Workbook checksum matches metadata | Passed |
| Worksheet XML contains table rows despite workbook dimension issue | Passed |
| Fiscal-year header maps 1940 to column B | Passed |
| Social-insurance total row `A24:B24` equals Table 2.1 row 11 column D | Passed |
| Excise total row `A54:B54` equals Table 2.1 row 11 column G | Passed |
| Missing markers excluded from numeric extraction | Passed |
| Trust-fund caveat preserved | Passed |

## Reconciled Totals

| Fiscal year | Parent category | Table 2.4 total | Table 2.1 parent | Difference |
|---:|---|---:|---:|---:|
| 1940 | Social insurance and retirement receipts | 1,785 | 1,785 | 0 |
| 1940 | Excise taxes | 1,977 | 1,977 | 0 |

## Limits

- These records are source-reviewed, not public taxpayer allocations.
- Federal-funds subcomponent rows are not relabeled as general-fund rows.
- Trust-fund rows preserve budget-accounting treatment and do not imply private
  trust ownership.
- Account-specific statutory rules remain future work.
