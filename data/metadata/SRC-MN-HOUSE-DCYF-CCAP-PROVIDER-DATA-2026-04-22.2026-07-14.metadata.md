# Source Metadata: SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22

| Field | Value |
|---|---|
| `source_id` | `SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22` |
| `publisher` | Minnesota Department of Children, Youth, and Families |
| `title` | MN DCYF First Production provider data |
| `source_url` | <https://www.house.mn.gov/comm/docs/oyZeI7aBIUu8IIo8wDk6qw.pdf> |
| `published_date` | Not established by the captured PDF. |
| `document_date` | PDF creation and modification metadata report 2026-04-22; treat this as the DCYF production/document date, not an independently verified House posting or publication date. |
| `observed_date` | 2026-07-14 |
| `retrieved_at` | 2026-07-14; precise retrieval time was not recorded. |
| `capture_method` | PowerShell `Invoke-WebRequest`; byte count and SHA-256 verified locally; `pdfinfo` and `pdftotext -layout` used for inspection. |
| `raw_path` | `data/raw/minnesota-house/SRC-MN-HOUSE-DCYF-CCAP-PROVIDER-DATA-2026-04-22/2026-07-14/dcyf-ccap-provider-data.pdf` |
| `bytes` | `1277757` |
| `checksum_sha256` | `E7068E1198D8DCE851907B60FC4A2A16FEDD5DE7A1D41AFCD2B02DCAABF3DEC1` |
| `pages` | 7 PDF file pages. |
| `scope_used` | PDF file page 6, provider row for license 1087038, including license status, inactive date, annual CCAP payments, and aggregate overpayment fields. |
| `status` | `source-reviewed`; Minnesota House-hosted official copy captured and checksum verified. |
| `secret_scan` | No credential, token, private-key, password, or authorization-header patterns found in the extracted text. |
| `notes` | The table is separately owned official context for the witness statement. It does not record the testimony's cutoff or basis, a recipient response, the initiator or reason for license closure, or a finding that all or any CCAP payments were fraudulent or improper. |

## Concise extraction note

PDF file page 6 identifies license 1087038 as `Quality Learning Center Inc`,
at 1411 Nicollet Ave, Minneapolis, Minnesota 55403, with a current or most
recent licensed capacity of 99. The official name spells `Learning` correctly;
the testimony's `Quality Learing Center` is retained only when attributing the
witness statement.

The same row reports `License Status` as `Closed`, an open date of 10/17/2017,
and a `License Inactive Date` of 1/6/2026. The table does not state who
initiated closure, the authority or reason, or any causal relationship among
closure, CCAP payments, complaints, violations, the assessed or repaid
overpayment, or the testimony.

The row reports CCAP payments by calendar year as follows:

| Calendar year | CCAP payments |
|---|---:|
| CY2018 | $1,028,063 |
| CY2019 | $1,521,528 |
| CY2020 | $1,124,721 |
| CY2021 | $1,361,054 |
| CY2022 | $1,433,733 |
| CY2023 | $1,210,794 |
| CY2024 | $1,730,115 |
| CY2025 | $2,150,964 |

The testimony's source-stated 2025 $1.9 million does not equal the table's
CY2025 annual value. The table can support entity identity and the reported
annual payment totals, but it cannot establish the testimony's data cutoff,
whether the figure is year-to-date as of the December 16 visit narrative, or
its calculation basis.

The same row reports 159 unique CCAP children served in CY2025, three licensing
complaints received in CY2025, and 13 licensing violations determined in
CY2025. It also reports total CCAP overpayments assessed of $69,365 and total
repayments of $69,365 without an annual allocation in the row. Complaints,
violations, and assessed overpayments have distinct meanings and scopes; they
do not establish fraud, make every payment improper, create a current debt, or
show that the testimony's $1.9 million was improper.

## Recommended context use

Attach the source to the existing testimony atom as `supplies_context`. Keep
the claim spelling and source-stated CY2025 period, leave subject IDs, corroboration,
counterevidence, and response-source arrays empty, and keep every attributed
and substantive public gate false. Seek transaction-level payment lineage and
the testimony's cutoff and calculation basis before considering exact-amount
corroboration or counterevidence.
Treat the closed status and inactive date as license context only, not as an
explanation of closure or evidence of misconduct.
