# Veterans Claims Backlog Floor Value Packet Schema

Draft schema for
`veterans_claims_backlog_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the Veterans lane, claims-timeliness
  floor, Veterans floor-definition packet, Veterans depth card, and Wave D
  readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with observed date, primary backlog probe, supporting quality
  context, source IDs, source paths, and status;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that this is not pass/fail evidence, savings,
  solver input, a rate calculation, or a balanced-budget claim.
