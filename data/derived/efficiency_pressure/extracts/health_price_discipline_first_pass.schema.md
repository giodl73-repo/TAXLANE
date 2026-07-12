# Health Price Discipline First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable health price-discipline probe row ID. |
| `record_family` | string | Always `health_price_discipline_probe`. |
| `source_evidence_queue_record_id` | string | Related health price-discipline evidence queue row. |
| `source_ids` | array | Source IDs backing the row. |
| `observed_date` | string | Date the row was recorded in TAXLANE. |
| `program_part` | string | Medicare part or all-health scope. |
| `service_or_drug_category` | string | Service, drug, benchmark, or blocker category. |
| `fiscal_or_calendar_year` | string | Year basis. |
| `price_or_expenditure_basis` | string | Basis for the amount or benchmark. |
| `benchmark_or_comparison` | string | Benchmark or comparison statement. |
| `metric_value` | number or null | Source metric value, when applicable. |
| `metric_unit` | string or null | Unit for `metric_value`. |
| `denominator_value` | number or null | Denominator value, when applicable. |
| `denominator_unit` | string or null | Denominator unit. |
| `computed_value_usd` | number or null | Derived per-unit dollar value, when applicable. |
| `quality_or_access_measure` | string | Quality/access floor status. |
| `source_record_ids` | array | Source or derived record IDs used. |
| `readiness_status` | string | Whether the row is ready as an anchor or blocked before scoring. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-Use Rule

Rows are price-discipline probes. High-level international benchmarks and
Medicare per-enrollee anchors are not service-price findings, waste findings, or
savings estimates.
