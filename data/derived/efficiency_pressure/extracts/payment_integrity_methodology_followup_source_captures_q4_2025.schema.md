# Payment Integrity Methodology Follow-Up Source Captures Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up source-capture identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_source_capture`. |
| `source_followup_source_query_run_record_id` | string | Follow-up query-run row that produced the capture. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from the query run. |
| `observed_date` | string | Date the source was inspected. |
| `source_url` | string | Official source URL. |
| `source_title` | string | Source title. |
| `captured_source_scope` | string | Source section or scope captured. |
| `captured_boundary_summary` | string | Conservative summary of recoverability boundary evidence. |
| `recoverability_boundary_status` | string | Boundary result status. |
| `closure_effect` | string | Effect on closure-readiness. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are source captures for internal methodology review. They are not closure
decisions, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
