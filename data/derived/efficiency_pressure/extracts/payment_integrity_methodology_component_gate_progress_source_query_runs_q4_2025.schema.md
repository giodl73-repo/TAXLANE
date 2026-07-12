# Payment Integrity Methodology Component Gate Progress Source Query Runs Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate progress source-query-run identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_progress_source_query_run`. |
| `source_component_gate_progress_source_query_record_id` | string | Source query to run. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the source query. |
| `run_status` | string | Must be `pending_not_run`. |
| `planned_query_text` | string | Query text to run. |
| `result_capture_status` | string | Must be `no_result_captured`. |
| `required_capture_fields` | array | Fields required before any source capture. |
| `next_run_rule` | string | Rule for creating a source capture. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal query-run queue records. They are not source captures, field
closures, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
