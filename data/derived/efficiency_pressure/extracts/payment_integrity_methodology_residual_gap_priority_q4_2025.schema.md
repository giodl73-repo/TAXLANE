# Payment Integrity Methodology Residual Gap Priority Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology residual-gap priority identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_residual_gap_priority`. |
| `source_open_program_status_record_id` | string | Open-program status row that the priority belongs to. |
| `source_residual_source_gap_record_id` | string | Residual source-gap row selected as the next priority. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Overall source-work priority rank. |
| `selected_methodology_field` | string | Methodology field selected for the next source-work pass. |
| `priority_reason` | string | Why this blocker is the next priority. |
| `next_query_text` | string | Query text copied from the selected residual source gap. |
| `resolution_rule` | string | Evidence condition needed before the field can move toward closure. |
| `blocked_claims_note` | string | Claim types blocked while this gap remains unresolved. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source-work priorities. They are not scoring gates, savings
estimates, waste findings, fraud findings, recoverable-dollar claims, or public
claims.
