# OMB Table 2.4 1940 Reconciliation

## Source

- Subcomponent source: `SRC-OMB-HIST-2-4-FY2027`
- Parent receipt source: `SRC-OMB-HIST-2-1-FY2027`
- Concept source: `SRC-OMB-AP-13-FUNDS-FY2027`

## Slice

The first Table 2.4 draft extraction covers fiscal year 1940:

- Social insurance and retirement receipts by major subcomponent and fund group.
- Excise taxes by federal-funds subcomponent.
- Table 2.4 parent totals for social-insurance and excise categories.

All records are `source-reviewed`.

## Reconciliation

| Fiscal year | Parent category | Table 2.4 total | Table 2.1 parent | Difference |
|---:|---|---:|---:|---:|
| 1940 | Social insurance and retirement receipts | 1,785 | 1,785 | 0 |
| 1940 | Excise taxes | 1,977 | 1,977 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Trust-fund subcomponent rows use `allocation_status = "dedicated_receipt"`
  only when the Table 2.4 row or table note supports trust-fund treatment and
  AP13 provides the reviewed trust-fund concept.
- Federal-funds subcomponent rows keep `allocation_status = "unknown"` because
  Federal Funds is broader than the General Fund.
- Social-insurance and excise total rows are not taxpayer allocation claims.
- Missing markers are not extracted as zero-value rows in this first slice.
