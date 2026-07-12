# Debt Primary Balance First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable fiscal-balance probe row ID. |
| `record_family` | string | Always `debt_primary_balance_fiscal_probe`. |
| `source_evidence_queue_record_id` | string | Related primary-balance evidence queue row. |
| `fiscal_year` | integer | Fiscal year. |
| `source_ids` | array[string] | Source ledger IDs and internal derived source paths. |
| `total_receipts_millions` | number | FY receipts, millions of dollars. |
| `total_outlays_millions` | number | FY outlays, millions of dollars. |
| `deficit_gap_millions` | number | Outlays minus receipts, millions of dollars. |
| `gross_treasury_interest_outlays_millions` | number | OMB Table 3.2 gross Treasury-interest subfunction outlays. |
| `primary_deficit_proxy_millions` | number | Deficit gap minus gross Treasury-interest outlays. |
| `borrowed_share_percent_of_outlays` | number | Deficit gap divided by total outlays. |
| `income_tax_coverage_percent_of_outlays` | number | Individual income-tax receipts divided by total outlays. |
| `basis_note` | string | Scope and proxy caveat. |
| `next_extract_need` | string | Next extraction needed before scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Primary-balance probe rows provide fiscal-balance context. They do not estimate
savings, score policy, recommend cuts or taxes, or treat interest as a program
service failure.
