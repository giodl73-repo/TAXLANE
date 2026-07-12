# Payment Integrity Methodology Closure Coverage Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology closure-coverage identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_closure_coverage`. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_methodology_closure_decision_record_id` | string | Closure-decision row supporting the coverage count. |
| `total_methodology_fields` | number | Total fields in the methodology checklist. |
| `closed_field_count` | number | Number of internally closed fields. |
| `open_field_count` | number | Number of fields still open. |
| `closed_fields` | array[string] | Closed methodology fields. |
| `open_fields` | array[string] | Open methodology fields. |
| `coverage_status` | string | Must be `partial_methodology_closure`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal closure-coverage summaries. They are not scoring decisions,
savings estimates, waste findings, performance findings, fraud findings, or
public claims.
