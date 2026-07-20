# Medicare HI income-category OMB mapping gap schema

Draft schema for
`medicare_hi_income_category_omb_mapping_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-income-category-omb-mapping-gap:v1`
- `record_family = medicare_hi_income_category_omb_mapping_gap`
- `schema_version = v1.draft`
- `pulse = 150`

Required analytical fields:

- contract, closure queue, CMS income-split, and OMB/CMS perimeter-evidence
  paths.
- custody booleans showing existing official-source custody and no external
  contact or new download.
- work-queue item 2 with `completed = false`, `ready = false`, and
  `value = null`.
- CMS income-category context and three category groups.
- six OMB mapping requirements, all not ready/null.
- recomputed CMS income-split formulas.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `medicare_hi_income_category_omb_mapping_gap_published` true.
