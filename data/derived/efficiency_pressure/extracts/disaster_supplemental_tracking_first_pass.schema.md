# Disaster Supplemental Tracking First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable FEMA declaration probe row ID. |
| `record_family` | string | Always `disaster_declaration_probe`. |
| `source_evidence_queue_record_id` | string | Related disaster supplemental-tracking evidence queue row. |
| `source_id` | string | Always `SRC-FEMA-DISASTER-DECLARATIONS`. |
| `query_date` | string | Date the API was queried. |
| `api_url` | string | API query URL. |
| `disaster_number` | integer | FEMA disaster number. |
| `declaration_date` | string | FEMA declaration date. |
| `incident_type` | string | Incident type. |
| `state` | string | State or territory code. |
| `designated_area` | string | Designated area. |
| `declaration_title` | string | FEMA declaration title. |
| `ih_program_declared` | boolean | Individual and Households Program flag. |
| `ia_program_declared` | boolean | Individual Assistance flag. |
| `pa_program_declared` | boolean | Public Assistance flag. |
| `hm_program_declared` | boolean | Hazard Mitigation flag. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before outlay tracking. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Declaration rows are event/geography markers. They are not outlays, damages,
benefit-cost estimates, waste findings, or savings estimates.
