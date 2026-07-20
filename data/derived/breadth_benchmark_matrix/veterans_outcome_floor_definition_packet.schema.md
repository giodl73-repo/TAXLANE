# Veterans outcome floor definition packet schema

Draft schema for `veterans_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the veterans lane, target-cost contract,
  Pulse 160 outcome-floor gap, payment-integrity packet precedent, veterans
  depth card, and lane-depth tracker;
- source-custody booleans with no new external request or agency/person contact;
- definition-policy guardrails preserving statutory continuity, earned
  eligibility, null missing values, false blocked gates, and no
  international/fraud shortcut;
- five mandatory floor classes and five veterans-specific floor definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Eligible cohort models, service package models, statutory continuity findings,
target costs, federal effects, savings, solver inputs, department-cut
instructions, technology-savings claims, and balanced-budget claims remain
blocked.
