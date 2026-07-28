# Social Security old-age poverty context bridge schema

`social_security_old_age_poverty_context_bridge.v1.draft.json` bridges existing
OECD old-age relative-poverty source custody into the Social Security lane.

Required checks:

- `record_family` is `social_security_old_age_poverty_context_bridge`.
- The record links the Social Security source-capture rollup, Wave D floor-value
  readiness audit, and existing age-relative-poverty country panel.
- Source custody points to `SRC-OECD-IDD-AGE-POVERTY-PANELS`, the existing raw
  old-age-poverty CSV, metadata, byte count, and SHA-256.
- The bridge may publish international old-age relative-poverty context only.
- Domestic old-age poverty custody, near-poverty context, income-unit boundary,
  threshold rationale, floor values, pass/fail findings, target costs, solver
  inputs, rates, savings, and balanced-budget claims remain blocked.
