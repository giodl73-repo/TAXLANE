# Payment-integrity outcome floor definition packet schema

Draft schema for
`payment_integrity_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the payment-integrity overlay, target-cost
  contract, Pulse 160 outcome-floor gap, net-interest packet precedent,
  payment-integrity depth card, and lane-depth tracker;
- source-custody booleans with no new external request or agency/person contact;
- definition-policy guardrails preserving non-additivity, no fraud inference,
  no savings credit without causal prevention or same-cohort collection lineage,
  null missing values, and false blocked gates;
- five mandatory floor classes and five payment-integrity-specific floor
  definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Fraud findings, waste findings, recoverable savings credits, federal effects,
savings, solver inputs, department-cut instructions, technology-savings claims,
and balanced-budget claims remain blocked.
