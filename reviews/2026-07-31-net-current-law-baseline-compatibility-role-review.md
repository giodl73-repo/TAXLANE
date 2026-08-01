# NET current-law baseline compatibility role review — 2026-07-31

## Fixed point

- **Source Custodian:** passes. CORE-G retains the CBO raw path, vintage, byte
  count, and SHA-256; the Treasury contexts retain their narrower custody.
- **Budget Accountant:** passes. Annual debt, interest, matching-vintage average
  rates, other financing, and reduced-form feedback are admitted, while
  maturity-aware interest feedback remains blocked.
- **Taxpayer and Public Goods:** passes. The audit changes no spending pot,
  service floor, financing requirement, or rate.
- **Beneficiary and Compliance Burden:** passes as not triggered. No candidate
  effect or administrative burden is inferred.
- **Fiscal Sustainability and Reform Skeptic:** passes. Aggregate average rates
  are used only for reduced-form feedback, not promoted to bucket paths; the
  separate primary-shock fixture is never described as maturity-aware.

The 2026-08-01 fixed point is seven of eight formula inputs and six of nine
completion steps. Reduced-form feedback is testable; the full-stock maturity
result, candidate savings, solver, and rate claims remain blocked.
