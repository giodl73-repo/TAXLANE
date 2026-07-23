# Taxlane showcase readiness summary schema

`taxlane_showcase_readiness_summary.v1.draft.json` records the current
showcase posture for Taxlane.

Required checks:

- `record_family` is `taxlane_showcase_readiness_summary`.
- `pulse` is `201`.
- The record links the active wave, income-security/family source status rollup,
  solver-input readiness rollup, rate-publication readiness rollup, and final
  closure gate.
- `demo_ready` and `demo_ready_as_readiness_system` may be true.
- Solver input, rate calculation, savings, public rate card, department-cut
  instruction, technology-savings, and balanced-budget claim fields remain
  null/false.
- The public warning states that this is a showcase readiness summary, not a
  solver run, rate calculation, savings estimate, or balanced-budget claim.
