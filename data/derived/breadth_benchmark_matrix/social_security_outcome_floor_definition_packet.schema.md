# Social Security outcome floor definition packet schema

Draft schema for `social_security_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the Social Security analytical lane, target
  cost contract, Pulse 160 outcome-floor gap, Pulse 161 health packet precedent,
  and lane-depth tracker;
- source-custody booleans with no new external request, no agency/person
  contact, and no populated threshold or pass/fail values;
- definition-policy guardrails preserving separate OASDI trust funds, explicit
  transfers, null missing values, false blocked gates, and no international or
  fraud shortcut;
- five mandatory floor classes and five Social Security-specific floor
  definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Threshold values, observed values, pass/fail findings, demographic scores,
trust-fund solvency scores, target costs, federal effects, savings, assigned
base rates, solver inputs, department-cut instructions, technology-savings
claims, and balanced-budget claims remain blocked.
