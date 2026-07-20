# Medicare HI benefits-tax and income split schema

Draft schema for
`medicare_hi_benefits_tax_income_split.v1.draft.json`.

Required identity fields:

- `record_id = medicare-hi-benefits-tax-income-split:v1`
- `record_family = medicare_hi_benefits_tax_income_split`
- `schema_version = v1.draft`
- `pulse = 142`

Required custody fields:

- official source custody booleans.
- explicit no-FOIA/no-records-request/no-contact booleans.
- CMS split evidenced boolean.
- OMB receipt-row mapping, component readiness, perimeter bridge, assigned base,
  rate publication, and solver-input booleans all false.

Required analytical fields:

- component id `taxation_of_benefits_and_other_income_split`.
- source table metadata and FY2025 period.
- CMS income split amounts in millions of dollars.
- full CMS revenue-row list with payroll-tax-yield flag.
- formulas for total revenue, total non-payroll income, other non-payroll
  income, and payroll-tax-yield component membership.
- reconciliation row sum, published total, residual, payroll taxes,
  benefit taxation, other non-payroll income, total non-payroll income, shares,
  component status, and blocked readiness.

Required blocker fields:

- still-required list.
- blocked outputs with null values.
- public warning phrases.
- claim booleans with only `cms_hi_income_split_published` true and all rate,
  solver, savings, fraud, waste, target-cost, federal-effect, technology, and
  balanced-budget claims false.
