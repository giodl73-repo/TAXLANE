# Wave: Contribution-Benefit Alignment

## Goal

Make TAXLANE explicit about whether each lane is aligned to what people pay in,
who benefits, and which public purpose the money supports. This is especially
important for mixed programs such as Medicare, where Hospital Insurance is tied
to payroll contributions but Supplementary Medical Insurance relies heavily on
premiums and general revenue.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Alignment framework | done | Added rules for per-person, per-worker, per-beneficiary, and public-good alignment. |
| 02 | Initial FY2025 alignment records | done | Added draft rows for Social Security, Medicare, health, defense, interest, and transfers. |
| 03 | Medicare part split source ladder | done | Added OMB Table 3.2 vs Table 8.5 boundary check plus CMS Trustees CY2025 HI, Part B, and Part D financing rows. |
| 04 | Per-person denominator requirements | done | Added required denominator records before publishing per-person receipts. |
| 05 | Reader packet | done | Added a public packet explaining contribution alignment, the Medicare split, and denominator rules. |
| 06 | First denominator values | done | Added TY2022 IRS SOI filed-return and taxable-return denominator values plus a source ladder for remaining denominators. |
| 07 | Medicare denominator values | done | Added CY2025 Medicare Part A/B/D, total-beneficiary, and private-plan enrollment denominator values from the CMS Trustees report. |
| 08 | Civic denominator values | done | Added CY2025 Census resident-population and household denominator values for broad civic cost displays. |
| 09 | Social Security denominator values | done | Added rounded CY2025 OASDI covered-worker and beneficiary denominators from the SSA Trustees report. |
| 10 | Per-unit readiness layer | done | Added ready/illustrative/blocked per-unit display rows and dashboard. |
| 11 | Per-unit receipt cards | done | Added structured card rows and reader packet that preserve basis and caveats. |
| 12 | Flagship honest receipt | done | Added a one-page before/after receipt that makes the financing relationship and five required labels public-facing. |

## Status

Active. Pulses 01-12 establish the alignment model, source the first tax-return,
Medicare, civic, and Social Security denominators, publish readiness-gated
per-unit cards, and assemble the flagship honest receipt. Existing cards are
public only with their basis and caveats attached. Next evidence work should
source Medicare HI covered workers and non-Medicare beneficiary/enrollee
denominators before expanding the receipt with additional per-unit amounts.

## Design Rules

- Per-person display requires a named denominator.
- "Proportional to what goes in" is appropriate for contributory lanes, not all
  public goods.
- Medicare must be split into HI and SMI before claiming contribution alignment.
- General-revenue transfers should show redistribution openly, not pretend each
  person receives exactly what they paid.
- Debt interest is a past-borrowing cost, not a personal benefit lane.

## Validation

```powershell
git diff --check
```
