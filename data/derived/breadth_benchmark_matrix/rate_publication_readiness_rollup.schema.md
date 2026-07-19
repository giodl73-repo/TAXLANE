# Rate publication readiness rollup schema

Schema for `rate_publication_readiness_rollup.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`, `public_rate_card_v2_contract_path`,
`assigned_receipt_base_source_gap_path`,
`distribution_incidence_source_gap_path`,
`administration_compliance_burden_source_gap_path`, `source_custody_status`,
`readiness_rows`, `summary`, `blocked_outputs`, `public_warning_phrases`, and
`claim_booleans`.

This rollup may summarize blocker status only. It must not populate statutory
rates, effective rates, assigned-base rates, public rate cards, tax proposals,
solver inputs, savings estimates, waste/fraud findings, technology-savings
claims, or balanced-budget claims.
