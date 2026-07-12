# Cost-Down Source Packet Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable source-packet row ID. |
| `record_family` | string | Always `cost_down_source_packet`. |
| `source_backlog_record_id` | string | Related `cost_down_backlog` row. |
| `source_pressure_record_id` | string | Related `efficiency_pressure` row. |
| `lane_id` | string | Program, financing, or pressure lane. |
| `packet_status` | string | Current value: `reviewed_source_packet_no_savings_estimate`. |
| `source_ids` | array[string] | Ledger-backed sources attached to the packet. |
| `evidence_summary` | array[string] | What the attached sources support. |
| `metric_candidates` | array[string] | Candidate metrics for future scoring. |
| `outcome_floor_checks` | array[string] | Conditions that must stay protected before any savings claim. |
| `missing_before_estimate` | array[string] | Evidence still needed before estimating savings. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Source packets can justify why a backlog item deserves investigation. They do
not estimate savings, prove waste, or authorize a public performance claim.
