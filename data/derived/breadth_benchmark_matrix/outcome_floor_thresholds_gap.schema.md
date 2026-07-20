# Outcome-floor thresholds gap schema

Draft schema for `outcome_floor_thresholds_gap.v1.draft.json`.

Required fields:

- identity fields with `record_id = outcome-floor-thresholds-gap:v1`,
  `record_family = outcome_floor_thresholds_gap`, and `pulse = 160`.
- contract, post-Medicare-HI queue, trust-fund reconciliation gap, and lane
  scaffold rollup paths.
- source-custody booleans showing no external contact or new download.
- rank-3 work item with `completed = false`, `ready = false`, and null value.
- five mandatory floor classes.
- six floor requirements, all not ready/null.
- fifteen lane rows, all threshold/pass-fail not ready and null-valued.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only `outcome_floor_thresholds_gap_published` true.
