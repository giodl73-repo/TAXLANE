# Payment Integrity Methodology Plans Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology-plan identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_plan`. |
| `source_program_status_record_id` | string | Program status row that selected methodology as next priority. |
| `source_methodology_task_record_id` | string | Methodology task row being made extractable. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `required_methodology_fields` | array[string] | Required fields before methodology can be marked complete. |
| `source_discovery_targets` | array[string] | Source families to inspect first. |
| `extraction_priority` | integer | Priority order across the methodology plans. |
| `methodology_completion_rule` | string | Rule for closing the methodology blocker. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are methodology extraction plans. They are not savings estimates, waste
findings, performance findings, fraud findings, or public claims.
