# Accountability Performance Demand Response Intake Core Contract Review

## Scope

Reviewed the Rust core response intake contract and guardrail tests.

## Findings

- `taxlane-core` now exposes a typed response intake record and response class
  enum for future UI/API importers.
- Validation requires custody fields, a `YYYY-MM-DD` received date, sender,
  missing evidence, and the canonical intake use rule.
- Validation keeps `role_review_needed` true and `public_claim_allowed` false
  for intake records, blocking public-claim bypasses.
- Tests cover a valid partial-evidence intake row, a blocked public-claim
  bypass, and a process-only reply that incorrectly lists evidence.

## Decision

Accepted as the reusable intake contract. It moves reply-intake guardrails into
the core crate without changing current response-log status or public-claim
eligibility.
