# Wave: Accountability Core Workflow

## Objective

Move accountability workflow policy into the Rust core crate.

## Context

Readiness, next action, public-use blocker, and demand-question rules should be
shared by CLI reports and future UI/API surfaces. Keeping this policy in the
core crate avoids duplicating claim-boundary logic.

## Pulses

| Pulse | Name | Status | Notes |
|---|---|---|---|
| 01 | Core workflow helpers | done | Added core helpers and tests, simplified CLI report generation, review, manifest coverage, and VTRACE closeout. |

## Acceptance

- `taxlane-core` exposes next-action, demand-question, and public-use-blocker helpers.
- CLI reports call the core helpers.
- Generated reports remain stable.
- Unit tests cover role-review and missing-evidence workflow paths.
