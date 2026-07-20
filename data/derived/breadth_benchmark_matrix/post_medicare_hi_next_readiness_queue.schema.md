# Post-Medicare HI next-readiness queue schema

Draft schema for `post_medicare_hi_next_readiness_queue.v1.draft.json`.

Required identity fields:

- `record_id = post-medicare-hi-next-readiness-queue:v1`
- `record_family = post_medicare_hi_next_readiness_queue`
- `schema_version = v1.draft`
- `pulse = 157`

Required analytical fields:

- contract, post-rollup queue, and Medicare HI closure-series rollup paths.
- sequence rules preserving source custody, fund reconciliation, outcome floors,
  receipt-base modeling, payment-integrity lineage, net-interest feedback, null
  missingness, and false blocked gates.
- eight work-queue rows, all `ready = false` and `value = null`.
- aggregate status showing zero ready items and no solver/rate/public-claim
  readiness.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only
  `post_medicare_hi_next_readiness_queue_published` true.
