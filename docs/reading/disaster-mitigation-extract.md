# Disaster Mitigation Extract

## Purpose

This packet records the first FEMA Hazard Mitigation Assistance Projects probe
for the disaster-mitigation queue item.

Machine rows:
`data/derived/efficiency_pressure/extracts/disaster_mitigation_first_pass.jsonl`.

## Extracted Probe

The first pass locks 5 FEMA HMA project rows returned by the OpenFEMA v4 API on
2026-06-30. The rows include project identifiers, program area, geography,
status, project amount, federal share where reported, cost share, benefit-cost
ratio, and net-value-benefit fields.

The probe shows why mitigation is a cost-down candidate: some records expose
both up-front project amounts and benefit-cost context that can later be paired
with hazards, declarations, and federal outlay/account records.

## Boundary

These rows are project-level source markers only. They are not federal outlay
totals, verified avoided-loss estimates, waste findings, or savings estimates.
The next extract must attach the benefit-cost method, hazard/geography
crosswalk, declaration history, event-to-account bridge, and emergency-response
floor before any scored cost-down claim.
