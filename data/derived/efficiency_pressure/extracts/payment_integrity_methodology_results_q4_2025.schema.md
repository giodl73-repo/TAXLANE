# Payment Integrity Methodology Results Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology-result identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_result`. |
| `source_methodology_query_run_record_id` | string | Query-run row that produced this result. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `observed_date` | string | Date the source was observed. |
| `source_url` | string | Canonical source URL. |
| `source_title` | string | Source title. |
| `reporting_period` | string | Reporting period stated by the source. |
| `captured_methodology_text` | string | Short captured methodology summary. |
| `captured_field_scope` | array[string] | Fields touched by this capture. |
| `field_closure_allowed` | boolean | Must remain `false` until reviewed against field checklist. |
| `result_status` | string | Must be `source_captured_review_needed`. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are captured source-result summaries. They are not savings estimates, waste
findings, performance findings, fraud findings, or public claims.
