# HPMS Spatial All Sections 2024 — TRN screening capture

Source dataset: `42um-tgh5`, *HPMS Spatial All Sections - 2024*, published on
the U.S. Department of Transportation Data Portal. Captured 2026-07-28 through
the portal's Socrata API.

All aggregate requests use this base predicate:

```text
urban_id=99999
and stateid != 72
and f_system in (3,4,5,6)
and facility_type=2
and through_lanes=2
```

The files preserve official API responses rather than reconstructed values:

- `dataset-metadata.json`: dataset metadata and field definitions.
- `rural-screening-functional-system-totals.json`: rural section rows and
  miles by functional system before the two-way/two-lane conditions.
- `rural-two-way-two-lane-functional-system-totals.json`: filtered rows and
  miles by functional system.
- `rural-two-way-two-lane-state-totals.json`: filtered rows and miles by state.
- `suitability-field-completeness.json`: filtered total rows/miles and non-null
  counts for left/right shoulder width, IRI, PSR, cracking percent, surface
  type, and lane width.
- `suitability-shoulder-bands.json`: rows/miles grouped by both reported
  widths at least four feet, one or both reported widths below four feet, or a
  missing width.
- `suitability-iri-bands.json`: rows/miles grouped by reported IRI at or below
  170, reported IRI above 170, or missing IRI.
- `suitability-by-functional-system.json`: rows/miles and reported-mile sums
  for both shoulders, IRI, cracking, and surface type by functional system.

The suitability aggregates implement data-availability and triage bands only.
They do not turn a missing field into a failed design test, verify usable clear
width beyond a rumble strip, establish pavement structure, or identify an
existing treatment. The derived builder records and verifies every captured
file's SHA-256 checksum.
