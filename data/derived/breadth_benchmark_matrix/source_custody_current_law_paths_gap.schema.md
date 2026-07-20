# Source custody and current-law paths gap schema

Draft schema for `source_custody_current_law_paths_gap.v1.draft.json`.

Required identity fields:

- `record_id = source-custody-current-law-paths-gap:v1`
- `record_family = source_custody_current_law_paths_gap`
- `schema_version = v1.draft`
- `pulse = 158`

Required analytical fields:

- contract, post-Medicare-HI queue, source-custody preflight, and current-law
  path inventory paths.
- custody booleans showing existing official-source custody context and no
  external contact or new download.
- rank-1 work item with `completed = false`, `ready = false`, and
  `value = null`.
- required custody fields.
- eight current-law path status rows, all not custody-ready, not value-ready,
  not path-ready, and null-valued.
- 2025 through 2035 horizon requirement with no interpolation.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `source_custody_current_law_paths_gap_published` true.
