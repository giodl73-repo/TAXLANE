# Income-security/family source capture status rollup schema

`income_security_family_source_capture_status_rollup.v1.draft.json` summarizes
the post-Pulse 191 income-security/family source-capture state.

Required invariants:

- `record_family` is `income_security_family_source_capture_status_rollup`.
- `pulse` is `192`.
- The record links the target-cost contract, income-security/family
  source-readiness gap, income-security/family source-capture queue, and
  income-security/family outcome-floor definition packet.
- Exactly six source families are summarized.
- All six source families remain open capture items.
- Raw custody, context readiness, program outlay perimeter, benefit-package
  models, take-up models, floor values, federal translation, solver inputs,
  rates, savings, department-cut instructions, technology-savings claims, and
  balanced-budget claims remain null/false.
- Only publication booleans for the rollup and its prerequisite records may be
  true.
