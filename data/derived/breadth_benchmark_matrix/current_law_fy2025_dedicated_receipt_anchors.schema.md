# Current-law FY2025 dedicated receipt anchors schema

Schema for `current_law_fy2025_dedicated_receipt_anchors.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `fiscal_year`, `year_basis`, `unit`,
`current_law_fy2025_fund_group_path`, `contract_path`, `rubric_path`,
`source_custody_status`, `source_packets`, `receipt_anchor_rows`,
`reconciliation`, `blocked_outputs`, `public_warning_phrases`, and
`claim_booleans`.

`receipt_anchor_rows` records source-labeled FY2025 dedicated-receipt anchors
from OMB Historical Table 2.4. Missing fund accounting fields remain absent from
the anchor rows and null in `blocked_outputs`; they must not be filled with zero.

Every row must include `anchor_id`, `path_id`, `source_row_ref`, `source_label`,
`amount_musd`, `fund_group`, `budget_treatment`, `legal_dedication_status`,
`value_role`, and `may_populate_solver`. Derived sums must include `formula`.

The record may publish receipt anchors only. It may not publish outlays, fund
balances, explicit transfers, solver inputs, target costs, federal effects,
savings, rates, rate cards, waste findings, fraud findings, technology-savings
claims, or balanced-budget claims.
