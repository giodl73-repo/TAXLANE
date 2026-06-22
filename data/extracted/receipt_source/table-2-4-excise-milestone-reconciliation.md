# OMB Table 2.4 Excise Milestone Reconciliation

## Source

- Subcomponent source: `SRC-OMB-HIST-2-4-FY2027`
- Parent receipt source: `SRC-OMB-HIST-2-1-FY2027`
- Concept source: `SRC-OMB-AP-13-FUNDS-FY2027`

## Slice

The milestone draft extraction covers excise-tax subcomponents for fiscal years
1957 and 1983:

- 1957: transportation trust-fund excise receipts are non-missing.
- 1983: airport and airway, black lung disability, inland waterway, and
  hazardous substance superfund trust-fund rows are non-missing.

All records are `draft-extracted`.

## Reconciliation

| Fiscal year | Parent category | Table 2.4 total | Table 2.1 parent | Difference |
|---:|---|---:|---:|---:|
| 1957 | Excise taxes | 10,534 | 10,534 | 0 |
| 1983 | Excise taxes | 35,300 | 35,300 | 0 |

Amounts are in millions of dollars.

## Extraction Decisions

- Federal-funds excise rows remain `unknown` for allocation status.
- Trust-fund excise rows use `dedicated_receipt` with AP13 trust-fund caveats.
- Excise totals use `mixed` because they combine federal-funds and trust-funds
  treatment.
- Missing markers are not extracted as zero-value rows.
