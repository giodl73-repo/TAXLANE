# Taxlane showcase readiness summary schema

`taxlane_showcase_readiness_summary.v1.draft.json` records the current
showcase posture for Taxlane.

The showable frontier includes completed CORE-G through CORE-N, fourteen named
lane tracks at bounded E, one completed TRN cost note, and active REV Level 1.
No solver run or rate calculation occurs. PAY and REV remain
non-additive overlays, while NET remains endogenous. These workflow claims do
not open solver, rate, savings, or balanced-budget outputs.

Required checks:

- `record_family` is `taxlane_showcase_readiness_summary`.
- `pulse` is `412`.
- The record links the active wave, income-security/family source status rollup,
  lane full coverage matrix, solver-input readiness rollup,
  rate-publication readiness rollup, final closure gate, and Wave F calibration.
- `demo_ready`, `demo_ready_as_readiness_system`, and
  `wave_f_deterministic_calibration_ready` may be true.
- The CORE completion claims, fourteen bounded-E tracks, and one typed
  output-ready E closure are true;
  selection-complete, overlay, and endogenous treatment claims are also true.
- The common F contract, fifteen-lane start audit, and two-level
  advancement queue are linked and showable as readiness evidence.
- CORE-N's typed public surfaces and the completed TRN cost note are linked;
  REV Level 1 is shown without implying a matched base or rate.
- Solver input, rate calculation, savings, public rate card, department-cut
  instruction, technology-savings, and balanced-budget claim fields remain
  null/false.
- The public warning states that this is a showcase readiness summary, not a
  solver run, rate calculation, savings estimate, or balanced-budget claim.
