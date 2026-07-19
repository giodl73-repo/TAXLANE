# Pulse 128 — Assigned receipt base source gap

## Result

Added a machine-readable source-gap packet for assigned receipt bases.

## Finding

The local IRS HT23 source is source-custodied, but it is a rate/bracket timeline,
not an aggregate AGI, taxable-income, taxable-payroll, or transportation fee
base. No external request was submitted and no agency or person was contacted.

## Boundary

The existing TY2022 illustrative statutory-rate file remains legacy
illustrative only. It cannot feed a solver, rate output, public rate card, tax
proposal, or balanced-budget claim.

## Still blocked

- Matched base amounts.
- Elasticities.
- Incidence and distribution.
- Administration and compliance burdens.
- Current-law and reform yields.
- Statutory and effective rates.
