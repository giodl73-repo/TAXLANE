# Lane depth explainability tracker schema

Schema for `lane_depth_explainability_tracker.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `lane-depth-explainability-tracker:v1`.
- `record_family`: must equal `lane_depth_explainability_tracker`.
- `version`
- `status`
- `pulse`: must equal `110`.
- `program_lane_target_cost_contract_path`
- `international_comparator_target_rubric_path`
- `solver_input_readiness_rollup_path`
- `current_law_source_custody_preflight_path`
- `purpose`
- `completion_definition`
- `aggregate_status`
- `lane_rows`
- `public_questions_required_for_each_lane`
- `claim_booleans`
- `plain_english_status`

Each `lane_rows` entry must include:

- `lane_id`
- `public_label`
- `depth_artifact_paths`
- `current_law_baseline_status`
- `source_custody_status`
- `policy_scenario_status`
- `outcome_floor_status`
- `modernization_transition_status`
- `public_explainer_status`
- `solver_mapping_status`
- `lane_depth_complete`
- `public_explainability_complete`
- `next_work`

All rows remain incomplete in this draft. This tracker does not publish rates,
target costs, savings, waste findings, fraud findings, technology savings, or a
balanced-budget claim.
