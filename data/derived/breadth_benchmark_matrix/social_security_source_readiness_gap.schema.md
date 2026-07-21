# Social Security source readiness gap schema

`social_security_source_readiness_gap.v1.draft.json` records the current
Social Security/OASDI source-readiness state.

Required invariants:

- `record_family` is `social_security_source_readiness_gap`.
- `pulse` is `184`.
- The record links the target-cost contract, Social Security outcome-floor
  definition packet, floor source work queue, lane-depth tracker, receipt-base
  source-capture packet, and SSA-derived denominator values.
- The four CY2025 derived denominator context rows are present.
- SSA raw custody, 75-year solvency path, annual fund path, taxable payroll
  base, adequacy floor values, poverty floor values, solver inputs, rates, and
  savings remain null/false.
- Only publication and derived-denominator-context booleans may be true.
