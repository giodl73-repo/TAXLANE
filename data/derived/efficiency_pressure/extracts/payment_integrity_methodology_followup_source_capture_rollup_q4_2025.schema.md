# Payment Integrity Methodology Follow-Up Source Capture Rollup Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up source-capture rollup identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_source_capture_rollup`. |
| `source_followup_source_capture_record_id` | string | Follow-up source-capture row being rolled up. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from source capture. |
| `capture_rollup_status` | string | `reviewer_boundary_decision_needed` or `additional_positive_basis_needed`. |
| `boundary_finding` | string | Conservative boundary finding from the capture. |
| `remaining_review_need` | string | Remaining question before any field closure. |
| `reviewer_action` | string | Next reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal rollups for follow-up source captures. They are not closure
decisions, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
