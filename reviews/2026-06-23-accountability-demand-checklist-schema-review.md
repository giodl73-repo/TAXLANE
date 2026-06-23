# Accountability Demand Checklist Schema Review

## Scope

Reviewed `data/derived/accountability_evidence/performance-demand-checklist.schema.md`,
the accountability evidence README link, and manifest coverage.

## Findings

- The schema documents every generated JSONL field and its required status.
- The schema points implementers to the Rust contract
  `taxlane_core::PerformanceDemandChecklistRecord`.
- The public-use rule preserves the boundary against fraud, waste, abuse,
  legal-dedication, and performance findings.
- Rust validation now requires README discoverability for the schema alongside
  the JSONL artifact.

## Decision

Accepted as the downstream UI/API schema note for demand checklist rows.
