# Historical Funding 1940 Note

## Purpose

This note records what TAXLANE learns from the first source-reviewed OMB Table
2.4 subcomponent slice. It is descriptive historical budget accounting, not a
taxpayer receipt and not a reform proposal.

## Sources

- `SRC-OMB-HIST-2-1-FY2027`
- `SRC-OMB-HIST-2-4-FY2027`
- `SRC-OMB-AP-13-FUNDS-FY2027`
- Source-reviewed rows:
  `data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-4-FY2027.2026-06-21.source-reviewed.jsonl`

## What 1940 Shows

For fiscal year 1940, OMB Table 2.4 separates parent receipt categories into
funding streams:

- Social-insurance and retirement receipts total 1,785 million dollars.
- Of that total, old-age and survivors insurance includes 550 million dollars
  reported as trust funds and off-budget.
- Unemployment insurance includes both federal-funds and trust-funds rows.
- Other retirement rows are treated as trust funds and on-budget under the
  table note unless otherwise noted.
- Excise taxes total 1,977 million dollars in 1940.
- The 1940 excise detail is reported under federal funds; trust-fund excise rows
  are missing/not applicable in that year.

## What This Teaches

Historical funding is not one receipt pipe:

- A parent receipt category can contain multiple fund treatments.
- Trust-fund treatment can exist inside social-insurance receipts while ordinary
  individual income tax remains a separate general-receipt category.
- Off-budget treatment is a source label that needs to be preserved, not
  collapsed into ordinary receipts.
- Federal-funds excise receipts should not be relabeled as general-fund receipts
  without narrower support.
- Total receipt rows are useful for reconciliation but too mixed for legal
  allocation claims.

## What Remains Blocked

TAXLANE still cannot say:

- a taxpayer's income-tax payment funded Social Security, unemployment, or an
  excise-funded program,
- all social-insurance receipts have the same fund treatment,
- all excise receipts are general revenue,
- or historical receipt categories can be turned into program lanes without
  outlay and deficit context.

## Next Safe Work

The next safe data step is to expand Table 2.4 extraction across a small set of
years where the composition changes materially, such as early Social Security
coverage years, the start of hospital insurance, and years when transportation
or airport excise trust-fund rows become non-missing.
