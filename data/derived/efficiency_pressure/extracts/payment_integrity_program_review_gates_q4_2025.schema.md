# Payment Integrity Program Review Gates Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable program-review gate identifier. |
| `record_family` | string | Must be `payment_integrity_program_review_gate`. |
| `source_scorecard_record_id` | string | Source PaymentAccuracy scorecard probe row. |
| `source_readiness_record_id` | string | Source cost-down scoring-readiness row. |
| `agency_code` | string | Agency code from the scorecard probe. |
| `program_or_activity` | string | Program or activity from the scorecard probe. |
| `reporting_period` | string | Must be `Q4 2025` for this extract. |
| `fy2024_overpayment_amount_millions` | number | Scorecard probe overpayment amount, in millions. |
| `fy2024_overpayment_rate_percent` | number | Scorecard probe overpayment rate. |
| `methodology_status` | string | Whether the methodology extract is complete. |
| `access_floor_status` | string | Whether beneficiary/producer/veteran access floors are complete. |
| `corrective_action_status` | string | Whether corrective-action owner/status evidence is complete. |
| `confidence_limit_status` | string | Whether uncertainty/confidence-limit evidence is complete. |
| `claim_boundary_status` | string | Public claim boundary status. |
| `required_next_evidence` | array[string] | Evidence required before a savings score. |
| `review_gate_status` | string | Must be `blocked_before_savings_score` for draft rows. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are review gates for program-level extraction. They are not savings
estimates, waste findings, performance findings, fraud findings, or public
claims.
