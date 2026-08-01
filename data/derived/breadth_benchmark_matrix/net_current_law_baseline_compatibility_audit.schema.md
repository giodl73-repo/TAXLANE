# NET current-law baseline compatibility audit schema

This record maps every NET formula input and every baseline-completion step to
existing source custody. A `ready` disposition requires a compatible admitted
artifact, not merely a related number.

The audit admits the CBO CORE-G spine for annual debt stock, annual net
interest, other financing, the FY2025–FY2035 horizon, and zero-policy-change
topline reconciliation. It must reject aggregate average rates as maturity-
bucket rates and reject unreconciled MSPD diagnostics as an annual rollover
schedule.

The zero-policy fixture is limited to replaying the published topline and debt
identities. It may not claim that the endogenous maturity-aware interest
formula or a primary-balance shock has been reconciled.

Readiness totals must equal the underlying disposition rows: four of eight
formula inputs and five of nine completion steps. Solver, interest-savings, and
rate-effect claims remain false.
