# Transportation Pilot Stress Path Contract Schema

Canonical artifact:
`transportation_pilot_stress_path_contract.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-stress-path-contract:v1`
- `record_family = transportation_pilot_stress_path_contract`
- `version`
- `status`
- `pulse = 94`
- governing artifact paths
- `source_custody_status`
- `selected_pilot`
- `non_claim_boundary`
- `stress_policy`
- `required_stress_dimensions`
- `required_stress_record_fields`
- `scenario_values`
- `blocked_gates`
- `stress_records`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Stress must be an adverse realization of the same policy, not an aggressive
  cut.
- Required stress dimensions are weaker modernization effect, higher
  implementation cost, higher utilization or volume, access/quality/equity
  remediation, weaker receipt or fund-balance context, and higher interest
  context.
- All stress values remain `null` and all dimensions remain
  `planned_not_scored`.
- Future stress rows require same-policy linkage, central value, stress value,
  delta value, implementation/admin outlays, access remediation outlays,
  floor-impact link, fund-balance context, source ID, raw path, byte count, and
  SHA-256.
- Scenario values remain `null`.
- `stress_records` remains empty.
- Every blocked gate remains `false`.
- Every output placeholder remains `null`.
- Only `stress_contract_published` may be true; source-custody, stress-path,
  simulator, target-cost, rate, public-card, savings, waste, fraud,
  department-cut, technology-savings, and balanced-budget claims remain false.
