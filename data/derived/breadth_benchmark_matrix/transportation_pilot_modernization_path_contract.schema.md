# Transportation Pilot Modernization Path Contract Schema

Canonical artifact:
`transportation_pilot_modernization_path_contract.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-modernization-path-contract:v1`
- `record_family = transportation_pilot_modernization_path_contract`
- `version`
- `status`
- `pulse = 93`
- `selected_pilot_decision_path`
- `source_plan_path`
- `baseline_path_contract_path`
- `floor_indicator_contract_path`
- `technology_transition_operating_model_path`
- `program_lane_target_cost_contract_path`
- `deterministic_annual_update_simulator_contract_path`
- `phase_plan_path`
- `source_custody_status`
- `selected_pilot`
- `non_claim_boundary`
- `modernization_policy`
- `required_intervention_segments`
- `required_modernization_record_fields`
- `scenario_linkage`
- `blocked_gates`
- `modernization_records`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- The selected pilot must remain transportation asset maintenance and safety
  under `transportation-infrastructure`.
- Technology is a transition path, not an automatic cut.
- Productivity credit requires same service or better, floor passes, transition
  costs, implementation/admin costs, and measured net effect.
- No headcount, department-cut, savings, waste, fraud, rate, target-cost, or
  balanced-budget claim may be opened by this contract.
- Required segments are asset inventory/condition data, project delivery and
  permitting controls, predictive maintenance/operations, and safety targeting
  and network design.
- Every segment initially has `central_effect_millions = null`,
  `transition_cost_millions = null`, `net_effect_millions = null`,
  `productivity_credit_allowed = false`, and `status = planned_not_scored`.
- Future modernization rows require policy instrument, implementation/admin
  outlays, transition cost, monitoring/enforcement cost, gross and net effects,
  utilization or volume response, vendor/procurement response, workforce
  transition effect, service-level effect, floor-pass link, source ID, raw path,
  byte count, and SHA-256.
- Scenario effects remain `null`; stress must later be an adverse realization
  of the same policy, not an aggressive price reduction.
- `modernization_records` remains empty.
- Every blocked gate remains `false`.
- Every output placeholder remains `null`.
- Only `modernization_contract_published` may be true; source-custody,
  modernization-path, productivity-credit, technology-savings, simulator,
  target-cost, rate, public-card, savings, waste, fraud, department-cut, and
  balanced-budget claims remain false.
