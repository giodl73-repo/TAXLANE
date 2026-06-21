# Source Metadata: SRC-IRS-SOI-HT23

| Field | Value |
|---|---|
| `source_id` | `SRC-IRS-SOI-HT23` |
| `publisher` | Internal Revenue Service, Statistics of Income |
| `source_url` | <https://www.irs.gov/statistics/soi-tax-stats-historical-table-23> |
| `artifact_url` | <https://www.irs.gov/pub/irs-soi/histab23.xls> |
| `observed_date` | 2026-06-21 |
| `capture_method` | Scripted fetch with PowerShell `Invoke-WebRequest` after resolving the official IRS page link. |
| `raw_path` | `data/raw/irs/SRC-IRS-SOI-HT23/2026-06-21/histab23.xls` |
| `checksum_sha256` | `57AED4C02AC6C6DCD39D0FEA18CA231EBE22085ACEDF098B3B993FB154399557` |
| `schema_file` | `docs/data/rates-timeline-schema.md` |
| `coverage` | Individual income tax rates and brackets from 1913 through the table's current endpoint. |
| `status` | `captured` |
| `notes` | IRS page link resolved to `/pub/irs-soi/histab23.xls`; no row extraction performed in this pulse. |

## Extraction Constraints

- Extract into `data/extracted/rates_timeline/SRC-IRS-SOI-HT23/`.
- Preserve IRS table labels and tax-year semantics.
- Do not mix nominal and inflation-adjusted values in one field.
- Keep draft records at `draft-extracted` until row anchors and rate/bracket
  semantics are reviewed.
