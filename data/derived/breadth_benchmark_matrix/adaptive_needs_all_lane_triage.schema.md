# Adaptive needs all-lane triage schema

This schema describes `adaptive_needs_all_lane_triage.v1.draft.json`.

The record must contain all fifteen tracks exactly once. Every row carries one
source-custodied anchor with measure, unit, observation period, and period-
alignment status; available benchmark context; a provisional problem class; a
candidate response class; and a funding-type hypothesis.

The following fields remain null until comparable current measures and
intervention evidence exist: `pain_severity_score`,
`marginal_outcome_gain_score`, `delivery_capacity_score`,
`whole_system_net_value_score`, evidence and delivery multipliers, candidate
capacity, `overall_need_score`, and `reserve_allocation_billions`.

Problem classification is not a spending recommendation. Benchmark gaps are
not causal findings; baseline values set equal to no-regression thresholds are
not proof that pain is absent; improper payments are not recoverable savings;
REV is a financing constraint; PAY is non-additive; and NET is endogenous.
