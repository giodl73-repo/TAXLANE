# Payment Integrity Methodology Component Gate Progress Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate progress identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_progress`. |
| `source_open_program_status_record_id` | string | Open-program status row whose field counts remain unchanged. |
| `source_component_gate_narrow_decision_record_id` | string | Component gate narrow decision being reflected. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `component_progress_status` | string | Must be `component_gate_progress_recorded_no_field_closure`. |
| `total_methodology_fields` | integer | Must be `8`. |
| `closed_field_count_after_component_decision` | integer | Full methodology fields closed after the component decision. |
| `open_field_count_after_component_decision` | integer | Full methodology fields still open after the component decision. |
| `component_gate_decision_count` | integer | Component gate decisions reflected in this row. |
| `component_progress_summary` | string | What component-level progress was recorded. |
| `unchanged_field_count_reason` | string | Why full field counts did not change. |
| `next_gate_condition` | string | Evidence or decision needed before field closure or scoring. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal component gate progress summaries. They are not field-closure
decisions, scoring gates, savings estimates, waste findings, fraud findings,
recoverable-dollar claims, or public claims.
