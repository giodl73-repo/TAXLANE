# Health quality/access indicator source gap schema

`health_quality_access_indicator_source_gap.v1.draft.json` records that
health/Medicare quality, access, risk-adjusted outcome, rural-capacity, and
safety-net-capacity indicator families are needed but are not yet supported by
local raw source custody.

Required invariants:

- `record_family` is `health_quality_access_indicator_source_gap`.
- `pulse` is `182`.
- The record links the health floor source-capture status, Medicare Trustees
  source-capture status, NHE source-custody gap, CBO source-custody gap, and
  health outcome-floor definition packet.
- Quality/access source family, metadata, SHA-256, and source-custody readiness
  remain false/null.
- Floor thresholds, observed values, policy values, stress values, pass/fail
  findings, solver inputs, savings, and rates remain null/false.
- Only publication and "source family declared" booleans may be true.
