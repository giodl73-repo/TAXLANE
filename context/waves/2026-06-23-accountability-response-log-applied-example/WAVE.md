# Accountability Response Log Applied Example

## Goal

Generate typed response-log example rows after applying the response-intake
fixture, so future importer consumers can inspect the guarded output shape.

## Scope

- Generate `performance-demand-response-log.applied-example.jsonl`.
- Apply intake fixture rows through `PerformanceDemandResponseLogRecord::apply_intake`.
- Validate every applied response-log row through `taxlane-core`.
- Keep all public-claim gates blocked.

## Status

Complete.
