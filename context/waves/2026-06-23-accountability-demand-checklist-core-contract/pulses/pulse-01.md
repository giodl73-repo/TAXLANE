# Pulse 01

## Objective

Move demand checklist JSONL row construction from the CLI into the reusable
Rust core crate.

## Done

- Added `PerformanceDemandChecklistRow`, shared demand evidence, and use rule
  constants to `taxlane-core`.
- Switched `taxlane-tools` JSONL generation to
  `record.performance_demand_checklist_row()`.
- Added core unit coverage for the blocked missing-evidence checklist row.
- Added review and VTRACE rows for `WP-TAX-021`.
