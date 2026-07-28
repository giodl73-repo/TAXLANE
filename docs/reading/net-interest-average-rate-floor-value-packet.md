# Net Interest Average Rate Floor Value Packet

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_average_rate_floor_value_packet.v1.draft.json`

Source context:
`data/derived/breadth_benchmark_matrix/net_interest_treasury_average_interest_rate_context.v1.draft.json`

Current-law context:
`data/derived/breadth_benchmark_matrix/net_interest_pbd_fy2025_2031_current_law_context_path.v1.draft.json`

This is a Wave D floor-value packet for the net-interest lane. It converts
Treasury latest-month average-interest-rate custody into a draft debt-service
rate-path threshold rationale and baseline value, but it does not pass or fail
any policy scenario.

Draft threshold rule: a net-interest path cannot pass this adequacy/resilience
floor if reviewed policy and stress evidence raise the average rate on total
interest-bearing debt above the latest Treasury baseline before debt stock,
maturity, fiscal-year rate-path, and primary-balance feedback evidence are
ready.

Selected baseline and threshold:

| Field | Value |
| --- | ---: |
| Total Interest-bearing Debt average-rate ceiling | 3.409 percent |
| Total Marketable average interest rate | 3.411 percent |
| Total Non-marketable average interest rate | 3.399 percent |
| FY2025 OMB PBD net-interest outlay context | $970,065 million |
| FY2031 OMB PBD projected net-interest outlay context | $1,363,769 million |

This baseline is net-interest adequacy and resilience context only. Net
interest remains endogenous. It is not a complete FY2025-FY2035 net-interest
path, not a debt-stock projection, not a maturity schedule, not a fiscal-year
rate path, not primary-balance feedback, not a direct net-interest cut, not
policy values, not stress values, not pass/fail evidence, not solver input, not
rate calculation, not a public rate card, not gross savings, not net savings,
and not a balanced-budget claim.

Compact validator phrase: draft no-regression net-interest average-rate floor threshold.
Compact validator phrase: net interest remains endogenous.
Compact validator phrase: Net interest remains endogenous.
Compact validator phrase: policy and stress values remain null.
Compact validator phrase: not a direct net-interest cut.
Compact validator phrase: not a balanced-budget claim.
