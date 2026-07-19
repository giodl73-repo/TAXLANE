# Distribution and incidence source gap schema

Schema for `distribution_incidence_source_gap.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `distributional_effect_placeholder_path`,
`assigned_receipt_base_source_gap_path`, `assigned_receipt_base_inventory_path`,
`contract_path`, `source_custody_status`, `required_source_families`,
`blocked_outputs`, `gate_rules`, `public_warning_phrases`, and
`claim_booleans`.

This record may publish source gaps and gate rules only. It must not populate
distributional analysis, incidence analysis, macro feedback, interaction
scoring, solver inputs, rates, public rate cards, tax proposals, savings
estimates, waste/fraud findings, technology-savings claims, or balanced-budget
claims.
