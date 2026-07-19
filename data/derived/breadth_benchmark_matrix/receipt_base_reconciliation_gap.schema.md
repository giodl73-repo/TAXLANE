# Receipt-base reconciliation gap schema

This schema describes `receipt_base_reconciliation_gap` records.

Required fields:

- Identity fields and paths to the target-cost contract, official source capture
  record, and rate-publication readiness rollup.
- Source-custody status booleans.
- Four reconciliation rows covering IRS individual income, SSA OASDI payroll,
  Medicare HI payroll, and transportation excise/user-fee work items.
- Each row must distinguish captured context from matched assigned-base
  readiness and keep readiness booleans false.
- Summary counts, blocked outputs, warning phrases, and claim booleans.

The record may explain why captured values are not ready. It must not publish
matched assigned receipt bases, rates, solver inputs, public rate cards, tax
proposals, savings estimates, waste/fraud findings, technology-savings claims,
or balanced-budget claims.
