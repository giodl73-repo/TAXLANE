# Current-law source-custody batch plan schema

Schema for `current_law_source_custody_batch_plan.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `current-law-source-custody-batch-plan:v1`.
- `record_family`: must equal `current_law_source_custody_batch_plan`.
- `version`
- `status`
- `pulse`: must equal `119`.
- Links to the post-rollup readiness queue, current-law source-custody
  preflight, and current-law path inventory.
- `batch_rules`
- `custody_batches`
- `covered_path_ids`
- `aggregate_status`
- `claim_booleans`
- `plain_english_status`

Every batch must keep `source_ids`, `raw_artifact_paths`, `metadata_paths`, and
`values` null. `custody_ready` and `values_may_be_populated` must remain false.
The plan may order future source-custody work but may not capture source values,
run the solver, publish rates, or make public fiscal claims.
