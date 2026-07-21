# Health Medicare Trustees source capture status schema

`health_medicare_trustees_source_capture_status.v1.draft.json` records local
CMS Medicare Trustees source custody for health/Medicare context.

Required invariants:

- `record_family` is `health_medicare_trustees_source_capture_status`.
- `pulse` is `179`.
- The record links the health floor source-capture status, lane floor source
  work queue, receipt-base source capture, Medicare part-financing rows, and
  Medicare denominator rows.
- The CMS Trustees raw artifact is custody-ready only for CY2025 Medicare
  financing and enrollment context.
- Medicare part-financing context has three records.
- Medicare enrollment denominator context has five records.
- NHE, CBO baseline, quality/access indicator, and threshold-review source gaps
  remain not custody-ready.
- Every health floor threshold/value remains `null` and every pass flag remains
  `false`.
- Blocked outputs remain `null`.
- Only publication, CMS Trustees custody, and financing/enrollment-context
  booleans may be true.
