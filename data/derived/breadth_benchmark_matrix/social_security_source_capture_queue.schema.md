# Social Security source capture queue schema

`social_security_source_capture_queue.v1.draft.json` converts the Social
Security/OASDI source-readiness gap into ordered official-source capture work.

Required invariants:

- `record_family` is `social_security_source_capture_queue`.
- `pulse` is `185`.
- The record links the target-cost contract, Social Security source-readiness
  gap, Social Security outcome-floor definition packet, and lane floor source
  work queue.
- Exactly six capture items are present.
- Each capture item has null raw artifact path, byte count, SHA-256, metadata,
  and value fields with `ready: false`.
- Aggregate ready/value counts remain zero.
- Solver inputs, rates, savings, target costs, and balanced-budget claims remain
  null/false.
