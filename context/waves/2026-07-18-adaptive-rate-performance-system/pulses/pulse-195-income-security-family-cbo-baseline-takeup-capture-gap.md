# Pulse 195: Income-security/family CBO baseline/take-up capture gap

## Goal

Test whether the next income-security/family CBO baseline/take-up source gate can
be closed from the current environment without weakening source custody.

## Implemented

- Added `income_security_family_cbo_baseline_takeup_capture_gap.v1.draft.json`.
- Added schema and public reader.
- Recorded the blocked automated CBO SNAP PDF capture attempts.
- Recorded that the official CBO open-data catalog was reachable but did not
  expose the selected-program SNAP baseline PDF as a machine-readable CSV.
- Added Rust validation and a focused regression test.
- Linked the gap from breadth-matrix and reading indexes.

## Boundary

This pulse does not capture raw CBO source custody, populate CBO baseline values,
populate take-up context, complete CBO baseline/take-up lineage, model a benefit
package, publish a take-up model, set floor values, complete federal/state/local
translation, populate solver inputs, publish rates, claim savings, or claim
balanced-budget readiness.

No external request was submitted and no agency or person was contacted.
