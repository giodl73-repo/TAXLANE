# Accountability Performance Demand Response Status Core Contract Review

## Scope

Reviewed the Rust core response status summary and CLI integration for
generated status JSON.

## Findings

- `taxlane-core` now exposes `PerformanceDemandResponseStatus` as the typed
  status summary for response-log counts.
- Core aggregation validates typed response-log rows before counting totals,
  not-yet-received rows, allowed public claims, and blocked public claims.
- Status validation rejects empty summaries, inconsistent claim counts, and
  unexpected use-rule text.
- CLI status JSON generation now serializes the core type instead of counting
  generic JSON values.

## Decision

Accepted as the reusable response status contract. It keeps UI/API counts tied
to typed response-log rows and preserves blocked claim gates.
