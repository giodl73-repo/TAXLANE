# Medicare HI payroll-tax perimeter bridge schema

This schema describes `medicare_hi_payroll_tax_perimeter_bridge` records.

Required fields:

- Identity fields and paths to the target-cost contract, Medicare HI perimeter
  bridge requirements, Medicare HI receipt-base reconciliation, and
  rate-publication readiness rollup.
- Source-custody status booleans.
- CMS evidence rows distinguishing payroll taxes from taxation of OASDI benefits
  and taxable-payroll definition context.
- OMB anchor context with payroll-tax-only perimeter unconfirmed.
- Reconciliation, still-required evidence, blocked outputs, warning phrases, and
  claim booleans.

The record may partially evidence the CMS payroll-tax component. It must not
complete the perimeter bridge or publish an assigned base, rate, solver input,
public rate card, tax proposal, savings estimate, waste/fraud finding,
technology-savings claim, or balanced-budget claim.
