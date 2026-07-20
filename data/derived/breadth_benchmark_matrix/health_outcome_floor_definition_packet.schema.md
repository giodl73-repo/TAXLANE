# Health outcome floor definition packet schema

Draft schema for `health_outcome_floor_definition_packet.v1.draft.json`.

Required structure:

- identity fields: `record_id`, `record_family`, `schema_version`, `pulse`,
  `as_of_date`, `lane_id`, and path fields tying the packet to the target-cost
  contract, Pulse 160 outcome-floor gap, and health sensitivity/admissibility
  chain;
- `source_custody_status` booleans with no new external request or contact;
- `definition_policy` guardrails preserving null missing values, false blocked
  gates, PHI/federal separation, international-comparison boundaries, and no
  fraud inference;
- five `required_floor_classes`: access/coverage, quality/safety,
  equity/distribution, adequacy/resilience, and fiscal/delivery feasibility;
- five health-specific floor definitions matching the health lane contract;
- `blocked_inputs` and `blocked_outputs`, all null;
- `summary` counts and readiness booleans;
- reader-facing warning phrases;
- claim booleans with only `definition_packet_published` true.

Threshold values, observed values, pass/fail findings, federal effects, target
costs, savings, solver inputs, rates, technology-savings claims, and
balanced-budget claims remain blocked.
