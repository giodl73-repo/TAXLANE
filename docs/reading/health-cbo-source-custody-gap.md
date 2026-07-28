# Health CBO source custody gap

Machine record:
`data/derived/breadth_benchmark_matrix/health_cbo_source_custody_gap.v1.draft.json`

Pulse 181 records a specific health/Medicare source gap: CBO source IDs appear
in derived health context artifacts, official CBO browser access is documented,
February 2026 CBO health-insurance PDF/spreadsheet raw custody is captured
through manual browser download, and the May 11, 2026 CBO health-subsidy
presentation page/PDF/data workbook are browser-visible context only. Complete
CBO source capture remains blocked.

Referenced but not raw-custody-ready:

- `SRC-CBO-LTBO`
- `SRC-CBO-COMMERCIAL-PROVIDER-PRICES`

Required before CBO can populate federal health policy translation or score
context fields:

- official access boundary;
- February 2026 health baseline raw custody;
- browser-verified Table 2 rowmap;
- raw artifact path;
- raw byte count;
- raw SHA-256;
- metadata path;
- retrieval date;
- health baseline table lineage;
- behavior and incidence table lineage.

Official access boundary:

- `SRC-CBO-LTBO`: CBO publication page `https://www.cbo.gov/publication/61270`
  and official PDF `https://www.cbo.gov/system/files/2025-03/61187-Long-Term-Outlook-2025.pdf`
  were browser-reviewed on 2026-07-24.
- `SRC-CBO-COMMERCIAL-PROVIDER-PRICES`: CBO publication page
  `https://www.cbo.gov/publication/57778` and official PDF
  `https://www.cbo.gov/system/files/2022-01/57422-medical-prices.pdf` were
  browser-reviewed on 2026-07-24.
- Command-line raw download returned HTTP 403 for the older CBO reference
  probes, so those local raw byte custody fields remain blocked.
- `SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02`: official February 2026
  Federal Subsidies for Health Insurance PDF and spreadsheet are now in local
  raw custody after manual browser download.
- `no_source_id_assigned_cbo_62380_browser_context_only`: CBO publication
  `https://www.cbo.gov/publication/62380`, the 23-page official presentation
  PDF, and the `62380-Data.xlsx` workbook link are browser-visible, but
  command-line raw custody returned HTTP 403 and no local byte custody is
  claimed.
- `cbo-health-insurance-table2-browser-rowmap-fy2026-2036.md` assigns the
  February 2026 Federal Subsidies for Health Insurance Table 2 row order from
  browser visual review and is supported by local spreadsheet custody.

CBO source IDs appear in derived health context artifacts, official CBO browser access is documented, the February 2026 CBO health-insurance PDF/spreadsheet raw files are captured through manual browser download, the May 11, 2026 CBO health-subsidy presentation page/PDF/data workbook are browser-visible context only, and the February 2026 health-insurance Table 2 rowmap is assigned as context only, but CBO source capture is still incomplete because other CBO health references plus May and July 2026 raw files remain blocked. This is not complete CBO source capture, not federal health policy translation, not behavior modeling, not incidence modeling, not pass/fail findings, not lower-cost scenario admissibility, not target-cost selection, not gross savings, not net savings, not solver input, not rate calculation, not a public rate card, not a technology-savings claim, and not a balanced-budget claim.
