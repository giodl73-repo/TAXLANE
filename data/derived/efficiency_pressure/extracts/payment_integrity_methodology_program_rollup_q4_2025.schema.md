# Payment Integrity Methodology Program Rollup Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology program-rollup identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_program_rollup`. |
| `source_methodology_scoring_gate_record_id` | string | Scoring-gate row supporting this rollup. |
| `source_methodology_closure_coverage_record_id` | string | Closure-coverage row supporting this rollup. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `total_methodology_fields` | integer | Must be `8`. |
| `closed_field_count` | integer | Number of internally closed methodology fields. |
| `open_field_count` | integer | Number of still-open methodology fields. |
| `scoring_gate_status` | string | Must be `blocked_methodology_incomplete`. |
| `next_open_methodology_fields` | array[string] | Open fields that must be resolved before scoring. |
| `next_action` | string | Next source-work action. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal cross-program status rollups. They are not savings estimates,
waste findings, performance findings, fraud findings, or public claims.
