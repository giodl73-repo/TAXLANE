# Lane floor readiness rollup schema

`lane_floor_readiness_rollup.v1.draft.json` records whether every analytical
lane has an outcome-floor definition packet and whether that coverage is enough
to support scenarios, solver inputs, savings, or rates.

Required invariants:

- `record_family` is `lane_floor_readiness_rollup`.
- `pulse` is `176`.
- The record links the target-cost contract, comparator rubric, comparison
  coverage, lane-depth tracker, and solver-readiness rollup.
- `coverage_rule.analytical_lane_count` is `15`, while
  `coverage_rule.budget_row_count` is `17`.
- Exactly fifteen `lane_rows` are present.
- Every row has a floor-definition packet path and
  `floor_definition_packet_exists: true`.
- Every threshold, baseline, policy, stress, floor-passage,
  component-policy-path, behavior/incidence/transition, and solver-readiness
  flag remains `false`.
- Blocked output values remain `null`.
- Only publication/coverage booleans may be true.
- The public warning phrases must preserve the distinction between floor
  definitions and floor passage, target costs, savings, solver input, and rates.
