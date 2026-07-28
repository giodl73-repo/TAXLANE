# Health quality/access indicator source gap

Machine record:
`data/derived/breadth_benchmark_matrix/health_quality_access_indicator_source_gap.v1.draft.json`

Pulse 182 records a specific health/Medicare source gap: quality, access,
risk-adjusted outcome, rural-capacity, and safety-net-capacity indicator
families are needed before floor passage. Local raw custody is now partially
ready for six CMS Provider Data Catalog hospital datasets, CMS Hospital Data
Dictionary lineage context is locally captured, and CMS/QualityNet methodology
surface HTML/JavaScript custody is locally captured, but the packet still
blocks floor passage because complete denominator, all-measure case-mix,
rural/safety-net, and threshold review are not ready.

Captured CMS Provider Data Catalog context:

- Hospital General Information;
- Complications and Deaths - Hospital;
- Healthcare Associated Infections - Hospital;
- Unplanned Hospital Visits - Hospital;
- Timely and Effective Care - Hospital;
- Rural Emergency Hospital Timely and Effective Care - Hospital.

Captured CMS lineage context:

- Hospital DDB Data Dictionary April 2026.

Captured CMS/QualityNet methodology surface context:

- CMS Provider Data Catalog Overall Hospital Quality Star Rating topic;
- QualityNet inpatient mortality methodology route;
- QualityNet overall ratings resources route;
- raw file count: 8;
- raw total byte count: 14167176.

Captured CMS methodology report content:

- CMS Measure Methodology page;
- Hybrid Hospital-Wide Risk-Standardized Mortality Methodology Report Version
  2.1;
- 2022 Condition-Specific Mortality Measures Updates and Specifications Report;
- raw file count: 3;
- raw total byte count: 6518093.

Captured CMS dataset field crosswalk context:

- six captured Provider Data Catalog datasets;
- Denominator, Sample, measure-count fields, and HAI measure-ID pattern context;
- captured total rows: 479209.

Captured rural/safety-net capacity context:

- CMS TEAM safety-net and rural hospital fact sheet;
- CMS Provider Specific Data for Public Use page;
- CMS Inpatient PSF October 2025 ZIP;
- CMS PSF capacity fields including `bedSize`, `supplementalSecurityIncomeRatio`,
  `medicaidRatio`, `operatingDsh`, `providerType`, `stateCode`, and
  `caseMixIndex`;
- HRSA FORHP rural data files are browser-visible official context, but HRSA
  local raw custody remains blocked by command-line access controls.

Source families needed but not custody-ready:

- complete CMS quality/access denominator-to-dataset field crosswalk;
- complete all-measure risk-adjusted outcome case-mix methodology crosswalk
  across selected floor indicators;
- rural and safety-net capacity series beyond rural emergency hospital
  timely/effective-care context.

Required before these indicators can populate floor thresholds, observed values,
or pass/fail findings:

- raw artifact path: ready for six CMS CSVs;
- raw byte count: ready for six CMS CSVs;
- raw SHA-256: ready for six CMS CSVs;
- metadata path: ready for the custody packet;
- retrieval date: 2026-07-24;
- official methodology surface HTML/JavaScript custody: ready for CMS Provider
  Data Catalog and QualityNet app-shell custody, but methodology report content
  remains incomplete;
- selected mortality methodology report content: ready for CMS hospital-wide
  risk-standardized mortality and condition-specific mortality methodology
  report custody, but the complete all-measure case-mix crosswalk remains
  incomplete;
- partial denominator or measure-count field presence: ready for field-presence
  context, but the complete denominator-to-dataset field crosswalk remains
  incomplete;
- partial rural/safety-net capacity context: ready for CMS definition and PSF
  capacity-field context, but HRSA local raw custody and facility-to-county
  joins remain incomplete;
- indicator definition and denominator lineage: partial data dictionary context
  captured, but the dataset-specific denominator-to-field crosswalk remains
  incomplete;
- risk adjustment and case mix lineage;
- rural and safety-net capacity lineage.

CMS Provider Data Catalog hospital quality/access raw context custody is partially ready for six local datasets, CMS Hospital Data Dictionary lineage context is locally captured, CMS/QualityNet methodology surface HTML/JavaScript custody is locally captured, selected CMS mortality methodology report content is locally captured, partial CMS dataset denominator-field crosswalk context is locally captured, and partial CMS/HRSA rural safety-net capacity context is locally captured, but complete denominator-to-field crosswalk, complete all-measure case-mix lineage, rural capacity series, safety-net capacity series, threshold selection, observed values, and pass/fail lineage remain blocked. This is not complete CMS quality/access source capture, not health floor threshold selection, not observed floor values, not pass/fail findings, not lower-cost scenario admissibility, not a federal policy score, not target-cost selection, not gross savings, not net savings, not solver input, not rate calculation, not a public rate card, not a technology-savings claim, and not a balanced-budget claim.
