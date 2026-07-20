# Medicare HI behavior and reform-yield gap schema

Draft schema for
`medicare_hi_behavior_reform_yield_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-behavior-reform-yield-gap:v1`
- `record_family = medicare_hi_behavior_reform_yield_gap`
- `schema_version = v1.draft`
- `pulse = 146`

Required custody and readiness fields:

- official source / existing-custody / no-contact booleans.
- `behavior_reform_yield_gap_defined = true`.
- policy instrument, elasticity, avoidance, compliance, administration,
  incidence/distribution, trust-fund solver mapping, reform-yield,
  rate-publication, and solver-input booleans all false.

Required analytical fields:

- component id `behavior_and_reform_yield`.
- current-law context preserving CMS payroll taxes, CMS total HI revenue, OMB
  HI receipt anchor, diagnostic difference, and diagnostic ratio as non-reform
  context.
- seven reform-yield requirements, all null/not ready.
- blocked outputs with null values.
- still-required list.
- public warning phrases.
- claim booleans with only `behavior_reform_yield_gap_published` true.
