# Performance Demand Checklist JSONL Schema

## Purpose

This schema documents the generated `performance-demand-checklist.jsonl` rows.
The Rust contract is `taxlane_core::PerformanceDemandChecklistRecord`.

## Row Fields

| Field | Type | Required | Meaning |
|---|---|---|---|
| `record_id` | string | yes | Accountability evidence record ID. |
| `lane_id` | string or null | conditional | Public-purpose lane when available. |
| `program_or_account_id` | string or null | conditional | Program, account, or OMB function identifier when available. |
| `demand_question` | string | yes | Public-safe evidence request question. |
| `demand_evidence` | array of strings | yes | Required evidence bundle to request before accepting a claim. |
| `do_not_accept_yet` | string | yes | Current blocker that prevents public claim use. |
| `public_claim_allowed` | boolean | yes | Explicit claim gate for public use. |
| `claim_gate` | string | yes | Human-readable claim-gate label. |
| `use_rule` | string | yes | Boundary rule for using the row. |

At least one of `lane_id` or `program_or_account_id` must be present.

## Fixed Demand Evidence

- source record and source version
- reviewed performance target, outcome measure, audit source, or official finding
- role-approved exact public wording
- public-claim eligibility

## Public-Use Rule

Rows may support evidence requests. They must not be used as findings of fraud,
waste, abuse, legal dedication of income taxes, or poor performance.
