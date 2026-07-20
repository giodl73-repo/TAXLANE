# Medicare HI economic-base definition gap schema

Draft schema for
`medicare_hi_economic_base_definition_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-economic-base-definition-gap:v1`
- `record_family = medicare_hi_economic_base_definition_gap`
- `schema_version = v1.draft`
- `pulse = 144`

Required custody and readiness fields:

- official source / existing-custody / no-contact booleans.
- `economic_base_gap_defined = true`.
- incidence, employer burden, household burden, distribution, administration,
  economic-base completion, assigned-base, rate-publication, and solver-input
  booleans all false.

Required analytical fields:

- component id `economic_base_definition`.
- five required model components, all null/not ready.
- boundary findings preserving the diagnostic ratio as non-rate context.
- blocked outputs with null values.
- still-required list.
- public warning phrases.
- claim booleans with only `economic_base_gap_published` true.
