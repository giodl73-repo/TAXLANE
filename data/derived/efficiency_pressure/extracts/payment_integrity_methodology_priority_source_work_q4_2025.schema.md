# Payment Integrity Methodology Priority Source Work Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology priority source-work identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_priority_source_work`. |
| `source_residual_gap_priority_record_id` | string | Priority row this source-work result addresses. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `priority_rank` | integer | Priority rank copied from the source priority row. |
| `selected_methodology_field` | string | Methodology field copied from the source priority row. |
| `observed_date` | string | Date the source work was recorded. |
| `source_work_status` | string | Source-work outcome status. |
| `official_source_urls` | array | Official source URLs used for the result. |
| `source_summary` | string | What the source supports. |
| `resolution_effect` | string | How the source affects the selected blocker. |
| `remaining_blocker` | string | Why the row still does not authorize scoring or claims. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source-work results. They are not closure decisions, scoring
gates, savings estimates, waste findings, fraud findings, recoverable-dollar
claims, or public claims.
