# Pulse 01

## Objective

Make response-log row validation reusable outside CLI-specific JSON checks.

## Done

- Added `PerformanceDemandResponseLogRecord` and
  `PerformanceDemandResponseLogClass` to `taxlane-core`.
- Added validation for row identity, blocked claim gates, use rule, next action,
  and response-class/evidence consistency.
- Updated CLI JSONL generation and validation to use core-derived records.
