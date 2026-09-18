# TRN PennDOT existing-rumble-strip inventory schema

This record audits PennDOT's official 2024 statewide rumble-strip workbook as
the first road-owner inventory pilot. It preserves treatment rows, interval
line-mile sums, within-type unions, date semantics, and offset anomalies.

The `candidate_setting_triage` retains records marked both-direction/two-way,
two-lane, and centerline, shoulder, or edgeline. Its union is an analytical
treatment footprint. It is not official route-mile inventory: the workbook can
hold multiple strip lines per roadway interval and does not contain the
rural/urban or federal-functional-system fields needed to match the national
HPMS screen.

The Pennsylvania HPMS total is displayed but never mechanically reduced by the
workbook footprint. A PennDOT RMS/LRS join must first align road ownership,
location, functional class, geometry, pavement, user constraints, and target
crashes while reporting unmatched records. The record prohibits state-to-
national extrapolation, causal interpretation of PennDOT's descriptive public
trend, candidate costing, allocation, funding, and release.
