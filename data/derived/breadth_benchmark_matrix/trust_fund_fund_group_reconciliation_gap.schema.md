# Trust-fund and fund-group reconciliation gap schema

Draft schema for `trust_fund_fund_group_reconciliation_gap.v1.draft.json`.

Required identity fields:

- `record_id = trust-fund-fund-group-reconciliation-gap:v1`
- `record_family = trust_fund_fund_group_reconciliation_gap`
- `schema_version = v1.draft`
- `pulse = 159`

Required analytical fields:

- contract, post-Medicare-HI queue, source-custody gap, FY2025 fund-group
  path, and named-fund balance/transfer gap paths.
- custody booleans showing existing official-source context and no external
  contact or new download.
- rank-2 work item with `completed = false`, `ready = false`, and
  `value = null`.
- available FY2025 aggregate fund-group context with substitution flags false.
- eight reconciliation requirements and four fund path rows, all not ready/null.
- blocked fund-balance formula and blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `trust_fund_fund_group_reconciliation_gap_published` true.
