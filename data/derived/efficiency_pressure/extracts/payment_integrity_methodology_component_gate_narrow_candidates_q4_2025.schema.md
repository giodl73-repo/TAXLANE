# Payment Integrity Methodology Component Gate Narrow Candidates Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate narrow-candidate identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_narrow_candidate`. |
| `source_component_gate_boundary_readiness_record_id` | string | Component gate boundary-readiness row being promoted. |
| `agency_code` | string | Agency code inherited from the readiness row. |
| `program_or_activity` | string | Program or activity name inherited from the readiness row. |
| `source_target_priority` | integer | Priority inherited from the readiness row. |
| `candidate_status` | string | Must be `narrow_component_candidate_internal_only`. |
| `candidate_scope` | string | Narrow process component that may move to an internal decision. |
| `candidate_basis` | string | Positive process-boundary basis. |
| `excluded_scoring_basis` | string | Explicit reason scoring and public claims remain blocked. |
| `next_required_action` | string | Next reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal narrow component candidates. They are not field closures,
scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
