# Wave 4 component-and-pilot lane-depth packets schema

Schema for `wave4_component_and_pilot_lane_depth_packets.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `wave4-component-and-pilot-lane-depth-packets:v1`.
- `record_family`: must equal `wave4_component_and_pilot_lane_depth_packets`.
- `version`
- `status`
- `pulse`: must equal `115`.
- `lane_agent_work_order_plan_path`
- `lane_depth_explainability_tracker_path`
- `program_lane_target_cost_contract_path`
- `purpose`
- `wave`
- `lane_packets`
- `integration_review`
- `claim_booleans`
- `plain_english_status`

Each lane packet must include the public explanation fields, current pay-now
status with a null value, blocker list, and claim booleans. This draft keeps
all missing values null and all blocked public claims false.
