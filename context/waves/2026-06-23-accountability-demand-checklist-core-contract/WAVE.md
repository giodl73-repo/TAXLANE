# Accountability Demand Checklist Core Contract Wave

## Goal

Move the machine-readable performance demand checklist row shape into
`taxlane-core` so downstream UI/API surfaces can reuse the same contract.

## Scope

- Add `PerformanceDemandChecklistRow` to `taxlane-core`.
- Switch CLI JSONL generation to the core method.
- Add a core unit test for blocked claim state and demand evidence.
- Close VTRACE work package `WP-TAX-021`.

## Status

Complete.
