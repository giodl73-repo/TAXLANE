# Overspending-risk taxonomy

Machine record:
`data/derived/breadth_benchmark_matrix/overspending_risk_taxonomy.v1.draft.json`

This is an overspending-risk taxonomy, not a waste finding, fraud finding, recoverable-savings estimate, causal-savings estimate, budget score, or department-cut instruction.

Overspending risk means review needed, not proven waste.

The allowed classes are:

- descriptive anomaly;
- efficiency pressure;
- operations review candidate;
- control weakness;
- recoverability candidate;
- causal savings candidate;
- blocked / no claim.

The taxonomy allows a signal to create a review queue. It does not allow a signal
to become a public savings, waste, fraud, budget-score, or department-cut claim
without separate positive evidence.

Hard prohibitions:

- no fraud inference from an international comparison;
- no fraud inference from an improper-payment estimate;
- no savings credit from an improper-payment estimate alone;
- no savings credit from a peer gap alone;
- no recoverable-savings claim without same-cohort collection lineage;
- no technology-savings claim without transition costs and floor results;
- no department cut from a risk signal.

All public finding, score, savings, and department-cut outputs remain null. Every claim boolean remains false.
