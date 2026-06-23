# Performance Demand Response Intake Schema

## Purpose

This schema documents the fields a future UI/API importer should capture from `performance-demand-response-intake.md`.
It defines an intake contract only; it does not authorize public claims or response-log updates without validation.

## Row Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `record_id` | string | yes | Accountability evidence record ID copied from the response log row. |
| `reply_source_id` | string | yes | Source-ledger identifier or custody pointer for the received reply artifact. |
| `reply_received_date` | string | yes | ISO date (`YYYY-MM-DD`) when the reply was received. |
| `sender_or_office` | string | yes | Responding office or official exactly as written in the reply. |
| `response_class` | string | yes | One allowed response class from this schema. |
| `evidence_received` | array of strings | yes | Concrete documents, datasets, citations, or official findings supplied by the reply. |
| `missing_evidence` | string | yes | Remaining source, performance, wording, or claim-basis evidence gap. |
| `role_review_needed` | boolean | yes | Must remain `true` until exact public wording receives role review. |
| `public_claim_allowed` | boolean | yes | Must remain `false` unless claim gates are explicitly revalidated. |
| `use_rule` | string | yes | Boundary rule for using the intake row. |

## Allowed Response Classes

- `complete-evidence-response`
- `partial-evidence-response`
- `process-only-response`
- `no-evidence-response`

## Gate Rules

- `role_review_needed` must be `true` for unreviewed replies.
- `public_claim_allowed` must be `false` until the response log, role review, and claim gates are revalidated.
- Empty `evidence_received` is allowed only when `response_class` is `process-only-response` or `no-evidence-response`.

## Public-Use Rule

Capture reply custody and classification; do not claim TAXLANE found fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
