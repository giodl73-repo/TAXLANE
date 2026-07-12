# Payment Integrity Methodology Narrow Closure Decisions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable narrow closure-decision identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_narrow_closure_decision`. |
| `source_narrow_closure_candidate_record_id` | string | Narrow closure-candidate row supporting this decision. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from candidate. |
| `narrow_decision_status` | string | Must be `component_closed_internal_only`. |
| `closed_component` | string | Component closed internally. |
| `decision_basis` | string | Source-backed basis for the narrow decision. |
| `excluded_scope` | string | Scope explicitly not closed. |
| `residual_open_need` | string | Remaining source or reviewer need. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are narrow internal component decisions. They are not full field closure
decisions, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
