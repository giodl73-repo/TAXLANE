# Defense source readiness gap schema

`defense_source_readiness_gap.v1.draft.json` records that defense source IDs
appear in source-packet context but are not yet supported by local raw custody
for force-structure, readiness, procurement, strategy, or solver use.

Required invariants:

- `record_family` is `defense_source_readiness_gap`.
- `pulse` is `186`.
- The record links the target-cost contract, defense outcome-floor definition
  packet, lane floor source work queue, and lane-depth tracker.
- Defense source IDs may be referenced only as context.
- Raw custody, force-structure plans, readiness floor values, procurement
  schedules, pass/fail findings, solver inputs, rates, target costs, and savings
  remain null/false.
- Only publication and "source references present" booleans may be true.
