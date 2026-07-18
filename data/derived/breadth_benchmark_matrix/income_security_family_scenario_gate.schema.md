# Income Security and Family Scenario Gate Schema

Schema for `income_security_family_scenario_gate.v1.draft.json`.

The record gates income-security/family target-cost scenarios until benefit
package, eligible-population, take-up, federalism, distribution,
administration, transition-cost, and outcome-floor inputs exist.

Required sections:

- Identity and paths to the target-cost contract, target rubric, rate model, and
  receipts allocation model.
- Evidence context paths for family spending, age/child poverty, education
  access transition, and worker baseline context.
- FY2025 current-law context.
- `required_model_inputs`.
- Scenarios: `current_law`, `central_reform`, and `stress`.
- `claim_booleans`, `explicit_blockers`, and `readiness`.

Missing package, take-up, federalism, floor, distribution, administration,
transition-cost, and score-provenance values must remain null or false. The
record must not select or publish a benefit package, eligibility rule,
child-poverty result, childcare-access result, work-transition result, savings
claim, target cost, or balanced rate.
