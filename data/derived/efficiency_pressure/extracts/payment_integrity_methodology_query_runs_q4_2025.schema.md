# Payment Integrity Methodology Query Runs Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable query-run identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_query_run`. |
| `source_methodology_query_record_id` | string | Planned query row being prepared for execution. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `run_status` | string | Must be `pending_not_run` until query execution. |
| `planned_query_text` | string | Query text to run. |
| `result_capture_status` | string | Must be `no_result_captured` until a result is attached. |
| `required_capture_fields` | array[string] | Fields required for a valid result capture. |
| `next_run_rule` | string | Rule for executing the query and creating a source extract. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are pending query-run scaffolds. They are not executed source extracts,
savings estimates, waste findings, performance findings, fraud findings, or
public claims.
