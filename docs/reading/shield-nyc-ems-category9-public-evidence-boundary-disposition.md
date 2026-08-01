# Category 9 public data now identifies the acquisition boundary—not an intervention

Taxlane replayed SHIELD's seven-source Category 9 public-evidence boundary. The
new direct NYC Open Data surface makes the official monthly borough records
machine-readable, but its calendar-2025 rows contain **216,986 incidents**—387
more than the **216,599** in the captured Power BI model. Its share reconstructed
from rounded rows is 39.232%, versus 39.260% in Power BI. The revision reason is
unknown, so Taxlane keeps both sources separate.

The public operations inventory closes several tempting shortcuts. MMR has
citywide ambulance in-service hours, hospital turnaround, call volume, and
end-to-end time, but its 60 borough response rows publish no 2025 values. Other
operations sources do not share the Category 9 qualifying set or incident key.
SPARCS supplies privacy-safe hospital disposition fields but cannot link them to
Category 9 incidents.

Controlling descriptively for borough and month reduces the average-response
association to `r=-0.187` and dispatch to `r=-0.091`; travel remains the largest
signal at `r=-0.481`. This makes incident-linked ALS availability, posting, and
travel exposure the leading acquisition hypothesis—not an identified driver.

No intervention enters candidate design. The next admissible step requires a
stable qualifying-event key, ALS unit availability and posting exposure,
privacy-safe outcomes, a pre-period, and unaffected comparison units under a
pre-specified evaluation. Until then HLT remains closed, the target stays
**$813.727B**, and both analytical rate schedules remain unchanged.

Machine record:
[`shield_nyc_ems_category9_public_evidence_boundary_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_nyc_ems_category9_public_evidence_boundary_disposition.v1.draft.json).
