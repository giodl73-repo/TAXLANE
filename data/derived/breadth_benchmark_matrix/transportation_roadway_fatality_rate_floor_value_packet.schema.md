# Transportation Roadway Fatality Rate Floor Value Packet Schema

Draft schema for
`transportation_roadway_fatality_rate_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the transportation/infrastructure lane,
  quality/safety floor class, transportation floor-definition packet, NHTSA
  CrashStats source, and Wave D readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with the reported 2024 FARS ARF fatality rate and count,
  clearly separated 2025 statistical-projection context, source custody, and
  source paths;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that this is not a complete transportation floor,
  not asset-condition or access evidence, not a simulator run, not solver input,
  not a rate calculation, not savings, and not a balanced-budget claim.
