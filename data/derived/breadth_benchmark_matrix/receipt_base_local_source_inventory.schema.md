# Receipt base local source inventory schema

Schema for `receipt_base_local_source_inventory.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`,
`assigned_receipt_base_source_gap_path`,
`rate_publication_readiness_rollup_path`, `source_custody_status`,
`local_source_rows`, `blocked_base_rows`, `blocked_outputs`,
`public_warning_phrases`, and `claim_booleans`.

This inventory may describe local source custody and source gaps only. It must
not populate assigned base amounts, matched receipt bases, elasticities, yields,
assigned-base rates, public rate cards, tax proposals, solver inputs, savings
estimates, waste/fraud findings, technology-savings claims, or balanced-budget
claims.
