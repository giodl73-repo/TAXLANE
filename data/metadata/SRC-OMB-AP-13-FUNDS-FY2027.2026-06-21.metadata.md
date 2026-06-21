# Source Metadata: SRC-OMB-AP-13-FUNDS-FY2027

| Field | Value |
|---|---|
| `source_id` | `SRC-OMB-AP-13-FUNDS-FY2027` |
| `publisher` | Office of Management and Budget |
| `source_url` | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_13_funds_fy2027.pdf> |
| `artifact_url` | <https://www.whitehouse.gov/wp-content/uploads/2026/04/ap_13_funds_fy2027.pdf> |
| `observed_date` | 2026-06-21 |
| `capture_method` | Scripted fetch with PowerShell `Invoke-WebRequest`. |
| `raw_path` | `data/raw/omb/SRC-OMB-AP-13-FUNDS-FY2027/2026-06-21/ap_13_funds_fy2027.pdf` |
| `checksum_sha256` | `6A332E8291DB7F8E6A4252C79E444C782F5CF2D369CAE4B738FE63B7DC0D4437` |
| `schema_file` | `docs/data/receipts-funds-schema.md` |
| `coverage` | OMB Analytical Perspectives FY2027 Chapter 13, trust funds and federal funds. |
| `status` | `captured` |
| `notes` | Use for fund group concepts before interpreting fund-group rows or legal dedication. |

## Extraction Constraints

- Extract concept records into `data/extracted/budget_concept/SRC-OMB-AP-13-FUNDS-FY2027/`.
- Preserve page or section anchors from the captured PDF.
- Do not treat broad fund-group concepts as statute-specific legal dedication.
- Use paraphrased concept summaries in records; public docs can quote only short
  excerpts when needed.
