# Disaster Supplemental Tracking Extract

## Purpose

This packet records the first FEMA declaration probe for the disaster
supplemental-tracking queue item.

Machine rows:
`data/derived/efficiency_pressure/extracts/disaster_supplemental_tracking_first_pass.jsonl`.

## Extracted Probe

The first pass locks the 8 most recent FEMA declaration-area rows returned by
the API on 2026-06-30. The rows cover recent fire declarations in Colorado and
Utah, including Aspen Acres Fire, Cherry Fire, Cottonwood Fire, and Iron Fire.

## Boundary

These rows are event and geography markers only. They are not outlays,
obligations, damages, benefit-cost estimates, waste findings, or savings
estimates. The next extract must link declarations to agency accounts,
obligations, awards, outlays, and supplemental-versus-base budget records.
