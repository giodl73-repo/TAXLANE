# INCOME-TAX Plan Role Review

## Scope

This review applies the `.roles` panel to the current repo plan:

- `README.md`
- `PRODUCT_PLAN.md`
- `docs/research/2026-06-21-tax-design-source-map.md`
- `context/waves/2026-06-21-tax-civics-foundation/WAVE.md`

## Decision

The current plan is safe to continue as a foundation research/design plan. It is
not yet approved for public taxpayer receipts, program-linked tax lane proposals,
or final repo naming.

## Summary

| Role | Result | Notes |
|---|---|---|
| Taxpayer Advocate | Pass with P2 follow-up | The plan is readable and clearly says allocation claims need labels, but future public material needs examples that distinguish tax owed, withholding, refund, credit, and annual liability. |
| Budget Accountant | Pass with P2 follow-up | The source map correctly separates receipts, outlays, fund groups, borrowing, and allocation methods. Public receipt work must quote exact OMB definitions before use. |
| Source Custodian | Pass with P2 follow-up | Primary source families are identified. The next data step needs a source-version ledger with table year, URL, download date, and coverage. |
| Public Goods Steward | Pass | The plan treats taxes as funding public obligations and public goods while separating taxes, fees, borrowing, transfers, administration, and debt service. |
| Program Beneficiary Reviewer | Pass with P2 follow-up | Program-linked lanes are deferred appropriately. Future lane models need shortfall, surplus, continuity, and beneficiary-impact rules. |
| Compliance Burden Reviewer | Pass with P2 follow-up | The plan names withholding and compliance, but the current record schema should add taxpayer/employer/admin burden fields before reform modeling. |
| Fiscal Sustainability Reviewer | Pass | Deficits, borrowing, debt service, and long-term obligations are visible enough for the foundation stage. |
| Reform Skeptic | Pass | The plan explicitly treats program-linked taxes as a hypothesis and requires fungibility caveats. |

## Findings

### P2: Budget definitions must be quoted before public receipt claims

- **Roles**: Budget Accountant, Reform Skeptic.
- **Evidence**: The source map identifies OMB Analytical Perspectives chapters
  for budget concepts, governmental receipts, offsetting collections, and trust
  funds, but it has not extracted exact definitions yet.
- **Risk**: A public receipt could use familiar words like "receipt," "outlay,"
  "trust fund," or "paid for" in ways that differ from federal budget accounting.
- **Decision**: Block public taxpayer-receipt publication until the accounting
  explainer quotes and cites exact OMB definitions.

### P2: Source-version ledger is required before data extraction

- **Roles**: Source Custodian, Budget Accountant.
- **Evidence**: The source map lists official IRS and OMB URLs, but the repo does
  not yet record source table versions, download dates, coverage years, or update
  cadence.
- **Risk**: Historical tables update over time; without a ledger, future readers
  cannot reproduce which rates, receipts, or outlays were used.
- **Decision**: Add a source-version ledger before importing or summarizing
  spreadsheet data.

### P2: Program-lane models need shortfall and surplus rules

- **Roles**: Program Beneficiary Reviewer, Fiscal Sustainability Reviewer, Reform
  Skeptic.
- **Evidence**: The source map says program-linked taxes should be auditable and
  disclose fungibility. It does not yet define how a lane behaves if receipts do
  not match program needs.
- **Risk**: A lane can falsely imply service continuity or budget control without
  naming what happens in shortfall, surplus, reserve, borrowing, or appropriation
  override cases.
- **Decision**: Require every future lane to include shortfall, surplus, reserve,
  and legislative override fields.

### P2: Compliance burden must be a first-class design field

- **Roles**: Compliance Burden Reviewer, Taxpayer Advocate.
- **Evidence**: Current docs name withholding, filing, and compliance, but the
  proposed records do not yet include a burden-cost field.
- **Risk**: A clearer taxpayer receipt could be confused with a more complicated
  tax return, and a program-lane design could add complexity without making that
  cost visible.
- **Decision**: Add compliance burden to future reform-record schemas before
  evaluating program-linked taxes.

### P3: Rename candidates should wait until the first data dictionary exists

- **Roles**: Taxpayer Advocate, Source Custodian, Reform Skeptic.
- **Evidence**: `PRODUCT_PLAN.md` already treats INCOME-TAX as a working name.
- **Risk**: Renaming too early may bias the repo toward advocacy, history, or a
  narrow income-tax frame before the evidence spine is clear.
- **Decision**: Keep the working name until the rates/receipts/outlays data
  dictionary and accounting explainer are drafted.

## Adopted plan changes

1. Add a source-version ledger before extracting IRS or OMB data.
2. Draft an OMB budget-accounting explainer before any public receipt.
3. Add shortfall, surplus, reserve, override, and beneficiary-impact fields to
   program-lane model work.
4. Add taxpayer/employer/admin compliance burden fields to reform model work.
5. Keep the repo name provisional until the source model is clearer.

## Non-goals

- This review does not approve any final tax rate, bracket, lane, or program
  funding proposal.
- This review does not provide tax, legal, accounting, or investment advice.
- This review does not decide whether program-linked income taxes are preferable
  to general revenue.
