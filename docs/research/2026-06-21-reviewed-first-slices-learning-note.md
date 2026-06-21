# Reviewed First Slices Learning Note

## Purpose

This note summarizes what TAXLANE has learned from the first source-reviewed and
budget-reviewed extraction slices. It is not a taxpayer receipt, not a reform
proposal, and not a full-table analysis.

## Reviewed Inputs

| Slice | Source | Reviewed years | What passed |
|---|---|---:|---|
| IRS rates | `SRC-IRS-SOI-HT23` | 1913-1918 | Source checksum, row anchors, summary-rate values, tax-year basis. |
| OMB receipts | `SRC-OMB-HIST-2-1-FY2027` plus Table 1.1 reconciliation | 1934-1936 | Source checksum, row anchors, receipt category amounts, fiscal-year basis, total-receipts reconciliation. |
| OMB outlays | `SRC-OMB-HIST-3-1-FY2027` plus Table 1.1 reconciliation | 1940-1942 | Source checksum, row anchors, outlay category amounts, fiscal-year basis, total-outlays reconciliation. |

## What We Learned

1. The data path is workable. TAXLANE can move from captured raw artifacts to
   source-anchored JSONL records, then to source and budget review.
2. IRS Table 23 is useful as a historical rates spine, but the extracted rows
   are summary lowest/highest bracket rows. They are not complete statutory
   schedules.
3. OMB Table 2.1 cleanly separates individual income taxes, corporation income
   taxes, social-insurance receipts, excise taxes, other receipts, and total
   receipts in the reviewed years.
4. Social-insurance receipts should remain separate from individual income
   taxes in all later public explanations.
5. OMB Table 3.1 makes net interest and undistributed offsetting receipts
   visible in the outlay structure. TAXLANE should not hide either one inside
   broad public-purpose lanes.
6. The reviewed receipt and outlay totals reconcile to OMB Table 1.1 for the
   reviewed years, which confirms Table 1.1 is a useful fiscal spine.
7. Nothing in these slices traces an individual income-tax dollar to a specific
   program. Any future taxpayer receipt remains a labeled allocation model
   unless a legal dedication source is cited.

## What Remains Blocked

- Full IRS rates extraction is blocked until footnote parsing is implemented.
- Receipt allocation is blocked until fund-group and budget-concept records are
  reviewed.
- Outlay lane mapping is blocked until Table 3.2 subfunction records and a lane
  crosswalk are reviewed.
- Public taxpayer allocation is blocked until allocation method and deficit
  treatment are defined.
- Dynamic Treasury, USAspending, and CBO uses remain blocked until query
  snapshot rules are applied to actual captures.

## Practical Next Step

The next best data wave should not jump to public receipts. It should either:

1. implement a footnote-aware IRS Table 23 extractor, or
2. extract and review OMB Table 1.4 fund groups so receipt categories can be
   interpreted without overclaiming legal dedication.
