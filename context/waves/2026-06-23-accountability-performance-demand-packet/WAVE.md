# Wave: Accountability Performance Demand Packet

## Objective

Generate a question packet that helps people ask for performance evidence before
accepting public-money claims.

## Context

The action queue groups internal reviewer tasks. TAXLANE also needs a safer
reader-facing bridge: questions that can be asked publicly without alleging
fraud, waste, abuse, or nonperformance.

## Pulses

| Pulse | Name | Status | Notes |
|---|---|---|---|
| 01 | Generated demand packet | done | Added generated performance-demand packet, stale-output validation, review, manifest coverage, and VTRACE closeout. |

## Acceptance

- Packet states what TAXLANE can say now.
- Packet asks a demand question for each evidence record.
- Packet names each claim boundary.
- Validation fails if the generated packet is stale.
