# Wave lane-depth scaffold rollup schema

Schema for `wave_lane_depth_scaffold_rollup.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `wave-lane-depth-scaffold-rollup:v1`.
- `record_family`: must equal `wave_lane_depth_scaffold_rollup`.
- `version`
- `status`
- `pulse`: must equal `117`.
- `lane_agent_work_order_plan_path`
- `lane_depth_explainability_tracker_path`
- `solver_input_readiness_rollup_path`
- `balanced_rate_readiness_gate_path`
- `public_rate_card_v2_contract_path`
- `purpose`
- `wave_packet_paths`
- `coverage_summary`
- `covered_ids_by_wave`
- `all_analytical_lane_ids`
- `remaining_completion_gates`
- `highest_priority_next_work`
- `integration_review`
- `claim_booleans`
- `plain_english_status`

The rollup may say all 15 analytical lanes have scaffold packets. It must not
say lane depth is complete, public explainability is complete, rates are ready,
solver outputs are ready, savings are scored, waste or fraud is found, cuts are
instructed, technology savings are claimed, or the budget is balanced.
