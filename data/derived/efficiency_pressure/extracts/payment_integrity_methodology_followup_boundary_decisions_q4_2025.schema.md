# Payment Integrity Methodology Follow-Up Boundary Decisions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up boundary-decision identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_boundary_decision`. |
| `source_followup_source_capture_rollup_record_id` | string | Follow-up source-capture rollup row supporting the decision. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from rollup. |
| `boundary_decision_status` | string | Internal boundary-decision status. |
| `boundary_decision` | string | Narrow internal boundary decision. |
| `scoring_implication` | string | Why scoring remains blocked. |
| `next_required_action` | string | Next source or reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal boundary decisions. They are not closure decisions, scoring
gates, savings estimates, waste findings, fraud findings, recoverable-dollar
claims, or public claims.
