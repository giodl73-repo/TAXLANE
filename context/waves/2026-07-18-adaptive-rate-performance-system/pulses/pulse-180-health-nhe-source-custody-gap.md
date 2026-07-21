# Pulse 180: Health NHE source custody gap

## Goal

Prevent derived health sensitivity references to CMS NHE from being treated as
local raw source custody or health floor passage.

## Implemented

- Added `health_nhe_source_custody_gap.v1.draft.json`.
- Added schema and public reader.
- Added Rust validation and focused regression test.
- Linked the artifact from breadth-matrix and reading indexes.

## Boundary

This pulse does not capture new NHE raw bytes, select thresholds, populate floor
values, make pass/fail findings, score policy, populate solver inputs, publish
rates, claim savings, or claim balanced-budget readiness.

No external request was submitted and no agency or person was contacted.
