# TRN rumble-strip screening denominator schema

This record extracts the U.S. totals from FHWA Highway Statistics 2024 tables
HM-20 and FI-20 under raw-byte, SHA-256-verified repository custody. It sums
rural other principal arterials, minor arterials, major collectors, and minor
collectors into a broad screening frame.

The screening frame is deliberately not called eligible mileage. HM-20 reports
route miles by rural functional system, not two-lane, undivided, untreated,
pavement-feasible, user-accommodated segments. FI-20 reports all fatalities on
those functional systems, not the crash types affected by centerline or shoulder
rumble strips.

`mechanical_historical_cost_product_billions` is the arithmetic product of the
broad route-mile screen and the dossier's historical $3,800-per-mile input. It
is neither a candidate cost nor an upper bound because current prices,
eligibility exclusions, maintenance, accommodations, pavement work,
administration, and existing treatments are unresolved.
