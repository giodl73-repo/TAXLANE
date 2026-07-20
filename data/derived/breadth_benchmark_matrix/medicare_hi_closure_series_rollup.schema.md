# Medicare HI closure series rollup schema

Draft schema for `medicare_hi_closure_series_rollup.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-closure-series-rollup:v1`
- `record_family = medicare_hi_closure_series_rollup`
- `schema_version = v1.draft`
- `pulse = 156`

Required analytical fields:

- contract path, bridge closure queue path, and seven closure packet paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- seven closure packet status rows with `packet_published = true`,
  `completed = false`, `ready = false`, and `value = null`.
- series summary showing seven packets published, zero completed items, zero
  ready items, and zero ready bridge components.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only publication/status context true and all rate, solver,
  savings, waste/fraud, technology, and balanced-budget claims false.
