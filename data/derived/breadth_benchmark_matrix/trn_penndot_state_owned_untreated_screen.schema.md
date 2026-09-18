# TRN PennDOT state-owned untreated-screen schema

This record builds a complete Pennsylvania state-owned screening denominator
from public RMS roadway-segment and administration services. The segment query
selects state-jurisdiction, rural, two-way, two-lane records; administration
intervals restrict the result to FHWA functional classes 3 through 6.

PennDOT segment-and-offset coordinates are used for both denominator intervals
and the existing-treatment workbook. Treatment intervals are intersected and
split at functional-class boundaries, unioned, and subtracted on the same
basis. The resulting untreated value is therefore a valid state-owned
screening mileage—not a design-eligible inventory.

The record keeps the separate HPMS all-public-road value visible and does not
force the state-owned RMS and HPMS universes to equal. IRI bands are ride-
quality availability measures only. Complete shoulder intervals divide the
untreated screen into unambiguous current-paved-width bands, missing-side
records, and ambiguous or missing width records. Four reported paved feet are
not verified usable clear width beyond a proposed strip. Pavement structure,
usable shoulder, bicycle and noise accommodations, programmed work, target
crashes, current costs, national extrapolation, allocation, funding, and
release remain blocked.

The pavement-availability section reports surface-year and condition-date
vintages, official OPI text labels, field coverage, and shoulder-by-pavement
cross-tabs. Its named cross-field queues prioritize the next engineering-record
review only. OPI, surface year, and IRI cannot substitute for thickness,
cracking, base structure, remaining life, resurfacing commitments, or design
approval.
