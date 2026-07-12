# Defense Procurement Control First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable GAO procurement-control probe row ID. |
| `record_family` | string | Always `defense_procurement_control_probe`. |
| `source_evidence_queue_record_id` | string | Related defense procurement-control evidence queue row. |
| `source_id` | string | Always `SRC-GAO-WEAPON-SYSTEMS-2025`. |
| `observed_date` | string | Date the source was recorded in TAXLANE. |
| `report_url` | string | Source report URL. |
| `report_number` | string | GAO report number. |
| `report_year` | integer | Report year. |
| `program_or_portfolio` | string | Program or portfolio named by the probe. |
| `service_or_scope` | string | Service, agency, or portfolio scope. |
| `acquisition_pathway` | string or null | Acquisition pathway, when applicable. |
| `signal_type` | string | Cost, schedule, leading-practice, or portfolio signal type. |
| `signal_title` | string | Reader-facing signal label. |
| `reported_amount_usd` | number or null | Dollar context reported by GAO, when applicable. |
| `reported_amount_basis` | string or null | Basis for any reported amount. |
| `reported_percent` | number or null | Percent context reported by GAO, when applicable. |
| `reported_months` | number or null | Schedule-month context reported by GAO, when applicable. |
| `reviewed_program_count` | integer or null | Program count for portfolio rows. |
| `control_signal` | string | What the source supports. |
| `recommendation_signal` | string | What should be tracked next. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Rows are procurement-control probes. Cost growth, schedule risk, and
leading-practice gaps are not automatically waste, readiness findings, program
cancellation recommendations, or savings estimates.
