# TRN HPMS rumble-strip suitability-availability schema

This record audits how much of the existing 2024 HPMS rural, two-way,
two-through-lane screen has reported fields that may assist later rumble-strip
design review. It does not estimate nationally eligible or untreated mileage.

`shoulder_width_triage` partitions the full filtered mileage into records with
both reported shoulder widths at least four feet, records with a reported width
below four feet, and records missing either side. FHWA's four-foot reference is
usable width beyond a rumble strip, while the HPMS fields are reported shoulder
widths. The bands are therefore triage and availability measures, not pass/fail
design rules. Modified designs may also be appropriate in constrained settings.

`iri_triage` uses FHWA's good/fair reporting threshold of 170 inches per mile.
IRI measures ride quality; it does not establish pavement depth, structural
condition, remaining life, or milling suitability. Missing fields are reported
as missing and never converted to ineligibility.

`national_existing_treatment_inventory` records only that no rumble-named field
appears in the captured HPMS metadata and that this review did not identify a
national installed-mile inventory. It does not claim that no inventory exists.
Road-owner treatment records, pavement and geometry review, bicycle and noise
accommodations, target-crash exposure, current costs, and delivery review remain
necessary before a candidate can be scored or funded.
