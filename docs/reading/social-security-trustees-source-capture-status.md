# Social Security Trustees source capture status

Machine record:
`data/derived/breadth_benchmark_matrix/social_security_trustees_source_capture_status.v1.draft.json`

Pulse 202 makes concrete source-capture progress for the Social Security lane.
The official SSA 2026 Trustees report page, highlights page, and single-year
table index were browser-verified on 2026-07-23. Local automated byte capture
from this environment returned HTTP 403/Akamai access-denied responses, so the
packet records an access boundary instead of a byte count or SHA-256.

What is now available as context:

- OASI projected reserve depletion year, combined OASDI depletion year, payable
  benefit percentages, and the 75-year actuarial balance from SSA highlights;
- CY2025 rounded OASDI income, cost, reserve, covered-worker, and beneficiary
  context from SSA highlights;
- official table locations for annual rates, worker/beneficiary counts,
  trust-fund ratios, and combined OASDI current-dollar operations.

What remains blocked:

- local SSA raw-byte custody and SHA-256;
- full OASI/DI annual fund paths and FY2025-FY2035 fiscal-year bridge;
- taxable payroll base and assigned receipt-base readiness;
- benefit adequacy, old-age poverty, and administration/transition floor
  values;
- thresholds, pass/fail findings, lower-cost scenario admissibility, solver
  inputs, rates, savings, public cards, technology-savings claims, and
  balanced-budget claims.

This is source progress, not a rate or savings result.
