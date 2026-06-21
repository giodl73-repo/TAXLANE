# IRS Rates Source Review

## Scope

This review covers only the first IRS rates draft slice:

- Draft records:
  `data/extracted/rates_timeline/SRC-IRS-SOI-HT23/rates_timeline.SRC-IRS-SOI-HT23.2026-06-21.draft.jsonl`
- Raw artifact:
  `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/histab23.xls`
- Metadata:
  `data/metadata/SRC-IRS-SOI-HT23.2026-06-21.metadata.md`
- Schema:
  `docs/data/rates-timeline-schema.md`

It covers 12 extracted rows: 1913-1918 lowest and highest regular-tax bracket
summary records.

## Source Custodian Findings

| Check | Result |
|---|---|
| Source ID exists in ledger | Pass: `SRC-IRS-SOI-HT23` is recorded. |
| Metadata exists | Pass: metadata file records source URL, artifact URL, observed date, raw path, checksum, schema, and status. |
| Raw checksum matches metadata | Pass: SHA-256 is `57AED4C02AC6C6DCD39D0FEA18CA231EBE22085ACEDF098B3B993FB154399557`. |
| Source table named | Pass: draft rows identify IRS SOI Historical Table 23 and Sheet1. |
| Row anchors specific | Pass: row anchors use Sheet1 row and column ranges. |
| Source values match row anchors | Pass: all 12 draft records match workbook tax year, rate, and threshold cells. |
| Source labels preserved | Pass: draft notes preserve lowest/highest regular-tax summary framing. |
| Footnote hazards documented | Pass: the reviewed rows are before later bracketed footnote values; broader extraction still requires the footnote parser documented in `workbook-profile.md`. |
| Dynamic query issue | Not applicable. |

## Validation Evidence

The review script checked:

- raw artifact SHA-256,
- JSONL parse for 12 rows,
- `Sheet1!A9:H14` row references,
- lowest-bracket rates and ceiling thresholds from columns E-F,
- highest-bracket rates and floor thresholds from columns G-H.

Result:

```text
hash ok
row_refs ok
rows 12
years 1913 1918
```

## Decision

The reviewed 1913-1918 IRS rates slice is eligible for `source-reviewed`
status.

This does not review the full IRS Table 23 workbook. Rows after 1918 remain
blocked from promotion until footnote parsing and exact source-cell preservation
are implemented.

## Public-Use Caveat

These records are summary lowest/highest bracket records, not complete statutory
rate schedules. Public docs may use them only with that caveat and without
claiming they represent every bracket in the tax year.
