# Social Security taxable payroll base bridge

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_taxable_payroll_base_bridge.cy2025-2035.v1.draft.json`

Pulse 204 adds official SSA calendar-year taxable payroll base context for the
Social Security lane. It uses SSA Trustees Table VI.G1 for taxable payroll,
GDP, and AWI, and Table V.C1 for the contribution and benefit base through
2035.

What is now usable:

- CY2025-CY2035 OASDI taxable payroll in billions;
- CY2025-CY2035 contribution and benefit base dollar amounts;
- matched GDP and taxable-payroll-to-GDP context;
- current-law combined OASDI payroll tax rate context.

What remains blocked:

- fiscal-year taxable payroll bridge;
- OMB receipt-yield reconciliation;
- reform yield, behavior, incidence, distribution, and administration burden;
- solver receipt rows, rates, public rate cards, savings, technology-savings
  claims, and balanced-budget claims.

This closes a calendar-year base context gap. It does not make the Social Security lane solver-ready or rate-ready.
