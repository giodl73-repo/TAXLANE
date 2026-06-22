# OMB Table 2.4 Social-Insurance Milestone Reconciliation

## Source

- Subcomponent source: `SRC-OMB-HIST-2-4-FY2027`
- Parent receipt source: `SRC-OMB-HIST-2-1-FY2027`
- Concept source: `SRC-OMB-AP-13-FUNDS-FY2027`

## Slice

The milestone draft extraction covers social-insurance and retirement receipt
subcomponents for fiscal years 1957 and 1966:

- 1957: disability insurance and transportation-era social-insurance splits are
  visible in the selected rows.
- 1966: hospital insurance appears as a non-missing row.

All records are `draft-extracted`.

## Reconciliation

| Fiscal year | Parent category | Table 2.4 total | Table 2.1 parent | Difference |
|---:|---|---:|---:|---:|
| 1957 | Social insurance and retirement receipts | 9,997 | 9,997 | 0 |
| 1966 | Social insurance and retirement receipts | 25,546 | 25,546 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Missing markers are not extracted as zero-value rows.
- Off-budget is preserved as a source label.
- Federal-funds rows remain separate from general-fund claims.
- Trust-fund rows use AP13 concept support but still do not imply private trust
  ownership or direct taxpayer-to-program tracing.
