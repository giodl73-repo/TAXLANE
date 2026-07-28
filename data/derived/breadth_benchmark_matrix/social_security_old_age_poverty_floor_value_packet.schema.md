# Social Security old-age poverty floor value packet schema

`social_security_old_age_poverty_floor_value_packet.v1.draft.json` records the
first Wave D floor-value conversion for the Social Security lane.

Required contract:

- `record_family` is `social_security_old_age_poverty_floor_value_packet`.
- The packet links the Social Security floor definition, domestic Census
  old-age poverty context bridge, OECD old-age poverty context bridge, and Wave
  D readiness artifact.
- `threshold_rationale` selects the draft measure, rationale, threshold rule,
  threshold value, unit, source table, and review status.
- `baseline_values` carries source-custodied baseline values.
- `policy_values`, `stress_values`, and `pass_fail_evidence` remain null until
  reviewed policy and stress scenarios exist.
- Downstream lower-cost, solver, rate, savings, technology-savings,
  department-cut, and balanced-budget claims remain false.
