# Receipt-base official source capture schema

This schema describes `receipt_base_official_source_capture` records.

Required fields:

- Identity fields: `record_id`, `record_family`, `schema_version`, `pulse`, and
  `as_of_date`.
- Paths to the target-cost contract, receipt-base work queue, prior receipt-base
  completion/progress records, and rate-publication readiness rollup.
- `source_custody_status` booleans separating official public file capture from
  matched-base, rate, and solver readiness.
- `captured_source_packets` with publisher, title, URL, retrieval date, raw path,
  byte count, SHA-256, metadata path, and custody readiness.
- `blocked_source_packets` for attempted but uncustodied source families.
- `extracted_context_rows` for source-captured context values only, with year
  basis, unit, source reference, value role, and false readiness booleans.
- `reconciliation`, `summary`, `blocked_outputs`, `public_warning_phrases`, and
  `claim_booleans`.

The record may capture official public files and guarded context values. It must
not publish matched assigned receipt bases, statutory rates, effective rates,
solver inputs, public rate cards, tax proposals, savings estimates, waste/fraud
findings, technology-savings claims, or balanced-budget claims.
