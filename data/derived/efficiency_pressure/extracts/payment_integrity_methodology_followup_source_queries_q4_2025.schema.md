# Payment Integrity Methodology Follow-Up Source Queries Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up source-query identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_source_query`. |
| `source_priority_reviewer_action_record_id` | string | Reviewer action that requires the follow-up query. |
| `source_field_update_record_id` | string or null | Field-update row used by the query, if any. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from reviewer action. |
| `query_objective` | string | Narrow objective for the next source search. |
| `query_text` | string | Query text to execute. |
| `source_scope` | string | Official source scope for the query. |
| `capture_rule` | string | Required fields if a source is found. |
| `success_rule` | string | Rule for deciding whether the query resolves the blocker. |
| `query_status` | string | Must be `open_not_executed`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are follow-up query plans. They are not executed source extracts, closure
decisions, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
