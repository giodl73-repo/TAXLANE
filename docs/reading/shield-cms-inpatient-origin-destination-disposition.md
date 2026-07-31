# SHIELD observes cross-ZIP inpatient flow—not emergency access

Taxlane replayed SHIELD's 2024 CMS Hospital Service Area origin-destination
baseline. The source contains 1,156,702 hospital/beneficiary-mailing-ZIP pairs:
146,996 numeric and 1,009,706 suppressed. Exact CCN identity joins 5,902 of
7,536 HSA providers to same-year Q4 Provider of Services hospital locations.

The exact join retains 13,330,744 observable cases. After preserving 276 cases
with an invalid origin ZIP, 13,330,468 can be classified: 11,586,529, or 86.92%,
have a beneficiary mailing ZIP different from the hospital ZIP.

That is substantive evidence that hospital care pathways commonly extend
beyond local facility co-location. It is not a county crossing, road distance,
travel time, emergency-department destination, unique-patient count, reason for
travel, burden, or access failure. Suppressed pairs remain suppressed.

Taxlane admits the official inpatient-flow context but no candidate effect or
savings. HLT does not reopen. The remaining target stays **$813.727B**, and both
analytical rate schedules remain unchanged.

Machine record:
[`shield_cms_inpatient_origin_destination_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_cms_inpatient_origin_destination_disposition.v1.draft.json).
