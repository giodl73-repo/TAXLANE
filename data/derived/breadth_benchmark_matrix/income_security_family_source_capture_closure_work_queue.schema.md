# Income-security/family source capture closure work queue schema

`income_security_family_source_capture_closure_work_queue.v1.draft.json`
converts the income-security/family source-capture status rollup into ordered
closure gates.

Required invariants:

- `record_family` is `income_security_family_source_capture_closure_work_queue`.
- `pulse` is `193`.
- The record links the target-cost contract, income-security/family
  source-capture queue, source-capture status rollup, and outcome-floor
  definition packet.
- Exactly six closure work items are present.
- Each closure work item has null raw artifact path, metadata path, and closure
  value fields with `ready: false`.
- Aggregate ready/value counts remain zero.
- Lineage reviews, raw custody, program perimeters, benefit-package models,
  take-up models, floor values, federal translation, solver inputs, rates,
  savings, department-cut instructions, technology-savings claims, and
  balanced-budget claims remain null/false.
