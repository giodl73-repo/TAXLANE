# Transportation receipt-base work-item progress schema

This schema describes `transportation_receipt_base_work_item_progress` records.

Required fields:

- `record_id`, `record_family`, `schema_version`, `pulse`, and `as_of_date`.
- Artifact paths for the target-cost contract, receipt-base source work queue,
  prior work-item completion record, FY2025 dedicated-receipt anchors, and rate
  publication readiness rollup.
- `work_item_id` fixed to `capture-transportation-excise-user-fee-base`.
- `source_custody_status` booleans separating receipt-yield context from legal
  and economic assigned-base readiness.
- `progress_rows` with source row references, FY2025 receipt-yield context,
  null legal/economic bases, null assigned-base rates, and false readiness
  booleans.
- `reconciliation` recomputing the transportation and airport-and-airway receipt
  yield context sum.
- `remaining_transportation_fields`, `summary`, `blocked_outputs`,
  `public_warning_phrases`, and `claim_booleans`.

The record may publish source-custodied receipt-yield context only. It must not
publish legal receipt bases, economic receipt bases, assigned-base rates, solver
inputs, public rate cards, tax proposals, savings estimates, waste/fraud
findings, technology-savings claims, or balanced-budget claims.
