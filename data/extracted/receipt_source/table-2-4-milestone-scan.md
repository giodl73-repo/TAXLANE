# OMB Table 2.4 Milestone Scan

## Source

- Source ID: `SRC-OMB-HIST-2-4-FY2027`
- Raw artifact:
  `data/raw/omb/SRC-OMB-HIST-2-4-FY2027/2026-06-21/hist02z4_fy2027.xlsx`
- Prior profile: `table-2-4-profile.md`

## Candidate Years

| Year | Why it matters |
|---:|---|
| 1940 | First covered year; already source-reviewed. |
| 1957 | Disability insurance appears and transportation trust-fund excise receipts are non-missing. |
| 1966 | Hospital insurance appears in social-insurance receipts. |
| 1983 | Airport and airway, black lung, hazardous substance, and other trust-fund excise rows are non-missing. |
| 1991 | Transportation fuels row appears under federal funds and multiple excise trust-fund rows are active. |
| 2025 | Recent actual year for later comparison after historical milestones are reviewed. |

## Observed Row Signals

| Row | Label | 1940 | 1957 | 1966 | 1983 | 1991 | 2025 |
|---:|---|---:|---:|---:|---:|---:|---:|
| 8 | OASI trust/off-budget | 550 | 6,457 | 17,556 | 128,972 | 265,502 | 1,097,382 |
| 9 | Disability insurance/off-budget | Missing | 332 | 1,530 | 18,348 | 28,382 | 186,354 |
| 10 | Hospital insurance | Missing | Missing | 893 | 35,641 | 72,842 | 395,350 |
| 24 | Total social-insurance and retirement receipts | 1,785 | 9,997 | 25,546 | 208,994 | 396,015 | 1,748,294 |
| 40 | Transportation trust-fund excise | Missing | 1,479 | 3,917 | 8,297 | 16,979 | 43,768 |
| 41 | Airport and airway trust-fund excise | Missing | Missing | Missing | 2,165 | 4,910 | 23,118 |
| 44 | Hazardous substance superfund excise | Missing | Missing | Missing | 230 | 810 | 1,556 |
| 53 | Total excise trust funds | Missing | 1,479 | 3,917 | 11,214 | 24,127 | 73,372 |
| 54 | Total excise taxes | 1,977 | 10,534 | 13,062 | 35,300 | 42,402 | 105,937 |

Amounts are in millions of dollars. `Missing` means the source marker is
`..........`, not zero.

## Extraction Plan

1. Extract social-insurance milestone rows for 1957 and 1966.
2. Extract excise trust-fund milestone rows for 1957 and 1983.
3. Source-review milestone rows and reconcile parent totals to Table 2.1.
4. Add a historical funding evolution note after review.

## Guardrails

- Keep off-budget as a source label, not an allocation claim.
- Keep federal-funds rows separate from general-fund claims.
- Use trust-fund labels only with the AP13 caveat that Federal trust funds are
  budget-accounting terms.
