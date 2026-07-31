# SHIELD locates the hospital footprint by county class—not by healthcare need

Taxlane replayed SHIELD's deterministic join between the May 2026 CMS hospital
footprint and USDA ERS 2023 Rural-Urban Continuum Codes. The join matches 5,360
of 5,432 facilities, or 98.67%, without fuzzy matching or manual aliases. It
leaves 72 source-interface residuals explicit and unallocated.

Of the matched facilities, 3,456 are in metro counties and 1,904 are in
nonmetro counties. The latter include 1,086 of 1,371 matched Critical Access
Hospitals and 36 of all 41 matched Rural Emergency Hospitals.

This is meaningful distribution evidence, but RUCC classifies counties—not
patients, travel paths, catchments, staffing, service availability, formal
shortage, need, quality, outcomes, or adequacy. Taxlane therefore admits the
official rurality distribution as HLT context while admitting no candidate
effect or savings. HLT does not reopen. The remaining target stays
**$813.727B**, and both analytical rate schedules remain unchanged.

Machine record:
[`shield_cms_usda_rurality_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_cms_usda_rurality_disposition.v1.draft.json).
