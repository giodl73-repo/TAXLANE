# Income Tax General Fund Note

## Purpose

This note records what the reviewed AP13 concept records support about ordinary
income-tax receipts and the general fund. It is descriptive budget accounting,
not a reform proposal and not taxpayer-level allocation.

## Source Records

- `concept:omb-ap13:general-fund`
- `concept:omb-ap13:federal-funds-group`
- `concept:omb-ap13:special-funds`
- `concept:omb-ap13:trust-funds`
- Source file:
  `data/extracted/budget_concept/SRC-OMB-AP-13-FUNDS-FY2027/budget_concept.SRC-OMB-AP-13-FUNDS-FY2027.2026-06-21.source-reviewed.jsonl`

## Supported Statements

TAXLANE can now make these source-backed statements:

- OMB treats the general fund as the destination for collections not dedicated
  to another fund.
- OMB identifies virtually all income taxes as general-fund collections, with
  the caveat that statute-specific exceptions still require their own source.
- OMB's Federal funds group is broader than the general fund because it also
  includes special and revolving funds.
- Trust-fund and special-fund treatment depends on legal designation or
  dedication; it should not be inferred from an ordinary income-tax amount.

## Blocked Statements

TAXLANE still cannot say:

- a taxpayer's ordinary income-tax payment went to a specific program,
- the Table 1.4 Federal Funds column is identical to the General Fund,
- all income-tax receipts have no possible statutory exception,
- or a proportional spending receipt is a legal dedication.

## Data Implications

- `receipt_source` rows for individual income taxes may be reviewed toward
  `allocation_status = "general_receipt"` when paired with the reviewed AP13
  general-fund concept.
- That label should mean "general receipt absent a cited dedication," not
  "available for any program without appropriation."
- Public taxpayer receipts still need an allocation method label: legal
  dedication, proportional allocation, deficit-inclusive allocation,
  illustrative, or proposed reform lane.

## Public Wording

Use:

> Ordinary individual income-tax receipts are generally treated as general-fund
> revenue unless a specific legal dedication is cited.

Avoid:

> Your income tax paid for this program.

unless the claim is explicitly labeled as a model or backed by a specific legal
dedication source.
