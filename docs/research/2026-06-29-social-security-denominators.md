# Social Security Denominator Values

## Decision Supported

TAXLANE can now support rounded Social Security per-worker and per-beneficiary
context on a CY2025 Trustees basis. These values are appropriate for contributory
lane framing, with the wage-base cap and solvency gap still visible.

## CY2025 Trustees Denominators

| Denominator | CY2025 value | Source |
|---|---:|---|
| Workers with earnings covered by Social Security | 185.0M | `SRC-SSA-TRUSTEES-2026`, 2025 in Review |
| OASDI beneficiaries receiving monthly benefits | 70.5M | `SRC-SSA-TRUSTEES-2026`, section III.A |
| OASI beneficiaries receiving monthly benefits | 62.3M | `SRC-SSA-TRUSTEES-2026`, section III.A |
| DI beneficiaries receiving monthly benefits | 8.2M | `SRC-SSA-TRUSTEES-2026`, section III.A |

## Display Rule

Use these as rounded calendar-year Trustees denominators. They are not exact
administrative person-level counts, tax return counts, household counts, or
Medicare covered-worker counts.

## Remaining Blocker

Social Security can now show rounded per-worker and per-beneficiary context, but
policy claims still need the wage-base cap, taxable payroll, benefit formula,
and solvency gap visible. Medicare HI still needs a clean HI covered-worker
denominator before per-worker Medicare HI displays.
