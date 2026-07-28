# Health NHE source custody gap schema

`health_nhe_source_custody_gap.v1.draft.json` records that CMS NHE source IDs
are referenced by derived health sensitivity artifacts and now have local raw
source custody, while all floor and solver uses remain blocked.

Required invariants:

- `record_family` is `health_nhe_source_custody_gap`.
- `pulse` is `180`.
- The record links the health floor source-capture status, Medicare Trustees
  source-capture status, PHI sensitivity, and service price-volume bridge.
- The referenced CMS NHE source IDs are listed.
- NHE raw artifact, metadata, SHA-256, and source-custody readiness are present.
- Derived health sensitivity artifacts may not populate thresholds, observed
  floor values, pass/fail findings, solver inputs, savings, or rates.
- Publication, “source referenced,” and raw-custody booleans may be true.
