# Social Security source capture status rollup

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_source_capture_status_rollup.v1.draft.json`

Pulse 206 consolidates the new Social Security source-capture progress. The
lane is no longer just a queue: it now has partial current-law baseline context,
partial taxable-payroll base context, and an OMB/SSA receipt boundary.

What moved forward:

- combined OASDI FY2025-FY2035 fiscal-year path context;
- 2026 Trustees solvency highlight context and table inventory;
- CY2025-CY2035 taxable payroll and wage-base context;
- OMB FY2025 OASDI receipt anchor versus SSA CY2025 taxable-payroll yield
  boundary;
- OECD modeled pension replacement-rate context for benefit adequacy boundary;
- OECD old-age relative-poverty context for the United States and peer
  comparison boundary;
- Census domestic 65-plus official poverty, SPM poverty, Social Security SPM
  element-effect, and official income-to-poverty ratio context;
- browser-visible SSA service-channel and processing-time context, with the
  command-line access boundary preserved.

What remains blocked:

- local SSA raw-byte custody;
- separate OASI and DI annual paths;
- full 75-year normalized solvency path;
- calendar-to-fiscal bridge and OMB row-perimeter reconciliation;
- domestic benefit adequacy custody, observed current-retiree benefit values,
  and replacement-adequacy threshold rationale;
- old-age poverty and administration/transition floor values;
- old-age poverty measure selection, income-unit boundary review, threshold
  rationale, and pass/fail evidence;
- complete administration transition capacity values, payment accuracy context,
  staffing/workload context, and raw-byte custody;
- pass/fail findings, solver inputs, rates, public cards, savings,
  technology-savings claims, and balanced-budget claims.

This rollup makes Social Security current-law baseline and receipt/rate bridge
coverage partial, not complete. The old-age poverty bridge is international
relative-poverty plus Census domestic 65-plus context only, not domestic floor
values.

Compact validator phrase: international relative-poverty context only.
Compact validator phrase: international modeled pension replacement-rate context only.
Compact validator phrase: Census domestic 65-plus old-age poverty context only.
Compact validator phrase: browser-visible SSA service context only.
