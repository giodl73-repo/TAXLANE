# Medicare HI economic-base closure gap schema

Draft schema for `medicare_hi_economic_base_closure_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-economic-base-closure-gap:v1`
- `record_family = medicare_hi_economic_base_closure_gap`
- `schema_version = v1.draft`
- `pulse = 152`

Required analytical fields:

- contract, closure queue, economic-base gap, and legal-base closure paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 4 with `completed = false`, `ready = false`, and
  `value = null`.
- seven economic-base requirements, all not ready/null.
- six model component gaps, all not ready/null.
- context values preserving CMS taxable-payroll context as non-economic-base
  context.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only `medicare_hi_economic_base_closure_gap_published`
  true.
