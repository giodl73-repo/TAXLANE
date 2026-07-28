# Health source readiness rollup schema

`health_source_readiness_rollup.v1.draft.json` summarizes the post-Pulse-182
health/Medicare source-custody state.

Required invariants:

- `record_family` is `health_source_readiness_rollup`.
- `pulse` is `183`.
- The record links the target-cost contract, health floor source-capture status,
  Medicare Trustees source-capture status, NHE source-custody gap, CBO
  source-custody gap, and quality/access indicator source gap.
- Exactly five source families are summarized.
- Exactly three source families may be custody-ready, and all are context-only.
- One source family may be partial custody context-only: CMS quality/access.
- CBO remains a custody gap, while remaining CMS quality/access lineage remains
  incomplete for floor passage.
- Floor passage, federal policy translation, solver inputs, savings, and rates
  remain null/false.
- Only publication, context-readiness, and partial quality/access custody
  booleans may be true.
