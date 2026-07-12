# Payment Integrity Methodology Component Gate Source Capture Rollups Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate source-capture rollup identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_source_capture_rollup`. |
| `source_component_gate_source_capture_record_id` | string | Source capture being rolled up. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the capture. |
| `capture_rollup_status` | string | `reviewer_gate_decision_needed` or `additional_positive_basis_needed`. |
| `gate_finding` | string | Conservative reviewer-facing finding. |
| `remaining_review_need` | string | Evidence or review still needed. |
| `reviewer_action` | string | Next reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal capture rollups. They are not field closures, scoring gates,
savings estimates, waste findings, fraud findings, recoverable-dollar claims,
or public claims.
