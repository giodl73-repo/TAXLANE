# Income-security/family Census child poverty and income capture gap schema

`income_security_family_census_child_poverty_income_capture_gap.v1.draft.json`
records the still-open Census domestic child poverty and income source-custody
gate.

Required checks:

- `record_family` is `income_security_family_census_child_poverty_income_capture_gap`.
- The record links the income-security/family source capture queue and closure
  queue.
- Candidate official Census poverty, SPM, and CPS ASEC source surfaces are
  named.
- Local raw artifact paths, byte counts, SHA-256 hashes, retrieval dates, and
  metadata paths remain null until source custody exists.
- Child poverty values, SPM values, deep/near poverty, income-unit perimeter,
  floor values, solver inputs, rates, savings, and balanced-budget claims remain
  blocked.
