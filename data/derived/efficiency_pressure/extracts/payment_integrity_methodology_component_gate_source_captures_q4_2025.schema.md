# Payment Integrity Methodology Component Gate Source Captures Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate source-capture identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_source_capture`. |
| `source_component_gate_source_query_run_record_id` | string | Source query-run row that produced this capture. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Priority inherited from the query run. |
| `observed_date` | string | Date the source was inspected. |
| `source_url` | string | Official source URL. |
| `source_title` | string | Source title. |
| `captured_source_scope` | string | Source section or scope captured. |
| `captured_gate_summary` | string | Conservative summary of component-gate evidence. |
| `component_gate_status` | string | Capture status for the component gate. |
| `next_review_action` | string | Next reviewer action before closure or scoring. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source captures. They are not field closures, scoring gates,
savings estimates, waste findings, fraud findings, recoverable-dollar claims,
or public claims.
