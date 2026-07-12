# Medicare Denominator Values

## Decision Supported

TAXLANE can now support Medicare per-enrollee examples on a source-specific
calendar-year basis, while still blocking generic "per person" Medicare claims.

## CY2025 Trustees Denominators

| Denominator | CY2025 value | Source |
|---|---:|---|
| Medicare HI / Part A enrollment | 69.100M | `SRC-CMS-MEDICARE-TRUSTEES-2026`, Table V.B3 |
| Medicare SMI / Part B enrollment | 63.448M | `SRC-CMS-MEDICARE-TRUSTEES-2026`, Table V.B3 |
| Medicare SMI / Part D enrollment | 56.754M | `SRC-CMS-MEDICARE-TRUSTEES-2026`, Table V.B3 |
| Medicare beneficiaries with HI and/or SMI coverage | 69.289M | `SRC-CMS-MEDICARE-TRUSTEES-2026`, Table V.B3 |
| Medicare private health plan enrollment | 35.359M | `SRC-CMS-MEDICARE-TRUSTEES-2026`, Table V.B3 |

Table IV.B7 also reports the Part D enrollment family rounded to 56.8M for
CY2025.

## Display Rule

Use these values only when the visible claim names the exact denominator:

- per Part A enrollee;
- per Part B enrollee;
- per Part D enrollee;
- per Medicare beneficiary with HI and/or SMI coverage;
- per private health plan enrollee.

Do not collapse those into a generic "per person" figure.

## Remaining Blockers

These are calendar-year Trustees denominators. They can be paired with
calendar-year Trustees financing rows, but not directly with FY2025 OMB outlays
without a calendar-year/fiscal-year bridge.

Covered-worker denominators for Medicare HI and Social Security remain blocked
until a clean source table is extracted.
