# Debt Maturity Risk First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable Treasury probe row ID. |
| `record_family` | string | Always `debt_maturity_risk_treasury_probe`. |
| `source_evidence_queue_record_id` | string | Related debt maturity-risk evidence queue row. |
| `source_id` | string | `SRC-TREASURY-DEBT-PENNY` or `SRC-TREASURY-AVG-INTEREST`. |
| `query_date` | string | Date the API was queried. |
| `api_url` | string | Query URL used for the row family. |
| `record_date` | string | Treasury source record date. |
| `row_kind` | string | `debt_stock` or `average_interest_rate`. |
| `security_type` | string | Security type or `all`. |
| `security_description` | string | Security class or debt-stock label. |
| `debt_held_public_amount` | number/null | Debt held by the public, dollars. |
| `intragovernmental_holdings_amount` | number/null | Intragovernmental holdings, dollars. |
| `total_public_debt_outstanding_amount` | number/null | Total public debt outstanding, dollars. |
| `average_interest_rate_percent` | number/null | Average interest rate percent. |
| `source_scope_note` | string | Scope caveat for the row. |
| `next_extract_need` | string | Next extraction needed before rate-risk scoring. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Treasury rate-risk probes provide query-locked debt stock and average-rate
context. They do not estimate savings, recommend debt management, or imply
delayed/defaulted debt service.
