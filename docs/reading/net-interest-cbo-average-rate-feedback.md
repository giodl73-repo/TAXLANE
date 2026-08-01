# NET CBO average-rate feedback

Machine record:
`data/derived/breadth_benchmark_matrix/net_interest_cbo_average_rate_feedback.v1.draft.json`.

The matching February 2026 CBO source already contains an annual projected
average interest rate on debt. It runs from `3.383%` in FY2025 to `3.919%` in
FY2035 and is part of the same source vintage as TAXLANE's admitted debt and
net-interest baseline.

That supports a bounded incremental model. TAXLANE leaves every CBO baseline
row unchanged and calculates only the debt and interest difference created by
an admitted primary-balance change. Beginning debt differences receive the
full annual CBO average rate; current-year financing uses the validated midpoint
exposure and closed-form circularity. Baseline interest receipts stay embedded
in CBO net interest, while their incremental policy delta defaults to zero
unless a candidate supplies contrary evidence.

The regression fixture applies a mechanical one-time `$100 billion` primary
improvement in FY2026. It produces `$1.731470 billion` less net interest in
FY2026, `$42.326809 billion` cumulatively through FY2035, and `$142.326809
billion` less debt at the end of FY2035. These numbers test the model; they are
not admitted savings or a policy proposal.

This closes a matching-vintage aggregate rate path and a reduced-form feedback
test. It is not the full-stock maturity model and does not supply annual
maturity-bucket rollover, bucket-specific rate
stress, an auction-yield forecast, an admitted candidate effect, a solver
result, a rate change, or a balanced-budget claim.
