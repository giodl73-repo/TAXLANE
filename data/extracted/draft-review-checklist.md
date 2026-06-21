# Draft Record Review Checklist

## Purpose

This checklist defines how TAXLANE draft extracted records can move from
`draft-extracted` to reviewed status. It applies to rates, receipts, outlays,
fund groups, programs, and lane crosswalks.

Draft records are not public claims. They are source-backed working records
that still need role review.

## Status Path

| Status | Meaning | Required review |
|---|---|---|
| `draft-extracted` | Parsed from a captured source artifact but not reviewed. | None beyond JSON/schema validation. |
| `source-reviewed` | Source identity, version, row anchors, and labels checked. | Source Custodian. |
| `budget-reviewed` | Year basis, budget semantics, and reconciliation checked. | Budget Accountant. |
| `reviewed` | Source and budget review complete, with caveats recorded. | Source Custodian and Budget Accountant. |
| `superseded` | Replaced by a later source version or corrected extraction. | Source Custodian. |
| `retired` | Should not be used. | Reviewer note required. |

## Source Custodian Checklist

Before promoting to `source-reviewed`, confirm:

1. Every `source_id` appears in `docs/sources/source-version-ledger.md`.
2. The source artifact has metadata in `data/metadata/`.
3. Raw artifact checksum matches the metadata record.
4. `source_table` names the table and release.
5. `source_row_ref` is specific enough to locate the source row or cell range.
6. Exact source labels are preserved in label fields.
7. Footnotes, bracketed notes, `N/A`, stars, dotted blanks, and negative values
   are preserved or explicitly documented.
8. No dynamic query record is used without query parameters, timestamp,
   pagination, and checksum.

## Budget Accountant Checklist

Before promoting to `budget-reviewed`, confirm:

1. `year_basis` matches the schema: tax year for rates and fiscal year for OMB
   receipt/outlay records.
2. Amount rows have `amount` and null `percent`.
3. Percentage rows have `percent` and null `amount`.
4. Rates rows do not turn summary lowest/highest bracket rows into complete
   statutory schedules.
5. Receipt rows keep individual income taxes, corporation income taxes, and
   social-insurance receipts separate.
6. Fund labels do not claim legal dedication without budget-concept support.
7. Outlay rows keep net interest visible.
8. Outlay rows keep undistributed offsetting receipts visible and labeled.
9. Totals reconcile to the relevant fiscal spine within displayed precision.
10. Public allocation remains blocked unless allocation method and deficit
    treatment are defined.

## Public-Claim Gate

Before any public doc uses extracted values:

1. Link to the source IDs and extracted record family.
2. State whether records are draft, source-reviewed, budget-reviewed, or
   reviewed.
3. State tax-year versus fiscal-year basis.
4. State units.
5. Avoid implying legal dedication or taxpayer-dollar tracing unless cited.
6. Include deficit or borrowing context for spending allocation claims.

## Current Draft Records

| Record family | Draft file | Review blocker |
|---|---|---|
| `rates_timeline` | `data/extracted/rates_timeline/SRC-IRS-SOI-HT23/rates_timeline.SRC-IRS-SOI-HT23.2026-06-21.draft.jsonl` | Footnote parser and formal source review. |
| `receipt_source` | `data/extracted/receipt_source/receipt_source.SRC-OMB-HIST-2-1-FY2027.2026-06-21.draft.jsonl` | Fund-group allocation review and broader Table 2.1 extraction plan. |
| `outlay_function` | `data/extracted/outlay_function/outlay_function.SRC-OMB-HIST-3-1-FY2027.2026-06-21.draft.jsonl` | Function/subfunction crosswalk review before lane use. |
