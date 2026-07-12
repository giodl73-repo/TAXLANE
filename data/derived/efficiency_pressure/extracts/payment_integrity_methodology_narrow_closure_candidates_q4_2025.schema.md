# Payment Integrity Methodology Narrow Closure Candidates Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable narrow closure-candidate identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_narrow_closure_candidate`. |
| `source_followup_boundary_readiness_record_id` | string | Boundary-readiness row supporting this candidate. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from readiness row. |
| `candidate_scope` | string | Narrow scope eligible for internal reviewer decision. |
| `candidate_basis` | string | Source-backed basis for the candidate. |
| `excluded_scoring_basis` | string | Explicitly excluded scoring or savings interpretation. |
| `next_required_action` | string | Next reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal narrow closure candidates. They are not closure decisions,
scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
