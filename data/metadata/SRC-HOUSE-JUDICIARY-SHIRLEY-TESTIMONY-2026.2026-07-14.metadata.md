# Source Metadata: SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026

| Field | Value |
|---|---|
| `source_id` | `SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026` |
| `publisher` | U.S. House Committee on the Judiciary |
| `title` | Written testimony of Nick Shirley |
| `source_url` | <https://judiciary.house.gov/sites/evo-subsites/republicans-judiciary.house.gov/files/evo-media-document/shirley-testimony.pdf> |
| `observed_date` | 2026-07-14 |
| `retrieved_at` | 2026-07-14T19:04:46-07:00 |
| `capture_method` | PowerShell `Invoke-WebRequest`; byte count and SHA-256 verified locally; `pdfinfo` and `pdftotext -layout` used for inspection. |
| `raw_path` | `data/raw/house/SRC-HOUSE-JUDICIARY-SHIRLEY-TESTIMONY-2026/2026-07-14/shirley-testimony.pdf` |
| `bytes` | `60433` |
| `checksum_sha256` | `E90266A876DCB6882593A1A63DF70646270C7F9A037F6BA49D20F9E310C040C5` |
| `pages` | 1 PDF file page. |
| `document_date` | PDF creation and modification metadata report 2026-01-21; this is document metadata, not an independently verified publication timestamp. |
| `schema_file` | Not applicable; custody and attributed-testimony extraction only. |
| `coverage` | Shirley's account of his Minnesota research and December 16 visits, amounts received in 2025 for the visited providers, the Quality Learing Center amount, and his allegations about fraud and government response. |
| `status` | `source-reviewed`; official-host copy captured and checksum verified. |
| `secret_scan` | No credential, token, private-key, password, or authorization-header patterns found in the captured bytes or extracted text. |
| `notes` | Official House hosting establishes custody of submitted testimony, not committee adoption, truth of the statements, an official finding, or independent corroboration. The testimony does not state the Pulse 53 claim that more than $110 million was uncovered in one day. |

## Concise extraction note

All locations below are PDF file page 1.

- Paragraph 3 attributes a 2025 CCAP amount to the first unnamed daycare: “received over $1 million in CCAP funding.”
- Paragraph 3 introduces the visited providers' enrollment and CCAP amounts as amounts received in 2025, then attributes $1.9 million in CCAP funding to Quality Learing Center. The source does not disclose the amount's data cutoff, whether it is year-to-date as of the December 16 visit narrative, or its calculation basis.
- Paragraph 4 alleges that the reporting preceded an HHS freeze of $185 million in childcare funding and that no business had submitted proof of legitimacy. This is testimony, not an agency record or independently verified response.
- Paragraph 5 characterizes the subject as widespread Minnesota fraud. This is the witness's conclusion, not an official finding.

## Recommended custody-supported atom

Record only an attributed testimony atom: Nick Shirley testified that Quality
Learing Center had received $1.9 million in CCAP funding. Model the amount as a
source-stated exact `1.9` million USD payment assertion for `CY2025`; the data
cutoff, calculation basis, program payment universe, and underlying accuracy
remain unverified. Keep substantive,
official-finding, fraud, debt, recovery, prevention, and savings gates false.

This captured testimony does not support the separate more-than-$110-million
one-day atom; that amount requires custody of its own originating publication.
