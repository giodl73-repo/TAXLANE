# SHIELD adds an operational hospital spine—not staffed-capacity proof

Taxlane replayed SHIELD's CMS 2023 Hospital Provider Cost Report baseline.
The source has 6,103 unique report records across 6,040 provider CCNs. Of
those, 5,953 reports covering 5,895 CCNs have complete, internally valid
adult-and-pediatric available-bed, bed-day, and inpatient-day observations.
They record 241,546,243 available bed-days and 151,101,088 inpatient days, or
62.56% bed-day-weighted use.

Exact CCN identity matches 5,144 of the 5,432 hospitals in SHIELD's May 13,
2026 footprint. Of the current IDs, 5,032 have at least one usable operational
report—92.64% coverage—with 62.33% weighted use. The 125 missing and 25 invalid
records remain visible. Sixty-two repeated CCNs form 63 adjacent,
non-overlapping reporting-period pairs; point-in-time bed counts are not added
across them.

CMS defines these as beds available for patient use, not staffed beds.
Weighted use therefore does not prove service-line readiness, workforce or
surge capacity, patient access, local need, quality, or adequacy. This result
closes the shared CCN and available-bed-use gap, but it does not define an HLT
intervention or fiscal case.

Taxlane admits no candidate effect or savings. HLT does not reopen. The
remaining target stays **$813.727B**, and both analytical rate schedules remain
unchanged.

Machine record:
[`shield_cms_hospital_operational_capacity_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_cms_hospital_operational_capacity_disposition.v1.draft.json).
