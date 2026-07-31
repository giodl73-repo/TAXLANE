# SHIELD adds county emergency demand—not a no-access map

Taxlane replayed SHIELD's 2024 CMS Original Medicare county emergency-demand
bridge. The source contains 3,197 county rows, 3,143 usable ED-visit rates, and
54 suppressed or missing rates. Nationally it records 27,732,177 Original
Medicare beneficiaries and 16,377,193 ED visits, or 590.5484 visits per 1,000.

Exact POS county FIPS places 5,300 current hospitals inside the demand surface.
There are 2,435 demand counties with a current hospital and 762 without one.
Among the latter, 708 have numeric demand covering 1,527,795 Original Medicare
beneficiaries and 906,563 ED visits.

That is a substantive planning queue, but not a finding of 762 access failures.
The county is beneficiary residence, not treating-hospital location, and
cross-county travel is unobserved. Original Medicare excludes Medicare
Advantage and non-Medicare populations. Higher utilization is not automatically
unmet need.

Taxlane admits the official demand and facility-location context but no
candidate effect or savings. HLT does not reopen. The remaining target stays
**$813.727B**, and both analytical rate schedules remain unchanged.

Machine record:
[`shield_cms_county_emergency_demand_disposition.v1.draft.json`](../../data/derived/breadth_benchmark_matrix/shield_cms_county_emergency_demand_disposition.v1.draft.json).
