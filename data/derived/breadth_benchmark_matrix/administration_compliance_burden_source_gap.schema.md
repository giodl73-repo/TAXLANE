# Administration and compliance burden source gap schema

Schema for `administration_compliance_burden_source_gap.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`,
`distribution_incidence_source_gap_path`,
`assigned_receipt_base_source_gap_path`, `public_rate_card_v2_contract_path`,
`rate_adjustment_operating_model_path`, `source_custody_status`,
`required_source_families`, `blocked_outputs`, `gate_rules`,
`public_warning_phrases`, and `claim_booleans`.

This record may publish source gaps and gate rules only. It must not populate
agency burden, taxpayer burden, employer burden, avoidance, compliance,
transition cost, technology-productivity, solver inputs, rates, public rate
cards, tax proposals, savings estimates, waste/fraud findings, or
balanced-budget claims.
