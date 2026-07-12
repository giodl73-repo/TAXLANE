# Payment Integrity Methodology Gap Followups Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable methodology gap-followup identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_gap_followup`. |
| `source_methodology_field_review_record_id` | string | Field-review row that created this follow-up. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `methodology_field` | string | Field name from the methodology checklist. |
| `gap_class` | string | `unsupported_field_source_needed` or `partial_support_citation_needed`. |
| `followup_priority` | number | Priority order, 1 through 8 within each program queue. |
| `source_target` | string | Source family to inspect next. |
| `next_action` | string | Concrete source-work action. |
| `completion_evidence_required` | array[string] | Evidence required before closure review. |
| `field_closure_allowed` | boolean | Must remain `false`; follow-ups do not close fields. |
| `public_claim_allowed` | boolean | Must remain `false` for draft rows. |
| `savings_estimate_allowed` | boolean | Must remain `false` for draft rows. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source-work follow-ups. They are not savings estimates, waste
findings, performance findings, fraud findings, or public claims.
