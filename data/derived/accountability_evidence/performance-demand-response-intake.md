# Performance Demand Response Intake

## Purpose

Use this generated intake template when a reply arrives for a performance demand.
It records source custody and classification inputs before any response-log row is updated.
It is not a finding of fraud, waste, abuse, legal dedication, poor performance, or reform success.

## Required Capture

| Field | Capture Rule |
|---|---|
| `record_id` | Copy from `performance-demand-response-log.jsonl`. |
| `reply_source_id` | Assign or cite a source-ledger ID for the reply artifact. |
| `reply_received_date` | Record the received date as `YYYY-MM-DD`. |
| `sender_or_office` | Name the responding office or official exactly as written. |
| `response_class` | Choose one class from the response log schema. |
| `evidence_received` | List concrete documents, datasets, citations, or official findings supplied by the reply. |
| `missing_evidence` | State the remaining missing source, performance, wording, or claim-basis evidence. |
| `role_review_needed` | Keep `true` until role review approves exact public wording. |
| `public_claim_allowed` | Keep `false` unless the claim gate is explicitly revalidated. |

## Response Classes

- `complete-evidence-response`: All requested evidence and claim basis were provided, pending role review.
- `partial-evidence-response`: At least one requested evidence item remains missing or unclear.
- `process-only-response`: The reply explains process but does not provide requested evidence.
- `no-evidence-response`: The reply declines, ignores, or cannot identify requested evidence.

## Update Rule

After intake, update `performance-demand-response-log.jsonl` only with source-custodied reply evidence and rerun validation.
Do not convert a reply into a fraud, waste, abuse, legal dedication, poor performance, or reform-benefit claim without reviewed evidence and an explicit public-claim gate.
