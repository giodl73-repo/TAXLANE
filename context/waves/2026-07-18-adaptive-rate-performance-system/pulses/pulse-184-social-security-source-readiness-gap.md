# Pulse 184: Social Security source readiness gap

## Goal

Make the Social Security/OASDI source-readiness boundary explicit using existing
derived CY2025 denominator context while keeping raw custody, annual fund paths,
solvency paths, floors, solver inputs, rates, and savings blocked.

## Implemented

- Added `social_security_source_readiness_gap.v1.draft.json`.
- Added schema and public reader.
- Added Rust validation and focused regression test.
- Linked the artifact from breadth-matrix and reading indexes.

## Boundary

This pulse does not capture new SSA raw bytes, select thresholds, populate floor
values, build an annual OASDI fund path, publish a taxable payroll base, score a
policy, populate solver inputs, publish rates, claim savings, or claim balanced
budget readiness.

No external request was submitted and no agency or person was contacted.
