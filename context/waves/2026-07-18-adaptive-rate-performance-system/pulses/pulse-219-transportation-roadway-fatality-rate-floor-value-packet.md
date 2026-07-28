# Pulse 219 — Transportation roadway fatality-rate floor-value packet

## Purpose

Advance the transportation/infrastructure lane's Wave D floor-value work with
one source-custodied roadway-safety baseline without implying complete floor
passage or simulator readiness.

## Changes

- Captured NHTSA DOT HS 813 800 from the official CrashStats endpoint.
- Separated the completed-year 2024 FARS ARF values from the 2025 statistical
  projections.
- Added a draft no-regression fatality-rate ceiling and baseline of 1.19
  fatalities per 100 million vehicle miles traveled.
- Updated the Wave D and lane-floor rollups from eight to nine lanes with draft
  thresholds and sourced baselines.
- Added a validator and focused test for identity, values, source custody,
  projection labeling, null scenario fields, and blocked claim gates.

## Boundary

This packet is one roadway-safety floor slice. It is not a complete
transportation floor, not serious-injury, reliability, asset-condition, access,
equity, resilience, or delivery-feasibility passage, not policy values, not
stress values, not pass/fail evidence, not a simulator run, not solver input,
not rates, not savings, and not a balanced-budget claim.
