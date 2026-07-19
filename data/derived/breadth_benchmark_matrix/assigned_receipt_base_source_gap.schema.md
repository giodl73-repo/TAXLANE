# Assigned receipt base source gap schema

Schema for `assigned_receipt_base_source_gap.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`, `assigned_receipt_base_inventory_path`,
`legacy_illustrative_rate_path`, `source_custody_status`,
`available_source_packets`, `blocked_base_rows`, `legacy_quarantine`,
`required_before_rates`, `blocked_outputs`, `public_warning_phrases`, and
`claim_booleans`.

This record may publish source-custody status and gaps only. It may not populate
assigned base amounts, elasticities, incidence, distribution, administration,
yields, statutory rates, effective rates, solver inputs, public rate cards, tax
proposals, savings estimates, waste/fraud findings, technology-savings claims,
or balanced-budget claims.
