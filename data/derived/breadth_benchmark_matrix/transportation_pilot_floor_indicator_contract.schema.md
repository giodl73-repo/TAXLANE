# Transportation Pilot Floor Indicator Contract Schema

Canonical artifact:
`transportation_pilot_floor_indicator_contract.v1.draft.json`.

Required top-level fields:

- `record_id = transportation-pilot-floor-indicator-contract:v1`
- `record_family = transportation_pilot_floor_indicator_contract`
- `version`
- `status`
- `pulse = 92`
- `selected_pilot_decision_path`
- `source_plan_path`
- `baseline_path_contract_path`
- `transportation_depth_card_path`
- `program_lane_target_cost_contract_path`
- `international_comparator_target_rubric_path`
- `deterministic_annual_update_simulator_contract_path`
- `phase_plan_path`
- `source_custody_status`
- `selected_pilot`
- `non_claim_boundary`
- `floor_policy`
- `required_floor_families`
- `indicator_record_requirements`
- `blocked_gates`
- `indicator_records`
- `blocking_conditions`
- `output_placeholders`
- `claim_booleans`

Validation rules:

- The selected pilot must remain transportation asset maintenance and safety
  under `transportation-infrastructure`.
- The non-claim boundary must state that the artifact is not a threshold
  decision, floor pass finding, simulator run, target-cost selection, rate
  calculation, savings estimate, waste finding, fraud finding, technology
  savings claim, solver result, or balanced-budget claim.
- All lower-cost scenarios must be blocked unless required floors pass.
- No thresholds are set in this contract.
- No floor pass findings are recorded in this contract.
- Missing values remain `null`; blocked gates remain `false`.
- International differences are not savings and cannot imply fraud.
- Required floor families are access/coverage, quality/safety,
  equity/distribution, adequacy/resilience, delivery feasibility, and the
  transportation-specific asset-condition floor.
- Each required floor family initially has `threshold_value = null`,
  `observed_value = null`, `passed = false`, and
  `status = planned_not_thresholded`.
- Future indicator records require source custody, period, unit, perimeter,
  observed value, threshold value, comparison direction, pass flag, missingness
  reason, and federal/state/local translation status.
- `indicator_records` remains empty until official source custody and threshold
  decisions are complete.
- Every blocked gate remains `false`.
- Every output placeholder remains `null`.
- Only `floor_indicator_contract_published` may be true; source-custody,
  indicator-record, threshold, floor-pass, simulator, target-cost, rate,
  public-card, savings, waste, fraud, department-cut, technology-savings, and
  balanced-budget claims remain false.
