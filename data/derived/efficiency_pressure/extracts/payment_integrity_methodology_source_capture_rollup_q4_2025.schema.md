# Payment Integrity Methodology Source Capture Rollup Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology source-capture rollup identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_source_capture_rollup`. |
| `source_methodology_gap_followup_record_id` | string | Gap-followup row being rolled up. |
| `source_methodology_gap_source_capture_record_id` | string | Source-capture row being rolled up. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Methodology field under review. |
| `capture_coverage_status` | string | Must be `source_captured_review_needed`. |
| `remaining_review_need` | string | Remaining question before field closure. |
| `reviewer_action` | string | Next reviewer action. |
| `field_closure_allowed` | boolean | Must remain `false`; rollups do not close fields. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source-capture rollups for methodology review. They are not
savings estimates, waste findings, performance findings, fraud findings, or
public claims.
