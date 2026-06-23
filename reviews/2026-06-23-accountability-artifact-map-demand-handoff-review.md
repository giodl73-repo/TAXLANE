# Accountability Artifact Map Demand Handoff Review

## Scope

Reviewed `data/derived/accountability_evidence/artifact-map.md` and the Rust
validation hook for demand artifact discoverability.

## Findings

- The artifact map now routes citizen readers to the performance demand
  checklist and dashboard.
- The artifact map now routes product implementers to the demand checklist
  JSONL, claim-gates JSON, and schema note.
- Each row carries an avoid rule that preserves blocked-claim and no-finding
  boundaries.
- Rust validation requires the artifact map to include the dashboard, claim
  gates, JSONL, and schema entries.

## Decision

Accepted as the current demand-artifact navigation handoff. It improves
discoverability without changing evidence records or public-claim eligibility.
