# Medicare HI solver-yield mapping gap schema

Draft schema for
`medicare_hi_solver_yield_mapping_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-solver-yield-mapping-gap:v1`
- `record_family = medicare_hi_solver_yield_mapping_gap`
- `schema_version = v1.draft`
- `pulse = 145`

Required custody and readiness fields:

- official source / existing-custody / no-contact booleans.
- `solver_yield_mapping_gap_defined = true`.
- OMB receipt-row mapping, trust-fund accounting, fund-balance path, transfer
  schedule, current-law yield matching, solver-row, rate-publication, and
  solver-input booleans all false.

Required analytical fields:

- component id `solver_yield_mapping`.
- current-law context preserving CMS payroll taxes, CMS total HI revenue, OMB
  HI receipt anchor, diagnostic difference, and diagnostic ratio as non-rate
  context.
- six mapping requirements, all null/not ready.
- blocked outputs with null values.
- still-required list.
- public warning phrases.
- claim booleans with only `solver_yield_mapping_gap_published` true.
