# Payment Integrity Methodology Component Gate Source Queries Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate source-query identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_source_query`. |
| `source_component_gate_source_target_record_id` | string | Source target that created this query. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the source target. |
| `query_text` | string | Search query to run against official sources. |
| `query_scope` | string | What the query is intended to resolve. |
| `expected_evidence` | array | Evidence expected from a useful result. |
| `insufficient_result_rule` | string | Rule for keeping closure/scoring blocked. |
| `next_artifact_family` | string | Must be `payment_integrity_methodology_component_gate_source_query_run`. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source queries. They are not source captures, field closures,
scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
