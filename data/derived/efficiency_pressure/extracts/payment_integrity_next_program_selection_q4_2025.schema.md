# Payment Integrity Next Program Selection Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable next-program selection identifier. |
| `record_family` | string | Must be `payment_integrity_next_program_selection`. |
| `selected_program_key` | string | Stable key for the selected next branch. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity selected for the next methodology branch. |
| `selection_status` | string | Must be `selected_for_methodology_planning`. |
| `selection_reason` | string | Why this branch is next. |
| `official_source_urls` | array[string] | Official source URLs used to justify branch selection. |
| `starting_methodology_fields` | array[string] | Fields the next methodology plan must resolve. |
| `next_artifact_family` | string | Must be `payment_integrity_methodology_plan`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows select the next internal payment-integrity branch. They are not savings
estimates, waste findings, performance findings, fraud findings, or public
claims.
