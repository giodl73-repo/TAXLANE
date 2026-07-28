# Lane Full Coverage Matrix

Machine record:
`data/derived/breadth_benchmark_matrix/lane_full_coverage_matrix.v1.draft.json`.

This packet is the single reader-facing view of full-coverage status across
exactly 15 analytical lanes. It does not declare any lane complete. It makes
the nine gates visible for every lane and keeps every blocked output null until
source custody, values, floors, scenarios, transition models, solver mappings,
receipt/rate bridges, and claim boundaries are ready.

## The Nine Gates

Each lane is checked against nine gates:

| Gate | Completion requirement |
|---|---|
| Current-law baseline | FY2025-FY2035 annual path from official sources, with component and fund treatment where relevant. |
| Source custody | Source ID, publisher, URL, retrieval date, byte count or access boundary, SHA-256 when local bytes exist, and review status. |
| Public explainer | Plain-language answers for what the lane does, what taxpayers pay now, who is served or protected, outcomes, overspending/underfunding boundary, technology transition, evidence gaps, and blocked claims. |
| Outcome floors | Threshold rationale, baseline values, policy values, stress values, and pass/fail evidence. |
| Policy scenarios | Policy instrument, phase-in, behavior, transition/admin cost, incidence, score provenance, and floor results. |
| Transition model | Implementation, training, cybersecurity/privacy/fallback where relevant, service-risk, stress, and measured productivity or explicit no-credit boundary. |
| Solver mapping | Lane row mapping, fund treatment, interactions, null policy, and blocked outputs. |
| Receipt/rate bridge | Legal/economic base, payer universe, distribution/incidence, administration burden, and current-law/reform yield. |
| Claim boundary | Explicit allowed, blocked, and still-null claims. |

Statuses are `missing`, `partial`, or `complete`. A `partial` gate means there
is some evidence or a contract artifact, not that the gate can be used for
rates, savings, solver output, or target costs.

Current aggregate gate counts: 15 lanes have partial current-law baseline coverage, 15 lanes have partial source-custody coverage, 15 public explainers
are complete, 15 claim boundaries are complete, 15 outcome-floor gates are
partial, all 15 lanes have one source-custodied anchor threshold and baseline,
and 15 policy-scenario gates are partial.

Transition models remain partial for 2 lanes and missing for 13 lanes.
Solver mapping remains partial for 7 lanes and missing for 8 lanes.
Receipt/rate bridge coverage remains partial for 4 lanes and missing for 11 lanes.

Wave C now marks the public-explainer gate complete for all 15 analytical
lanes through
`data/derived/breadth_benchmark_matrix/public_explainer_wave_c_promotion.v1.draft.json`.
That completion is limited to explainability; it does not complete lane depth,
current-law values, source custody, floors, scenarios, solver mapping, or
receipt/rate readiness.

Wave D now completes its lane-anchor contract by attaching floor-definition
packets, descriptive baseline context, and one source-custodied draft threshold
and baseline to the outcome-floor gate for all 15 analytical lanes through
`data/derived/breadth_benchmark_matrix/outcome_floor_wave_d_value_readiness.v1.draft.json`.
That is not full component-floor completion. Wave E now supplies reference
policy/stress calibration values and comparator results, but real policy
performance, lower-cost scenarios, solver inputs, rates, and savings remain
blocked.

Wave E now completes one current-policy continuation reference calibration for
all 15 analytical lanes through
`data/derived/breadth_benchmark_matrix/lane_scenario_pack_wave_e_readiness.v1.draft.json`.
All reference packs, identity-projected policy values, synthetic stress values,
and comparator results are ready. Real reform instruments, empirical behavior,
transition/admin costs, incidence, federal effects, lower-cost admissibility,
solver inputs, rates, and savings remain blocked.

Wave F now records its completed deterministic calibration and the separate solver/rate prerequisite boundary through
`data/derived/breadth_benchmark_matrix/solver_rate_wave_f_readiness.v1.draft.json`.
That is calibration readiness, not substantive solver/rate readiness. The matrix has zero ready substantive Wave F prerequisites, one deterministic transportation dry run, zero solver-ready
lanes, zero public rates, and zero public rate cards.

## Current Matrix Result

Transportation is the deepest pilot. It has the richest current set of
baseline, custody, floor-contract, modernization, stress, trust-fund, and
receipt-base work, but it still lacks FY2032-FY2035 baseline values,
trust-fund reconciliation, floor thresholds, policy/stress values, and
simulator clearance. OMB Table 13-4 now supplies FY2025-FY2031 Highway Trust
Fund and Airport and Airway Trust Fund context rows; it does not supply
FY2032-FY2035 or a solver-ready fund-balance path.

