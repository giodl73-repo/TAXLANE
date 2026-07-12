# Health Administrative Simplification First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable health administrative-simplification probe row ID. |
| `record_family` | string | Always `health_admin_simplification_probe`. |
| `source_evidence_queue_record_id` | string | Related health administrative-simplification evidence queue row. |
| `source_ids` | array | Source IDs backing the row. |
| `observed_date` | string | Date the row was recorded in TAXLANE. |
| `program_part` | string | Medicare part or all-health scope. |
| `workflow_step` | string | Workflow, context, or blocker step. |
| `period` | string | Period or year basis. |
| `administrative_cost_or_cycle_time_basis` | string | Cost, cycle-time, or blocker basis. |
| `claim_or_case_count` | number or null | Claim, case, appeal, authorization, or rework count, when available. |
| `claim_or_case_count_unit` | string or null | Count unit. |
| `metric_value` | number or null | Context metric, when available. |
| `metric_unit` | string or null | Metric unit. |
| `access_or_integrity_floor` | string | Access, due-process, service-level, payment-accuracy, or integrity floor status. |
| `source_record_ids` | array | Source or derived record IDs used. |
| `readiness_status` | string | Whether the row is context-ready or blocked before scoring. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Rows are administrative-simplification probes. Literature context and
trust-fund totals are not administrative-cost findings, workflow findings, or
savings estimates.
