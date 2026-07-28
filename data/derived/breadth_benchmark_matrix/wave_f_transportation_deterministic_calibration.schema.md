# Wave F Transportation Deterministic Calibration Schema

This schema governs `wave_f_transportation_deterministic_calibration.v1.draft.json`.

## Completion boundary

Wave F is complete when the Pulse 87 transportation pilot simulator has a
role-reviewed, reproducible calibration fixture that:

- covers FY2025-FY2035 with baseline, modernization, and adverse stress paths;
- performs no optimization;
- recomputes primary outlays, net cash requirements, and transportation trust-fund balance changes;
- exercises explicit transfers, a no-op reserve interface, rounding, debt, and endogenous net-interest feedback;
- proves that incomplete floors block the modeled productivity reduction, lower target cost, and rate recognition;
- exercises all ten Wave F prerequisite interfaces while keeping every substantive readiness flag false; and
- publishes no official solver input, real reform score, target cost, rate, savings estimate, or balanced-budget result.

## Required identities

For every annual path row:

- `primary_outlays = gross_program_outlays + implementation_admin_outlays + fallback_remediation_outlays` where omitted terms are zero;
- `net_cash_requirement = primary_outlays - credited_offsetting_collections`;
- `fund_balance_change = dedicated_receipts + explicit_general_fund_transfer + other_scored_fund_income - net_cash_requirement`.

The interest fixture must use `closing_debt = opening_debt + primary_deficit + opening_interest` and `next_interest = closing_debt * effective_rate`. A larger primary deficit must increase subsequent debt and interest.

## Calibration boundary

All numeric values are synthetic calibration units. They test mechanics and
blocking behavior only. Substantive source custody, complete floors, real
reforms, distribution, incidence, receipt bases, reserve choices, and fiscal
paths remain prerequisites for any publishable solver or rate output.
