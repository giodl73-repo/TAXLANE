# Performance Demand Response Log JSONL Schema

## Purpose

This schema documents the generated `performance-demand-response-log.jsonl` rows.
Rows track replies to evidence requests without changing claim gates or creating findings.

## Row Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `record_id` | string | yes | Accountability evidence record ID. |
| `lane_id` | string or null | conditional | Public-purpose lane when available. |
| `program_or_account_id` | string or null | conditional | Program, account, or OMB function identifier when available. |
| `response_class` | string | yes | Current response status. Initial generated value is `not-yet-received`. |
| `evidence_received` | array of strings | yes | Evidence items logged from a reply. Initial generated value is empty. |
| `missing_evidence` | string | yes | Current blocker or missing evidence item. |
| `claim_gate` | string | yes | Human-readable claim-gate label. Initial generated value is `Public claim blocked.` |
| `public_claim_allowed` | boolean | yes | Explicit claim gate for public use. Initial generated value is `false`. |
| `next_action` | string | yes | Safe next workflow action. |
| `use_rule` | string | yes | Boundary rule for using the row. |

At least one of `lane_id` or `program_or_account_id` must be present.

## Response Classes

- `not-yet-received`: no reply has been logged in TAXLANE.
- `complete-evidence-response`: all required evidence and public-claim basis were provided and still need role review before public use.
- `partial-evidence-response`: at least one requested evidence item remains missing or unclear.
- `process-only-response`: the reply explains process but does not provide requested evidence.
- `no-evidence-response`: the reply declines, ignores, or cannot identify requested evidence.

## Public-Use Rule

Rows may support response tracking. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
