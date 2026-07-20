# Medicare HI bridge status rollup schema

Draft schema for `medicare_hi_bridge_status_rollup.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-bridge-status-rollup:v1`
- `record_family = medicare_hi_bridge_status_rollup`
- `schema_version = v1.draft`
- `pulse = 147`

Required analytical fields:

- six component record paths for the Medicare HI bridge.
- custody booleans showing existing official-source custody and no external
  contact.
- six component status rows, each with `ready = false` and `value = null`.
- summary counts: 6 components, 0 ready, 6 blocked, 2 partial-context rows, and
  4 gap rows.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only `medicare_hi_bridge_rollup_published` true.
