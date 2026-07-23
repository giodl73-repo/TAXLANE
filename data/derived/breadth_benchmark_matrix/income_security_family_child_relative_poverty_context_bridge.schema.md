# Income-security/family child relative poverty context bridge schema

`income_security_family_child_relative_poverty_context_bridge.v1.draft.json`
bridges existing OECD child relative-poverty source custody into the
income-security/family lane.

Required checks:

- `record_family` is `income_security_family_child_relative_poverty_context_bridge`.
- The record links the income-security/family source capture queue and closure
  queue.
- Source custody points to `SRC-OECD-IDD-AGE-POVERTY-PANELS`, the existing raw
  child-poverty CSV, metadata, byte count, and SHA-256.
- The bridge may publish international child relative-poverty context only.
- Census domestic child poverty and income context, floor values, pass/fail
  findings, target costs, solver inputs, rates, savings, and balanced-budget
  claims remain blocked.
