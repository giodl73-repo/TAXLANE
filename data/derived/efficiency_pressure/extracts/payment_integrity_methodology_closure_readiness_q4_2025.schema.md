# Payment Integrity Methodology Closure Readiness Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology closure-readiness identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_closure_readiness`. |
| `source_methodology_source_capture_rollup_record_id` | string | Source-capture rollup row being assessed. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Methodology field under review. |
| `closure_readiness_status` | string | `closure_review_candidate` or `additional_source_needed`. |
| `readiness_reason` | string | Why this status was assigned. |
| `next_required_action` | string | Next reviewer or source-work action. |
| `field_closure_allowed` | boolean | Must remain `false`; readiness rows do not close fields. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal closure-readiness triage. They are not closure decisions,
savings estimates, waste findings, performance findings, fraud findings, or
public claims.
