# IRS SOI Historical Table 23 Extraction Workspace

This directory is reserved for draft `rates_timeline` records extracted from
`SRC-IRS-SOI-HT23`.

## Source Artifact

- Metadata: `data/metadata/SRC-IRS-SOI-HT23.2026-06-21.metadata.md`
- Raw artifact: `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/histab23.xls`
- Schema: `docs/data/rates-timeline-schema.md`

## Planned Output

Use JSONL records named by source and observed date:

```text
rates_timeline.SRC-IRS-SOI-HT23.2026-06-21.draft.jsonl
```

Each row must include:

- `record_id`
- `record_family`
- `tax_year`
- `year_basis`
- `source_ids`
- `source_table`
- `source_row_ref`
- `filing_status`
- `rate_percent`
- `bracket_floor`
- `bracket_ceiling`
- `threshold_units`
- `tax_base_note`
- `status`
- `observed_date`
- `notes`

## Review Gates

1. Confirm workbook sheet names and source labels.
2. Confirm whether each row is statutory rate, bracket threshold, exemption,
   surtax, normal tax, or contextual table note.
3. Preserve tax-year basis.
4. Mark extracted rows `draft-extracted`.
5. Run `git diff --check`.
