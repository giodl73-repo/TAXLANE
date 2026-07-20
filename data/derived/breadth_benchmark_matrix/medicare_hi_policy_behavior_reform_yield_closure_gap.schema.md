# Medicare HI policy-behavior reform-yield closure gap schema

Draft schema for
`medicare_hi_policy_behavior_reform_yield_closure_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-policy-behavior-reform-yield-closure-gap:v1`
- `record_family = medicare_hi_policy_behavior_reform_yield_closure_gap`
- `schema_version = v1.draft`
- `pulse = 154`

Required analytical fields:

- contract, closure queue, behavior gap, and trust-fund solver-yield closure
  paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 6 with `completed = false`, `ready = false`, and
  `value = null`.
- current-law context preserving CMS/OMB values as non-policy and
  non-reform-yield context.
- nine reform-yield requirements, all not ready/null.
- blocked model fields and blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_policy_behavior_reform_yield_closure_gap_published` true.
