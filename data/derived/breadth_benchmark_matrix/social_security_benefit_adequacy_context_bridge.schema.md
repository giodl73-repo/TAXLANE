# Social Security benefit adequacy context bridge schema

`social_security_benefit_adequacy_context_bridge.v1.draft.json` bridges the
existing OECD modeled pension replacement-rate panel into the Social Security
lane.

Required checks:

- `record_family` is `social_security_benefit_adequacy_context_bridge`.
- The record links the Social Security source-capture rollup, Wave D floor-value
  readiness audit, and existing pension replacement country panel.
- Source custody points to `SRC-OECD-PAG-PENSION-REPLACEMENT-PANEL-2024`, the
  existing gross and net raw CSVs, metadata, byte counts, and SHA-256 values.
- The bridge may publish international modeled pension replacement-rate context
  only.
- Domestic benefit adequacy custody, observed current-retiree values, threshold
  rationale, floor values, pass/fail findings, target costs, solver inputs,
  rates, savings, and balanced-budget claims remain blocked.
