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

`fiscal_debt_dynamics_2026_2036.v1.draft.json` extends those targets across an
annual CBO baseline and adds first-order interest feedback, while preserving a
hard boundary between transparent scenario arithmetic and a policy score.

`fiscal_policy_scale_baskets.v1.draft.json` compares the cumulative paths with
CBO-scored policy-option magnitudes. Its arithmetic baskets demonstrate scale;
they are not additive package scores or recommendations.

`fiscal_policy_distribution_screen.v1.draft.json` maps statutory channels to
likely burden bearers and protection gates without inventing quantified
incidence. A joint microsimulation remains required before package claims.

`payment_integrity_depth_card.fy2024.v1.draft.json` reconciles the official
FY2024 annual workbook's improper, unknown, overpayment, underpayment, and
technically-improper totals while keeping court-confirmed-fraud and agency-
recovery tables parallel until program, period, and definitions match.

`federal_crop_insurance_payment_integrity_bridge.fy2024-q4-2025.v1.draft.json`
reconciles the FCIC annual row, Q4 2025 scorecard, and RMA review-period
evidence. The linked
`federal_crop_insurance_root_cause_definition_bridge.fy2024.v1.draft.json`
adds same-period USDA definitions for the failure-to-access and inability-to-
access categories. The linked
`federal_crop_insurance_payment_universe_bridge.fy2024.v1.draft.json` adds the
official FY2024 payment categories and AIP payment tiers. Four fields are closed
internally while sample design, estimation method, exclusion rules, and
recoverable-savings basis remain open. Every scoring, public-claim, fraud,
waste, recovery, and savings gate
remain blocked. USDA-wide Do Not Pay figures following the FCIC section are not
part of the bridges. Other Information on printed pages 60-61 of the FCIC/RMA
financial statements is unaudited; its apparent $579.93M typo is excluded in
favor of the annual workbook's $573.93M.

`federal_crop_insurance_sample_design_component_bridge.fy2024.v1.draft.json`
adds a narrow component-only decision for the disclosed FY2024 sampling
governance and independent methodology review. It does not change the four-open
field count: frame, allocation, probabilities, randomization, replacement,
nonresponse, weights, estimator, and variance remain undisclosed, and compliance
is not public reproducibility.

`federal_crop_insurance_historical_sampling_method_bridge.fy2020.v1.draft.json`
preserves a separate historical benchmark from FY2020 Other Information. For
reinsurance year 2018, the source discloses simple-random policy selection, all
three named payment categories, and statistically valid rate and dollar
estimates. The section is unaudited, and nothing in it proves that the method
continued through FY2024. It closes no current field, changes no 4/4 count, and
relaxes no scoring or claim gate.

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

