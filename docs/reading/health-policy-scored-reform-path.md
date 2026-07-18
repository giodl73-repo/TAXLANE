# Health policy-scored reform path

Machine record:
`data/derived/breadth_benchmark_matrix/health_policy_scored_reform_path.v1.draft.json`.

This is a gate, not a score.

It says that Taxlane does not yet have a policy-scored federal health reform
path for `central_reform` or `stress`. The current private-insurance evidence
remains useful as sensitivity context, but it does not become a federal budget
effect.

Required boundary:

Private-insurance payer-payment sensitivities are not federal gross savings,
net savings, premium forecasts, provider revenue forecasts, deficit effects, or
target costs.

For `central_reform`, the missing inputs are a specific federal policy
instrument, service/provider segmentation, annual phase-in, utilization and
volume response, coding and site-of-care behavior, network and consolidation
response, transition and enforcement costs, premium/wage/tax incidence, a
policy-specific score source, and passed outcome floors.

For `stress`, the path must be the same policy under adverse realization:
weaker payment effect, higher utilization, higher implementation cost, access
remediation, weaker receipts, and higher interest rates where relevant.

Stress is not the aggressive Medicare-relative private-insurance price sensitivity.

Until those inputs exist, every central and stress federal cash-flow field remains null and solver-ineligible.
