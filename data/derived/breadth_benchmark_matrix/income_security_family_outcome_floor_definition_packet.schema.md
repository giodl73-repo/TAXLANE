# Income-security/family outcome floor definition packet schema

Draft schema for
`income_security_family_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the income-security/family analytical lane,
  target-cost contract, Pulse 160 outcome-floor gap, defense packet precedent,
  and lane-depth tracker;
- source-custody booleans with no new external request or agency/person
  contact;
- definition-policy guardrails preserving null missing values, false blocked
  gates, benefit-package/take-up prerequisites, no international shortcut, and
  no fraud inference;
- five mandatory floor classes and five lane-specific floor definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Threshold values, observed values, pass/fail findings, benefit-package models,
take-up models, federal scores, target costs, savings, solver inputs,
department-cut instructions, technology-savings claims, and balanced-budget
claims remain blocked.
