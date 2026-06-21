# Fund Group Concept Guardrail

## Purpose

This note records what OMB Historical Table 1.4 can and cannot prove for
TAXLANE. It supports fund-group extraction and blocks premature legal-dedication
claims.

## Sources

- `SRC-OMB-HIST-1-4-FY2027`
- `SRC-OMB-AP-13-FUNDS-FY2027`
- `SRC-OMB-AP-6-CONCEPTS-FY2027`
- `docs/research/2026-06-21-budget-accounting-explainer.md`
- `data/extracted/fund_group/table-1-4-profile.md`

## What Table 1.4 Can Support

Table 1.4 can support statements like:

- OMB reports receipts, outlays, and surplus/deficit by fund group.
- The table separates Total, Federal Funds, Trust Funds, and Interfund
  Transactions for receipts and outlays.
- The table separates Total, Federal Funds, and Trust Funds for
  surplus/deficit.
- Total receipts, total outlays, and total surplus/deficit can reconcile to OMB
  Table 1.1.
- Interfund transactions must remain visible because they affect fund-group
  totals and are not ordinary public receipts or program outlays.

## What Table 1.4 Cannot Prove By Itself

Table 1.4 cannot by itself support statements like:

- individual income taxes are legally dedicated to a specific fund,
- a taxpayer's income-tax payment entered a specific program,
- trust-fund balances create private ownership rights,
- federal-fund spending is automatically general-fund spending,
- a receipt source caused an equal amount of spending,
- or interfund transactions are ordinary tax receipts.

## Required Guardrails

### Federal funds are broader than the general fund

OMB Table 1.4 reports `Federal Funds`, not a separate `General Fund` column.
TAXLANE may use `federal-funds` as a source-backed fund group from Table 1.4,
but it should not map that column directly to `general-fund` without a concept
source or account-level evidence.

### Trust funds are budget accounts, not private trusts

The budget accounting explainer records OMB's warning that federal trust funds
do not work like private trusts. TAXLANE should preserve `trust-funds` as a
fund-group label, but public text must avoid implying private ownership,
unchangeable benefits, or automatic spending.

### Legal dedication needs a concept or statute source

A `fund_group` row may cite `SRC-OMB-AP-13-FUNDS-FY2027` as the budget-concept
source for interpreting fund groups. It should still keep
`legal_dedication = "unknown"` until a specific source supports dedication for
the row or linked receipt source.

### Interfund transactions are not a receipt lane

Interfund transactions should remain visible as `interfund-transactions`. They
are accounting adjustments between fund groups and should not be presented as
ordinary tax receipts, program spending, or taxpayer lanes.

## Promotion Rule

For fund-group rows:

- `source-reviewed` can be granted after checksum, source label, row anchor, and
  source-cell values are checked.
- `budget-reviewed` can be granted after year basis, measure semantics,
  reconciliation, interfund handling, and legal-dedication guardrails are
  checked.
- `reviewed` still does not mean legal dedication unless `legal_dedication` is
  no longer `unknown` and a specific legal or budget-concept source supports the
  label.

## Public Wording

Use wording like:

- "OMB reports this amount in the Federal Funds group."
- "OMB reports this amount in the Trust Funds group."
- "This table gives fund-group context; it does not by itself trace individual
  income-tax dollars to a program."
- "Interfund transactions are shown separately because they are accounting
  transactions between fund groups."

Avoid wording like:

- "Your income tax went to this fund."
- "Trust fund means private trust ownership."
- "Federal Funds equals General Fund in every case."
- "Interfund transactions are taxpayer receipts."
