# Social Security Old-Age Poverty Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_old_age_poverty_floor_value_packet.v1.draft.json`

This is the first Wave D floor-value packet for Social Security. It converts
the Census domestic 65+ poverty context into a draft threshold rationale and
baseline value, but it does not pass or fail any policy scenario.

Draft threshold rule: a lower-cost or receipt-increasing Social Security
scenario cannot pass the old-age poverty floor if reviewed policy and stress
evidence show the 65+ Supplemental Poverty Measure poverty rate rising above
the 2024 Census baseline.

Selected baseline and threshold:

| Field | Value |
| --- | ---: |
| 65+ SPM poverty threshold ceiling | 15.0 percent |
| 65+ SPM poverty baseline | 15.0 percent |
| 65+ SPM poverty count | 9.223 million |
| 65+ population | 61.490 million |
| 65+ SPM poverty margin of error | 0.5 percentage points |

Supporting context remains visible: official 65+ poverty is 9.9 percent, 65+
income below 125 percent of poverty is 14.2 percent, below 150 percent is 18.9
percent, and below 200 percent is 28.1 percent. Census Table B-7 also records a
Social Security SPM element effect of -20.100 million for people 65+.

This is not role-reviewed final threshold selection, not policy values, not
stress values, not pass/fail evidence, not lower-cost scenario admissibility,
not solver input, not rate calculation, not gross savings, not net savings, and
not a balanced-budget claim.

Compact validator phrase: draft no-regression old-age poverty floor threshold.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a balanced-budget claim.
