# Accountability Response Intake Importer Fixture

## Goal

Validate that generated response-intake rows can be applied to response-log rows
through the guarded `taxlane-core` handoff before future UI/API importers rely on
the contract.

## Scope

- Generate a response-intake example JSONL row.
- Validate the row as `PerformanceDemandResponseIntakeRecord`.
- Apply the row to the matching `PerformanceDemandResponseLogRecord`.
- Keep public-claim gates blocked after the update.

## Status

Complete.
