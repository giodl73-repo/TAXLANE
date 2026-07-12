# Payment Integrity Methodology Component Gate Narrow Decisions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate narrow-decision identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_narrow_decision`. |
| `source_component_gate_narrow_candidate_record_id` | string | Component gate narrow-candidate row supporting this decision. |
| `agency_code` | string | Agency code inherited from the candidate. |
| `program_or_activity` | string | Program or activity name inherited from the candidate. |
| `source_target_priority` | integer | Priority inherited from the candidate. |
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

Rows are narrow internal component gate decisions. They are not full field
closure decisions, scoring gates, savings estimates, waste findings, fraud
findings, recoverable-dollar claims, or public claims.
