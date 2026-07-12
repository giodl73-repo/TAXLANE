# Payment Integrity Methodology Component Gate Source Targets Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable component gate source-target identifier. |
| `record_family` | string | Must be `payment_integrity_methodology_component_gate_source_target`. |
| `source_component_gate_requirement_record_id` | string | Component gate requirement that created this target. |
| `agency_code` | string | Agency code. |
| `program_or_activity` | string | Program or activity name. |
| `source_target_priority` | integer | Positive priority within the source requirement. |
| `source_target_name` | string | Source family to search. |
| `source_target_scope` | string | What the source family must resolve. |
| `evidence_to_extract` | array | Required fields to extract from sources. |
| `negative_evidence_rule` | string | Rule for keeping closure/scoring blocked if evidence is insufficient. |
| `next_artifact_family` | string | Must be `payment_integrity_methodology_component_gate_source_query`. |
| `field_closure_allowed` | boolean | Must remain `false`. |
| `scoring_allowed` | boolean | Must remain `false`. |
| `public_claim_allowed` | boolean | Must remain `false`. |
| `savings_estimate_allowed` | boolean | Must remain `false`. |
| `public_use_rule` | string | Required public-use boundary. |

## Public-Use Rule

Rows are internal source targets. They are not field closures, scoring gates,
savings estimates, waste findings, fraud findings, recoverable-dollar claims,
or public claims.
