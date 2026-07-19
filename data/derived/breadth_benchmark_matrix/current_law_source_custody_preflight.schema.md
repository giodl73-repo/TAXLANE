# Current-law source-custody preflight schema

Schema for `current_law_source_custody_preflight.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `current-law-source-custody-preflight:v1`.
- `record_family`: must equal `current_law_source_custody_preflight`.
- `version`: draft record version.
- `status`: draft preflight status.
- `pulse`: must equal `109`.
- `current_law_path_inventory_path`: path to the current-law path inventory.
- `solver_input_readiness_rollup_path`: path to the solver readiness rollup.
- `program_lane_target_cost_contract_path`: path to the governing target-cost contract.
- `purpose`: explanation of the record boundary.
- `custody_packet_requirements`: exact list of fields required before values may be populated.
- `preflight_rows`: one row for each required current-law path.
- `preflight_rules`: booleans enforcing no external request, no captured values, and source-custody prerequisites.
- `claim_booleans`: public-claim flags.
- `non_claim_boundary`: public boundary text.

Each `preflight_rows` entry must include:

- `path_id`
- `required`
- `candidate_official_source_family`
- `source_id`
- `official_host_or_publisher`
- `source_vintage`
- `retrieval_date`
- `raw_artifact_path`
- `raw_byte_count`
- `raw_sha256`
- `metadata_path`
- `extraction_method`
- `annual_years_covered`
- `component_mapping`
- `review_status`
- `custody_ready`
- `values_may_be_populated`
- `remaining_blockers`

Until a custody packet exists, all custody fields remain `null`,
`custody_ready` is `false`, and `values_may_be_populated` is `false`.
