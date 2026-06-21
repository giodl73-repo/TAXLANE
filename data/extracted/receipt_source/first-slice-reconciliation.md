# OMB Receipt Source First Slice Reconciliation

## Source

- Receipt source: `SRC-OMB-HIST-2-1-FY2027`
- Fiscal spine: `SRC-OMB-HIST-1-1-FY2027`
- Schema: `docs/data/receipts-funds-schema.md`

## Slice

The first draft receipt extraction covers fiscal years 1934-1936 from OMB
Historical Table 2.1:

- Individual income taxes
- Corporation income taxes
- Social insurance and retirement receipts
- Excise taxes
- Other receipts
- Total receipts

All records are `source-reviewed`.

## Reconciliation

| Fiscal year | Table 2.1 total receipts | Table 1.1 total receipts | Difference |
|---:|---:|---:|---:|
| 1934 | 2,955 | 2,955 | 0 |
| 1935 | 3,609 | 3,609 | 0 |
| 1936 | 3,923 | 3,923 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Social-insurance receipts remain separate from individual income taxes.
- Individual income-tax rows use `allocation_status = "general_receipt"` based
  on reviewed AP13 general-fund concept support.
- Total receipt rows use `allocation_status = "mixed"` because they combine
  categories with different treatment.
- Corporation income-tax, social-insurance, excise, and other receipt rows keep
  `allocation_status = "unknown"` pending narrower review.
- Amount rows have `amount` values and null `percent` values.
- No public taxpayer allocation should use these records without an explicit
  allocation method and deficit context.
