# Wave: Accountability Work Items

## Objective

Generate machine-readable accountability work items from validated evidence
records.

## Context

Markdown reports are useful for review, but future UI and API surfaces need a
structured handoff. Work items should come from `taxlane-core` workflow helpers
so every surface uses the same readiness, blocker, and demand-question rules.

## Pulses

| Pulse | Name | Status | Notes |
|---|---|---|---|
| 01 | Work-items JSONL | done | Added core work-item view, generated JSONL, stale-output validation, review, manifest coverage, and VTRACE closeout. |

## Acceptance

- JSONL has one row per accountability evidence record.
- Each row includes readiness, next action, demand question, blocker, and public-claim allowance.
- Validation fails if the JSONL is stale.
- Current non-public records remain `public_claim_allowed:false`.
