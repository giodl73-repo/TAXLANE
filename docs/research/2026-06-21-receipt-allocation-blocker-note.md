# Receipt Allocation Blocker Note

## Purpose

This note summarizes how the first OMB fund-group slice affects future taxpayer
receipt allocation work.

## What Changed

TAXLANE now has:

- a Table 1.4 workbook profile,
- an explicit `interfund-transactions` fund-group value,
- 1934 fund-group draft records,
- source review for the first fund-group slice,
- and a concept guardrail for Table 1.4 interpretation.

## What We Can Say Now

For reviewed rows, TAXLANE can say:

- OMB reports receipts, outlays, and surplus/deficit by fund group.
- In 1934, Table 1.4 total receipts, outlays, and surplus/deficit reconcile to
  Table 1.1.
- Federal Funds, Trust Funds, and Interfund Transactions are separate source
  columns in Table 1.4.
- Interfund transactions should stay visible instead of being folded into
  ordinary receipts or outlays.

## What Remains Blocked

TAXLANE still cannot say:

- a specific income-tax receipt is legally dedicated to a program,
- a taxpayer's income-tax dollars entered a trust fund,
- the Table 1.4 Federal Funds column equals the General Fund in every case,
- trust funds create private ownership rights,
- or interfund transactions are taxpayer receipts.

## Effect on Future Taxpayer Receipts

Any future taxpayer-facing receipt must still carry an allocation label:

| Claim type | Current status |
|---|---|
| Legal dedication | Blocked until a specific legal or budget-concept source supports the receipt-source link. |
| Proportional outlay allocation | Possible later, but must be labeled as modeled and include deficit context. |
| Deficit-inclusive allocation | Possible later, after outlay and deficit records are reviewed. |
| Program-linked reform lane | Blocked until shortfall, surplus, reserve, appropriation, and override rules are modeled. |

## Next Safe Work

The next safe data work is one of:

1. expand Table 1.4 fund-group extraction across the same years already used for
   receipt records, or
2. extract OMB Analytical Perspectives fund definitions into concept records so
   `legal_dedication`, `appropriation_required`, and `general-fund` labels can
   be reviewed instead of left `unknown`.
