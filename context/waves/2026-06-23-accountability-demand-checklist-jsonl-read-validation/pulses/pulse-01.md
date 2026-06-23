# Pulse 01

## Objective

Harden performance demand checklist JSONL validation before it becomes a UI/API
handoff.

## Done

- Added `PerformanceDemandChecklistRecord` to `taxlane-core`.
- Added record-level validation for demand evidence, use rule, and claim gate.
- Updated `taxlane-tools` to deserialize generated JSONL into the core record
  type and compare rows to core-derived expected records.
- Added review and VTRACE rows for `WP-TAX-022`.
