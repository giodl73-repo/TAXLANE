# Payment Integrity Methodology Follow-Up Boundary Readiness Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable follow-up boundary-readiness identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_followup_boundary_readiness`. |
| `source_followup_boundary_decision_record_id` | string | Boundary-decision row being triaged. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from boundary decision. |
| `boundary_readiness_status` | string | `narrow_internal_readiness_candidate` or `additional_positive_basis_needed`. |
| `readiness_scope` | string | Narrow scope of any internal readiness. |
| `readiness_reason` | string | Why this status was assigned. |
| `next_required_action` | string | Next reviewer or source action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal boundary-readiness triage. They are not closure decisions,
scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
