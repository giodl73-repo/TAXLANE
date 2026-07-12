# Payment Integrity Methodology Queries Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable query identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_query`. |
| `source_methodology_target_record_id` | string | Source-target row being queried. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `query_text` | string | Query text to execute. |
| `query_scope` | string | Source scope for the query. |
| `capture_rule` | string | Required capture fields if a source is found. |
| `query_status` | string | Must be `open_not_executed` until run. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are query plans. They are not executed source extracts, savings estimates,
waste findings, performance findings, fraud findings, or public claims.
