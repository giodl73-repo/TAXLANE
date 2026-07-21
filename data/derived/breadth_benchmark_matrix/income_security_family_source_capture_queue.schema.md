# Income-security/family source capture queue schema

`income_security_family_source_capture_queue.v1.draft.json` converts the
income-security/family source-readiness gap into ordered official-source capture
work.

Required invariants:

- `record_family` is `income_security_family_source_capture_queue`.
- `pulse` is `191`.
- The record links the target-cost contract, income-security/family
  source-readiness gap, income-security/family outcome-floor definition packet,
  and lane floor source work queue.
- Exactly six capture items are present.
- Each capture item has null raw artifact path, byte count, SHA-256, metadata,
  and value fields with `ready: false`.
- Aggregate ready/value counts remain zero.
- Benefit-package models, take-up models, floor values, federal translation,
  solver inputs, rates, savings, department-cut instructions,
  technology-savings claims, and balanced-budget claims remain null/false.
