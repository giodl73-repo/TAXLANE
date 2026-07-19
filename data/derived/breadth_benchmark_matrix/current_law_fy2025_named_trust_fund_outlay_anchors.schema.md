# Current-law FY2025 named trust-fund outlay anchors schema

Schema for
`current_law_fy2025_named_trust_fund_outlay_anchors.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `fiscal_year`, `year_basis`, `unit`,
`input_unit_from_source`, `conversion_formula`,
`current_law_fy2025_dedicated_receipt_anchors_path`,
`current_law_fy2025_fund_group_path`, `contract_path`, `rubric_path`,
`source_custody_status`, `source_packet`, `outlay_anchor_rows`,
`reconciliation`, `blocked_outputs`, `public_warning_phrases`, and
`claim_booleans`.

`outlay_anchor_rows` records source-labeled FY2025 OMB Public Budget Database
outlay anchors converted from thousands of dollars to millions of dollars.
Rows may include OASI, DI, OASDI-sum, and Medicare HI anchors. Missing fund
balances, transfer schedules, credited offsetting collections, and complete
transportation trust-fund path values must remain null in `blocked_outputs`.

The record may publish outlay anchors only. It may not publish complete
trust-fund paths, solver inputs, target costs, federal effects, savings, rates,
rate cards, waste findings, fraud findings, technology-savings claims, or
balanced-budget claims.
