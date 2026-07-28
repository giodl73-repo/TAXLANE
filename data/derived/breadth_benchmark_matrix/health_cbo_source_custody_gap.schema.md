# Health CBO source custody gap schema

`health_cbo_source_custody_gap.v1.draft.json` records that CBO source IDs
appear in derived health context artifacts, official CBO browser access has
been documented, the February 2026 Table 2 browser rowmap has been assigned as
context only, and February 2026 CBO health-baseline PDF/spreadsheet raw custody
is captured through manual browser download.

Required invariants:

- `record_family` is `health_cbo_source_custody_gap`.
- `pulse` is `181`.
- The record links the health floor source-capture status, Medicare Trustees
  source-capture status, NHE source-custody gap, CBO Table 2 browser rowmap,
  and lane floor source work queue.
- The referenced CBO source IDs are listed.
- CBO official access boundaries, February 2026 health baseline raw custody, and
  browser rowmap context are present, while incomplete CBO source-custody
  readiness remains blocked for downstream use.
- CBO references may not populate federal policy translation, behavior,
  incidence, pass/fail findings, solver inputs, savings, or rates.
- Publication, "source referenced," and access-boundary booleans may be true.
