---
title: "From Spending Evidence to an Adaptive Rate"
author: "Taxlane"
date: "2026-07-27"
status: "accepted repository-only after round-two review"
---

# Abstract

This paper documents the transition from Taxlane's spending-admission result to
its internal ordinary-income rate recommendation. Zero admitted FY2026 primary
spending reduction [NUM-03] leaves a frozen $813.727 billion model target
[NUM-04]. Taxlane tests fourteen uniform rate uplifts [NUM-05] under three
taxable-income response cases and
three internal macro-stress values. It selects a preferred central schedule,
identifies a behavior-robust contingency, and reports a severe stress ceiling.
The method separates recommendation from sensitivity and bracket rates from
individual liability.

The schedules contain marginal rates—rates applied only to taxable income
within each bracket, not one person's effective rate. Evidence keys route to
the EXPL-A ledgers. The fiscal object is a one-year ordinary-income model target,
not total receipts/outlays, trust-fund solvency, borrowing, or formal balance
[CLM-11, CLM-12].

# 1. Input discipline

The rate step begins only after the fifteen-track admission pass. The frozen
inputs include zero admitted primary spending reduction, zero additive PAY
contribution, no direct NET cut, a $0.077 billion administration ceiling, and a
$813.727 billion FY2026 revenue target on the ordinary-income model rail
[NUM-04, NUM-10].

The target is not an official government requirement or the whole federal
budget. It is the amount Taxlane's defined experiment seeks to cover after its
admission decisions.

# 2. Model and grid

The experiment uses Tax-Calculator 6.5.1 with the bundled CPS tax-unit file for
tax year 2026. It applies uniform percentage-point uplifts to seven model bracket
rates. Fourteen uplifts are tested [NUM-05]. Each is evaluated under substitution-
elasticity cases of 0.15, 0.25, and 0.35 [NUM-21], producing forty-two behavior
cases [NUM-06].

Taxlane also combines each behavior case with macro stresses of 0, -2.5, and -5
percent [NUM-22]. These values are deliberately adverse internal sensitivities, not
forecasts. The combined grid therefore has nine stress cases per candidate.

# 3. Central recommendation

The selected 11.0-point uplift [NUM-07] yields marginal model rates of
21/23/33/35/43/46/48 percent [NUM-08]. Its central first-year cash proxy is
$819.220 billion [NUM-09]. After the $0.077 billion administration ceiling
[NUM-10] and stated debt sensitivity, its central gap is $5.416 billion
[NUM-11]. The ceiling is not a complete taxpayer, employer, preparer,
enforcement, avoidance, or transition-burden model.

Taxlane selects this tier because it is the smallest tested one-decimal uplift
that clears the frozen central case. That selection rule embodies a policy
objective: satisfy the target in the central case while minimizing the tested
uniform uplift. It is not the only possible objective.

# 4. Contingency and stress

The 12.0-point uplift [NUM-12] yields 22/24/34/36/44/47/49 percent [NUM-13]. It is the smallest
tested schedule covering all three behavior cases without additional macro
stress. Taxlane labels it a behavior-robust contingency rather than the central
recommendation.

The 12.6-point uplift [NUM-14] yields
22.6/24.6/34.6/36.6/44.6/47.6/49.6 percent [NUM-15]. It is
the first tested schedule covering all nine combined adverse cases. The worst-
case model gap is $3.094 billion [NUM-19]. Taxlane labels this a severe internal stress
ceiling, not a baseline or forecast.

# 5. Bracket rates are not tax bills

A marginal rate applies only to taxable income within a bracket. An effective
rate compares total tax with a defined income measure. The grid does not provide
thresholds, filing statuses, deductions, credits, payroll taxes, withholding,
refunds, or a legislative transition. It cannot calculate an individual's
liability.

The model reports distributional summaries, including the central mean tax
change, after-tax-income change, and top-decile share, but these do not replace
a complete distribution and incidence design for legislation. Tax-unit results
also do not by themselves establish employer, preparer, enforcement,
avoidance, or administrative effects. The bundled CPS analysis also does not
establish statutory incidence, family transitions, geographic effects, or the
full shifting of tax burdens through wages and prices.

# 6. Timing and cash

Taxlane converts full-year model output to a first-year cash proxy using a
0.774223895 realization ratio [NUM-20]. A cash proxy is not a conventional score. Fiscal-
year receipts, tax-year liability, withholding, refunds, implementation timing,
and behavioral response have distinct bases.

The $5.416 billion central gap [NUM-11] is likewise a model result, not proof of exact
balance. Rounding, specification, legal drafting, macro response, debt timing,
and unmodeled compliance can change it.

# 7. What adaptation means

“Adaptive” means analytically recomputable; it does not mean rates change
automatically or without legal authority. The recommendation can be recomputed
when admitted spending
paths, targets, model inputs, behavior assumptions, or objectives change. Any
actual rate, base, bracket threshold, effective date, or appropriation remains
a legal and political decision outside Taxlane's authority.

# 8. Limits and responsible use

The experiment is one-year and internal. It is not an official JCT/CBO score,
ten-year budget path, long-run solvency proof, legislative specification, or tax
advice. The preferred tier should not be presented without the contingency,
stress, model-basis, marginal-rate, and authority disclosures.

Within those limits, the method supplies a reproducible connection between an
evidence-gated spending decision and a transparent revenue sensitivity ladder.

# Evidence

1. `data/derived/breadth_benchmark_matrix/rev_internal_analysis_baseline_freeze.v1.draft.json`
2. `data/derived/breadth_benchmark_matrix/rev_internal_rate_sensitivity_grid_run.v1.generated.json`
3. `data/derived/breadth_benchmark_matrix/rev_internal_rate_sensitivity_grid_extension.v1.generated.json`
4. `data/derived/breadth_benchmark_matrix/rev_internal_rate_candidate_analysis.v1.generated.json`
5. `data/derived/breadth_benchmark_matrix/rev_internal_rate_analysis_completion.v1.draft.json`
6. `experiments/rev-level-3-taxcalc/run_grid.py`
7. `experiments/rev-level-3-taxcalc/analyze_grid.py`
