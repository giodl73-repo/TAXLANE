# Wave: Accountability Claim Guard

## Objective

Generate a claim guard summary for accountability evidence records.

## Context

Work items are machine-readable, but review leads need a compact summary before
anything is used publicly. The summary should count allowed claims and blockers
without presenting records as fraud, waste, abuse, or performance findings.

## Pulses

| Pulse | Name | Status | Notes |
|---|---|---|---|
| 01 | Claim guard report | done | Added generated claim guard report, stale-output validation, review, manifest coverage, and VTRACE closeout. |

## Acceptance

- Report counts total records, allowed public claims, and blocked claims.
- Report groups records by readiness and public-use blocker.
- Report states safe and unsafe use.
- Validation fails if the report is stale.
