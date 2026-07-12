# Payment Integrity Methodology Field Updates Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology field-update identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_field_update`. |
| `source_priority_reviewer_action_record_id` | string | Reviewer action authorizing the field update. |
| `source_methodology_field_record_id` | string | Original methodology-field row being repaired. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `old_methodology_field` | string | Original field name. |
| `revised_methodology_field` | string | Revised field name. |
| `old_required_source_target` | string | Original source target. |
| `revised_required_source_target` | string | Revised source target. |
| `old_completion_rule` | string | Original completion rule. |
| `revised_completion_rule` | string | Revised completion rule. |
| `update_status` | string | Must be `field_reframed_internal_only`. |
| `update_scope` | string | Narrow scope of the field update. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal methodology field updates. They are not closure decisions,
scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
