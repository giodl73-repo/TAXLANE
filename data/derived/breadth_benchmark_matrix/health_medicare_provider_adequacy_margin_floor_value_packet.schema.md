# Health Medicare Provider Adequacy Margin Floor Value Packet Schema

Draft schema for
`health_medicare_provider_adequacy_margin_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the health/Medicare lane,
  adequacy/resilience floor class, health floor-definition packet, health target
  admissibility context, and Wave D readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with the FY2024 relatively efficient hospital median FFS
  Medicare margin, supporting margin/access/quality context, source IDs, and
  source path;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that the packet is not a universal target, not
  access or quality passage, not a federal score, not savings, not solver input,
  not a rate calculation, and not a balanced-budget claim.
