# Cost-Down Backlog Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable backlog row ID. |
| `record_family` | string | Always `cost_down_backlog`. |
| `source_pressure_record_id` | string | Related `efficiency_pressure` row. |
| `lane_id` | string | Program, financing, or pressure lane. |
| `lever_id` | string | Stable short lever ID. |
| `lever_label` | string | Human-readable lever name. |
| `lever_type` | string | `price_discipline`, `administrative_simplification`, `procurement_control`, `fiscal_balance`, `risk_mitigation`, or `payment_integrity`. |
| `action_question` | string | The concrete evidence-seeking question. |
| `required_evidence` | array[string] | Source classes required before estimating savings. |
| `measurement_metric` | string | Metric that would show whether the lever is working. |
| `outcome_floor` | string | Public-purpose, legal, access, service, readiness, or resilience floor. |
| `time_horizon` | string | `near_term`, `medium_term`, or `long_term`. |
| `estimated_savings_usd` | number or null | Must be null until a reviewed estimate source is attached. |
| `savings_claim_status` | string | Current value: `blocked_no_estimate`. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Backlog rows are work items. They do not prove waste, do not estimate savings,
and do not authorize cuts unless the evidence and outcome floor are satisfied.
