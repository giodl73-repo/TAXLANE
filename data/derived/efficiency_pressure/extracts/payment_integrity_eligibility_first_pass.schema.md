# Payment Integrity Eligibility First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable portal-probe row ID. |
| `record_family` | string | Always `payment_integrity_portal_probe`. |
| `source_evidence_queue_record_id` | string | Related cost-down evidence queue row. |
| `source_id` | string | Always `SRC-OMB-PAYMENTACCURACY`. |
| `observed_date` | string | Date the source page was observed. |
| `page_url` | string | Source page URL. |
| `row_kind` | string | `homepage_highest_performing_agency` or `homepage_lowest_performing_agency`. |
| `agency_code` | string | PaymentAccuracy agency code shown on the homepage. |
| `agency_name` | string | Agency display name. |
| `high_priority_program_count` | integer | Homepage high-priority-program count for the agency row. |
| `improper_payment_percentage` | number | Homepage agency percentage shown by PaymentAccuracy. |
| `source_scope_note` | string | Scope caveat for this portal-level row. |
| `next_extract_need` | string | Next source extraction needed before program-level analysis. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

This extract is a source-access probe. It records visible PaymentAccuracy
homepage trend rows and the next data pull needed. It does not prove waste,
fraud, abuse, poor performance, or savings.
