# Revenue-solvency outcome floor definition packet schema

Draft schema for
`revenue_solvency_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the revenue-solvency overlay, target-cost
  contract, Pulse 160 outcome-floor gap, prior packet precedent, assigned
  receipt-base inventory, distribution placeholder, and lane-depth tracker;
- source-custody booleans with no new external request or agency/person contact;
- definition-policy guardrails preserving non-additivity, null missing values,
  false blocked gates, rate-publication prerequisites, and no savings/fraud
  shortcut;
- five mandatory floor classes and five revenue-solvency-specific floor
  definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Receipt bases, statutory rates, effective rates, assigned-base rates, tax
proposals, federal effects, savings, solver inputs, department-cut instructions,
technology-savings claims, and balanced-budget claims remain blocked.
