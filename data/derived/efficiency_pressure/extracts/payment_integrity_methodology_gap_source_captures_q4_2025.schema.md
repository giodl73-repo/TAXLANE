# Payment Integrity Methodology Gap Source Captures Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology gap source-capture identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_gap_source_capture`. |
| `source_methodology_gap_followup_record_id` | string | Gap-followup row that prompted this capture. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Methodology field being supported. |
| `observed_date` | string | Date the source was observed. |
| `source_url` | string | Canonical source URL. |
| `source_title` | string | Source title. |
| `source_publisher` | string | Source publisher. |
| `captured_source_scope` | string | Source section or scope reviewed. |
| `captured_methodology_summary` | string | Short summary of captured methodology text. |
| `support_status` | string | Must be `partial_support_review_needed`. |
| `field_closure_allowed` | boolean | Must remain `false`; captures do not close fields. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source captures for methodology follow-up work. They are not
savings estimates, waste findings, performance findings, fraud findings, or
public claims.
