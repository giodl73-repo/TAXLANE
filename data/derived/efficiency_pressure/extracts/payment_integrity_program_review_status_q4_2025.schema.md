# Payment Integrity Program Review Status Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable status identifier. |
| `record_family` | string | Must be `payment_integrity_program_review_status`. |
| `source_program_gate_record_id` | string | Program gate summarized by the status row. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `total_required_task_count` | integer | Required task count; currently `4`. |
| `completed_task_count` | integer | Completed task count. |
| `blocked_task_count` | integer | Blocked task count. |
| `blocker_summary` | string | Short summary of remaining blockers. |
| `next_priority_task_family` | string | Next task family to extract first. |
| `next_priority_reason` | string | Why that task family comes first. |
| `review_status` | string | Must be `blocked_before_savings_score` for draft rows. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal review-status summaries. They are not savings estimates, waste
findings, performance findings, fraud findings, or public claims.
