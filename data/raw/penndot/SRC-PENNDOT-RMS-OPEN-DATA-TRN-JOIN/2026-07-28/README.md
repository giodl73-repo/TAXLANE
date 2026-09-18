# PennDOT RMS open-data join capture

Captured 2026-07-28 from PennDOT's public ArcGIS REST services. The capture is
restricted to the 1,382 county-route pairs present in the official Statewide
Rumble Strips (2024) workbook, returns attributes only, and preserves each
county query predicate and feature count.

- `roadwaysegments-workbook-route-universe.json`: 44,180 segment features with
  route/segment/direction, rural status, facility type, lanes, pavement and IRI
  fields, traffic, ownership indicators, and bicycle-lane field.
- `roadwayadmin-workbook-route-universe.json`: 22,425 administration intervals
  with segment/offset bounds and PennDOT/FHWA functional class.
- `roadwayshoulder-workbook-route-universe.json`: 56,968 side-specific shoulder
  intervals with current paved and total widths.

Source services:

- `https://gis.penndot.pa.gov/gis/rest/services/opendata/roadwaysegments/MapServer/0`
- `https://gis.penndot.pa.gov/gis/rest/services/opendata/roadwayadmin/MapServer/0`
- `https://gis.penndot.pa.gov/gis/rest/services/opendata/roadwayshoulder/MapServer/0`

This is not the full Pennsylvania roadway universe. Routes with no treatment
workbook record are intentionally absent, so the capture can audit treatment
matches but cannot supply an untreated denominator.
