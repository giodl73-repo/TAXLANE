# Income-security/family source capture status rollup schema

`income_security_family_source_capture_status_rollup.v1.draft.json` summarizes
the post-Pulse 200 income-security/family source-capture state.

Required invariants:

- `record_family` is `income_security_family_source_capture_status_rollup`.
- `pulse` is `201`.
- The record links the target-cost contract, income-security/family
  source-readiness gap, income-security/family source-capture queue, post-queue
  bridge/gap artifacts, and income-security/family outcome-floor definition
  packet.
- Exactly six source families are summarized.
- Two families may show narrow source custody or context readiness: FY2025
  federal account-perimeter source custody, and OECD SOCX family-benefit
  comparator context.
- Four families remain documented capture gaps: CBO baseline/take-up, Census
  child poverty/income, HHS/ACF childcare/family services, and USDA
  food/nutrition.
- Full source capture, program outlay perimeter model, benefit-package models,
  take-up models, floor values, federal/state/local translation, solver inputs,
  rates, savings, department-cut instructions, technology-savings claims, and
  balanced-budget claims remain null/false.
- Publication booleans for the rollup/prerequisite records and the two narrow
  ready contexts may be true; downstream modeling and claim booleans must
  remain false.
