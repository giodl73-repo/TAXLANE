# Fiscal Accounting Rate Definitions Schema

Machine record:
`fiscal_accounting_rate_definitions.v1.draft.json`.

This record freezes the accounting vocabulary required before the integrated
solver can calculate rates. It resolves the old receipt-share ambiguity by
separating two quantities:

- `all_receipt_funding_share`: gross program cost divided by total funded
  federal cost.
- `residual_general_fund_requirement_share`: residual general-fund need divided
  by total residual general-fund need.

A value calculated after subtracting dedicated receipts is not a "share of every
tax dollar." It must be labeled as a residual general-fund requirement share.

Required top-level fields:

- `record_id`, `record_family`, `schema_version`, `as_of_date`, and `status`;
- `contract_path` and `source_paths`;
- `quantity_definitions`;
- `prohibited_language`;
- `sign_conventions`;
- `identities`;
- `reserve_accounting`;
- `rounding_treatment`;
- `solver_boundary`.

The sign conventions are frozen as:

- outlays: positive;
- implementation costs: positive outlays;
- receipts: positive;
- offsetting collections: positive offsets;
- deficit: positive financing need;
- surplus: positive excess receipts.

The required identities are:

```text
primary_outlays = gross_program_outlays + implementation_outlays
net_cash_requirement = primary_outlays - credited_offsetting_collections
fund_balance_change = dedicated_receipts
  + explicit_general_fund_transfer
  + other_scored_fund_income
  - net_cash_requirement
primary_balance = total_federal_receipts - primary_outlays
deficit = primary_outlays + net_interest - total_federal_receipts
debt_t = debt_t_minus_1 + deficit_t + explicit_other_financing_t
```

Reserve numeric parameters remain blocked until contributions, withdrawals,
caps, and emergency overrides are specified. Public rounding residuals must go
to an explicit rounding line.
