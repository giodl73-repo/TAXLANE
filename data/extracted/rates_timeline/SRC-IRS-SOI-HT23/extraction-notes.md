# IRS SOI Table 23 Draft Extraction Notes

## Source

- Source ID: `SRC-IRS-SOI-HT23`
- Raw artifact:
  `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/histab23.xls`
- Metadata:
  `data/metadata/SRC-IRS-SOI-HT23.2026-06-21.metadata.md`
- Schema: `docs/data/rates-timeline-schema.md`

## First Slice

The first draft extraction covers Sheet1 rows 9-14, tax years 1913-1918, for
the IRS Table 23 lowest and highest regular-tax bracket summary columns:

- Lowest bracket rate and taxable income under: columns E-F.
- Highest bracket rate and taxable income over: columns G-H.

These are summary records, not complete statutory rate schedules.

## Extraction Decisions

- `filing_status` is `not_stated` because the rate summary columns do not split
  rates by modern filing status.
- `bracket_floor` is null for the lowest-bracket summary rows because the source
  reports taxable income "under" a threshold and also reports personal
  exemptions. The draft does not infer a zero floor.
- `bracket_ceiling` is null for highest-bracket rows because the source reports
  taxable income "over" a threshold.
- `authority_event_id` is null until formal `history_event` records exist.
- `status` is `draft-extracted`; no source or budget review has promoted these
  rows.

## Hazards

- Table 23 includes footnoted values in later years.
- The workbook has empty Sheet2 and Sheet3 tabs.
- Later extraction needs a footnote parser before rows with bracketed notes are
  promoted.
