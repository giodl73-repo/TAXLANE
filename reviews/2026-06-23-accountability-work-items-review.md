# Accountability Work Items Review

## Scope

This review covers the generated machine-readable accountability work items:

| Artifact | Role |
|---|---|
| `data/derived/accountability_evidence/accountability-work-items.jsonl` | UI/API-ready accountability workflow handoff. |
| `crates/taxlane-core/src/lib.rs` | Serializable work item view generated from evidence records. |
| `tools/taxlane/src/main.rs` | JSONL generation and stale-output validation. |

## Decision

Accept the work-items JSONL as the machine-readable handoff for future
accountability surfaces.

Each row carries readiness, next action, demand question, public-use blocker,
and `public_claim_allowed`. Current draft records remain blocked from public
claims.

## Role Findings

| Role | Finding |
|---|---|
| Maintainer | Pass: future UI/API code can consume JSONL instead of scraping Markdown. |
| Reform Skeptic | Pass: `public_claim_allowed` is explicit and false for current records. |
| Taxpayer Advocate | Pass: demand questions remain available in a structured handoff. |
| Source Custodian | Pass: work items are generated only after record shape and source IDs validate. |

## Guardrails

- Treat JSONL work items as workflow state, not proof of fraud, waste, abuse, or performance failure.
- Do not allow UI/API code to infer public eligibility from missing blockers; use `public_claim_allowed`.
- Keep work item generation tied to `taxlane-core` helpers.
