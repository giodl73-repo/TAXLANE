# Integrated Fiscal Solver Schema

Schema for `integrated_fiscal_solver.v1.draft.json`.

The record is a narrow deterministic scaffold, not an optimizer and not a
balanced-budget claim.

Required sections:

- Identity, contract, rate-model, allocation-model, and debt-dynamics paths.
- `horizon_years_plus_baseline`: baseline year plus ten years.
- Sign conventions and accounting identities.
- Separate funds: OASDI, Medicare HI, transportation trust, general fund, and
  reserves.
- FY2025 reconciliation for the 17-row ledger.
- `budget_rows`: exactly 17 rows, including the two negative offset rows.
- `debt_interest_path`: baseline plus ten years with primary balance, net
  interest, total deficit, debt path, and explicit other financing.
- `reserve_path`, `fund_balance_outputs`, `primary_change_regression_fixture`,
  and `blocked_outputs`.

Missing reserve, trust-fund balance, assigned-base rate, and distributional
values must remain null until their source paths and rules exist.

The regression fixture must show that a primary-balance improvement changes
subsequent debt and net interest. A primary change that leaves both unchanged
fails validation.
