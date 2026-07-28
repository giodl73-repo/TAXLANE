# Social Security domestic old-age poverty context bridge schema

`social_security_domestic_old_age_poverty_context_bridge.v1.draft.json` bridges
existing Census P60-287 raw custody into the Social Security old-age poverty
floor work item.

Required contract:

- `record_family` is `social_security_domestic_old_age_poverty_context_bridge`.
- The bridge links the Social Security source-capture rollup, Wave D floor-value
  readiness record, and the existing Census poverty custody packet.
- `source_custody.raw_files` lists the Census workbooks used for domestic 65+
  official poverty, SPM poverty, SPM Social Security element effects, and
  official income-to-poverty ratios.
- `context_values` may carry Census domestic baseline context, but it must not
  select a Taxlane old-age poverty floor threshold or pass/fail result.
- `blocked_outputs` remain null.
- Downstream solver, rate, savings, technology-savings, department-cut, and
  balanced-budget claims remain false.
