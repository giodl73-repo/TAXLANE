# Medicare Source Boundary

## Decision Supported

The Medicare alignment work should not treat every OMB Medicare row as the same
concept. OMB Table 3.2 supports the public-purpose subfunction view; OMB Table
8.5 supports mandatory-program context. Neither table, by itself, provides the
Part A/B/D split needed for contribution-benefit alignment.

## FY2025 Boundary Check

| Source | Medicare measure | FY2025 value |
|---|---|---:|
| `SRC-OMB-HIST-3-2-FY2027` | Medicare subfunction outlays | $996.718B |
| `SRC-OMB-HIST-8-5-FY2027` | Medicare mandatory-program outlays | $987.656B |
| Difference | Table 3.2 minus Table 8.5 | $9.062B |

The difference is 0.9092% of the Table 3.2 Medicare subfunction value.

## Interpretation

This is a source-scope boundary, not a detected arithmetic discrepancy. For
TAXLANE's public-purpose outlay allocation, keep Table 3.2 as the Medicare
spending row. For mandatory-program context, Table 8.5 can cross-check the main
Medicare program magnitude.

Table 8.5 in the local FY2027 OMB workbook exposes a single "Medicare" row. It
does not expose Hospital Insurance, Supplementary Medical Insurance, Part B, or
Part D rows. Therefore it cannot complete the Medicare contribution split.

## Required Next Source

Finish the Medicare alignment split with Trustees or CMS sources that expose:

- HI / Part A income, outgo, trust-fund status, and covered-worker basis.
- SMI / Part B and Part D premiums, general revenue support, and outgo.
- Part A/B/D enrollment or beneficiary denominators.

Until those are extracted, TAXLANE can say Medicare is mixed, but it should not
publish a Part A/B/D per-worker or per-enrollee receipt.
