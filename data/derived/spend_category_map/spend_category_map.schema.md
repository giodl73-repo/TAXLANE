# Spend Category Map Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `model_id` | string | Always `spend-category-map-v1`. |
| `record_id` | string | Stable row ID. |
| `fiscal_year` | number | Federal fiscal year. |
| `rank` | number | Rank within FY2025 top spend subfunctions. |
| `source_level` | string | Evidence level, currently `omb_subfunction`. |
| `source_id` | string | Source ID from the source-version ledger. |
| `function_code` | string | OMB budget function code. |
| `function_label` | string | OMB budget function label. |
| `subfunction_code` | string | OMB budget subfunction code. |
| `subfunction_label` | string | OMB budget subfunction label. |
| `subfunction_outlays_millions` | number | OMB outlays in millions of dollars. |
| `share_of_total_outlays_percent` | number | Share of FY2025 total outlays. |
| `modeled_income_tax_allocation_millions` | number | Proportional modeled individual income-tax allocation in millions of dollars. |
| `allocation_method` | string | Current value: `proportional_outlay_share`. |
| `legal_allocation_status` | string | Current value: `modeled_not_legal_dedication`. |
| `funding_caveat` | string | Public caveat for the category. |
| `next_source_need` | string | Next source family required for a deeper question. |
| `accountability_status` | string | Current value: `question_surface_only`. |

## Required labels

- `source_level` must make the evidence grain visible.
- `legal_allocation_status` must block taxpayer-dollar tracing.
- `accountability_status` must block performance, fraud, waste, abuse, and
  duplication claims until a specific source supports them.
