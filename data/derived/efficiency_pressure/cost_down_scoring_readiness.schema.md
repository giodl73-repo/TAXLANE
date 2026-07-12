# Cost-Down Scoring Readiness Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable identifier for the readiness row. |
| `record_family` | string | Must be `cost_down_scoring_readiness`. |
| `source_rollup_record_id` | string | Cost-down first-pass rollup row being ranked. |
| `source_evidence_queue_record_id` | string | Original evidence queue row. |
| `lane_id` | string | Program lane or pressure surface. |
| `lever_id` | string | Cost-down lever being assessed. |
| `prioritization_rank` | integer | Extraction priority, starting at 1. |
| `readiness_tier` | string | Plain-language readiness bucket. |
| `evidence_maturity_score` | integer | 1-5 maturity rating for currently attached evidence. |
| `scale_pressure_score` | integer | 1-5 pressure rating based on fiscal scale/context. |
| `scoring_complexity_score` | integer | 1-5 complexity rating, where 5 means harder to score safely. |
| `priority_rationale` | string | Why this row has its priority position. |
| `immediate_next_artifact` | string | Next extract or bridge needed before scoring. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows rank extraction readiness only. They are not savings estimates, waste
findings, performance findings, or public claims.
