# Income-security/family childcare and family-service capture gap schema

`income_security_family_childcare_family_service_capture_gap.v1.draft.json`
records the still-open HHS/ACF childcare and family-service source-custody gate.

Required checks:

- `record_family` is `income_security_family_childcare_family_service_capture_gap`.
- The record links the income-security/family source capture queue and closure
  queue.
- Candidate official ACF CCDF and TANF source surfaces are named.
- Local raw artifact paths, byte counts, SHA-256 hashes, retrieval dates, and
  metadata paths remain null until source custody exists.
- Childcare access floors, work/care transition context, delivery feasibility,
  solver inputs, rates, savings, and balanced-budget claims remain blocked.
