# Payment Integrity Methodology Component Gate Boundary Decisions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate boundary-decision identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_boundary_decision`. |
| `source_component_gate_source_capture_rollup_record_id` | string | Component gate source-capture rollup being decided. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the rollup. |
| `boundary_decision_status` | string | Internal boundary decision status. |
| `boundary_decision` | string | Conservative internal boundary decision. |
| `scoring_implication` | string | Why scoring remains blocked. |
| `next_required_action` | string | Next reviewer or source-work action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal boundary decisions. They are not field closures, scoring
gates, savings estimates, waste findings, fraud findings, recoverable-dollar
claims, or public claims.
