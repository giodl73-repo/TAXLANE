# Source Metadata: SRC-USDA-OIG-FCIC-RMA-FS-FY2024

| Field | Value |
|---|---|
| `source_id` | `SRC-USDA-OIG-FCIC-RMA-FS-FY2024` |
| `publisher` | U.S. Department of Agriculture, Office of Inspector General |
| `title` | Federal Crop Insurance Corporation/Risk Management Agency's Financial Statements for Fiscal Year 2024 |
| `audit_number` | `05403-0001-11` |
| `source_url` | <https://usdaoig.oversight.gov/sites/default/files/reports/2024-11/05403-0001-11_FR_508.pdf> |
| `source_date` | 2024-11-12 |
| `observed_date` | 2026-07-13 |
| `capture_method` | Scripted fetch of the official USDA OIG PDF with PowerShell `Invoke-WebRequest`. |
| `raw_path` | `data/raw/usda/SRC-USDA-OIG-FCIC-RMA-FS-FY2024/2026-07-13/05403-0001-11_FR_508.pdf` |
| `bytes` | 7242677 |
| `checksum_sha256` | `0797BD2CCB1027B568BCE3B640849E89F30A235F528B1B1A2B249D525695ED32` |
| `evidence_locator` | Printed page 18 (PDF file page 29; zero-based viewer index 28) and printed pages 60-61 (PDF file pages 71-72; zero-based viewer indices 70-71). |
| `coverage` | FCIC/RMA states that its improper-payment rate considers premium subsidy, Administrative and Operating expense, and indemnity payments; separates payments by AIP into high, medium, and low tiers; draws an annual statistically valid sample; reports the FY2024 result; and says payment-integrity methodologies did not change. |
| `status` | `captured` |
| `notes` | The payment-integrity table and discussion on printed pages 60-61 are explicitly labeled Other Information (Unaudited). Printed page 18 is in Management's Discussion and Analysis (Unaudited). The FY2024 table appears to print `$579.93M` as overpayments, conflicting with the authoritative PaymentAccuracy annual workbook value of `$573.93M`; that table cell is excluded from numeric use. The source does not disclose sample-frame construction, tier allocation, selection probabilities, randomization details, policy-level exclusions, review exclusions, projection weights, variance method, debt, collections, prevention attribution, recoverability, or savings. |
