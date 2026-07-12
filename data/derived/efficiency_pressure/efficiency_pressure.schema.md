# Efficiency Pressure Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable row ID. |
| `record_family` | string | Always `efficiency_pressure`. |
| `fiscal_year` | number | Fiscal year for the outlay context. |
| `surface` | string | Spend or financing surface under pressure. |
| `related_spend_categories` | array[string] | Related spend-category row IDs or lane labels. |
| `pressure_basis` | array[string] | Why the surface deserves scrutiny. |
| `pressure_level` | string | `highest`, `high`, or `watch`. |
| `not_a_finding` | boolean | Must be `true` until a reviewed finding source is attached. |
| `cost_down_levers` | array[string] | Candidate ways to lower cost over time. |
| `outcome_floor` | string | Public purpose, coverage, legal obligation, readiness, or resilience floor that savings must preserve. |
| `evidence_needed` | array[string] | Sources required before a public waste/performance claim. |
| `public_claim_status` | string | Current value: `blocked_question_surface_only`. |

## Public-use rule

Pressure rows ask where to look. They do not prove waste.
