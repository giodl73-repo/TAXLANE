# Receipt Source Allocation Review

## Sources

- Receipt rows: `SRC-OMB-HIST-2-1-FY2027`
- Fiscal reconciliation: `SRC-OMB-HIST-1-1-FY2027`
- Budget concepts: `SRC-OMB-AP-13-FUNDS-FY2027`
- General-fund note:
  `docs/research/2026-06-21-income-tax-general-fund-note.md`

## Decisions

| Receipt category | `allocation_status` | Reason |
|---|---|---|
| `individual-income-taxes` | `general_receipt` | Reviewed AP13 concept records support ordinary income-tax receipts as general-fund receipts absent a cited legal dedication. |
| `total-receipts` | `mixed` | Total receipts combine categories with different allocation treatments. |
| `corporation-income-taxes` | `unknown` | Source row reviewed, but this pulse focuses on individual income-tax public wording. |
| `social-insurance-and-retirement-receipts` | `unknown` | Must remain separate from individual income taxes; Table 2.4 review is needed before subcomponent allocation. |
| `excise-taxes` | `unknown` | Excise taxes can include general and dedicated treatment; Table 2.4 review is needed. |
| `other-receipts` | `unknown` | Category requires source-specific review. |

## Limits

- `general_receipt` is not a program allocation.
- The label does not mean no appropriation process applies to spending.
- The label does not turn income-tax receipts into a legal destination for any
  public-purpose lane.
- Future taxpayer receipt models still need allocation method and deficit
  context.
