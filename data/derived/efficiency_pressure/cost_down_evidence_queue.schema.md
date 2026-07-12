# Cost-Down Evidence Queue Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable evidence-queue row ID. |
| `record_family` | string | Always `cost_down_evidence_queue`. |
| `source_packet_record_id` | string | Related `cost_down_source_packet` row. |
| `source_backlog_record_id` | string | Related `cost_down_backlog` row. |
| `source_pressure_record_id` | string | Related `efficiency_pressure` row. |
| `lane_id` | string | Program, financing, or pressure lane. |
| `extraction_priority` | string | `first_pass`, `follow_up`, or `blocked`. |
| `primary_source_ids` | array[string] | Ledger-backed sources to query first. |
| `extract_question` | string | The immediate extraction question. |
| `first_extract` | string | First concrete source pull or source inventory. |
| `extract_grain` | string | Intended row grain for the future extract. |
| `query_lock_fields` | array[string] | Fields needed to make a dynamic source reproducible. |
| `output_artifact_candidate` | string | Candidate path for the future extracted artifact. |
| `scoring_blockers` | array[string] | Missing elements before savings or performance scoring. |
| `outcome_floor` | string | Floor that must remain protected. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Evidence queue rows are work-order records. They identify which source data to
lock next; they do not estimate savings, prove waste, or authorize public
performance claims.
