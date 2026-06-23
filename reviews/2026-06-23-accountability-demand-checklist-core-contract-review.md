# Accountability Demand Checklist Core Contract Review

## Scope

Reviewed `crates/taxlane-core/src/lib.rs`, `tools/taxlane/src/main.rs`, and the
generated performance demand checklist JSONL path.

## Findings

- `taxlane-core` now exposes a typed `PerformanceDemandChecklistRow` contract.
- The CLI serializes checklist JSONL rows from the core contract instead of
  constructing ad hoc JSON.
- The core unit test verifies the missing-performance-evidence row remains
  blocked from public claims and carries the shared demand evidence/use rule.
- The change does not alter generated JSONL content or relax public-claim
  guardrails.

## Decision

Accepted as the reusable Rust contract for future UI/API performance-demand
checklist surfaces.
