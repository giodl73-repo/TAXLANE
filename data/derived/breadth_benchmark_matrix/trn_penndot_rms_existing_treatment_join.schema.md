# TRN PennDOT RMS existing-treatment join schema

This record joins the official PennDOT rumble-strip workbook to public RMS
roadway-segment, administration, and shoulder attributes. Exact segment keys
use county, state route, segment, and direction. Administration and shoulder
records are matched with segment-and-offset interval containment or overlap.

The comparable treated footprint requires unique segment and administration
matches, state jurisdiction, rural status, two-way facility type, two lanes,
and FHWA functional classes 3 through 6. Overlapping treatment intervals are
unioned within roadway segment-direction keys.

This is a treatment-selected capture: only county-route pairs appearing in the
workbook were queried. Consequently, the footprint cannot be subtracted from
HPMS or used to estimate untreated mileage until the complete state-owned RMS
universe is captured and reconciled. Pavement and shoulder fields describe
already-treated corridors and do not establish structural suitability or usable
width for untreated roads. National extrapolation, costing, allocation,
funding, and release remain prohibited.
