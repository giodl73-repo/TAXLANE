# Payment Integrity Methodology Component Gate Progress Requirements Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate progress-requirement identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_progress_requirement`. |
| `source_component_gate_progress_record_id` | string | Component gate progress row that remains blocked. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `gate_status` | string | Must be `positive_evidence_required_before_field_closure`. |
| `required_positive_evidence` | string | Evidence needed before field closure or scoring. |
| `blocked_translation` | string | Claim translation that remains blocked. |
| `next_source_target` | string | Source family to pursue next. |
| `next_decision_type` | string | Must be `full_field_closure_review`. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal gate requirements. They are not field closures, scoring gates,
savings estimates, waste findings, fraud findings, recoverable-dollar claims,
or public claims.
