# NET current-law baseline compatibility audit schema

This record maps every NET formula input and every baseline-completion step to
existing source custody. A `ready` disposition requires a compatible admitted
artifact, not merely a related number.

The audit admits the CBO CORE-G spine for annual debt stock, annual net
interest, matching-vintage average rates, other financing, the FY2025–FY2035
horizon, zero-policy-change topline reconciliation, and reduced-form feedback.
It must not mislabel aggregate average rates as maturity-bucket rates or MSPD
snapshot diagnostics as an annual rollover schedule.

The original zero-policy fixture remains limited to replaying published topline
and debt identities. A separate validated reduced-form fixture may prove
incremental primary-balance feedback but may not claim maturity-aware feedback.

Readiness totals must equal the underlying disposition rows: seven of eight
formula inputs and six of nine completion steps. Full-stock maturity, solver,
admitted interest-savings, and rate-effect claims remain false.
