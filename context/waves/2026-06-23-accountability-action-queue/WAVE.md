# Wave: Accountability Action Queue

## Objective

Generate an internal action queue for accountability evidence records.

## Context

The readiness report now includes deterministic next actions. TAXLANE needs a
task-oriented view so reviewers can work records by evidence gap, role-review
need, or public-wording preparation without turning the records into public
claims.

## Pulses

| Pulse | Name | Status | Notes |
|---|---|---|---|
| 01 | Generated action queue | done | Added generated action queue, stale-output validation, review, manifest coverage, and VTRACE closeout. |

## Acceptance

- Queue groups records by next reviewer task.
- Queue names each record's public-use blocker.
- Validation fails if the generated queue is stale.
- No queue text claims fraud, waste, abuse, or poor performance.
