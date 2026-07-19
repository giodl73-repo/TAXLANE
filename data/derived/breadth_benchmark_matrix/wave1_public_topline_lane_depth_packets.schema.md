# Wave 1 public-topline lane-depth packets schema

Schema for `wave1_public_topline_lane_depth_packets.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `wave1-public-topline-lane-depth-packets:v1`.
- `record_family`: must equal `wave1_public_topline_lane_depth_packets`.
- `version`
- `status`
- `pulse`: must equal `112`.
- `lane_agent_work_order_plan_path`
- `lane_depth_explainability_tracker_path`
- `program_lane_target_cost_contract_path`
- `purpose`
- `wave`
- `lane_packets`
- `integration_review`
- `claim_booleans`
- `plain_english_status`

Each lane packet must include:

- `lane_id`
- `public_label`
- `what_it_does`
- `what_taxpayers_pay_now`
- `who_is_served_or_protected`
- `outcomes_that_matter`
- `overspending_underfunding_boundary`
- `technology_transition_boundary`
- `evidence_now`
- `blocked_evidence`
- `claim_booleans`

This draft keeps all missing values null and all blocked public claims false.
