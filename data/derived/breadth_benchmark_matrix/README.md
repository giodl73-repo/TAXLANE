# Breadth And Benchmark Matrix

This family answers three portfolio questions in one controlled record set:

1. Are the important fiscal lanes covered?
2. Do we have a top-line current value and a defensible comparison?
3. Have efficiency gaps, improper payments, fraud, and savings remained
   separate?

Canonical draft rows:
`breadth_benchmark_matrix.v1.draft.jsonl`.

Public scoreboard: `docs/reading/current-versus-benchmark-scoreboard.md`.

The health depth phase begins with
`health_cost_decomposition.v1.draft.json`, which separates price, volume and
intensity, administration, coverage and case mix, and outcomes without treating
different years or denominators as additive savings.

`health_service_price_volume_bridge.cy2024.v1.draft.json` then decomposes
CY2024 growth for hospital, physician/clinical, and retail-drug spending while
keeping unmatched category peer benchmarks blocked.

`health_category_benchmark_ladder.v1.draft.json` records which hospital,
physician, and retail-drug comparisons are matched spending measures, domestic
price references, or mechanism evidence—and why none is yet a savings target.

`health_target_admissibility.v1.draft.json` tests whether Medicare-relative
references can become scenario anchors using current access, margin, quality,
and payment-adequacy evidence; it blocks a universal target.

`health_medicare_relative_scenarios.v1.draft.json` defines low, central, and
high policy paths while blocking dollar scoring until a matched commercial
allowed-spending base and behavioral/access model exist.

`health_commercial_sample_sensitivity.v1.draft.json` applies those paths only
to Milliman's cited analytical data volumes, exposing basis sensitivity while
blocking national and net-savings claims.

`health_national_phi_sensitivity.v1.draft.json` bridges the scenarios to CMS
CY2024 private-insurance hospital and physician/clinical payments, labels the
cross-source result Grade C, and blocks savings and federal-budget claims.

`fiscal_path_scenarios.v1.draft.json` translates CBO's 2036 primary-deficit
baseline into partial-closure, balance, and surplus adjustment equivalents
without claiming that primary balance automatically stabilizes debt.

The matrix deliberately includes coverage-gap rows. An explicit missing value
is more useful than a false benchmark.

The first Tier 2 depth artifact is
`veterans_depth_card.fy2025.v1.draft.json`, which reconciles the complete
Veterans Benefits and Services function and keeps service probes separate from
performance, fraud, and savings claims.

`transportation_depth_card.fy2025.v1.draft.json` reconciles all four federal
transportation subfunctions and blocks peer comparisons until state/local,
trust-fund, asset, delivery, and outcome scopes are matched.

`education_depth_card.fy2025.v1.draft.json` reconciles function 500 and keeps
the negative higher-education net entry blocked behind account-level analysis.

