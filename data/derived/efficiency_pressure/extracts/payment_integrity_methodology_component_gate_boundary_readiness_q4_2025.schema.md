# Payment Integrity Methodology Component Gate Boundary Readiness Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate boundary-readiness identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_boundary_readiness`. |
| `source_component_gate_boundary_decision_record_id` | string | Component gate boundary decision being triaged. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the decision. |
| `boundary_readiness_status` | string | `narrow_internal_readiness_candidate` or `additional_positive_basis_needed`. |
| `readiness_scope` | string | Scope that is or is not ready. |
| `readiness_reason` | string | Why the status applies. |
| `next_required_action` | string | Next reviewer or source-work action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal readiness triage records. They are not field closures, scoring
gates, savings estimates, waste findings, fraud findings, recoverable-dollar
claims, or public claims.
