# Social Security OASDI FY2025-FY2035 current-law path

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_oasdi_fy2025_2035_current_law_path.v1.draft.json`

Pulse 203 adds an official SSA fiscal-year path for the combined OASI and DI
Trust Funds. The values come from SSA 2026 Trustees Report Table VI.C6,
`Operations of the Combined OASI and DI Trust Funds, Fiscal Years 2021-2035`.

What is now usable:

- FY2025 historical combined OASDI income, cost, reserve, net-change, and
  trust-fund ratio values;
- FY2026-FY2035 intermediate-assumption fiscal-year rows;
- explicit nulls where SSA does not display reserve-dependent fields after
  projected reserve depletion.

What remains blocked:

- local raw-byte custody and SHA-256;
- a complete separate OASI and DI split path;
- taxable payroll base and receipt/rate bridge;
- floor thresholds, floor values, and pass/fail findings;
- solver inputs, rates, public rate cards, savings, technology-savings claims,
  and balanced-budget claims.

This is a current-law path input candidate with a documented access boundary,
not a solver-ready Social Security lane.
