# CMS/HRSA Rural Safety-Net Capacity Context

Machine record:
`data/derived/breadth_benchmark_matrix/cms_hrsa_rural_safety_net_capacity_context.v1.draft.json`

This packet records partial rural and safety-net hospital capacity context for
the health/Medicare quality/access source gap.

Local CMS custody:

- CMS TEAM safety-net and rural hospital fact sheet;
- CMS Provider Specific Data for Public Use page;
- CMS Inpatient PSF October 2025 ZIP.

The inpatient PSF ZIP contains:

- `IPSF_INP_2025-12-05.csv`: 332369 rows;
- `IPSF_INP_LRO_2025-12-05.csv`: 9413 rows;
- `IPSF_INP_LRO_2025-12-05.xlsx`.

Capacity fields identified in the inpatient PSF include `bedSize`,
`supplementalSecurityIncomeRatio`, `medicaidRatio`, `operatingDsh`,
`providerType`, `stateCode`, and `caseMixIndex`.

HRSA boundary:

- HRSA FORHP rural data files are browser-visible official context;
- command-line retrieval of the HRSA page and rural health areas XLSX returned
  an access-denied boundary;
- HRSA local raw custody is not ready.

CMS rural/safety-net definition context, CMS Provider Specific File page custody,
and CMS inpatient PSF ZIP custody are locally captured, and HRSA FORHP rural
data files are browser-visible official context, but HRSA local raw custody and
facility-to-county rural joins remain blocked. This is not a complete
rural/safety-net capacity series, not floor thresholds, not observed floor
values, not pass/fail findings, not solver input, not rate calculation, and not
a balanced-budget claim.
