# Defense source capture closure work queue schema

`defense_source_capture_closure_work_queue.v1.draft.json` converts the defense
source-capture status rollup into ordered closure gates.

Required invariants:

- `record_family` is `defense_source_capture_closure_work_queue`.
- `pulse` is `189`.
- The record links the target-cost contract, defense source-capture queue,
  defense source-capture status rollup, and defense outcome-floor definition
  packet.
- Exactly six closure work items are present.
- Each closure work item has null raw artifact path, metadata path, and closure
  value fields with `ready: false`.
- Aggregate ready/value counts remain zero.
- Lineage reviews, raw custody, force-structure plans, readiness floor values,
  procurement schedules, solver inputs, rates, savings, department-cut
  instructions, technology-savings claims, and balanced-budget claims remain
  null/false.
