# Payment Integrity Methodology Field Reviews Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology field-review identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_field_review`. |
| `source_methodology_result_record_id` | string | Captured methodology-result row being reviewed. |
| `source_methodology_field_record_id` | string | Methodology-field checklist row being reviewed. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Field name from the methodology checklist. |
| `evidence_status` | string | `partial_support_review_needed` or `not_supported_by_result`. |
| `reviewed_source_scope` | string | Source scope reviewed for this field. |
| `review_note` | string | Short explanation of support or gap. |
| `field_closure_allowed` | boolean | Must remain `false`; reviews do not close fields. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal methodology field reviews. They are not savings estimates,
waste findings, performance findings, fraud findings, or public claims.
