# Income-security/family SOCX family-benefit comparator bridge schema

`income_security_family_socx_family_benefit_comparator_bridge.v1.draft.json`
bridges existing OECD SOCX family-benefit source custody into the
income-security/family lane.

Required checks:

- `record_family` is `income_security_family_socx_family_benefit_comparator_bridge`.
- The record links the income-security/family source capture queue and closure
  queue.
- Source custody points to `SRC-OECD-SOCX-OLDAGE-FAMILY-PANEL-2022`, the raw
  SOCX CSV, metadata, byte count, and SHA-256.
- The bridge may publish SOCX public family-benefit total, cash, and in-kind
  service context only.
- Tax-credit composition, childcare participation, ESSPROS/ILO context,
  child-outcome linkage, target costs, solver inputs, rates, savings, and
  balanced-budget claims remain blocked.
