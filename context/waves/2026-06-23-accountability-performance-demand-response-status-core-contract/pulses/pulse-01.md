# Pulse 01

## Objective

Make response status counts reusable outside CLI-specific JSON aggregation.

## Done

- Added `PerformanceDemandResponseStatus` to `taxlane-core`.
- Added aggregation and validation for total, not-yet-received, allowed, and
  blocked counts from typed response-log records.
- Updated CLI status JSON generation and validation to use the core type.
