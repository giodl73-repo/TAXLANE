# Payment Integrity Methodology Result Review Readiness Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology result-review-readiness identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_result_review_readiness`. |
| `source_methodology_result_record_ids` | array[string] | Captured methodology-result rows ready for field review. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_capture_count` | integer | Count of captured result rows. |
| `review_readiness_status` | string | Must be `ready_for_field_review_queue`. |
| `next_field_review_count` | integer | Number of field-review rows to create next. |
| `next_methodology_fields` | array[string] | Methodology fields queued for review. |
| `next_action` | string | Next source-work action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal readiness markers for field review. They are not savings
estimates, waste findings, performance findings, fraud findings, recoverable
amount claims, or public claims.
