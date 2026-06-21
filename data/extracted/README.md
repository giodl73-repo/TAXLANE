# Extracted Draft Records

This directory holds schema-backed records extracted from raw source artifacts.

## Layout

Use source-family directories:

```text
data/extracted/{record-family}/{source-id}/
```

Examples:

```text
data/extracted/rates_timeline/SRC-IRS-SOI-HT23/
data/extracted/receipt_source/SRC-OMB-HIST-2-1-FY2027/
data/extracted/outlay_function/SRC-OMB-HIST-3-2-FY2027/
```

## Rules

1. Extracted records must include `source_ids`, `source_table`,
   `source_row_ref`, `observed_date`, and `status`.
2. Draft extracted records should use `draft-extracted` until source review and
   budget review are complete.
3. Do not publish extracted values as public claims until the status is
   `reviewed` or the public doc clearly labels the draft status.
4. Do not place raw spreadsheets or PDFs here.
5. Do not place modeled taxpayer allocations here.

## Format Decision

Use JSONL as the default extracted-record format because every row can carry
source IDs, status, notes, and optional fields without widening CSV tables.
CSV may be added later for simple tabular exports.
