# Payment Integrity Scorecards Q4 2025 First-Pass Extract Schema

One JSON object per line.

| Field | Type | Meaning |
|---|---|---|
| `record_id` | string | Stable scorecard-probe row ID. |
| `record_family` | string | Always `payment_integrity_scorecard_probe`. |
| `source_evidence_queue_record_id` | string | Related payment-integrity evidence queue row. |
| `source_id` | string | Always `SRC-OMB-PAYMENTACCURACY`. |
| `observed_date` | string | Date the scorecard was observed. |
| `scorecard_url` | string | PaymentAccuracy scorecard PDF URL. |
| `reporting_period` | string | Scorecard reporting period. |
| `agency_code` | string | Agency code used by TAXLANE for the row. |
| `program_or_activity` | string | Program or activity named on the scorecard. |
| `fy2024_overpayment_amount_millions` | number | FY2024 overpayment amount in millions of dollars. |
| `fy2024_overpayment_rate_percent` | number | FY2024 overpayment rate. |
| `sample_period_note` | string | Methodology/sample caveat. |
| `primary_root_cause_amount_millions` | number | Largest extracted root-cause amount in millions of dollars. |
| `root_cause_control_scope` | string | Extracted root-cause control scope. |
| `root_cause_data_access_issue` | string | Extracted root-cause data issue. |
| `mitigation_strategy` | string | Scorecard mitigation language, paraphrased. |
| `source_scope_note` | string | Scope caveat for this probe. |
| `next_extract_need` | string | Next extraction needed before analysis. |
| `public_claim_allowed` | boolean | Must be false. |
| `savings_estimate_allowed` | boolean | Must be false. |
| `public_use_rule` | string | Must block savings-estimate and finding claims. |

## Public-use rule

Scorecard probe rows identify program-level payment-integrity evidence that
deserves extraction. They do not prove fraud, waste, abuse, poor performance, or
savings.
