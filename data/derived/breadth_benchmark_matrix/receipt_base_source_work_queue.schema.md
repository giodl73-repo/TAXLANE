# Receipt base source work queue schema

Schema for `receipt_base_source_work_queue.v1.draft.json`.

Required top-level fields: `record_id`, `record_family`, `schema_version`,
`pulse`, `as_of_date`, `contract_path`,
`receipt_base_local_source_inventory_path`,
`assigned_receipt_base_source_gap_path`,
`rate_publication_readiness_rollup_path`, `source_custody_status`,
`work_queue_rows`, `summary`, `blocked_outputs`, `public_warning_phrases`, and
`claim_booleans`.

This work queue may name source-capture and extraction tasks only. It must not
populate captured sources, assigned base amounts, matched receipt bases,
elasticities, yields, assigned-base rates, public rate cards, tax proposals,
solver inputs, savings estimates, waste/fraud findings, technology-savings
claims, or balanced-budget claims.
