# Cost-Down First-Pass Rollup Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable rollup row ID. |
| `record_family` | string | Always `cost_down_first_pass_rollup`. |
| `source_evidence_queue_record_id` | string | Evidence queue row summarized. |
| `source_backlog_record_id` | string | Backlog row summarized. |
| `source_pressure_record_id` | string | Pressure row summarized. |
| `lane_id` | string | Lane or pressure surface. |
| `lever_id` | string | Cost-down lever. |
| `first_pass_artifacts` | array | First-pass extract artifacts used. |
| `first_pass_row_count` | integer | Total extract rows represented by the rollup row. |
| `signal_status` | string | First-pass status. |
| `strongest_current_signal` | string | Highest-signal source-backed observation now available. |
| `scoring_blockers` | array | Missing items before scoring or public claims. |
| `next_scoring_step` | string | Next extract or method step. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Rollup rows are implementation status records. They summarize evidence readiness
and blockers, not savings estimates, waste findings, performance findings, or
legal allocation claims.
