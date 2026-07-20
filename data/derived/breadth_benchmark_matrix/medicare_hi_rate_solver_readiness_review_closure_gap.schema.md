# Medicare HI rate and solver readiness review closure gap schema

Draft schema for
`medicare_hi_rate_solver_readiness_review_closure_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-rate-solver-readiness-review-closure-gap:v1`
- `record_family = medicare_hi_rate_solver_readiness_review_closure_gap`
- `schema_version = v1.draft`
- `pulse = 155`

Required analytical fields:

- contract, closure queue, and six prerequisite bridge packet paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 7 with `completed = false`, `ready = false`, and
  `value = null`.
- six prerequisite bridge items, all not ready/null.
- six readiness-review requirements, all not ready/null.
- readiness summary showing zero ready prerequisite bridge items and closure
  review not ready.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_rate_solver_readiness_review_closure_gap_published` true.
