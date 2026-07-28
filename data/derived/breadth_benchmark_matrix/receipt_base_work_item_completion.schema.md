# Receipt base work item completion schema

Schema for `receipt_base_work_item_completion.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`,
`receipt_base_source_work_queue_path`, `omb_receipt_category_context_path`,
`rate_publication_readiness_rollup_path`, `completed_work_items`,
`remaining_work_items`, `summary`, `source_custody_status`, `blocked_outputs`,
`public_warning_phrases`, and `claim_booleans`.

This record may mark context extraction for a receipt-base work item complete
only when every listed completion artifact exists. Expanded OMB context may
include receipt amounts, shares, detail rows, and amount/share reconciliation.
Non-OMB context may include IRS individual-income context, SSA calendar-year
OASDI taxable-payroll context, CMS Medicare HI calendar-year context, and
transportation receipt-yield context. It must not populate legal/economic
assigned bases, rates, solver inputs, public rate cards, tax proposals, savings
estimates, waste/fraud findings, technology-savings claims, or balanced-budget
claims.
