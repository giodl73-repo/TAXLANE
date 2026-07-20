# Net-interest outcome floor definition packet schema

Draft schema for `net_interest_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields tying the packet to the net-interest lane, target-cost
  contract, Pulse 160 outcome-floor gap, revenue-solvency packet precedent,
  net-interest formula contract, and lane-depth tracker;
- source-custody booleans with no new external request or agency/person contact;
- definition-policy guardrails preserving endogenous net interest, no direct
  cuts, null missing values, false blocked gates, and no savings/fraud shortcut;
- five mandatory floor classes and five net-interest-specific floor definitions;
- blocked inputs and blocked outputs, all null;
- summary counts/readiness booleans;
- public warning phrases;
- claim booleans with only `definition_packet_published` true.

Debt paths, maturity paths, rate paths, direct cut amounts, federal effects,
savings, solver inputs, solver runs, department-cut instructions,
technology-savings claims, and balanced-budget claims remain blocked.
