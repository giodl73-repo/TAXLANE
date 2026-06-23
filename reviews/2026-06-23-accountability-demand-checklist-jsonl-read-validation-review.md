# Accountability Demand Checklist JSONL Read Validation Review

## Scope

Reviewed `crates/taxlane-core/src/lib.rs`, `tools/taxlane/src/main.rs`, and the
performance demand checklist JSONL validation path.

## Findings

- `taxlane-core` now provides an owned `PerformanceDemandChecklistRecord` for
  deserializing generated JSONL rows.
- The record validates required fields, expected demand evidence, expected use
  rule, and claim-gate consistency.
- `taxlane-tools` deserializes generated JSONL into the core record type,
  compares rows against core-derived expected records, and still rejects any
  unexpected public-claim allowance in the current dataset.
- This strengthens read-side validation without changing the public claim
  boundary or adding findings.

## Decision

Accepted as the read-side guard for future UI/API consumers of the performance
demand checklist JSONL.
