# Payment Integrity Methodology Source Targets Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable source-target identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_source_target`. |
| `source_methodology_plan_record_id` | string | Methodology plan row that names the target. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target` | string | Source target to inspect. |
| `target_priority` | integer | Priority within the methodology plan. |
| `target_status` | string | Must be `open_source_needed` until source text is captured. |
| `target_use` | string | Why the source target is needed. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are source-discovery targets. They are not savings estimates, waste
findings, performance findings, fraud findings, or public claims.
