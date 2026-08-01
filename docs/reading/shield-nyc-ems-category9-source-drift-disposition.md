# Category 9 source drift is now characterized—not explained away

Taxlane replayed SHIELD's complete comparison of the two official calendar-2025
Category 9 publication surfaces. Open Data contains 216,986 incidents and the
later-refresh Power BI snapshot contains 216,599, a difference of 387.

The 72-cell month/borough join rules out a localized discrepancy. Open Data is
higher in 54 cells, equal in 18, and lower in none. Every month and all six
borough labels contribute a nonzero aggregate difference; both independent
rollups close to 387. The largest individual cell difference is 23 incidents.

This pattern is consistent with broad cross-snapshot revision, but the publisher
has not exposed a revision mechanism or public row history. Taxlane therefore
does not call either official source wrong. It retains Power BI for the captured
later-refresh headline and Open Data for documented, labelled machine replay.
It never averages, overwrites, or splices the snapshots.

This closes a source-custody question, not a healthcare intervention. No driver,
patient effect, candidate, cost, or savings enters HLT. The revenue target stays
**$813.727B**, and the preferred and contingency schedules remain unchanged.

Machine record:
[`shield_nyc_ems_category9_source_drift_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_nyc_ems_category9_source_drift_disposition.v1.draft.json).
