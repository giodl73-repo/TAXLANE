# Health CBO source custody gap schema

`health_cbo_source_custody_gap.v1.draft.json` records that CBO source IDs
appear in derived health context artifacts but are not yet supported by local
raw CBO health-baseline custody.

Required invariants:

- `record_family` is `health_cbo_source_custody_gap`.
- `pulse` is `181`.
- The record links the health floor source-capture status, Medicare Trustees
  source-capture status, NHE source-custody gap, and lane floor source work
  queue.
- The referenced CBO source IDs are listed.
- CBO raw artifact, metadata, SHA-256, and source-custody readiness remain
  false/null.
- CBO references may not populate federal policy translation, behavior,
  incidence, pass/fail findings, solver inputs, savings, or rates.
- Only publication and "source referenced" booleans may be true.
