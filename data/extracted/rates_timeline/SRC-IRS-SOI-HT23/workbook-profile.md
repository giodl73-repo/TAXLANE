# IRS SOI Table 23 Workbook Profile

## Artifact

- Source ID: `SRC-IRS-SOI-HT23`
- Raw artifact:
  `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/histab23.xls`
- SHA-256:
  `57AED4C02AC6C6DCD39D0FEA18CA231EBE22085ACEDF098B3B993FB154399557`

## Workbook Shape

| Sheet | Rows | Columns | Use |
|---|---:|---:|---|
| `Sheet1` | 245 | 15 | Source table, notes, footnotes, and source statement. |
| `Sheet2` | 0 | 0 | Empty. |
| `Sheet3` | 0 | 0 | Empty. |

## Sheet1 Regions

| Rows | Region | Extraction rule |
|---|---|---|
| 1 | Title | Preserve as table title in metadata or notes. |
| 2 | Units note | Amounts are dollars. Treat as nominal unless a derived record states otherwise. |
| 3-5 | Header rows | Build column meanings from merged-style header text. |
| 8 | Column-number row | IRS column references 1-7. Do not treat as data. |
| 9-114 | Data rows | Numeric tax-year rows, 1913-2018. |
| 115 | `N/A` note | Treat as note, not data. |
| 116-237 | Footnotes | Parse before promoting footnoted rows. |
| 238-245 | Source statement | Preserve in source metadata; do not treat as data. |

## Data Columns

| Excel column | IRS column | Meaning | Draft field mapping |
|---|---:|---|---|
| A | none | Tax year | `tax_year` |
| B | 1 | Personal exemption, single persons | `tax_base_note` until a separate exemption family exists. |
| C | 2 | Personal exemption, married couples | `tax_base_note` until a separate exemption family exists. |
| D | 3 | Personal exemption, dependents | `tax_base_note` until a separate exemption family exists. |
| E | 4 | Lowest-bracket regular tax rate | `rate_percent` for `regular-tax-lowest-bracket`. |
| F | 5 | Taxable income under lowest-bracket threshold | `bracket_ceiling` for `regular-tax-lowest-bracket`. |
| G | 6 | Highest-bracket regular tax rate | `rate_percent` for `regular-tax-highest-bracket`. |
| H | 7 | Taxable income over highest-bracket threshold | `bracket_floor` for `regular-tax-highest-bracket`. |

Columns I-O are empty in the captured workbook.

## Footnote Hazards

Rows after 1918 include bracketed footnote markers in data cells, for example
`[5] 3.0`. A full extraction needs a parser that preserves:

- the numeric value,
- the footnote number,
- the exact source cell text,
- and the footnote text from rows 116-237.

Do not promote footnoted rows beyond `draft-extracted` until that parser exists.

## Extraction Boundary

Table 23 gives lowest and highest bracket summary values. It does not provide
every intermediate statutory bracket. TAXLANE can use it for a historical rates
spine and top/lowest marginal-rate summaries, but complete statutory schedules
need statute or fuller IRS table extraction.
