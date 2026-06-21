# Receipt Category Allocation Blockers

## Purpose

This note explains why the first receipt-source review changed individual
income-tax rows but left several other receipt categories at `unknown` or
`mixed`. It is a descriptive data note, not a reform proposal.

## Current Reviewed Labels

| Receipt category | Current label | Why |
|---|---|---|
| `individual-income-taxes` | `general_receipt` | AP13 supports ordinary income-tax receipts as general-fund receipts absent a cited dedication. |
| `total-receipts` | `mixed` | OMB total receipts combine categories with different legal and budget treatment. |
| `corporation-income-taxes` | `unknown` | Candidate for later general-receipt review, but not changed in the individual-income-tax pulse. |
| `social-insurance-and-retirement-receipts` | `unknown` | Needs Table 2.4 and trust-fund concept review before subcomponents can be labeled. |
| `excise-taxes` | `unknown` | Excise taxes can include both general and dedicated treatment; Table 2.4 is needed. |
| `other-receipts` | `unknown` | The category is heterogeneous and needs source-specific review. |

## Next Source Needed

| Category | Next source or action |
|---|---|
| `corporation-income-taxes` | Decide whether AP13's broad income-tax general-fund concept should be applied to corporation income-tax rows in a separate pulse. |
| `social-insurance-and-retirement-receipts` | Extract Table 2.4 subcomponents and link them to trust-fund or other concept records where supported. |
| `excise-taxes` | Extract Table 2.4 subcomponents before distinguishing general excise receipts from dedicated excise streams. |
| `other-receipts` | Use OMB receipt concept sources and, where needed, source-specific accounts. |
| `total-receipts` | Keep `mixed`; do not use total receipts as a legal allocation category. |

## Public Constraint

A public receipt may use individual income-tax rows as general receipts only
with a method label and caveat. It should not allocate total receipts, payroll
receipts, excise receipts, or other receipts to public-purpose lanes until the
category has a reviewed allocation status and deficit context.
