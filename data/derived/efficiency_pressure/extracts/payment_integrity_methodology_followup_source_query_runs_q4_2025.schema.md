# Payment Integrity Methodology Follow-Up Source Query Runs Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up source-query-run identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_source_query_run`. |
| `source_followup_source_query_record_id` | string | Follow-up source-query row being prepared for execution. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from source query. |
| `run_status` | string | Must be `pending_not_run`. |
| `planned_query_text` | string | Query text to run. |
| `result_capture_status` | string | Must be `no_result_captured`. |
| `required_capture_fields` | array[string] | Fields required for a valid follow-up source capture. |
| `next_run_rule` | string | Rule for executing the query and creating a capture. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are pending follow-up query-run scaffolds. They are not executed source
extracts, closure decisions, scoring gates, savings estimates, waste findings,
fraud findings, recoverable-dollar claims, or public claims.
