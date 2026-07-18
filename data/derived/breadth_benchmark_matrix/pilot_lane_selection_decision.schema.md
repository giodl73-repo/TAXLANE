# Pilot Lane Selection Decision Schema

Canonical artifact:
`pilot_lane_selection_decision.v1.draft.json`.

Required top-level fields:

- `record_id = pilot-lane-selection-decision:v1`
- `record_family = pilot_lane_selection_decision`
- `version`
- `status`
- `pulse = 89`
- `selection_gate_path`
- `deterministic_annual_update_simulator_contract_path`
- `public_thesis_packet_path`
- `adaptive_rate_system_contract_path`
- `technology_transition_operating_model_path`
- `role_review_path`
- `phase_plan_path`
- `source_custody_status`
- `decision_boundary`
- `selected_pilot`
- `criteria_results`
- `non_selected_candidates`
- `excluded_first_pilots_preserved`
- `role_review_summary`
- `next_required_artifacts`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody must show no new external request.
- The selected candidate must be
  `transportation_asset_maintenance_and_safety`.
- The selected lane must be `transportation-infrastructure`.
- Selection status must be `selected_for_scaffold_only`.
- `simulator_ready` and `public_claim_allowed` remain `false`.
- Criteria results must cover all seven pilot-selection criteria.
- The excluded first-pilot list must preserve Social Security, Medicare, broad
  health, veterans, and immediate normative distribution choices.
- Role review must cover all eight roles.
- Next required artifacts remain required with `path = null`.
- Every output placeholder remains `null`.
- The only true claim boolean is `pilot_selected_for_scaffold`; every public,
  rate, savings, waste, fraud, department-cut, technology-savings, simulator,
  and balanced-budget claim remains `false`.