The OMB PBD 17-row context path now supplies FY2025-FY2031 current-law outlay
context for the existing ledger categories, including national defense,
health, income security, veterans, transportation, education, disaster,
justice/general government, science/energy/environment, agriculture, and
international affairs. That evidence improves current-law baseline visibility,
but it still does not complete any lane because FY2032-FY2035, component/fund
treatment, floors, scenarios, solver mapping, and receipt/rate bridges remain
open.

The CBO open-data extension context now supplies official FY2032-FY2035
top-line budget, revenue, debt, net-interest, and selected trust-fund balance
context. That is real source-custody progress, but it is not an OMB 17-row lane
ledger, not trust-fund income/outgo reconciliation, and not solver input.
CBO major outlay-category context also supplies FY2026-FY2035 category values
for several lanes, but those values remain outside the OMB 17-row ledger,
policy-scenario, floor-value, solver, rate, and savings gates.

The CBO revenue-detail context now supplies official FY2026-FY2035 receipt
category values for the revenue-solvency overlay. Those values improve receipt
context visibility, but they are not matched legal/economic bases, not
incidence or distribution models, not current-law or reform yields, not a rate
bridge, and not solver input.

OMB Historical Table 2.1 context now supplies FY2025-FY2031 fiscal receipt
category values for individual income taxes, corporation income taxes, social
insurance and retirement receipts, excise taxes, other receipts, and total
receipts. These values are receipt-category context only; they are not assigned
bases, not rate denominators, and not solver inputs.

IRS SOI Publication 1304 Table 1.1 TY2023 context now supplies source-custodied
individual-income AGI, taxable-income, and income-tax-after-credits values. The
IRS listing check found Table 1.1 listed through TY2023, so no TY2024 Table 1.1
value is used. These values are not a matched FY2025 assigned base and do not
open rate, solver, tax-proposal, or balanced-budget gates.

Revenue solvency and payment integrity are non-additive overlays. They may not
be added to spending lanes as ordinary outlay cuts or savings credits. Revenue
solvency remains blocked by matched receipt bases, behavior, incidence,
distribution, administration, yields, interactions, and review. Payment
integrity remains blocked by causal-prevention or same-cohort collection
lineage, due-process floors, false-positive floors, access floors, and control
costs.

Net interest is endogenous. It cannot be cut directly. It needs debt stock,
maturity schedule, rate path, primary-balance feedback, and a deterministic
feedback fixture before solver use.

Social Security now has partial current-law baseline context, partial source
custody context, and partial taxable-payroll receipt-base context through the
Social Security source-capture rollup. That is real coverage progress, but it
does not clear the lane for solver, rate, savings, or adequacy claims because
separate OASI/DI paths, complete post-depletion fields, floor values,
calendar-to-fiscal receipt conversion, incidence, administration, and review
remain incomplete.

Outcome-floor coverage is partial for all 15 lanes even though Wave D's
lane-anchor contract is complete: every lane has a floor-definition packet,
descriptive baseline context, and one source-custodied draft threshold and
baseline. The full outcome-floor gate remains incomplete because complete
component thresholds and baselines plus actual reform policy/stress performance
and complete-floor pass/fail evidence are absent.

Policy-scenario coverage is still partial for full-coverage purposes even
though all 15 Wave E reference scenario packs are ready. Every lane has a
current-policy continuation value, a synthetic adverse stress value, and a
comparator result. No lane has a ready reform scenario or federal effect.
Science, energy, and environment remain split; agriculture remains split;
international affairs remains split; revenue solvency and payment integrity
remain non-additive overlays; net interest remains endogenous.

Solver/rate readiness remains fully blocked. Wave F records current-law paths,
source custody, floor values, policy scenarios, transition costs, receipt
bases, distribution/incidence, payment-integrity lineage, net-interest
feedback, and reserve parameters as blockers. It does not make missing solver mapping or receipt/rate bridge gates partial.

All lanes remain partial or missing across at least one required gate. The
matrix is not solver-ready, not rate-ready, not savings-ready, and not balanced-budget-ready.

## Public Boundary

This matrix is coverage visibility only. It is not a solver run, not a target
cost selection, not a rate calculation, not a public rate card, not a tax
proposal, not a savings estimate, not a waste finding, not a fraud finding, not
a department-cut instruction, not a technology-savings claim, and not a
balanced-budget claim.
