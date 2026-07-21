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
- Exactly two source families may be custody-ready, and both are context-only.
- NHE, CBO, and quality/access source families remain custody gaps.
- Floor passage, federal policy translation, solver inputs, savings, and rates
  remain null/false.
- Only publication and context-readiness booleans may be true.
