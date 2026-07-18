# Social Security Scenario Gate Schema

Schema for `social_security_scenario_gate.v1.draft.json`.

The record gates Social Security target-cost scenarios until the required
demographic, trust-fund, policy-lever, distribution, and outcome-floor inputs
exist.

Required sections:

- Identity and paths to the target-cost contract, target rubric, rate model, and
  receipts allocation model.
- Evidence context paths for SOCX old-age spending, pension replacement, age
  poverty, and denominator research.
- FY2025 current-law context.
- `required_model_inputs`.
- Scenarios: `current_law`, `central_reform`, and `stress`.
- `claim_booleans`, `explicit_blockers`, and `readiness`.

Missing demographic paths, 75-year trust-fund paths, policy levers, floor
thresholds, distribution, administration, transition cost, and score provenance
must remain null or false. The record must not select a wage-base, rate,
eligibility, benefit-formula, solvency, savings, or balanced-rate claim.
