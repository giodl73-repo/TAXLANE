# Payment Integrity Methodology Fields Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology-field checklist identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_field`. |
| `source_methodology_plan_record_id` | string | Methodology plan row that requires the field. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Required methodology field. |
| `field_status` | string | Must be `open_source_needed` until source text is attached. |
| `required_source_target` | string | Source family or document class to inspect. |
| `completion_rule` | string | Rule for marking the field complete. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are source-checklist items. They are not savings estimates, waste findings,
performance findings, fraud findings, or public claims.
