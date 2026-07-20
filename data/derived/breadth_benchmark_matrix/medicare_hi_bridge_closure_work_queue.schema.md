# Medicare HI bridge closure work queue schema

Draft schema for
`medicare_hi_bridge_closure_work_queue.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-bridge-closure-work-queue:v1`
- `record_family = medicare_hi_bridge_closure_work_queue`
- `schema_version = v1.draft`
- `pulse = 148`

Required analytical fields:

- contract and Medicare HI bridge status rollup paths.
- sequence rules preserving source custody, trust-fund separation, null values,
  and false readiness gates.
- seven work-queue rows ordered from OMB/CMS perimeter bridge through final
  readiness review.
- every work item must have `value = null` and `ready = false`.
- aggregate status with 7 work items, 0 ready items, 6 required bridge
  components, and 0 ready bridge components.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_bridge_closure_work_queue_published` true.
