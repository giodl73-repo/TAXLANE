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

- `not-yet-received`: No reply has been logged in TAXLANE.
- `complete-evidence-response`: Provides source record/version, reviewed performance evidence or official finding, role-approved wording, and public-claim basis.
- `partial-evidence-response`: Provides some requested evidence but leaves at least one required item missing or unclear.
- `process-only-response`: Explains process, office ownership, or future work but does not provide the requested evidence.
- `no-evidence-response`: Declines, ignores, or cannot identify the requested evidence.

## Gate Rules

- `evidence_received` must be non-empty when `response_class` is `complete-evidence-response` or `partial-evidence-response`.
- `evidence_received` must be empty when `response_class` is `not-yet-received`, `process-only-response`, or `no-evidence-response`.
- `public_claim_allowed` must remain `false` unless a separate reviewed evidence record and public-claim gate allow a public statement.

## Public-Use Rule

Rows may support response tracking. They must not be used as findings of fraud, waste, abuse, legal dedication of income taxes, poor performance, or proven reform benefits.
