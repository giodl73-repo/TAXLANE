# Payment Integrity Methodology Scoring Gate Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology scoring-gate identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_scoring_gate`. |
| `source_methodology_closure_coverage_record_id` | string | Closure-coverage row supporting this gate. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `gate_status` | string | Must be `blocked_methodology_incomplete`. |
| `gate_reason` | string | Why scoring remains blocked. |
| `blockers` | array[string] | Blocking methodology gaps. |
| `next_milestone` | string | Required next step before scoring. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal scoring gates. They are not savings estimates, waste findings,
performance findings, fraud findings, or public claims.
