# Payment Integrity Claims Timeliness First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable claims-timeliness probe row ID. |
| `record_family` | string | Always `payment_integrity_claims_timeliness_probe`. |
| `source_evidence_queue_record_id` | string | Related claims-timeliness evidence queue row. |
| `source_id` | string | `SRC-SSA-PERFORMANCE` or `SRC-VA-CLAIMS-DATA`. |
| `observed_date` | string | Date the source page was observed. |
| `page_url` | string | Source page URL. |
| `agency_code` | string | Agency code. |
| `metric_name` | string | Metric label. |
| `metric_value` | number | Numeric metric value. |
| `metric_unit` | string | Unit for the value. |
| `comparison_operator` | string | `reported_value`, `less_than`, or `improvement`. |
| `metric_period` | string | Period or observation basis. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before analysis. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Claims-timeliness probe rows identify service-floor evidence to lock next. They
do not prove poor performance, waste, fraud, abuse, or savings.
