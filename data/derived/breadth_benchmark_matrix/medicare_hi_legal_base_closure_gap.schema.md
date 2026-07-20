# Medicare HI legal-base closure gap schema

Draft schema for `medicare_hi_legal_base_closure_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-legal-base-closure-gap:v1`
- `record_family = medicare_hi_legal_base_closure_gap`
- `schema_version = v1.draft`
- `pulse = 151`

Required analytical fields:

- contract, closure queue, legal-base gap, and income-category mapping paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 3 with `completed = false`, `ready = false`, and
  `value = null`.
- five candidate terms.
- six legal-base requirements, all not ready/null.
- legal-base gap fields all null/not ready.
- context values preserving CMS taxable-payroll context as non-selected legal
  base context.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only `medicare_hi_legal_base_closure_gap_published` true.
