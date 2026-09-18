# TRN HPMS two-way, two-lane screen schema

This record applies a reproducible Socrata query contract to the official 2024
HPMS Spatial All Sections dataset. It selects the 50-state rural records
(`urban_id=99999`) in functional systems 3 through 6, then retains two-way
roadways (`facility_type=2`) with two through lanes (`through_lanes=2`).

The record reports two deliberately different screens:

- `raw_hpms_filter.section_miles` is the sum of spatial full-join section
  lengths returned by the official dataset.
- `calibrated_screen.miles` applies each HPMS within-functional-system filtered
  share to the corresponding official HM-20 route-mile total.

Neither value is official two-lane mileage or design-eligible mileage. FHWA
warns that spatial full-join aggregation may differ from official Highway
Statistics totals, and the section data do not resolve existing treatments,
pavement suitability, user accommodations, noise, target crashes, current
prices, or delivery capacity. Historical cost products remain explicitly
blocked from candidate-cost use.
