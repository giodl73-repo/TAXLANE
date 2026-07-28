# Data Acquisition Eight Gap Status

Machine record:
`data/derived/breadth_benchmark_matrix/data_acquisition_eight_gap_status.v1.draft.json`

This packet records the follow-up acquisition pass across the eight open data
gaps. All eight were reviewed. New local custody was added for:

- CBO February 2026 Federal Subsidies for Health Insurance PDF/spreadsheet
  files, supplied through manual browser download after command-line access
  remained blocked;
- CMS Hospital Data Dictionary lineage context;
- CMS/QualityNet methodology surface HTML/JavaScript custody, recorded without
  treating app-shell bytes as complete methodology report content;
- selected CMS mortality methodology report content, recorded without treating
  selected mortality reports as complete all-measure quality/access lineage;
- partial CMS dataset denominator-field crosswalk context, recorded without
  treating field presence as complete denominator lineage or floor values;
- partial CMS/HRSA rural safety-net capacity context, recorded without treating
  CMS PSF fields or HRSA browser context as complete capacity series;
- OMB Public Budget Database User's Guide FY2027 horizon-boundary context,
  recorded without creating FY2032-FY2035 OMB 17-row values.
- Treasury Fiscal Data average-interest-rate context;
- Treasury Fiscal Data debt-to-the-penny, MSPD Table 1 debt context, and full
  MSPD Table 3/Table 5 maturity-detail context.
- Treasury MTS Table 8 FY2025 federal-fund context, recorded without relabeling
  federal funds as the general fund.
- Treasury MTS Tables 4 and 5 FY2025 Medicare HI fiscal-year anchor context,
  recorded without converting CMS calendar-year Trustees values into a fiscal
  path.
- Treasury MTS Tables 4 and 5 FY2025 transportation trust-fund anchor context,
  recorded without reconciling income/outgo, balances, or Function 400 mapping.
- Treasury MSPD latest-month Table 3/Table 5 maturity bucket diagnostics,
  recorded without combining the tables.
- OMB Table 13-4 transportation trust-fund internal identity diagnostics for
  FY2025-FY2031, recorded without Function 400 or solver mapping.
- CBO FY2032-FY2035 transportation trust-fund balance extension context,
  recorded without income/outgo reconciliation.
- OMB/CBO revenue overlap diagnostics for FY2026-FY2031 receipt source-vintage
  differences, recorded without choosing assigned bases.
- existing IRS SOI Publication 1304 Table 1.1 TY2023 individual-income context,
  attached to the receipt-base acquisition row without choosing a matched FY2025
  assigned base.
- IRS SOI Publication 16 Table 2.3 TY2022 corporate-income context, recorded
  without choosing a matched FY2025 assigned base.
- Medicare HI CMS/OMB FY2025 timing-perimeter diagnostics, recorded without
  converting calendar-year CMS values into fiscal-year solver values.
- USDA FNS SNAP annual summary, monthly, persons, households, benefits, and
  FY1969-current ZIP raw custody, recorded without treating SNAP custody as ERS
  food-security custody, a broader nutrition-program boundary, floor values, or
  solver input.

The pass did not complete the eight gates. The CBO health February 2026 PDF and
spreadsheet now have local raw custody through manual browser download, but the
July 2026 latest-publication page/PDF endpoints remain blocked by command-line
403, login-redirect, or Datadome boundary responses. OMB FY2032-FY2035 17-row
values, a general-fund annual path, a Medicare HI fiscal-year bridge, matched
receipt bases, transportation fund-balance reconciliation, and net-interest
remaining-maturity reconciliation across Treasury monthly detail and CBO/OMB
fiscal-year projections remain blocked or partial.
The remaining CBO latest-publication raw capture is an anti-bot/JavaScript challenge boundary,
not evidence that the source does not exist.

The machine record now also includes a supporting out-of-scope USDA custody block.
That block points to the existing income-security/family
food-hardship/nutrition packet with 8 ERS food-security raw/context files and 7 FNS SNAP raw files,
including the FNS FY1969-current ZIP. This supporting block
is not one of the primary eight gap closures, not complete USDA raw source
custody, not a complete nutrition-program boundary, not floor values, not solver
input, and not a rate or savings claim.

The new OMB PBD guide horizon-boundary context captures the official FY2027
Public Budget Database User's Guide PDF. It documents the three PBD files
for outlays, receipts, and budget authority and the FY2031 file horizon. It is
not FY2032-FY2035 OMB 17-row values, not interpolation, not solver input, and
not a balanced-budget claim.

The new CBO health-insurance context records the official selected program page,
local raw custody for the February 2026 Federal Subsidies for Health Insurance
PDF/spreadsheet, the May 11, 2026 CBO health-subsidy presentation
browser-visible PDF/data context, the July 23, 2026 CBO `Federal Subsidies for Health Insurance,
2026 to 2036` latest-publication boundary, and Table 2 rowmap context. May and
July 2026 local raw byte custody remain blocked by command-line access controls.
It is not a health federal policy score, not a solver input, and not a rate
calculation.

