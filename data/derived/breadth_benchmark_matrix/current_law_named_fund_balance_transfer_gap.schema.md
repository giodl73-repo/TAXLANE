# Current-law named fund balance and transfer gap schema

Schema for `current_law_named_fund_balance_transfer_gap.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `year_basis`, `baseline_fiscal_year`, `contract_path`,
`rubric_path`, `current_law_fy2025_fund_group_path`,
`current_law_fy2025_dedicated_receipt_anchors_path`,
`current_law_fy2025_named_trust_fund_outlay_anchors_path`,
`source_custody_status`, `available_anchor_evidence`,
`missing_required_sources`, `blocked_formula`, `blocked_outputs`,
`public_warning_phrases`, and `claim_booleans`.

This record is a gap-control packet. It may reference existing receipt, outlay,
and fund-group anchors, but all fund-balance paths, explicit transfers,
credited offsetting collections by named fund, solver inputs, rates, savings,
target costs, waste/fraud findings, technology-savings claims, and
balanced-budget claims must remain null or false.
