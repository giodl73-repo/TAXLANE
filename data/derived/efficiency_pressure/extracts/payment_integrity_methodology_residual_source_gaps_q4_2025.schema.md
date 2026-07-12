# Payment Integrity Methodology Residual Source Gaps Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology residual source-gap identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_residual_source_gap`. |
| `source_methodology_closure_readiness_record_id` | string | Additional-source-needed readiness row. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Methodology field that remains open. |
| `residual_gap_class` | string | `detail_source_needed`, `current_year_source_needed`, or `reviewer_determination_needed`. |
| `source_need` | string | Narrow source need. |
| `next_query_text` | string | Suggested next official-source query. |
| `closure_blocked_reason` | string | Why closure remains blocked. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal residual source gaps. They are not closure decisions, savings
estimates, waste findings, performance findings, fraud findings, or public
claims.
