# Fiscal Accounting And Rate Definitions

Machine record:
`data/derived/breadth_benchmark_matrix/fiscal_accounting_rate_definitions.v1.draft.json`.

Schema:
`data/derived/breadth_benchmark_matrix/fiscal_accounting_rate_definitions.schema.md`.

## Two Separate Shares

| Quantity | Formula | Public meaning |
|---|---|---|
| all-receipt funding share | gross program cost / total funded federal cost | Where the fully funded federal cost goes before dedicated receipts are subtracted. |
| residual general-fund requirement share | residual general-fund need / total residual general-fund need | Which remaining general-fund needs must be covered after dedicated receipts and credited offsets. |

A value calculated after subtracting dedicated receipts is not "share of every
tax dollar." It is a residual general-fund requirement share.

## Frozen Signs

- Outlays are positive.
- Implementation costs are positive outlays.
- Receipts are positive.
- Offsetting collections are positive offsets.
- Deficit is a positive financing need.
- Surplus is positive excess receipts.

## Frozen Identities

```text
primary outlays
  = gross program outlays + implementation outlays

net cash requirement
  = primary outlays - credited offsetting collections

fund balance change
  = dedicated receipts
  + explicit general-fund transfer
  + other scored fund income
  - net cash requirement

primary balance
  = total federal receipts - primary outlays

deficit
  = primary outlays + net interest - total federal receipts

debt[t]
  = debt[t-1] + deficit[t] + explicit other financing[t]
```

## Reserve And Rounding Boundary

Reserve contributions, withdrawals, caps, and emergency overrides are documented
but not numerically parameterized yet. They remain solver-blocking until those
fields are populated.

The solver must use unrounded values. Public rounded displays must send any
rounding residual to an explicit rounding line, not to a program lane, trust
fund, receipt source, or offset row.
