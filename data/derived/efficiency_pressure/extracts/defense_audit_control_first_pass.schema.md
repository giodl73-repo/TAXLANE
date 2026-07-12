# Defense Audit Control First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable DoD audit-control probe row ID. |
| `record_family` | string | Always `defense_audit_control_probe`. |
| `source_evidence_queue_record_id` | string | Related defense audit-control evidence queue row. |
| `source_id` | string | Always `SRC-DODIG-FY2025-AUDIT`. |
| `observed_date` | string | Date the source was recorded in TAXLANE. |
| `report_url` | string | Source report URL. |
| `report_number` | string | DoD OIG report number. |
| `fiscal_year` | integer | Fiscal year covered by the audit. |
| `finding_type` | string | Summary, material weakness, significant deficiency, or noncompliance category. |
| `finding_identifier` | string | Stable normalized identifier. |
| `finding_title` | string | Source-facing title or label. |
| `audit_opinion` | string or null | Audit opinion label, when applicable. |
| `material_weakness_count` | integer or null | Count reported in the summary row, when applicable. |
| `significant_deficiency_count` | integer or null | Count reported in the summary row, when applicable. |
| `noncompliance_count` | integer or null | Count reported in the summary row, when applicable. |
| `reported_amount_usd` | number or null | Dollar context reported by the source, when applicable. |
| `reported_amount_basis` | string or null | Basis for any reported amount. |
| `affected_area` | string | Control or reporting area affected. |
| `control_signal` | string | What the source supports. |
| `recommendation_signal` | string | What should be tracked next. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Rows are audit-control probes. Material weaknesses, significant deficiencies,
and balances are not automatically fraud, waste, abuse, readiness findings, or
savings estimates.
