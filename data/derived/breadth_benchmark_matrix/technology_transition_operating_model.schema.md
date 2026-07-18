# Technology Transition Operating Model Schema

Canonical artifact:
`technology_transition_operating_model.v1.draft.json`.

Required top-level fields:

- `record_id = technology-transition-operating-model:v1`
- `record_family = technology_transition_operating_model`
- `version`
- `status`
- `pulse = 84`
- `adaptive_rate_system_contract_path`
- `overspending_risk_taxonomy_path`
- `phase_plan_path`
- `source_custody_status`
- `non_claim_boundary`
- `required_scenario_fields`
- `phase_definitions`
- `outcome_floor_families`
- `lower_cost_gate`
- `rate_treatment`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- Required scenario fields must include implementation cost, training/change
  management cost, cybersecurity, privacy, fallback/resilience, service risk,
  annual phase-in, measured productivity, and stress case.
- Phase definitions must include `baseline`, `transition`,
  `measured_productivity`, and `stress`.
- All outcome-floor rows start `status = missing`, `passed = false`, and
  `value = null`.
- The lower-cost gate must be closed.
- Every output placeholder remains `null`.
- Every claim boolean remains `false`.
- The reader must preserve the public warning that technology is a transition
  path, not an automatic cut.
