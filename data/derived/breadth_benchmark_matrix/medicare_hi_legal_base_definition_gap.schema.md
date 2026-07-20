# Medicare HI legal-base definition gap schema

Draft schema for
`medicare_hi_legal_base_definition_gap.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-legal-base-definition-gap:v1`
- `record_family = medicare_hi_legal_base_definition_gap`
- `schema_version = v1.draft`
- `pulse = 143`

Required custody fields:

- official source custody booleans.
- explicit no-FOIA/no-records-request/no-contact booleans.
- CMS glossary definition evidence true.
- legal perimeter text, additional Medicare tax treatment, legal-base
  completion, economic-base completion, assigned-base, rate-publication, and
  solver-input booleans all false.

Required analytical fields:

- component id `legal_base_definition`.
- CMS definition evidence rows for payroll taxes, taxable earnings, and taxable
  payroll.
- legal-base gap object with all selected/value fields null and readiness false.
- boundary findings preserving the diagnostic ratio as non-rate context.
- still-required list.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only CMS glossary-term publication true.
