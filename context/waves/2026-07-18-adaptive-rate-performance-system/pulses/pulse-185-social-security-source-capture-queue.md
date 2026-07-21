# Pulse 185: Social Security source capture queue

## Goal

Turn the Social Security/OASDI source-readiness gap into ordered official-source
capture work while keeping values, floors, solver inputs, rates, and savings
blocked.

## Implemented

- Added `social_security_source_capture_queue.v1.draft.json`.
- Added schema and public reader.
- Added Rust validation and focused regression test.
- Linked the artifact from breadth-matrix and reading indexes.

## Boundary

This pulse does not capture new raw bytes, select thresholds, populate floor
values, build an annual OASDI fund path, publish a taxable payroll base, score a
policy, populate solver inputs, publish rates, claim savings, or claim balanced
budget readiness.

No external request was submitted and no agency or person was contacted.
