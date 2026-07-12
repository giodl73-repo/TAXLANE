# Payment Integrity Methodology Closure Decisions Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology closure-decision identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_closure_decision`. |
| `source_methodology_closure_readiness_record_id` | string | Closure-readiness row supporting this decision. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Methodology field being closed internally. |
| `decision_status` | string | Must be `field_closed_internal_only`. |
| `field_closed` | boolean | Must be `true` for closure-decision rows. |
| `decision_basis` | string | Source-backed basis for closure. |
| `closure_scope` | string | Narrow field-level scope of the decision. |
| `residual_limitations` | array[string] | Explicit limitations after closure. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal methodology closure decisions. They are not savings estimates,
waste findings, performance findings, fraud findings, or public claims.
