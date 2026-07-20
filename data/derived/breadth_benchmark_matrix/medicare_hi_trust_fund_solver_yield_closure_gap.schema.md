# Medicare HI trust-fund solver-yield closure gap schema

Draft schema for
`medicare_hi_trust_fund_solver_yield_closure_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-trust-fund-solver-yield-closure-gap:v1`
- `record_family = medicare_hi_trust_fund_solver_yield_closure_gap`
- `schema_version = v1.draft`
- `pulse = 153`

Required analytical fields:

- contract, closure queue, solver-yield gap, and economic-base closure paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 5 with `completed = false`, `ready = false`, and
  `value = null`.
- current-law context preserving CMS/OMB values as non-solver-yield context.
- eight solver-yield requirements, all not ready/null.
- blocked solver fields and blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_trust_fund_solver_yield_closure_gap_published` true.
