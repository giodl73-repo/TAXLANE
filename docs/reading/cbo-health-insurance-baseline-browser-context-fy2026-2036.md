# CBO Health Insurance Baseline Browser Context

Machine record:
`data/derived/breadth_benchmark_matrix/cbo_health_insurance_baseline_browser_context_fy2026_2036.v1.draft.json`

This packet records official context from CBO's selected-program baseline page
and local raw custody for the February 2026 Federal Subsidies for Health
Insurance PDF and spreadsheet.
CBO's page lists health baseline categories for Children's Health Insurance
Program, Federal Subsidies for Health Insurance, Medicaid, Medicare, and Premium
Tax Credit and Related Spending.

The July 23, 2026 CBO publication `Federal Subsidies for Health Insurance, 2026
to 2036` is now recorded as latest official public context. The publication page
and likely July 2026 system-file endpoints still return 403 responses or login
redirects from command-line retrieval in this environment, so this update does
not claim July 2026 local raw byte custody.

The May 11, 2026 CBO presentation `CBO's Baseline Projections of Federal
Subsidies for Health Insurance` is also recorded as browser-visible official
context. Its page identifies the document as a presentation, and the
browser-visible PDF has 23 pages. CBO also exposes a `62380-Data.xlsx` workbook
link for data underlying figures and tables. Command-line retrieval of the May
2026 PDF and workbook returned 403 responses in this environment, so this
packet does not claim May 2026 local raw byte custody, byte counts, or SHA-256
hashes.

The PDF is browser-visible official context. Its table context says CBO/JCT
baseline projections cover health insurance coverage and federal subsidy costs,
and Table 2 is federal subsidies for health insurance under current law. The
browser-visible notes exclude discretionary outlays and federal-employer
outlays from those subsidy estimates.

Local raw custody is captured for the February 2026 files after manual browser
download supplied by the repository owner:

- PDF: `data/raw/cbo/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02/2026-07-24/51298-2026-02-healthinsurance.pdf`;
- PDF byte count: 747901;
- PDF SHA-256:
  `2c24c10b855be1e1a9e9c87a30ddaf5b4c62a8dbe9d92d3e2ebf18524a54d349`;
- spreadsheet:
  `data/raw/cbo/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02/2026-07-24/51298-2026-02-healthinsurance.xlsx`;
- spreadsheet byte count: 42861;
- spreadsheet SHA-256:
  `f2d7cc186f3a0afa909e648f8224a1f7f80af202db234848b507fd49416e1001`;
- metadata:
  `data/metadata/SRC-CBO-FEDERAL-SUBSIDIES-HEALTH-INSURANCE-2026-02.2026-07-24.metadata.md`.

Workbook inspection found:

- `healthinsuranceT1_02-2026`: `A1:AA76`, 76 sheet rows;
- `healthinsuranceT2_02-2026`: `A1:Y175`, 175 sheet rows.

Command-line retrieval of the CBO health spreadsheet and PDF endpoints returned
an anti-bot/JavaScript challenge, including a Datadome access boundary for the
PDF attempt. The July 2026 CBO health publication probes returned 403 responses
or login redirects, so latest-publication raw custody remains blocked.

This is not a health federal policy score, not a current-law health solver path,
not solver input, not a solver run, not a rate calculation, not a public rate
card, not a savings estimate, and not a balanced-budget claim.
