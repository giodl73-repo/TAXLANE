# Disaster Mitigation First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable FEMA HMA project probe row ID. |
| `record_family` | string | Always `disaster_mitigation_project_probe`. |
| `source_evidence_queue_record_id` | string | Related disaster mitigation evidence queue row. |
| `source_id` | string | Always `SRC-FEMA-HMA-PROJECTS`. |
| `query_date` | string | Date the API was queried. |
| `api_url` | string | API query URL. |
| `project_identifier` | string | FEMA project identifier. |
| `program_area` | string | FEMA mitigation program area. |
| `program_fy` | integer | Program fiscal year. |
| `state` | string | State. |
| `county` | string | County or local geography field. |
| `disaster_number` | integer or null | FEMA disaster number, when supplied. |
| `project_type` | string | FEMA project type text. |
| `status` | string | Project status. |
| `recipient` | string | Recipient. |
| `subrecipient` | string | Subrecipient. |
| `data_source` | string | FEMA data-source label. |
| `date_approved` | string or null | Approval date, when supplied. |
| `date_closed` | string or null | Closure date, when supplied. |
| `project_amount` | number or null | Project amount as reported by FEMA. |
| `federal_share_obligated` | number or null | Federal share obligated as reported by FEMA. |
| `cost_share_percentage` | number or null | Cost-share percentage as reported by FEMA. |
| `benefit_cost_ratio` | number or null | Benefit-cost ratio as reported by FEMA. |
| `net_value_benefits` | number or null | Net benefits as reported by FEMA. |
| `number_of_properties` | integer or null | Property count field as reported by FEMA. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Project rows are mitigation-project markers. Amounts and benefit-cost fields are
not federal outlay totals, verified avoided-loss findings, waste findings, or
savings estimates without reviewed method and account linkage.
