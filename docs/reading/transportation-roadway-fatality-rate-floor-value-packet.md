# Transportation Roadway Fatality Rate Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/transportation_roadway_fatality_rate_floor_value_packet.v1.draft.json`

Official source custody:
`data/raw/nhtsa/SRC-NHTSA-CRASHSTATS-813800-2025-EARLY-2024-ARF/2026-07-26/dot-hs-813-800-2025-early-estimate.pdf`

This is a Wave D floor-value packet for the transportation/infrastructure lane.
It converts NHTSA's completed-year 2024 FARS Annual Report File roadway fatality
rate into a draft quality/safety threshold rationale and baseline value, but it
does not pass or fail any policy scenario.

Draft threshold rule: a transportation lower-cost scenario cannot pass this
roadway-safety floor if reviewed policy or stress evidence raises the nationwide
fatality rate above the completed-year baseline of 1.19 fatalities per 100
million vehicle miles traveled.

Selected baseline and threshold:

| Field | Value | Evidence status |
| --- | ---: | --- |
| 2024 roadway fatality-rate ceiling | 1.19 per 100 million VMT | Reported 2024 FARS ARF |
| 2024 roadway fatalities | 39,254 people | Reported 2024 FARS ARF |
| 2025 roadway fatality rate | 1.10 per 100 million VMT | Statistical projection; context only |
| 2025 roadway fatalities | 36,640 people | Statistical projection; context only |

The rate is used instead of the fatality count alone because it controls for
changes in vehicle miles traveled. The 2025 projection is not used as the
baseline and must not be treated as a final FARS value.

This packet is not a complete transportation safety floor, not serious-injury
evidence, not reliability, not asset condition, not access, not equity, not
climate resilience, not delivery feasibility, not policy values, not stress
values, not pass/fail evidence, not a simulator run, not solver input, not a
rate calculation, not savings, and not a balanced-budget claim.

Compact validator phrase: draft no-regression roadway fatality-rate floor threshold.
Compact validator phrase: 2025 values are statistical projections and context only.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a complete transportation floor.
Compact validator phrase: not a balanced-budget claim.
