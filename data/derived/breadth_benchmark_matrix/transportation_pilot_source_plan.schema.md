# Transportation Pilot Source Plan Schema

Canonical artifact:
`transportation_pilot_source_plan.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-source-plan:v1`
- `record_family = transportation_pilot_source_plan`
- `version`
- `status`
- `pulse = 90`
- `selected_pilot_decision_path`
- `transportation_depth_card_path`
- `program_lane_target_cost_contract_path`
- `deterministic_annual_update_simulator_contract_path`
- `technology_transition_operating_model_path`
- `phase_plan_path`
- `source_custody_status`
- `selected_pilot`
- `non_claim_boundary`
- `custody_requirements`
- `source_families`
- `matching_rules`
- `floor_indicator_families_planned`
- `planned_downstream_contracts`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- Source custody status must say no new external request and no source bytes
  captured.
- The selected pilot must remain transportation asset maintenance and safety
  under `transportation-infrastructure`.
- Custody requirements must require retrieval date, URL, publisher, vintage,
  raw bytes, byte count, SHA-256, local raw path, metadata, matched period/unit/
  perimeter, and missingness disclosure.
- Source families must include OMB, Treasury, DOT, FHWA, NHTSA/BTS, Census,
  GAO/OIG, and ITF/OECD families.
- Matching rules must keep trust funds separate, require explicit general-fund
  transfers and credited offsets, keep state/local/private/user-financed
  activity contextual until translated, and prohibit treating international
  differences as savings.
- Planned floor families remain `planned_not_thresholded`, with `value = null`
  and `passed = false`.
- Planned downstream contract paths remain `null`.
- Every output placeholder remains `null`.
- Only `source_plan_published` may be true; all source-custody, baseline, floor,
  modernization, stress, simulator, target-cost, rate, card, savings, waste,
  fraud, department-cut, technology-savings, and balanced-budget claims remain
  false.
