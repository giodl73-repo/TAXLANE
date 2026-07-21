# Income-security/family source readiness gap schema

`income_security_family_source_readiness_gap.v1.draft.json` records that
income-security/family source families are identified but not yet supported by
local raw custody for benefit-package, take-up, child-poverty, material-hardship,
childcare-access, work-transition, federal/state/local translation, or solver
use.

Required invariants:

- `record_family` is `income_security_family_source_readiness_gap`.
- `pulse` is `190`.
- The record links the target-cost contract, income-security/family outcome
  floor definition packet, lane floor source work queue, and lane-depth tracker.
- Exactly six required source families are present.
- Raw custody, floor values, benefit-package models, take-up models, federal
  translation, pass/fail findings, solver inputs, rates, savings,
  department-cut instructions, technology-savings claims, and balanced-budget
  claims remain null/false.
- Only publication and existing floor-definition-presence booleans may be true.
