# Post-rollup readiness work queue schema

Schema for `post_rollup_readiness_work_queue.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `post-rollup-readiness-work-queue:v1`.
- `record_family`: must equal `post_rollup_readiness_work_queue`.
- `version`
- `status`
- `pulse`: must equal `118`.
- Links to the scaffold rollup, current-law custody preflight, current-law path
  inventory, lane full coverage matrix, solver-input readiness rollup, and
  balanced-rate readiness gate.
- `sequence_rules`
- `work_queue`
- `aggregate_status`
- `claim_booleans`
- `plain_english_status`

The work queue must keep every work item `ready: false` with `value: null`.
Items may show partial progress evidence, but partial progress is not
completion. The queue may order next work, but it must not populate current-law
values, solver inputs, policy scores, target costs, rates, savings, waste/fraud
findings, department-cut instructions, technology-savings claims, or
balanced-budget claims.