The new CBO health-insurance Table 2 browser rowmap assigns the February 2026
row order from visual review and is now supported by the official spreadsheet
in local raw custody. Current-law health solver paths, policy scores, rates,
savings, and balanced-budget claims remain blocked.

The new CMS/QualityNet methodology surface context records official CMS
Provider Data Catalog overall star rating, QualityNet mortality methodology,
and QualityNet overall ratings resources routes with eight local HTML, CSS, and
JavaScript files. The QualityNet captures are app-shell bytes, so this is not
complete methodology report content, not a denominator-to-field crosswalk, not
risk-adjustment case-mix lineage, not rural/safety-net capacity series, not
health floor values, and not pass/fail findings.

The new CMS hospital measure methodology report custody packet records the CMS
Measure Methodology page plus two selected mortality reports: Hybrid
Hospital-Wide Risk-Standardized Mortality Methodology Report Version 2.1 and the
2022 Condition-Specific Mortality Measures Updates and Specifications Report.
This is real methodology content custody, but it is still not complete
all-measure case-mix lineage, not a denominator-to-field crosswalk, not
rural/safety-net capacity series, not health floor values, and not pass/fail
findings.

The new CMS hospital quality dataset field crosswalk records six captured
Provider Data Catalog datasets, 479209 captured rows, and field-presence context
for Denominator, Sample, measure-count fields, and HAI measure-ID patterns. It
is not a complete denominator-to-dataset field crosswalk, not a
methodology-to-dataset join, not all-measure case-mix lineage, not floor
thresholds, not observed floor values, and not pass/fail findings.

The new CMS/HRSA rural safety-net capacity context records the CMS TEAM
safety-net/rural hospital fact sheet, the CMS Provider Specific Data page, and
the CMS Inpatient PSF October 2025 ZIP. It identifies PSF capacity fields
including bed-size, SSI, Medicaid, DSH, provider-type, state, and case-mix
context. HRSA FORHP rural data files are browser-visible official context, but
command-line retrieval returned access denied, so HRSA local raw custody and
facility-to-county rural joins remain blocked.

The new net-interest bucket diagnostic improves visibility into source-backed
remaining-maturity coverage, but it also exposes the unit/perimeter
reconciliation required before weighted average maturity, a remaining-maturity
schedule, a rate path, or solver input can be used.

The new Treasury average-interest-rate diagnostic records latest-month rate
context through `2026-06-30`, including Total Interest-bearing Debt at `3.409`
percent. It is not a fiscal-year rate path, CBO/OMB projection bridge, or solver
input.

The new transportation identity diagnostic verifies Table 13-4 internal
rounding reconciliation for the Highway Trust Fund and Airport and Airway Trust
Fund. It does not supply FY2032-FY2035 values, explicit general-fund transfers,
credited offsetting collections, Function 400 mapping, or solver input.

The new Treasury MTS transportation anchor context captures FY2025 Airport and
Airway Trust Fund and Highway Trust Fund receipt rows plus selected Table 5
outlay context. It is not transportation trust-fund income/outgo reconciliation,
fund-balance reconciliation, explicit transfer treatment, Function 400 mapping,
or solver input.

The new CBO transportation trust-fund balance extension context captures
FY2032-FY2035 Highway Trust Fund and Airport and Airway Trust Fund balances. It
does not supply OMB Table 13-4 FY2032-FY2035 income/outgo rows, explicit
general-fund transfers, credited offsetting collections, Function 400 mapping,
or solver input.

The new OMB/CBO revenue overlap diagnostic compares selected receipt categories
across FY2026-FY2031. Existing IRS SOI Publication 1304 Table 1.1 TY2023
individual-income custody is attached beside the corporate-income source, but
both remain context only. The row exposes source-vintage and base-context
differences before any matched receipt-base, legal/economic base, rate bridge,
or solver input can be used.

The new IRS SOI corporate-income context captures TY2022 active-corporation
return count, business receipts, income subject to tax, and income tax after
credits for corporations other than Forms 1120S, 1120-REIT, and 1120-RIC. It is
not a matched FY2025 assigned base, legal/economic receipt base, rate bridge, or
solver input.

The new Medicare HI Treasury MTS anchor diagnostic captures final FY2025
Federal Hospital Insurance Trust Fund receipts from MTS Table 4 and outlays from
MTS Table 5. It aligns with the existing rounded OMB FY2025 anchors, but it is
not a calendar-to-fiscal conversion, FY2025-FY2035 fiscal-year HI path,
income-category crosswalk, OMB/CMS receipt-row bridge, or solver input.

The new Treasury MTS Table 8 diagnostic captures FY2025 federal-fund receipts
and outlays and compares them with OMB Historical Table 1.4 federal-fund totals.
It exposes federal-fund/general-fund source-boundary work, but federal funds are
broader than the general fund and this is not a general-fund annual path.

No solver input, solver run, rate calculation, public rate card, savings
estimate, or balanced-budget claim is populated.
