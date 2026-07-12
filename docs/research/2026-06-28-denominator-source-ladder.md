# Denominator Source Ladder

## Decision Supported

TAXLANE can show per-unit spending or receipt figures only after the denominator
is explicitly sourced, named, and matched to the display basis. This note records
the source ladder so future "per person" views do not silently mix residents,
tax returns, workers, beneficiaries, enrollees, and households.

## Current Extracted Denominator

| Display basis | Denominator | Source | Value | Public status |
|---|---|---|---:|---|
| Per taxpayer | Individual income tax returns filed, TY2022 | `SRC-IRS-SOI-1304` | 161.3M | Sourced for TY2022 return-basis display only |
| Per taxpayer | Taxable individual income tax returns, TY2022 | `SRC-IRS-SOI-1304` | 110.6M | Sourced for TY2022 taxable-return-basis display only |
| Per enrollee | Medicare Part A enrollment, CY2025 | `SRC-CMS-MEDICARE-TRUSTEES-2026` | 69.100M | Sourced for Part A enrollee display only |
| Per enrollee | Medicare Part B enrollment, CY2025 | `SRC-CMS-MEDICARE-TRUSTEES-2026` | 63.448M | Sourced for Part B enrollee display only |
| Per enrollee | Medicare Part D enrollment, CY2025 | `SRC-CMS-MEDICARE-TRUSTEES-2026` | 56.754M | Sourced for Part D enrollee display only |
| Per beneficiary | Medicare beneficiaries with HI and/or SMI coverage, CY2025 | `SRC-CMS-MEDICARE-TRUSTEES-2026` | 69.289M | Sourced for total Medicare beneficiary context only |
| Per resident | U.S. resident population, July 1 2025 | `SRC-CENSUS-POP-EST-2025` | 341.785M | Sourced for broad civic cost per resident only |
| Per household | U.S. households, 2025 | `SRC-CENSUS-HH1-2025` | 134.790M | Sourced for household display only |
| Per worker | OASDI covered workers, CY2025 | `SRC-SSA-TRUSTEES-2026` | 185.0M | Sourced for rounded Social Security worker context only |
| Per beneficiary | OASDI beneficiaries, CY2025 | `SRC-SSA-TRUSTEES-2026` | 70.5M | Sourced for rounded Social Security beneficiary context only |

These values are useful for rate and receipt framing. They are source-specific:
Medicare, Census, and Social Security values are calendar-year denominators, not
fiscal-year OMB outlay denominators.

## Remaining Source Ladder

| Display basis | Needed source family | Required before public numeric display |
|---|---|---|
| Per resident | Census population estimate | Year, geography, resident-population definition, and whether calendar-year or fiscal-year proxy is used. |
| Per taxpayer | IRS SOI or tax-unit source | Whether denominator is all filed returns, taxable returns, adults, households, or tax units. |
| Per worker | SSA/OACT or program payroll source | Covered-worker definition, program scope, wage-base treatment, and year. |
| Per beneficiary | Program administrative source | Benefit-program definition, beneficiary count, year, and whether beneficiaries can appear in multiple programs. |
| Per enrollee | CMS or program enrollment source | Medicare Part A/B/D or Medicaid/marketplace enrollment definition, year, and duplicated-enrollment treatment. |
| Per household | Census household source | Household definition, geography, and year. |

## Display Rules

- A per-unit figure must cite both an outlay/allocation source and a denominator
  value source.
- The denominator label must be part of the visible claim, for example "per
  filed return" or "per resident"; avoid generic "per person" unless the source
  is actually a resident-population denominator.
- Mixed programs need split denominators. Medicare HI can use worker and Part A
  enrollee/beneficiary views; SMI needs Part B/D enrollee and premium/general
  support views.
- Cross-year displays are allowed only as illustrative and must say which year
  each side uses.

## Next Extraction Targets

1. Medicare HI covered-worker denominators.
2. Non-Medicare health enrollment/beneficiary counts.
3. Veterans and income-security beneficiary counts.
