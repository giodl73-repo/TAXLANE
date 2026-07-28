# Revenue Solvency Total Receipts Floor Value Packet Schema

Draft schema for
`revenue_solvency_total_receipts_floor_value_packet.v1.draft.json`.

Required fields:

- identity fields tying the packet to the revenue-solvency non-additive
  overlay, adequacy/resilience floor class, revenue-solvency floor-definition
  packet, OMB receipt-category context, and Wave D readiness rollup;
- a threshold rationale with selected measure, rule, type, value, unit, source,
  and review status;
- baseline values with FY2025 total receipts, receipt-category context, source
  custody, source ID, and source path;
- null policy values, null stress values, null pass/fail evidence, null
  downstream outputs, and false downstream claim booleans;
- a public warning preserving that the packet is not matched receipt bases, not
  a legal/economic base, not incidence/distribution, not a tax rate, not a tax
  proposal, not solver input, and not a balanced-budget claim.
