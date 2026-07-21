# Defense source capture queue schema

`defense_source_capture_queue.v1.draft.json` converts the defense
source-readiness gap into ordered official-source capture work.

Required invariants:

- `record_family` is `defense_source_capture_queue`.
- `pulse` is `187`.
- The record links the target-cost contract, defense source-readiness gap,
  defense outcome-floor definition packet, and lane floor source work queue.
- Exactly six capture items are present.
- Each capture item has null raw artifact path, byte count, SHA-256, metadata,
  and value fields with `ready: false`.
- Aggregate ready/value counts remain zero.
- Force-structure plans, readiness floor values, procurement schedules, solver
  inputs, rates, savings, department-cut instructions, technology-savings
  claims, and balanced-budget claims remain null/false.
