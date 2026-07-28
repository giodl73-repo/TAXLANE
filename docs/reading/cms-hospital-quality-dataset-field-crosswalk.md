# CMS hospital quality dataset field crosswalk

Machine record:
`data/derived/breadth_benchmark_matrix/cms_hospital_quality_dataset_field_crosswalk.v1.draft.json`

This packet records a partial field-level crosswalk across the six captured CMS
Provider Data Catalog hospital quality/access datasets. It identifies where the
captured files expose denominator-like fields or measure-count context:

- Hospital General Information: measure-count fields for star-rating groups;
- Complications and Deaths - Hospital: `Denominator`;
- Healthcare Associated Infections - Hospital: HAI measure-ID patterns such as
  `_ELIGCASES`, `_DOPC`, `_NUMERATOR`, and `_SIR`;
- Unplanned Hospital Visits - Hospital: `Denominator`;
- Timely and Effective Care - Hospital: `Sample`;
- Rural Emergency Hospital Timely and Effective Care - Hospital: `Sample`.

Custody status:

- captured dataset count: 6;
- captured total rows: 479209;
- partial denominator or measure-count field presence ready;
- complete denominator-to-dataset field crosswalk is not ready;
- measure methodology to dataset join is not ready;
- all-measure case-mix lineage is not ready.

CMS hospital quality dataset field crosswalk custody is partially ready for six
captured Provider Data Catalog datasets, including Denominator, Sample,
measure-count fields, and HAI measure-ID pattern context. This is not a complete
denominator-to-dataset field crosswalk, not a methodology-to-dataset join, not
all-measure case-mix lineage, not floor thresholds, not observed floor values,
not pass/fail findings, not solver input, not rate calculation, and not a balanced-budget claim.
