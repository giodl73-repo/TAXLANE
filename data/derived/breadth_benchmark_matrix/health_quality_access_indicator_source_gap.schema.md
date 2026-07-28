# Health quality/access indicator source gap schema

`health_quality_access_indicator_source_gap.v1.draft.json` records partial local
raw custody for CMS Provider Data Catalog hospital quality/access context,
captures CMS Hospital Data Dictionary lineage context, and keeps remaining
measure-lineage, threshold, pass/fail, solver, savings, and rate uses blocked.

Required invariants:

- `record_family` is `health_quality_access_indicator_source_gap`.
- `pulse` is `182`.
- The record links the health floor source-capture status, Medicare Trustees
  source-capture status, NHE source-custody gap, CBO source-custody gap, and
  health outcome-floor definition packet.
- Six CMS Provider Data Catalog datasets have local CSV and metadata custody.
- The CMS Hospital Data Dictionary PDF has local byte custody and may only be
  used as lineage context.
- Quality/access complete source-custody readiness remains false until
  dataset-specific denominator crosswalk, risk-adjustment methodology,
  case-mix, rural-capacity, safety-net-capacity, and review lineage are
  complete.
- Floor thresholds, observed values, policy values, stress values, pass/fail
  findings, solver inputs, savings, and rates remain null/false.
- Only publication, "source family declared", partial raw/metadata custody, and
  lineage-context captured booleans may be true.
