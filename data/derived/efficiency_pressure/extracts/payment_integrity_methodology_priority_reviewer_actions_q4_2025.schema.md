# Payment Integrity Methodology Priority Reviewer Actions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology priority reviewer-action identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_priority_reviewer_action`. |
| `source_priority_source_work_record_id` | string | Priority source-work row reviewed. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from source work. |
| `selected_methodology_field` | string | Methodology field copied from source work. |
| `reviewer_action_status` | string | `field_reframing_approved_internal_only` or `additional_source_work_required`. |
| `reviewer_action` | string | Internal reviewer action to take. |
| `field_reframing_allowed` | boolean | Whether the methodology field definition can be repaired internally. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `next_required_artifact` | string | Artifact family needed next. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal reviewer actions. They are not closure decisions, scoring
gates, savings estimates, waste findings, fraud findings, recoverable-dollar
claims, or public claims.
