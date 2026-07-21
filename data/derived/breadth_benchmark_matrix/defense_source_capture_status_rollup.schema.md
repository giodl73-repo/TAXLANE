# Defense source capture status rollup schema

`defense_source_capture_status_rollup.v1.draft.json` summarizes the post-Pulse
187 defense source-capture state.

Required invariants:

- `record_family` is `defense_source_capture_status_rollup`.
- `pulse` is `188`.
- The record links the target-cost contract, defense source-readiness gap,
  defense source-capture queue, and defense outcome-floor definition packet.
- Exactly six source families are summarized.
- All six source families remain open capture items.
- Raw custody, context readiness, force-structure plans, readiness floor values,
  procurement schedules, solver inputs, rates, savings, department-cut
  instructions, technology-savings claims, and balanced-budget claims remain
  null/false.
- Only publication booleans for the rollup and its prerequisite records may be
  true.
