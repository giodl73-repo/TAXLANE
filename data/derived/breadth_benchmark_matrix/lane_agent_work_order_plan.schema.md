# Lane agent work-order plan schema

Schema for `lane_agent_work_order_plan.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `lane-agent-work-order-plan:v1`.
- `record_family`: must equal `lane_agent_work_order_plan`.
- `version`
- `status`
- `pulse`: must equal `111`.
- `lane_depth_explainability_tracker_path`
- `program_lane_target_cost_contract_path`
- `international_comparator_target_rubric_path`
- `purpose`
- `parallelization_rules`
- `standard_agent_work_order`
- `assignment_waves`
- `integration_review_checklist`
- `claim_booleans`
- `plain_english_status`

Every assignment wave must declare:

- `wave_id`
- `max_parallel_agents`
- `lane_ids`
- `integration_required`
- `rationale`

This plan is orchestration-only. It does not execute agents, produce lane
outputs, publish target costs, calculate rates, claim savings, identify waste or
fraud, direct department cuts, claim technology savings, or claim a balanced
budget.
