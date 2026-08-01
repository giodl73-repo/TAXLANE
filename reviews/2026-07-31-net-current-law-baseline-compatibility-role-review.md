# NET current-law baseline compatibility role review — 2026-07-31

## Fixed point

- **Source Custodian:** passes. CORE-G retains the CBO raw path, vintage, byte
  count, and SHA-256; the Treasury contexts retain their narrower custody.
- **Budget Accountant:** passes. Annual debt, interest, and other financing are
  admitted, while maturity-aware interest feedback remains blocked.
- **Taxpayer and Public Goods:** passes. The audit changes no spending pot,
  service floor, financing requirement, or rate.
- **Beneficiary and Compliance Burden:** passes as not triggered. No candidate
  effect or administrative burden is inferred.
- **Fiscal Sustainability and Reform Skeptic:** passes. Aggregate average rates
  are not promoted to bucket paths, and the zero-policy replay is not described
  as a primary-shock feedback test.

The fixed point is partial NET baseline readiness: three of eight formula
inputs and four of nine completion steps are supported. Solver, savings,
interest-feedback, and rate claims remain blocked.
