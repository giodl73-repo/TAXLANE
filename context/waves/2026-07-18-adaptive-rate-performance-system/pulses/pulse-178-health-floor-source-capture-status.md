# Pulse 178: Health floor source capture status

## Goal

Start source capture for the first prioritized lane, health/Medicare, while
preserving the boundary between fiscal source custody and outcome-floor passage.

## Implemented

- Added `health_floor_source_capture_status.v1.draft.json`.
- Added a schema and public reader.
- Added Rust validation and a focused regression test.
- Linked the artifact from the breadth-matrix and reading indexes.

## Result

Existing OMB FY2025 fiscal custody is recognized for Medicare, non-Medicare
health, and the Medicare HI receipt anchor. Health floor indicator sources,
thresholds, observed values, pass/fail findings, policy scores, solver inputs,
rates, savings, and balanced-budget claims remain blocked.

No external request was submitted and no agency or person was contacted.
