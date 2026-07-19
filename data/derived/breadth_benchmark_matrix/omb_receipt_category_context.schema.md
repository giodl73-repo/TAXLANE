# OMB receipt category context schema

Schema for `omb_receipt_category_context.fy2025.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `fiscal_year`, `year_basis`, `unit`, `contract_path`,
`receipt_base_source_work_queue_path`,
`receipt_base_local_source_inventory_path`, `source_custody`, `extraction`,
`receipt_category_rows`, `reconciliation_checks`, `blocked_outputs`,
`public_warning_phrases`, and `claim_booleans`.

This record may publish OMB fiscal receipt-category context only. It must not
populate legal or economic assigned receipt bases, rates, public rate cards, tax
proposals, solver inputs, savings estimates, waste/fraud findings,
technology-savings claims, or balanced-budget claims.
