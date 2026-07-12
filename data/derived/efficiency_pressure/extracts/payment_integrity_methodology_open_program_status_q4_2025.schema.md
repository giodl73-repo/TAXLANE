# Payment Integrity Methodology Open Program Status Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology open-program status identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_open_program_status`. |
| `source_methodology_plan_record_id` | string | Methodology plan supporting this program status row. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `closure_path_status` | string | `closure_coverage_available` or `fully_open_no_closure_decision`. |
| `total_methodology_fields` | integer | Must be `8`. |
| `closed_field_count` | integer | Internally closed methodology fields. |
| `open_field_count` | integer | Methodology fields still open. |
| `closure_decision_count` | integer | Internal closure-decision count. |
| `residual_source_gap_count` | integer | Residual source gaps still open. |
| `blocker_summary` | string | Why the program remains blocked before scoring. |
| `next_priority` | string | Next source-work priority. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal open-program status summaries. They are not scoring gates,
savings estimates, waste findings, performance findings, fraud findings,
recoverable-dollar claims, or public claims.
