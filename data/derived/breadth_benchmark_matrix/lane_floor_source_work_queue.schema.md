# Lane floor source work queue schema

`lane_floor_source_work_queue.v1.draft.json` converts lane floor-definition
coverage into a source-capture work queue.

Required invariants:

- `record_family` is `lane_floor_source_work_queue`.
- `pulse` is `177`.
- The record links the target-cost contract, comparator rubric, lane-depth
  tracker, and lane-floor readiness rollup.
- Source rules require official sources only, no external contact, no records
  request, and no threshold selection.
- Exactly fifteen work items are present.
- Each work item has floor dimensions, official source families, and a next
  capture action.
- `threshold_value`, `baseline_value`, `policy_value`, and `stress_value`
  remain `null`.
- `pass_fail` and `solver_ready` remain `false` for every work item.
- Blocked outputs remain `null`.
- Only the publication boolean may be true.
