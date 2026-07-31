# SHIELD locates shortage components without painting whole counties

Taxlane replayed SHIELD's same-vintage bridge across the July 31, 2026 HRSA
primary-care HPSA registry. Of 7,682 current designations, 2,838 are area
designations expanding to 15,524 component rows; the other 4,844 are facility
designations with one component row each.

The area designations include 2,088 using Single County components, 586 using
Census Tracts, and 164 using County Subdivisions. Importantly, 762 area
designations span multiple components and 155 span multiple county keys. A
Single County component is therefore not always a single-component or
single-county designation.

The bridge validates common county keys for 7,664 designation IDs across 2,932
distinct codes. Eighteen facility IDs remain visible as geography residuals:
17 have placeholder county keys and one has a state-prefix inconsistency. No
residual was silently repaired from an alternate field.

This advances HLT geography readiness, but county location is not CMS facility
identity, whole-county shortage, unique affected population, patient access,
or staffed capacity. Taxlane therefore admits no candidate effect or savings.
HLT does not reopen. The remaining target stays **$813.727B**, and both
analytical rate schedules remain unchanged.

Machine record:
[`shield_hrsa_primary_care_geography_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_hrsa_primary_care_geography_disposition.v1.draft.json).
