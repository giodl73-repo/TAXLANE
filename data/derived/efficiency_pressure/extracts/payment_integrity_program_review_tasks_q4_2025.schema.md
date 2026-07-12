# Payment Integrity Program Review Tasks Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable task identifier. |
| `record_family` | string | Must be `payment_integrity_program_review_task`. |
| `source_program_gate_record_id` | string | Program gate that created the task. |
| `source_scorecard_record_id` | string | PaymentAccuracy scorecard probe row. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `evidence_family` | string | One of `methodology`, `access_floor`, `corrective_action`, or `confidence_limits`. |
| `extraction_task` | string | Specific extraction task. |
| `target_source_or_system` | string | Source family or system to inspect. |
| `completion_gate` | string | Must be `required_before_savings_score`. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are extraction tasks. They are not savings estimates, waste findings,
performance findings, fraud findings, or public claims.
