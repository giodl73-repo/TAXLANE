# Net Interest Average Rate Floor Value Packet Schema

Draft schema for `net_interest_average_rate_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the net-interest lane, adequacy/resilience
  floor class, net-interest floor-definition packet, Treasury average-rate
  context, OMB PBD current-law context, and Wave D readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with the latest Total Interest-bearing Debt average interest
  rate, supporting Treasury average-rate and OMB net-interest context, source
  custody, and source paths;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that the packet is not a direct cut, not a
  complete FY2025-FY2035 net-interest path, not debt stock, not a maturity
  schedule, not primary-balance feedback, not solver input, not a rate
  calculation, not savings, and not a balanced-budget claim.
