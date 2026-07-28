# Income-security/family Census child poverty and income capture gap schema

`income_security_family_census_child_poverty_income_capture_gap.v1.draft.json`
records the still-open Census domestic child poverty and income source-custody
gate, with raw Census context custody now captured.

Required checks:

- `record_family` is `income_security_family_census_child_poverty_income_capture_gap`.
- The record links the income-security/family source capture queue and closure
  queue.
- Candidate official Census poverty, SPM, and CPS ASEC source surfaces are
  named.
- Local raw Census report, official-poverty table, SPM table, supporting raw
  files, byte counts, SHA-256 hashes, retrieval date, observed structure, and
  metadata path may be recorded once custody exists.
- Child-poverty floor values, pass/fail findings, solver inputs, rates, savings,
  and balanced-budget claims remain blocked.
