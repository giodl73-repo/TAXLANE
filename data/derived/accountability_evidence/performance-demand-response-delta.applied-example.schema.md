# Performance Demand Response Delta Applied Example JSONL Schema

## Purpose

This schema documents `performance-demand-response-delta.applied-example.jsonl` rows.
Rows are generated from `PerformanceDemandResponseDeltaRow` to show importer fixture changes without creating findings.

## Row Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `record_id` | string | yes | Accountability evidence record ID for the changed response row. |
| `before_response_class` | string | yes | Response-log class before applying the intake fixture. |
| `after_response_class` | string | yes | Response-log class after applying the intake fixture. |
| `before_evidence_received_count` | integer | yes | Count of evidence items before applying intake. |
| `after_evidence_received_count` | integer | yes | Count of evidence items after applying intake. |
| `missing_evidence_changed` | boolean | yes | Whether the missing-evidence text changed. |
| `next_action_changed` | boolean | yes | Whether the next-action text changed. |
| `before_claim_gate` | string | yes | Claim-gate label before applying intake. Must remain `Public claim blocked.`. |
| `after_claim_gate` | string | yes | Claim-gate label after applying intake. Must remain `Public claim blocked.`. |

## Gate Rules

- Rows must validate through `PerformanceDemandResponseDeltaRow`.
- Both claim-gate fields must remain `Public claim blocked.`.
- Rows describe fixture deltas only; they are not canonical response status.

## Public-Use Rule

Rows may support importer and UI/API testing. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
